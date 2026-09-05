//! The status bar, as a wlr-layer-shell surface.
//!
//!     cp-eras-ui-bar                # follow the desktop theme
//!     cp-eras-ui-bar --era kitsch   # force one
//!
//! The library half lives in `cp_eras_ui::bar` and is a pure function
//! of Style and Readings. This binary is the two things that cannot be:
//! the layer-shell surfaces, and gathering the readings. Its companion
//! `cp-eras-ui-bar-window` draws the same view in an ordinary window,
//! which is what the golden tests capture.
//!
//! Hyprland's IPC is spoken directly rather than through hyprland-rs,
//! which GitHub reports as NOASSERTION -- no clear licence. The protocol
//! is a line of text over a unix socket, so the dependency bought little
//! and the licence question was not worth inheriting.
//!
//! Readings split by cost. The cheap ones -- clock, CPU, and memory --
//! are taken inline on the tick. The four that can stall (Hyprland's
//! IPC socket under load, a PulseAudio handshake, a wireless driver,
//! another process answering a D-Bus call) live on their own threads
//! in `bar/`, publish snapshots, and are read here without waiting.
//! See `bar/sensor.rs`.
//!
//! ## Two surfaces, and why the second one is the whole screen
//!
//! Right-clicking a tray item draws that item's `com.canonical.dbusmenu`
//! menu, which needs a surface of its own. There are three ways to make
//! one here and only one of them dismisses:
//!
//!   * An `xdg_popup` on the bar (`NewMenu`) is positioned by the
//!     compositor at the pointer, which is exactly what a menu wants --
//!     but `layershellev` never calls `xdg_popup.grab()` -- checked
//!     again on 0.19.1, and still true -- so
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
//! ## Submenus need no third surface
//!
//! A submenu opens *inside* that overlay, as another panel in the same
//! row -- see `cp_eras_ui::bar::tray_menu`, which does the placement.
//! There is no `layershell_open` for it and no second window id here.
//!
//! That is not a shortcut, it is the same argument one step further on.
//! A second output-sized overlay would cover the panel underneath it,
//! so every row of the parent menu would stop answering until the child
//! was drawn with a hole cut in it; a menu-sized layer surface is the
//! option already rejected above, for the grab. The overlay is the
//! whole output and the chain is a few hundred pixels of it, so the
//! only thing another surface would buy is a second thing to dismiss.
//!
//! Dismissal therefore does not change at all. One click anywhere else
//! destroys the one surface and takes the whole chain with it, rather
//! than unwinding a stack; `Message::Submenu` is the only thing that
//! moves within it, and clicking the row that is already open walks
//! back up one level.
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
#[path = "bar/hypr.rs"]
mod hypr;
#[path = "bar/icon.rs"]
mod icon;
#[path = "bar/network.rs"]
mod network;
#[path = "bar/sensor.rs"]
mod sensor;
#[path = "bar/tray.rs"]
mod tray;

use cp_eras_ui::bar::{
    bar, tray_menu, MenuEntry, MenuPath, Readings, TrayAction, TrayMenu,
};
use cp_eras_ui::shell;
use cp_eras_ui::{Element, Style};
use iced::widget::{column, container, mouse_area, row, Space};
use iced::{Length, Task};
use iced_layershell::build_pattern::daemon;
use iced_layershell::reexport::{
    Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings, OutputOption,
};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::to_layer_message;
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
    /// Which chain of submenus is open, empty for the root panel
    /// alone. State of the panel and not of the item, which is why it
    /// lives here rather than on the [`TrayMenu`].
    path: MenuPath,
    /// Where the chain's right edge sits, in output pixels.
    x: f32,
}

/// The entry a path names, if it still names one.
///
/// `None` for a path that has outrun the tree, which a re-read can
/// arrange -- see `Message::Opened`.
fn entry_at<'a>(menu: &'a TrayMenu, path: &[usize]) -> Option<&'a MenuEntry> {
    let mut entries = &menu.entries;
    let mut found = None;
    for &index in path {
        found = entries.get(index);
        entries = &found?.children;
    }
    found
}

struct BarApp {
    style: Style,
    readings: Readings,
    system: sysinfo::System,
    audio: audio::Monitor,
    network: network::Monitor,
    hypr: hypr::Monitor,
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
    /// A submenu row was clicked: open the chain it names, or close it
    /// again when it is the one already open.
    Submenu(MenuPath),
    /// Anywhere else on the screen was clicked.
    Dismiss,
    /// The compositor destroyed a surface. Only ever the menu's: the
    /// bar's own surface outlives the process.
    ///
    /// Was `daemon`'s `remove_id` callback until iced_layershell 0.19
    /// dropped it in favour of broadcasting the window event.
    Closed(iced::window::Id),
}

/// The tray's menu stream, and the identity iced keys its subscription
/// on.
///
/// `Subscription::run_with` wants one value that is both, and an
/// `async_channel::Receiver` is not `Hash` -- nor would hashing a
/// channel handle be right, since a fresh clone on every `view` would
/// then look like a different subscription and restart the stream. So
/// the hash is a constant: the same id `run_with_id` was given before.
#[derive(Debug, Clone)]
struct MenuStream(async_channel::Receiver<tray::Opened>);

impl std::hash::Hash for MenuStream {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash("cp-eras-ui-bar-tray-menus", state);
    }
}

impl BarApp {
    fn new(style: Style) -> Self {
        let tray = tray::Config {
            show_passive: flag("--show-passive"),
            icon_theme: option("--icon-theme"),
            // Decode at the size the cell will draw, so the icon is
            // resampled once rather than twice.
            icon_size: cp_eras_ui::bar::icon_size(&style).round() as u32,
            menu_icon_size: cp_eras_ui::bar::menu_icon_size(&style).round() as u32,
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
            hypr: hypr::Monitor::spawn(),
            tray: tray::Monitor::spawn(tray),
            pointer: 0.0,
            pending: None,
            open: None,
        };
        app.refresh();
        app
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
                // A menu arriving for the item already on screen is
                // the re-read that follows opening a submenu. Spliced
                // in place rather than reopened: the surface, its
                // placement and the open chain are all still good, and
                // destroying the surface to draw the same menu one row
                // deeper would flash.
                if let Some(open) = &mut self.open {
                    if open.key == opened.key {
                        // A dbusmenu id is the protocol's own notion
                        // of row identity and is stable across a
                        // re-read; a path that no longer lands on the
                        // row it was opened for means the application
                        // rearranged its menu underneath us, and the
                        // honest answer to that is the root.
                        let was = entry_at(&open.menu, &open.path).map(|entry| entry.id);
                        let now = entry_at(&opened.menu, &open.path).map(|entry| entry.id);
                        if was != now {
                            open.path.clear();
                        }
                        open.menu = opened.menu;
                    }
                    return Task::none();
                }

                // Only the menu this bar last asked for: an item is
                // free to answer late and twice.
                let asked = self
                    .pending
                    .as_ref()
                    .is_some_and(|pending| pending.key == opened.key);
                if !asked {
                    return Task::none();
                }
                let Some(pending) = self.pending.take() else {
                    return Task::none();
                };
                let (window, open_surface) = Message::layershell_open(NewLayerShellSettings {
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
                    // Was `use_last_output: true` before
                    // iced_layershell 0.19 widened the choice to an
                    // enum; `LastOutput` is the same answer.
                    output_option: OutputOption::LastOutput,
                    events_transparent: false,
                    namespace: None,
                });
                self.open = Some(Open {
                    window,
                    key: opened.key.clone(),
                    menu: opened.menu.clone(),
                    path: MenuPath::new(),
                    x: pending.x,
                });
                return open_surface;
            }
            Message::Entry(entry) => {
                if let Some(open) = &self.open {
                    self.tray.activate(&open.key, entry);
                }
                return self.dismiss();
            }
            Message::Submenu(path) => {
                let Some(open) = &mut self.open else {
                    return Task::none();
                };
                // Clicking the row that is already open closes it,
                // which on a surface that reads no keys is the only
                // way back up the chain that is not "start again".
                // Nothing is sent for that: dbusmenu has an event for
                // closing the *menu*, which `dismiss` sends, and none
                // for closing one branch of it.
                if open.path == path {
                    open.path.pop();
                    return Task::none();
                }
                open.path = path;
                // The panel is already drawn, from the tree that came
                // with the menu. This is the protocol courtesy that
                // gives an application filling a submenu on demand its
                // chance to; its answer arrives as another `Opened`
                // and is spliced in above.
                if let Some(entry) = entry_at(&open.menu, &open.path) {
                    self.tray.expand(&open.key, entry.id);
                }
            }
            Message::Dismiss => return self.dismiss(),
            // The compositor destroyed a surface. Only ever the menu's:
            // the bar's own surface outlives the process, and a
            // `Closed` naming any other window falls through.
            Message::Closed(id)
                if self.open.as_ref().is_some_and(|open| open.window == id) =>
            {
                self.open = None;
            }
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

    fn view(&self, window: iced::window::Id) -> Element<'_, Message> {
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
                        .style(move |_: &Style| container::Style {
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
    fn menu<'a>(&'a self, open: &'a Open) -> Element<'a, Message> {
        // The *chain's* width, not the root panel's: submenus open
        // leftwards, so opening one widens the element to the left and
        // leaves the root panel exactly where it was.
        let width = cp_eras_ui::bar::menu_chain_width(&self.style, &open.menu, &open.path);
        // Right edge on the pointer, clamped at the left edge of the
        // output. See the module header for why this way round.
        //
        // The clamp is the one case where opening a submenu does move
        // the root panel: a chain longer than the pointer's distance
        // from the left edge has nowhere further left to go, so the
        // whole thing slides right. Sliding is better than the
        // alternative, which is drawing off the screen, and it takes a
        // deep chain under a pointer near the left edge -- where the
        // tray never is.
        let left = (open.x - width).max(0.0);

        // No offset from the top: the surface already begins where the
        // bars end, which is where a menu hanging off a bar belongs.
        mouse_area(
            column![row![
                Space::new().width(Length::Fixed(left)).height(Length::Shrink),
                tray_menu(
                    &self.style,
                    &open.menu,
                    &open.path,
                    Message::Entry,
                    Message::Submenu,
                ),
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

    // `iced_layershell::Appearance` was an alias for iced's own
    // `theme::Style` and stopped being public in 0.19; the type is the
    // same one, named where it comes from.
    fn style(&self, _theme: &Style) -> iced::theme::Style {
        // Transparent, not the era's ground: the runtime paints one
        // colour for every surface this application owns, and the menu
        // surface covers the whole output. The bar paints its own in
        // `view`.
        iced::theme::Style {
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
            // ... and, on the same listener, the compositor destroying
            // a surface. iced_layershell 0.19 dropped `daemon`'s
            // `remove_id` hook and broadcasts a window `Closed` event
            // instead, so this is where the menu's id is forgotten.
            iced::event::listen_with(|event, _status, window| match event {
                iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                    Some(Message::Pointer(position.x))
                }
                iced::Event::Window(iced::window::Event::Closed) => {
                    Some(Message::Closed(window))
                }
                _ => None,
            }),
            // Menus, as the tray thread fetches them. A subscription
            // rather than a snapshot read on the tick, because a menu
            // that turns up as much as a second after the click is a
            // menu that feels broken.
            //
            // 0.14 replaced `run_with_id(id, stream)` with
            // `run_with(data, fn(&data) -> stream)`, where `data` is
            // both the stream's source and the subscription's identity.
            // [`MenuStream`] is that pair: a receiver handle that
            // hashes as one constant id, so the subscription is the
            // same one across every rebuild of the view.
            iced::Subscription::run_with(MenuStream(self.tray.opened()), |menus| {
                futures_lite::StreamExt::map(menus.0.clone(), Message::Opened)
            }),
        ])
    }

    fn refresh(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        self.readings.cpu = self.system.global_cpu_usage().round().clamp(0.0, 100.0) as u8;
        let total = self.system.total_memory().max(1);
        self.readings.memory = ((self.system.used_memory() * 100) / total).min(100) as u8;

        // Snapshots from the sensor threads: whatever they have as of
        // now, never a wait for something newer. Hyprland's socket is
        // one of them for the same reason the tray is -- a busy
        // compositor can take half a second to answer, and a frame
        // must not wait on it.
        self.readings.audio = self.audio.reading();
        self.readings.network = self.network.reading();
        let hypr = self.hypr.reading();
        self.readings.workspaces = hypr.workspaces;
        self.readings.window = hypr.window;
        self.readings.tray = self.tray.reading();

        let now = chrono::Local::now();
        self.readings.clock = now.format("%H:%M").to_string();
        self.readings.date = now.format("%Y-%m-%d").to_string();
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

fn main() -> Result<(), iced_layershell::Error> {
    let style = shell::style();
    let height = style.bar.height;

    // 0.19's `daemon` boots the state itself, as iced's own builders
    // do, and has no `remove_id` slot -- see `Message::Closed`. Its
    // namespace is taken before the state exists rather than off it,
    // so it is built here from the same resolved `Style`.
    let namespace = format!("cp-eras-ui-bar ({})", style.era.name());

    daemon(
        move || BarApp::new(style),
        move || namespace.clone(),
        BarApp::update,
        BarApp::view,
    )
    .theme(|app: &BarApp, _window| app.style)
    .style(BarApp::style)
    .subscription(BarApp::subscription)
    .settings(Settings {
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
        fonts: shell::faces(),
        default_font: shell::DEFAULT_FONT,
        default_text_size: 14.0.into(),
        antialiasing: true,
        id: Some("cp-eras-ui-bar".to_string()),
        virtual_keyboard_support: None,
        with_connection: None,
    })
    .run()
}
