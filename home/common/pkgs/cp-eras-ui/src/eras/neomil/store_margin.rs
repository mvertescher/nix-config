//! Source-derived small MASURAO margin word and short slash on STORE.
//! Six letter contours come from the clear native primary ink; the terminal
//! O/plaque hatch remains a bounded approximation. Coordinates are in the
//! existing `MARGIN_BRAND` Turn frame (1600×900 design units).
use crate::style::{Ink, Prim, Seg};


// M: 4 compact rings, 39 vertices.
const M_SEGS: &[Seg] = &[
    Seg::Line(47.125, -2.417),
    Seg::Line(44.625, -0.750),
    Seg::Line(43.792, -2.417),
    Seg::Line(42.542, -2.417),
    Seg::Line(40.250, 1.125),
    Seg::Line(41.083, 1.958),
    Seg::Line(40.458, 2.583),
    Seg::Line(40.042, 1.750),
    Seg::Line(39.417, 3.208),
    Seg::Line(40.042, 3.417),
    Seg::Line(41.500, 2.375),
    Seg::Line(41.917, 1.125),
    Seg::Line(41.292, 1.333),
    Seg::Line(41.083, 0.708),
    Seg::Line(41.708, -0.333),
    Seg::Line(43.375, 1.333),
    Seg::Line(44.625, 1.333),
    Seg::Line(45.458, 0.500),
    Seg::Line(45.667, 1.125),
    Seg::Line(44.417, 2.792),
    Seg::Line(45.042, 3.417),
    Seg::Line(46.292, 3.000),
    Seg::Move(48.792, 1.750),
    Seg::Line(48.167, 2.792),
    Seg::Line(48.792, 3.417),
    Seg::Move(46.292, 1.750),
    Seg::Line(48.167, -1.375),
    Seg::Line(47.958, -2.000),
    Seg::Line(46.083, -0.958),
    Seg::Line(46.917, -0.542),
    Seg::Line(46.083, 0.708),
    Seg::Move(44.833, 0.292),
    Seg::Line(44.000, -0.125),
    Seg::Line(43.375, -2.000),
    Seg::Line(41.917, -0.958),
    Seg::Line(42.125, -0.333),
    Seg::Line(42.958, -0.750),
    Seg::Line(43.792, 0.917),
];
const M: Prim = Prim::Path { x: 49.000, y: -2.208,
    segs: M_SEGS, close: true, fill: Some(Ink::Fg), stroke: None, width: 0.0 };

// A1: 4 compact rings, 30 vertices.
const A1_SEGS: &[Seg] = &[
    Seg::Line(50.875, 0.500),
    Seg::Line(54.625, 0.500),
    Seg::Line(55.042, 1.333),
    Seg::Line(56.083, 0.292),
    Seg::Line(56.917, -2.208),
    Seg::Line(51.292, -2.417),
    Seg::Move(51.917, -1.792),
    Seg::Line(55.458, -1.583),
    Seg::Line(55.042, 0.500),
    Seg::Line(56.500, -1.792),
    Seg::Move(54.833, -0.958),
    Seg::Line(52.125, -1.167),
    Seg::Line(51.500, -0.125),
    Seg::Line(54.208, 0.083),
    Seg::Move(48.583, 3.208),
    Seg::Line(49.625, 3.000),
    Seg::Line(50.875, 1.333),
    Seg::Line(53.375, 1.333),
    Seg::Line(53.167, 1.958),
    Seg::Line(54.000, 2.375),
    Seg::Line(52.958, 2.583),
    Seg::Line(52.958, 3.417),
    Seg::Line(54.208, 3.000),
    Seg::Line(54.833, 1.958),
    Seg::Line(54.208, 2.167),
    Seg::Line(53.792, 0.917),
    Seg::Line(50.042, 1.333),
    Seg::Line(49.625, 0.500),
    Seg::Line(48.583, 1.958),
];
const A1: Prim = Prim::Path { x: 49.833, y: -0.125,
    segs: A1_SEGS, close: true, fill: Some(Ink::Fg), stroke: None, width: 0.0 };

// S: 2 compact rings, 29 vertices.
const S_SEGS: &[Seg] = &[
    Seg::Line(59.625, -2.417),
    Seg::Line(58.167, -0.542),
    Seg::Line(58.167, 0.708),
    Seg::Line(61.292, 0.917),
    Seg::Line(61.500, 1.542),
    Seg::Line(57.542, 1.333),
    Seg::Line(56.917, 2.792),
    Seg::Line(57.542, 3.417),
    Seg::Line(60.875, 3.417),
    Seg::Line(57.958, 3.000),
    Seg::Line(57.333, 2.375),
    Seg::Line(61.917, 1.958),
    Seg::Line(62.125, 0.500),
    Seg::Line(58.583, 0.292),
    Seg::Line(59.208, -1.167),
    Seg::Line(59.625, 0.083),
    Seg::Line(63.167, 0.292),
    Seg::Line(62.750, 1.958),
    Seg::Line(61.500, 2.792),
    Seg::Line(62.542, 3.000),
    Seg::Line(63.583, 1.542),
    Seg::Line(63.583, -0.125),
    Seg::Line(59.833, -0.542),
    Seg::Line(60.458, -1.167),
    Seg::Line(64.625, -0.750),
    Seg::Move(60.250, -1.792),
    Seg::Line(64.417, -1.375),
    Seg::Line(64.208, -2.000),
];
const S: Prim = Prim::Path { x: 64.833, y: -2.208,
    segs: S_SEGS, close: true, fill: Some(Ink::Fg), stroke: None, width: 0.0 };

// U: 4 compact rings, 26 vertices.
const U_SEGS: &[Seg] = &[
    Seg::Line(72.750, 2.375),
    Seg::Line(72.958, 3.417),
    Seg::Line(74.000, 2.792),
    Seg::Move(68.583, 2.375),
    Seg::Line(70.458, 3.000),
    Seg::Line(72.750, -0.542),
    Seg::Line(71.917, -1.375),
    Seg::Line(73.583, -2.208),
    Seg::Line(72.125, -2.417),
    Seg::Line(71.083, -0.958),
    Seg::Line(71.917, -0.542),
    Seg::Line(71.500, 0.708),
    Seg::Line(70.667, 1.125),
    Seg::Line(70.875, -0.333),
    Seg::Move(68.792, -2.417),
    Seg::Line(66.917, -1.792),
    Seg::Line(66.083, -0.125),
    Seg::Line(66.917, 0.292),
    Seg::Line(66.292, 2.167),
    Seg::Line(68.167, 1.958),
    Seg::Line(66.917, 1.542),
    Seg::Move(65.875, 0.500),
    Seg::Line(64.833, 2.792),
    Seg::Line(66.708, 3.417),
    Seg::Line(65.250, 2.375),
];
const U: Prim = Prim::Path { x: 73.792, y: 0.500,
    segs: U_SEGS, close: true, fill: Some(Ink::Fg), stroke: None, width: 0.0 };

// R: 4 compact rings, 32 vertices.
const R_SEGS: &[Seg] = &[
    Seg::Line(80.667, 2.792),
    Seg::Line(81.292, 3.417),
    Seg::Line(81.917, 2.792),
    Seg::Move(81.083, -2.208),
    Seg::Line(75.458, -2.417),
    Seg::Line(72.750, 3.208),
    Seg::Line(74.208, 3.000),
    Seg::Line(75.458, 1.333),
    Seg::Line(76.917, 1.958),
    Seg::Line(76.708, 0.917),
    Seg::Line(74.000, 1.542),
    Seg::Line(75.042, -0.333),
    Seg::Line(75.458, 0.500),
    Seg::Line(79.000, 0.708),
    Seg::Line(77.958, 0.917),
    Seg::Line(78.167, 2.375),
    Seg::Line(76.917, 2.792),
    Seg::Line(77.958, 3.417),
    Seg::Line(78.583, 2.792),
    Seg::Line(78.167, 1.542),
    Seg::Line(80.667, -0.125),
    Seg::Move(75.667, -1.792),
    Seg::Line(75.875, -1.167),
    Seg::Line(79.625, -1.583),
    Seg::Line(79.625, 0.083),
    Seg::Line(80.667, -1.375),
    Seg::Line(80.458, -2.000),
    Seg::Move(79.000, -0.958),
    Seg::Line(76.708, -1.167),
    Seg::Line(76.083, -0.125),
    Seg::Line(78.375, 0.083),
];
const R: Prim = Prim::Path { x: 81.708, y: 0.917,
    segs: R_SEGS, close: true, fill: Some(Ink::Fg), stroke: None, width: 0.0 };

// A2: 2 compact rings, 25 vertices.
const A2_SEGS: &[Seg] = &[
    Seg::Line(87.125, -2.417),
    Seg::Line(84.208, -2.417),
    Seg::Line(82.750, -0.958),
    Seg::Line(80.667, 2.792),
    Seg::Line(81.292, 3.417),
    Seg::Line(83.375, 1.333),
    Seg::Line(85.875, 1.333),
    Seg::Line(85.458, 3.417),
    Seg::Line(89.000, 3.208),
    Seg::Line(88.792, -1.167),
    Seg::Line(86.292, 3.000),
    Seg::Line(86.292, 0.917),
    Seg::Line(82.750, 0.708),
    Seg::Line(83.375, -0.750),
    Seg::Line(83.792, 0.500),
    Seg::Line(87.125, 0.500),
    Seg::Line(87.750, -0.542),
    Seg::Line(87.333, -1.792),
    Seg::Line(88.375, -2.833),
    Seg::Line(89.000, -2.625),
    Seg::Move(86.500, -0.958),
    Seg::Line(84.625, -1.167),
    Seg::Line(84.000, -0.125),
    Seg::Line(85.875, 0.083),
];
const A2: Prim = Prim::Path { x: 88.792, y: -4.917,
    segs: A2_SEGS, close: true, fill: Some(Ink::Fg), stroke: None, width: 0.0 };

const O_RING_SEGS: &[Seg] = &[
    Seg::Line(95.250, 2.580),
    Seg::Line(90.250, 3.000),
    Seg::Line(89.420, 1.750),
    Seg::Line(92.330, -2.000),
    Seg::Line(94.420, -2.420),
    Seg::Move(95.250, 0.900),
    Seg::Line(93.670, 1.750),
    Seg::Line(90.830, 1.750),
    Seg::Line(90.830, 0.100),
    Seg::Line(92.330, -1.170),
    Seg::Line(94.420, -1.170),
];
const O_RING: Prim = Prim::Path { x: 96.5, y: 0.9, segs: O_RING_SEGS,
    close: true, fill: Some(Ink::Bg), stroke: None, width: 0.0 };

// Source leading edge slopes from local (106.5, -5) to (103.583333, 4.6).
// Trim only hatch tails at this edge; primary letters, ring and cadence stay fixed.
const HATCH_0_SEGS: &[Seg] = &[Seg::Line(93.600, 4.600)];
const HATCH_0: Prim = Prim::Path { x: 85.000, y: -4.000, segs: HATCH_0_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_1_SEGS: &[Seg] = &[Seg::Line(95.100, 4.600)];
const HATCH_1: Prim = Prim::Path { x: 85.500, y: -5.000, segs: HATCH_1_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_2_SEGS: &[Seg] = &[Seg::Line(96.600, 4.600)];
const HATCH_2: Prim = Prim::Path { x: 87.000, y: -5.000, segs: HATCH_2_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_3_SEGS: &[Seg] = &[Seg::Line(98.100, 4.600)];
const HATCH_3: Prim = Prim::Path { x: 88.500, y: -5.000, segs: HATCH_3_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_4_SEGS: &[Seg] = &[Seg::Line(99.600, 4.600)];
const HATCH_4: Prim = Prim::Path { x: 90.000, y: -5.000, segs: HATCH_4_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_5_SEGS: &[Seg] = &[Seg::Line(101.100, 4.600)];
const HATCH_5: Prim = Prim::Path { x: 91.500, y: -5.000, segs: HATCH_5_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_6_SEGS: &[Seg] = &[Seg::Line(102.600, 4.600)];
const HATCH_6: Prim = Prim::Path { x: 93.000, y: -5.000, segs: HATCH_6_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_7_SEGS: &[Seg] = &[Seg::Line(103.703728, 4.203728)];
const HATCH_7: Prim = Prim::Path { x: 94.500, y: -5.000, segs: HATCH_7_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_8_SEGS: &[Seg] = &[Seg::Line(104.053262, 3.053262)];
const HATCH_8: Prim = Prim::Path { x: 96.000, y: -5.000, segs: HATCH_8_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_9_SEGS: &[Seg] = &[Seg::Line(104.402796, 1.902796)];
const HATCH_9: Prim = Prim::Path { x: 97.500, y: -5.000, segs: HATCH_9_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_10_SEGS: &[Seg] = &[Seg::Line(104.752330, 0.752330)];
const HATCH_10: Prim = Prim::Path { x: 99.000, y: -5.000, segs: HATCH_10_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_11_SEGS: &[Seg] = &[Seg::Line(105.101864, -0.398136)];
const HATCH_11: Prim = Prim::Path { x: 100.500, y: -5.000, segs: HATCH_11_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_12_SEGS: &[Seg] = &[Seg::Line(105.451398, -1.548602)];
const HATCH_12: Prim = Prim::Path { x: 102.000, y: -5.000, segs: HATCH_12_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_13_SEGS: &[Seg] = &[Seg::Line(105.800932, -2.699068)];
const HATCH_13: Prim = Prim::Path { x: 103.500, y: -5.000, segs: HATCH_13_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_14_SEGS: &[Seg] = &[Seg::Line(106.150466, -3.849534)];
const HATCH_14: Prim = Prim::Path { x: 105.000, y: -5.000, segs: HATCH_14_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };

const SLASH_SEGS: &[Seg] = &[Seg::Line(3.0, 5.06)];
const SLASH: Prim = Prim::Path { x: 8.94, y: -5.19, segs: SLASH_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 1.8 };

pub(super) const PRIMARY: &[Prim] = &[
    SLASH,
    M,
    A1,
    S,
    U,
    R,
    A2,
];

pub(super) const HATCH: &[Prim] = &[
    HATCH_0,
    HATCH_1,
    HATCH_2,
    HATCH_3,
    HATCH_4,
    HATCH_5,
    HATCH_6,
    HATCH_7,
    HATCH_8,
    HATCH_9,
    HATCH_10,
    HATCH_11,
    HATCH_12,
    HATCH_13,
    HATCH_14,
];

pub(super) const RING: &[Prim] = &[O_RING];
