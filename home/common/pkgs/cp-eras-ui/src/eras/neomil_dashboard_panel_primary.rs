//! Source-fitted GO HOME heading, maker material and geometric microtext.
//! The heading uses bundled Rajdhani Bold; microtext reconstructs the visible
//! monospace centre lines without claiming an unavailable original font.
use crate::palette::rgb;
use crate::style::{fill_path, fill_rect, Anchor, Face, Ink, Prim, Seg};

pub(super) const HEADING: Prim = Prim::Wide { x: 1135.7819965, y: 333.3901147, size: 20.8708111, stretch: 0.9876806, ink: Ink::Fixed(rgb(0xf83333)), face: Face::Bold, anchor: Anchor::Start, content: "GO HOME" };
const MAKER_PATH: [Seg; 33] = [
    Seg::Move(1239.2000000, 682.0833000),
    Seg::Line(1255.1261657, 682.0833000),
    Seg::Quad { cx: 1255.7261657, cy: 682.0833000, x: 1255.7261657, y: 682.6833000 },
    Seg::Line(1255.7261657, 698.6000000),
    Seg::Quad { cx: 1255.7261657, cy: 699.0000000, x: 1256.0761657, y: 698.5000000 },
    Seg::Line(1266.6438931, 682.7500000),
    Seg::Quad { cx: 1267.0938931, cy: 682.0833000, x: 1267.8938931, y: 682.0833000 },
    Seg::Line(1282.9000000, 682.0833000),
    Seg::Quad { cx: 1283.4500000, cy: 682.0833000, x: 1283.4500000, y: 682.6333000 },
    Seg::Line(1283.4500000, 707.1862860),
    Seg::Quad { cx: 1283.4500000, cy: 708.4410112, x: 1281.1493037, y: 708.4410112 },
    Seg::Line(1280.3269195, 708.4410112),
    Seg::Quad { cx: 1279.4269195, cy: 708.4410112, x: 1278.9269195, y: 709.1910112 },
    Seg::Line(1276.3522211, 713.2500000),
    Seg::Quad { cx: 1275.6522211, cy: 714.6840949, x: 1273.2722413, y: 714.8840949 },
    Seg::Line(1273.2722413, 717.5000000),
    Seg::Line(1271.6482693, 720.6000000),
    Seg::Quad { cx: 1268.7464494, cy: 725.8204942, x: 1263.3164056, y: 725.8204942 },
    Seg::Line(1249.0000000, 725.8204942),
    Seg::Quad { cx: 1248.2000000, cy: 725.8204942, x: 1248.2000000, y: 725.0204942 },
    Seg::Line(1248.2000000, 714.1000000),
    Seg::Quad { cx: 1248.2000000, cy: 713.4000000, x: 1247.7000000, y: 714.2000000 },
    Seg::Line(1243.2620243, 721.4000000),
    Seg::Quad { cx: 1240.6613467, cy: 725.8204942, x: 1234.6368704, y: 725.8204942 },
    Seg::Line(1221.3647604, 725.8204942),
    Seg::Quad { cx: 1220.4647604, cy: 725.8204942, x: 1220.4647604, y: 724.9204942 },
    Seg::Line(1220.4647604, 700.1792751),
    Seg::Quad { cx: 1220.4647604, cy: 699.0792751, x: 1221.5647604, y: 699.0792751 },
    Seg::Line(1224.6000000, 699.0792751),
    Seg::Quad { cx: 1227.8000000, cy: 699.0792751, x: 1229.1000000, y: 697.1000000 },
    Seg::Line(1238.2000000, 683.0000000),
    Seg::Quad { cx: 1238.8000000, cy: 682.0833000, x: 1239.2000000, y: 682.0833000 },
    Seg::Line(1239.2000000, 682.0833000),
];
const DOT_PATH: [Seg; 10] = [
    Seg::Move(1273.4922287, 715.3527256),
    Seg::Line(1283.8122287, 715.3527256),
    Seg::Quad { cx: 1284.1122287, cy: 715.3527256, x: 1284.1122287, y: 715.6527256 },
    Seg::Line(1284.1122287, 726.3582908),
    Seg::Quad { cx: 1284.1122287, cy: 726.6582908, x: 1283.8122287, y: 726.6582908 },
    Seg::Line(1273.4922287, 726.6582908),
    Seg::Quad { cx: 1273.1922287, cy: 726.6582908, x: 1273.1922287, y: 726.3582908 },
    Seg::Line(1273.1922287, 715.6527256),
    Seg::Quad { cx: 1273.1922287, cy: 715.3527256, x: 1273.4922287, y: 715.3527256 },
    Seg::Line(1273.4922287, 715.3527256),
];
const MAIN_CYCLE: &[(f32, iced::Color)] = &[
    (0.0000000, rgb(0xf03030)),
    (0.0312500, rgb(0xf03030)),
    (0.0625000, rgb(0xf13030)),
    (0.0937500, rgb(0xf13131)),
    (0.1250000, rgb(0xf13131)),
    (0.1562500, rgb(0xf13131)),
    (0.1875000, rgb(0xf13131)),
    (0.2187500, rgb(0xf13030)),
    (0.2500000, rgb(0xf03030)),
    (0.2812500, rgb(0xf03030)),
    (0.3125000, rgb(0xf03030)),
    (0.3437500, rgb(0xef3030)),
    (0.3750000, rgb(0xef3030)),
    (0.4062500, rgb(0xee3030)),
    (0.4375000, rgb(0xee3030)),
    (0.4687500, rgb(0xed3131)),
    (0.5000000, rgb(0xed3131)),
    (0.5312500, rgb(0xed3131)),
    (0.5625000, rgb(0xec3232)),
    (0.5937500, rgb(0xec3232)),
    (0.6250000, rgb(0xec3232)),
    (0.6562500, rgb(0xec3232)),
    (0.6875000, rgb(0xec3232)),
    (0.7187500, rgb(0xec3232)),
    (0.7500000, rgb(0xed3131)),
    (0.7812500, rgb(0xed3131)),
    (0.8125000, rgb(0xed3030)),
    (0.8437500, rgb(0xee3030)),
    (0.8750000, rgb(0xee3030)),
    (0.9062500, rgb(0xef3030)),
    (0.9375000, rgb(0xef3030)),
    (0.9687500, rgb(0xf03030)),
    (1.0000000, rgb(0xf03030)),
];
const fn main_field() -> [Prim; 27] {
    let mut rows = [fill_rect(1218.0, 680.0, 69.0, 50.0, Ink::Fixed(rgb(0xf03030))); 27];
    let mut i = 0;
    while i < 26 {
        rows[i + 1] = Prim::Ramp { x: 1218.0, y: 678.8474280 + i as f32 * 1.984934, w: 69.0, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops: MAIN_CYCLE };
        i += 1;
    }
    rows
}
const DOT_CYCLE: &[(f32, iced::Color)] = &[
    (0.0000000, rgb(0xbe2223)),
    (0.0312500, rgb(0xc02324)),
    (0.0625000, rgb(0xc12425)),
    (0.0937500, rgb(0xc12526)),
    (0.1250000, rgb(0xc22526)),
    (0.1562500, rgb(0xc22526)),
    (0.1875000, rgb(0xc22526)),
    (0.2187500, rgb(0xc22425)),
    (0.2500000, rgb(0xc12424)),
    (0.2812500, rgb(0xbf2323)),
    (0.3125000, rgb(0xbe2222)),
    (0.3437500, rgb(0xbc2222)),
    (0.3750000, rgb(0xba2222)),
    (0.4062500, rgb(0xb82223)),
    (0.4375000, rgb(0xb62324)),
    (0.4687500, rgb(0xb52525)),
    (0.5000000, rgb(0xb32727)),
    (0.5312500, rgb(0xb22829)),
    (0.5625000, rgb(0xb12a2a)),
    (0.5937500, rgb(0xb12b2b)),
    (0.6250000, rgb(0xb12c2c)),
    (0.6562500, rgb(0xb12c2c)),
    (0.6875000, rgb(0xb22b2b)),
    (0.7187500, rgb(0xb32a2a)),
    (0.7500000, rgb(0xb42828)),
    (0.7812500, rgb(0xb52627)),
    (0.8125000, rgb(0xb72425)),
    (0.8437500, rgb(0xb82323)),
    (0.8750000, rgb(0xb92222)),
    (0.9062500, rgb(0xbb2122)),
    (0.9375000, rgb(0xbc2122)),
    (0.9687500, rgb(0xbd2222)),
    (1.0000000, rgb(0xbe2223)),
];
const fn dot_field() -> [Prim; 10] {
    let mut rows = [fill_rect(1271.0, 713.0, 17.0, 17.0, Ink::Fixed(rgb(0xbe2223))); 10];
    let mut i = 0;
    while i < 9 {
        rows[i + 1] = Prim::Ramp { x: 1271.0, y: 712.5913060 + i as f32 * 1.984934, w: 17.0, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops: DOT_CYCLE };
        i += 1;
    }
    rows
}
/// Append in the leading panel Soft, after maker printing echoes, under GO_HOME_OPEN.
pub(super) const PRIMARY_MATERIAL: &[Prim] = &[
    Prim::Masked { prims: &main_field(), mask: &[fill_path(0.0, 0.0, &MAKER_PATH, Ink::Fixed(rgb(0xffffff)))] },
    Prim::Masked { prims: &dot_field(), mask: &[fill_path(0.0, 0.0, &DOT_PATH, Ink::Fixed(rgb(0xffffff)))] },
    Prim::Path { x: 0.0, y: 0.0, segs: &DOT_PATH, close: true, fill: None, stroke: Some(Ink::Fixed(rgb(0xef3333))), width: 0.2500000 },
];
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
const MICRO_P: [Seg; 7] = [
    Seg::Move(0.0000000, 1.0000000),
    Seg::Line(0.0000000, 0.0000000),
    Seg::Line(0.7500000, 0.0000000),
    Seg::Line(1.0000000, 0.1600000),
    Seg::Line(1.0000000, 0.3500000),
    Seg::Line(0.7600000, 0.5000000),
    Seg::Line(0.0000000, 0.5000000),
];
const MICRO_R: [Seg; 9] = [
    Seg::Move(0.0000000, 1.0000000),
    Seg::Line(0.0000000, 0.0000000),
    Seg::Line(0.7500000, 0.0000000),
    Seg::Line(1.0000000, 0.1600000),
    Seg::Line(1.0000000, 0.3500000),
    Seg::Line(0.7600000, 0.5000000),
    Seg::Line(0.0000000, 0.5000000),
    Seg::Move(0.5000000, 0.5000000),
    Seg::Line(1.0000000, 1.0000000),
];
const MICRO_E: [Seg; 6] = [
    Seg::Move(1.0000000, 0.0000000),
    Seg::Line(0.0000000, 0.0000000),
    Seg::Line(0.0000000, 1.0000000),
    Seg::Line(1.0000000, 1.0000000),
    Seg::Move(0.0000000, 0.5000000),
    Seg::Line(0.9000000, 0.5000000),
];
const MICRO_C: [Seg; 8] = [
    Seg::Move(1.0000000, 0.1600000),
    Seg::Line(0.7600000, 0.0000000),
    Seg::Line(0.2400000, 0.0000000),
    Seg::Line(0.0000000, 0.1600000),
    Seg::Line(0.0000000, 0.8400000),
    Seg::Line(0.2400000, 1.0000000),
    Seg::Line(0.7600000, 1.0000000),
    Seg::Line(1.0000000, 0.8400000),
];
const MICRO_I: [Seg; 6] = [
    Seg::Move(0.1000000, 0.0000000),
    Seg::Line(0.9000000, 0.0000000),
    Seg::Move(0.5000000, 0.0000000),
    Seg::Line(0.5000000, 1.0000000),
    Seg::Move(0.1000000, 1.0000000),
    Seg::Line(0.9000000, 1.0000000),
];
const MICRO_S: [Seg; 12] = [
    Seg::Move(1.0000000, 0.1600000),
    Seg::Line(0.7600000, 0.0000000),
    Seg::Line(0.2400000, 0.0000000),
    Seg::Line(0.0000000, 0.1600000),
    Seg::Line(0.0000000, 0.3400000),
    Seg::Line(0.2400000, 0.5000000),
    Seg::Line(0.7600000, 0.5000000),
    Seg::Line(1.0000000, 0.6600000),
    Seg::Line(1.0000000, 0.8400000),
    Seg::Line(0.7600000, 1.0000000),
    Seg::Line(0.2400000, 1.0000000),
    Seg::Line(0.0000000, 0.8400000),
];
const MICRO_O: [Seg; 9] = [
    Seg::Move(0.2400000, 0.0000000),
    Seg::Line(0.7600000, 0.0000000),
    Seg::Line(1.0000000, 0.1600000),
    Seg::Line(1.0000000, 0.8400000),
    Seg::Line(0.7600000, 1.0000000),
    Seg::Line(0.2400000, 1.0000000),
    Seg::Line(0.0000000, 0.8400000),
    Seg::Line(0.0000000, 0.1600000),
    Seg::Line(0.2400000, 0.0000000),
];
const MICRO_N: [Seg; 6] = [
    Seg::Move(0.0000000, 1.0000000),
    Seg::Line(0.0000000, 0.0000000),
    Seg::Move(0.0000000, 0.0000000),
    Seg::Line(1.0000000, 1.0000000),
    Seg::Move(1.0000000, 1.0000000),
    Seg::Line(1.0000000, 0.0000000),
];
const MICRO_L: [Seg; 3] = [
    Seg::Move(0.0000000, 0.0000000),
    Seg::Line(0.0000000, 1.0000000),
    Seg::Line(1.0000000, 1.0000000),
];
const MICRO_Q: [Seg; 11] = [
    Seg::Move(0.2400000, 0.0000000),
    Seg::Line(0.7600000, 0.0000000),
    Seg::Line(1.0000000, 0.1600000),
    Seg::Line(1.0000000, 0.8400000),
    Seg::Line(0.7600000, 1.0000000),
    Seg::Line(0.2400000, 1.0000000),
    Seg::Line(0.0000000, 0.8400000),
    Seg::Line(0.0000000, 0.1600000),
    Seg::Line(0.2400000, 0.0000000),
    Seg::Move(0.5500000, 0.7200000),
    Seg::Line(1.1300000, 1.1400000),
];
const MICRO_U: [Seg; 6] = [
    Seg::Move(0.0000000, 0.0000000),
    Seg::Line(0.0000000, 0.8400000),
    Seg::Line(0.2400000, 1.0000000),
    Seg::Line(0.7600000, 1.0000000),
    Seg::Line(1.0000000, 0.8400000),
    Seg::Line(1.0000000, 0.0000000),
];
const MICRO_D: [Seg; 7] = [
    Seg::Move(0.0000000, 1.0000000),
    Seg::Line(0.0000000, 0.0000000),
    Seg::Line(0.7300000, 0.0000000),
    Seg::Line(1.0000000, 0.1700000),
    Seg::Line(1.0000000, 0.8300000),
    Seg::Line(0.7300000, 1.0000000),
    Seg::Line(0.0000000, 1.0000000),
];
const MICRO_Y: [Seg; 5] = [
    Seg::Move(0.0000000, 0.0000000),
    Seg::Line(0.5000000, 0.5000000),
    Seg::Line(1.0000000, 0.0000000),
    Seg::Move(0.5000000, 0.5000000),
    Seg::Line(0.5000000, 1.0000000),
];
const MICRO_M: [Seg; 5] = [
    Seg::Move(0.0000000, 1.0000000),
    Seg::Line(0.0000000, 0.0000000),
    Seg::Line(0.5000000, 0.5200000),
    Seg::Line(1.0000000, 0.0000000),
    Seg::Line(1.0000000, 1.0000000),
];
const MICROTEXT_PRIMARY: [Prim; 28] = [
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_P, 1220.2237099, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_R, 1224.3099598, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_E, 1228.3962097, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_C, 1232.4824596, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_I, 1236.5687095, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_S, 1240.6549594, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_I, 1244.7412094, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_O, 1248.8274593, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_N, 1252.9137092, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_L, 1261.0862090, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_I, 1265.1724589, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_Q, 1269.2587088, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_U, 1273.3449587, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_I, 1277.4312086, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_D, 1281.5174585, 732.6243160, 2.9454497, 4.3127485), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4167000 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_P, 1224.2702438, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_O, 1228.3679965, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_L, 1232.4657492, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_Y, 1236.5635019, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_M, 1240.6612545, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_E, 1244.7590072, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_R, 1248.8567599, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_M, 1257.0522652, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_U, 1261.1500179, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_S, 1265.2477706, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_C, 1269.3455233, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_L, 1273.4432759, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
    Prim::Path { x: 0.0, y: 0.0, segs: &placed(MICRO_E, 1277.5410286, 741.1262866, 2.8555179, 4.1797937), close: false, fill: None, stroke: Some(Ink::Fixed(rgb(0xf93333))), width: 0.4175971 },
];
// The source's faint copies are fitted independently on each text line.
// Group masks apply opacity once at letter junctions; all copies stay in
// the existing bounded software layer and GO HOME opening clip.
const fn micro_line<const N: usize>(start: usize) -> [Prim; N] {
    let mut out = [MICROTEXT_PRIMARY[start]; N];
    let mut i = 0;
    while i < N { out[i] = MICROTEXT_PRIMARY[start + i]; i += 1; }
    out
}
const MICRO_PRECISION: &[Prim] = &micro_line::<15>(0);
const MICRO_POLYMER: &[Prim] = &micro_line::<13>(15);
macro_rules! micro_copy {
    ($prims:expr, $x:expr, $y:expr, $alpha:expr) => {
        Prim::At { x: $x, y: $y, prims: &[Prim::Masked {
            prims: $prims,
            mask: &[fill_rect(1217.0, 730.0, 72.0, 21.0,
                Ink::Fixed(iced::Color { a: $alpha, ..rgb(0xffffff) }))],
        }] }
    };
}
pub(super) const MICROTEXT: &[Prim] = &[
    micro_copy!(MICRO_PRECISION, 1.444407, 0.439918, 0.002273),
    micro_copy!(MICRO_PRECISION, 1.937394, 0.439918, 0.009090),
    micro_copy!(MICRO_PRECISION, 2.430380, 0.439918, 0.013636),
    micro_copy!(MICRO_PRECISION, 2.923367, 0.439918, 0.009090),
    micro_copy!(MICRO_PRECISION, 3.416354, 0.439918, 0.002273),
    micro_copy!(MICRO_PRECISION, 1.444407, 0.946793, 0.009090),
    micro_copy!(MICRO_PRECISION, 1.937394, 0.946793, 0.036362),
    micro_copy!(MICRO_PRECISION, 2.430380, 0.946793, 0.054542),
    micro_copy!(MICRO_PRECISION, 2.923367, 0.946793, 0.036362),
    micro_copy!(MICRO_PRECISION, 3.416354, 0.946793, 0.009090),
    micro_copy!(MICRO_PRECISION, 1.444407, 1.453668, 0.013636),
    micro_copy!(MICRO_PRECISION, 1.937394, 1.453668, 0.054542),
    micro_copy!(MICRO_PRECISION, 2.430380, 1.453668, 0.081814),
    micro_copy!(MICRO_PRECISION, 2.923367, 1.453668, 0.054542),
    micro_copy!(MICRO_PRECISION, 3.416354, 1.453668, 0.013636),
    micro_copy!(MICRO_PRECISION, 1.444407, 1.960543, 0.009090),
    micro_copy!(MICRO_PRECISION, 1.937394, 1.960543, 0.036362),
    micro_copy!(MICRO_PRECISION, 2.430380, 1.960543, 0.054542),
    micro_copy!(MICRO_PRECISION, 2.923367, 1.960543, 0.036362),
    micro_copy!(MICRO_PRECISION, 3.416354, 1.960543, 0.009090),
    micro_copy!(MICRO_PRECISION, 1.444407, 2.467418, 0.002273),
    micro_copy!(MICRO_PRECISION, 1.937394, 2.467418, 0.009090),
    micro_copy!(MICRO_PRECISION, 2.430380, 2.467418, 0.013636),
    micro_copy!(MICRO_PRECISION, 2.923367, 2.467418, 0.009090),
    micro_copy!(MICRO_PRECISION, 3.416354, 2.467418, 0.002273),
    micro_copy!(MICRO_PRECISION, -0.416667, 0.000000, 0.181299),
    micro_copy!(MICRO_PRECISION, 0.416667, 0.000000, 0.181299),
    micro_copy!(MICRO_PRECISION, 0.000000, -0.416667, 0.296318),
    micro_copy!(MICRO_PRECISION, 0.000000, 0.416667, 0.296318),
    Prim::At { x: 0.0, y: 0.0, prims: MICRO_PRECISION },
    micro_copy!(MICRO_POLYMER, 1.695197, 0.494538, 0.002580),
    micro_copy!(MICRO_POLYMER, 2.361151, 0.494538, 0.010321),
    micro_copy!(MICRO_POLYMER, 3.027105, 0.494538, 0.015482),
    micro_copy!(MICRO_POLYMER, 3.693059, 0.494538, 0.010321),
    micro_copy!(MICRO_POLYMER, 4.359012, 0.494538, 0.002580),
    micro_copy!(MICRO_POLYMER, 1.695197, 0.965914, 0.010321),
    micro_copy!(MICRO_POLYMER, 2.361151, 0.965914, 0.041284),
    micro_copy!(MICRO_POLYMER, 3.027105, 0.965914, 0.061926),
    micro_copy!(MICRO_POLYMER, 3.693059, 0.965914, 0.041284),
    micro_copy!(MICRO_POLYMER, 4.359012, 0.965914, 0.010321),
    micro_copy!(MICRO_POLYMER, 1.695197, 1.437290, 0.015482),
    micro_copy!(MICRO_POLYMER, 2.361151, 1.437290, 0.061926),
    micro_copy!(MICRO_POLYMER, 3.027105, 1.437290, 0.092889),
    micro_copy!(MICRO_POLYMER, 3.693059, 1.437290, 0.061926),
    micro_copy!(MICRO_POLYMER, 4.359012, 1.437290, 0.015482),
    micro_copy!(MICRO_POLYMER, 1.695197, 1.908666, 0.010321),
    micro_copy!(MICRO_POLYMER, 2.361151, 1.908666, 0.041284),
    micro_copy!(MICRO_POLYMER, 3.027105, 1.908666, 0.061926),
    micro_copy!(MICRO_POLYMER, 3.693059, 1.908666, 0.041284),
    micro_copy!(MICRO_POLYMER, 4.359012, 1.908666, 0.010321),
    micro_copy!(MICRO_POLYMER, 1.695197, 2.380042, 0.002580),
    micro_copy!(MICRO_POLYMER, 2.361151, 2.380042, 0.010321),
    micro_copy!(MICRO_POLYMER, 3.027105, 2.380042, 0.015482),
    micro_copy!(MICRO_POLYMER, 3.693059, 2.380042, 0.010321),
    micro_copy!(MICRO_POLYMER, 4.359012, 2.380042, 0.002580),
    micro_copy!(MICRO_POLYMER, -0.416667, 0.000000, 0.184115),
    micro_copy!(MICRO_POLYMER, 0.416667, 0.000000, 0.184115),
    micro_copy!(MICRO_POLYMER, 0.000000, -0.416667, 0.324727),
    micro_copy!(MICRO_POLYMER, 0.000000, 0.416667, 0.324727),
    Prim::At { x: 0.0, y: 0.0, prims: MICRO_POLYMER },
];
