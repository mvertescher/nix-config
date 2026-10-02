//! Independently fitted store A/B/C contours, chamfers and lower tabs.
//! Local coordinates retain the original scene anchors and uniform scaling.
//! See docs/neokitsch/scene-badges.md for source measurements and remaining limits.
use crate::style::{Ink, Prim, Seg};
use super::{STORE_WIRE, TAB, BRIGHT};

pub(super) static STORE_A_BADGE: [Prim; 3] = [
    // A frame; local design coordinates relative to (360.0, 143.0).
    Prim::Path {
        x: 2.0833, y: 0.9583,
        segs: &[
            Seg::Line(24.5834, 0.9583),
            Seg::Quad { cx: 25.8334, cy: 0.9583, x: 25.8334, y: 2.4167 },
            Seg::Line(25.8334, 19.0833),
            Seg::Line(19.1667, 25.5417),
            Seg::Quad { cx: 18.3333, cy: 25.9583, x: 17.0833, y: 25.9583 },
            Seg::Line(2.0833, 25.9583),
            Seg::Quad { cx: 0.8333, cy: 25.9583, x: 0.8333, y: 24.0833 },
            Seg::Line(0.8333, 2.4167),
            Seg::Quad { cx: 0.8333, cy: 0.9583, x: 2.0833, y: 0.9583 },
        ],
        close: true, fill: None, stroke: Some(Ink::Fixed(STORE_WIRE)), width: 1.0,
    },
    // A tab; local design coordinates relative to (360.0, 143.0).
    Prim::Path {
        x: 5.0, y: 22.8333,
        segs: &[
            Seg::Line(15.8333, 22.8333),
            Seg::Line(16.25, 24.5),
            Seg::Line(18.75, 26.0833),
            Seg::Line(2.0833, 26.0833),
            Seg::Line(4.1667, 24.5),
        ],
        close: true, fill: Some(Ink::Fixed(TAB)), stroke: None, width: 0.0,
    },
    // A glyph; local design coordinates relative to (360.0, 143.0).
    Prim::Path {
        x: 10.8333, y: 5.3333,
        segs: &[
            Seg::Line(12.9167, 5.3333),
            Seg::Quad { cx: 13.75, cy: 5.3333, x: 14.1667, y: 6.5833 },
            Seg::Line(19.5833, 16.5833),
            Seg::Line(17.0833, 16.5833),
            Seg::Line(15.8333, 14.0833),
            Seg::Line(7.5, 14.0833),
            Seg::Line(6.6667, 16.5833),
            Seg::Line(4.5833, 16.5833),
            Seg::Move(11.6667, 7.0),
            Seg::Line(14.5833, 12.4167),
            Seg::Line(9.1667, 12.4167),
        ],
        close: true, fill: Some(Ink::Fixed(BRIGHT)), stroke: None, width: 0.0,
    },
];


pub(super) static STORE_B_BADGE: [Prim; 3] = [
    // B frame; local design coordinates relative to (675.0, 775.0).
    Prim::Path {
        x: 1.6667, y: 1.25,
        segs: &[
            Seg::Line(23.75, 1.25),
            Seg::Quad { cx: 25.0, cy: 1.25, x: 25.0, y: 2.9167 },
            Seg::Line(25.0, 20.0),
            Seg::Line(18.3333, 26.0417),
            Seg::Quad { cx: 17.5, cy: 26.4583, x: 16.25, y: 26.4583 },
            Seg::Line(1.6667, 26.4583),
            Seg::Quad { cx: 0.4167, cy: 26.4583, x: 0.4167, y: 24.5833 },
            Seg::Line(0.4167, 2.9167),
            Seg::Quad { cx: 0.4167, cy: 1.25, x: 1.6667, y: 1.25 },
        ],
        close: true, fill: None, stroke: Some(Ink::Fixed(STORE_WIRE)), width: 1.0,
    },
    // B tab; local design coordinates relative to (675.0, 775.0).
    Prim::Path {
        x: 4.1667, y: 23.75,
        segs: &[
            Seg::Line(15.0, 23.75),
            Seg::Line(15.4167, 25.4167),
            Seg::Line(18.3333, 26.5833),
            Seg::Line(0.8333, 26.5833),
            Seg::Line(2.9167, 25.4167),
        ],
        close: true, fill: Some(Ink::Fixed(TAB)), stroke: None, width: 0.0,
    },
    // B glyph; local design coordinates relative to (675.0, 775.0).
    Prim::Path {
        x: 4.5833, y: 4.5833,
        segs: &[
            Seg::Line(15.4167, 4.5833),
            Seg::Quad { cx: 18.75, cy: 4.5833, x: 18.75, y: 7.5 },
            Seg::Line(18.75, 9.1667),
            Seg::Quad { cx: 18.75, cy: 11.25, x: 17.0833, y: 11.6667 },
            Seg::Quad { cx: 18.75, cy: 12.5, x: 18.75, y: 14.5833 },
            Seg::Line(18.75, 15.8333),
            Seg::Quad { cx: 18.75, cy: 18.3333, x: 16.25, y: 18.3333 },
            Seg::Line(4.5833, 18.3333),
            Seg::Move(7.0833, 6.6667),
            Seg::Line(15.4167, 6.6667),
            Seg::Quad { cx: 16.6667, cy: 6.6667, x: 16.6667, y: 8.3333 },
            Seg::Quad { cx: 16.6667, cy: 10.4167, x: 15.4167, y: 10.4167 },
            Seg::Line(7.0833, 10.4167),
            Seg::Move(7.0833, 12.0833),
            Seg::Line(15.4167, 12.0833),
            Seg::Quad { cx: 16.6667, cy: 12.0833, x: 16.6667, y: 13.75 },
            Seg::Line(16.6667, 15.0),
            Seg::Quad { cx: 16.6667, cy: 16.6667, x: 15.4167, y: 16.6667 },
            Seg::Line(7.0833, 16.6667),
        ],
        close: true, fill: Some(Ink::Fixed(BRIGHT)), stroke: None, width: 0.0,
    },
];


pub(super) static STORE_C_BADGE: [Prim; 3] = [
    // C frame; local design coordinates relative to (1178.0, 143.0).
    Prim::Path {
        x: 2.4167, y: 0.9583,
        segs: &[
            Seg::Line(24.9167, 0.9583),
            Seg::Quad { cx: 26.1667, cy: 0.9583, x: 26.1667, y: 2.4167 },
            Seg::Line(26.1667, 19.0833),
            Seg::Line(19.5, 25.5417),
            Seg::Quad { cx: 18.6667, cy: 25.9583, x: 17.4167, y: 25.9583 },
            Seg::Line(2.4167, 25.9583),
            Seg::Quad { cx: 1.1667, cy: 25.9583, x: 1.1667, y: 24.0833 },
            Seg::Line(1.1667, 2.4167),
            Seg::Quad { cx: 1.1667, cy: 0.9583, x: 2.4167, y: 0.9583 },
        ],
        close: true, fill: None, stroke: Some(Ink::Fixed(STORE_WIRE)), width: 1.0,
    },
    // C tab; local design coordinates relative to (1178.0, 143.0).
    Prim::Path {
        x: 5.3333, y: 22.8333,
        segs: &[
            Seg::Line(15.3333, 22.8333),
            Seg::Line(16.5833, 24.5),
            Seg::Line(19.0833, 26.0833),
            Seg::Line(1.5833, 26.0833),
            Seg::Line(4.5, 24.5),
        ],
        close: true, fill: Some(Ink::Fixed(TAB)), stroke: None, width: 0.0,
    },
    // C glyph; local design coordinates relative to (1178.0, 143.0).
    Prim::Path {
        x: 7.8333, y: 4.9167,
        segs: &[
            Seg::Line(17.8333, 4.9167),
            Seg::Quad { cx: 19.5, cy: 4.9167, x: 19.5, y: 6.5833 },
            Seg::Line(19.5, 9.9167),
            Seg::Line(17.8333, 9.9167),
            Seg::Line(17.8333, 7.0),
            Seg::Line(8.25, 7.0),
            Seg::Line(8.25, 17.0),
            Seg::Line(17.8333, 17.0),
            Seg::Line(17.8333, 13.6667),
            Seg::Line(19.5, 13.6667),
            Seg::Line(19.5, 17.0),
            Seg::Quad { cx: 19.5, cy: 19.0833, x: 17.4167, y: 19.0833 },
            Seg::Line(8.25, 19.0833),
            Seg::Quad { cx: 5.75, cy: 19.0833, x: 5.75, y: 17.0 },
            Seg::Line(5.75, 7.0),
            Seg::Quad { cx: 5.75, cy: 4.9167, x: 7.8333, y: 4.9167 },
        ],
        close: true, fill: Some(Ink::Fixed(BRIGHT)), stroke: None, width: 0.0,
    },
];
