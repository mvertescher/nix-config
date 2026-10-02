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

// A narrow source-dark continuation of the A2 counter. Use semantic ground
// ink so custom palettes keep the knockout in the same role as the O ring.
const A2_COUNTER_GAP_SEGS: &[Seg] = &[Seg::Line(82.8, 0.75)];
const A2_COUNTER_GAP: Prim = Prim::Path { x: 84.6, y: 0.75,
    segs: A2_COUNTER_GAP_SEGS, close: false, fill: None, stroke: Some(Ink::Bg), width: 0.25 };

// The photographed lower A2 strokes have two short dark gaps between them.
// Preserve their bright crossing and use the existing ground role.
const A2_LOWER_GAP_SEGS: &[Seg] = &[
    Seg::Line(82.1875, 1.542),
    Seg::Move(81.875, 1.917),
    Seg::Line(81.792, 2.333),
];
const A2_LOWER_GAP: Prim = Prim::Path { x: 82.625, y: 0.708,
    segs: A2_LOWER_GAP_SEGS, close: false, fill: None,
    stroke: Some(Ink::Bg), width: 0.5 };

const O_RING_SEGS: &[Seg] = &[
    Seg::Line(95.250, 2.580),
    Seg::Line(90.250, 3.000),
    Seg::Line(89.420, 1.750),
    Seg::Line(92.330, -2.000),
    Seg::Line(94.420, -2.420),
    Seg::Move(95.250, 0.900),
    Seg::Line(93.670, 1.750),
    Seg::Line(93.500, 2.020),
    Seg::Line(92.750, 2.020),
    Seg::Line(92.550, 1.750),
    Seg::Line(90.830, 1.750),
    Seg::Line(90.830, 0.100),
    Seg::Line(92.330, -1.170),
    Seg::Line(94.420, -1.170),
];
const O_RING: Prim = Prim::Path { x: 96.5, y: 0.9, segs: O_RING_SEGS,
    close: true, fill: Some(Ink::Bg), stroke: None, width: 0.0 };

// The source plaque has broader ink across its central stripe field.
// Keep the accepted leading slant and A2 junction at the original width.
// Split each diagonal only where its width changes (local x=89 and x=99).
const HATCH_0_A_SEGS: &[Seg] = &[Seg::Line(89.000000, 0.000000)];
const HATCH_0_A: Prim = Prim::Path { x: 85.000000, y: -4.000000, segs: HATCH_0_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_0_B_SEGS: &[Seg] = &[Seg::Line(93.600000, 4.600000)];
const HATCH_0_B: Prim = Prim::Path { x: 89.000000, y: 0.000000, segs: HATCH_0_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_1_A_SEGS: &[Seg] = &[Seg::Line(89.000000, -1.500000)];
const HATCH_1_A: Prim = Prim::Path { x: 85.500000, y: -5.000000, segs: HATCH_1_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_1_B_SEGS: &[Seg] = &[Seg::Line(95.100000, 4.600000)];
const HATCH_1_B: Prim = Prim::Path { x: 89.000000, y: -1.500000, segs: HATCH_1_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_2_A_SEGS: &[Seg] = &[Seg::Line(89.000000, -3.000000)];
const HATCH_2_A: Prim = Prim::Path { x: 87.000000, y: -5.000000, segs: HATCH_2_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_2_B_SEGS: &[Seg] = &[Seg::Line(96.600000, 4.600000)];
const HATCH_2_B: Prim = Prim::Path { x: 89.000000, y: -3.000000, segs: HATCH_2_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_3_A_SEGS: &[Seg] = &[Seg::Line(89.000000, -4.500000)];
const HATCH_3_A: Prim = Prim::Path { x: 88.500000, y: -5.000000, segs: HATCH_3_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_3_B_SEGS: &[Seg] = &[Seg::Line(98.100000, 4.600000)];
const HATCH_3_B: Prim = Prim::Path { x: 89.000000, y: -4.500000, segs: HATCH_3_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_4_A_SEGS: &[Seg] = &[Seg::Line(99.000000, 4.000000)];
const HATCH_4_A: Prim = Prim::Path { x: 90.000000, y: -5.000000, segs: HATCH_4_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_4_B_SEGS: &[Seg] = &[Seg::Line(99.600000, 4.600000)];
const HATCH_4_B: Prim = Prim::Path { x: 99.000000, y: 4.000000, segs: HATCH_4_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_5_A_SEGS: &[Seg] = &[Seg::Line(99.000000, 2.500000)];
const HATCH_5_A: Prim = Prim::Path { x: 91.500000, y: -5.000000, segs: HATCH_5_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_5_B_SEGS: &[Seg] = &[Seg::Line(101.100000, 4.600000)];
const HATCH_5_B: Prim = Prim::Path { x: 99.000000, y: 2.500000, segs: HATCH_5_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_6_A_SEGS: &[Seg] = &[Seg::Line(99.000000, 1.000000)];
const HATCH_6_A: Prim = Prim::Path { x: 93.000000, y: -5.000000, segs: HATCH_6_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_6_B_SEGS: &[Seg] = &[Seg::Line(102.600000, 4.600000)];
const HATCH_6_B: Prim = Prim::Path { x: 99.000000, y: 1.000000, segs: HATCH_6_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_7_A_SEGS: &[Seg] = &[Seg::Line(99.000000, -0.500000)];
const HATCH_7_A: Prim = Prim::Path { x: 94.500000, y: -5.000000, segs: HATCH_7_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_7_B_SEGS: &[Seg] = &[Seg::Line(103.703728, 4.203728)];
const HATCH_7_B: Prim = Prim::Path { x: 99.000000, y: -0.500000, segs: HATCH_7_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_8_A_SEGS: &[Seg] = &[Seg::Line(99.000000, -2.000000)];
const HATCH_8_A: Prim = Prim::Path { x: 96.000000, y: -5.000000, segs: HATCH_8_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_8_B_SEGS: &[Seg] = &[Seg::Line(104.053262, 3.053262)];
const HATCH_8_B: Prim = Prim::Path { x: 99.000000, y: -2.000000, segs: HATCH_8_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_9_A_SEGS: &[Seg] = &[Seg::Line(99.000000, -3.500000)];
const HATCH_9_A: Prim = Prim::Path { x: 97.500000, y: -5.000000, segs: HATCH_9_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.60 };
const HATCH_9_B_SEGS: &[Seg] = &[Seg::Line(104.402796, 1.902796)];
const HATCH_9_B: Prim = Prim::Path { x: 99.000000, y: -3.500000, segs: HATCH_9_B_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_10_A_SEGS: &[Seg] = &[Seg::Line(104.752330, 0.752330)];
const HATCH_10_A: Prim = Prim::Path { x: 99.000000, y: -5.000000, segs: HATCH_10_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_11_A_SEGS: &[Seg] = &[Seg::Line(105.101864, -0.398136)];
const HATCH_11_A: Prim = Prim::Path { x: 100.500000, y: -5.000000, segs: HATCH_11_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_12_A_SEGS: &[Seg] = &[Seg::Line(105.451398, -1.548602)];
const HATCH_12_A: Prim = Prim::Path { x: 102.000000, y: -5.000000, segs: HATCH_12_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_13_A_SEGS: &[Seg] = &[Seg::Line(105.800932, -2.699068)];
const HATCH_13_A: Prim = Prim::Path { x: 103.500000, y: -5.000000, segs: HATCH_13_A_SEGS,
    close: false, fill: None, stroke: Some(Ink::Fg), width: 0.45 };
const HATCH_14_A_SEGS: &[Seg] = &[Seg::Line(106.150466, -3.849534)];
const HATCH_14_A: Prim = Prim::Path { x: 105.000000, y: -5.000000, segs: HATCH_14_A_SEGS,
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
    A2_COUNTER_GAP,
    A2_LOWER_GAP,
];

pub(super) const HATCH: &[Prim] = &[
    HATCH_0_A,
    HATCH_0_B,
    HATCH_1_A,
    HATCH_1_B,
    HATCH_2_A,
    HATCH_2_B,
    HATCH_3_A,
    HATCH_3_B,
    HATCH_4_A,
    HATCH_4_B,
    HATCH_5_A,
    HATCH_5_B,
    HATCH_6_A,
    HATCH_6_B,
    HATCH_7_A,
    HATCH_7_B,
    HATCH_8_A,
    HATCH_8_B,
    HATCH_9_A,
    HATCH_9_B,
    HATCH_10_A,
    HATCH_11_A,
    HATCH_12_A,
    HATCH_13_A,
    HATCH_14_A,
];

pub(super) const RING: &[Prim] = &[O_RING];
