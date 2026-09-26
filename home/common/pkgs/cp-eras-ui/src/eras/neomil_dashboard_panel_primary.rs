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
pub(super) const MICROTEXT: &[Prim] = &[
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
const HEADING_E: [Seg; 26] = [
    Seg::Move(8.5134563, 0.0000000),
    Seg::Line(1.7727778, 0.0000000),
    Seg::Quad { cx: 1.3398902, cy: 0.0000000, x: 1.3398902, y: -0.4382870 },
    Seg::Line(1.3398902, -12.9816445),
    Seg::Quad { cx: 1.3398902, cy: -13.4199316, x: 1.7727778, y: -13.4199316 },
    Seg::Line(8.5134563, -13.4199316),
    Seg::Quad { cx: 8.9257303, cy: -13.4199316, x: 8.9257303, y: -12.9816445 },
    Seg::Line(8.9257303, -11.5624294),
    Seg::Quad { cx: 8.9257303, cy: -11.1241423, x: 8.5134563, y: -11.1241423 },
    Seg::Line(4.2051939, -11.1241423),
    Seg::Quad { cx: 3.9578296, cy: -11.1241423, x: 3.9578296, y: -10.8945634 },
    Seg::Line(3.9578296, -8.1813580),
    Seg::Quad { cx: 3.9578296, cy: -7.9517790, x: 4.2051939, y: -7.9517790 },
    Seg::Line(7.7301359, -7.9517790),
    Seg::Quad { cx: 8.1630235, cy: -7.9517790, x: 8.1630235, y: -7.5134920 },
    Seg::Line(8.1630235, -6.0942769),
    Seg::Quad { cx: 8.1630235, cy: -5.6559898, x: 7.7301359, y: -5.6559898 },
    Seg::Line(4.2051939, -5.6559898),
    Seg::Quad { cx: 3.9578296, cy: -5.6559898, x: 3.9578296, y: -5.4264109 },
    Seg::Line(3.9578296, -2.5253681),
    Seg::Quad { cx: 3.9578296, cy: -2.2957892, x: 4.2051939, y: -2.2957892 },
    Seg::Line(8.5134563, -2.2957892),
    Seg::Quad { cx: 8.9257303, cy: -2.2957892, x: 8.9257303, y: -1.8575022 },
    Seg::Line(8.9257303, -0.4382870),
    Seg::Quad { cx: 8.9257303, cy: 0.0000000, x: 8.5134563, y: 0.0000000 },
    Seg::Line(8.5134563, 0.0000000),
];
const HEADING_G: [Seg; 38] = [
    Seg::Move(6.7200648, 0.0000000),
    Seg::Line(4.5350131, 0.0000000),
    Seg::Quad { cx: 2.9065311, cy: 0.0000000, x: 2.0201422, y: -0.8765741 },
    Seg::Quad { cx: 1.1337533, cy: -1.7531481, x: 1.1337533, y: -3.4019422 },
    Seg::Line(1.1337533, -10.0179893),
    Seg::Quad { cx: 1.1337533, cy: -11.6667834, x: 2.0201422, y: -12.5433575 },
    Seg::Quad { cx: 2.9065311, cy: -13.4199316, x: 4.5350131, y: -13.4199316 },
    Seg::Line(6.7200648, -13.4199316),
    Seg::Quad { cx: 8.3279331, cy: -13.4199316, x: 9.2143220, y: -12.5329221 },
    Seg::Quad { cx: 10.1007109, cy: -11.6459126, x: 10.1007109, y: -10.0179893 },
    Seg::Line(10.1007109, -9.3918650),
    Seg::Quad { cx: 10.1007109, cy: -8.9327072, x: 9.6678233, y: -8.9327072 },
    Seg::Line(7.9156592, -8.9327072),
    Seg::Quad { cx: 7.4827715, cy: -8.9327072, x: 7.4827715, y: -9.3918650 },
    Seg::Line(7.4827715, -9.8927645),
    Seg::Quad { cx: 7.4827715, cy: -10.5606304, x: 7.2147935, y: -10.8423864 },
    Seg::Quad { cx: 6.9468155, cy: -11.1241423, x: 6.2871772, y: -11.1241423 },
    Seg::Line(4.9472870, -11.1241423),
    Seg::Quad { cx: 4.3082624, cy: -11.1241423, x: 4.0299775, y: -10.8423864 },
    Seg::Quad { cx: 3.7516926, cy: -10.5606304, x: 3.7516926, y: -9.8927645 },
    Seg::Line(3.7516926, -3.5271671),
    Seg::Quad { cx: 3.7516926, cy: -2.8593011, x: 4.0299775, y: -2.5775452 },
    Seg::Quad { cx: 4.3082624, cy: -2.2957892, x: 4.9472870, y: -2.2957892 },
    Seg::Line(6.2871772, -2.2957892),
    Seg::Quad { cx: 6.9468155, cy: -2.2957892, x: 7.2147935, y: -2.5775452 },
    Seg::Quad { cx: 7.4827715, cy: -2.8593011, x: 7.4827715, y: -3.5271671 },
    Seg::Line(7.4827715, -4.8420282),
    Seg::Quad { cx: 7.4827715, cy: -5.0716071, x: 7.2560209, y: -5.0716071 },
    Seg::Line(6.0191992, -5.0716071),
    Seg::Quad { cx: 5.5863115, cy: -5.0716071, x: 5.5863115, y: -5.5098941 },
    Seg::Line(5.5863115, -6.9291093),
    Seg::Quad { cx: 5.5863115, cy: -7.3673963, x: 6.0191992, y: -7.3673963 },
    Seg::Line(9.6265959, -7.3673963),
    Seg::Quad { cx: 10.1007109, cy: -7.3673963, x: 10.1007109, y: -6.8873677 },
    Seg::Line(10.1007109, -3.4019422),
    Seg::Quad { cx: 10.1007109, cy: -1.7740189, x: 9.2143220, y: -0.8870095 },
    Seg::Quad { cx: 8.3279331, cy: 0.0000000, x: 6.7200648, y: 0.0000000 },
    Seg::Line(6.7200648, 0.0000000),
];
const HEADING_H: [Seg; 26] = [
    Seg::Move(3.5249420, 0.0000000),
    Seg::Line(1.7727778, 0.0000000),
    Seg::Quad { cx: 1.3398902, cy: 0.0000000, x: 1.3398902, y: -0.4382870 },
    Seg::Line(1.3398902, -12.9816445),
    Seg::Quad { cx: 1.3398902, cy: -13.4199316, x: 1.7727778, y: -13.4199316 },
    Seg::Line(3.5249420, -13.4199316),
    Seg::Quad { cx: 3.9578296, cy: -13.4199316, x: 3.9578296, y: -12.9816445 },
    Seg::Line(3.9578296, -8.0770039),
    Seg::Quad { cx: 3.9578296, cy: -7.8474250, x: 4.2051939, y: -7.8474250 },
    Seg::Line(7.6889085, -7.8474250),
    Seg::Quad { cx: 7.9156592, cy: -7.8474250, x: 7.9156592, y: -8.0770039 },
    Seg::Line(7.9156592, -12.9816445),
    Seg::Quad { cx: 7.9156592, cy: -13.4199316, x: 8.3485468, y: -13.4199316 },
    Seg::Line(10.1007109, -13.4199316),
    Seg::Quad { cx: 10.5335985, cy: -13.4199316, x: 10.5335985, y: -12.9816445 },
    Seg::Line(10.5335985, -0.4382870),
    Seg::Quad { cx: 10.5335985, cy: 0.0000000, x: 10.1007109, y: 0.0000000 },
    Seg::Line(8.3485468, 0.0000000),
    Seg::Quad { cx: 7.9156592, cy: 0.0000000, x: 7.9156592, y: -0.4382870 },
    Seg::Line(7.9156592, -5.3220568),
    Seg::Quad { cx: 7.9156592, cy: -5.5516358, x: 7.6889085, y: -5.5516358 },
    Seg::Line(4.2051939, -5.5516358),
    Seg::Quad { cx: 3.9578296, cy: -5.5516358, x: 3.9578296, y: -5.3220568 },
    Seg::Line(3.9578296, -0.4382870),
    Seg::Quad { cx: 3.9578296, cy: 0.0000000, x: 3.5249420, y: 0.0000000 },
    Seg::Line(3.5249420, 0.0000000),
];
const HEADING_M: [Seg; 29] = [
    Seg::Move(10.1419383, -13.4199316),
    Seg::Line(13.1927653, -13.4199316),
    Seg::Quad { cx: 13.6256529, cy: -13.4199316, x: 13.6256529, y: -12.9816445 },
    Seg::Line(13.6256529, -0.4382870),
    Seg::Quad { cx: 13.6256529, cy: 0.0000000, x: 13.1927653, y: 0.0000000 },
    Seg::Line(11.5024422, 0.0000000),
    Seg::Quad { cx: 11.0695546, cy: 0.0000000, x: 11.0695546, y: -0.4382870 },
    Seg::Line(11.0695546, -10.3310515),
    Seg::Line(10.9870998, -10.3310515),
    Seg::Line(8.8845029, -3.3393298),
    Seg::Quad { cx: 8.7608207, cy: -2.9219136, x: 8.3691605, y: -2.9219136 },
    Seg::Line(6.6376100, -2.9219136),
    Seg::Quad { cx: 6.2459498, cy: -2.9219136, x: 6.1222676, y: -3.3393298 },
    Seg::Line(3.9784433, -10.3519223),
    Seg::Line(3.8959885, -10.3519223),
    Seg::Line(3.8959885, -0.4382870),
    Seg::Quad { cx: 3.8959885, cy: -0.1878373, x: 3.8135337, y: -0.0939187 },
    Seg::Quad { cx: 3.7310789, cy: 0.0000000, x: 3.4631009, y: 0.0000000 },
    Seg::Line(1.7727778, 0.0000000),
    Seg::Quad { cx: 1.3398902, cy: 0.0000000, x: 1.3398902, y: -0.4382870 },
    Seg::Line(1.3398902, -12.9816445),
    Seg::Quad { cx: 1.3398902, cy: -13.4199316, x: 1.7727778, y: -13.4199316 },
    Seg::Line(4.8442185, -13.4199316),
    Seg::Quad { cx: 5.1328102, cy: -13.4199316, x: 5.2358787, y: -13.1277402 },
    Seg::Line(7.4209305, -5.7603439),
    Seg::Line(7.5446126, -5.7603439),
    Seg::Line(9.7502781, -13.1277402),
    Seg::Quad { cx: 9.8121192, cy: -13.4199316, x: 10.1419383, y: -13.4199316 },
    Seg::Line(10.1419383, -13.4199316),
];
const HEADING_O: [Seg; 28] = [
    Seg::Move(4.9472870, -2.2957892),
    Seg::Line(6.4108594, -2.2957892),
    Seg::Quad { cx: 7.0704976, cy: -2.2957892, x: 7.3487825, y: -2.5775452 },
    Seg::Quad { cx: 7.6270674, cy: -2.8593011, x: 7.6270674, y: -3.5271671 },
    Seg::Line(7.6270674, -9.8927645),
    Seg::Quad { cx: 7.6270674, cy: -10.5606304, x: 7.3487825, y: -10.8423864 },
    Seg::Quad { cx: 7.0704976, cy: -11.1241423, x: 6.4108594, y: -11.1241423 },
    Seg::Line(4.9472870, -11.1241423),
    Seg::Quad { cx: 4.2876487, cy: -11.1241423, x: 4.0196707, y: -10.8423864 },
    Seg::Quad { cx: 3.7516926, cy: -10.5606304, x: 3.7516926, y: -9.8927645 },
    Seg::Line(3.7516926, -3.5271671),
    Seg::Quad { cx: 3.7516926, cy: -2.8593011, x: 4.0196707, y: -2.5775452 },
    Seg::Quad { cx: 4.2876487, cy: -2.2957892, x: 4.9472870, y: -2.2957892 },
    Seg::Line(4.9472870, -2.2957892),
    Seg::Move(6.8437470, 0.0000000),
    Seg::Line(4.5350131, 0.0000000),
    Seg::Quad { cx: 2.9065311, cy: 0.0000000, x: 2.0201422, y: -0.8765741 },
    Seg::Quad { cx: 1.1337533, cy: -1.7531481, x: 1.1337533, y: -3.4019422 },
    Seg::Line(1.1337533, -10.0179893),
    Seg::Quad { cx: 1.1337533, cy: -11.6667834, x: 2.0201422, y: -12.5433575 },
    Seg::Quad { cx: 2.9065311, cy: -13.4199316, x: 4.5350131, y: -13.4199316 },
    Seg::Line(6.8437470, -13.4199316),
    Seg::Quad { cx: 8.4722289, cy: -13.4199316, x: 9.3586179, y: -12.5433575 },
    Seg::Quad { cx: 10.2450068, cy: -11.6667834, x: 10.2450068, y: -10.0179893 },
    Seg::Line(10.2450068, -3.4019422),
    Seg::Quad { cx: 10.2450068, cy: -1.7531481, x: 9.3586179, y: -0.8765741 },
    Seg::Quad { cx: 8.4722289, cy: 0.0000000, x: 6.8437470, y: 0.0000000 },
    Seg::Line(6.8437470, 0.0000000),
];
const HEADING_INK_1: Ink = Ink::Fixed(iced::Color { a: 0.3244518, ..rgb(0xf83333) });
const HEADING_COPY_1: &[Prim] = &[
    Prim::At { x: 0.0000000, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_G, HEADING_INK_1)] },
    Prim::At { x: 11.2344642, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_O, HEADING_INK_1)] },
    Prim::At { x: 27.1482373, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_H, HEADING_INK_1)] },
    Prim::At { x: 39.0217260, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_O, HEADING_INK_1)] },
    Prim::At { x: 50.4004861, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_M, HEADING_INK_1)] },
    Prim::At { x: 65.3660292, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_E, HEADING_INK_1)] },
];
const HEADING_INK_2: Ink = Ink::Fixed(iced::Color { a: 0.2368014, ..rgb(0xf83333) });
const HEADING_COPY_2: &[Prim] = &[
    Prim::At { x: 0.0000000, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_G, HEADING_INK_2)] },
    Prim::At { x: 11.2344642, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_O, HEADING_INK_2)] },
    Prim::At { x: 27.1482373, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_H, HEADING_INK_2)] },
    Prim::At { x: 39.0217260, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_O, HEADING_INK_2)] },
    Prim::At { x: 50.4004861, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_M, HEADING_INK_2)] },
    Prim::At { x: 65.3660292, y: 0.0, prims: &[fill_path(0.0, 0.0, &HEADING_E, HEADING_INK_2)] },
];
const HEADING_COPIES: &[Prim] = &[
    Prim::At { x: 1139.8028302, y: 332.0057576, prims: HEADING_COPY_2 },
    Prim::At { x: 1137.7924133, y: 332.6979361, prims: HEADING_COPY_1 },
];
const ECHO_CYCLE: &[(f32, iced::Color)] = &[
    (0.0000000, iced::Color { a: 0.0000000, ..rgb(0xffffff) }),
    (0.0625000, iced::Color { a: 0.0380602, ..rgb(0xffffff) }),
    (0.1250000, iced::Color { a: 0.1464466, ..rgb(0xffffff) }),
    (0.1875000, iced::Color { a: 0.3086583, ..rgb(0xffffff) }),
    (0.2500000, iced::Color { a: 0.5000000, ..rgb(0xffffff) }),
    (0.3125000, iced::Color { a: 0.6913417, ..rgb(0xffffff) }),
    (0.3750000, iced::Color { a: 0.8535534, ..rgb(0xffffff) }),
    (0.4375000, iced::Color { a: 0.9619398, ..rgb(0xffffff) }),
    (0.5000000, iced::Color { a: 1.0000000, ..rgb(0xffffff) }),
    (0.5625000, iced::Color { a: 0.9619398, ..rgb(0xffffff) }),
    (0.6250000, iced::Color { a: 0.8535534, ..rgb(0xffffff) }),
    (0.6875000, iced::Color { a: 0.6913417, ..rgb(0xffffff) }),
    (0.7500000, iced::Color { a: 0.5000000, ..rgb(0xffffff) }),
    (0.8125000, iced::Color { a: 0.3086583, ..rgb(0xffffff) }),
    (0.8750000, iced::Color { a: 0.1464466, ..rgb(0xffffff) }),
    (0.9375000, iced::Color { a: 0.0380602, ..rgb(0xffffff) }),
    (1.0000000, iced::Color { a: 0.0000000, ..rgb(0xffffff) }),
];
const fn heading_scan() -> [Prim; 15] {
    let mut rows = [fill_rect(1131.0, 314.0, 94.0, 26.0, Ink::Fixed(iced::Color { a: 0.4534689, ..rgb(0xffffff) })); 15];
    let mut i = 0;
    while i < 14 {
        rows[i + 1] = Prim::Ramp { x: 1131.0, y: 312.9078698 + i as f32 * 1.984934, w: 94.0, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops: ECHO_CYCLE };
        i += 1;
    }
    rows
}
pub(super) const HEADING_ECHOES: &[Prim] = &[Prim::Masked { prims: HEADING_COPIES, mask: &heading_scan() }];
