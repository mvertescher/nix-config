//! Measured connected dashboard panel echoes. Source img-07-dashboard.png.
//! This reproduces observed pixels without classifying UI versus presentation.
use crate::palette::rgb;
use crate::style::{fill_path, fill_rect, shut_path, Ink, Prim, Seg};

const ECHO_2_CONTOUR: &[Seg] = &[
    Seg::Line(1364.34830, 312.84775),
    Seg::Line(1372.34733, 320.83639),
    Seg::Line(1372.34733, 508.78127),
    Seg::Line(1364.34830, 516.77001),
    Seg::Line(1364.34830, 758.53352),
    Seg::Line(1174.05633, 758.53352),
    Seg::Line(1131.95630, 716.48775),
];
const ECHO_2_BAR: &[Seg] = &[
    Seg::Line(1372.34733, 508.78127),
    Seg::Line(1364.34830, 516.77001),
    Seg::Line(1364.34830, 413.75775),
];
const ECHO_1_CONTOUR: &[Seg] = &[
    Seg::Line(1361.44497, 313.61139),
    Seg::Line(1369.40283, 321.56401),
    Seg::Line(1369.40283, 508.66145),
    Seg::Line(1361.44497, 516.61417),
    Seg::Line(1361.44497, 757.28757),
    Seg::Line(1172.13233, 757.28757),
    Seg::Line(1130.24897, 715.43139),
];
const ECHO_1_BAR: &[Seg] = &[
    Seg::Line(1369.40283, 508.66145),
    Seg::Line(1361.44497, 516.61417),
    Seg::Line(1361.44497, 414.06639),
];

const CYCLE: &[(f32, iced::Color)] = &[
    (0.0, iced::Color { a: 0.0000000, ..rgb(0xffffff) }),
    (0.125, iced::Color { a: 0.1464466, ..rgb(0xffffff) }),
    (0.25, iced::Color { a: 0.5000000, ..rgb(0xffffff) }),
    (0.375, iced::Color { a: 0.8535534, ..rgb(0xffffff) }),
    (0.5, iced::Color { a: 1.0000000, ..rgb(0xffffff) }),
    (0.625, iced::Color { a: 0.8535534, ..rgb(0xffffff) }),
    (0.75, iced::Color { a: 0.5000000, ..rgb(0xffffff) }),
    (0.875, iced::Color { a: 0.1464466, ..rgb(0xffffff) }),
    (1.0, iced::Color { a: 0.0000000, ..rgb(0xffffff) }),
];
// The opaque base and transparent cycle endpoints avoid strip-edge seams.
const fn echo_mask() -> [Prim; 234] {
    let mut rows = [fill_rect(1120.0, 306.0, 260.0, 460.0, Ink::Fixed(rgb(0x909090))); 234];
    let mut i = 1;
    while i < rows.len() {
        rows[i] = Prim::Ramp { x: 1120.0, y: 302.96943500 + (i - 1) as f32 * 1.984934,
            w: 260.0, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops: CYCLE };
        i += 1;
    }
    rows
}
const ECHO_MASK: &[Prim] = &echo_mask();
const ECHO_PATHS: &[Prim] = &[
    shut_path(1131.95630, 312.84775, ECHO_2_CONTOUR, Ink::Fixed(iced::Color { a: 0.023538, ..rgb(0xf93333) }), 2.8),
    shut_path(1131.95630, 312.84775, ECHO_2_CONTOUR, Ink::Fixed(iced::Color { a: 0.042619, ..rgb(0xf93333) }), 2.0),
    shut_path(1131.95630, 312.84775, ECHO_2_CONTOUR, Ink::Fixed(iced::Color { a: 0.094793, ..rgb(0xf93333) }), 1.2),
    shut_path(1131.95630, 312.84775, ECHO_2_CONTOUR, Ink::Fixed(iced::Color { a: 0.064106, ..rgb(0xf93333) }), 0.4),
    fill_path(1372.34733, 405.34852, ECHO_2_BAR, Ink::Fixed(iced::Color { a: 0.3, ..rgb(0xf93333) })),
    shut_path(1130.24897, 313.61139, ECHO_1_CONTOUR, Ink::Fixed(iced::Color { a: 0.056679, ..rgb(0xf93333) }), 2.8),
    shut_path(1130.24897, 313.61139, ECHO_1_CONTOUR, Ink::Fixed(iced::Color { a: 0.060609, ..rgb(0xf93333) }), 2.0),
    shut_path(1130.24897, 313.61139, ECHO_1_CONTOUR, Ink::Fixed(iced::Color { a: 0.146174, ..rgb(0xf93333) }), 1.2),
    shut_path(1130.24897, 313.61139, ECHO_1_CONTOUR, Ink::Fixed(iced::Color { a: 0.093723, ..rgb(0xf93333) }), 0.4),
    fill_path(1369.40283, 405.69507, ECHO_1_BAR, Ink::Fixed(iced::Color { a: 0.39, ..rgb(0xf93333) })),
];
/// Place inside a separate Soft under GO_HOME_OPEN, after PANEL_MATERIAL.
/// Remove the four former solid foreground echo rectangles from GO_HOME.
pub(super) const PANEL_ECHOES: &[Prim] = &[Prim::Masked { prims: ECHO_PATHS, mask: ECHO_MASK }];
