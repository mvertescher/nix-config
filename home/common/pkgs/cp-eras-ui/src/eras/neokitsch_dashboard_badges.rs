//! Source-fitted dashboard section badges; each anchor is measured in this screen.
//! Scene coordinates scale uniformly. Counters use Scene's even-odd path filling.
//! See docs/neokitsch/scene-badges.md for source measurements and remaining limits.
use crate::style::{Ink, Prim, Seg, fill_path, shut_path};
use super::{CAPTION, HUB_MID};

// Independently sampled A/B/C/D lower-tab interiors in the dashboard source.
const BADGE_TAB: iced::Color = crate::palette::rgb(0xfcbe6d);

const DASH_A_OUTLINE: &[Seg] = &[
    Seg::Line(22.5, 0.9),
    Seg::Quad { cx: 25.25, cy: 0.9, x: 25.25, y: 3.9 },
    Seg::Line(25.25, 18.5),
    Seg::Line(18.5, 25.91667),
    Seg::Line(3.5, 25.91667),
    Seg::Quad { cx: 0.5, cy: 25.91667, x: 0.5, y: 22.91667 },
    Seg::Line(0.5, 3.9),
    Seg::Quad { cx: 0.5, cy: 0.9, x: 3.5, y: 0.9 },
];
const DASH_B_OUTLINE: &[Seg] = &[
    Seg::Line(22.16667, 0.9),
    Seg::Quad { cx: 24.91667, cy: 0.9, x: 24.91667, y: 3.9 },
    Seg::Line(24.91667, 18.5),
    Seg::Line(17.75, 25.91667),
    Seg::Line(3.5, 25.91667),
    Seg::Quad { cx: 0.5, cy: 25.91667, x: 0.5, y: 22.91667 },
    Seg::Line(0.5, 3.9),
    Seg::Quad { cx: 0.5, cy: 0.9, x: 3.5, y: 0.9 },
];
const DASH_HEADER_TAB: &[Seg] = &[
    Seg::Line(4.3, 23.5),
    Seg::Quad { cx: 4.5, cy: 23.0, x: 5.0, y: 23.0 },
    Seg::Line(15.0, 23.0),
    Seg::Quad { cx: 15.5, cy: 23.0, x: 15.7, y: 23.5 },
    Seg::Line(16.5, 25.7),
];
const DASH_A_GLYPH: &[Seg] = &[
            Seg::Line(0.71667, -12.58333),
            Seg::Line(7.38333, -0.91667),
            Seg::Line(4.88333, -0.91667),
            Seg::Line(3.21667, -3.83333),
            Seg::Line(-3.45000, -3.83333),
            Seg::Line(-5.11667, -0.91667),
            Seg::Line(-7.61667, -0.91667),
            Seg::Line(-1.36667, -12.58333),
            Seg::Move(-0.11667, -10.08333),
            Seg::Line(-2.61667, -5.50000),
            Seg::Line(2.38333, -5.50000),
            Seg::Line(-0.11667, -10.08333),
];
const DASH_A_GLYPH_START: (f32, f32) = (-1.36667, -12.58333);
const DASH_B_GLYPH: &[Seg] = &[
            Seg::Line(3.33333, -12.58333),
            Seg::Cubic { c1x: 6.25000, c1y: -12.58333, c2x: 7.50000, c2y: -11.33333, x: 7.50000, y: -9.25000 },
            Seg::Cubic { c1x: 7.50000, c1y: -7.58333, c2x: 6.25000, c2y: -6.75000, x: 4.58333, y: -6.33333 },
            Seg::Cubic { c1x: 7.08333, c1y: -5.50000, c2x: 7.50000, c2y: -4.25000, x: 7.50000, y: -2.58333 },
            Seg::Cubic { c1x: 7.50000, c1y: -0.08333, c2x: 6.25000, c2y: 1.16667, x: 3.33333, y: 1.16667 },
            Seg::Line(-7.08333, 1.16667),
            Seg::Line(-7.08333, -12.58333),
            Seg::Move(-4.16667, -10.50000),
            Seg::Line(-4.16667, -7.16667),
            Seg::Line(2.91667, -7.16667),
            Seg::Cubic { c1x: 4.58333, c1y: -7.16667, c2x: 4.58333, c2y: -7.58333, x: 4.58333, y: -8.83333 },
            Seg::Cubic { c1x: 4.58333, c1y: -10.08333, c2x: 4.58333, c2y: -10.50000, x: 2.91667, y: -10.50000 },
            Seg::Line(-4.16667, -10.50000),
            Seg::Move(-4.16667, -5.08333),
            Seg::Line(-4.16667, -0.50000),
            Seg::Line(2.91667, -0.50000),
            Seg::Cubic { c1x: 4.58333, c1y: -0.50000, c2x: 5.00000, c2y: -1.33333, x: 5.00000, y: -2.58333 },
            Seg::Cubic { c1x: 5.00000, c1y: -4.25000, c2x: 4.58333, c2y: -5.08333, x: 2.50000, y: -5.08333 },
            Seg::Line(-4.16667, -5.08333),
];
const DASH_B_GLYPH_START: (f32, f32) = (-7.08333, -12.58333);

/// Two frame paths plus one even-odd glyph; place after the existing mask rect.
pub(super) const DASH_HEADER_A: &[Prim] = &[
    Prim::At { x: 238.75, y: 98.0, prims: &[
        Prim::Path { x: 3.5, y: 0.9, segs: DASH_A_OUTLINE, close: true,
            fill: None, stroke: Some(Ink::Fixed(CAPTION)), width: 1.0 },
        Prim::Path { x: 3.5, y: 25.7, segs: DASH_HEADER_TAB, close: true,
            fill: Some(Ink::Fixed(BADGE_TAB)), stroke: None, width: 0.0 },
    ] },
    Prim::At { x: 250.45, y: 115.5, prims: &[
        Prim::Path { x: DASH_A_GLYPH_START.0, y: DASH_A_GLYPH_START.1,
            segs: DASH_A_GLYPH, close: true,
            fill: Some(Ink::Fixed(CAPTION)), stroke: None, width: 0.0 },
    ] },
];

/// Two frame paths plus one even-odd glyph; place after the existing mask rect.
pub(super) const DASH_HEADER_B: &[Prim] = &[
    Prim::At { x: 1011.75, y: 98.0, prims: &[
        Prim::Path { x: 3.5, y: 0.9, segs: DASH_B_OUTLINE, close: true,
            fill: None, stroke: Some(Ink::Fixed(CAPTION)), width: 1.0 },
        Prim::Path { x: 3.5, y: 25.7, segs: DASH_HEADER_TAB, close: true,
            fill: Some(Ink::Fixed(BADGE_TAB)), stroke: None, width: 0.0 },
    ] },
    Prim::At { x: 1023.25, y: 115.5, prims: &[
        Prim::Path { x: DASH_B_GLYPH_START.0, y: DASH_B_GLYPH_START.1,
            segs: DASH_B_GLYPH, close: true,
            fill: Some(Ink::Fixed(CAPTION)), stroke: None, width: 0.0 },
    ] },
];

const HUB_FOOT_C_FRAME: &[Seg] = &[
    Seg::Line(22.08333, 0.41667),
    Seg::Quad { cx: 25.41667, cy: 0.41667, x: 25.41667, y: 3.75000 },
    Seg::Line(25.41667, 17.91667),
    Seg::Line(18.33333, 25.41667),
    Seg::Line(2.91667, 25.41667),
    Seg::Quad { cx: 0.41667, cy: 25.41667, x: 0.41667, y: 22.50000 },
    Seg::Line(0.41667, 3.75000),
    Seg::Quad { cx: 0.41667, cy: 0.41667, x: 2.91667, y: 0.41667 },
];
// frame start in local design units: (2.91667, 0.41667).

const HUB_FOOT_C_TAB: &[Seg] = &[
    Seg::Line(5.00000, 22.91667),
    Seg::Quad { cx: 5.41667, cy: 22.50000, x: 6.25000, y: 22.50000 },
    Seg::Line(16.66667, 22.50000),
    Seg::Quad { cx: 17.50000, cy: 22.50000, x: 17.91667, y: 23.75000 },
    Seg::Line(18.75000, 25.41667),
    Seg::Line(4.16667, 25.41667),
];
// tab start in local design units: (4.16667, 25.41667).

const HUB_FOOT_C_GLYPH: &[Seg] = &[
    Seg::Line(15.83333, 4.16667),
    Seg::Quad { cx: 19.58333, cy: 4.16667, x: 19.58333, y: 7.50000 },
    Seg::Line(19.58333, 9.16667),
    Seg::Line(17.08333, 9.16667),
    Seg::Line(17.08333, 7.50000),
    Seg::Quad { cx: 17.08333, cy: 6.25000, x: 15.83333, y: 6.25000 },
    Seg::Line(9.16667, 6.25000),
    Seg::Quad { cx: 7.50000, cy: 6.25000, x: 7.50000, y: 7.91667 },
    Seg::Line(7.50000, 14.58333),
    Seg::Quad { cx: 7.50000, cy: 16.25000, x: 9.16667, y: 16.25000 },
    Seg::Line(15.83333, 16.25000),
    Seg::Quad { cx: 17.08333, cy: 16.25000, x: 17.08333, y: 14.58333 },
    Seg::Line(17.08333, 12.91667),
    Seg::Line(19.58333, 12.91667),
    Seg::Line(19.58333, 15.00000),
    Seg::Quad { cx: 19.58333, cy: 18.33333, x: 15.83333, y: 18.33333 },
    Seg::Line(9.16667, 18.33333),
    Seg::Quad { cx: 5.00000, cy: 18.33333, x: 5.00000, y: 14.58333 },
    Seg::Line(5.00000, 7.91667),
    Seg::Quad { cx: 5.00000, cy: 4.16667, x: 9.16667, y: 4.16667 },
];
// glyph start in local design units: (9.16667, 4.16667).

// C source frame origin: 4K (1379,1972); design (574.58333,821.66667).
pub(super) const HUB_FOOT_C_ART: &[Prim] = &[
    shut_path(2.91667, 0.41667, HUB_FOOT_C_FRAME, Ink::Fixed(HUB_MID), 0.95),
    fill_path(4.16667, 25.41667, HUB_FOOT_C_TAB, Ink::Fixed(BADGE_TAB)),
    fill_path(9.16667, 4.16667, HUB_FOOT_C_GLYPH, Ink::Fixed(CAPTION)),
];

// Place with Prim::At { x: 574.58333, y: 821.66667, prims: HUB_FOOT_C_ART }.
// Keep annotation Prim::Text runs and their anchors untouched.

const HUB_FOOT_D_FRAME: &[Seg] = &[
    Seg::Line(22.50000, 0.41667),
    Seg::Quad { cx: 25.83333, cy: 0.41667, x: 25.83333, y: 3.75000 },
    Seg::Line(25.83333, 17.91667),
    Seg::Line(18.75000, 25.83333),
    Seg::Line(2.91667, 25.83333),
    Seg::Quad { cx: 0.41667, cy: 25.83333, x: 0.41667, y: 22.91667 },
    Seg::Line(0.41667, 3.75000),
    Seg::Quad { cx: 0.41667, cy: 0.41667, x: 2.91667, y: 0.41667 },
];
// frame start in local design units: (2.91667, 0.41667).

const HUB_FOOT_D_TAB: &[Seg] = &[
    Seg::Line(5.00000, 22.91667),
    Seg::Quad { cx: 5.41667, cy: 22.50000, x: 6.25000, y: 22.50000 },
    Seg::Line(16.66667, 22.50000),
    Seg::Quad { cx: 17.50000, cy: 22.50000, x: 17.91667, y: 23.75000 },
    Seg::Line(18.75000, 25.83333),
    Seg::Line(4.16667, 25.83333),
];
// tab start in local design units: (4.16667, 25.83333).

const HUB_FOOT_D_GLYPH: &[Seg] = &[
    Seg::Line(11.66667, 4.16667),
    Seg::Cubic { c1x: 17.50000, c1y: 4.16667, c2x: 20.41667, c2y: 7.08333, x: 20.41667, y: 10.83333 },
    Seg::Cubic { c1x: 20.41667, c1y: 15.00000, c2x: 17.50000, c2y: 18.33333, x: 11.66667, y: 18.33333 },
    Seg::Line(4.58333, 18.33333),
    Seg::Line(4.58333, 4.16667),
    Seg::Move(7.08333, 6.25000),
    Seg::Line(7.08333, 15.83333),
    Seg::Line(11.66667, 15.83333),
    Seg::Cubic { c1x: 15.41667, c1y: 15.83333, c2x: 17.50000, c2y: 13.75000, x: 17.50000, y: 10.83333 },
    Seg::Cubic { c1x: 17.50000, c1y: 7.91667, c2x: 15.41667, c2y: 6.25000, x: 11.66667, y: 6.25000 },
    Seg::Line(7.08333, 6.25000),
];
// glyph start in local design units: (4.58333, 4.16667).

// D source frame origin: 4K (2809,1972); design (1170.41667,821.66667).
pub(super) const HUB_FOOT_D_ART: &[Prim] = &[
    shut_path(2.91667, 0.41667, HUB_FOOT_D_FRAME, Ink::Fixed(HUB_MID), 0.95),
    fill_path(4.16667, 25.83333, HUB_FOOT_D_TAB, Ink::Fixed(BADGE_TAB)),
    fill_path(4.58333, 4.16667, HUB_FOOT_D_GLYPH, Ink::Fixed(CAPTION)),
];

// Place with Prim::At { x: 1170.41667, y: 821.66667, prims: HUB_FOOT_D_ART }.
// Keep annotation Prim::Text runs and their anchors untouched.
