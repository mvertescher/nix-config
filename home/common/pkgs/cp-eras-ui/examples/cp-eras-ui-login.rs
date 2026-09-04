//! The login screen, in any era.
//!
//!     cp-eras-ui-login                # follow the desktop theme
//!     cp-eras-ui-login --era kitsch   # force one
//!
//! See examples/cp-eras-ui-store.rs for the reasoning behind the
//! --era handling; it is the same here.
//!
//! The screen's *content* -- how many accounts are offered, what they
//! are called, which one is live, and every string on the page -- is
//! era table data (`Style::access`), transcribed from
//! `docs/<era>/login-trace.svg`. So this file only picks the era,
//! loads the faces and opens a 1600x900 window, which is the frame the
//! traces are measured in and the one `scripts/fidelity_check.sh
//! --implementation <era> login` captures.

use cp_eras_ui::screens::login::{Message, Login};
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

    iced::application(move || Login::new(style), Login::update, Login::view)
        .title(Login::title)
        // The access screen sets text at four weights: neokitsch's
        // ENTER / LOGIN bar is a light face, the guest names and the
        // clocks are medium, the badge digits and the compliance
        // brand are bold, and everything else regular. A weight the
        // application has not loaded falls back to the regular face
        // and the run comes out the wrong width.
        .font(cp_eras_ui::fonts::RAJDHANI_LIGHT)
        .font(cp_eras_ui::fonts::RAJDHANI_REGULAR)
        .font(cp_eras_ui::fonts::RAJDHANI_MEDIUM)
        .font(cp_eras_ui::fonts::RAJDHANI_SEMIBOLD)
        .font(cp_eras_ui::fonts::RAJDHANI_BOLD)
        .font(cp_eras_ui::fonts::NOTO_SANS_CJK_JP_BOLD)
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
