//! The login screen, in any era.
//!
//!     cyberpunk-ui-login                # follow the desktop theme
//!     cyberpunk-ui-login --era kitsch   # force one
//!
//! See examples/cyberpunk-ui-store.rs for the reasoning behind the
//! --era handling; it is the same here.

use cyberpunk_ui::screens::login::{Message, Login};
use cyberpunk_ui::{Era, Style};

fn main() -> iced::Result {
    let style = match era_from_args() {
        Some(era) => {
            let mut style = era.style();
            let theme = cyberpunk_ui::theme::Theme::load();
            if Era::parse(&theme.era) == Some(era) {
                style.palette = style.palette.with_theme(&theme);
            }
            style
        }
        None => Style::from_desktop(),
    };

    iced::application(Login::title, Login::update, Login::view)
        .font(cyberpunk_ui::fonts::RAJDHANI_REGULAR)
        .font(cyberpunk_ui::fonts::RAJDHANI_MEDIUM)
        .font(cyberpunk_ui::fonts::RAJDHANI_BOLD)
        .font(cyberpunk_ui::fonts::ORBITRON_REGULAR)
        .font(cyberpunk_ui::fonts::ORBITRON_BOLD)
        .default_font(cyberpunk_ui::fonts::FONT_RAJDHANI_REGULAR)
        .window_size((1600.0, 900.0))
        .antialiasing(true)
        .run_with(move || (Login::new(style), iced::Task::none()))
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

#[allow(dead_code)]
fn _assert_message(_: Message) {}
