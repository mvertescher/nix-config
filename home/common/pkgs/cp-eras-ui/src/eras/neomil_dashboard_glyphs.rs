//! Source-measured dashboard printing, docs/neomil/dashboard-trace.svg
//! #dashboard-glyph-0 through -5. Module occupancy is the observed artwork;
//! the code labels are source outlines, not font or generated QR substitutes.
use crate::palette::rgb;
use crate::style::{fill_path, fill_rect, Ink, Prim, Seg};

// Source img-07-dashboard.png at 3840×2160; coordinates in its 1600×900 trace frame.

// Upright marks are turned -45 degrees about their original tile centres.

// Filled modules are measured from the artwork, not generated QR codes.

const HUB_CODE_NUMBER: &[Seg] = &[
    Seg::Move(-20.9, 14.0),
    Seg::Line(-20.8, 12.1),
    Seg::Line(-20.4, 11.2),
    Seg::Line(-20.0, 10.8),
    Seg::Line(-19.0, 10.4),
    Seg::Line(-18.2, 10.4),
    Seg::Line(-17.3, 10.9),
    Seg::Line(-17.1, 11.7),
    Seg::Line(-17.5, 11.8),
    Seg::Line(-17.6, 12.0),
    Seg::Line(-18.0, 11.9),
    Seg::Line(-18.3, 11.5),
    Seg::Line(-19.1, 11.5),
    Seg::Line(-19.4, 11.7),
    Seg::Line(-19.6, 12.2),
    Seg::Line(-19.6, 12.8),
    Seg::Line(-18.0, 12.7),
    Seg::Line(-17.1, 13.2),
    Seg::Line(-16.8, 13.9),
    Seg::Line(-16.8, 14.8),
    Seg::Line(-17.0, 15.0),
    Seg::Line(-16.9, 15.4),
    Seg::Line(-17.1, 15.6),
    Seg::Line(-17.1, 15.9),
    Seg::Line(-17.5, 16.3),
    Seg::Line(-18.7, 16.6),
    Seg::Line(-19.4, 16.6),
    Seg::Line(-20.1, 16.3),
    Seg::Line(-20.6, 15.7),
    Seg::Line(-20.9, 14.8),
    Seg::Line(-20.9, 14.0),
    Seg::Move(-19.6, 14.5),
    Seg::Line(-19.4, 15.3),
    Seg::Line(-19.0, 15.6),
    Seg::Line(-18.4, 15.6),
    Seg::Line(-18.0, 15.1),
    Seg::Line(-18.0, 14.2),
    Seg::Line(-18.6, 13.6),
    Seg::Line(-19.2, 13.7),
    Seg::Line(-19.5, 14.0),
    Seg::Line(-19.6, 14.5),
    Seg::Move(-15.4, 12.2),
    Seg::Line(-14.8, 11.7),
    Seg::Line(-14.6, 11.7),
    Seg::Line(-13.8, 10.9),
    Seg::Line(-13.7, 10.6),
    Seg::Line(-13.2, 10.4),
    Seg::Line(-12.9, 10.4),
    Seg::Line(-12.7, 10.6),
    Seg::Line(-12.6, 15.5),
    Seg::Line(-12.9, 16.5),
    Seg::Line(-13.5, 16.7),
    Seg::Line(-13.9, 16.5),
    Seg::Line(-13.9, 12.5),
    Seg::Line(-14.3, 12.4),
    Seg::Line(-14.9, 13.0),
    Seg::Line(-15.2, 13.0),
    Seg::Line(-15.4, 12.8),
    Seg::Line(-15.4, 12.2),
    Seg::Move(-10.6, 14.3),
    Seg::Line(-10.2, 14.0),
    Seg::Line(-8.3, 14.1),
    Seg::Line(-8.2, 14.6),
    Seg::Line(-8.7, 15.0),
    Seg::Line(-9.1, 14.9),
    Seg::Line(-10.2, 15.1),
    Seg::Line(-10.5, 14.9),
    Seg::Line(-10.6, 14.3),
    Seg::Move(-7.5, 11.8),
    Seg::Line(-6.8, 10.8),
    Seg::Line(-6.0, 10.4),
    Seg::Line(-4.9, 10.5),
    Seg::Line(-4.0, 11.1),
    Seg::Line(-3.4, 12.4),
    Seg::Line(-3.6, 15.3),
    Seg::Line(-4.0, 16.0),
    Seg::Line(-4.6, 16.1),
    Seg::Line(-5.1, 16.6),
    Seg::Line(-6.2, 16.6),
    Seg::Line(-7.1, 16.1),
    Seg::Line(-7.3, 15.7),
    Seg::Line(-7.3, 15.3),
    Seg::Line(-7.1, 15.1),
    Seg::Line(-6.3, 15.1),
    Seg::Line(-5.9, 15.6),
    Seg::Line(-5.2, 15.6),
    Seg::Line(-4.7, 14.9),
    Seg::Line(-4.7, 14.4),
    Seg::Line(-6.0, 14.4),
    Seg::Line(-6.2, 14.2),
    Seg::Line(-6.8, 14.1),
    Seg::Line(-7.2, 13.7),
    Seg::Line(-7.5, 13.0),
    Seg::Line(-7.5, 11.8),
    Seg::Move(-6.3, 11.9),
    Seg::Line(-6.2, 13.0),
    Seg::Line(-5.7, 13.4),
    Seg::Line(-5.1, 13.4),
    Seg::Line(-4.7, 12.8),
    Seg::Line(-4.8, 11.9),
    Seg::Line(-5.3, 11.5),
    Seg::Line(-6.0, 11.5),
    Seg::Line(-6.3, 11.9),
    Seg::Move(-2.6, 16.3),
    Seg::Line(-2.3, 15.9),
    Seg::Line(-2.3, 15.4),
    Seg::Line(-2.1, 15.2),
    Seg::Line(-2.0, 14.5),
    Seg::Line(-1.7, 14.1),
    Seg::Line(-1.7, 13.7),
    Seg::Line(-1.4, 13.3),
    Seg::Line(-1.4, 12.9),
    Seg::Line(-0.9, 11.8),
    Seg::Line(-0.9, 11.5),
    Seg::Line(-0.7, 11.3),
    Seg::Line(-0.6, 10.7),
    Seg::Line(-0.3, 10.4),
    Seg::Line(1.2, 10.5),
    Seg::Line(1.3, 11.0),
    Seg::Line(1.6, 11.4),
    Seg::Line(1.6, 11.8),
    Seg::Line(1.9, 12.2),
    Seg::Line(1.9, 12.6),
    Seg::Line(2.2, 13.0),
    Seg::Line(2.3, 13.7),
    Seg::Line(2.5, 13.9),
    Seg::Line(2.5, 14.4),
    Seg::Line(2.8, 14.9),
    Seg::Line(3.0, 15.8),
    Seg::Line(3.3, 16.2),
    Seg::Line(3.3, 16.5),
    Seg::Line(3.1, 16.7),
    Seg::Line(2.1, 16.6),
    Seg::Line(1.9, 16.4),
    Seg::Line(1.7, 15.5),
    Seg::Line(1.3, 15.2),
    Seg::Line(0.6, 15.1),
    Seg::Line(-0.7, 15.2),
    Seg::Line(-1.2, 15.7),
    Seg::Line(-1.3, 16.3),
    Seg::Line(-1.9, 16.6),
    Seg::Line(-2.4, 16.7),
    Seg::Line(-2.6, 16.6),
    Seg::Line(-2.6, 16.3),
    Seg::Move(-0.5, 13.8),
    Seg::Line(-0.4, 14.3),
    Seg::Line(0.8, 14.3),
    Seg::Line(1.1, 14.1),
    Seg::Line(1.0, 13.3),
    Seg::Line(0.7, 12.8),
    Seg::Line(0.7, 12.3),
    Seg::Line(0.4, 11.9),
    Seg::Line(0.2, 11.9),
    Seg::Line(-0.1, 12.2),
    Seg::Line(-0.1, 12.8),
    Seg::Line(-0.3, 13.1),
    Seg::Line(-0.3, 13.6),
    Seg::Line(-0.5, 13.8),
];

const HUB_CODE_LEADING_ONE: &[Seg] = &[
    Seg::Move(-15.4, 12.2),
    Seg::Line(-14.8, 11.7),
    Seg::Line(-14.6, 11.7),
    Seg::Line(-13.8, 10.9),
    Seg::Line(-13.7, 10.6),
    Seg::Line(-13.2, 10.4),
    Seg::Line(-12.9, 10.4),
    Seg::Line(-12.7, 10.6),
    Seg::Line(-12.6, 15.5),
    Seg::Line(-12.9, 16.5),
    Seg::Line(-13.5, 16.7),
    Seg::Line(-13.9, 16.5),
    Seg::Line(-13.9, 12.5),
    Seg::Line(-14.3, 12.4),
    Seg::Line(-14.9, 13.0),
    Seg::Line(-15.2, 13.0),
    Seg::Line(-15.4, 12.8),
    Seg::Line(-15.4, 12.2),
];

const HUB_CODE_SYMBOL_STEM: &[Seg] = &[
    Seg::Move(8.8, 13.0),
    Seg::Line(8.9, 12.5),
    Seg::Line(9.6, 11.8),
    Seg::Line(10.5, 11.6),
    Seg::Line(11.1, 11.7),
    Seg::Line(12.0, 12.3),
    Seg::Line(12.3, 14.1),
    Seg::Line(13.0, 14.9),
    Seg::Line(13.2, 14.9),
    Seg::Line(13.4, 14.7),
    Seg::Line(13.3, 13.1),
    Seg::Line(12.1, 12.2),
    Seg::Line(12.0, 11.1),
    Seg::Line(12.3, 10.2),
    Seg::Line(12.8, 9.7),
    Seg::Line(14.1, 9.5),
    Seg::Line(14.9, 9.8),
    Seg::Line(15.3, 10.2),
    Seg::Line(15.6, 11.1),
    Seg::Line(15.4, 12.3),
    Seg::Line(14.5, 12.8),
    Seg::Line(14.3, 13.2),
    Seg::Line(14.4, 14.8),
    Seg::Line(14.8, 14.8),
    Seg::Line(15.3, 14.3),
    Seg::Line(15.3, 13.2),
    Seg::Line(15.5, 12.6),
    Seg::Line(16.0, 12.0),
    Seg::Line(16.6, 11.7),
    Seg::Line(17.2, 11.6),
    Seg::Line(18.2, 11.9),
    Seg::Line(18.7, 12.5),
    Seg::Line(18.9, 13.4),
    Seg::Line(18.8, 14.0),
    Seg::Line(18.1, 14.9),
    Seg::Line(17.7, 15.1),
    Seg::Line(16.7, 15.1),
    Seg::Line(16.1, 14.9),
    Seg::Line(15.4, 15.6),
    Seg::Line(14.9, 15.8),
    Seg::Line(14.4, 16.6),
    Seg::Line(14.4, 17.3),
    Seg::Line(14.2, 17.9),
    Seg::Line(14.3, 18.5),
    Seg::Line(14.0, 18.7),
    Seg::Line(13.6, 18.7),
    Seg::Line(13.3, 18.4),
    Seg::Line(13.3, 16.5),
    Seg::Line(11.6, 14.9),
    Seg::Line(10.9, 14.9),
    Seg::Line(10.4, 15.2),
    Seg::Line(9.4, 14.9),
    Seg::Line(8.8, 14.1),
    Seg::Line(8.8, 13.0),
];

const HUB_CODE_GRID_0: &[&str] = &[
    ".......#.#.#.#.#.",
    ".#####.#.##.###.#",
    ".#...#.##..#..##.",
    ".#...#.#...###..#",
    ".#...#.###..##...",
    ".#####.#.##.##..#",
    ".......#....#...#",
    "#########..##.##.",
    ".##.#...##.###..#",
    "#.#.###..#####..#",
];

const HUB_CODE_GRID_1: &[&str] = &[
    "....#...##.###...",
    ".#....##.#....###",
    ".#...#....#..#.#.",
    "##.######....#..#",
    "...###.....#.#.#.",
    "...###....##..###",
    "#....#...##..###.",
    "#..#####..###.#.#",
    "#..###.##...#.##.",
    ".##.##..#########",
];

const HUB_CODE_GRID_3: &[&str] = &[
    "...#..###.##.#.#.",
    ".###.....#....#.#",
    ".........#....##.",
    "...###.##..###.##",
    "#..###.#######.#.",
    "..########...##.#",
    ".#.#...#...#...#.",
    ".#.#...##..##..##",
    "#..#....##..#.#..",
    "##....#..########",
    "..#..###.#.......",
    "#...##.#.#.#####.",
];

const HUB_CODE_GRID_4: &[&str] = &[
    "....#...##.###...",
    ".#....##.#....###",
    ".#...#....#..#.#.",
    "##.######....#..#",
    "...###.....#.#.#.",
    "...###....##..###",
    "#....#...##..###.",
    "#..#####..###.#.#",
    "#..###.##...#.##.",
    ".##.##..#########",
    "#...#....#.......",
    "#..##.##.#.#####.",
];

// Dots uses iced's rectangle fast path, which collapses these squares
// under a -45-degree transform. A compound path preserves their geometry.
// Emit four corners per occupied source module; Path closes each subpath.
const fn module_segments(rows: &[&str]) -> usize {
    let mut n = 0;
    let mut r = 0;
    while r < rows.len() {
        let bytes = rows[r].as_bytes();
        let mut c = 0;
        while c < bytes.len() {
            if bytes[c] == b'#' { n += 4; }
            c += 1;
        }
        r += 1;
    }
    n
}
const fn modules<const N: usize>(rows: &[&str], x: f32, y: f32) -> [Seg; N] {
    let mut out = [Seg::Move(0.0, 0.0); N];
    let mut i = 0;
    let mut r = 0;
    while r < rows.len() {
        let bytes = rows[r].as_bytes();
        let mut c = 0;
        while c < bytes.len() {
            if bytes[c] == b'#' {
                let left = x + c as f32 * 2.515;
                let top = y + r as f32 * 2.515;
                out[i] = Seg::Move(left, top);
                out[i + 1] = Seg::Line(left + 2.515, top);
                out[i + 2] = Seg::Line(left + 2.515, top + 2.515);
                out[i + 3] = Seg::Line(left, top + 2.515);
                i += 4;
            }
            c += 1;
        }
        r += 1;
    }
    out
}

macro_rules! hub_glyph_0 {
    ($ink:expr) => {
    &[
        fill_path(0.0, 0.0, &modules::<{ module_segments(HUB_CODE_GRID_0) }>(HUB_CODE_GRID_0, -22.6, -20.9), $ink),
        Prim::At { x: -0.50, y: -0.40, prims: &[fill_path(0.0, 0.0, HUB_CODE_NUMBER, $ink)] },
        Prim::At { x: -0.50, y: -0.50, prims: &[fill_path(0.0, 0.0, HUB_CODE_SYMBOL_STEM, $ink)] },
        Prim::Circle { x: 13.25, y: 13.20, r: 6.15, fill: None, stroke: Some($ink), width: 0.8 },

        Prim::At { x: -10.8, y: -0.5, prims: &[fill_path(0.0, 0.0, HUB_CODE_LEADING_ONE, $ink)] },

    ]
    };
}

macro_rules! hub_glyph_1 {
    ($ink:expr) => {
    &[
        fill_path(0.0, 0.0, &modules::<{ module_segments(HUB_CODE_GRID_1) }>(HUB_CODE_GRID_1, -22.0, -20.3), $ink),
        Prim::At { x: -0.30, y: -0.30, prims: &[fill_path(0.0, 0.0, HUB_CODE_NUMBER, $ink)] },
        Prim::At { x: -0.35, y: -0.30, prims: &[fill_path(0.0, 0.0, HUB_CODE_SYMBOL_STEM, $ink)] },
        Prim::Circle { x: 13.40, y: 13.40, r: 6.15, fill: None, stroke: Some($ink), width: 0.8 },

    ]
    };
}

macro_rules! hub_glyph_2 {
    ($ink:expr) => {
    &[
        fill_path(0.0, 0.0, &modules::<{ module_segments(HUB_CODE_GRID_1) }>(HUB_CODE_GRID_1, -22.0, -20.2), $ink),
        Prim::At { x: 0.00, y: 0.00, prims: &[fill_path(0.0, 0.0, HUB_CODE_NUMBER, $ink)] },
        Prim::At { x: 0.00, y: 0.00, prims: &[fill_path(0.0, 0.0, HUB_CODE_SYMBOL_STEM, $ink)] },
        Prim::Circle { x: 13.75, y: 13.70, r: 6.15, fill: None, stroke: Some($ink), width: 0.8 },

    ]
    };
}

macro_rules! hub_glyph_3 {
    ($ink:expr) => {
    &[
        fill_path(0.0, 0.0, &modules::<{ module_segments(HUB_CODE_GRID_3) }>(HUB_CODE_GRID_3, -20.0, -20.4), $ink),
        Prim::At { x: 0.00, y: 4.60, prims: &[fill_path(0.0, 0.0, HUB_CODE_NUMBER, $ink)] },
        Prim::At { x: -0.10, y: 4.60, prims: &[fill_path(0.0, 0.0, HUB_CODE_SYMBOL_STEM, $ink)] },
        Prim::Circle { x: 13.65, y: 18.30, r: 6.15, fill: None, stroke: Some($ink), width: 0.8 },

    ]
    };
}

macro_rules! hub_glyph_4 {
    ($ink:expr) => {
    &[
        fill_path(0.0, 0.0, &modules::<{ module_segments(HUB_CODE_GRID_4) }>(HUB_CODE_GRID_4, -21.6, -20.7), $ink),
        Prim::At { x: -1.40, y: 4.70, prims: &[fill_path(0.0, 0.0, HUB_CODE_NUMBER, $ink)] },
        Prim::At { x: -1.35, y: 4.70, prims: &[fill_path(0.0, 0.0, HUB_CODE_SYMBOL_STEM, $ink)] },
        Prim::Circle { x: 12.40, y: 18.40, r: 6.15, fill: None, stroke: Some($ink), width: 0.8 },

    ]
    };
}

macro_rules! hub_glyph_5 {
    ($ink:expr) => {
    &[
        fill_path(0.0, 0.0, &modules::<{ module_segments(HUB_CODE_GRID_4) }>(HUB_CODE_GRID_4, -21.8, -20.9), $ink),
        Prim::At { x: -0.40, y: 5.70, prims: &[fill_path(0.0, 0.0, HUB_CODE_NUMBER, $ink)] },
        Prim::At { x: -0.35, y: 5.70, prims: &[fill_path(0.0, 0.0, HUB_CODE_SYMBOL_STEM, $ink)] },
        Prim::Circle { x: 13.40, y: 19.40, r: 6.15, fill: None, stroke: Some($ink), width: 0.8 },

        fill_rect(-14.255, 9.280, 5.030, 1.25, $ink),

        fill_rect(-4.195, 9.280, 7.545, 1.25, $ink),

        fill_rect(5.865, 9.280, 2.515, 1.25, $ink),

        fill_rect(15.925, 9.280, 2.515, 1.25, $ink),

    ]
    };
}

pub(super) const REST: [Prim; 6] = [
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_0!(Ink::Fixed(super::GLYPH_INK)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_1!(Ink::Fixed(super::GLYPH_INK)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_2!(Ink::Fixed(super::GLYPH_INK)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_3!(Ink::Fixed(super::GLYPH_INK)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_4!(Ink::Fixed(super::GLYPH_INK)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_5!(Ink::Fixed(super::GLYPH_INK)) },
];
pub(super) const HOVER: [Prim; 6] = [
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_0!(Ink::Fixed(rgb(0x59171b))) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_1!(Ink::Fixed(rgb(0x59171b))) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_2!(Ink::Fixed(rgb(0x59171b))) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_3!(Ink::Fixed(rgb(0x59171b))) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_4!(Ink::Fixed(rgb(0x59171b))) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_5!(Ink::Fixed(rgb(0x59171b))) },
];
pub(super) const PRESSED: [Prim; 6] = [
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_0!(Ink::Fixed(super::ON_CARD)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_1!(Ink::Fixed(super::ON_CARD)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_2!(Ink::Fixed(super::ON_CARD)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_3!(Ink::Fixed(super::ON_CARD)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_4!(Ink::Fixed(super::ON_CARD)) },
    Prim::Turn { x: 0.0, y: 0.0, angle: -45.0, prims: hub_glyph_5!(Ink::Fixed(super::ON_CARD)) },
];
