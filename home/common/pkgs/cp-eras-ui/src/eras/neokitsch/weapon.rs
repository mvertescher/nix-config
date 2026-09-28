//! NeoKitsch placement of the shared source-native MAGNUM contours.
//!
//! Both plain and raised cards print the same weapon. The source fits the
//! Entropism-local contours at 0.99 scale, offset (-8.5, -67.5) within the
//! card. The parent adds its own top y (345.0 or 263.3); no contour data
//! are copied into this module.
use crate::style::{Ink, Prim, Seg};
use super::super::magnum_art::{RIFLE, RIFLE_BRIGHT};

const SCALE: f32 = 0.99;
const DX: f32 = -8.5;
const DY: f32 = -67.5;
const fn xy(x: f32, y: f32) -> (f32, f32) { (x * SCALE + DX, y * SCALE + DY) }

const fn fitted<const N: usize>(source: &[Seg; N]) -> [Seg; N] {
    let mut out = *source;
    let mut i = 0;
    while i < N {
        out[i] = match source[i] {
            Seg::Move(x, y) => { let (x, y) = xy(x, y); Seg::Move(x, y) },
            Seg::Line(x, y) => { let (x, y) = xy(x, y); Seg::Line(x, y) },
            Seg::Quad { cx, cy, x, y } => {
                let (cx, cy) = xy(cx, cy); let (x, y) = xy(x, y);
                Seg::Quad { cx, cy, x, y }
            },
            Seg::Cubic { c1x, c1y, c2x, c2y, x, y } => {
                let (c1x, c1y) = xy(c1x, c1y);
                let (c2x, c2y) = xy(c2x, c2y);
                let (x, y) = xy(x, y);
                Seg::Cubic { c1x, c1y, c2x, c2y, x, y }
            },
        };
        i += 1;
    }
    out
}

const BASE: &[Seg] = &fitted(RIFLE);
const BRIGHT: &[Seg] = &fitted(RIFLE_BRIGHT);
const BASE_START: (f32, f32) = xy(237.833, 131.667);
const BRIGHT_START: (f32, f32) = xy(78.25, 105.833);

pub(super) const PATHS: &[Prim] = &[
    Prim::Path {
        x: BASE_START.0, y: BASE_START.1, segs: BASE, close: true,
        fill: Some(Ink::Fixed(super::GUN_BASE)), stroke: None, width: 0.0,
    },
    Prim::Path {
        x: BRIGHT_START.0, y: BRIGHT_START.1, segs: BRIGHT, close: true,
        fill: Some(Ink::Fixed(super::GUN_BRIGHT)), stroke: None, width: 0.0,
    },
];
