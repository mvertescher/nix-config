//! Dashboard secondary printing measured from source screen #60.
//! Reusable smooth geometric and chamfered monospace centre lines; these
//! are a compact reconstruction of the visible letterforms, not a font claim.
use crate::style::{Ink, Prim, Seg};
use crate::palette::rgb;

const CODE: Ink = Ink::Fixed(rgb(0x671b21));

// Transform reusable glyph centre lines at compile time. Stroke widths stay
// in design units, so horizontal fitting does not distort the stroke.
const fn placed<const N: usize>(mut segs: [Seg; N], x: f32, y: f32, w: f32, h: f32) -> [Seg; N] {
    let mut i = 0;
    while i < N {
        segs[i] = match segs[i] {
            Seg::Move(a, b) => Seg::Move(x + a * w, y + b * h),
            Seg::Line(a, b) => Seg::Line(x + a * w, y + b * h),
            Seg::Quad { cx, cy, x: a, y: b } => Seg::Quad { cx: x + cx * w, cy: y + cy * h, x: x + a * w, y: y + b * h },
            Seg::Cubic { c1x, c1y, c2x, c2y, x: a, y: b } => Seg::Cubic { c1x: x + c1x * w, c1y: y + c1y * h, c2x: x + c2x * w, c2y: y + c2y * h, x: x + a * w, y: y + b * h },
        };
        i += 1;
    }
    segs
}

macro_rules! letter {
    ($glyph:ident, $x:expr, $y:expr, $w:expr, $h:expr, $stroke:expr, $ink:expr) => {
        Prim::Path { x: 0.0, y: 0.0, segs: &placed($glyph, $x, $y, $w, $h), close: false, fill: None, stroke: Some($ink), width: $stroke }
    };
}

const GEO_T: [Seg; 4] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(1.0, 0.0),
    Seg::Move(0.5, 0.0),
    Seg::Line(0.5, 1.0)
];

const GEO_E: [Seg; 6] = [
    Seg::Move(1.0, 0.0),
    Seg::Line(0.0, 0.0),
    Seg::Line(0.0, 1.0),
    Seg::Line(1.0, 1.0),
    Seg::Move(0.0, 0.49),
    Seg::Line(0.82, 0.49)
];

const GEO_C: [Seg; 4] = [
    Seg::Move(1.0, 0.17),
    Seg::Cubic { c1x: 0.8, c1y: -0.08, c2x: 0.26, c2y: -0.08, x: 0.08, y: 0.19 },
    Seg::Cubic { c1x: -0.07, c1y: 0.39, c2x: -0.07, c2y: 0.71, x: 0.09, y: 0.87 },
    Seg::Cubic { c1x: 0.31, c1y: 1.09, c2x: 0.78, c2y: 1.04, x: 1.0, y: 0.83 }
];

const GEO_H: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.0),
    Seg::Move(1.0, 0.0),
    Seg::Line(1.0, 1.0),
    Seg::Move(0.0, 0.5),
    Seg::Line(1.0, 0.5)
];

const GEO_N: [Seg; 6] = [
    Seg::Move(0.0, 1.0),
    Seg::Line(0.0, 0.0),
    Seg::Move(0.0, 0.0),
    Seg::Line(1.0, 1.0),
    Seg::Move(1.0, 1.0),
    Seg::Line(1.0, 0.0)
];

const GEO_O: [Seg; 5] = [
    Seg::Move(0.5, 0.0),
    Seg::Cubic { c1x: 0.16, c1y: 0.0, c2x: 0.0, c2y: 0.18, x: 0.0, y: 0.5 },
    Seg::Cubic { c1x: 0.0, c1y: 0.82, c2x: 0.16, c2y: 1.0, x: 0.5, y: 1.0 },
    Seg::Cubic { c1x: 0.84, c1y: 1.0, c2x: 1.0, c2y: 0.82, x: 1.0, y: 0.5 },
    Seg::Cubic { c1x: 1.0, c1y: 0.18, c2x: 0.84, c2y: 0.0, x: 0.5, y: 0.0 }
];

const GEO_L: [Seg; 3] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.0),
    Seg::Line(1.0, 1.0)
];

const GEO_G: [Seg; 8] = [
    Seg::Move(1.0, 0.17),
    Seg::Cubic { c1x: 0.8, c1y: -0.08, c2x: 0.26, c2y: -0.08, x: 0.08, y: 0.19 },
    Seg::Cubic { c1x: -0.07, c1y: 0.39, c2x: -0.07, c2y: 0.71, x: 0.09, y: 0.87 },
    Seg::Cubic { c1x: 0.31, c1y: 1.09, c2x: 0.78, c2y: 1.04, x: 1.0, y: 0.83 },
    Seg::Line(1.0, 0.54),
    Seg::Line(0.58, 0.54),
    Seg::Move(1.0, 0.83),
    Seg::Line(1.0, 1.0)
];

const GEO_Y: [Seg; 5] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.5, 0.58),
    Seg::Line(1.0, 0.0),
    Seg::Move(0.5, 0.58),
    Seg::Line(0.5, 1.0)
];

const MONO_J: [Seg; 4] = [
    Seg::Move(0.95, 0.0),
    Seg::Line(0.95, 0.78),
    Seg::Quad { cx: 0.95, cy: 1.0, x: 0.5, y: 1.0 },
    Seg::Quad { cx: 0.15, cy: 1.0, x: 0.0, y: 0.84 }
];

const MONO_H: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.0),
    Seg::Move(1.0, 0.0),
    Seg::Line(1.0, 1.0),
    Seg::Move(0.0, 0.5),
    Seg::Line(1.0, 0.5)
];

const MONO_N: [Seg; 6] = [
    Seg::Move(0.0, 1.0),
    Seg::Line(0.0, 0.0),
    Seg::Move(0.0, 0.0),
    Seg::Line(1.0, 1.0),
    Seg::Move(1.0, 1.0),
    Seg::Line(1.0, 0.0)
];

const MONO_1: [Seg; 5] = [
    Seg::Move(0.1, 0.0),
    Seg::Line(0.46, 0.0),
    Seg::Line(0.46, 1.0),
    Seg::Move(0.0, 1.0),
    Seg::Line(1.0, 1.0)
];

const MONO_0: [Seg; 9] = [
    Seg::Move(0.24, 0.0),
    Seg::Line(0.76, 0.0),
    Seg::Line(1.0, 0.15),
    Seg::Line(1.0, 0.85),
    Seg::Line(0.76, 1.0),
    Seg::Line(0.24, 1.0),
    Seg::Line(0.0, 0.85),
    Seg::Line(0.0, 0.15),
    Seg::Line(0.24, 0.0)
];

const MONO_2: [Seg; 10] = [
    Seg::Move(0.0, 0.16),
    Seg::Line(0.24, 0.0),
    Seg::Line(0.76, 0.0),
    Seg::Line(1.0, 0.15),
    Seg::Line(1.0, 0.38),
    Seg::Line(0.8, 0.52),
    Seg::Line(0.18, 0.76),
    Seg::Line(0.0, 0.86),
    Seg::Line(0.0, 1.0),
    Seg::Line(1.0, 1.0)
];

const MONO_C: [Seg; 8] = [
    Seg::Move(1.0, 0.16),
    Seg::Line(0.76, 0.0),
    Seg::Line(0.24, 0.0),
    Seg::Line(0.0, 0.16),
    Seg::Line(0.0, 0.84),
    Seg::Line(0.24, 1.0),
    Seg::Line(0.76, 1.0),
    Seg::Line(1.0, 0.84)
];

const MONO_K: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.0),
    Seg::Move(1.0, 0.0),
    Seg::Line(0.0, 0.61),
    Seg::Move(0.39, 0.37),
    Seg::Line(1.0, 1.0)
];

const MONO_5: [Seg; 9] = [
    Seg::Move(1.0, 0.0),
    Seg::Line(0.0, 0.0),
    Seg::Line(0.0, 0.46),
    Seg::Line(0.75, 0.46),
    Seg::Line(1.0, 0.62),
    Seg::Line(1.0, 0.85),
    Seg::Line(0.76, 1.0),
    Seg::Line(0.24, 1.0),
    Seg::Line(0.0, 0.84)
];

const MONO_A: [Seg; 6] = [
    Seg::Move(0.0, 1.0),
    Seg::Line(0.4, 0.0),
    Seg::Line(0.6, 0.0),
    Seg::Line(1.0, 1.0),
    Seg::Move(0.19, 0.66),
    Seg::Line(0.81, 0.66)
];

const MONO_S: [Seg; 12] = [
    Seg::Move(1.0, 0.16),
    Seg::Line(0.76, 0.0),
    Seg::Line(0.24, 0.0),
    Seg::Line(0.0, 0.16),
    Seg::Line(0.0, 0.35),
    Seg::Line(0.24, 0.5),
    Seg::Line(0.76, 0.5),
    Seg::Line(1.0, 0.65),
    Seg::Line(1.0, 0.84),
    Seg::Line(0.76, 1.0),
    Seg::Line(0.24, 1.0),
    Seg::Line(0.0, 0.84)
];

const GEO_K: [Seg; 6] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.0),
    Seg::Move(1.0, 0.0),
    Seg::Line(0.0, 0.58),
    Seg::Move(0.42, 0.35),
    Seg::Line(1.0, 1.0)
];

const GEO_I: [Seg; 2] = [
    Seg::Move(0.0, 0.0),
    Seg::Line(0.0, 1.0)
];

const GEO_R: [Seg; 7] = [
    Seg::Move(0.0, 1.0),
    Seg::Line(0.0, 0.0),
    Seg::Line(0.62, 0.0),
    Seg::Cubic { c1x: 1.0, c1y: 0.0, c2x: 1.0, c2y: 0.45, x: 0.62, y: 0.45 },
    Seg::Line(0.0, 0.45),
    Seg::Move(0.44, 0.45),
    Seg::Line(1.0, 1.0)
];

const GEO_OPTICAL_O: [Seg; 7] = [
    Seg::Move(0.5, 0.0),
    Seg::Cubic { c1x: 0.16, c1y: 0.0, c2x: 0.0, c2y: 0.18, x: 0.0, y: 0.5 },
    Seg::Cubic { c1x: 0.0, c1y: 0.82, c2x: 0.16, c2y: 1.0, x: 0.5, y: 1.0 },
    Seg::Cubic { c1x: 0.84, c1y: 1.0, c2x: 1.0, c2y: 0.82, x: 1.0, y: 0.5 },
    Seg::Cubic { c1x: 1.0, c1y: 0.18, c2x: 0.84, c2y: 0.0, x: 0.5, y: 0.0 },
    Seg::Move(0.73, 0.04),
    Seg::Cubic { c1x: 0.36, c1y: 0.22, c2x: 0.34, c2y: 0.76, x: 0.72, y: 0.97 }
];

const GEO_S: [Seg; 5] = [
    Seg::Move(1.0, 0.17),
    Seg::Cubic { c1x: 0.8, c1y: -0.08, c2x: 0.22, c2y: -0.05, x: 0.03, y: 0.18 },
    Seg::Cubic { c1x: -0.12, c1y: 0.42, c2x: 0.29, c2y: 0.46, x: 0.51, y: 0.5 },
    Seg::Cubic { c1x: 0.83, c1y: 0.56, c2x: 1.1, c2y: 0.6, x: 0.98, y: 0.84 },
    Seg::Cubic { c1x: 0.78, c1y: 1.09, c2x: 0.24, c2y: 1.07, x: 0.0, y: 0.83 }
];

pub const TECHNOLOGY: &[Prim] = &[
    letter!(GEO_T, 258.6138, 134.2332, 4.2584, 4.9291, 0.7454, Ink::Fg),
    letter!(GEO_E, 270.9663, 134.201, 3.3395, 4.5904, 0.7454, Ink::Fg),
    letter!(GEO_C, 282.2557, 134.1952, 3.8696, 4.633, 0.835, Ink::Fg),
    letter!(GEO_H, 294.5554, 133.9894, 3.7739, 5.0474, 0.7455, Ink::Fg),
    letter!(GEO_N, 307.0327, 133.9787, 3.7898, 4.9883, 0.8123, Ink::Fg),
    letter!(GEO_O, 319.2593, 134.0563, 4.1169, 4.8692, 0.8045, Ink::Fg),
    letter!(GEO_L, 331.9303, 133.958, 3.484, 4.8004, 0.7454, Ink::Fg),
    letter!(GEO_O, 342.8387, 134.0715, 4.2933, 4.8339, 0.7554, Ink::Fg),
    letter!(GEO_G, 355.4104, 134.1768, 3.8413, 4.6449, 0.8433, Ink::Fg),
    letter!(GEO_Y, 367.3723, 133.9065, 4.0225, 5.2506, 0.7454, Ink::Fg),
];

pub const TAPE: &[Prim] = &[
    letter!(MONO_J, 283.5976, 152.3466, 1.956, 4.0394, 0.5306, CODE),
    letter!(MONO_H, 287.0488, 152.0963, 2.8019, 4.467, 0.5315, CODE),
    letter!(MONO_N, 291.7423, 152.0011, 2.5259, 4.5917, 0.5305, CODE),
    letter!(MONO_1, 298.1854, 152.0936, 2.2513, 4.1179, 0.6634, CODE),
    letter!(MONO_0, 301.9625, 152.0899, 2.2865, 4.2674, 0.6634, CODE),
    letter!(MONO_2, 306.0346, 152.0594, 2.3501, 4.2339, 0.5316, CODE),
    letter!(MONO_C, 312.0039, 152.1305, 2.4425, 4.2531, 0.5998, CODE),
    letter!(MONO_K, 316.1312, 152.1464, 2.2589, 4.4792, 0.5328, CODE),
    letter!(MONO_C, 320.3431, 152.22, 2.5618, 4.1681, 0.5929, CODE),
    letter!(MONO_1, 326.4506, 152.1245, 2.3731, 4.0587, 0.5315, CODE),
    letter!(MONO_5, 330.5068, 152.3485, 2.2896, 4.0291, 0.7156, CODE),
    letter!(MONO_1, 334.581, 152.1334, 2.4657, 4.1025, 0.5319, CODE),
    letter!(MONO_C, 340.345, 152.1713, 2.4886, 4.1624, 0.6019, CODE),
    letter!(MONO_C, 344.5067, 152.0224, 2.4585, 4.3036, 0.5317, CODE),
    letter!(MONO_1, 348.7721, 152.1601, 2.3781, 4.0365, 0.5302, CODE),
    letter!(MONO_0, 352.6322, 152.1186, 2.3904, 4.3061, 0.5318, CODE),
    letter!(MONO_5, 358.8408, 152.3388, 2.2692, 4.0297, 0.7187, CODE),
    letter!(MONO_1, 362.6669, 152.0951, 2.8482, 4.1398, 0.6634, CODE),
    letter!(MONO_1, 367.0377, 152.1485, 1.9907, 4.0736, 0.5304, CODE),
    letter!(MONO_1, 370.8294, 152.0914, 2.5456, 4.0372, 0.5429, CODE),
];

pub const LEFT_CODE: &[Prim] = &[
    Prim::Turn { x: 52.9167, y: 460.0, angle: 90.0, prims: &[
        letter!(MONO_J, 3.0375, 1.9194, 2.8035, 5.336, 0.5301, Ink::Fg),
        letter!(MONO_H, 7.7407, 1.6619, 3.3565, 5.6697, 0.6634, Ink::Fg),
        letter!(MONO_N, 13.7012, 1.8243, 3.2724, 5.6439, 0.5308, Ink::Fg),
        letter!(MONO_1, 22.2083, 1.7879, 3.0269, 5.3509, 0.6634, Ink::Fg),
        letter!(MONO_0, 27.0191, 1.9283, 3.0537, 5.2995, 0.6089, Ink::Fg),
        letter!(MONO_2, 32.2413, 1.8167, 3.1401, 5.154, 0.6634, Ink::Fg),
        letter!(MONO_C, 40.3023, 1.9845, 3.4488, 5.236, 0.5316, Ink::Fg),
        letter!(MONO_K, 45.6591, 2.0045, 2.8292, 5.4491, 0.6634, Ink::Fg),
        letter!(MONO_C, 50.7307, 1.9847, 3.5243, 5.1726, 0.5311, Ink::Fg),
        letter!(MONO_1, 58.9878, 1.8228, 2.9153, 5.3568, 0.6636, Ink::Fg),
        letter!(MONO_5, 64.2026, 1.9127, 3.0142, 5.275, 0.6048, Ink::Fg),
        letter!(MONO_1, 69.4409, 1.8225, 2.7477, 5.3285, 0.7193, Ink::Fg),
        letter!(MONO_C, 76.9994, 2.0028, 3.4974, 5.1753, 0.5315, Ink::Fg),
        letter!(MONO_C, 82.3941, 1.8831, 3.4452, 5.3134, 0.531, Ink::Fg),
        letter!(MONO_1, 88.1035, 1.8725, 2.9628, 5.2912, 0.6322, Ink::Fg),
        letter!(MONO_0, 93.2562, 1.9266, 3.0834, 5.3045, 0.6084, Ink::Fg),
        letter!(MONO_A, 100.2693, 2.2699, 4.1989, 5.4259, 0.5265, Ink::Fg),
        letter!(MONO_S, 106.5775, 1.9499, 3.5163, 5.2747, 0.5316, Ink::Fg),
        letter!(MONO_5, 112.1792, 1.9221, 2.9769, 5.2978, 0.6016, Ink::Fg),
    ] },
];

pub const RIGHT_CODE: &[Prim] = &[
    Prim::Turn { x: 1544.1667, y: 670.8333, angle: -90.0, prims: &[
        letter!(MONO_J, 2.5413, 1.9434, 2.843, 5.3181, 0.5321, Ink::Fg),
        letter!(MONO_H, 7.4117, 1.8038, 3.2424, 5.3725, 0.5325, Ink::Fg),
        letter!(MONO_N, 13.2612, 1.8566, 3.2645, 5.5153, 0.5326, Ink::Fg),
        letter!(MONO_1, 21.8498, 1.8557, 2.8092, 5.3016, 0.5316, Ink::Fg),
        letter!(MONO_0, 26.5436, 1.8998, 3.0477, 5.3948, 0.5304, Ink::Fg),
        letter!(MONO_2, 31.7371, 1.8283, 3.287, 5.1179, 0.5321, Ink::Fg),
        letter!(MONO_C, 39.4285, 1.8809, 3.421, 5.3193, 0.5314, Ink::Fg),
        letter!(MONO_K, 44.8093, 1.8815, 2.9185, 5.6137, 0.5306, Ink::Fg),
        letter!(MONO_C, 50.249, 1.9834, 3.4869, 5.2372, 0.5318, Ink::Fg),
        letter!(MONO_1, 58.5477, 1.8606, 2.8866, 5.3178, 0.5313, Ink::Fg),
        letter!(MONO_5, 63.6441, 1.9724, 3.1548, 5.1442, 0.5402, Ink::Fg),
        letter!(MONO_1, 68.8699, 1.8103, 3.0168, 5.3418, 0.5349, Ink::Fg),
        letter!(MONO_C, 76.5962, 1.983, 3.3707, 5.2257, 0.532, Ink::Fg),
        letter!(MONO_C, 82.0124, 1.9939, 3.3705, 5.2277, 0.5319, Ink::Fg),
        letter!(MONO_1, 87.6605, 1.8308, 2.9799, 5.3668, 0.5308, Ink::Fg),
        letter!(MONO_0, 92.2956, 1.9307, 3.175, 5.3272, 0.5307, Ink::Fg),
        letter!(MONO_A, 99.9494, 2.219, 4.0581, 5.4169, 0.537, Ink::Fg),
        letter!(MONO_S, 105.6495, 1.9655, 3.5334, 5.2856, 0.5315, Ink::Fg),
        letter!(MONO_5, 111.6558, 1.9323, 2.9826, 5.334, 0.531, Ink::Fg),
    ] },
];

pub const KIROSHI: &[Prim] = &[
    Prim::Turn { x: 1542.9167, y: 765.8333, angle: -90.0, prims: &[
        letter!(GEO_K, 3.3168, 2.7426, 4.235, 5.449, 0.6642, Ink::Fg),
        letter!(GEO_I, 9.7535, 2.6631, 0.5392, 5.4022, 0.6417, Ink::Fg),
        letter!(GEO_R, 12.4578, 2.8192, 4.7148, 5.3372, 0.7454, Ink::Fg),
        letter!(GEO_OPTICAL_O, 18.8886, 2.6804, 5.1218, 5.1669, 0.7454, Ink::Fg),
        letter!(GEO_S, 26.0963, 2.6363, 4.1359, 5.1413, 0.7532, Ink::Fg),
        letter!(GEO_H, 32.4617, 2.443, 4.1749, 5.3506, 0.718, Ink::Fg),
        letter!(GEO_I, 39.3377, 2.5452, 0.5405, 5.3684, 0.6523, Ink::Fg),
    ] },
];

pub const CHIP1: &[Prim] = &[
    Prim::Path { x: 0.0, y: 0.0, segs: &[
        Seg::Move(60.0, 247.0833),
        Seg::Line(61.25, 245.4167),
        Seg::Line(62.9167, 245.4167),
        Seg::Line(62.9167, 253.75),
        Seg::Line(61.6667, 253.75),
        Seg::Line(61.6667, 247.0833),
        Seg::Line(60.0, 247.9167),
        Seg::Line(60.0, 247.0833)
    ], close: true, fill: Some(CODE), stroke: None, width: 1.0 },
];

pub const CHIP2: &[Prim] = &[
    Prim::Path { x: 0.0, y: 0.0, segs: &[
        Seg::Move(1542.5833, 247.5),
        Seg::Line(1542.5833, 246.0417),
        Seg::Quad { cx: 1542.5833, cy: 244.5833, x: 1544.1667, y: 244.5833 },
        Seg::Line(1546.25, 244.5833),
        Seg::Quad { cx: 1548.0, cy: 244.5833, x: 1548.0, y: 246.0417 },
        Seg::Line(1548.0, 247.25),
        Seg::Quad { cx: 1548.0, cy: 247.9583, x: 1546.75, y: 249.0833 },
        Seg::Line(1544.1667, 250.9583),
        Seg::Line(1544.1667, 251.6667),
        Seg::Line(1548.0, 251.6667),
        Seg::Line(1548.0, 252.9167),
        Seg::Line(1542.5833, 252.9167),
        Seg::Line(1542.5833, 251.1667),
        Seg::Quad { cx: 1542.5833, cy: 250.2917, x: 1543.75, y: 249.4167 },
        Seg::Line(1546.0833, 247.5),
        Seg::Quad { cx: 1546.7083, cy: 247.0833, x: 1546.7083, y: 246.5 },
        Seg::Line(1546.7083, 246.0417),
        Seg::Quad { cx: 1546.7083, cy: 245.625, x: 1546.0417, y: 245.625 },
        Seg::Line(1544.375, 245.625),
        Seg::Quad { cx: 1543.8333, cy: 245.625, x: 1543.8333, y: 246.25 },
        Seg::Line(1543.8333, 247.5),
        Seg::Line(1542.5833, 247.5)
    ], close: true, fill: Some(CODE), stroke: None, width: 1.0 },
];
