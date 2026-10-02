//! Ordinary MAGNUM 650 impressions from AM's frozen, complete SVG model.
//!
//! The coordinates of the scan windows are 4K source rows divided by 2.4.
//! Each clipped run is the entire shifted title, including its counters;
//! the registered title follows these groups in the card display list.
//! Selected GROWN has a different title grid and receives no ordinary echo.
use super::*;

// Existing canvas feedback technique: constant-opacity Motion preserves the
// live semantic ink. Source/SVG/native review retains the SVG opacities;
// the existing canvas rebase accounts for linear-light blending.
const fn opacity(alpha: f32) -> Motion {
    Motion { id: "store-title-impression", begin: 0, dur: 0,
        ease: Easing::Linear, change: Change::Opacity { alpha: (alpha, alpha) } }
}

const fn title(x: f32, y: f32, ink: Ink) -> Prim {
    Prim::ReusableWide { x, y, size: 22.5, stretch: 0.98, ink,
        face: Face::Bold, anchor: Anchor::Start, content: "MAGNUM 650" }
}

// A Viewport clips one source scan row interval. Its text run is a complete
// impression, so letters and real source printing inside 6/0 remain intact.
macro_rules! row {
    ($first:literal, $last:literal, $alpha:literal, $glyph:expr) => {
        Prim::Viewport {
            x: -20.0, y: $first as f32 / 2.4, w: 324.0,
            h: ($last - $first + 1) as f32 / 2.4,
            prims: &[Prim::Motion { motion: opacity($alpha), prims: $glyph }],
        }
    };
}

macro_rules! main_rows {
    ($glyph:expr) => {&[
        row!(451, 451, 0.18, $glyph),
        row!(452, 453, 0.35, $glyph),
        row!(454, 454, 0.23, $glyph),
        row!(456, 456, 0.25, $glyph),
        row!(457, 458, 0.35, $glyph),
        row!(459, 460, 0.07, $glyph),
        row!(461, 463, 0.35, $glyph),
        row!(464, 465, 0.07, $glyph),
        row!(466, 468, 0.35, $glyph),
        row!(469, 470, 0.07, $glyph),
        row!(471, 473, 0.35, $glyph),
        row!(474, 474, 0.10, $glyph),
        row!(475, 477, 0.35, $glyph),
        row!(478, 479, 0.07, $glyph),
        row!(480, 482, 0.30, $glyph),
        row!(483, 484, 0.07, $glyph),
    ]};
}

macro_rules! card1_echo {
    ($name:ident, $ink:expr) => {
        mod $name {
            use super::*;
            // SVG text baseline 204.5; native primary baseline 203.25.
            // Both shifted copies use the native baseline so their relative
            // registration agrees with the frozen model.
            const LEAD: &[Prim] = &[title(10.2 - 6.0 / 2.4, 203.25 - 12.0 / 2.4, $ink)];
            const MAIN: &[Prim] = &[title(10.2 - 3.0 / 2.4, 203.25 - 6.0 / 2.4, $ink)];
            const LEAD_ROWS: &[Prim] = &[
                row!(443, 443, 0.04, LEAD), row!(444, 444, 0.09, LEAD),
                row!(447, 447, 0.05, LEAD), row!(448, 448, 0.10, LEAD),
                row!(449, 449, 0.15, LEAD),
            ];
            const MAIN_ROWS: &[Prim] = main_rows!(MAIN);
            pub(super) const ALL: &[Prim] = &[
                Prim::At { x: 0.0, y: 0.0, prims: LEAD_ROWS },
                Prim::At { x: 0.0, y: 0.0, prims: MAIN_ROWS },
            ];
        }
    };
}
macro_rules! card3_echo {
    ($name:ident, $ink:expr) => {
        mod $name {
            use super::*;
            const LEAD: &[Prim] = &[title(8.9 + 5.0 / 2.4, 203.25 - 9.0 / 2.4, $ink)];
            const MAIN: &[Prim] = &[title(8.9 + 3.0 / 2.4, 203.25 - 7.0 / 2.4, $ink)];
            const LEAD_ROWS: &[Prim] = &[
                row!(447, 447, 0.02, LEAD), row!(448, 448, 0.06, LEAD),
                row!(449, 449, 0.10, LEAD),
            ];
            const MAIN_ROWS: &[Prim] = main_rows!(MAIN);
            pub(super) const ALL: &[Prim] = &[
                Prim::At { x: 0.0, y: 0.0, prims: LEAD_ROWS },
                Prim::At { x: 0.0, y: 0.0, prims: MAIN_ROWS },
            ];
        }
    };
}
macro_rules! card4_echo {
    ($name:ident, $ink:expr) => {
        mod $name {
            use super::*;
            const EARLY: &[Prim] = &[title(8.1 - 3.0 / 2.4, 203.25 - 15.0 / 2.4, $ink)];
            const LEAD: &[Prim] = &[title(8.1 + 8.0 / 2.4, 203.25 - 12.0 / 2.4, $ink)];
            const MAIN: &[Prim] = &[title(8.1 + 9.0 / 2.4, 203.25 - 7.0 / 2.4, $ink)];
            const LEAD_ROWS: &[Prim] = &[
                row!(443, 443, 0.04, EARLY), row!(444, 444, 0.08, EARLY),
                row!(447, 447, 0.04, LEAD), row!(448, 448, 0.10, LEAD),
                row!(449, 449, 0.17, LEAD),
            ];
            const MAIN_ROWS: &[Prim] = main_rows!(MAIN);
            pub(super) const ALL: &[Prim] = &[
                Prim::At { x: 0.0, y: 0.0, prims: LEAD_ROWS },
                Prim::At { x: 0.0, y: 0.0, prims: MAIN_ROWS },
            ];
        }
    };
}

card1_echo!(one, Ink::Fg);
card3_echo!(three, Ink::Fg);
card4_echo!(four, Ink::Fg);
card1_echo!(one_held, card_ink(Ink::Fg, true));
card3_echo!(three_held, card_ink(Ink::Fg, true));
card4_echo!(four_held, card_ink(Ink::Fg, true));

pub(super) const CARD1: &[Prim] = one::ALL;
pub(super) const CARD3: &[Prim] = three::ALL;
pub(super) const CARD4: &[Prim] = four::ALL;
pub(super) const CARD1_HELD: &[Prim] = one_held::ALL;
pub(super) const CARD3_HELD: &[Prim] = three_held::ALL;
pub(super) const CARD4_HELD: &[Prim] = four_held::ALL;
