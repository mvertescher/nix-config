//! The 4ST store, in any era.
//!
//!     cp-eras-ui-store                # follow the desktop theme
//!     cp-eras-ui-store --era kitsch   # force one
//!
//! Rendering the same screen in each era side by side is the quickest
//! way to see whether the toolkit's claim holds -- and to compare
//! against `docs/<era>/store-trace.svg`, which is what
//! `scripts/fidelity_check.sh --implementation <era> store` holds this
//! binary to.
//!
//! The screen is live: clicking a category or a card selects it, and
//! the era's own table says which drawing each wears. It opens on the
//! selection its trace shows -- entropism's first card, everyone
//! else's second -- so a capture of it is comparable with the trace.

use cp_eras_ui::screens::store::Store;
use cp_eras_ui::{Era, Style};

fn main() -> iced::Result {
    let style = match era_from_args() {
        Some(era) => {
            let mut style = era.style();
            // Forcing an era still honours a published palette when the
            // desktop is already in it, so --era kitsch on a kitsch host
            // shows that host's variant rather than the reference.
            let theme = cp_eras_ui::theme::Theme::load();
            if Era::parse(&theme.era) == Some(era) {
                style.palette = style.palette.with_theme(&theme);
            }
            style
        }
        None => Style::from_desktop(),
    };

    iced::application(move || Store::new(style), Store::update, Store::view)
        .title(Store::title)
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
