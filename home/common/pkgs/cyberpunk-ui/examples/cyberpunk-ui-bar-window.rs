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
use iced::widget::{column, container, image, Space};
use iced::{Element, Length};

/// A tray icon, drawn here rather than read from anywhere.
///
/// The golden has to hold the *icon* path still, not just the label
/// fallback, and it cannot do that with a real icon: the sandbox has no
/// icon theme, and even on a desktop the file behind `IconName` belongs
/// to whatever version of whatever package is installed. So the sample
/// synthesises one -- a diamond, because it is four comparisons and it
/// is obvious in a diff which way round it went.
///
/// 28 square is what the tray decodes at for a 26px bar: twice the
/// 14px the cell draws, so the capture shows the same downsampling the
/// live bar does.
fn sample_icon(rgb: [u8; 3]) -> image::Handle {
    const SIDE: i32 = 28;
    let mut data = Vec::with_capacity((SIDE * SIDE * 4) as usize);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let distance = (x * 2 - SIDE + 1).abs() + (y * 2 - SIDE + 1).abs();
            let inside = distance <= SIDE - 2;
            data.extend_from_slice(&[rgb[0], rgb[1], rgb[2], if inside { 255 } else { 0 }]);
        }
    }
    image::Handle::from_rgba(SIDE as u32, SIDE as u32, data)
}

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
        // All four shapes a tray cell can take: an icon, an icon from
        // an item that is shouting, and the label fallback in both of
        // its inks. A cell that is not in the sample is a cell the
        // golden says nothing about, and the fallback is not a
        // deprecated path -- it is what an item whose icon no installed
        // theme has actually gets.
        tray: vec![
            TrayItem {
                label: "FLAM".to_string(),
                icon: Some(sample_icon([0x88, 0x00, 0xaa])),
                attention: false,
            },
            TrayItem {
                label: "ELEM".to_string(),
                icon: Some(sample_icon([0xff, 0x66, 0x00])),
                attention: true,
            },
            TrayItem {
                label: "BLUE".to_string(),
                icon: None,
                attention: false,
            },
            TrayItem {
                label: "SYNC".to_string(),
                icon: None,
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
            // `None`: the golden is a still life, and hit-testing it
            // could never exercise would only be a claim the capture
            // cannot check.
            container(bar(&self.style, &self.readings, None))
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
