//! NK-03: native-source T2 badge geometry (#69 hub, #71 mailbox).
//! A shallow shoulder rises 8.4px. Echoes step down/right, then their
//! curved legs converge into the unchanged bottom corners. Photographic
//! residue is not encoded; the last low-opacity strands are visible on
//! the source shoulder and outer fan, not an enlarged readability fan.
use super::{fill_path, rgb, shut_path, Ink, Prim, Seg};

const fn steps(index: usize, mail: bool) -> [Seg; 12] {
    let i = index as f32;
    let dx = if mail { -0.8 } else { 0.0 };
    let dy = if mail { 1.25 } else { 0.0 };
    let left = 1281.9 + dx;
    let right = 1337.3 + dx;
    let bottom = if mail { 106.7 } else { 105.1 };
    let sx = 1.18 * i;
    let y = 41.7 + dy + 1.23 * i;
    let top = 33.3 + dy + 1.23 * i;
    [
        Seg::Line(1298.8 + dx + sx, y),
        Seg::Quad { cx: 1301.6 + dx + sx, cy: y, x: 1304.0 + dx + sx, y: y - 1.6 },
        Seg::Line(1311.7 + dx + sx, top + 1.4),
        Seg::Quad { cx: 1313.7 + dx + sx, cy: top, x: 1316.3 + dx + sx, y: top },
        Seg::Line(right - 2.8 + sx, top),
        Seg::Quad { cx: right + sx, cy: top, x: right + sx, y: top + 2.8 },
        Seg::Cubic { c1x: right + sx, c1y: 65.0 + dy + 0.3 * i, c2x: right, c2y: 96.0 + dy, x: right, y: bottom - 2.8 },
        Seg::Quad { cx: right, cy: bottom, x: right - 2.8, y: bottom },
        Seg::Line(left + 2.8, bottom),
        Seg::Quad { cx: left, cy: bottom, x: left, y: bottom - 2.8 },
        Seg::Cubic { c1x: left, c1y: 93.0 + dy, c2x: left + sx, c2y: 72.0 + dy + 0.3 * i, x: left + sx, y: y + 3.0 },
        Seg::Quad { cx: left + sx, cy: y, x: left + 3.0 + sx, y },
    ]
}

macro_rules! ring {
    ($i:expr, $mail:expr, $alpha:expr) => {{
        const STEPS: [Seg; 12] = steps($i, $mail);
        shut_path(1284.9 + if $mail { -0.8 } else { 0.0 } + 1.18 * $i as f32,
            41.7 + if $mail { 1.25 } else { 0.0 } + 1.23 * $i as f32,
            &STEPS, Ink::Fixed(iced::Color { a: $alpha, ..rgb(0xe8c186) }),
            if $i == 0 { 1.1 } else { 0.7 })
    }};
}

pub(super) const DASHBOARD: &[Prim] = &[
    ring!(12, false, 0.06),
    ring!(11, false, 0.13),
    ring!(10, false, 0.21),
    ring!(9, false, 0.30),
    ring!(8, false, 0.40),
    ring!(7, false, 0.49),
    ring!(6, false, 0.55),
    ring!(5, false, 0.60),
    ring!(4, false, 0.65),
    ring!(3, false, 0.72),
    ring!(2, false, 0.78),
    ring!(1, false, 0.85),
    ring!(0, false, 1.00),
    fill_path(1293.0, 105.1, &[
        Seg::Quad { cx: 1294.2, cy: 105.1, x: 1294.6, y: 103.5 },
        Seg::Line(1295.1, 101.3),
        Seg::Quad { cx: 1295.2, cy: 100.6, x: 1296.0, y: 100.6 },
        Seg::Line(1324.7, 100.6),
        Seg::Quad { cx: 1325.5, cy: 100.6, x: 1325.7, y: 101.4 },
        Seg::Line(1326.2, 103.5),
        Seg::Quad { cx: 1326.7, cy: 105.1, x: 1327.8, y: 105.1 },
    ], Ink::Fixed(rgb(0xf2b463))),
];

pub(super) const MAILBOX: &[Prim] = &[
    ring!(14, true, 0.08),
    ring!(13, true, 0.15),
    ring!(12, true, 0.23),
    ring!(11, true, 0.30),
    ring!(10, true, 0.38),
    ring!(9, true, 0.46),
    ring!(8, true, 0.54),
    ring!(7, true, 0.62),
    ring!(6, true, 0.69),
    ring!(5, true, 0.74),
    ring!(4, true, 0.78),
    ring!(3, true, 0.82),
    ring!(2, true, 0.86),
    ring!(1, true, 0.90),
    ring!(0, true, 1.00),
    fill_path(1292.2, 106.7, &[
        Seg::Quad { cx: 1293.4, cy: 106.7, x: 1293.8, y: 105.1 },
        Seg::Line(1294.3, 102.9),
        Seg::Quad { cx: 1294.4, cy: 102.2, x: 1295.2, y: 102.2 },
        Seg::Line(1323.9, 102.2),
        Seg::Quad { cx: 1324.7, cy: 102.2, x: 1324.9, y: 103.0 },
        Seg::Line(1325.4, 105.1),
        Seg::Quad { cx: 1325.9, cy: 106.7, x: 1327.0, y: 106.7 },
    ], Ink::Fixed(rgb(0xf2b463))),
];
