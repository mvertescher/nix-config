//! Bounded dashboard glyph ink, docs/neomil/dashboard-trace.svg
//! #dashboard-matrix-0 through -5. The accepted glyph silhouette clips two
//! displaced coverage copies; no new modules or exterior haze are drawn.
//! RGB row material is fitted locally at the existing 1.984934 px period.
//! Use only inside the dashboard's leading Soft group, over its palette-role
//! tile faces. Hover/held keep their opaque, independently inferred inks.
use crate::palette::rgb;
use crate::style::{fill_rect, Ink, Prim};
use super::dashboard_glyphs;

const fn alpha(a: f32) -> Prim {
    fill_rect(-38.0, -38.0, 76.0, 76.0,
        Ink::Fixed(iced::Color { a, ..rgb(0xffffff) }))
}

const fn cycles<const N: usize>(first: f32, stops: &'static [(f32, iced::Color)]) -> [Prim; N] {
    let mut out = [fill_rect(0.0, 0.0, 0.0, 0.0, Ink::Fg); N];
    let mut i = 0;
    while i < N {
        out[i] = Prim::Ramp { x: -38.0, y: first + i as f32 * 1.984934,
            w: 76.0, h: 1.984934, from: (0.0, 0.0), to: (0.0, 1.0), stops };
        i += 1;
    }
    out
}

// VEHICLES: source-centred local coordinates.
const CYCLE_0: &[(f32, iced::Color)] = &[
    (0.0000, rgb(0x260000)),
    (0.0625, rgb(0x2b0000)),
    (0.1250, rgb(0x2e0000)),
    (0.1875, rgb(0x2c0000)),
    (0.2500, rgb(0x270000)),
    (0.3125, rgb(0x1f0000)),
    (0.3750, rgb(0x150000)),
    (0.4375, rgb(0x0b0606)),
    (0.5000, rgb(0x020b0d)),
    (0.5625, rgb(0x000f11)),
    (0.6250, rgb(0x001012)),
    (0.6875, rgb(0x000f11)),
    (0.7500, rgb(0x010c0e)),
    (0.8125, rgb(0x090708)),
    (0.8750, rgb(0x130101)),
    (0.9375, rgb(0x1d0000)),
    (1.0000, rgb(0x260000)),
];
const FIELD_0: &[Prim] = &[
    fill_rect(-38.0, -38.0, 76.0, 76.0, Ink::Fixed(rgb(0x260000))),
    Prim::At { x: 0.0, y: 0.0, prims: &cycles::<39>(-39.19399200, CYCLE_0) },
];
const COVERAGE_0: &[Prim] = &[
    alpha(0.54843792),
    // Each opacity applies to the whole silhouette union, like SVG use opacity.
    Prim::At { x: -2.32947284, y: -0.14997453, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[0]], mask: &[alpha(0.62943826)] },
    ] },
    Prim::At { x: -4.65894568, y: -0.29994905, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[0]], mask: &[alpha(0.38581572)] },
    ] },
];

// LOCATIONS: source-centred local coordinates.
const CYCLE_1: &[(f32, iced::Color)] = &[
    (0.0000, rgb(0x180000)),
    (0.0625, rgb(0x1c0000)),
    (0.1250, rgb(0x1e0000)),
    (0.1875, rgb(0x1c0000)),
    (0.2500, rgb(0x190000)),
    (0.3125, rgb(0x130000)),
    (0.3750, rgb(0x0c0000)),
    (0.4375, rgb(0x050405)),
    (0.5000, rgb(0x000809)),
    (0.5625, rgb(0x000a0c)),
    (0.6250, rgb(0x000b0d)),
    (0.6875, rgb(0x000a0c)),
    (0.7500, rgb(0x00080a)),
    (0.8125, rgb(0x040506)),
    (0.8750, rgb(0x0b0001)),
    (0.9375, rgb(0x120000)),
    (1.0000, rgb(0x180000)),
];
const FIELD_1: &[Prim] = &[
    fill_rect(-38.0, -38.0, 76.0, 76.0, Ink::Fixed(rgb(0x180000))),
    Prim::At { x: 0.0, y: 0.0, prims: &cycles::<39>(-39.19399200, CYCLE_1) },
];
const COVERAGE_1: &[Prim] = &[
    alpha(0.53736864),
    // Each opacity applies to the whole silhouette union, like SVG use opacity.
    Prim::At { x: -1.36872569, y: -0.17318157, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[1]], mask: &[alpha(0.57355369)] },
    ] },
    Prim::At { x: -2.73745137, y: -0.34636314, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[1]], mask: &[alpha(0.56605754)] },
    ] },
];

// FACTIONS: source-centred local coordinates.
const CYCLE_2: &[(f32, iced::Color)] = &[
    (0.0000, rgb(0x1f080a)),
    (0.0625, rgb(0x21070a)),
    (0.1250, rgb(0x22070a)),
    (0.1875, rgb(0x21070a)),
    (0.2500, rgb(0x20070a)),
    (0.3125, rgb(0x1d080b)),
    (0.3750, rgb(0x1a090c)),
    (0.4375, rgb(0x170a0d)),
    (0.5000, rgb(0x150b0d)),
    (0.5625, rgb(0x130b0e)),
    (0.6250, rgb(0x120b0e)),
    (0.6875, rgb(0x120b0e)),
    (0.7500, rgb(0x140b0d)),
    (0.8125, rgb(0x170a0d)),
    (0.8750, rgb(0x1a090c)),
    (0.9375, rgb(0x1d080b)),
    (1.0000, rgb(0x1f080a)),
];
const FIELD_2: &[Prim] = &[
    fill_rect(-38.0, -38.0, 76.0, 76.0, Ink::Fixed(rgb(0x1f080a))),
    Prim::At { x: 0.0, y: 0.0, prims: &cycles::<39>(-39.19399200, CYCLE_2) },
];
const COVERAGE_2: &[Prim] = &[
    alpha(0.72926767),
    // Each opacity applies to the whole silhouette union, like SVG use opacity.
    Prim::At { x: -0.29770151, y: -0.48437809, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[2]], mask: &[alpha(0.45576068)] },
    ] },
    Prim::At { x: -0.59540302, y: -0.96875617, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[2]], mask: &[alpha(0.99000000)] },
    ] },
];

// WEAPONS: source-centred local coordinates.
const CYCLE_3: &[(f32, iced::Color)] = &[
    (0.0000, rgb(0x1f0000)),
    (0.0625, rgb(0x240000)),
    (0.1250, rgb(0x260000)),
    (0.1875, rgb(0x250000)),
    (0.2500, rgb(0x210000)),
    (0.3125, rgb(0x1a0000)),
    (0.3750, rgb(0x120000)),
    (0.4375, rgb(0x0a0405)),
    (0.5000, rgb(0x020709)),
    (0.5625, rgb(0x000a0c)),
    (0.6250, rgb(0x000b0d)),
    (0.6875, rgb(0x000a0d)),
    (0.7500, rgb(0x00080a)),
    (0.8125, rgb(0x070506)),
    (0.8750, rgb(0x0f0101)),
    (0.9375, rgb(0x170000)),
    (1.0000, rgb(0x1f0000)),
];
const FIELD_3: &[Prim] = &[
    fill_rect(-38.0, -38.0, 76.0, 76.0, Ink::Fixed(rgb(0x1f0000))),
    Prim::At { x: 0.0, y: 0.0, prims: &cycles::<39>(-39.20341400, CYCLE_3) },
];
const COVERAGE_3: &[Prim] = &[
    alpha(0.54477343),
    // Each opacity applies to the whole silhouette union, like SVG use opacity.
    Prim::At { x: -1.75021662, y: 0.45807165, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[3]], mask: &[alpha(0.56730081)] },
    ] },
    Prim::At { x: -3.50043325, y: 0.91614329, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[3]], mask: &[alpha(0.55767881)] },
    ] },
];

// PRODUCTS: source-centred local coordinates.
const CYCLE_4: &[(f32, iced::Color)] = &[
    (0.0000, rgb(0x120000)),
    (0.0625, rgb(0x160000)),
    (0.1250, rgb(0x170000)),
    (0.1875, rgb(0x160000)),
    (0.2500, rgb(0x140000)),
    (0.3125, rgb(0x100000)),
    (0.3750, rgb(0x0b0000)),
    (0.4375, rgb(0x060203)),
    (0.5000, rgb(0x020406)),
    (0.5625, rgb(0x000608)),
    (0.6250, rgb(0x000709)),
    (0.6875, rgb(0x000608)),
    (0.7500, rgb(0x000507)),
    (0.8125, rgb(0x040304)),
    (0.8750, rgb(0x090101)),
    (0.9375, rgb(0x0e0000)),
    (1.0000, rgb(0x120000)),
];
const FIELD_4: &[Prim] = &[
    fill_rect(-38.0, -38.0, 76.0, 76.0, Ink::Fixed(rgb(0x120000))),
    Prim::At { x: 0.0, y: 0.0, prims: &cycles::<39>(-38.20341400, CYCLE_4) },
];
const COVERAGE_4: &[Prim] = &[
    alpha(0.60908263),
    // Each opacity applies to the whole silhouette union, like SVG use opacity.
    Prim::At { x: -0.99554722, y: 0.72262209, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[4]], mask: &[alpha(0.71818216)] },
    ] },
    Prim::At { x: -1.99109444, y: 1.44524419, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[4]], mask: &[alpha(0.51598081)] },
    ] },
];

// CORPORATIONS: source-centred local coordinates.
const CYCLE_5: &[(f32, iced::Color)] = &[
    (0.0000, rgb(0x0d0001)),
    (0.0625, rgb(0x0f0001)),
    (0.1250, rgb(0x100001)),
    (0.1875, rgb(0x100001)),
    (0.2500, rgb(0x0e0001)),
    (0.3125, rgb(0x0b0002)),
    (0.3750, rgb(0x080003)),
    (0.4375, rgb(0x040104)),
    (0.5000, rgb(0x010304)),
    (0.5625, rgb(0x000405)),
    (0.6250, rgb(0x000505)),
    (0.6875, rgb(0x000505)),
    (0.7500, rgb(0x000405)),
    (0.8125, rgb(0x020204)),
    (0.8750, rgb(0x060003)),
    (0.9375, rgb(0x090002)),
    (1.0000, rgb(0x0d0001)),
];
const FIELD_5: &[Prim] = &[
    fill_rect(-38.0, -38.0, 76.0, 76.0, Ink::Fixed(rgb(0x0d0001))),
    Prim::At { x: 0.0, y: 0.0, prims: &cycles::<39>(-38.20341400, CYCLE_5) },
];
const COVERAGE_5: &[Prim] = &[
    alpha(0.65800094),
    // Each opacity applies to the whole silhouette union, like SVG use opacity.
    Prim::At { x: 0.28755087, y: 0.70289150, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[5]], mask: &[alpha(0.59295099)] },
    ] },
    Prim::At { x: 0.57510173, y: 1.40578301, prims: &[
        Prim::Masked { prims: &[dashboard_glyphs::WHITE[5]], mask: &[alpha(0.79861326)] },
    ] },
];

pub(super) const REST: [Prim; 6] = [
    Prim::Masked { mask: &[dashboard_glyphs::WHITE[0]], prims: &[
        Prim::Masked { prims: FIELD_0, mask: COVERAGE_0 },
    ] },
    Prim::Masked { mask: &[dashboard_glyphs::WHITE[1]], prims: &[
        Prim::Masked { prims: FIELD_1, mask: COVERAGE_1 },
    ] },
    Prim::Masked { mask: &[dashboard_glyphs::WHITE[2]], prims: &[
        Prim::Masked { prims: FIELD_2, mask: COVERAGE_2 },
    ] },
    Prim::Masked { mask: &[dashboard_glyphs::WHITE[3]], prims: &[
        Prim::Masked { prims: FIELD_3, mask: COVERAGE_3 },
    ] },
    Prim::Masked { mask: &[dashboard_glyphs::WHITE[4]], prims: &[
        Prim::Masked { prims: FIELD_4, mask: COVERAGE_4 },
    ] },
    Prim::Masked { mask: &[dashboard_glyphs::WHITE[5]], prims: &[
        Prim::Masked { prims: FIELD_5, mask: COVERAGE_5 },
    ] },
];
