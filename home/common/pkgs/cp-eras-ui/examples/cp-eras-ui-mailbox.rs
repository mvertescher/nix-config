//! The mailbox screen, in any era.
//!
//!     cp-eras-ui-mailbox                # follow the desktop theme
//!     cp-eras-ui-mailbox --era kitsch   # force one
//!
//! See examples/cp-eras-ui-store.rs for the reasoning behind the
//! --era handling; it is the same here.

use cp_eras_ui::screens::mail::{Message, MailBox};
use cp_eras_ui::{Era, Style};

fn main() -> iced::Result {
    let style = match era_from_args() {
        Some(era) => {
            let mut style = era.style();
            let theme = cp_eras_ui::theme::Theme::load();
            if Era::parse(&theme.era) == Some(era) {
                style.palette = style.palette.with_theme(&theme);
            }
            style
        }
        None => Style::from_desktop(),
    };

    iced::application(move || MailBox::new(style), MailBox::update, MailBox::view)
        .title(MailBox::title)
        .font(cp_eras_ui::fonts::RAJDHANI_REGULAR)
        .font(cp_eras_ui::fonts::RAJDHANI_MEDIUM)
        .font(cp_eras_ui::fonts::RAJDHANI_BOLD)
        .font(cp_eras_ui::fonts::ORBITRON_REGULAR)
        .font(cp_eras_ui::fonts::ORBITRON_BOLD)
        .default_font(cp_eras_ui::fonts::FONT_RAJDHANI_REGULAR)
        .window_size((1600.0, 900.0))
        .antialiasing(true)
        .run()
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
