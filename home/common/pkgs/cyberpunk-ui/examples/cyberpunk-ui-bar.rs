//! The status bar, as a wlr-layer-shell surface.
//!
//!     cyberpunk-ui-bar                # follow the desktop theme
//!     cyberpunk-ui-bar --era kitsch   # force one
//!
//! The library half lives in `cyberpunk_ui::bar` and is a pure function
//! of Style and Readings. This binary is the two things that cannot be:
//! the layer-shell surface, and gathering the readings.
//!
//! Hyprland's IPC is spoken directly rather than through hyprland-rs,
//! which GitHub reports as NOASSERTION -- no clear licence. The protocol
//! is a line of text over a unix socket, so the dependency bought little
//! and the licence question was not worth inheriting.

use cyberpunk_ui::bar::{bar, Readings, Workspace};
use cyberpunk_ui::{Era, Style};
use iced::{Element, Task, Theme};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::{to_layer_message, Application};
use std::io::{Read, Write};
use std::time::Duration;

struct BarApp {
    style: Style,
    readings: Readings,
    system: sysinfo::System,
}

// Adds the layer-shell control variants the runtime dispatches through
// our own message type. Without it Message cannot convert into
// LayershellCustomActions and `run` will not accept the application.
#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Application for BarApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Style;

    fn new(style: Self::Flags) -> (Self, Task<Message>) {
        let mut app = BarApp {
            style,
            readings: Readings {
                host: hostname(),
                ..Readings::default()
            },
            system: sysinfo::System::new(),
        };
        app.refresh();
        (app, Task::none())
    }

    fn namespace(&self) -> String {
        format!("cyberpunk-ui-bar ({})", self.style.era.name())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::Tick = message {
            self.refresh();
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        bar(&self.style, &self.readings)
    }

    fn style(&self, _theme: &Self::Theme) -> iced_layershell::Appearance {
        // The bar paints its own ground; anything the runtime puts
        // behind it would show through the era's corner cuts.
        iced_layershell::Appearance {
            background_color: self.style.palette.bg.into(),
            text_color: self.style.palette.fg.into(),
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // One second is the clock's resolution; the cheaper readings
        // ride along rather than each keeping their own timer.
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}

impl BarApp {
    fn refresh(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        self.readings.cpu = self.system.global_cpu_usage().round().clamp(0.0, 100.0) as u8;
        let total = self.system.total_memory().max(1);
        self.readings.memory = ((self.system.used_memory() * 100) / total).min(100) as u8;

        let now = chrono::Local::now();
        self.readings.clock = now.format("%H:%M").to_string();
        self.readings.date = now.format("%Y-%m-%d").to_string();

        self.readings.workspaces = workspaces();
        self.readings.window = active_window();
    }
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
    sock.set_read_timeout(Some(Duration::from_millis(200))).ok()?;
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
    let style = resolve_style();
    let height = style.bar.height;

    BarApp::run(Settings {
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
        flags: style,
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
}

fn resolve_style() -> Style {
    match era_from_args() {
        Some(era) => {
            let mut style = era.style();
            let theme = cyberpunk_ui::theme::Theme::load();
            if Era::parse(&theme.era) == Some(era) {
                style.palette = style.palette.with_theme(&theme);
            }
            style
        }
        None => Style::from_desktop(),
    }
}

fn era_from_args() -> Option<Era> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if let Some(name) = arg.strip_prefix("--era=") {
            return Era::parse(name);
        }
        if arg == "--era" {
            return args.next().as_deref().and_then(Era::parse);
        }
    }
    None
}
