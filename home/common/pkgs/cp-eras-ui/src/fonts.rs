pub const ORBITRON_REGULAR: &[u8] = include_bytes!("../fonts/Orbitron-Regular.ttf");
pub const ORBITRON_MEDIUM: &[u8] = include_bytes!("../fonts/Orbitron-Medium.ttf");
pub const ORBITRON_SEMIBOLD: &[u8] = include_bytes!("../fonts/Orbitron-SemiBold.ttf");
pub const ORBITRON_BOLD: &[u8] = include_bytes!("../fonts/Orbitron-Bold.ttf");

pub const RAJDHANI_LIGHT: &[u8] = include_bytes!("../fonts/Rajdhani-Light.ttf");
pub const RAJDHANI_REGULAR: &[u8] = include_bytes!("../fonts/Rajdhani-Regular.ttf");
pub const RAJDHANI_MEDIUM: &[u8] = include_bytes!("../fonts/Rajdhani-Medium.ttf");
pub const RAJDHANI_SEMIBOLD: &[u8] = include_bytes!("../fonts/Rajdhani-SemiBold.ttf");
pub const RAJDHANI_BOLD: &[u8] = include_bytes!("../fonts/Rajdhani-Bold.ttf");

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
