use iced::Color;

// Palette sampled from the Neo-Militarism reference images (Behance
// img-07-dashboard et al., 2026-08-21): the system is three reds on
// near-black with a cold blue ambient glow. There is NO yellow anywhere
// in the references — the old `COLOR_PRIMARY_BLACK = #DEDE17` was a
// double typo in the original task spec (almost certainly a mangled
// transcription of the #DE2E2E fill red, mislabeled "black") that got
// faithfully implemented. Off-white exists for rare secondary text
// only; hierarchy is otherwise done with red brightness and opacity.

pub const COLOR_BG: Color = Color {
    r: 0.02,
    g: 0.012,
    b: 0.016,
    a: 1.0,
};

pub const COLOR_BG_TRANSPARENT: Color = Color {
    r: 0.02,
    g: 0.012,
    b: 0.016,
    a: 0.9,
};

pub const COLOR_BG_VERY_TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.15,
};

// Bright red #FF3B45 — hot elements: active text, focus, alerts, glow.
pub const COLOR_PRIMARY_RED: Color = Color {
    r: 1.0,
    g: 0.2314,
    b: 0.2706,
    a: 1.0,
};

// Fill red #DE2E2E — sampled from the reference diamonds; large fills
// and pressed/selected surfaces.
pub const COLOR_RED_FILL: Color = Color {
    r: 0.8706,
    g: 0.1804,
    b: 0.1804,
    a: 1.0,
};

// Deep red #5E1112 — sampled from the reference borders; inactive
// strokes, dividers, dimmed states.
pub const COLOR_RED_DEEP: Color = Color {
    r: 0.3686,
    g: 0.0667,
    b: 0.0706,
    a: 1.0,
};

// Off-white #DEDEDE — sparing secondary text/values only; the
// references are otherwise red-monochrome.
pub const COLOR_OFF_WHITE: Color = Color {
    r: 0.8706,
    g: 0.8706,
    b: 0.8706,
    a: 1.0,
};

// Opacity-adjusted variants (translucent)
pub const COLOR_PRIMARY_RED_TRANSLUCENT: Color = Color {
    r: 1.0,
    g: 0.2314,
    b: 0.2706,
    a: 0.15,
};

pub const COLOR_OFF_WHITE_TRANSLUCENT: Color = Color {
    r: 0.8706,
    g: 0.8706,
    b: 0.8706,
    a: 0.15,
};

// Cold blue ambient glow — reference-real (the wash behind the top of
// every screen).
pub const COLOR_GLOW: Color = Color {
    r: 0.0,
    g: 0.1,
    b: 0.2,
    a: 1.0,
};
