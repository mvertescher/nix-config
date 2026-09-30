//! Source-fitted directional echoes behind the store's 25 scatter-code cells.
//! Idle reference material is composited in the existing shelf Soft group;
//! feedback uses semantic inks after the opaque card coat and before Dots.
use super::*;

const fn points() -> [(f32, f32); 25] {
    let mut out = [(0.0, 0.0); 25];
    let mut n = 0;
    let mut row = 0;
    while row < QR.len() {
        let bytes = QR[row].as_bytes();
        let mut col = 0;
        while col < bytes.len() {
            if bytes[col] == b'#' {
                out[n] = (col as f32 * 3.6667, row as f32 * 3.6667);
                n += 1;
            }
            col += 1;
        }
        row += 1;
    }
    assert!(n == 25);
    out
}
const POINTS: [(f32, f32); 25] = points();
const fn cells(origin: (f32, f32), local: &'static [Prim]) -> [Prim; 25] {
    let mut out = [Prim::At { x: 0.0, y: 0.0, prims: local }; 25];
    let mut i = 0;
    while i < 25 {
        out[i] = Prim::At { x: origin.0 + POINTS[i].0, y: origin.1 + POINTS[i].1, prims: local };
        i += 1;
    }
    out
}

const RED: iced::Color = rgb(0xfb1818);
const fn tint(a: f32) -> iced::Color { iced::Color { a, ..RED } }
const fn white(a: f32) -> iced::Color { iced::Color { a, ..rgb(0xffffff) } }
// The far-to-near curve is unchanged from the source-fitted broad-tail
// scratch. The vertical mask is the card-1 band profile; card 4 moves it
// upward one native pixel, with no amplitude or shape retune.
const H: &[(f32, iced::Color)] = &[
    (0.0, tint(0.0)), (0.15,tint(0.0232)), (0.30,tint(0.0928)),
    (0.45,tint(0.1856)), (0.60,tint(0.29)), (0.77,tint(0.4408)), (1.0,tint(0.696)),
];
const V: &[(f32, iced::Color)] = &[
    (0.0,white(0.08)), (0.0667,white(0.10)), (0.1333,white(0.13)),
    (0.20,white(0.22)), (0.2667,white(0.55)), (0.3333,white(1.0)),
    (0.40,white(0.92)), (0.4667,white(0.66)), (0.5333,white(0.61)),
    (0.60,white(0.91)), (0.6667,white(1.0)), (0.7333,white(0.80)),
    (0.80,white(0.52)), (0.8667,white(0.30)), (0.9333,white(0.13)), (1.0,white(0.0)),
];
const LEFT_COLOR: &[Prim] = &[Prim::Ramp { x: -8.0, y: 0.0, w: 8.0, h: 6.0,
    from: (0.0,0.0), to: (1.0,0.0), stops: H }];
const RIGHT_COLOR: &[Prim] = &[Prim::Ramp { x: 3.0, y: 0.0, w: 8.0, h: 6.0,
    from: (1.0,0.0), to: (0.0,0.0), stops: H }];
const V_MASK: &[Prim] = &[Prim::Ramp { x: -8.0, y: 0.0, w: 19.0, h: 6.0,
    from: (0.0,0.0), to: (0.0,1.0), stops: V }];
const LEFT: &[Prim] = &[Prim::Masked { prims: LEFT_COLOR, mask: V_MASK }];
const RIGHT: &[Prim] = &[Prim::Masked { prims: RIGHT_COLOR, mask: V_MASK }];
const SEL: &[(f32, iced::Color)] = &[
    (0.0000000, tint(0.6038)),
    (0.0312500, tint(0.6038)),
    (0.0937500, tint(0.4362)),
    (0.1562500, tint(0.2516)),
    (0.2187500, tint(0.3015)),
    (0.2812500, tint(0.3984)),
    (0.3437500, tint(0.3143)),
    (0.4062500, tint(0.1763)),
    (0.4687500, tint(0.1851)),
    (0.5312500, tint(0.2589)),
    (0.5937500, tint(0.2237)),
    (0.6562500, tint(0.1264)),
    (0.7187500, tint(0.1143)),
    (0.7812500, tint(0.1657)),
    (0.8437500, tint(0.1308)),
    (0.9062500, tint(0.0398)),
    (0.9687500, tint(0.0024)),
    (1.0000000, tint(0.0000)),
];
const DOWN: &[Prim] = &[Prim::Ramp { x: 0.0, y: 3.0, w: 3.0, h: 8.0,
    from: (0.0,0.0), to: (0.0,1.0), stops: SEL }];

pub(super) const IDLE_LEFT1: &[Prim] = &cells((14.375, 520.5833), LEFT);
pub(super) const IDLE_RIGHT3: &[Prim] = &cells((12.4583, 520.5833), RIGHT);
pub(super) const IDLE_RIGHT4: &[Prim] = &cells((11.7917, 520.1666), RIGHT);
pub(super) const IDLE_DOWN: &[Prim] = &cells((12.4167, 711.2917), DOWN);

// A constant-opacity Motion is the existing scene primitive that fades
// Ink::Fg without fixing its RGB. Both alpha endpoints coincide and dur=0:
// no time-dependent painting or extra soft image is introduced.
const fn opacity(a: f32) -> Motion { Motion { id: "store-echo-ink-alpha",
    begin: 0, dur: 0, ease: Easing::Linear, change: Change::Opacity { alpha: (a,a) } } }
const XL_HOVER: &[Prim] = &[
    Prim::Motion { motion: opacity(0.0097), prims: &[fill_rect(-8.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.0406), prims: &[fill_rect(-7.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1005), prims: &[fill_rect(-6.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1779), prims: &[fill_rect(-5.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.2639), prims: &[fill_rect(-4.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.3676), prims: &[fill_rect(-3.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.4880), prims: &[fill_rect(-2.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.6267), prims: &[fill_rect(-1.0, 0.0, 1.0, 0.5, Ink::Fg)] },
];
const XR_HOVER: &[Prim] = &[
    Prim::Motion { motion: opacity(0.6267), prims: &[fill_rect(3.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.4880), prims: &[fill_rect(4.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.3676), prims: &[fill_rect(5.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.2639), prims: &[fill_rect(6.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1779), prims: &[fill_rect(7.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1005), prims: &[fill_rect(8.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.0406), prims: &[fill_rect(9.0, 0.0, 1.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.0097), prims: &[fill_rect(10.0, 0.0, 1.0, 0.5, Ink::Fg)] },
];
const XL_HELD: &[Prim] = &[
    Prim::Motion { motion: opacity(0.0097), prims: &[fill_rect(-8.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.0406), prims: &[fill_rect(-7.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1005), prims: &[fill_rect(-6.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1779), prims: &[fill_rect(-5.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.2639), prims: &[fill_rect(-4.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.3676), prims: &[fill_rect(-3.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.4880), prims: &[fill_rect(-2.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.6267), prims: &[fill_rect(-1.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
];
const XR_HELD: &[Prim] = &[
    Prim::Motion { motion: opacity(0.6267), prims: &[fill_rect(3.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.4880), prims: &[fill_rect(4.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.3676), prims: &[fill_rect(5.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.2639), prims: &[fill_rect(6.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1779), prims: &[fill_rect(7.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1005), prims: &[fill_rect(8.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.0406), prims: &[fill_rect(9.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.0097), prims: &[fill_rect(10.0, 0.0, 1.0, 0.5, card_ink(Ink::Fg, true))] },
];
const LH: &[Prim] = &[
    Prim::Motion { motion: opacity(0.0925), prims: &[Prim::At { x: 0.0, y: 0.0, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.1263), prims: &[Prim::At { x: 0.0, y: 0.5, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.2613), prims: &[Prim::At { x: 0.0, y: 1.0, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.7188), prims: &[Prim::At { x: 0.0, y: 1.5, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.9500), prims: &[Prim::At { x: 0.0, y: 2.0, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.6925), prims: &[Prim::At { x: 0.0, y: 2.5, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.6475), prims: &[Prim::At { x: 0.0, y: 3.0, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.9437), prims: &[Prim::At { x: 0.0, y: 3.5, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.8750), prims: &[Prim::At { x: 0.0, y: 4.0, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.5550), prims: &[Prim::At { x: 0.0, y: 4.5, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.2788), prims: &[Prim::At { x: 0.0, y: 5.0, prims: XL_HOVER }] },
    Prim::Motion { motion: opacity(0.0812), prims: &[Prim::At { x: 0.0, y: 5.5, prims: XL_HOVER }] },
];
const RH: &[Prim] = &[
    Prim::Motion { motion: opacity(0.0925), prims: &[Prim::At { x: 0.0, y: 0.0, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.1263), prims: &[Prim::At { x: 0.0, y: 0.5, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.2613), prims: &[Prim::At { x: 0.0, y: 1.0, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.7188), prims: &[Prim::At { x: 0.0, y: 1.5, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.9500), prims: &[Prim::At { x: 0.0, y: 2.0, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.6925), prims: &[Prim::At { x: 0.0, y: 2.5, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.6475), prims: &[Prim::At { x: 0.0, y: 3.0, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.9437), prims: &[Prim::At { x: 0.0, y: 3.5, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.8750), prims: &[Prim::At { x: 0.0, y: 4.0, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.5550), prims: &[Prim::At { x: 0.0, y: 4.5, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.2788), prims: &[Prim::At { x: 0.0, y: 5.0, prims: XR_HOVER }] },
    Prim::Motion { motion: opacity(0.0812), prims: &[Prim::At { x: 0.0, y: 5.5, prims: XR_HOVER }] },
];
const LP: &[Prim] = &[
    Prim::Motion { motion: opacity(0.0925), prims: &[Prim::At { x: 0.0, y: 0.0, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.1263), prims: &[Prim::At { x: 0.0, y: 0.5, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.2613), prims: &[Prim::At { x: 0.0, y: 1.0, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.7188), prims: &[Prim::At { x: 0.0, y: 1.5, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.9500), prims: &[Prim::At { x: 0.0, y: 2.0, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.6925), prims: &[Prim::At { x: 0.0, y: 2.5, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.6475), prims: &[Prim::At { x: 0.0, y: 3.0, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.9437), prims: &[Prim::At { x: 0.0, y: 3.5, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.8750), prims: &[Prim::At { x: 0.0, y: 4.0, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.5550), prims: &[Prim::At { x: 0.0, y: 4.5, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.2788), prims: &[Prim::At { x: 0.0, y: 5.0, prims: XL_HELD }] },
    Prim::Motion { motion: opacity(0.0812), prims: &[Prim::At { x: 0.0, y: 5.5, prims: XL_HELD }] },
];
const RP: &[Prim] = &[
    Prim::Motion { motion: opacity(0.0925), prims: &[Prim::At { x: 0.0, y: 0.0, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.1263), prims: &[Prim::At { x: 0.0, y: 0.5, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.2613), prims: &[Prim::At { x: 0.0, y: 1.0, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.7188), prims: &[Prim::At { x: 0.0, y: 1.5, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.9500), prims: &[Prim::At { x: 0.0, y: 2.0, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.6925), prims: &[Prim::At { x: 0.0, y: 2.5, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.6475), prims: &[Prim::At { x: 0.0, y: 3.0, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.9437), prims: &[Prim::At { x: 0.0, y: 3.5, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.8750), prims: &[Prim::At { x: 0.0, y: 4.0, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.5550), prims: &[Prim::At { x: 0.0, y: 4.5, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.2788), prims: &[Prim::At { x: 0.0, y: 5.0, prims: XR_HELD }] },
    Prim::Motion { motion: opacity(0.0812), prims: &[Prim::At { x: 0.0, y: 5.5, prims: XR_HELD }] },
];
const DH: &[Prim] = &[
    Prim::Motion { motion: opacity(0.6038), prims: &[fill_rect(0.0, 3.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.4362), prims: &[fill_rect(0.0, 3.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.2516), prims: &[fill_rect(0.0, 4.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.3015), prims: &[fill_rect(0.0, 4.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.3984), prims: &[fill_rect(0.0, 5.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.3143), prims: &[fill_rect(0.0, 5.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1763), prims: &[fill_rect(0.0, 6.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1851), prims: &[fill_rect(0.0, 6.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.2589), prims: &[fill_rect(0.0, 7.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.2237), prims: &[fill_rect(0.0, 7.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1264), prims: &[fill_rect(0.0, 8.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1143), prims: &[fill_rect(0.0, 8.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1657), prims: &[fill_rect(0.0, 9.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.1308), prims: &[fill_rect(0.0, 9.5, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.0398), prims: &[fill_rect(0.0, 10.0, 3.0, 0.5, Ink::Fg)] },
    Prim::Motion { motion: opacity(0.0024), prims: &[fill_rect(0.0, 10.5, 3.0, 0.5, Ink::Fg)] },
];
const DP: &[Prim] = &[
    Prim::Motion { motion: opacity(0.6038), prims: &[fill_rect(0.0, 3.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.4362), prims: &[fill_rect(0.0, 3.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.2516), prims: &[fill_rect(0.0, 4.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.3015), prims: &[fill_rect(0.0, 4.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.3984), prims: &[fill_rect(0.0, 5.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.3143), prims: &[fill_rect(0.0, 5.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1763), prims: &[fill_rect(0.0, 6.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1851), prims: &[fill_rect(0.0, 6.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.2589), prims: &[fill_rect(0.0, 7.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.2237), prims: &[fill_rect(0.0, 7.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1264), prims: &[fill_rect(0.0, 8.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1143), prims: &[fill_rect(0.0, 8.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1657), prims: &[fill_rect(0.0, 9.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.1308), prims: &[fill_rect(0.0, 9.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.0398), prims: &[fill_rect(0.0, 10.0, 3.0, 0.5, card_ink(Ink::Fg, true))] },
    Prim::Motion { motion: opacity(0.0024), prims: &[fill_rect(0.0, 10.5, 3.0, 0.5, card_ink(Ink::Fg, true))] },
];
pub(super) const ACTIVE_LEFT1_HOVER: &[Prim] = &cells((14.375, 520.5833), LH);
pub(super) const ACTIVE_LEFT1_HELD: &[Prim] = &cells((14.375, 520.5833), LP);
pub(super) const ACTIVE_RIGHT3_HOVER: &[Prim] = &cells((12.4583, 520.5833), RH);
pub(super) const ACTIVE_RIGHT3_HELD: &[Prim] = &cells((12.4583, 520.5833), RP);
pub(super) const ACTIVE_RIGHT4_HOVER: &[Prim] = &cells((11.7917, 520.1666), RH);
pub(super) const ACTIVE_RIGHT4_HELD: &[Prim] = &cells((11.7917, 520.1666), RP);
pub(super) const ACTIVE_DOWN_HOVER: &[Prim] = &cells((12.4167, 711.2917), DH);
pub(super) const ACTIVE_DOWN_HELD: &[Prim] = &cells((12.4167, 711.2917), DP);
