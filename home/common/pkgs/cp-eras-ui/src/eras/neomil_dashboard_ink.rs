//! Measured local tape/chip faces and two validated printing modulations.
//! Main tile red, vector contours and the unreadable tape mark stay unchanged.
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
/// Remove only the corresponding tape face/code and chip-1 foreground prims.
/// Keep the eight tiny leading tape-mark paths in HEADER unchanged.
pub(super) const MATERIAL: &[Prim] = &[
    TAPE_FACE,
    Prim::Masked { prims: TAPE_FIELD, mask: TAPE_MASK },
    fill_rect(56.25, 243.3333, 12.5, 12.5, LOCAL_FACE),
    fill_rect(46.25, 243.75, 4.5833, 4.5833, LOCAL_FACE),
    Prim::Masked { prims: CHIP1_FIELD, mask: CHIP1_MASK },
];
