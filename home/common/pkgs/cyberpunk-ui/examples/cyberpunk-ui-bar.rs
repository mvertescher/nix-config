//! The status bar, as a wlr-layer-shell surface.
//!
//!     cyberpunk-ui-bar                # follow the desktop theme
//!     cyberpunk-ui-bar --era kitsch   # force one
//!
//! The library half lives in `cyberpunk_ui::bar` and is a pure function
//! of Style and Readings. This binary is the two things that cannot be:
//! the layer-shell surfaces, and gathering the readings. Its companion
//! `cyberpunk-ui-bar-window` draws the same view in an ordinary window,
//! which is what the golden tests capture.
//!
//! Hyprland's IPC is spoken directly rather than through hyprland-rs,
//! which GitHub reports as NOASSERTION -- no clear licence. The protocol
//! is a line of text over a unix socket, so the dependency bought little
//! and the licence question was not worth inheriting.
//!
//! Readings split by cost. The cheap ones -- clock, CPU, memory, and
//! Hyprland's two IPC round trips over a local socket -- are taken
//! inline on the tick. The three that can stall (a PulseAudio
//! handshake, a wireless driver, another process answering a D-Bus
//! call) live on their own threads in `bar/`, publish snapshots, and
//! are read here without waiting. See `bar/sensor.rs`.
//!
//! ## Two surfaces, and why the second one is the whole screen
//!
//! Right-clicking a tray item draws that item's `com.canonical.dbusmenu`
//! menu, which needs a surface of its own. There are three ways to make
//! one here and only one of them dismisses:
//!
//!   * An `xdg_popup` on the bar (`NewMenu`) is positioned by the
//!     compositor at the pointer, which is exactly what a menu wants --
//!     but `layershellev` 0.13.7 never calls `xdg_popup.grab()`, so
//!     nothing tells us when the user clicks somewhere else, and the
//!     bar takes no keyboard focus to lose either. The menu would only
//!     ever close by being clicked.
//!   * A layer surface the size of the menu has the same problem.
//!   * A layer surface the size of the *output*, transparent except
//!     where the menu is drawn, hears every click on the screen. Which
//!     is what a pointer grab is for, done with the one primitive this
//!     stack does offer.
//!
//! So the menu is an `Overlay` layer surface anchored to all four
//! edges, and the application background is `TRANSPARENT` -- the bar
//! paints its own ground in `view` instead, because the runtime paints
//! one colour for every surface and only one of the two wants it.
//!
//! Its exclusive zone is `0`, "reserve nothing, respect what others
//! reserved", which is what makes the compositor start the surface
//! immediately below every bar on the screen. That is both where the
//! menu belongs and the reason this file never has to ask where the bar
//! is. The cost is that the bar strip itself is not covered, so the bar
//! carries its own dismiss-on-click.
//!
//! The menu is right-aligned on the pointer rather than left. The tray
//! is the last group on the right-hand side of the bar, so a menu
//! hanging to the right of the pointer is a menu hanging off the edge
//! of the screen; hanging it to the left needs no knowledge of how wide
//! the output is.
//!
//! Two flags beyond `--era`, both about the tray:
//!
//!     --icon-theme <name>   the icon theme tray icons are looked up
//!                           in. Nothing on this desktop sets
//!                           `gtk-icon-theme-name`, so without this the
//!                           search falls through to `hicolor` and to
//!                           whatever an item ships itself.
//!     --show-passive        draw items whose `Status` is `Passive`,
//!                           which the spec says a host should hide.

#[path = "bar/audio.rs"]
mod audio;
#[path = "bar/icon.rs"]
mod icon;
#[path = "bar/network.rs"]
mod network;
#[path = "bar/sensor.rs"]
mod sensor;
#[path = "bar/style.rs"]
mod style;
#[path = "bar/tray.rs"]
mod tray;

use cyberpunk_ui::bar::{bar, tray_menu, Readings, TrayAction, TrayMenu, Workspace};
use cyberpunk_ui::Style;
use iced::widget::{column, container, mouse_area, row, Space};
use iced::{Element, Length, Task, Theme};
use iced_layershell::build_pattern::{daemon, MainSettings};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings};
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;
use std::io::{Read, Write};
use std::time::Duration;

/// A menu that has been asked for and not yet arrived.
///
/// The tray answers on its own thread, so the click and the menu are
/// two events. The key is kept so a menu that arrives after the item it
/// belongs to has gone is dropped rather than drawn against the wrong
/// one; the pointer position is kept because by then the pointer has
/// moved.
struct Pending {
    key: String,
    x: f32,
}

/// A menu currently on screen.
struct Open {
    /// The surface it is drawn on, which is also how `view` tells the
    /// two windows apart -- there is no need to know the bar's own id.
    window: iced::window::Id,
    key: String,
    menu: TrayMenu,
    /// Where the menu's right edge sits, in output pixels.
    x: f32,
}

struct BarApp {
    style: Style,
    readings: Readings,
    system: sysinfo::System,
    audio: audio::Monitor,
    network: network::Monitor,
    tray: tray::Monitor,
    /// Last pointer position, in surface pixels. Both surfaces span the
    /// output's width, so this is an output x either way.
    pointer: f32,
    pending: Option<Pending>,
    open: Option<Open>,
}

// Adds the layer-shell control variants the runtime dispatches through
// our own message type. `multi` because this application owns more than
// one surface; without it Message cannot convert into
// LayershellCustomActionsWithId and the daemon will not accept it.
#[to_layer_message(multi)]
#[derive(Debug, Clone)]
enum Message {
    Tick,
    /// A pointer event on the tray cell at this index. The bar is
    /// otherwise entirely non-interactive, and this is the only reason
    /// it accepts pointer input at all.
    Tray(usize, TrayAction),
    /// The pointer moved, on either surface.
    Pointer(f32),
    /// An item answered with its menu.
    Opened(tray::Opened),
    /// A row of the open menu was clicked.
    Entry(i32),
    /// Anywhere else on the screen was clicked.
    Dismiss,
}

impl BarApp {
    fn new(style: Style) -> Self {
        let tray = tray::Config {
            show_passive: flag("--show-passive"),
            icon_theme: option("--icon-theme"),
            // Decode at the size the cell will draw, so the icon is
            // resampled once rather than twice.
            icon_size: cyberpunk_ui::bar::icon_size(&style).round() as u32,
        };
        let mut app = BarApp {
            style,
            readings: Readings {
                host: hostname(),
                ..Readings::default()
            },
            system: sysinfo::System::new(),
            audio: audio::Monitor::spawn(),
            network: network::Monitor::spawn(),
            tray: tray::Monitor::spawn(tray),
            pointer: 0.0,
            pending: None,
            open: None,
        };
        app.refresh();
        app
    }

    fn namespace(&self) -> String {
        format!("cyberpunk-ui-bar ({})", self.style.era.name())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.refresh();
                // An application that exits with its menu up leaves a
                // panel over the screen that answers to nobody.
                if self
                    .open
                    .as_ref()
                    .is_some_and(|open| !self.tray.holds(&open.key))
                {
                    return self.dismiss();
                }
            }
            Message::Pointer(x) => self.pointer = x,
            // Handed to the tray thread rather than acted on here: the
            // call goes to another process, and the bar must not wait
            // on one.
            Message::Tray(index, action) => {
                // A second right click, on this cell or another one,
                // replaces the menu rather than stacking a surface on
                // top of one already up.
                let close = self.dismiss();
                self.pending = self.tray.key(index).map(|key| Pending {
                    key,
                    x: self.pointer,
                });
                self.tray.dispatch(index, action);
                return close;
            }
            Message::Opened(opened) => {
                // Only the menu this bar last asked for, and only when
                // nothing is already up: an item is free to answer late
                // and twice.
                let asked = self
                    .pending
                    .as_ref()
                    .is_some_and(|pending| pending.key == opened.key);
                if !asked || self.open.is_some() {
                    return Task::none();
                }
                let Some(pending) = self.pending.take() else {
                    return Task::none();
                };
                let (window, message) = Message::layershell_open(NewLayerShellSettings {
                    // Anchored on all four edges with a zero size, the
                    // compositor hands back whatever area is left.
                    size: Some((0, 0)),
                    layer: Layer::Overlay,
                    anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
                    // Zero, which is "reserve nothing but respect what
                    // others reserved". That is what puts the top of
                    // this surface exactly under the last bar on the
                    // screen -- ours, and anything else running -- so
                    // the menu hangs off the bottom of the bar without
                    // this file ever having to work out where the bar
                    // is. `-1` would ignore those zones and start the
                    // surface at the top of the output, which is only
                    // the same answer when nothing else is running.
                    exclusive_zone: Some(0),
                    margin: None,
                    // The bar reads no keys and neither does this. A
                    // menu that took the keyboard would take it from
                    // whatever the person was typing in.
                    keyboard_interactivity: KeyboardInteractivity::None,
                    use_last_output: true,
                    events_transparent: false,
                });
                self.open = Some(Open {
                    window,
                    key: opened.key.clone(),
                    menu: opened.menu.clone(),
                    x: pending.x,
                });
                return Task::done(message);
            }
            Message::Entry(entry) => {
                if let Some(open) = &self.open {
                    self.tray.activate(&open.key, entry);
                }
                return self.dismiss();
            }
            Message::Dismiss => return self.dismiss(),
            _ => {}
        }
        Task::none()
    }

    /// Take the menu off the screen, and say so.
    fn dismiss(&mut self) -> Task<Message> {
        self.pending = None;
        let Some(open) = self.open.take() else {
            return Task::none();
        };
        self.tray.closed(&open.key);
        iced::window::close(open.window)
    }

    /// The compositor destroyed a surface. Only ever the menu's: the
    /// bar's own surface outlives the process.
    fn remove_id(&mut self, id: iced::window::Id) {
        if self.open.as_ref().is_some_and(|open| open.window == id) {
            self.open = None;
        }
    }

    fn view(&self, window: iced::window::Id) -> Element<'_, Message, Theme, iced::Renderer> {
        match &self.open {
            Some(open) if open.window == window => self.menu(open),
            _ => {
                // The application background is transparent so the menu
                // surface can be; the bar paints its own.
                let bg = self.style.palette.bg;
                // The menu surface starts below the bar, so the bar is
                // the one strip of screen its dismiss-on-click does not
                // cover. A tray cell captures the press before this
                // sees it, so clicking one still opens a menu.
                mouse_area(
                    container(bar(&self.style, &self.readings, Some(Message::Tray)))
                        .style(move |_: &Theme| container::Style {
                            background: Some(bg.into()),
                            ..container::Style::default()
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .on_press(Message::Dismiss)
                .into()
            }
        }
    }

    /// The menu, and the rest of the screen it is listening to.
    fn menu<'a>(&'a self, open: &'a Open) -> Element<'a, Message, Theme, iced::Renderer> {
        let width = cyberpunk_ui::bar::menu_width(&self.style, &open.menu);
        // Right edge on the pointer, clamped at the left edge of the
        // output. See the module header for why this way round.
        let left = (open.x - width).max(0.0);

        // No offset from the top: the surface already begins where the
        // bars end, which is where a menu hanging off a bar belongs.
        mouse_area(
            column![row![
                Space::new(Length::Fixed(left), Length::Shrink),
                tray_menu(&self.style, &open.menu, Message::Entry),
            ]
            .height(Length::Shrink)]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        // Any button, because dismissing is what every one of them
        // means once a menu is up.
        .on_press(Message::Dismiss)
        .on_right_press(Message::Dismiss)
        .on_middle_press(Message::Dismiss)
        .into()
    }

    fn style(&self, _theme: &Theme) -> iced_layershell::Appearance {
        // Transparent, not the era's ground: the runtime paints one
        // colour for every surface this application owns, and the menu
        // surface covers the whole output. The bar paints its own in
        // `view`.
        iced_layershell::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: self.style.palette.fg,
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch([
            // One second is the clock's resolution; the cheaper readings
            // ride along rather than each keeping their own timer.
            iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick),
            // The pointer, so a menu can be placed where the click was.
            // `bar()` reports which cell was clicked and not where,
            // which is right -- a cell index is what the tray can act
            // on, and a pixel is only ever this.
            iced::event::listen_with(|event, _status, _window| match event {
                iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                    Some(Message::Pointer(position.x))
                }
                _ => None,
            }),
            // Menus, as the tray thread fetches them. A subscription
            // rather than a snapshot read on the tick, because a menu
            // that turns up as much as a second after the click is a
            // menu that feels broken.
            iced::Subscription::run_with_id(
                "cyberpunk-ui-bar-tray-menus",
                futures_lite::StreamExt::map(self.tray.opened(), Message::Opened),
            ),
        ])
    }

    fn refresh(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        self.readings.cpu = self.system.global_cpu_usage().round().clamp(0.0, 100.0) as u8;
        let total = self.system.total_memory().max(1);
        self.readings.memory = ((self.system.used_memory() * 100) / total).min(100) as u8;

        // Snapshots from the sensor threads: whatever they have as of
        // now, never a wait for something newer.
        self.readings.audio = self.audio.reading();
        self.readings.network = self.network.reading();
        self.readings.tray = self.tray.reading();

        let now = chrono::Local::now();
        self.readings.clock = now.format("%H:%M").to_string();
        self.readings.date = now.format("%Y-%m-%d").to_string();

        self.readings.workspaces = workspaces();
        self.readings.window = active_window();
    }
}

/// Whether a bare flag was given.
///
/// Hand-rolled for the same reason `--era` is, one file over: three
/// options is not an argument parser's worth of dependency.
fn flag(name: &str) -> bool {
    std::env::args().skip(1).any(|arg| arg == name)
}

/// The value of `--name value` or `--name=value`.
fn option(name: &str) -> Option<String> {
    let mut args = std::env::args().skip(1);
    let prefix = format!("{name}=");
    while let Some(arg) = args.next() {
        if let Some(value) = arg.strip_prefix(&prefix) {
            return Some(value.to_string());
        }
        if arg == name {
            return args.next();
        }
    }
    None
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// One request/response over Hyprland's IPC socket.
///
/// Returns None rather than propagating: a bar that vanishes because the
/// compositor socket hiccuped is worse than one showing a stale
/// workspace, and every caller here has a reasonable empty case.
fn hypr(command: &str) -> Option<String> {
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let runtime = std::env::var("XDG_RUNTIME_DIR").ok()?;
    let path = format!("{runtime}/hypr/{sig}/.socket.sock");

    let mut sock = std::os::unix::net::UnixStream::connect(path).ok()?;
    sock.set_read_timeout(Some(Duration::from_millis(200)))
        .ok()?;
    sock.write_all(command.as_bytes()).ok()?;

    let mut out = String::new();
    sock.read_to_string(&mut out).ok()?;
    Some(out)
}

/// Parsed from the plain-text form rather than `j/`, to avoid taking a
/// JSON dependency for two fields.
///
/// `workspaces` lists every workspace as "workspace ID <n> (<name>) on
/// monitor <m>:", and `activeworkspace` names the current one.
fn workspaces() -> Vec<Workspace> {
    let active = hypr("activeworkspace")
        .and_then(|s| parse_workspace_id(&s))
        .unwrap_or(-1);

    let mut ids: Vec<i32> = hypr("workspaces")
        .map(|s| s.lines().filter_map(parse_workspace_line).collect())
        .unwrap_or_default();

    // The compositor lists them in creation order, which jumps around as
    // workspaces come and go; the bar wants them stable.
    ids.sort_unstable();
    ids.dedup();

    if ids.is_empty() && active >= 0 {
        ids.push(active);
    }

    ids.into_iter()
        .map(|id| Workspace {
            id,
            active: id == active,
        })
        .collect()
}

fn parse_workspace_line(line: &str) -> Option<i32> {
    let rest = line.trim().strip_prefix("workspace ID ")?;
    rest.split_whitespace().next()?.parse().ok()
}

fn parse_workspace_id(reply: &str) -> Option<i32> {
    reply.lines().find_map(parse_workspace_line)
}

/// `activewindow` returns a block of "key: value" lines; the title is
/// the one worth showing. Empty when nothing is focused, which is a
/// normal state rather than a failure.
fn active_window() -> String {
    let Some(reply) = hypr("activewindow") else {
        return String::new();
    };
    reply
        .lines()
        .find_map(|l| l.trim().strip_prefix("title: "))
        .map(|t| {
            let t = t.trim();
            // Long titles push the right-hand modules off a narrow
            // screen, so clip rather than let the layout fight back.
            if t.chars().count() > 90 {
                let clipped: String = t.chars().take(89).collect();
                format!("{clipped}…")
            } else {
                t.to_string()
            }
        })
        .unwrap_or_default()
}

fn main() -> Result<(), iced_layershell::Error> {
    let style = style::resolve();
    let height = style.bar.height;

    daemon(
        BarApp::namespace,
        BarApp::update,
        BarApp::view,
        BarApp::remove_id,
    )
    .style(BarApp::style)
    .subscription(BarApp::subscription)
    .settings(MainSettings {
        layer_settings: LayerShellSettings {
            // Top rather than Overlay: the bar should sit above windows
            // but never above a lock screen or a fullscreen prompt.
            layer: Layer::Top,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            size: Some((0, height)),
            // Reserving the height keeps tiled windows from sliding
            // under the bar.
            exclusive_zone: height as i32,
            keyboard_interactivity: KeyboardInteractivity::None,
            ..Default::default()
        },
        fonts: vec![
            cyberpunk_ui::fonts::RAJDHANI_REGULAR.into(),
            cyberpunk_ui::fonts::RAJDHANI_MEDIUM.into(),
            cyberpunk_ui::fonts::RAJDHANI_BOLD.into(),
        ],
        default_font: cyberpunk_ui::fonts::FONT_RAJDHANI_REGULAR,
        default_text_size: 14.0.into(),
        antialiasing: true,
        id: Some("cyberpunk-ui-bar".to_string()),
        virtual_keyboard_support: None,
    })
    .run_with(move || (BarApp::new(style), Task::none()))
}
