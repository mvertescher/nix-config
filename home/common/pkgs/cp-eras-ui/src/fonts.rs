pub const ORBITRON_REGULAR: &[u8] = include_bytes!("../fonts/Orbitron-Regular.ttf");
pub const ORBITRON_MEDIUM: &[u8] = include_bytes!("../fonts/Orbitron-Medium.ttf");
pub const ORBITRON_SEMIBOLD: &[u8] = include_bytes!("../fonts/Orbitron-SemiBold.ttf");
pub const ORBITRON_BOLD: &[u8] = include_bytes!("../fonts/Orbitron-Bold.ttf");

pub const RAJDHANI_LIGHT: &[u8] = include_bytes!("../fonts/Rajdhani-Light.ttf");
pub const RAJDHANI_REGULAR: &[u8] = include_bytes!("../fonts/Rajdhani-Regular.ttf");
pub const RAJDHANI_MEDIUM: &[u8] = include_bytes!("../fonts/Rajdhani-Medium.ttf");
pub const RAJDHANI_SEMIBOLD: &[u8] = include_bytes!("../fonts/Rajdhani-SemiBold.ttf");
pub const RAJDHANI_BOLD: &[u8] = include_bytes!("../fonts/Rajdhani-Bold.ttf");
/// GNU FreeFont 20120503, copied from the pinned Nix font package.
/// Used only for Kitsch compliance; its Bold file declares weight 600.
pub const FREE_SANS_BOLD: &[u8] = include_bytes!("../fonts/FreeSansBold.ttf");
/// Named GNU FreeFont derivative for ordinary Kitsch Store second-line M.
pub const CP_ERAS_KITSCH_SANS_BOLD: &[u8] = include_bytes!("../fonts/CP-Eras-Kitsch-Sans-Bold.ttf");

/// Noto Sans CJK JP 700, subset to the kanji the era tables set --
/// today the neomil store's MASURAO logotype (`益荒男`, `eras/neomil.rs`).
/// Built by `home/common/pkgs/noto-cjk-subset`; extend that package's
/// `text` before setting a new CJK string, or the sandbox render draws
/// tofu where the host, which has the full face on fontconfig, draws
/// the glyph. Nothing names this family: a Rajdhani run reaches it
/// through the shaper's Han fallback, so load it beside the others.
pub const NOTO_SANS_CJK_JP_BOLD: &[u8] = include_bytes!("../fonts/NotoSansCJKjp-Bold-subset.otf");

// iced::Font constants for easy use
pub const FONT_ORBITRON_REGULAR: iced::Font = iced::Font {
    family: iced::font::Family::Name("Orbitron"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_ORBITRON_MEDIUM: iced::Font = iced::Font {
    family: iced::font::Family::Name("Orbitron"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_ORBITRON_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("Orbitron"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_RAJDHANI_REGULAR: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_RAJDHANI_MEDIUM: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

/// Rajdhani 600. The traces set most chrome at 500 or 600, and a
/// binary that names this weight without loading `RAJDHANI_SEMIBOLD`
/// gets *Bold* from the shaper (CSS matching climbs from 600), which is
/// how the bar strip and every `Face::SemiBold` label came to be drawn
/// a stop off in either direction. Load the bytes wherever this is used.
pub const FONT_RAJDHANI_SEMIBOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_RAJDHANI_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_FREE_SANS_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("FreeSans"),
    // This Bold-named file declares OS/2 weight 600;
    // matching it keeps host fonts from winning the fallback search.
    weight: iced::font::Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_CP_ERAS_KITSCH_SANS_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("CP Eras Kitsch Sans"),
    // The derivative retains GNU FreeSans Bold's OS/2 weight 600.
    weight: iced::font::Weight::Semibold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

/// Give test measurements the same complete font set the app loads at boot.
/// A late load changes global shaping widths between calls in parallel tests.
#[cfg(test)]
pub(crate) fn ensure_test_fonts() {
    use std::sync::OnceLock;

    static LOADED: OnceLock<()> = OnceLock::new();
    LOADED.get_or_init(|| {
        let mut system = iced::advanced::graphics::text::font_system()
            .write().expect("font system");
        for face in crate::shell::faces() {
            system.load_font(face);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::advanced::text::{Alignment, Paragraph as _, Shaping, Wrapping};

    #[test]
    fn free_sans_selector_matches_the_shaped_font_and_declared_weight() {
        ensure_test_fonts();
        // A family-only fontdb query can succeed even when advanced shaping
        // substitutes a different 700 face. Inspect the actual shaped glyphs.
        for shaping in [Shaping::Basic, Shaping::Advanced] {
            let paragraph = iced::advanced::graphics::text::Paragraph::with_text(
                iced::advanced::text::Text {
                    content: "ONLY CC35 MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
                    bounds: iced::Size::INFINITE,
                    size: 16.0.into(),
                    line_height: iced::widget::text::LineHeight::Relative(1.2),
                    font: FONT_FREE_SANS_BOLD,
                    align_x: Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                    shaping,
                    wrapping: Wrapping::None,
                },
            );
            let mut system = iced::advanced::graphics::text::font_system()
                .write().expect("font system");
            let db = system.raw().db();
            let mut count = 0;
            for glyph in paragraph.buffer().layout_runs().flat_map(|run| run.glyphs) {
                let face = db.face(glyph.font_id).expect("shaped face");
                assert_eq!(glyph.font_weight, face.weight);
                assert_eq!(db.with_face_data(glyph.font_id, |data, _| data == FREE_SANS_BOLD), Some(true));
                count += 1;
            }
            assert!(count > 0);
        }
    }

    #[test]
    fn kitsch_derivative_selector_resolves_to_its_own_600_weight_bytes() {
        ensure_test_fonts();
        assert_ne!(CP_ERAS_KITSCH_SANS_BOLD, FREE_SANS_BOLD);
        for shaping in [Shaping::Basic, Shaping::Advanced] {
            let paragraph = iced::advanced::graphics::text::Paragraph::with_text(
                iced::advanced::text::Text {
                    content: "MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
                    bounds: iced::Size::INFINITE,
                    size: 16.0.into(),
                    line_height: iced::widget::text::LineHeight::Relative(1.2),
                    font: FONT_CP_ERAS_KITSCH_SANS_BOLD,
                    align_x: Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                    shaping,
                    wrapping: Wrapping::None,
                },
            );
            let mut system = iced::advanced::graphics::text::font_system()
                .write().expect("font system");
            let db = system.raw().db();
            let mut count = 0;
            for glyph in paragraph.buffer().layout_runs().flat_map(|run| run.glyphs) {
                let face = db.face(glyph.font_id).expect("shaped face");
                assert_eq!(glyph.font_weight.0, 600);
                assert_eq!(glyph.font_weight, face.weight);
                assert_eq!(db.with_face_data(glyph.font_id,
                    |data, _| data == CP_ERAS_KITSCH_SANS_BOLD), Some(true));
                count += 1;
            }
            assert!(count > 0);
        }
    }
}
