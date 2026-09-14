//! Local exterior copies for the dashboard tape and numbered chips.
//! Accepted primary faces and printing remain unchanged. Finite profiles
//! reconstruct source trails without identifying their original renderer.
use crate::palette::rgb;
use crate::style::{fill_rect, Ink, Prim};
use super::dashboard_ink::{LOCAL_FACE, TAPE_FACE};

const fn profile(shape: Prim, width: f32) -> Prim {
    let stroke = if width > 0.0 { Some(LOCAL_FACE) } else { None };
    match shape {
        Prim::Path { x, y, segs, close, .. } => Prim::Path {
            x, y, segs, close, fill: Some(LOCAL_FACE), stroke, width,
        },
        Prim::Round { x, y, w, h, r, .. } => Prim::Round {
            x, y, w, h, r, fill: Some(LOCAL_FACE), stroke, width,
        },
        _ => panic!("tape/chip profile expects a face path or rounded rectangle"),
    }
}

const fn square(x: f32, y: f32, side: f32) -> Prim {
    // Round with r=0 keeps the accepted square. Its software stroke has
    // the round joins used by the exterior profile in the SVG.
    Prim::Round { x, y, w: side, h: side, r: 0.0,
        fill: Some(LOCAL_FACE), stroke: None, width: 0.0 }
}
const CHIP1: Prim = square(56.25, 243.3333, 12.5);
const FRAGMENT1: Prim = square(46.25, 243.75, 4.5833);
const CHIP2: Prim = square(1539.1667, 242.5, 12.5);
const FRAGMENT2: Prim = square(1529.1667, 243.3333, 4.5833);

const fn alpha_mask(b: (f32, f32, f32, f32), a: f32) -> Prim {
    fill_rect(b.0, b.1, b.2 - b.0, b.3 - b.1,
        Ink::Fixed(iced::Color { a, ..rgb(0xffffff) }))
}
macro_rules! layer {
    ($shape:ident, $width:expr, $alpha:expr, $bounds:expr) => {
        // Apply alpha once to the union of opaque fill and stroke.
        Prim::Masked { prims: &[profile($shape, $width)],
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
// A continuous opaque gray floor under transparent-ended ramp rows keeps
// fractional-scale scan masks free from interior antialiasing gaps.
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
const TAPE_BOUNDS: (f32, f32, f32, f32) = (242.0, 143.0, 387.0, 164.0);
const TAPE_SCAN: &[Prim] = &scan::<14>(TAPE_BOUNDS, 142.1423765, 134);
const TAPE_ECHOES: &[Prim] = &[
    Prim::Masked { mask: TAPE_SCAN, prims: &[Prim::At { x: -2.998308, y: -3.037574, prims: &[
        layer!(TAPE_FACE, 1.6, 0.057246, TAPE_BOUNDS),
        layer!(TAPE_FACE, 0.6, 0.060697, TAPE_BOUNDS),
        layer!(TAPE_FACE, 0.0, 0.173917, TAPE_BOUNDS),
    ] }] },
    Prim::Masked { mask: TAPE_SCAN, prims: &[Prim::At { x: -1.499154, y: -1.518787, prims: &[
        layer!(TAPE_FACE, 1.6, 0.127421, TAPE_BOUNDS),
        layer!(TAPE_FACE, 0.0, 0.308552, TAPE_BOUNDS),
    ] }] },
];
const CHIP1_BOUNDS: (f32, f32, f32, f32) = (44.0, 233.0, 74.0, 260.0);
const CHIP1_SCAN: &[Prim] = &scan::<17>(CHIP1_BOUNDS, 231.4822541, 123);
const CHIP1_ECHOES: &[Prim] = &[
    Prim::Masked { mask: CHIP1_SCAN, prims: &[Prim::At { x: -7.062409, y: -2.640003, prims: &[
        layer!(CHIP1, 3.0, 0.003279, CHIP1_BOUNDS),
        layer!(CHIP1, 1.6, 0.070403, CHIP1_BOUNDS),
        layer!(CHIP1, 0.0, 0.232920, CHIP1_BOUNDS),
    ] }] },
    Prim::Masked { mask: CHIP1_SCAN, prims: &[Prim::At { x: -3.531205, y: -1.320002, prims: &[
        layer!(CHIP1, 1.6, 0.100008, CHIP1_BOUNDS),
        layer!(CHIP1, 0.6, 0.186838, CHIP1_BOUNDS),
        layer!(CHIP1, 0.0, 0.186478, CHIP1_BOUNDS),
    ] }] },
];
const FRAGMENT1_BOUNDS: (f32, f32, f32, f32) = (32.0, 236.0, 53.0, 253.0);
const FRAGMENT1_SCAN: &[Prim] = &scan::<11>(FRAGMENT1_BOUNDS, 235.4359201, 117);
const FRAGMENT1_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAGMENT1_SCAN, prims: &[Prim::At { x: -6.799023, y: -2.745916, prims: &[
        layer!(FRAGMENT1, 1.6, 0.096695, FRAGMENT1_BOUNDS),
        layer!(FRAGMENT1, 0.6, 0.061219, FRAGMENT1_BOUNDS),
        layer!(FRAGMENT1, 0.0, 0.233557, FRAGMENT1_BOUNDS),
    ] }] },
    Prim::Masked { mask: FRAGMENT1_SCAN, prims: &[Prim::At { x: -3.399511, y: -1.372958, prims: &[
        layer!(FRAGMENT1, 3.0, 0.024610, FRAGMENT1_BOUNDS),
        layer!(FRAGMENT1, 1.6, 0.057156, FRAGMENT1_BOUNDS),
        layer!(FRAGMENT1, 0.6, 0.261022, FRAGMENT1_BOUNDS),
        layer!(FRAGMENT1, 0.0, 0.190874, FRAGMENT1_BOUNDS),
    ] }] },
];
const CHIP2_BOUNDS: (f32, f32, f32, f32) = (1535.0, 234.0, 1566.0, 261.0);
const CHIP2_SCAN: &[Prim] = &scan::<16>(CHIP2_BOUNDS, 233.4806853, 156);
const CHIP2_ECHOES: &[Prim] = &[
    Prim::Masked { mask: CHIP2_SCAN, prims: &[Prim::At { x: 6.823151, y: -2.266915, prims: &[
        layer!(CHIP2, 3.0, 0.000255, CHIP2_BOUNDS),
        layer!(CHIP2, 1.6, 0.092269, CHIP2_BOUNDS),
        layer!(CHIP2, 0.6, 0.009124, CHIP2_BOUNDS),
        layer!(CHIP2, 0.0, 0.263313, CHIP2_BOUNDS),
    ] }] },
    Prim::Masked { mask: CHIP2_SCAN, prims: &[Prim::At { x: 3.411576, y: -1.133458, prims: &[
        layer!(CHIP2, 3.0, 0.021289, CHIP2_BOUNDS),
        layer!(CHIP2, 1.6, 0.023981, CHIP2_BOUNDS),
        layer!(CHIP2, 0.6, 0.286683, CHIP2_BOUNDS),
        layer!(CHIP2, 0.0, 0.012583, CHIP2_BOUNDS),
    ] }] },
];
const FRAGMENT2_BOUNDS: (f32, f32, f32, f32) = (1525.0, 235.0, 1543.0, 252.0);
const FRAGMENT2_SCAN: &[Prim] = &scan::<12>(FRAGMENT2_BOUNDS, 233.5511643, 145);
const FRAGMENT2_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAGMENT2_SCAN, prims: &[Prim::At { x: 7.023760, y: -3.874016, prims: &[
        layer!(FRAGMENT2, 3.0, 0.030336, FRAGMENT2_BOUNDS),
        layer!(FRAGMENT2, 1.6, 0.255572, FRAGMENT2_BOUNDS),
        layer!(FRAGMENT2, 0.6, 0.044480, FRAGMENT2_BOUNDS),
    ] }] },
    Prim::Masked { mask: FRAGMENT2_SCAN, prims: &[Prim::At { x: 3.511880, y: -1.937008, prims: &[
        layer!(FRAGMENT2, 1.6, 0.054037, FRAGMENT2_BOUNDS),
        layer!(FRAGMENT2, 0.6, 0.241366, FRAGMENT2_BOUNDS),
        layer!(FRAGMENT2, 0.0, 0.276876, FRAGMENT2_BOUNDS),
    ] }] },
];

/// Place inside the dashboard's first Soft after the ground, BEFORE
/// dashboard_ink::MATERIAL. The opaque primary faces cover these copies.
pub(super) const ECHOES: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: TAPE_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: CHIP1_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAGMENT1_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: CHIP2_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAGMENT2_ECHOES },
];
