//! Source-fitted Neo-kitsch mailbox section badges A/B/C/D.
//!
//! Frame positions follow both window axes. Letter offsets are relative to
//! their existing Run anchors and scale uniformly through Piece::LabelArt.
//! Counters wind opposite the outer contour for nonzero canvas filling.
//! See docs/neokitsch/mailbox-badges.md for source controls and ink limits.
use crate::style::{Ink, Piece, Seg};

const MAIL_A_OUTLINE: &[Seg] = &[

    Seg::Line(260.5, 98.5),
    Seg::Quad { cx: 263.25, cy: 98.5, x: 263.25, y: 101.5 },
    Seg::Line(263.25, 116.5),
    Seg::Line(256.5, 123.5),
    Seg::Line(241.5, 123.5),
    Seg::Quad { cx: 238.5, cy: 123.5, x: 238.5, y: 120.5 },
    Seg::Line(238.5, 101.5),
    Seg::Quad { cx: 238.5, cy: 98.5, x: 241.5, y: 98.5 },
];

const MAIL_A_TAB: &[Seg] = &[

    Seg::Line(242.3, 121.5),
    Seg::Quad { cx: 242.5, cy: 121.0, x: 243.0, y: 121.0 },
    Seg::Line(253.0, 121.0),
    Seg::Quad { cx: 253.5, cy: 121.0, x: 253.7, y: 121.5 },
    Seg::Line(254.5, 123.7),
];

pub(super) const MAIL_A_FRAME: &[Piece] = &[
    Piece::Curve {
        start: (241.5, 98.5),
        steps: MAIL_A_OUTLINE,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Curve {
        start: (241.5, 123.7),
        steps: MAIL_A_TAB,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];

const MAIL_B_OUTLINE: &[Seg] = &[

    Seg::Line(1033.5, 98.5),
    Seg::Quad { cx: 1036.25, cy: 98.5, x: 1036.25, y: 101.5 },
    Seg::Line(1036.25, 116.5),
    Seg::Line(1029.5, 123.5),
    Seg::Line(1014.5, 123.5),
    Seg::Quad { cx: 1011.5, cy: 123.5, x: 1011.5, y: 120.5 },
    Seg::Line(1011.5, 101.5),
    Seg::Quad { cx: 1011.5, cy: 98.5, x: 1014.5, y: 98.5 },
];

const MAIL_B_TAB: &[Seg] = &[

    Seg::Line(1015.3, 121.5),
    Seg::Quad { cx: 1015.5, cy: 121.0, x: 1016.0, y: 121.0 },
    Seg::Line(1026.0, 121.0),
    Seg::Quad { cx: 1026.5, cy: 121.0, x: 1026.7, y: 121.5 },
    Seg::Line(1027.5, 123.7),
];

pub(super) const MAIL_B_FRAME: &[Piece] = &[
    Piece::Curve {
        start: (1014.5, 98.5),
        steps: MAIL_B_OUTLINE,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Curve {
        start: (1014.5, 123.7),
        steps: MAIL_B_TAB,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];

const MAIL_C_OUTLINE: &[Seg] = &[
    Seg::Line(162.0, 777.5),
    Seg::Quad { cx: 165.0, cy: 777.5, x: 165.0, y: 780.5 },
    Seg::Line(165.0, 794.0),
    Seg::Line(157.5, 802.5),
    Seg::Line(142.5, 802.5),
    Seg::Quad { cx: 139.5, cy: 802.5, x: 139.5, y: 799.5 },
    Seg::Line(139.5, 780.5),
    Seg::Quad { cx: 139.5, cy: 777.5, x: 142.5, y: 777.5 },
];

const MAIL_C_TAB: &[Seg] = &[
    Seg::Line(143.3, 800.5),
    Seg::Quad { cx: 143.5, cy: 800.0, x: 144.0, y: 800.0 },
    Seg::Line(154.0, 800.0),
    Seg::Quad { cx: 154.5, cy: 800.0, x: 154.7, y: 800.5 },
    Seg::Line(155.5, 802.7),
];

pub(super) const MAIL_C_FRAME: &[Piece] = &[
    Piece::Curve {
        start: (142.5, 777.5),
        steps: MAIL_C_OUTLINE,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Curve {
        start: (142.5, 802.7),
        steps: MAIL_C_TAB,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];

const MAIL_D_OUTLINE: &[Seg] = &[
    Seg::Line(758.0, 777.5),
    Seg::Quad { cx: 761.0, cy: 777.5, x: 761.0, y: 780.5 },
    Seg::Line(761.0, 794.0),
    Seg::Line(753.5, 802.5),
    Seg::Line(738.5, 802.5),
    Seg::Quad { cx: 735.5, cy: 802.5, x: 735.5, y: 799.5 },
    Seg::Line(735.5, 780.5),
    Seg::Quad { cx: 735.5, cy: 777.5, x: 738.5, y: 777.5 },
];

const MAIL_D_TAB: &[Seg] = &[
    Seg::Line(739.3, 800.5),
    Seg::Quad { cx: 739.5, cy: 800.0, x: 740.0, y: 800.0 },
    Seg::Line(750.0, 800.0),
    Seg::Quad { cx: 750.5, cy: 800.0, x: 750.7, y: 800.5 },
    Seg::Line(751.5, 802.7),
];

pub(super) const MAIL_D_FRAME: &[Piece] = &[
    Piece::Curve {
        start: (738.5, 777.5),
        steps: MAIL_D_OUTLINE,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Curve {
        start: (738.5, 802.7),
        steps: MAIL_D_TAB,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];

pub(super) const MAIL_A_CONTOUR: &[Piece] = &[
    Piece::Curve {
        start: (-1.36667, -12.58333),
        steps: &[
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
        ],
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];

pub(super) const MAIL_B_CONTOUR: &[Piece] = &[
    Piece::Curve {
        start: (-7.08333, -12.58333),
        steps: &[
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
        ],
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];

const MAIL_C_REL_START: (f32, f32) = (-5.8833, -14.3333);
const MAIL_C_REL: &[Seg] = &[
    Seg::Line(5.3667, -14.3333),
    Seg::Quad { cx: 7.0333, cy: -14.3333, x: 7.0333, y: -12.6667 },
    Seg::Line(7.0333, -9.7500),
    Seg::Line(4.9500, -9.7500),
    Seg::Line(4.9500, -12.6667),
    Seg::Line(-4.6333, -12.6667),
    Seg::Line(-4.6333, -2.6667),
    Seg::Line(4.9500, -2.6667),
    Seg::Line(4.9500, -5.5833),
    Seg::Line(7.0333, -5.5833),
    Seg::Line(7.0333, -2.2500),
    Seg::Quad { cx: 7.0333, cy: -0.5833, x: 5.3667, y: -0.5833 },
    Seg::Line(-5.4667, -0.5833),
    Seg::Quad { cx: -7.1333, cy: -0.5833, x: -7.1333, y: -2.2500 },
    Seg::Line(-7.1333, -12.6667),
    Seg::Quad { cx: -7.1333, cy: -14.3333, x: -5.8833, y: -14.3333 },
];

pub(super) const MAIL_C_CONTOUR: &[Piece] = &[
    Piece::Curve { start: MAIL_C_REL_START, steps: MAIL_C_REL,
        fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
];

pub(super) const MAIL_D_CONTOUR: &[Piece] = &[
    Piece::Curve {
        start: (-10.70000, -14.33333),
        steps: &[
            Seg::Line(-1.11667, -14.33333),
            Seg::Cubic { c1x: 2.63333, c1y: -14.33333, c2x: 5.13333, c2y: -11.83333, x: 5.13333, y: -7.66667 },
            Seg::Cubic { c1x: 5.13333, c1y: -3.50000, c2x: 2.63333, c2y: -0.58333, x: -1.11667, y: -0.58333 },
            Seg::Line(-10.70000, -0.58333),
            Seg::Line(-10.70000, -14.33333),
            Seg::Move(-8.20000, -12.66667),
            Seg::Line(-8.20000, -2.66667),
            Seg::Line(-1.11667, -2.66667),
            Seg::Cubic { c1x: 1.38333, c1y: -2.66667, c2x: 3.05000, c2y: -4.75000, x: 3.05000, y: -7.66667 },
            Seg::Cubic { c1x: 3.05000, c1y: -10.58333, c2x: 1.38333, c2y: -12.66667, x: -1.11667, y: -12.66667 },
            Seg::Line(-8.20000, -12.66667),
        ],
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
];
