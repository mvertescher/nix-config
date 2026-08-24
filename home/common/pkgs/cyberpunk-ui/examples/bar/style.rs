//! Which era the bar wears, shared by the two binaries that draw it.
//!
//! `cyberpunk-ui-bar` and `cyberpunk-ui-bar-window` have to agree about
//! this exactly, or the golden images are evidence about a style the
//! live bar never uses. Sharing the resolution is what makes them agree
//! by construction rather than by review.

use cyberpunk_ui::{Era, Style};

/// The desktop's published theme, or the one named on the command line.
///
/// `--era` takes the era's compiled defaults and only layers the
/// published palette over them when the desktop is *already* in that
/// era -- forcing kitsch on a neomil desktop should give kitsch's own
/// colours, not neomil's wearing kitsch's geometry.
pub fn resolve() -> Style {
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
