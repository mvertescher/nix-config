//! Local dashboard header echo reconstructions from source screen #60.
//! This reproduces observed trails without classifying their original renderer.
//! Caption paths use the bundled Rajdhani Medium; other templates preserve the
//! accepted header vectors. Shapes are reused across both copies and profiles.
use crate::palette::rgb;
use crate::style::{fill_rect, Ink, Prim, Seg};

const RED: Ink = Ink::Fixed(rgb(0xef3333));
const fn placed<const N: usize>(mut segs: [Seg; N], x: f32, y: f32) -> [Seg; N] {
    let mut i = 0;
    while i < N {
        segs[i] = match segs[i] {
            Seg::Move(a, b) => Seg::Move(x + a, y + b),
            Seg::Line(a, b) => Seg::Line(x + a, y + b),
            Seg::Quad { cx, cy, x: a, y: b } => Seg::Quad { cx: x + cx, cy: y + cy, x: x + a, y: y + b },
            Seg::Cubic { c1x, c1y, c2x, c2y, x: a, y: b } => Seg::Cubic { c1x: x + c1x, c1y: y + c1y, c2x: x + c2x, c2y: y + c2y, x: x + a, y: y + b },
        };
        i += 1;
    }
    segs
}
macro_rules! shape {
    ($path:ident, $x:expr, $y:expr, $filled:expr, $closed:expr) => {
        Prim::Path { x: $x, y: $y, segs: &placed($path, $x, $y), close: $closed,
            fill: if $filled { Some(RED) } else { None }, stroke: None, width: 1.0 }
    };
}
// Profile layers are opaque until Masked applies their one group alpha.
// This prevents overlapping fill/stroke from being blended twice.
const fn profile<const N: usize>(mut paths: [Prim; N], line: f32, fill: f32) -> [Prim; N] {
    let mut i = 0;
    while i < N {
        if let Prim::Path { x, y, segs, close, fill: old_fill, .. } = paths[i] {
            let width = if old_fill.is_some() { fill } else { line };
            paths[i] = Prim::Path { x, y, segs, close, fill: old_fill,
                stroke: if width > 0.0 { Some(RED) } else { None }, width };
        }
        i += 1;
    }
    paths
}
const fn alpha_mask(b: (f32, f32, f32, f32), a: f32) -> Prim {
    fill_rect(b.0, b.1, b.2 - b.0, b.3 - b.1,
        Ink::Fixed(iced::Color { a, ..rgb(0xffffff) }))
}
macro_rules! layer {
    ($src:ident, $line:expr, $fill:expr, $alpha:expr, $bounds:expr) => {
        Prim::Masked { prims: &profile($src, $line, $fill), mask: &[alpha_mask($bounds, $alpha)] }
    };
}
const CYCLE: &[(f32, iced::Color)] = &[
    (0.0, iced::Color { a: 0.0, ..rgb(0xffffff) }),
    (0.125, iced::Color { a: 0.14645, ..rgb(0xffffff) }),
    (0.25, iced::Color { a: 0.5, ..rgb(0xffffff) }),
    (0.375, iced::Color { a: 0.85355, ..rgb(0xffffff) }),
    (0.5, iced::Color { a: 1.0, ..rgb(0xffffff) }),
    (0.625, iced::Color { a: 0.85355, ..rgb(0xffffff) }),
    (0.75, iced::Color { a: 0.5, ..rgb(0xffffff) }),
    (0.875, iced::Color { a: 0.14645, ..rgb(0xffffff) }),
    (1.0, iced::Color { a: 0.0, ..rgb(0xffffff) }),
 ];
// An opaque floor and explicit transparent-ended ramps avoid pattern pitch
// quantization and internal antialiasing gaps at fractional scales.
const fn scan<const N: usize>(x: f32, y: f32, w: f32, h: f32, start: f32, floor: u32) -> [Prim; N] {
    let gray = (floor << 16) | (floor << 8) | floor;
    let mut rows = [fill_rect(x, y, w, h, Ink::Fixed(rgb(gray))); N];
    let mut i = 1;
    while i < N {
        rows[i] = Prim::Ramp { x, y: start + (i - 1) as f32 * 1.984934,
            w, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops: CYCLE };
        i += 1;
    }
    rows
}

const P0: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(56.6667, 0.0),
    Seg::Line(56.6667, 56.6667),
    Seg::Line(15.4167, 56.6667),
    Seg::Line(0.0, 41.25)
];
const P1: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(56.6667, 0.0),
    Seg::Line(56.6667, 56.25),
    Seg::Line(15.4167, 56.25),
    Seg::Line(0.0, 40.8333)
];
const P2: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(56.25, 0.0),
    Seg::Line(56.25, 56.25),
    Seg::Line(15.4167, 56.25),
    Seg::Line(0.0, 40.8333)
];
const P3: [Seg; 12] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -22.4),
    Seg::Line(3.4, -22.4),
    Seg::Line(3.4, -19.0),
    Seg::Cubic { c1x: 6.2, c1y: -22.0, c2x: 10.1, c2y: -22.5, x: 13.9, y: -22.5 },
    Seg::Cubic { c1x: 22.9, c1y: -22.5, c2x: 27.4, c2y: -19.6, x: 27.4, y: -12.0 },
    Seg::Line(27.4, 0.0),
    Seg::Line(23.8, 0.0),
    Seg::Line(23.8, -12.2),
    Seg::Cubic { c1x: 23.8, c1y: -17.0, c2x: 20.7, c2y: -19.3, x: 13.5, y: -19.3 },
    Seg::Cubic { c1x: 7.0, c1y: -19.3, c2x: 3.4, c2y: -17.0, x: 3.4, y: -12.2 },
    Seg::Line(3.4, 0.0)
];
const P4: [Seg; 21] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.6),
    Seg::Cubic { c1x: 0.0, c1y: 6.1, c2x: -3.9, c2y: 7.9, x: -10.4, y: 7.9 },
    Seg::Line(-18.4, 7.9),
    Seg::Cubic { c1x: -25.9, c1y: 7.9, c2x: -28.4, c2y: 5.1, x: -28.4, y: -2.4 },
    Seg::Line(-28.4, -5.1),
    Seg::Cubic { c1x: -28.4, c1y: -11.9, c2x: -25.4, c2y: -14.9, x: -18.4, y: -14.9 },
    Seg::Line(-10.4, -14.9),
    Seg::Cubic { c1x: -2.8, c1y: -14.9, c2x: 0.0, c2y: -11.4, x: 0.0, y: -4.4 },
    Seg::Line(0.0, -2.9),
    Seg::Line(-24.9, -2.9),
    Seg::Line(-24.9, -0.2),
    Seg::Cubic { c1x: -24.9, c1y: 3.6, c2x: -23.0, c2y: 4.8, x: -17.4, y: 4.8 },
    Seg::Line(-10.6, 4.8),
    Seg::Cubic { c1x: -5.6, c1y: 4.8, c2x: -3.6, c2y: 3.6, x: -3.6, y: 0.0 },
    Seg::Move(-24.9, -6.0),
    Seg::Line(-3.5, -6.0),
    Seg::Line(-3.5, -6.9),
    Seg::Cubic { c1x: -3.9, c1y: -10.8, c2x: -5.7, c2y: -11.7, x: -10.4, y: -11.7 },
    Seg::Line(-17.8, -11.7),
    Seg::Cubic { c1x: -22.8, c1y: -11.7, c2x: -24.9, c2y: -10.2, x: -24.9, y: -6.0 }
];
const P5: [Seg; 12] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(5.7, 0.0),
    Seg::Line(14.3, 8.6),
    Seg::Line(22.8, 0.0),
    Seg::Line(28.6, 0.0),
    Seg::Line(17.7, 10.8),
    Seg::Line(29.8, 22.5),
    Seg::Line(24.1, 22.5),
    Seg::Line(14.3, 13.5),
    Seg::Line(4.8, 22.5),
    Seg::Line(-0.6, 22.5),
    Seg::Line(11.0, 10.8)
];
const P6: [Seg; 17] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(3.7, 0.0),
    Seg::Line(3.7, 5.2),
    Seg::Line(17.2, 5.2),
    Seg::Line(17.2, 8.5),
    Seg::Line(3.7, 8.5),
    Seg::Line(3.7, 19.7),
    Seg::Cubic { c1x: 3.7, c1y: 23.8, c2x: 4.9, c2y: 24.6, x: 9.7, y: 24.6 },
    Seg::Cubic { c1x: 14.9, c1y: 24.6, c2x: 14.6, c2y: 23.6, x: 14.6, y: 17.7 },
    Seg::Line(17.9, 17.7),
    Seg::Line(17.9, 21.1),
    Seg::Cubic { c1x: 17.9, c1y: 26.1, c2x: 14.9, c2y: 27.9, x: 9.5, y: 27.9 },
    Seg::Cubic { c1x: 2.9, c1y: 27.9, c2x: 0.0, c2y: 25.5, x: 0.0, y: 20.3 },
    Seg::Line(0.0, 8.5),
    Seg::Line(-4.5, 8.5),
    Seg::Line(-4.5, 5.2),
    Seg::Line(0.0, 5.2)
];
const P7: [Seg; 4] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(4.2584, 0.0),
    Seg::Move(2.1292, 0.0),
    Seg::Line(2.1292, 4.9291)
];
const P8: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-3.3395, 0.0),
    Seg::Line(-3.3395, 4.5904),
    Seg::Line(0.0, 4.5904),
    Seg::Move(-3.3395, 2.2493),
    Seg::Line(-0.6011, 2.2493)
];
const P9: [Seg; 4] = [
    Seg::Move(0.0, 0.0),
    Seg::Cubic { c1x: -0.774, c1y: -1.1583, c2x: -2.8635, c2y: -1.1583, x: -3.5601, y: 0.0927 },
    Seg::Cubic { c1x: -4.1405, c1y: 1.0193, c2x: -4.1405, c2y: 2.5019, x: -3.5214, y: 3.2431 },
    Seg::Cubic { c1x: -2.6701, c1y: 4.2624, c2x: -0.8514, c2y: 4.0308, x: 0.0, y: 3.0578 }
];
const P10: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 5.0474),
    Seg::Move(3.7738, 0.0),
    Seg::Line(3.7738, 5.0474),
    Seg::Move(0.0, 2.5237),
    Seg::Line(3.7738, 2.5237)
];
const P11: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -4.9883),
    Seg::Move(0.0, -4.9883),
    Seg::Line(3.7898, 0.0),
    Seg::Move(3.7898, 0.0),
    Seg::Line(3.7898, -4.9883)
];
const P12: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Cubic { c1x: -1.3997, c1y: 0.0, c2x: -2.0584, c2y: 0.8764, x: -2.0584, y: 2.4346 },
    Seg::Cubic { c1x: -2.0584, c1y: 3.9927, c2x: -1.3997, c2y: 4.8692, x: 0.0, y: 4.8692 },
    Seg::Cubic { c1x: 1.3998, c1y: 4.8692, c2x: 2.0585, c2y: 3.9927, x: 2.0585, y: 2.4346 },
    Seg::Cubic { c1x: 2.0585, c1y: 0.8764, c2x: 1.3998, c2y: 0.0, x: 0.0, y: 0.0 }
];
const P13: [Seg; 3] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 4.8004),
    Seg::Line(3.484, 4.8004)
];
const P14: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Cubic { c1x: -1.4597, c1y: 0.0, c2x: -2.1467, c2y: 0.8701, x: -2.1467, y: 2.417 },
    Seg::Cubic { c1x: -2.1467, c1y: 3.9639, c2x: -1.4597, c2y: 4.834, x: 0.0, y: 4.834 },
    Seg::Cubic { c1x: 1.4597, c1y: 4.834, c2x: 2.1467, c2y: 3.9639, x: 2.1467, y: 2.417 },
    Seg::Cubic { c1x: 2.1467, c1y: 0.8701, c2x: 1.4597, c2y: 0.0, x: 0.0, y: 0.0 }
];
const P15: [Seg; 8] = [
    Seg::Move(0.0, 0.0),
    Seg::Cubic { c1x: -0.7683, c1y: -1.1613, c2x: -2.8425, c2y: -1.1613, x: -3.534, y: 0.0929 },
    Seg::Cubic { c1x: -4.1102, c1y: 1.0219, c2x: -4.1102, c2y: 2.5082, x: -3.4956, y: 3.2514 },
    Seg::Cubic { c1x: -2.6505, c1y: 4.2733, c2x: -0.8451, c2y: 4.0411, x: 0.0, y: 3.0656 },
    Seg::Line(0.0, 1.7186),
    Seg::Line(-1.6133, 1.7186),
    Seg::Move(0.0, 3.0656),
    Seg::Line(0.0, 3.8553)
];
const P16: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(2.0112, 3.0453),
    Seg::Line(4.0225, 0.0),
    Seg::Move(2.0112, 3.0453),
    Seg::Line(2.0112, 5.2506)
];
const P17: [Seg; 23] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 6.9),
    Seg::Line(6.3, 6.9),
    Seg::Move(7.9, 0.0),
    Seg::Line(7.9, 6.9),
    Seg::Line(14.6, 6.9),
    Seg::Move(7.9, 0.0),
    Seg::Line(14.6, 0.0),
    Seg::Move(7.9, 3.5),
    Seg::Line(14.3, 3.5),
    Seg::Move(16.2, 0.0),
    Seg::Line(20.8, 6.9),
    Seg::Line(25.1, 0.0),
    Seg::Move(26.7, 0.0),
    Seg::Line(26.7, 6.9),
    Seg::Line(33.6, 6.9),
    Seg::Move(26.7, 0.0),
    Seg::Line(33.6, 0.0),
    Seg::Move(26.7, 3.5),
    Seg::Line(33.1, 3.5),
    Seg::Move(35.9, 0.0),
    Seg::Line(35.9, 6.9),
    Seg::Line(42.1, 6.9)
];
const P18: [Seg; 16] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(12.9167, 0.0),
    Seg::Line(12.9167, 1.6667),
    Seg::Line(7.5, 1.6667),
    Seg::Line(7.5, 12.0834),
    Seg::Line(5.4167, 12.0834),
    Seg::Line(5.4167, 1.6667),
    Seg::Line(0.0, 1.6667),
    Seg::Move(15.0, 1.25),
    Seg::Line(16.6667, 1.25),
    Seg::Line(18.3333, -0.8333),
    Seg::Line(19.5833, -0.8333),
    Seg::Line(19.5833, 12.0834),
    Seg::Line(17.9167, 12.0834),
    Seg::Line(17.9167, 2.5),
    Seg::Line(15.0, 2.5)
];
const P19: [Seg; 20] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(12.9167, 0.0),
    Seg::Line(12.9167, 1.6666),
    Seg::Line(7.5, 1.6666),
    Seg::Line(7.5, 12.0833),
    Seg::Line(5.4167, 12.0833),
    Seg::Line(5.4167, 1.6666),
    Seg::Line(0.0, 1.6666),
    Seg::Move(15.3, 0.9833),
    Seg::Cubic { c1x: 18.2, c1y: -0.8167, c2x: 21.3, c2y: -0.6167, x: 23.4, y: -0.1167 },
    Seg::Cubic { c1x: 26.7, c1y: 1.0833, c2x: 27.0, c2y: 3.8833, x: 25.6, y: 5.6833 },
    Seg::Cubic { c1x: 24.4, c1y: 6.9833, c2x: 21.8, c2y: 7.5833, x: 20.3, y: 8.2833 },
    Seg::Cubic { c1x: 18.6, c1y: 9.1833, c2x: 18.0, c2y: 10.0833, x: 18.0, y: 10.3833 },
    Seg::Line(26.5, 10.3833),
    Seg::Line(26.5, 12.0833),
    Seg::Line(15.3, 12.0833),
    Seg::Line(15.3, 9.9833),
    Seg::Cubic { c1x: 15.3, c1y: 7.5833, c2x: 17.3, c2y: 6.4833, x: 20.8, y: 5.3833 },
    Seg::Cubic { c1x: 24.2, c1y: 4.2833, c2x: 25.0, c2y: 3.5833, x: 24.1, y: 2.3833 },
    Seg::Cubic { c1x: 22.8, c1y: 0.8833, c2x: 19.6, c2y: 1.2833, x: 16.0, y: 2.5833 }
];
const P20: [Seg; 21] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(12.9167, 0.0),
    Seg::Line(12.9167, 1.6666),
    Seg::Line(7.5, 1.6666),
    Seg::Line(7.5, 12.0833),
    Seg::Line(5.4167, 12.0833),
    Seg::Line(5.4167, 1.6666),
    Seg::Line(0.0, 1.6666),
    Seg::Move(15.2, 1.3833),
    Seg::Cubic { c1x: 18.6, c1y: -0.5167, c2x: 22.7, c2y: -0.7167, x: 25.1, y: 1.3833 },
    Seg::Cubic { c1x: 27.0, c1y: 3.1833, c2x: 25.8, c2y: 5.5833, x: 23.2, y: 6.4833 },
    Seg::Cubic { c1x: 26.8, c1y: 7.4833, c2x: 27.3, c2y: 10.0833, x: 24.7, y: 11.5833 },
    Seg::Cubic { c1x: 22.0, c1y: 13.4833, c2x: 17.5, c2y: 12.9833, x: 14.9, y: 11.2833 },
    Seg::Line(15.6, 9.6833),
    Seg::Cubic { c1x: 18.0, c1y: 11.2833, c2x: 20.9, c2y: 11.5833, x: 23.3, y: 10.4833 },
    Seg::Cubic { c1x: 25.2, c1y: 9.4833, c2x: 24.1, c2y: 7.6833, x: 22.3, y: 7.6833 },
    Seg::Line(18.4, 7.6833),
    Seg::Line(18.4, 5.9833),
    Seg::Line(22.0, 5.9833),
    Seg::Cubic { c1x: 24.5, c1y: 5.8833, c2x: 24.8, c2y: 4.1833, x: 23.7, y: 2.8833 },
    Seg::Cubic { c1x: 22.1, c1y: 1.3833, c2x: 19.0, c2y: 1.5833, x: 16.0, y: 3.0833 }
];
const P21: [Seg; 22] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(12.9167, 0.0),
    Seg::Line(12.9167, 2.0833),
    Seg::Line(7.5, 2.0833),
    Seg::Line(7.5, 12.0833),
    Seg::Line(5.4167, 12.0833),
    Seg::Line(5.4167, 2.0833),
    Seg::Line(0.0, 2.0833),
    Seg::Move(15.0, 7.5),
    Seg::Line(22.5, 0.0),
    Seg::Line(24.5833, 0.0),
    Seg::Line(24.5833, 7.5),
    Seg::Line(27.0833, 7.5),
    Seg::Line(27.0833, 9.1667),
    Seg::Line(24.5833, 9.1667),
    Seg::Line(24.5833, 12.0833),
    Seg::Line(22.5, 12.0833),
    Seg::Line(22.5, 9.1667),
    Seg::Line(15.0, 9.1667),
    Seg::Move(17.9167, 7.5),
    Seg::Line(22.5, 7.5),
    Seg::Line(22.5, 3.3333)
];
const P22: [Seg; 29] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-1.55001, 0.0),
    Seg::Quad { cx: -2.44126, cy: 0.0, x: -2.95147, y: -0.50375 },
    Seg::Quad { cx: -3.46168, cy: -1.0075, x: -3.46168, y: -1.91167 },
    Seg::Line(-3.46168, -6.39377),
    Seg::Quad { cx: -3.46168, cy: -7.29794, x: -2.95147, y: -7.80169 },
    Seg::Quad { cx: -2.44126, cy: -8.30544, x: -1.55001, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.89125, cy: -8.30544, x: 1.40146, y: -7.80169 },
    Seg::Quad { cx: 1.91167, cy: -7.29794, x: 1.91167, y: -6.39377 },
    Seg::Line(1.91167, -5.76085),
    Seg::Quad { cx: 1.91167, cy: -5.58001, x: 1.73084, y: -5.58001 },
    Seg::Line(1.14958, -5.58001),
    Seg::Quad { cx: 0.96875, cy: -5.58001, x: 0.96875, y: -5.76085 },
    Seg::Line(0.96875, -6.35502),
    Seg::Quad { cx: 0.96875, cy: -7.47877, x: -0.11625, y: -7.47877 },
    Seg::Line(-1.44667, -7.47877),
    Seg::Quad { cx: -2.51876, cy: -7.47877, x: -2.51876, y: -6.35502 },
    Seg::Line(-2.51876, -1.95042),
    Seg::Quad { cx: -2.51876, cy: -0.82667, x: -1.44667, y: -0.82667 },
    Seg::Line(-0.11625, -0.82667),
    Seg::Quad { cx: 0.96875, cy: -0.82667, x: 0.96875, y: -1.95042 },
    Seg::Line(0.96875, -2.54459),
    Seg::Quad { cx: 0.96875, cy: -2.72542, x: 1.14958, y: -2.72542 },
    Seg::Line(1.73084, -2.72542),
    Seg::Quad { cx: 1.91167, cy: -2.72542, x: 1.91167, y: -2.54459 },
    Seg::Line(1.91167, -1.91167),
    Seg::Quad { cx: 1.91167, cy: -1.0075, x: 1.40146, y: -0.50375 },
    Seg::Quad { cx: 0.89125, cy: 0.0, x: 0.0, y: 0.0 }
];
const P23: [Seg; 19] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.58125, 0.0),
    Seg::Quad { cx: 0.76208, cy: 0.0, x: 0.76208, y: 0.16792 },
    Seg::Line(0.76208, 6.39377),
    Seg::Quad { cx: 0.76208, cy: 7.29794, x: 0.25833, y: 7.80169 },
    Seg::Quad { cx: -0.24542, cy: 8.30544, x: -1.13667, y: 8.30544 },
    Seg::Line(-2.57043, 8.30544),
    Seg::Quad { cx: -3.46168, cy: 8.30544, x: -3.97189, y: 7.80169 },
    Seg::Quad { cx: -4.4821, cy: 7.29794, x: -4.4821, y: 6.39377 },
    Seg::Line(-4.4821, 0.18084),
    Seg::Quad { cx: -4.4821, cy: 0.0, x: -4.30127, y: 0.0 },
    Seg::Line(-3.72001, 0.0),
    Seg::Quad { cx: -3.53918, cy: 0.0, x: -3.53918, y: 0.18084 },
    Seg::Line(-3.53918, 6.35502),
    Seg::Quad { cx: -3.53918, cy: 7.47877, x: -2.46709, y: 7.47877 },
    Seg::Line(-1.25292, 7.47877),
    Seg::Quad { cx: -0.16792, cy: 7.47877, x: -0.16792, y: 6.35502 },
    Seg::Line(-0.16792, 0.18084),
    Seg::Quad { cx: -0.16792, cy: 0.0, x: 0.0, y: 0.0 }
];
const P24: [Seg; 41] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -0.41334),
    Seg::Quad { cx: 0.0, cy: -0.59417, x: 0.18083, y: -0.59417 },
    Seg::Line(0.76208, -0.59417),
    Seg::Quad { cx: 0.94292, cy: -0.59417, x: 0.94292, y: -0.41334 },
    Seg::Line(0.94292, -0.0775),
    Seg::Quad { cx: 0.94292, cy: 1.085, x: 2.0925, y: 1.085 },
    Seg::Line(3.22917, 1.085),
    Seg::Quad { cx: 4.37876, cy: 1.085, x: 4.37876, y: -0.10334 },
    Seg::Line(4.37876, -0.5425),
    Seg::Quad { cx: 4.37876, cy: -1.47251, x: 2.84167, y: -1.75667 },
    Seg::Quad { cx: 2.19584, cy: -1.87292, x: 1.55, y: -2.02792 },
    Seg::Quad { cx: 0.90417, cy: -2.18292, x: 0.45208, y: -2.62855 },
    Seg::Quad { cx: 0.0, cy: -3.07418, x: 0.0, y: -3.84918 },
    Seg::Line(0.0, -4.4821),
    Seg::Quad { cx: 0.0, cy: -5.37335, x: 0.51021, y: -5.88356 },
    Seg::Quad { cx: 1.02042, cy: -6.39377, x: 1.91167, y: -6.39377 },
    Seg::Line(3.34542, -6.39377),
    Seg::Quad { cx: 4.22376, cy: -6.39377, x: 4.74043, y: -5.88356 },
    Seg::Quad { cx: 5.2571, cy: -5.37335, x: 5.2571, y: -4.4821 },
    Seg::Line(5.2571, -4.15918),
    Seg::Quad { cx: 5.2571, cy: -3.96543, x: 5.08918, y: -3.96543 },
    Seg::Line(4.49501, -3.96543),
    Seg::Quad { cx: 4.32709, cy: -3.96543, x: 4.32709, y: -4.15918 },
    Seg::Line(4.32709, -4.39168),
    Seg::Quad { cx: 4.32709, cy: -5.5671, x: 3.17751, y: -5.5671 },
    Seg::Line(2.07959, -5.5671),
    Seg::Quad { cx: 0.93, cy: -5.5671, x: 0.93, y: -4.34001 },
    Seg::Line(0.93, -3.82334),
    Seg::Quad { cx: 0.93, cy: -3.16459, x: 1.79542, y: -2.91918 },
    Seg::Quad { cx: 2.18292, cy: -2.81584, x: 2.64792, y: -2.73188 },
    Seg::Quad { cx: 3.11292, cy: -2.64793, x: 3.58438, y: -2.5123 },
    Seg::Quad { cx: 4.05584, cy: -2.37667, x: 4.44334, y: -2.17647 },
    Seg::Quad { cx: 4.83084, cy: -1.97626, x: 5.0698, y: -1.56938 },
    Seg::Quad { cx: 5.30876, cy: -1.1625, x: 5.30876, y: -0.58125 },
    Seg::Line(5.30876, 0.0),
    Seg::Quad { cx: 5.30876, cy: 0.89125, x: 4.79209, y: 1.40146 },
    Seg::Quad { cx: 4.27543, cy: 1.91167, x: 3.39709, y: 1.91167 },
    Seg::Line(1.91167, 1.91167),
    Seg::Quad { cx: 1.03333, cy: 1.91167, x: 0.51667, y: 1.40146 },
    Seg::Quad { cx: 0.0, cy: 0.89125, x: 0.0, y: 0.0 }
];
const P25: [Seg; 17] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.58126, 0.0),
    Seg::Quad { cx: -0.76209, cy: 0.0, x: -0.76209, y: -0.18083 },
    Seg::Line(-0.76209, -7.36252),
    Seg::Quad { cx: -0.76209, cy: -7.47877, x: -0.89126, y: -7.47877 },
    Seg::Line(-2.64793, -7.47877),
    Seg::Quad { cx: -2.84168, cy: -7.47877, x: -2.84168, y: -7.6596 },
    Seg::Line(-2.84168, -8.1246),
    Seg::Quad { cx: -2.84168, cy: -8.30544, x: -2.64793, y: -8.30544 },
    Seg::Line(2.06667, -8.30544),
    Seg::Quad { cx: 2.26042, cy: -8.30544, x: 2.26042, y: -8.1246 },
    Seg::Line(2.26042, -7.6596),
    Seg::Quad { cx: 2.26042, cy: -7.47877, x: 2.06667, y: -7.47877 },
    Seg::Line(0.31, -7.47877),
    Seg::Quad { cx: 0.18083, cy: -7.47877, x: 0.18083, y: -7.36252 },
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 }
];
const P26: [Seg; 22] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1.31751, 0.0),
    Seg::Quad { cx: 2.40251, cy: 0.0, x: 2.40251, y: -1.12375 },
    Seg::Line(2.40251, -5.52835),
    Seg::Quad { cx: 2.40251, cy: -6.6521, x: 1.31751, y: -6.6521 },
    Seg::Line(0.0, -6.6521),
    Seg::Quad { cx: -1.07208, cy: -6.6521, x: -1.07208, y: -5.52835 },
    Seg::Line(-1.07208, -1.12375),
    Seg::Quad { cx: -1.07208, cy: 0.0, x: 0.0, y: 0.0 },
    Seg::Move(1.43376, 0.82667),
    Seg::Line(-0.10333, 0.82667),
    Seg::Quad { cx: -0.99458, cy: 0.82667, x: -1.50479, y: 0.32292 },
    Seg::Quad { cx: -2.015, cy: -0.18083, x: -2.015, y: -1.085 },
    Seg::Line(-2.015, -5.5671),
    Seg::Quad { cx: -2.015, cy: -6.47127, x: -1.50479, y: -6.97502 },
    Seg::Quad { cx: -0.99458, cy: -7.47877, x: -0.10333, y: -7.47877 },
    Seg::Line(1.43376, -7.47877),
    Seg::Quad { cx: 2.32501, cy: -7.47877, x: 2.83522, y: -6.96856 },
    Seg::Quad { cx: 3.34543, cy: -6.45835, x: 3.34543, y: -5.5671 },
    Seg::Line(3.34543, -1.085),
    Seg::Quad { cx: 3.34543, cy: -0.19375, x: 2.83522, y: 0.31646 },
    Seg::Quad { cx: 2.32501, cy: 0.82667, x: 1.43376, y: 0.82667 }
];
const P27: [Seg; 27] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1.04625, 0.0),
    Seg::Quad { cx: 1.22709, cy: 0.0, x: 1.22709, y: 0.18084 },
    Seg::Line(1.22709, 8.12461),
    Seg::Quad { cx: 1.22709, cy: 8.30544, x: 1.04625, y: 8.30544 },
    Seg::Line(0.47792, 8.30544),
    Seg::Quad { cx: 0.29708, cy: 8.30544, x: 0.29708, y: 8.12461 },
    Seg::Line(0.29708, 1.26584),
    Seg::Line(0.24542, 1.26584),
    Seg::Line(-1.75667, 6.32918),
    Seg::Quad { cx: -1.83417, cy: 6.4971, x: -1.98917, y: 6.4971 },
    Seg::Line(-2.60917, 6.4971),
    Seg::Quad { cx: -2.77709, cy: 6.4971, x: -2.82876, y: 6.32918 },
    Seg::Line(-4.8696, 1.25292),
    Seg::Line(-4.92126, 1.25292),
    Seg::Line(-4.92126, 8.12461),
    Seg::Quad { cx: -4.92126, cy: 8.30544, x: -5.08918, y: 8.30544 },
    Seg::Line(-5.65752, 8.30544),
    Seg::Quad { cx: -5.83835, cy: 8.30544, x: -5.83835, y: 8.12461 },
    Seg::Line(-5.83835, 0.18084),
    Seg::Quad { cx: -5.83835, cy: 0.0, x: -5.65752, y: 0.0 },
    Seg::Line(-4.61126, 0.0),
    Seg::Quad { cx: -4.49501, cy: 0.0, x: -4.45626, y: 0.10334 },
    Seg::Line(-2.33792, 5.45085),
    Seg::Line(-2.27334, 5.45085),
    Seg::Line(-0.155, 0.10334),
    Seg::Quad { cx: -0.12917, cy: 0.0, x: 0.0, y: 0.0 }
];
const P28: [Seg; 25] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-4.05584, 0.0),
    Seg::Quad { cx: -4.23668, cy: 0.0, x: -4.23668, y: -0.18083 },
    Seg::Line(-4.23668, -8.1246),
    Seg::Quad { cx: -4.23668, cy: -8.30544, x: -4.05584, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.18084, cy: -8.30544, x: 0.18084, y: -8.1246 },
    Seg::Line(0.18084, -7.6596),
    Seg::Quad { cx: 0.18084, cy: -7.47877, x: 0.0, y: -7.47877 },
    Seg::Line(-3.16459, -7.47877),
    Seg::Quad { cx: -3.29376, cy: -7.47877, x: -3.29376, y: -7.36252 },
    Seg::Line(-3.29376, -4.75335),
    Seg::Quad { cx: -3.29376, cy: -4.6371, x: -3.16459, y: -4.6371 },
    Seg::Line(-0.40042, -4.6371),
    Seg::Quad { cx: -0.21958, cy: -4.6371, x: -0.21958, y: -4.45626 },
    Seg::Line(-0.21958, -3.99126),
    Seg::Quad { cx: -0.21958, cy: -3.81043, x: -0.40042, y: -3.81043 },
    Seg::Line(-3.16459, -3.81043),
    Seg::Quad { cx: -3.29376, cy: -3.81043, x: -3.29376, y: -3.69418 },
    Seg::Line(-3.29376, -0.94292),
    Seg::Quad { cx: -3.29376, cy: -0.82667, x: -3.16459, y: -0.82667 },
    Seg::Line(0.0, -0.82667),
    Seg::Quad { cx: 0.18084, cy: -0.82667, x: 0.18084, y: -0.64583 },
    Seg::Line(0.18084, -0.18083),
    Seg::Quad { cx: 0.18084, cy: 0.0, x: 0.0, y: 0.0 }
];
const P29: [Seg; 30] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.58126, 0.0),
    Seg::Quad { cx: -0.76209, cy: 0.0, x: -0.76209, y: -0.18083 },
    Seg::Line(-0.76209, -8.1246),
    Seg::Quad { cx: -0.76209, cy: -8.30544, x: -0.58126, y: -8.30544 },
    Seg::Line(2.38958, -8.30544),
    Seg::Quad { cx: 3.28084, cy: -8.30544, x: 3.7975, y: -7.79523 },
    Seg::Quad { cx: 4.31417, cy: -7.28502, x: 4.31417, y: -6.39377 },
    Seg::Line(4.31417, -5.21835),
    Seg::Quad { cx: 4.31417, cy: -4.49501, x: 3.96542, y: -4.02355 },
    Seg::Quad { cx: 3.61667, cy: -3.55209, x: 2.99667, y: -3.39709 },
    Seg::Line(2.99667, -3.34543),
    Seg::Line(4.49501, -0.20667),
    Seg::Quad { cx: 4.61126, cy: 0.0, x: 4.36584, y: 0.0 },
    Seg::Line(3.78459, 0.0),
    Seg::Quad { cx: 3.565, cy: 0.0, x: 3.46167, y: -0.18083 },
    Seg::Line(2.015, -3.30668),
    Seg::Line(0.31, -3.30668),
    Seg::Quad { cx: 0.18083, cy: -3.30668, x: 0.18083, y: -3.19042 },
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 },
    Seg::Move(0.31, -4.09459),
    Seg::Line(2.26042, -4.09459),
    Seg::Quad { cx: 3.37125, cy: -4.09459, x: 3.37125, y: -5.20543 },
    Seg::Line(3.37125, -6.35502),
    Seg::Quad { cx: 3.37125, cy: -7.47877, x: 2.26042, y: -7.47877 },
    Seg::Line(0.31, -7.47877),
    Seg::Quad { cx: 0.18083, cy: -7.47877, x: 0.18083, y: -7.36252 },
    Seg::Line(0.18083, -4.21084),
    Seg::Quad { cx: 0.18083, cy: -4.09459, x: 0.31, y: -4.09459 }
];
const P30: [Seg; 48] = [
    Seg::Move(0.0, 0.0),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.18083, y: 0.18083 },
    Seg::Line(0.18083, 0.60709),
    Seg::Quad { cx: 0.18083, cy: 0.78792, x: 0.0, y: 0.78792 },
    Seg::Line(-1.29167, 0.78792),
    Seg::Line(-2.00209, 3.43584),
    Seg::Line(-0.80084, 3.43584),
    Seg::Quad { cx: -0.62, cy: 3.43584, x: -0.62, y: 3.61668 },
    Seg::Line(-0.62, 4.04293),
    Seg::Quad { cx: -0.62, cy: 4.22376, x: -0.80084, y: 4.22376 },
    Seg::Line(-2.22167, 4.22376),
    Seg::Line(-2.71251, 6.07085),
    Seg::Quad { cx: -2.76417, cy: 6.2646, x: -2.93209, y: 6.2646 },
    Seg::Line(-3.37126, 6.2646),
    Seg::Quad { cx: -3.55209, cy: 6.2646, x: -3.50043, y: 6.0321 },
    Seg::Line(-3.00959, 4.22376),
    Seg::Line(-5.1796, 4.22376),
    Seg::Line(-5.67043, 6.07085),
    Seg::Quad { cx: -5.7221, cy: 6.2646, x: -5.90293, y: 6.2646 },
    Seg::Line(-6.31627, 6.2646),
    Seg::Quad { cx: -6.52293, cy: 6.2646, x: -6.44543, y: 6.0321 },
    Seg::Line(-5.9546, 4.22376),
    Seg::Line(-7.09127, 4.22376),
    Seg::Quad { cx: -7.2721, cy: 4.22376, x: -7.2721, y: 4.04293 },
    Seg::Line(-7.2721, 3.61668),
    Seg::Quad { cx: -7.2721, cy: 3.43584, x: -7.09127, y: 3.43584 },
    Seg::Line(-5.74793, 3.43584),
    Seg::Line(-5.03751, 0.78792),
    Seg::Line(-6.29043, 0.78792),
    Seg::Quad { cx: -6.48418, cy: 0.78792, x: -6.48418, y: 0.60709 },
    Seg::Line(-6.48418, 0.18083),
    Seg::Quad { cx: -6.48418, cy: 0.0, x: -6.29043, y: 0.0 },
    Seg::Line(-4.81793, 0.0),
    Seg::Line(-4.32709, -1.84709),
    Seg::Quad { cx: -4.28834, cy: -2.04084, x: -4.12043, y: -2.04084 },
    Seg::Line(-3.70709, -2.04084),
    Seg::Quad { cx: -3.47459, cy: -2.04084, x: -3.55209, y: -1.80834 },
    Seg::Line(-4.04293, 0.0),
    Seg::Line(-1.87292, 0.0),
    Seg::Line(-1.38209, -1.84709),
    Seg::Quad { cx: -1.3175, cy: -2.04084, x: -1.13667, y: -2.04084 },
    Seg::Line(-0.74917, -2.04084),
    Seg::Quad { cx: -0.5425, cy: -2.04084, x: -0.59417, y: -1.80834 },
    Seg::Line(-1.085, 0.0),
    Seg::Move(-2.80292, 3.43584),
    Seg::Line(-2.0925, 0.78792),
    Seg::Line(-4.24959, 0.78792),
    Seg::Line(-4.96001, 3.43584)
];
const P31: [Seg; 21] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.56833, 0.0),
    Seg::Quad { cx: -0.74917, cy: 0.0, x: -0.74917, y: -0.18083 },
    Seg::Line(-0.74917, -8.1246),
    Seg::Quad { cx: -0.74917, cy: -8.30544, x: -0.56833, y: -8.30544 },
    Seg::Line(-0.03875, -8.30544),
    Seg::Quad { cx: 0.11625, cy: -8.30544, x: 0.16792, y: -8.2021 },
    Seg::Line(3.73293, -1.9375),
    Seg::Line(3.78459, -1.9375),
    Seg::Line(3.78459, -8.1246),
    Seg::Quad { cx: 3.78459, cy: -8.30544, x: 3.96543, y: -8.30544 },
    Seg::Line(4.53376, -8.30544),
    Seg::Quad { cx: 4.7146, cy: -8.30544, x: 4.7146, y: -8.1246 },
    Seg::Line(4.7146, -0.18083),
    Seg::Quad { cx: 4.7146, cy: 0.0, x: 4.53376, y: 0.0 },
    Seg::Line(4.05584, 0.0),
    Seg::Quad { cx: 3.91376, cy: 0.0, x: 3.78459, y: -0.16792 },
    Seg::Line(0.2325, -6.36793),
    Seg::Line(0.18083, -6.36793),
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 }
];
const P32: [Seg; 29] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-1.55, 0.0),
    Seg::Quad { cx: -2.44125, cy: 0.0, x: -2.95146, y: -0.50375 },
    Seg::Quad { cx: -3.46167, cy: -1.0075, x: -3.46167, y: -1.91167 },
    Seg::Line(-3.46167, -6.39377),
    Seg::Quad { cx: -3.46167, cy: -7.29794, x: -2.95146, y: -7.80169 },
    Seg::Quad { cx: -2.44125, cy: -8.30544, x: -1.55, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.89126, cy: -8.30544, x: 1.40147, y: -7.80169 },
    Seg::Quad { cx: 1.91167, cy: -7.29794, x: 1.91167, y: -6.39377 },
    Seg::Line(1.91167, -5.76085),
    Seg::Quad { cx: 1.91167, cy: -5.58001, x: 1.73084, y: -5.58001 },
    Seg::Line(1.14959, -5.58001),
    Seg::Quad { cx: 0.96876, cy: -5.58001, x: 0.96876, y: -5.76085 },
    Seg::Line(0.96876, -6.35502),
    Seg::Quad { cx: 0.96876, cy: -7.47877, x: -0.11625, y: -7.47877 },
    Seg::Line(-1.44667, -7.47877),
    Seg::Quad { cx: -2.51875, cy: -7.47877, x: -2.51875, y: -6.35502 },
    Seg::Line(-2.51875, -1.95042),
    Seg::Quad { cx: -2.51875, cy: -0.82667, x: -1.44667, y: -0.82667 },
    Seg::Line(-0.11625, -0.82667),
    Seg::Quad { cx: 0.96876, cy: -0.82667, x: 0.96876, y: -1.95042 },
    Seg::Line(0.96876, -2.54459),
    Seg::Quad { cx: 0.96876, cy: -2.72542, x: 1.14959, y: -2.72542 },
    Seg::Line(1.73084, -2.72542),
    Seg::Quad { cx: 1.91167, cy: -2.72542, x: 1.91167, y: -2.54459 },
    Seg::Line(1.91167, -1.91167),
    Seg::Quad { cx: 1.91167, cy: -1.0075, x: 1.40147, y: -0.50375 },
    Seg::Quad { cx: 0.89126, cy: 0.0, x: 0.0, y: 0.0 }
];
const P33: [Seg; 23] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -5.07626),
    Seg::Line(-2.91917, 0.0),
    Seg::Move(0.71042, 2.46709),
    Seg::Line(0.16792, 2.46709),
    Seg::Quad { cx: 0.0, cy: 2.46709, x: 0.0, y: 2.28626 },
    Seg::Line(0.0, 0.82667),
    Seg::Line(-3.59084, 0.82667),
    Seg::Quad { cx: -3.86209, cy: 0.82667, x: -3.86209, y: 0.55542 },
    Seg::Line(-3.86209, 0.24542),
    Seg::Quad { cx: -3.86209, cy: 0.02583, x: -3.77168, y: -0.14208 },
    Seg::Line(-0.62, -5.58001),
    Seg::Quad { cx: -0.50375, cy: -5.83835, x: -0.2325, y: -5.83835 },
    Seg::Line(0.60709, -5.83835),
    Seg::Quad { cx: 0.89125, cy: -5.83835, x: 0.89125, y: -5.5671 },
    Seg::Line(0.89125, 0.0),
    Seg::Line(1.84709, 0.0),
    Seg::Quad { cx: 2.02792, cy: 0.0, x: 2.02792, y: 0.16792 },
    Seg::Line(2.02792, 0.63292),
    Seg::Quad { cx: 2.02792, cy: 0.82667, x: 1.84709, y: 0.82667 },
    Seg::Line(0.89125, 0.82667),
    Seg::Line(0.89125, 2.28626),
    Seg::Quad { cx: 0.89125, cy: 2.46709, x: 0.71042, y: 2.46709 }
];
const P34: [Seg; 38] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1.13667, 0.0),
    Seg::Quad { cx: 2.22167, cy: 0.0, x: 2.22167, y: -1.085 },
    Seg::Line(2.22167, -1.86),
    Seg::Quad { cx: 2.22167, cy: -2.97084, x: 1.13667, y: -2.97084 },
    Seg::Line(0.0, -2.97084),
    Seg::Quad { cx: -1.085, cy: -2.97084, x: -1.085, y: -1.86 },
    Seg::Line(-1.085, -1.085),
    Seg::Quad { cx: -1.085, cy: 0.0, x: 0.0, y: 0.0 },
    Seg::Move(0.0, -3.73293),
    Seg::Line(1.13667, -3.73293),
    Seg::Quad { cx: 2.22167, cy: -3.73293, x: 2.22167, y: -4.85668 },
    Seg::Line(2.22167, -5.5671),
    Seg::Quad { cx: 2.22167, cy: -6.6521, x: 1.13667, y: -6.6521 },
    Seg::Line(0.0, -6.6521),
    Seg::Quad { cx: -1.085, cy: -6.6521, x: -1.085, y: -5.5671 },
    Seg::Line(-1.085, -4.85668),
    Seg::Quad { cx: -1.085, cy: -3.73293, x: 0.0, y: -3.73293 },
    Seg::Move(1.25292, 0.82667),
    Seg::Line(-0.11625, 0.82667),
    Seg::Quad { cx: -1.0075, cy: 0.82667, x: -1.51125, y: 0.34229 },
    Seg::Quad { cx: -2.015, cy: -0.14208, x: -2.015, y: -1.02042 },
    Seg::Line(-2.015, -1.80834),
    Seg::Quad { cx: -2.015, cy: -2.97084, x: -1.11083, y: -3.35834 },
    Seg::Quad { cx: -2.015, cy: -3.70709, x: -2.015, y: -4.90834 },
    Seg::Line(-2.015, -5.63168),
    Seg::Quad { cx: -2.015, cy: -6.52293, x: -1.51125, y: -7.00085 },
    Seg::Quad { cx: -1.0075, cy: -7.47877, x: -0.11625, y: -7.47877 },
    Seg::Line(1.25292, -7.47877),
    Seg::Quad { cx: 2.14417, cy: -7.47877, x: 2.64792, y: -7.00085 },
    Seg::Quad { cx: 3.15168, cy: -6.52293, x: 3.15168, y: -5.63168 },
    Seg::Line(3.15168, -4.90834),
    Seg::Quad { cx: 3.15168, cy: -4.31418, x: 2.90626, y: -3.9073 },
    Seg::Quad { cx: 2.66084, cy: -3.50042, x: 2.24751, y: -3.35834 },
    Seg::Quad { cx: 3.15168, cy: -2.98376, x: 3.15168, y: -1.79542 },
    Seg::Line(3.15168, -1.02042),
    Seg::Quad { cx: 3.15168, cy: -0.14208, x: 2.64792, y: 0.34229 },
    Seg::Quad { cx: 2.14417, cy: 0.82667, x: 1.25292, y: 0.82667 }
];
const P35: [Seg; 38] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1.13667, 0.0),
    Seg::Quad { cx: 2.22167, cy: 0.0, x: 2.22167, y: -1.085 },
    Seg::Line(2.22167, -1.86),
    Seg::Quad { cx: 2.22167, cy: -2.97084, x: 1.13667, y: -2.97084 },
    Seg::Line(0.0, -2.97084),
    Seg::Quad { cx: -1.08501, cy: -2.97084, x: -1.08501, y: -1.86 },
    Seg::Line(-1.08501, -1.085),
    Seg::Quad { cx: -1.08501, cy: 0.0, x: 0.0, y: 0.0 },
    Seg::Move(0.0, -3.73293),
    Seg::Line(1.13667, -3.73293),
    Seg::Quad { cx: 2.22167, cy: -3.73293, x: 2.22167, y: -4.85668 },
    Seg::Line(2.22167, -5.5671),
    Seg::Quad { cx: 2.22167, cy: -6.6521, x: 1.13667, y: -6.6521 },
    Seg::Line(0.0, -6.6521),
    Seg::Quad { cx: -1.08501, cy: -6.6521, x: -1.08501, y: -5.5671 },
    Seg::Line(-1.08501, -4.85668),
    Seg::Quad { cx: -1.08501, cy: -3.73293, x: 0.0, y: -3.73293 },
    Seg::Move(1.25292, 0.82667),
    Seg::Line(-0.11625, 0.82667),
    Seg::Quad { cx: -1.00751, cy: 0.82667, x: -1.51126, y: 0.34229 },
    Seg::Quad { cx: -2.01501, cy: -0.14208, x: -2.01501, y: -1.02042 },
    Seg::Line(-2.01501, -1.80834),
    Seg::Quad { cx: -2.01501, cy: -2.97084, x: -1.11084, y: -3.35834 },
    Seg::Quad { cx: -2.01501, cy: -3.70709, x: -2.01501, y: -4.90834 },
    Seg::Line(-2.01501, -5.63168),
    Seg::Quad { cx: -2.01501, cy: -6.52293, x: -1.51126, y: -7.00085 },
    Seg::Quad { cx: -1.00751, cy: -7.47877, x: -0.11625, y: -7.47877 },
    Seg::Line(1.25292, -7.47877),
    Seg::Quad { cx: 2.14417, cy: -7.47877, x: 2.64792, y: -7.00085 },
    Seg::Quad { cx: 3.15167, cy: -6.52293, x: 3.15167, y: -5.63168 },
    Seg::Line(3.15167, -4.90834),
    Seg::Quad { cx: 3.15167, cy: -4.31418, x: 2.90625, y: -3.9073 },
    Seg::Quad { cx: 2.66084, cy: -3.50042, x: 2.2475, y: -3.35834 },
    Seg::Quad { cx: 3.15167, cy: -2.98376, x: 3.15167, y: -1.79542 },
    Seg::Line(3.15167, -1.02042),
    Seg::Quad { cx: 3.15167, cy: -0.14208, x: 2.64792, y: 0.34229 },
    Seg::Quad { cx: 2.14417, cy: 0.82667, x: 1.25292, y: 0.82667 }
];
const P36: [Seg; 23] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -5.07626),
    Seg::Line(-2.91917, 0.0),
    Seg::Move(0.71042, 2.46709),
    Seg::Line(0.16792, 2.46709),
    Seg::Quad { cx: 0.0, cy: 2.46709, x: 0.0, y: 2.28626 },
    Seg::Line(0.0, 0.82667),
    Seg::Line(-3.59084, 0.82667),
    Seg::Quad { cx: -3.86209, cy: 0.82667, x: -3.86209, y: 0.55542 },
    Seg::Line(-3.86209, 0.24542),
    Seg::Quad { cx: -3.86209, cy: 0.02583, x: -3.77167, y: -0.14208 },
    Seg::Line(-0.62, -5.58001),
    Seg::Quad { cx: -0.50375, cy: -5.83835, x: -0.2325, y: -5.83835 },
    Seg::Line(0.60709, -5.83835),
    Seg::Quad { cx: 0.89126, cy: -5.83835, x: 0.89126, y: -5.5671 },
    Seg::Line(0.89126, 0.0),
    Seg::Line(1.84709, 0.0),
    Seg::Quad { cx: 2.02793, cy: 0.0, x: 2.02793, y: 0.16792 },
    Seg::Line(2.02793, 0.63292),
    Seg::Quad { cx: 2.02793, cy: 0.82667, x: 1.84709, y: 0.82667 },
    Seg::Line(0.89126, 0.82667),
    Seg::Line(0.89126, 2.28626),
    Seg::Quad { cx: 0.89126, cy: 2.46709, x: 0.71042, y: 2.46709 }
];
const P37: [Seg; 22] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1.05917, 0.0),
    Seg::Quad { cx: 2.14418, cy: 0.0, x: 2.14418, y: -1.12375 },
    Seg::Line(2.14418, -5.52835),
    Seg::Quad { cx: 2.14418, cy: -6.6521, x: 1.05917, y: -6.6521 },
    Seg::Line(0.0, -6.6521),
    Seg::Quad { cx: -1.07208, cy: -6.6521, x: -1.07208, y: -5.52835 },
    Seg::Line(-1.07208, -1.12375),
    Seg::Quad { cx: -1.07208, cy: 0.0, x: 0.0, y: 0.0 },
    Seg::Move(1.17542, 0.82667),
    Seg::Line(-0.11625, 0.82667),
    Seg::Quad { cx: -0.99458, cy: 0.82667, x: -1.50479, y: 0.32292 },
    Seg::Quad { cx: -2.015, cy: -0.18083, x: -2.015, y: -1.085 },
    Seg::Line(-2.015, -5.5671),
    Seg::Quad { cx: -2.015, cy: -6.45835, x: -1.50479, y: -6.96856 },
    Seg::Quad { cx: -0.99458, cy: -7.47877, x: -0.11625, y: -7.47877 },
    Seg::Line(1.17542, -7.47877),
    Seg::Quad { cx: 2.05376, cy: -7.47877, x: 2.56397, y: -6.96856 },
    Seg::Quad { cx: 3.07418, cy: -6.45835, x: 3.07418, y: -5.5671 },
    Seg::Line(3.07418, -1.085),
    Seg::Quad { cx: 3.07418, cy: -0.18083, x: 2.56397, y: 0.32292 },
    Seg::Quad { cx: 2.05376, cy: 0.82667, x: 1.17542, y: 0.82667 }
];
const P38: [Seg; 30] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -1.16251),
    Seg::Quad { cx: 0.0, cy: -2.02793, x: 0.72334, y: -2.58334 },
    Seg::Line(3.21626, -4.54668),
    Seg::Quad { cx: 3.8621, cy: -5.03752, x: 3.8621, y: -5.76085 },
    Seg::Line(3.8621, -6.22585),
    Seg::Quad { cx: 3.8621, cy: -7.32377, x: 2.75126, y: -7.32377 },
    Seg::Line(1.96334, -7.32377),
    Seg::Quad { cx: 0.87834, cy: -7.32377, x: 0.87834, y: -6.22585 },
    Seg::Line(0.87834, -5.69627),
    Seg::Quad { cx: 0.87834, cy: -5.50252, x: 0.6975, y: -5.50252 },
    Seg::Line(0.11625, -5.50252),
    Seg::Quad { cx: -0.05166, cy: -5.50252, x: -0.05166, y: -5.69627 },
    Seg::Line(-0.05166, -6.23877),
    Seg::Quad { cx: -0.05166, cy: -7.13002, x: 0.45209, y: -7.62732 },
    Seg::Quad { cx: 0.95584, cy: -8.12461, x: 1.84709, y: -8.12461 },
    Seg::Line(2.88043, -8.12461),
    Seg::Quad { cx: 3.77168, cy: -8.12461, x: 4.27543, y: -7.62732 },
    Seg::Quad { cx: 4.77918, cy: -7.13002, x: 4.77918, y: -6.23877 },
    Seg::Line(4.77918, -5.70919),
    Seg::Quad { cx: 4.77918, cy: -4.70168, x: 3.97835, y: -4.06876 },
    Seg::Line(1.44667, -2.06668),
    Seg::Quad { cx: 1.02042, cy: -1.73084, x: 1.02042, y: -1.25292 },
    Seg::Line(1.02042, -0.65876),
    Seg::Line(4.62418, -0.65876),
    Seg::Quad { cx: 4.80502, cy: -0.65876, x: 4.80502, y: -0.47792 },
    Seg::Line(4.80502, 0.0),
    Seg::Quad { cx: 4.80502, cy: 0.18083, x: 4.62418, y: 0.18083 },
    Seg::Line(0.18084, 0.18083),
    Seg::Quad { cx: 0.0, cy: 0.18083, x: 0.0, y: 0.0 }
];
const P39: [Seg; 41] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, -0.41334),
    Seg::Quad { cx: 0.0, cy: -0.59417, x: 0.18084, y: -0.59417 },
    Seg::Line(0.76209, -0.59417),
    Seg::Quad { cx: 0.94292, cy: -0.59417, x: 0.94292, y: -0.41334 },
    Seg::Line(0.94292, -0.0775),
    Seg::Quad { cx: 0.94292, cy: 1.085, x: 2.09251, y: 1.085 },
    Seg::Line(3.22918, 1.085),
    Seg::Quad { cx: 4.37876, cy: 1.085, x: 4.37876, y: -0.10334 },
    Seg::Line(4.37876, -0.5425),
    Seg::Quad { cx: 4.37876, cy: -1.47251, x: 2.84168, y: -1.75667 },
    Seg::Quad { cx: 2.19584, cy: -1.87292, x: 1.55001, y: -2.02792 },
    Seg::Quad { cx: 0.90417, cy: -2.18292, x: 0.45209, y: -2.62855 },
    Seg::Quad { cx: 0.0, cy: -3.07418, x: 0.0, y: -3.84918 },
    Seg::Line(0.0, -4.4821),
    Seg::Quad { cx: 0.0, cy: -5.37335, x: 0.51021, y: -5.88356 },
    Seg::Quad { cx: 1.02042, cy: -6.39377, x: 1.91167, y: -6.39377 },
    Seg::Line(3.34543, -6.39377),
    Seg::Quad { cx: 4.22376, cy: -6.39377, x: 4.74043, y: -5.88356 },
    Seg::Quad { cx: 5.2571, cy: -5.37335, x: 5.2571, y: -4.4821 },
    Seg::Line(5.2571, -4.15918),
    Seg::Quad { cx: 5.2571, cy: -3.96543, x: 5.08918, y: -3.96543 },
    Seg::Line(4.49501, -3.96543),
    Seg::Quad { cx: 4.3271, cy: -3.96543, x: 4.3271, y: -4.15918 },
    Seg::Line(4.3271, -4.39168),
    Seg::Quad { cx: 4.3271, cy: -5.5671, x: 3.17751, y: -5.5671 },
    Seg::Line(2.07959, -5.5671),
    Seg::Quad { cx: 0.93, cy: -5.5671, x: 0.93, y: -4.34001 },
    Seg::Line(0.93, -3.82334),
    Seg::Quad { cx: 0.93, cy: -3.16459, x: 1.79542, y: -2.91918 },
    Seg::Quad { cx: 2.18292, cy: -2.81584, x: 2.64793, y: -2.73188 },
    Seg::Quad { cx: 3.11293, cy: -2.64793, x: 3.58439, y: -2.5123 },
    Seg::Quad { cx: 4.05585, cy: -2.37667, x: 4.44335, y: -2.17647 },
    Seg::Quad { cx: 4.83085, cy: -1.97626, x: 5.06981, y: -1.56938 },
    Seg::Quad { cx: 5.30877, cy: -1.1625, x: 5.30877, y: -0.58125 },
    Seg::Line(5.30877, 0.0),
    Seg::Quad { cx: 5.30877, cy: 0.89125, x: 4.7921, y: 1.40146 },
    Seg::Quad { cx: 4.27543, cy: 1.91167, x: 3.39709, y: 1.91167 },
    Seg::Line(1.91167, 1.91167),
    Seg::Quad { cx: 1.03334, cy: 1.91167, x: 0.51667, y: 1.40146 },
    Seg::Quad { cx: 0.0, cy: 0.89125, x: 0.0, y: 0.0 }
];
const P40: [Seg; 25] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-4.05585, 0.0),
    Seg::Quad { cx: -4.23668, cy: 0.0, x: -4.23668, y: -0.18083 },
    Seg::Line(-4.23668, -8.1246),
    Seg::Quad { cx: -4.23668, cy: -8.30544, x: -4.05585, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.18083, cy: -8.30544, x: 0.18083, y: -8.1246 },
    Seg::Line(0.18083, -7.6596),
    Seg::Quad { cx: 0.18083, cy: -7.47877, x: 0.0, y: -7.47877 },
    Seg::Line(-3.1646, -7.47877),
    Seg::Quad { cx: -3.29376, cy: -7.47877, x: -3.29376, y: -7.36252 },
    Seg::Line(-3.29376, -4.75335),
    Seg::Quad { cx: -3.29376, cy: -4.6371, x: -3.1646, y: -4.6371 },
    Seg::Line(-0.40042, -4.6371),
    Seg::Quad { cx: -0.21959, cy: -4.6371, x: -0.21959, y: -4.45626 },
    Seg::Line(-0.21959, -3.99126),
    Seg::Quad { cx: -0.21959, cy: -3.81043, x: -0.40042, y: -3.81043 },
    Seg::Line(-3.1646, -3.81043),
    Seg::Quad { cx: -3.29376, cy: -3.81043, x: -3.29376, y: -3.69418 },
    Seg::Line(-3.29376, -0.94292),
    Seg::Quad { cx: -3.29376, cy: -0.82667, x: -3.1646, y: -0.82667 },
    Seg::Line(0.0, -0.82667),
    Seg::Quad { cx: 0.18083, cy: -0.82667, x: 0.18083, y: -0.64583 },
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 }
];
const P41: [Seg; 29] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-1.55, 0.0),
    Seg::Quad { cx: -2.44125, cy: 0.0, x: -2.95146, y: -0.50375 },
    Seg::Quad { cx: -3.46167, cy: -1.0075, x: -3.46167, y: -1.91167 },
    Seg::Line(-3.46167, -6.39377),
    Seg::Quad { cx: -3.46167, cy: -7.29794, x: -2.95146, y: -7.80169 },
    Seg::Quad { cx: -2.44125, cy: -8.30544, x: -1.55, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.89125, cy: -8.30544, x: 1.40146, y: -7.80169 },
    Seg::Quad { cx: 1.91167, cy: -7.29794, x: 1.91167, y: -6.39377 },
    Seg::Line(1.91167, -5.76085),
    Seg::Quad { cx: 1.91167, cy: -5.58001, x: 1.73084, y: -5.58001 },
    Seg::Line(1.14959, -5.58001),
    Seg::Quad { cx: 0.96875, cy: -5.58001, x: 0.96875, y: -5.76085 },
    Seg::Line(0.96875, -6.35502),
    Seg::Quad { cx: 0.96875, cy: -7.47877, x: -0.11625, y: -7.47877 },
    Seg::Line(-1.44667, -7.47877),
    Seg::Quad { cx: -2.51875, cy: -7.47877, x: -2.51875, y: -6.35502 },
    Seg::Line(-2.51875, -1.95042),
    Seg::Quad { cx: -2.51875, cy: -0.82667, x: -1.44667, y: -0.82667 },
    Seg::Line(-0.11625, -0.82667),
    Seg::Quad { cx: 0.96875, cy: -0.82667, x: 0.96875, y: -1.95042 },
    Seg::Line(0.96875, -2.54459),
    Seg::Quad { cx: 0.96875, cy: -2.72542, x: 1.14959, y: -2.72542 },
    Seg::Line(1.73084, -2.72542),
    Seg::Quad { cx: 1.91167, cy: -2.72542, x: 1.91167, y: -2.54459 },
    Seg::Line(1.91167, -1.91167),
    Seg::Quad { cx: 1.91167, cy: -1.0075, x: 1.40146, y: -0.50375 },
    Seg::Quad { cx: 0.89125, cy: 0.0, x: 0.0, y: 0.0 }
];
const P42: [Seg; 19] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.58125, 0.0),
    Seg::Quad { cx: 0.76209, cy: 0.0, x: 0.76209, y: 0.16792 },
    Seg::Line(0.76209, 6.39377),
    Seg::Quad { cx: 0.76209, cy: 7.29794, x: 0.25833, y: 7.80169 },
    Seg::Quad { cx: -0.24542, cy: 8.30544, x: -1.13667, y: 8.30544 },
    Seg::Line(-2.57042, 8.30544),
    Seg::Quad { cx: -3.46168, cy: 8.30544, x: -3.97189, y: 7.80169 },
    Seg::Quad { cx: -4.4821, cy: 7.29794, x: -4.4821, y: 6.39377 },
    Seg::Line(-4.4821, 0.18084),
    Seg::Quad { cx: -4.4821, cy: 0.0, x: -4.30126, y: 0.0 },
    Seg::Line(-3.72001, 0.0),
    Seg::Quad { cx: -3.53918, cy: 0.0, x: -3.53918, y: 0.18084 },
    Seg::Line(-3.53918, 6.35502),
    Seg::Quad { cx: -3.53918, cy: 7.47877, x: -2.46709, y: 7.47877 },
    Seg::Line(-1.25292, 7.47877),
    Seg::Quad { cx: -0.16792, cy: 7.47877, x: -0.16792, y: 6.35502 },
    Seg::Line(-0.16792, 0.18084),
    Seg::Quad { cx: -0.16792, cy: 0.0, x: 0.0, y: 0.0 }
];
const P43: [Seg; 30] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.58126, 0.0),
    Seg::Quad { cx: -0.76209, cy: 0.0, x: -0.76209, y: -0.18083 },
    Seg::Line(-0.76209, -8.1246),
    Seg::Quad { cx: -0.76209, cy: -8.30544, x: -0.58126, y: -8.30544 },
    Seg::Line(2.38959, -8.30544),
    Seg::Quad { cx: 3.28084, cy: -8.30544, x: 3.79751, y: -7.79523 },
    Seg::Quad { cx: 4.31417, cy: -7.28502, x: 4.31417, y: -6.39377 },
    Seg::Line(4.31417, -5.21835),
    Seg::Quad { cx: 4.31417, cy: -4.49501, x: 3.96542, y: -4.02355 },
    Seg::Quad { cx: 3.61667, cy: -3.55209, x: 2.99667, y: -3.39709 },
    Seg::Line(2.99667, -3.34543),
    Seg::Line(4.49501, -0.20667),
    Seg::Quad { cx: 4.61126, cy: 0.0, x: 4.36584, y: 0.0 },
    Seg::Line(3.78459, 0.0),
    Seg::Quad { cx: 3.565, cy: 0.0, x: 3.46167, y: -0.18083 },
    Seg::Line(2.015, -3.30668),
    Seg::Line(0.31, -3.30668),
    Seg::Quad { cx: 0.18083, cy: -3.30668, x: 0.18083, y: -3.19042 },
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 },
    Seg::Move(0.31, -4.09459),
    Seg::Line(2.26042, -4.09459),
    Seg::Quad { cx: 3.37125, cy: -4.09459, x: 3.37125, y: -5.20543 },
    Seg::Line(3.37125, -6.35502),
    Seg::Quad { cx: 3.37125, cy: -7.47877, x: 2.26042, y: -7.47877 },
    Seg::Line(0.31, -7.47877),
    Seg::Quad { cx: 0.18083, cy: -7.47877, x: 0.18083, y: -7.36252 },
    Seg::Line(0.18083, -4.21084),
    Seg::Quad { cx: 0.18083, cy: -4.09459, x: 0.31, y: -4.09459 }
];
const P44: [Seg; 9] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 7.94377),
    Seg::Quad { cx: 0.0, cy: 8.1246, x: -0.18083, y: 8.1246 },
    Seg::Line(-0.76208, 8.1246),
    Seg::Quad { cx: -0.94292, cy: 8.1246, x: -0.94292, y: 7.94377 },
    Seg::Line(-0.94292, 0.0),
    Seg::Quad { cx: -0.94292, cy: -0.18084, x: -0.76208, y: -0.18084 },
    Seg::Line(-0.18083, -0.18084),
    Seg::Quad { cx: 0.0, cy: -0.18084, x: 0.0, y: 0.0 }
];
const P45: [Seg; 17] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.58125, 0.0),
    Seg::Quad { cx: -0.76208, cy: 0.0, x: -0.76208, y: -0.18083 },
    Seg::Line(-0.76208, -7.36252),
    Seg::Quad { cx: -0.76208, cy: -7.47877, x: -0.89125, y: -7.47877 },
    Seg::Line(-2.64792, -7.47877),
    Seg::Quad { cx: -2.84167, cy: -7.47877, x: -2.84167, y: -7.6596 },
    Seg::Line(-2.84167, -8.1246),
    Seg::Quad { cx: -2.84167, cy: -8.30544, x: -2.64792, y: -8.30544 },
    Seg::Line(2.06668, -8.30544),
    Seg::Quad { cx: 2.26043, cy: -8.30544, x: 2.26043, y: -8.1246 },
    Seg::Line(2.26043, -7.6596),
    Seg::Quad { cx: 2.26043, cy: -7.47877, x: 2.06668, y: -7.47877 },
    Seg::Line(0.31001, -7.47877),
    Seg::Quad { cx: 0.18084, cy: -7.47877, x: 0.18084, y: -7.36252 },
    Seg::Line(0.18084, -0.18083),
    Seg::Quad { cx: 0.18084, cy: 0.0, x: 0.0, y: 0.0 }
];
const P46: [Seg; 18] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.58125, 0.0),
    Seg::Quad { cx: -0.76209, cy: 0.0, x: -0.76209, y: -0.18083 },
    Seg::Line(-0.76209, -3.11292),
    Seg::Line(-3.21626, -8.11169),
    Seg::Quad { cx: -3.25501, cy: -8.2021, x: -3.21626, y: -8.25377 },
    Seg::Quad { cx: -3.17751, cy: -8.30544, x: -3.10001, y: -8.30544 },
    Seg::Line(-2.37668, -8.30544),
    Seg::Quad { cx: -2.23459, cy: -8.30544, x: -2.15709, y: -8.1246 },
    Seg::Line(-0.31, -4.21084),
    Seg::Line(-0.24542, -4.21084),
    Seg::Line(1.58875, -8.1246),
    Seg::Quad { cx: 1.64042, cy: -8.30544, x: 1.79542, y: -8.30544 },
    Seg::Line(2.53167, -8.30544),
    Seg::Quad { cx: 2.75125, cy: -8.30544, x: 2.64792, y: -8.11169 },
    Seg::Line(0.18083, -3.11292),
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 }
];
const P47: [Seg; 13] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-4.14626, 0.0),
    Seg::Quad { cx: -4.32709, cy: 0.0, x: -4.32709, y: -0.18083 },
    Seg::Line(-4.32709, -8.1246),
    Seg::Quad { cx: -4.32709, cy: -8.30544, x: -4.14626, y: -8.30544 },
    Seg::Line(-3.56501, -8.30544),
    Seg::Quad { cx: -3.38418, cy: -8.30544, x: -3.38418, y: -8.1246 },
    Seg::Line(-3.38418, -0.96875),
    Seg::Quad { cx: -3.38418, cy: -0.8525, x: -3.25501, y: -0.8525 },
    Seg::Line(0.0, -0.8525),
    Seg::Quad { cx: 0.18083, cy: -0.8525, x: 0.18083, y: -0.65875 },
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 }
];
const P48: [Seg; 25] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-4.05584, 0.0),
    Seg::Quad { cx: -4.23667, cy: 0.0, x: -4.23667, y: -0.18083 },
    Seg::Line(-4.23667, -8.1246),
    Seg::Quad { cx: -4.23667, cy: -8.30544, x: -4.05584, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.18084, cy: -8.30544, x: 0.18084, y: -8.1246 },
    Seg::Line(0.18084, -7.6596),
    Seg::Quad { cx: 0.18084, cy: -7.47877, x: 0.0, y: -7.47877 },
    Seg::Line(-3.16459, -7.47877),
    Seg::Quad { cx: -3.29375, cy: -7.47877, x: -3.29375, y: -7.36252 },
    Seg::Line(-3.29375, -4.75335),
    Seg::Quad { cx: -3.29375, cy: -4.6371, x: -3.16459, y: -4.6371 },
    Seg::Line(-0.40041, -4.6371),
    Seg::Quad { cx: -0.21958, cy: -4.6371, x: -0.21958, y: -4.45626 },
    Seg::Line(-0.21958, -3.99126),
    Seg::Quad { cx: -0.21958, cy: -3.81043, x: -0.40041, y: -3.81043 },
    Seg::Line(-3.16459, -3.81043),
    Seg::Quad { cx: -3.29375, cy: -3.81043, x: -3.29375, y: -3.69418 },
    Seg::Line(-3.29375, -0.94292),
    Seg::Quad { cx: -3.29375, cy: -0.82667, x: -3.16459, y: -0.82667 },
    Seg::Line(0.0, -0.82667),
    Seg::Quad { cx: 0.18084, cy: -0.82667, x: 0.18084, y: -0.64583 },
    Seg::Line(0.18084, -0.18083),
    Seg::Quad { cx: 0.18084, cy: 0.0, x: 0.0, y: 0.0 }
];
const P49: [Seg; 15] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-0.91709, 0.0),
    Seg::Quad { cx: -1.085, cy: 0.0, x: -1.13667, y: -0.18083 },
    Seg::Line(-3.42293, -8.11169),
    Seg::Quad { cx: -3.46168, cy: -8.30544, x: -3.25501, y: -8.30544 },
    Seg::Line(-2.62209, -8.30544),
    Seg::Quad { cx: -2.45417, cy: -8.30544, x: -2.41542, y: -8.1246 },
    Seg::Line(-0.49083, -1.05917),
    Seg::Line(-0.43917, -1.05917),
    Seg::Line(1.48542, -8.1246),
    Seg::Quad { cx: 1.52417, cy: -8.30544, x: 1.69209, y: -8.30544 },
    Seg::Line(2.33792, -8.30544),
    Seg::Quad { cx: 2.53167, cy: -8.30544, x: 2.49292, y: -8.11169 },
    Seg::Line(0.20667, -0.18083),
    Seg::Quad { cx: 0.155, cy: 0.0, x: 0.0, y: 0.0 }
];
const P50: [Seg; 25] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-4.05584, 0.0),
    Seg::Quad { cx: -4.23668, cy: 0.0, x: -4.23668, y: -0.18083 },
    Seg::Line(-4.23668, -8.1246),
    Seg::Quad { cx: -4.23668, cy: -8.30544, x: -4.05584, y: -8.30544 },
    Seg::Line(0.0, -8.30544),
    Seg::Quad { cx: 0.18083, cy: -8.30544, x: 0.18083, y: -8.1246 },
    Seg::Line(0.18083, -7.6596),
    Seg::Quad { cx: 0.18083, cy: -7.47877, x: 0.0, y: -7.47877 },
    Seg::Line(-3.16459, -7.47877),
    Seg::Quad { cx: -3.29376, cy: -7.47877, x: -3.29376, y: -7.36252 },
    Seg::Line(-3.29376, -4.75335),
    Seg::Quad { cx: -3.29376, cy: -4.6371, x: -3.16459, y: -4.6371 },
    Seg::Line(-0.40042, -4.6371),
    Seg::Quad { cx: -0.21958, cy: -4.6371, x: -0.21958, y: -4.45626 },
    Seg::Line(-0.21958, -3.99126),
    Seg::Quad { cx: -0.21958, cy: -3.81043, x: -0.40042, y: -3.81043 },
    Seg::Line(-3.16459, -3.81043),
    Seg::Quad { cx: -3.29376, cy: -3.81043, x: -3.29376, y: -3.69418 },
    Seg::Line(-3.29376, -0.94292),
    Seg::Quad { cx: -3.29376, cy: -0.82667, x: -3.16459, y: -0.82667 },
    Seg::Line(0.0, -0.82667),
    Seg::Quad { cx: 0.18083, cy: -0.82667, x: 0.18083, y: -0.64583 },
    Seg::Line(0.18083, -0.18083),
    Seg::Quad { cx: 0.18083, cy: 0.0, x: 0.0, y: 0.0 }
];
const P51: [Seg; 13] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(-4.14626, 0.0),
    Seg::Quad { cx: -4.32709, cy: 0.0, x: -4.32709, y: -0.18083 },
    Seg::Line(-4.32709, -8.1246),
    Seg::Quad { cx: -4.32709, cy: -8.30544, x: -4.14626, y: -8.30544 },
    Seg::Line(-3.56501, -8.30544),
    Seg::Quad { cx: -3.38417, cy: -8.30544, x: -3.38417, y: -8.1246 },
    Seg::Line(-3.38417, -0.96875),
    Seg::Quad { cx: -3.38417, cy: -0.8525, x: -3.255, y: -0.8525 },
    Seg::Line(0.0, -0.8525),
    Seg::Quad { cx: 0.18084, cy: -0.8525, x: 0.18084, y: -0.65875 },
    Seg::Line(0.18084, -0.18083),
    Seg::Quad { cx: 0.18084, cy: 0.0, x: 0.0, y: 0.0 }
];
const P52: [Seg; 2] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1524.2735, 0.0)
];
const P53: [Seg; 2] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1530.91376, 0.0)
];
const FRAME_0: [Prim; 1] = [
    shape!(P0, 119.5833, 104.5833, false, true),
];
const FRAME_1: [Prim; 1] = [
    shape!(P1, 1132.6667, 104.1667, false, true),
];
const FRAME_2: [Prim; 1] = [
    shape!(P1, 1193.5, 104.1667, false, true),
];
const FRAME_3: [Prim; 1] = [
    shape!(P2, 1254.0, 104.1667, false, true),
];
const FRAME_4: [Prim; 1] = [
    shape!(P2, 1315.0, 104.1667, false, true),
];
const NEXT: [Prim; 4] = [
    shape!(P3, 257.9, 127.6, false, true),
    shape!(P4, 319.4, 119.9, false, true),
    shape!(P5, 322.0, 105.2, false, true),
    shape!(P6, 359.1, 99.9, false, true),
];
const TECHNOLOGY: [Prim; 10] = [
    shape!(P7, 258.6138, 134.2332, false, false),
    shape!(P8, 274.3058, 134.201, false, false),
    shape!(P9, 286.1253, 134.9828, false, false),
    shape!(P10, 294.5554, 133.9894, false, false),
    shape!(P11, 307.0327, 138.967, false, false),
    shape!(P12, 321.3177, 134.0563, false, false),
    shape!(P13, 331.9303, 133.958, false, false),
    shape!(P14, 344.9854, 134.0715, false, false),
    shape!(P15, 359.2517, 134.9665, false, false),
    shape!(P16, 367.3723, 133.9065, false, false),
];
const BADGE_PRINT_0: [Prim; 2] = [
    shape!(P17, 126.9, 110.7, false, false),
    shape!(P18, 137.5, 127.9166, true, true),
];
const BADGE_PRINT_1: [Prim; 2] = [
    shape!(P17, 1140.2333, 109.8667, false, false),
    shape!(P18, 1150.8333, 127.0833, true, true),
];
const BADGE_PRINT_2: [Prim; 2] = [
    shape!(P17, 1201.0667, 109.8667, false, false),
    shape!(P19, 1208.3333, 126.6667, true, true),
];
const BADGE_PRINT_3: [Prim; 2] = [
    shape!(P17, 1261.4833, 110.7, false, false),
    shape!(P20, 1269.1667, 126.6667, true, true),
];
const BADGE_PRINT_4: [Prim; 2] = [
    shape!(P17, 1322.3167, 110.7, false, false),
    shape!(P21, 1329.5833, 126.6667, true, true),
];
const CAPTION_CUSTOMER: [Prim; 8] = [
    shape!(P22, 128.10293, 90.4167, true, true),
    shape!(P23, 136.17587, 82.11126, true, true),
    shape!(P24, 138.82379, 88.50503, true, true),
    shape!(P25, 148.1109, 90.4167, true, true),
    shape!(P26, 153.48424, 89.59003, true, true),
    shape!(P27, 164.57969, 82.11126, true, true),
    shape!(P28, 172.08429, 90.4167, true, true),
    shape!(P29, 174.80972, 90.4167, true, true),
];
const CAPTION_ID: [Prim; 9] = [
    shape!(P30, 261.65502, 84.1521, true, true),
    shape!(P31, 264.23836, 90.4167, true, true),
    shape!(P32, 274.3263, 90.4167, true, true),
    shape!(P33, 281.37882, 87.94961, true, true),
    shape!(P34, 286.58425, 89.59003, true, true),
    shape!(P35, 293.37844, 89.59003, true, true),
    shape!(P36, 301.76137, 87.94961, true, true),
    shape!(P37, 306.9668, 89.59003, true, true),
    shape!(P38, 311.59098, 90.23587, true, true),
];
const CAPTION_SECURITY: [Prim; 13] = [
    shape!(P39, 1137.5967, 88.50503, true, true),
    shape!(P40, 1149.09257, 90.4167, true, true),
    shape!(P41, 1154.38841, 90.4167, true, true),
    shape!(P42, 1162.46135, 82.11126, true, true),
    shape!(P43, 1165.96178, 90.4167, true, true),
    shape!(P44, 1173.14346, 82.2921, true, true),
    shape!(P45, 1177.2768, 90.4167, true, true),
    shape!(P46, 1183.25724, 90.4167, true, true),
    shape!(P47, 1194.8306, 90.4167, true, true),
    shape!(P48, 1200.56561, 90.4167, true, true),
    shape!(P49, 1205.24146, 90.4167, true, true),
    shape!(P50, 1213.30148, 90.4167, true, true),
    shape!(P51, 1219.59191, 90.4167, true, true),
];
const RULE_FIRST: [Prim; 1] = [
    shape!(P52, 37.87045, 187.70833, false, false),
];
const RULE_SECOND: [Prim; 1] = [
    shape!(P53, 34.52551, 187.70833, false, false),
];
const RULE_BOUNDS: (f32, f32, f32, f32) = (25.0, 174.0, 1580.0, 198.0);
const RULE_ECHOES: &[Prim] = &[
    Prim::At { x: 0.0, y: -2.86776, prims: &[
        layer!(RULE_SECOND, 3.2, 3.0, 0.01177, RULE_BOUNDS),
        layer!(RULE_SECOND, 2.0, 1.6, 0.04852, RULE_BOUNDS),
        layer!(RULE_SECOND, 1.0, 0.6, 0.09903, RULE_BOUNDS),
        layer!(RULE_SECOND, 0.35, 0.0, 0.07524, RULE_BOUNDS),
    ] },
    Prim::At { x: 0.0, y: -1.35839, prims: &[
        layer!(RULE_FIRST, 3.2, 3.0, 0.04119, RULE_BOUNDS),
        layer!(RULE_FIRST, 2.0, 1.6, 0.0093, RULE_BOUNDS),
        layer!(RULE_FIRST, 1.0, 0.6, 0.13935, RULE_BOUNDS),
        layer!(RULE_FIRST, 0.35, 0.0, 0.20266, RULE_BOUNDS),
    ] },
];
const FRAME_0_MASK: &[Prim] = &scan::<38>(107.0, 95.0, 72.0, 69.0, 94.49991, 96);
const FRAME_0_BOUNDS: (f32, f32, f32, f32) = (107.0, 95.0, 179.0, 164.0);
const FRAME_0_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAME_0_MASK, prims: &[
    Prim::At { x: -5.91878, y: -3.2386, prims: &[
        layer!(FRAME_0, 3.2, 3.0, 0.01083, FRAME_0_BOUNDS),
        layer!(FRAME_0, 2.0, 1.6, 0.04623, FRAME_0_BOUNDS),
        layer!(FRAME_0, 1.0, 0.6, 0.00436, FRAME_0_BOUNDS),
        layer!(FRAME_0, 0.35, 0.0, 0.03092, FRAME_0_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: FRAME_0_MASK, prims: &[
    Prim::At { x: -2.95939, y: -1.6193, prims: &[
        layer!(FRAME_0, 3.2, 3.0, 0.04894, FRAME_0_BOUNDS),
        layer!(FRAME_0, 2.0, 1.6, 0.06751, FRAME_0_BOUNDS),
        layer!(FRAME_0, 1.0, 0.6, 0.10433, FRAME_0_BOUNDS),
    ] },
    ] },
];
const FRAME_1_MASK: &[Prim] = &scan::<37>(1129.0, 95.0, 69.0, 69.0, 94.55531, 130);
const FRAME_1_BOUNDS: (f32, f32, f32, f32) = (1129.0, 95.0, 1198.0, 164.0);
const FRAME_1_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAME_1_MASK, prims: &[
    Prim::At { x: 2.76015, y: -3.07704, prims: &[
        layer!(FRAME_1, 2.0, 1.6, 0.00139, FRAME_1_BOUNDS),
        layer!(FRAME_1, 1.0, 0.6, 0.11464, FRAME_1_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: FRAME_1_MASK, prims: &[
    Prim::At { x: 1.38007, y: -1.53852, prims: &[
        layer!(FRAME_1, 2.0, 1.6, 0.08974, FRAME_1_BOUNDS),
        layer!(FRAME_1, 1.0, 0.6, 0.10571, FRAME_1_BOUNDS),
        layer!(FRAME_1, 0.35, 0.0, 0.09302, FRAME_1_BOUNDS),
    ] },
    ] },
];
const FRAME_2_MASK: &[Prim] = &scan::<38>(1191.0, 95.0, 69.0, 69.0, 94.48948, 129);
const FRAME_2_BOUNDS: (f32, f32, f32, f32) = (1191.0, 95.0, 1260.0, 164.0);
const FRAME_2_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAME_2_MASK, prims: &[
    Prim::At { x: 3.7227, y: -3.41163, prims: &[
        layer!(FRAME_2, 2.0, 1.6, 0.07596, FRAME_2_BOUNDS),
        layer!(FRAME_2, 1.0, 0.6, 0.05605, FRAME_2_BOUNDS),
        layer!(FRAME_2, 0.35, 0.0, 0.01059, FRAME_2_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: FRAME_2_MASK, prims: &[
    Prim::At { x: 1.86135, y: -1.70582, prims: &[
        layer!(FRAME_2, 3.2, 3.0, 0.03078, FRAME_2_BOUNDS),
        layer!(FRAME_2, 2.0, 1.6, 0.20361, FRAME_2_BOUNDS),
        layer!(FRAME_2, 1.0, 0.6, 0.13938, FRAME_2_BOUNDS),
        layer!(FRAME_2, 0.35, 0.0, 0.18791, FRAME_2_BOUNDS),
    ] },
    ] },
];
const FRAME_3_MASK: &[Prim] = &scan::<38>(1252.0, 95.0, 68.0, 69.0, 94.42174, 131);
const FRAME_3_BOUNDS: (f32, f32, f32, f32) = (1252.0, 95.0, 1320.0, 164.0);
const FRAME_3_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAME_3_MASK, prims: &[
    Prim::At { x: 4.70705, y: -3.53341, prims: &[
        layer!(FRAME_3, 2.0, 1.6, 0.03726, FRAME_3_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: FRAME_3_MASK, prims: &[
    Prim::At { x: 2.35352, y: -1.7667, prims: &[
        layer!(FRAME_3, 3.2, 3.0, 0.07456, FRAME_3_BOUNDS),
        layer!(FRAME_3, 2.0, 1.6, 0.03069, FRAME_3_BOUNDS),
        layer!(FRAME_3, 1.0, 0.6, 0.11907, FRAME_3_BOUNDS),
    ] },
    ] },
];
const FRAME_4_MASK: &[Prim] = &scan::<38>(1313.0, 95.0, 70.0, 69.0, 94.47333, 141);
const FRAME_4_BOUNDS: (f32, f32, f32, f32) = (1313.0, 95.0, 1383.0, 164.0);
const FRAME_4_ECHOES: &[Prim] = &[
    Prim::Masked { mask: FRAME_4_MASK, prims: &[
    Prim::At { x: 4.9194, y: -2.82425, prims: &[
        layer!(FRAME_4, 3.2, 3.0, 0.01705, FRAME_4_BOUNDS),
        layer!(FRAME_4, 2.0, 1.6, 0.04887, FRAME_4_BOUNDS),
        layer!(FRAME_4, 0.35, 0.0, 0.05083, FRAME_4_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: FRAME_4_MASK, prims: &[
    Prim::At { x: 2.4597, y: -1.41213, prims: &[
        layer!(FRAME_4, 3.2, 3.0, 0.02174, FRAME_4_BOUNDS),
        layer!(FRAME_4, 2.0, 1.6, 0.11447, FRAME_4_BOUNDS),
        layer!(FRAME_4, 0.35, 0.0, 0.14561, FRAME_4_BOUNDS),
    ] },
    ] },
];
const NEXT_MASK: &[Prim] = &scan::<23>(247.0, 91.0, 137.0, 41.0, 90.54794, 139);
const NEXT_BOUNDS: (f32, f32, f32, f32) = (247.0, 91.0, 384.0, 132.0);
const NEXT_ECHOES: &[Prim] = &[
    Prim::Masked { mask: NEXT_MASK, prims: &[
    Prim::At { x: -4.48282, y: -3.66158, prims: &[
        layer!(NEXT, 3.2, 3.0, 0.08673, NEXT_BOUNDS),
        layer!(NEXT, 2.0, 1.6, 0.05291, NEXT_BOUNDS),
        layer!(NEXT, 1.0, 0.6, 0.11369, NEXT_BOUNDS),
        layer!(NEXT, 0.35, 0.0, 0.0018, NEXT_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: NEXT_MASK, prims: &[
    Prim::At { x: -2.24141, y: -1.83079, prims: &[
        layer!(NEXT, 3.2, 3.0, 0.03004, NEXT_BOUNDS),
        layer!(NEXT, 2.0, 1.6, 0.09036, NEXT_BOUNDS),
        layer!(NEXT, 1.0, 0.6, 0.15912, NEXT_BOUNDS),
        layer!(NEXT, 0.35, 0.0, 0.27189, NEXT_BOUNDS),
    ] },
    ] },
];
const TECHNOLOGY_MASK: &[Prim] = &scan::<11>(247.0, 127.0, 132.0, 16.0, 126.298, 130);
const TECHNOLOGY_BOUNDS: (f32, f32, f32, f32) = (247.0, 127.0, 379.0, 143.0);
const TECHNOLOGY_ECHOES: &[Prim] = &[
    Prim::Masked { mask: TECHNOLOGY_MASK, prims: &[
    Prim::At { x: -4.6814, y: -3.39028, prims: &[
        layer!(TECHNOLOGY, 3.2, 3.0, 0.02608, TECHNOLOGY_BOUNDS),
        layer!(TECHNOLOGY, 2.0, 1.6, 0.09141, TECHNOLOGY_BOUNDS),
        layer!(TECHNOLOGY, 1.0, 0.6, 0.0986, TECHNOLOGY_BOUNDS),
        layer!(TECHNOLOGY, 0.35, 0.0, 0.027, TECHNOLOGY_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: TECHNOLOGY_MASK, prims: &[
    Prim::At { x: -2.3407, y: -1.69514, prims: &[
        layer!(TECHNOLOGY, 3.2, 3.0, 0.02447, TECHNOLOGY_BOUNDS),
        layer!(TECHNOLOGY, 2.0, 1.6, 0.10411, TECHNOLOGY_BOUNDS),
        layer!(TECHNOLOGY, 1.0, 0.6, 0.19314, TECHNOLOGY_BOUNDS),
        layer!(TECHNOLOGY, 0.35, 0.0, 0.17792, TECHNOLOGY_BOUNDS),
    ] },
    ] },
];
const BADGE_PRINT_0_MASK: &[Prim] = &scan::<22>(119.0, 105.0, 56.0, 38.0, 104.47978, 85);
const BADGE_PRINT_0_BOUNDS: (f32, f32, f32, f32) = (119.0, 105.0, 175.0, 143.0);
const BADGE_PRINT_0_ECHOES: &[Prim] = &[
    Prim::Masked { mask: BADGE_PRINT_0_MASK, prims: &[
    Prim::At { x: -6.25437, y: -3.46657, prims: &[
        layer!(BADGE_PRINT_0, 3.2, 3.0, 0.01463, BADGE_PRINT_0_BOUNDS),
        layer!(BADGE_PRINT_0, 2.0, 1.6, 0.07046, BADGE_PRINT_0_BOUNDS),
        layer!(BADGE_PRINT_0, 1.0, 0.6, 0.06925, BADGE_PRINT_0_BOUNDS),
        layer!(BADGE_PRINT_0, 0.35, 0.0, 0.05095, BADGE_PRINT_0_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: BADGE_PRINT_0_MASK, prims: &[
    Prim::At { x: -3.12719, y: -1.73329, prims: &[
        layer!(BADGE_PRINT_0, 3.2, 3.0, 0.00215, BADGE_PRINT_0_BOUNDS),
        layer!(BADGE_PRINT_0, 2.0, 1.6, 0.09112, BADGE_PRINT_0_BOUNDS),
        layer!(BADGE_PRINT_0, 1.0, 0.6, 0.15557, BADGE_PRINT_0_BOUNDS),
        layer!(BADGE_PRINT_0, 0.35, 0.0, 0.10781, BADGE_PRINT_0_BOUNDS),
    ] },
    ] },
];
const BADGE_PRINT_1_MASK: &[Prim] = &scan::<23>(1135.0, 104.0, 56.0, 39.0, 102.44643, 102);
const BADGE_PRINT_1_BOUNDS: (f32, f32, f32, f32) = (1135.0, 104.0, 1191.0, 143.0);
const BADGE_PRINT_1_ECHOES: &[Prim] = &[
    Prim::Masked { mask: BADGE_PRINT_1_MASK, prims: &[
    Prim::At { x: 3.42399, y: -3.65028, prims: &[
        layer!(BADGE_PRINT_1, 3.2, 3.0, 0.04427, BADGE_PRINT_1_BOUNDS),
        layer!(BADGE_PRINT_1, 2.0, 1.6, 0.02819, BADGE_PRINT_1_BOUNDS),
        layer!(BADGE_PRINT_1, 1.0, 0.6, 0.07436, BADGE_PRINT_1_BOUNDS),
        layer!(BADGE_PRINT_1, 0.35, 0.0, 0.06914, BADGE_PRINT_1_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: BADGE_PRINT_1_MASK, prims: &[
    Prim::At { x: 1.712, y: -1.82514, prims: &[
        layer!(BADGE_PRINT_1, 2.0, 1.6, 0.06524, BADGE_PRINT_1_BOUNDS),
        layer!(BADGE_PRINT_1, 1.0, 0.6, 0.14938, BADGE_PRINT_1_BOUNDS),
        layer!(BADGE_PRINT_1, 0.35, 0.0, 0.16479, BADGE_PRINT_1_BOUNDS),
    ] },
    ] },
];
const BADGE_PRINT_2_MASK: &[Prim] = &scan::<23>(1196.0, 104.0, 57.0, 39.0, 102.46947, 21);
const BADGE_PRINT_2_BOUNDS: (f32, f32, f32, f32) = (1196.0, 104.0, 1253.0, 143.0);
const BADGE_PRINT_2_ECHOES: &[Prim] = &[
    Prim::Masked { mask: BADGE_PRINT_2_MASK, prims: &[
    Prim::At { x: 4.13094, y: -3.36828, prims: &[
        layer!(BADGE_PRINT_2, 3.2, 3.0, 0.00313, BADGE_PRINT_2_BOUNDS),
        layer!(BADGE_PRINT_2, 2.0, 1.6, 0.04692, BADGE_PRINT_2_BOUNDS),
        layer!(BADGE_PRINT_2, 1.0, 0.6, 0.01626, BADGE_PRINT_2_BOUNDS),
        layer!(BADGE_PRINT_2, 0.35, 0.0, 0.03991, BADGE_PRINT_2_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: BADGE_PRINT_2_MASK, prims: &[
    Prim::At { x: 2.06547, y: -1.68414, prims: &[
        layer!(BADGE_PRINT_2, 3.2, 3.0, 0.00176, BADGE_PRINT_2_BOUNDS),
        layer!(BADGE_PRINT_2, 2.0, 1.6, 0.0271, BADGE_PRINT_2_BOUNDS),
        layer!(BADGE_PRINT_2, 1.0, 0.6, 0.07278, BADGE_PRINT_2_BOUNDS),
        layer!(BADGE_PRINT_2, 0.35, 0.0, 0.05692, BADGE_PRINT_2_BOUNDS),
    ] },
    ] },
];
const BADGE_PRINT_3_MASK: &[Prim] = &scan::<23>(1257.0, 104.0, 56.0, 39.0, 102.45202, 102);
const BADGE_PRINT_3_BOUNDS: (f32, f32, f32, f32) = (1257.0, 104.0, 1313.0, 143.0);
const BADGE_PRINT_3_ECHOES: &[Prim] = &[
    Prim::Masked { mask: BADGE_PRINT_3_MASK, prims: &[
    Prim::At { x: 4.80211, y: -3.56804, prims: &[
        layer!(BADGE_PRINT_3, 3.2, 3.0, 0.0397, BADGE_PRINT_3_BOUNDS),
        layer!(BADGE_PRINT_3, 2.0, 1.6, 0.04363, BADGE_PRINT_3_BOUNDS),
        layer!(BADGE_PRINT_3, 1.0, 0.6, 0.01162, BADGE_PRINT_3_BOUNDS),
        layer!(BADGE_PRINT_3, 0.35, 0.0, 0.12362, BADGE_PRINT_3_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: BADGE_PRINT_3_MASK, prims: &[
    Prim::At { x: 2.40106, y: -1.78402, prims: &[
        layer!(BADGE_PRINT_3, 3.2, 3.0, 0.01284, BADGE_PRINT_3_BOUNDS),
        layer!(BADGE_PRINT_3, 2.0, 1.6, 0.07684, BADGE_PRINT_3_BOUNDS),
        layer!(BADGE_PRINT_3, 1.0, 0.6, 0.13363, BADGE_PRINT_3_BOUNDS),
        layer!(BADGE_PRINT_3, 0.35, 0.0, 0.20732, BADGE_PRINT_3_BOUNDS),
    ] },
    ] },
];
const BADGE_PRINT_4_MASK: &[Prim] = &scan::<23>(1318.0, 104.0, 57.0, 39.0, 102.44407, 110);
const BADGE_PRINT_4_BOUNDS: (f32, f32, f32, f32) = (1318.0, 104.0, 1375.0, 143.0);
const BADGE_PRINT_4_ECHOES: &[Prim] = &[
    Prim::Masked { mask: BADGE_PRINT_4_MASK, prims: &[
    Prim::At { x: 5.88513, y: -3.61166, prims: &[
        layer!(BADGE_PRINT_4, 3.2, 3.0, 0.01001, BADGE_PRINT_4_BOUNDS),
        layer!(BADGE_PRINT_4, 2.0, 1.6, 0.03319, BADGE_PRINT_4_BOUNDS),
        layer!(BADGE_PRINT_4, 1.0, 0.6, 0.04717, BADGE_PRINT_4_BOUNDS),
        layer!(BADGE_PRINT_4, 0.35, 0.0, 0.05788, BADGE_PRINT_4_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: BADGE_PRINT_4_MASK, prims: &[
    Prim::At { x: 2.94257, y: -1.80583, prims: &[
        layer!(BADGE_PRINT_4, 3.2, 3.0, 0.02685, BADGE_PRINT_4_BOUNDS),
        layer!(BADGE_PRINT_4, 2.0, 1.6, 0.06638, BADGE_PRINT_4_BOUNDS),
        layer!(BADGE_PRINT_4, 1.0, 0.6, 0.1315, BADGE_PRINT_4_BOUNDS),
        layer!(BADGE_PRINT_4, 0.35, 0.0, 0.18911, BADGE_PRINT_4_BOUNDS),
    ] },
    ] },
];
const CAPTION_CUSTOMER_MASK: &[Prim] = &scan::<13>(113.0, 73.0, 71.0, 21.0, 72.71696, 133);
const CAPTION_CUSTOMER_BOUNDS: (f32, f32, f32, f32) = (113.0, 73.0, 184.0, 94.0);
const CAPTION_CUSTOMER_ECHOES: &[Prim] = &[
    Prim::Masked { mask: CAPTION_CUSTOMER_MASK, prims: &[
    Prim::At { x: -5.86709, y: -3.96449, prims: &[
        layer!(CAPTION_CUSTOMER, 3.2, 3.0, 0.00951, CAPTION_CUSTOMER_BOUNDS),
        layer!(CAPTION_CUSTOMER, 2.0, 1.6, 0.04619, CAPTION_CUSTOMER_BOUNDS),
        layer!(CAPTION_CUSTOMER, 1.0, 0.6, 0.07855, CAPTION_CUSTOMER_BOUNDS),
        layer!(CAPTION_CUSTOMER, 0.35, 0.0, 0.07689, CAPTION_CUSTOMER_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: CAPTION_CUSTOMER_MASK, prims: &[
    Prim::At { x: -2.93354, y: -1.98224, prims: &[
        layer!(CAPTION_CUSTOMER, 3.2, 3.0, 0.01395, CAPTION_CUSTOMER_BOUNDS),
        layer!(CAPTION_CUSTOMER, 2.0, 1.6, 0.05331, CAPTION_CUSTOMER_BOUNDS),
        layer!(CAPTION_CUSTOMER, 1.0, 0.6, 0.10691, CAPTION_CUSTOMER_BOUNDS),
        layer!(CAPTION_CUSTOMER, 0.35, 0.0, 0.1863, CAPTION_CUSTOMER_BOUNDS),
    ] },
    ] },
];
const CAPTION_ID_MASK: &[Prim] = &scan::<13>(246.0, 73.0, 77.0, 21.0, 72.72442, 143);
const CAPTION_ID_BOUNDS: (f32, f32, f32, f32) = (246.0, 73.0, 323.0, 94.0);
const CAPTION_ID_ECHOES: &[Prim] = &[
    Prim::Masked { mask: CAPTION_ID_MASK, prims: &[
    Prim::At { x: -4.65354, y: -3.99042, prims: &[
        layer!(CAPTION_ID, 2.0, 1.6, 0.06382, CAPTION_ID_BOUNDS),
        layer!(CAPTION_ID, 1.0, 0.6, 0.06802, CAPTION_ID_BOUNDS),
        layer!(CAPTION_ID, 0.35, 0.0, 0.07333, CAPTION_ID_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: CAPTION_ID_MASK, prims: &[
    Prim::At { x: -2.32677, y: -1.99521, prims: &[
        layer!(CAPTION_ID, 3.2, 3.0, 0.02373, CAPTION_ID_BOUNDS),
        layer!(CAPTION_ID, 2.0, 1.6, 0.03364, CAPTION_ID_BOUNDS),
        layer!(CAPTION_ID, 1.0, 0.6, 0.12033, CAPTION_ID_BOUNDS),
        layer!(CAPTION_ID, 0.35, 0.0, 0.20412, CAPTION_ID_BOUNDS),
    ] },
    ] },
];
const CAPTION_SECURITY_MASK: &[Prim] = &scan::<13>(1131.0, 73.0, 102.0, 21.0, 72.77484, 143);
const CAPTION_SECURITY_BOUNDS: (f32, f32, f32, f32) = (1131.0, 73.0, 1233.0, 94.0);
const CAPTION_SECURITY_ECHOES: &[Prim] = &[
    Prim::Masked { mask: CAPTION_SECURITY_MASK, prims: &[
    Prim::At { x: 3.94532, y: -3.99602, prims: &[
        layer!(CAPTION_SECURITY, 3.2, 3.0, 0.0085, CAPTION_SECURITY_BOUNDS),
        layer!(CAPTION_SECURITY, 2.0, 1.6, 0.06025, CAPTION_SECURITY_BOUNDS),
        layer!(CAPTION_SECURITY, 1.0, 0.6, 0.00301, CAPTION_SECURITY_BOUNDS),
        layer!(CAPTION_SECURITY, 0.35, 0.0, 0.12233, CAPTION_SECURITY_BOUNDS),
    ] },
    ] },
    Prim::Masked { mask: CAPTION_SECURITY_MASK, prims: &[
    Prim::At { x: 1.97266, y: -1.99801, prims: &[
        layer!(CAPTION_SECURITY, 3.2, 3.0, 0.02307, CAPTION_SECURITY_BOUNDS),
        layer!(CAPTION_SECURITY, 2.0, 1.6, 0.0691, CAPTION_SECURITY_BOUNDS),
        layer!(CAPTION_SECURITY, 1.0, 0.6, 0.12436, CAPTION_SECURITY_BOUNDS),
        layer!(CAPTION_SECURITY, 0.35, 0.0, 0.13363, CAPTION_SECURITY_BOUNDS),
    ] },
    ] },
];
/// Add to the dashboard's first existing Soft after ground and all BADGES.
/// Primary header/rule printing remains later and unchanged.
pub(super) const HEADER_ECHOES: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: RULE_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAME_0_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAME_1_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAME_2_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAME_3_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: FRAME_4_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: NEXT_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: TECHNOLOGY_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: BADGE_PRINT_0_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: BADGE_PRINT_1_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: BADGE_PRINT_2_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: BADGE_PRINT_3_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: BADGE_PRINT_4_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: CAPTION_CUSTOMER_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: CAPTION_ID_ECHOES },
    Prim::At { x: 0.0, y: 0.0, prims: CAPTION_SECURITY_ECHOES },
];
