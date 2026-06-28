use iced::Color;

pub const COLOR_BG: Color = Color {
    r: 0.05,
    g: 0.05,
    b: 0.05,
    a: 1.0,
};

// Primary Red: #FF3B45
pub const COLOR_PRIMARY_RED: Color = Color {
    r: 1.0,
    g: 0.2314,
    b: 0.2706,
    a: 1.0,
};

// Primary Black (specified as #DEDE17 in the tasks, which is a bright Cyberpunk yellow)
pub const COLOR_PRIMARY_BLACK: Color = Color {
    r: 0.8706,
    g: 0.8706,
    b: 0.0902,
    a: 1.0,
};

pub const COLOR_YELLOW: Color = COLOR_PRIMARY_BLACK;

// Opacity-adjusted variants (translucent)
pub const COLOR_PRIMARY_RED_TRANSLUCENT: Color = Color {
    r: 1.0,
    g: 0.2314,
    b: 0.2706,
    a: 0.15,
};

pub const COLOR_PRIMARY_BLACK_TRANSLUCENT: Color = Color {
    r: 0.8706,
    g: 0.8706,
    b: 0.0902,
    a: 0.15,
};

// Subtle blue/teal glow color for background
pub const COLOR_GLOW: Color = Color {
    r: 0.0,
    g: 0.1,
    b: 0.2,
    a: 1.0,
};

