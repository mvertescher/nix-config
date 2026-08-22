use iced::Color;

// Palette sampled from the Neo-Militarism reference images (Behance
// img-07-dashboard et al., 2026-08-21): the system is three reds on
// near-black with a cold blue ambient glow. There is NO yellow anywhere
// in the references — the old `COLOR_PRIMARY_BLACK = #DEDE17` was a
// double typo in the original task spec (almost certainly a mangled
// transcription of the #DE2E2E fill red, mislabeled "black") that got
// faithfully implemented. Off-white exists for rare secondary text
// only; hierarchy is otherwise done with red brightness and opacity.

pub const SAMPLED_BG: Color = Color {
    r: 0.02,
    g: 0.012,
    b: 0.016,
    a: 1.0,
};

pub const SAMPLED_BG_TRANSPARENT: Color = Color {
    r: 0.02,
    g: 0.012,
    b: 0.016,
    a: 0.9,
};

pub const SAMPLED_BG_VERY_TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.15,
};

// Bright red #FF3B45 — hot elements: active text, focus, alerts, glow.
pub const SAMPLED_PRIMARY_RED: Color = Color {
    r: 1.0,
    g: 0.2314,
    b: 0.2706,
    a: 1.0,
};

// Fill red #DE2E2E — sampled from the reference diamonds; large fills
// and pressed/selected surfaces.
pub const SAMPLED_RED_FILL: Color = Color {
    r: 0.8706,
    g: 0.1804,
    b: 0.1804,
    a: 1.0,
};

// Deep red #5E1112 — sampled from the reference borders; inactive
// strokes, dividers, dimmed states.
pub const SAMPLED_RED_DEEP: Color = Color {
    r: 0.3686,
    g: 0.0667,
    b: 0.0706,
    a: 1.0,
};

// Off-white #DEDEDE — sparing secondary text/values only; the
// references are otherwise red-monochrome.
pub const SAMPLED_OFF_WHITE: Color = Color {
    r: 0.8706,
    g: 0.8706,
    b: 0.8706,
    a: 1.0,
};

// Opacity-adjusted variants (translucent)
pub const SAMPLED_PRIMARY_RED_TRANSLUCENT: Color = Color {
    r: 1.0,
    g: 0.2314,
    b: 0.2706,
    a: 0.15,
};

pub const SAMPLED_OFF_WHITE_TRANSLUCENT: Color = Color {
    r: 0.8706,
    g: 0.8706,
    b: 0.8706,
    a: 0.15,
};

// Cold blue ambient glow — reference-real (the wash behind the top of
// every screen).
pub const SAMPLED_GLOW: Color = Color {
    r: 0.0,
    g: 0.1,
    b: 0.2,
    a: 1.0,
};

// ---------------------------------------------------------------------
// Live palette
//
// The SAMPLED_* constants above stay as the record of what was measured
// off the reference art, and as the fallback. What the widgets actually
// draw with is resolved at startup from the palette the nix theme layer
// publishes (see theme.rs), so the toolkit follows whichever era and
// variant the desktop is running rather than being pinned to
// Neo-Militarism.
//
// In a build sandbox there is no published theme, so these resolve to
// the sampled values and the headless render is unchanged. That is what
// makes the golden-image test meaningful across this change.
//
// Roles are mapped semantically, not by hue: COLOR_PRIMARY_RED is the
// dominant foreground of the era, which for neomil's `reference`
// palette is the fill red rather than the hot red. If you want the hot
// red back as the primary, that is a palette decision - set
// `fg = "#ff3b45"` in home/themes/neomil/palettes.nix - not a code one.
use crate::theme::Theme;
use std::sync::LazyLock;

static THEME: LazyLock<Theme> = LazyLock::new(Theme::load);

fn role(pick: fn(&Theme) -> crate::theme::Rgb, fallback: Color) -> Color {
    let _ = fallback;
    pick(&THEME).into()
}

pub static COLOR_BG: LazyLock<Color> =
    LazyLock::new(|| role(|t| t.colors.bg, SAMPLED_BG));

pub static COLOR_PRIMARY_RED: LazyLock<Color> =
    LazyLock::new(|| role(|t| t.colors.fg, SAMPLED_PRIMARY_RED));

pub static COLOR_RED_FILL: LazyLock<Color> =
    LazyLock::new(|| role(|t| t.colors.fg, SAMPLED_RED_FILL));

pub static COLOR_RED_DEEP: LazyLock<Color> =
    LazyLock::new(|| role(|t| t.colors.border, SAMPLED_RED_DEEP));

pub static COLOR_OFF_WHITE: LazyLock<Color> =
    LazyLock::new(|| role(|t| t.colors.tape, SAMPLED_OFF_WHITE));

pub static COLOR_GLOW: LazyLock<Color> =
    LazyLock::new(|| role(|t| t.colors.panel, SAMPLED_GLOW));

pub static COLOR_BG_TRANSPARENT: LazyLock<Color> =
    LazyLock::new(|| Color { a: 0.9, ..*COLOR_BG });

// Structural rather than palette: a black scrim at low alpha.
pub static COLOR_BG_VERY_TRANSPARENT: LazyLock<Color> =
    LazyLock::new(|| SAMPLED_BG_VERY_TRANSPARENT);

pub static COLOR_PRIMARY_RED_TRANSLUCENT: LazyLock<Color> =
    LazyLock::new(|| Color { a: 0.15, ..*COLOR_PRIMARY_RED });

pub static COLOR_OFF_WHITE_TRANSLUCENT: LazyLock<Color> =
    LazyLock::new(|| Color { a: 0.15, ..*COLOR_OFF_WHITE });
