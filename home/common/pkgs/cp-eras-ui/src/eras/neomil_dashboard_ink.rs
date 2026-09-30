//! Measured local tape/chip faces and two validated printing modulations.
//! Main tile red, vector contours and the tape mark geometry stay unchanged.
use crate::palette::rgb;
use crate::style::{fill_rect, Ink, Prim};
use super::dashboard_type;
use crate::style::Seg;

pub(super) const LOCAL_FACE: Ink = Ink::Fixed(rgb(0xfb3535));

const fn printing_mask<const N: usize>(prims: &[Prim]) -> [Prim; N] {
    let mut out = [fill_rect(0.0, 0.0, 0.0, 0.0, Ink::Fixed(rgb(0xffffff))); N];
    let mut i = 0;
    while i < N {
        out[i] = match prims[i] {
            Prim::Path { x, y, segs, close, fill, stroke, width } => Prim::Path {
                x, y, segs, close, width,
                fill: match fill { Some(_) => Some(Ink::Fixed(rgb(0xffffff))), None => None },
                stroke: match stroke { Some(_) => Some(Ink::Fixed(rgb(0xffffff))), None => None },
            },
            _ => panic!("printing mask expects accepted paths"),
        };
        i += 1;
    }
    out
}

const fn cycles<const N: usize>(x: f32, first: f32, w: f32, stops: &'static [(f32, iced::Color)]) -> [Prim; N] {
    let mut out = [fill_rect(0.0, 0.0, 0.0, 0.0, LOCAL_FACE); N];
    let mut i = 0;
    while i < N {
        out[i] = Prim::Ramp { x, y: first + i as f32 * 1.984934, w, h: 1.984934,
            from: (0.0, 0.0), to: (0.0, 1.0), stops };
        i += 1;
    }
    out
}

const TAPE_CYCLE: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x8c262e)),
    (0.0625, rgb(0x942d34)),
    (0.125, rgb(0x983037)),
    (0.1875, rgb(0x972d34)),
    (0.25, rgb(0x91252d)),
    (0.3125, rgb(0x861e25)),
    (0.375, rgb(0x7b1c23)),
    (0.4375, rgb(0x702128)),
    (0.5, rgb(0x672b32)),
    (0.5625, rgb(0x63363d)),
    (0.625, rgb(0x623b43)),
    (0.6875, rgb(0x643941)),
    (0.75, rgb(0x693138)),
    (0.8125, rgb(0x6f272e)),
    (0.875, rgb(0x782028)),
    (0.9375, rgb(0x822028)),
    (1.0, rgb(0x8c262e)),
];

const TAPE_FIELD: &[Prim] = &[fill_rect(282.0, 151.0, 94.0, 7.0, Ink::Fixed(rgb(0x8c262e))), Prim::At { x: 0.0, y: 0.0, prims: &cycles::<5>(282.0, 150.854984, 94.0, TAPE_CYCLE) }];

const TAPE_MASK: &[Prim] = &printing_mask::<20>(dashboard_type::TAPE);

const CHIP1_CYCLE: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x991516)),
    (0.0625, rgb(0xa01c1d)),
    (0.125, rgb(0xa31f20)),
    (0.1875, rgb(0xa01d1d)),
    (0.25, rgb(0x991617)),
    (0.3125, rgb(0x8f1011)),
    (0.375, rgb(0x831010)),
    (0.4375, rgb(0x781717)),
    (0.5, rgb(0x6d2223)),
    (0.5625, rgb(0x662d2e)),
    (0.625, rgb(0x633232)),
    (0.6875, rgb(0x652d2e)),
    (0.75, rgb(0x6b2323)),
    (0.8125, rgb(0x761717)),
    (0.875, rgb(0x820f10)),
    (0.9375, rgb(0x8e0f10)),
    (1.0, rgb(0x991516)),
];

const CHIP1_FIELD: &[Prim] = &[fill_rect(59.0, 244.0, 5.0, 11.0, Ink::Fixed(rgb(0x991516))), Prim::At { x: 0.0, y: 0.0, prims: &cycles::<7>(59.0, 242.161948, 5.0, CHIP1_CYCLE) }];

const CHIP1_MASK: &[Prim] = &printing_mask::<1>(dashboard_type::CHIP1);

// Native rows 587..606 of the accepted chip-2 exterior echo, sampled in a
// text-free strip at x3729..3738. The numeral uses that row phase and a
// source-fitted left-to-right red slope; its green/blue ink stays #1b21.
const CHIP2_ECHO_RED: [u8; 20] = [
    80, 62, 63, 84, 92, 77, 60, 68, 88, 90,
    71, 59, 73, 89, 86, 67, 60, 77, 91, 61,
];

const CHIP2_BASE_X: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x4b1b21)), (1.0, rgb(0x751b21)),
];
const CHIP2_BRIGHT_X: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x751b21)), (1.0, rgb(0x9e1b21)),
];

const fn chip2_alpha_stops() -> [(f32, iced::Color); 20] {
    let white = iced::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    let mut out = [(0.0, white); 20];
    let mut i = 0;
    while i < out.len() {
        out[i] = (i as f32 / 19.0, iced::Color {
            a: (CHIP2_ECHO_RED[i] as f32 - 59.0) / 33.0, ..white
        });
        i += 1;
    }
    out
}

const CHIP2_ALPHA_STOPS: [(f32, iced::Color); 20] = chip2_alpha_stops();
// The two horizontal axes are the accepted SVG's x1542.9167..1547.5,
// and the vertical mask axis is y244.7917..252.7083. Full-area ramps
// let the compositor interpolate continuously at every scale.
const CHIP2_BASE: &[Prim] = &[Prim::Ramp {
    x: 1541.0, y: 243.0, w: 9.0, h: 12.0,
    from: ((1542.9167 - 1541.0) / 9.0, 0.0),
    to: ((1547.5 - 1541.0) / 9.0, 0.0), stops: CHIP2_BASE_X,
}];
const CHIP2_BRIGHT: &[Prim] = &[Prim::Ramp {
    x: 1541.0, y: 243.0, w: 9.0, h: 12.0,
    from: ((1542.9167 - 1541.0) / 9.0, 0.0),
    to: ((1547.5 - 1541.0) / 9.0, 0.0), stops: CHIP2_BRIGHT_X,
}];
const CHIP2_ALPHA_MASK: &[Prim] = &[Prim::Ramp {
    x: 1541.0, y: 243.0, w: 9.0, h: 12.0,
    from: (0.0, (244.7917 - 243.0) / 12.0),
    to: (0.0, (252.7083 - 243.0) / 12.0), stops: &CHIP2_ALPHA_STOPS,
}];
const CHIP2_FIELD: &[Prim] = &[
    CHIP2_BASE[0],
    Prim::Masked { prims: CHIP2_BRIGHT, mask: CHIP2_ALPHA_MASK },
];
const CHIP2_MASK: &[Prim] = &printing_mask::<1>(dashboard_type::CHIP2);

pub(super) const TAPE_FACE: Prim = Prim::Path { x: 0.0, y: 0.0, segs: &[
        Seg::Move(258.0, 150.4167),
        Seg::Line(375.4167, 150.4167),
        Seg::Quad { cx: 379.1667, cy: 150.4167, x: 379.1667, y: 154.1667 },
        Seg::Line(379.1667, 156.25),
        Seg::Quad { cx: 379.1667, cy: 160.0, x: 375.4167, y: 160.0 },
        Seg::Line(258.0, 160.0),
        Seg::Quad { cx: 254.5833, cy: 160.0, x: 254.5833, y: 156.25 },
        Seg::Line(254.5833, 154.1667),
        Seg::Quad { cx: 254.5833, cy: 150.4167, x: 258.0, y: 150.4167 },
        Seg::Line(258.0, 150.4167)
    ], close: true, fill: Some(LOCAL_FACE), stroke: None, width: 1.0 };

/// Add to the first Soft after dashboard ground and badge material.
/// Remove only the corresponding tape face/code and both chips' foreground prims.
/// Keep the eight tiny leading tape-mark paths in HEADER; their local ink is fitted there.
pub(super) const MATERIAL: &[Prim] = &[
    TAPE_FACE,
    Prim::Masked { prims: TAPE_FIELD, mask: TAPE_MASK },
    fill_rect(56.25, 243.3333, 12.5, 12.5, LOCAL_FACE),
    fill_rect(46.25, 243.75, 4.5833, 4.5833, LOCAL_FACE),
    Prim::Masked { prims: CHIP1_FIELD, mask: CHIP1_MASK },
    fill_rect(1539.1667, 242.5, 12.5, 12.5, LOCAL_FACE),
    fill_rect(1529.1667, 243.3333, 4.5833, 4.5833, LOCAL_FACE),
    Prim::Masked { prims: CHIP2_FIELD, mask: CHIP2_MASK },
];
