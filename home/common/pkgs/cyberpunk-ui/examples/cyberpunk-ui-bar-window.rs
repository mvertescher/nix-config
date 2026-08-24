//! The bar's view, in an ordinary window.
//!
//!     cyberpunk-ui-bar-window                # follow the desktop theme
//!     cyberpunk-ui-bar-window --era kitsch   # force one
//!
//! This exists so the bar can be golden-tested. `tests/visual.nix`
//! drives weston's headless backend, which has no `wlr-layer-shell`, so
//! it cannot run `cyberpunk-ui-bar` at all -- and even on a compositor
//! that could, a layer surface is not something `weston-screenshooter`
//! is any good at capturing.
//!
//! It is a separate binary rather than a `--windowed` flag on the real
//! bar on purpose. What a golden should hold still is `bar()`, the pure
//! function of `Style` and `Readings`; the live bar is that plus a layer
//! surface plus four sensor threads, none of which a screenshot can say
//! anything about. Keeping them apart means the test exercises exactly
//! what it claims to, and the binary that actually runs on the desktop
//! carries no branch that only a test ever takes.
//!
//! The readings are fixed for the same reason: a clock that ticked would
//! make every capture differ from the last, and the whole point is that
//! two runs are byte-identical.

#[path = "bar/style.rs"]
mod style;

use cyberpunk_ui::bar::{bar, Audio, Network, Readings, TrayItem, Workspace};
use cyberpunk_ui::Style;
use iced::widget::{column, container, Space};
use iced::{Element, Length};

/// Fixed readings. Every optional module is present, because a module
/// that is absent from the sample is a module the golden says nothing
/// about.
fn sample() -> Readings {
    Readings {
        host: "terra".to_string(),
        workspaces: (1..=6)
            .map(|id| Workspace {
                id,
                active: id == 3,
            })
            .collect(),
        window: "~/nix-config-private - nvim src/bar.rs".to_string(),
        cpu: 12,
        memory: 47,
        audio: Some(Audio {
            volume: 62,
            muted: false,
        }),
        // Two items, one of them shouting, so the golden covers both
        // inks a tray cell can take.
        tray: vec![
            TrayItem {
                label: "BLUE".to_string(),
                attention: false,
            },
            TrayItem {
                label: "SYNC".to_string(),
                attention: true,
            },
        ],
        network: Network::Wireless {
            interface: "wlp13s0".to_string(),
            ssid: "AFTERLIFE".to_string(),
        },
        clock: "23:41".to_string(),
        date: "2026-08-23".to_string(),
    }
}

struct BarWindow {
    style: Style,
    readings: Readings,
}

/// The window is a still life; there is nothing to send it.
#[derive(Debug, Clone)]
enum Message {}

impl BarWindow {
    fn title(&self) -> String {
        format!("cyberpunk-ui-bar - {}", self.style.era.name())
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        column![
            container(bar(&self.style, &self.readings))
                .height(Length::Fixed(self.style.bar.height as f32)),
            // The bar alone is a 26px strip. The empty ground under it
            // is what makes a capture legible as a desktop edge, and it
            // also puts the era's background role in the diff.
            container(Space::new(Length::Fill, Length::Fill)).height(Length::Fill),
        ]
        .into()
    }
}

fn main() -> iced::Result {
    let style = style::resolve();
    let bg = style.palette.bg;
    let fg = style.palette.fg;
    let state = BarWindow {
        style,
        readings: sample(),
    };

    iced::application(BarWindow::title, BarWindow::update, BarWindow::view)
        .font(cyberpunk_ui::fonts::RAJDHANI_REGULAR)
        .font(cyberpunk_ui::fonts::RAJDHANI_MEDIUM)
        .font(cyberpunk_ui::fonts::RAJDHANI_BOLD)
        .default_font(cyberpunk_ui::fonts::FONT_RAJDHANI_REGULAR)
        .style(move |_state, _theme| iced::application::Appearance {
            background_color: bg,
            text_color: fg,
        })
        .window_size((1600.0, 220.0))
        .antialiasing(true)
        .run_with(move || (state, iced::Task::none()))
}
