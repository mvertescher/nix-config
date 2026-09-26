//! Source-local copies of the dashboard's three margin printing groups.
//! The primary glyph paths remain owned by dashboard_type and are reused.
use crate::palette::rgb;
use crate::style::{fill_rect, Ink, Prim};
use super::dashboard_type;
const RED: Ink = Ink::Fixed(rgb(0xef3333));

const fn profile<const N: usize>(source: &[Prim], extra: f32) -> [Prim; N] {
    let paths = match source[0] {
        Prim::Turn { prims, .. } => prims,
        _ => panic!("margin printing expects one accepted rotated group"),
    };
    let mut out = [fill_rect(0.0, 0.0, 0.0, 0.0, RED); N];
    let mut i = 0;
    while i < N {
        out[i] = match paths[i] {
            Prim::Path { x, y, segs, close, width, .. } => Prim::Path {
                x, y, segs, close, fill: None, stroke: Some(RED), width: width + extra,
            },
            _ => panic!("margin printing expects accepted stroke paths"),
        };
        i += 1;
    }
    out
}
const fn turn(source: &[Prim], children: &'static [Prim]) -> Prim {
    match source[0] {
        Prim::Turn { x, y, angle, .. } => Prim::Turn { x, y, angle, prims: children },
        _ => panic!("margin printing expects one accepted rotated group"),
    }
}
const fn alpha_mask(b: (f32, f32, f32, f32), a: f32) -> Prim {
    fill_rect(b.0, b.1, b.2 - b.0, b.3 - b.1,
        Ink::Fixed(iced::Color { a, ..rgb(0xffffff) }))
}
macro_rules! layer {
    ($src:expr, $count:expr, $extra:expr, $alpha:expr, $bounds:expr) => {
        Prim::Masked { prims: &[turn($src, &profile::<$count>($src, $extra))],
            mask: &[alpha_mask($bounds, $alpha)] }
    };
}
const CYCLE: &[(f32, iced::Color)] = &[
    (0.0, iced::Color { a: 0.0, ..rgb(0xffffff) }),
    (0.125, iced::Color { a: 0.1464466, ..rgb(0xffffff) }),
    (0.25, iced::Color { a: 0.5, ..rgb(0xffffff) }),
    (0.375, iced::Color { a: 0.8535534, ..rgb(0xffffff) }),
    (0.5, iced::Color { a: 1.0, ..rgb(0xffffff) }),
    (0.625, iced::Color { a: 0.8535534, ..rgb(0xffffff) }),
    (0.75, iced::Color { a: 0.5, ..rgb(0xffffff) }),
    (0.875, iced::Color { a: 0.1464466, ..rgb(0xffffff) }),
    (1.0, iced::Color { a: 0.0, ..rgb(0xffffff) }),
];
// Explicit transparent-ended cycles over an opaque gray floor avoid seams.
const fn scan<const N: usize>(b: (f32, f32, f32, f32), start: f32, floor: u32) -> [Prim; N] {
    let gray = (floor << 16) | (floor << 8) | floor;
    let mut out = [fill_rect(b.0, b.1, b.2 - b.0, b.3 - b.1, Ink::Fixed(rgb(gray))); N];
    let mut i = 1;
    while i < N {
        out[i] = Prim::Ramp { x: b.0, y: start + (i - 1) as f32 * 1.984934,
            w: b.2 - b.0, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops: CYCLE };
        i += 1;
    }
    out
}
const LEFT_CODE_BOUNDS: (f32, f32, f32, f32) = (34.0, 454.0, 57.0, 583.0);
const LEFT_CODE_SCAN: &[Prim] = &scan::<68>(LEFT_CODE_BOUNDS, 453.8432265, 155);
const LEFT_CODE_ECHOES: &[Prim] = &[
    Prim::Masked { mask: LEFT_CODE_SCAN, prims: &[Prim::At { x: -7.230684, y: 0.331292, prims: &[
        layer!(dashboard_type::LEFT_CODE, 19, 2.4, 0.034424, LEFT_CODE_BOUNDS),
        layer!(dashboard_type::LEFT_CODE, 19, 1.2, 0.060979, LEFT_CODE_BOUNDS),
        layer!(dashboard_type::LEFT_CODE, 19, 0.4, 0.028837, LEFT_CODE_BOUNDS),
        layer!(dashboard_type::LEFT_CODE, 19, 0.0, 0.033413, LEFT_CODE_BOUNDS),
    ] }] },
    Prim::Masked { mask: LEFT_CODE_SCAN, prims: &[Prim::At { x: -3.615342, y: 0.165646, prims: &[
        layer!(dashboard_type::LEFT_CODE, 19, 2.4, 0.011363, LEFT_CODE_BOUNDS),
        layer!(dashboard_type::LEFT_CODE, 19, 1.2, 0.058298, LEFT_CODE_BOUNDS),
        layer!(dashboard_type::LEFT_CODE, 19, 0.4, 0.115865, LEFT_CODE_BOUNDS),
        layer!(dashboard_type::LEFT_CODE, 19, 0.0, 0.148096, LEFT_CODE_BOUNDS),
    ] }] },
];
const RIGHT_CODE_BOUNDS: (f32, f32, f32, f32) = (1538.0, 548.0, 1567.0, 676.0);
const RIGHT_CODE_SCAN: &[Prim] = &scan::<67>(RIGHT_CODE_BOUNDS, 547.1163111, 145);
const RIGHT_CODE_ECHOES: &[Prim] = &[
    Prim::Masked { mask: RIGHT_CODE_SCAN, prims: &[Prim::At { x: 7.207886, y: 1.194412, prims: &[
        layer!(dashboard_type::RIGHT_CODE, 19, 2.4, 0.032898, RIGHT_CODE_BOUNDS),
        layer!(dashboard_type::RIGHT_CODE, 19, 1.2, 0.054090, RIGHT_CODE_BOUNDS),
        layer!(dashboard_type::RIGHT_CODE, 19, 0.4, 0.074399, RIGHT_CODE_BOUNDS),
    ] }] },
    Prim::Masked { mask: RIGHT_CODE_SCAN, prims: &[Prim::At { x: 3.603943, y: 0.597206, prims: &[
        layer!(dashboard_type::RIGHT_CODE, 19, 2.4, 0.010717, RIGHT_CODE_BOUNDS),
        layer!(dashboard_type::RIGHT_CODE, 19, 1.2, 0.071648, RIGHT_CODE_BOUNDS),
        layer!(dashboard_type::RIGHT_CODE, 19, 0.4, 0.154202, RIGHT_CODE_BOUNDS),
        layer!(dashboard_type::RIGHT_CODE, 19, 0.0, 0.113876, RIGHT_CODE_BOUNDS),
    ] }] },
];
const KIROSHI_BOUNDS: (f32, f32, f32, f32) = (1538.0, 719.0, 1567.0, 772.0);
const KIROSHI_SCAN: &[Prim] = &scan::<30>(KIROSHI_BOUNDS, 717.8126928, 151);
const KIROSHI_ECHOES: &[Prim] = &[
    Prim::Masked { mask: KIROSHI_SCAN, prims: &[Prim::At { x: 7.216829, y: 2.515455, prims: &[
        layer!(dashboard_type::KIROSHI, 7, 2.4, 0.057858, KIROSHI_BOUNDS),
        layer!(dashboard_type::KIROSHI, 7, 1.2, 0.054275, KIROSHI_BOUNDS),
        layer!(dashboard_type::KIROSHI, 7, 0.4, 0.046079, KIROSHI_BOUNDS),
        layer!(dashboard_type::KIROSHI, 7, 0.0, 0.011443, KIROSHI_BOUNDS),
    ] }] },
    Prim::Masked { mask: KIROSHI_SCAN, prims: &[Prim::At { x: 3.608415, y: 1.257728, prims: &[
        layer!(dashboard_type::KIROSHI, 7, 2.4, 0.068681, KIROSHI_BOUNDS),
        layer!(dashboard_type::KIROSHI, 7, 1.2, 0.049302, KIROSHI_BOUNDS),
        layer!(dashboard_type::KIROSHI, 7, 0.4, 0.122157, KIROSHI_BOUNDS),
        layer!(dashboard_type::KIROSHI, 7, 0.0, 0.138684, KIROSHI_BOUNDS),
    ] }] },
];

/// Add to the first dashboard Soft after its ground. Primary margin
/// printing remains later in dashboard_chrome::MARGINS.
pub(super) const ECHOES: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: LEFT_CODE_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: RIGHT_CODE_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: KIROSHI_ECHOES },
];
