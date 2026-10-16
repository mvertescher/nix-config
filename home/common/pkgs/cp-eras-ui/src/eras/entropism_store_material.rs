//! Ordinary store weapons, composited once in the source's encoded RGB.
//! Shared static layers keep selection changes warm. Custom palettes use
//! the semantic foreground; selected and hovered inverse art stays native.
use super::*;
use crate::style::{PlateStates, StoreBackdrop, StoreReference};

// These two clipped paths are the accepted SVG stock seam and lower band.


const fn with_fill(p: Prim, ink: Ink) -> Prim {
    let Prim::Path { x, y, segs, close, stroke, width, .. } = p else {
        panic!("expected ordinary rifle path")
    };
    Prim::Path { x, y, segs, close, fill: Some(ink), stroke, width }
}
const MATERIAL: &[Prim] = &[
    with_fill(RIFLE_PLAIN, Ink::Fixed(rgb(0x779375))),
    with_fill(RIFLE_PLAIN_DETAIL, Ink::Fixed(rgb(0xb3ddb1))),
    SEAM, BAND,
];

const fn bare_card() -> [Prim; CARD.len()] {
    let mut out = [CARD[0]; CARD.len()];
    let mut i = 0;
    let mut removed = 0;
    while i < CARD.len() {
        out[i] = match CARD[i] {
            Prim::Path { x: 237.833, y: 131.667, fill: Some(Ink::Mid), .. }
            | Prim::Path { x: 78.25, y: 105.833, fill: Some(Ink::Select), .. } => {
                removed += 1;
                Prim::At { x: 0.0, y: 0.0, prims: &[] }
            }
            other => other,
        };
        i += 1;
    }
    assert!(removed == 2);
    out
}
const CARD_BARE: &[Prim] = &bare_card();
const fn bare_shelf(source: &[Prim]) -> [Prim; 1] {
    assert!(source.len() == 1);
    let mut p = source[0];
    let Prim::Plate { off, .. } = &mut p else { panic!("expected product plate") };
    *off = CARD_BARE;
    [p]
}
const FOURTH_PLATE: &[Prim] = &bare_shelf(match SHELF_3[0] {
    Prim::Viewport { prims, .. } => prims,
    _ => panic!("expected fourth-card cut"),
});
const FOURTH: &[Prim] = &{
    let mut p = SHELF_3[0];
    let Prim::Viewport { prims, .. } = &mut p else { panic!("expected fourth-card cut") };
    *prims = FOURTH_PLATE;
    [p]
};
const BODY: &[Prim] = &{
    let mut out = [STORE_BODY[0]; STORE_BODY.len()];
    let branches: [&[Prim]; 4] = [&bare_shelf(SHELF_0), &bare_shelf(SHELF_1),
        &bare_shelf(SHELF_2), FOURTH];
    let mut i = 0;
    let mut cards = 0;
    while i < out.len() {
        out[i] = match STORE_BODY[i] {
            Prim::At { x, y: 260.0, .. } if x == 461.0 || x == 783.0 || x == 1105.0 || x == 1429.0 => {
                let p = Prim::At { x, y: 260.0, prims: branches[cards] };
                cards += 1;
                p
            }
            other => other,
        };
        i += 1;
    }
    assert!(cards == 4);
    out
};
const SCENE: &[Prim] = &{
    let mut out = [STORE[0]; STORE.len()];
    let mut i = 0;
    let mut found = 0;
    while i < out.len() {
        out[i] = match STORE[i] {
            Prim::Motion { motion: motion @ Motion { change: Change::Clip { x: 100.0, y: 90.0, .. }, .. }, .. } => {
                found += 1;
                Prim::Motion { motion, prims: BODY }
            }
            other => other,
        };
        i += 1;
    }
    assert!(found == 1);
    out
};
const STATES: &[PlateStates] = &{
    let mut out = [STORE_STATES[0]; STORE_STATES.len()];
    let mut i = 0;
    while i < out.len() {
        out[i] = STORE_STATES[i];
        // Hover paints its opaque inverse header over the cached image.
        // A held ordinary card exposes that image, without drawing it twice.
        out[i].pressed = CARD_BARE;
        i += 1;
    }
    out
};

// Keep slice identity and ordered prefixes fixed for every selection.
// A selected card hides its cached ordinary weapon with an empty clip;
// prefetch still prepares that layer for a subsequent card selection.
static RIFLES: [[Prim; 1]; 4] = [
    [Prim::At { x: 461.0, y: 260.0, prims: MATERIAL }],
    [Prim::At { x: 783.0, y: 260.0, prims: MATERIAL }],
    [Prim::At { x: 1105.0, y: 260.0, prims: MATERIAL }],
    // Only this ordinary weapon sits left of its card chrome in the source.
    [Prim::At { x: 1427.0, y: 260.0, prims: MATERIAL }],
];
const SOFT_RIFLES: [&[Prim]; 4] = [
    &[Prim::Soft { prims: &RIFLES[0] }],
    &[Prim::Soft { prims: &RIFLES[1] }],
    &[Prim::Soft { prims: &RIFLES[2] }],
    &[Prim::Soft { prims: &RIFLES[3] }],
];
const fn clipped_rifle(index: usize, selected: usize) -> Prim {
    let x = [459.0, 781.0, 1103.0, 1427.0][index];
    let w = if index == selected { 0.0 } else if index == 3 { 137.6 } else { 269.0 };
    Prim::Motion {
        motion: Motion { id: "store-ordinary-rifle", begin: 0, dur: 0,
            ease: Easing::Linear,
            change: Change::Clip { x, y: 258.0, w: (w, w), h: (492.0, 492.0) } },
        prims: SOFT_RIFLES[index],
    }
}
const fn rifle_row(selected: usize) -> [Prim; 4] {
    [clipped_rifle(0, selected), clipped_rifle(1, selected),
     clipped_rifle(2, selected), clipped_rifle(3, selected)]
}
const ROWS: [&[Prim]; 4] = [&rifle_row(0), &rifle_row(1), &rifle_row(2), &rifle_row(3)];
const fn backdrop(card: usize) -> [Prim; 2] {
    [STORE[0], Prim::Motion {
        motion: body_scan(100.0, 90.0, 1500.0, 730.0), prims: ROWS[card],
    }]
}
const BACKDROPS: [&[Prim]; 4] = [&backdrop(0), &backdrop(1), &backdrop(2), &backdrop(3)];
const ROUTES: &[StoreBackdrop] = &{
    let mut out = [StoreBackdrop { category: 0, card: 0, prims: BACKDROPS[0] }; 20];
    let mut i = 0;
    while i < out.len() {
        let card = i / 5;
        out[i] = StoreBackdrop { category: i % 5, card, prims: BACKDROPS[card] };
        i += 1;
    }
    out
};
pub(super) const REFERENCE: StoreReference = StoreReference {
    scene: SCENE, backdrops: ROUTES, states: Some(STATES),
};

// Exact clipped stock contours from docs/entropism/store-trace.svg.
// Expanded and intersected offline; no runtime stroke or path construction.
const SEAM: Prim = Prim::Path {
    x: 207.16636592, y: 134.53759378,
    segs: &[
        Seg::Line(219.23660107, 147.80487705),
        Seg::Line(219.43417888, 147.93787400),
        Seg::Line(219.59277522, 148.04244193),
        Seg::Line(219.75087273, 148.14448215),
        Seg::Line(219.90847378, 148.24399339),
        Seg::Line(220.06558073, 148.34097422),
        Seg::Line(220.22219601, 148.43542313),
        Seg::Line(220.37832201, 148.52733844),
        Seg::Line(220.53396117, 148.61671835),
        Seg::Line(220.68911588, 148.70356092),
        Seg::Line(220.84378856, 148.78786404),
        Seg::Line(220.99798158, 148.86962547),
        Seg::Line(221.15169729, 148.94884277),
        Seg::Line(221.30493798, 149.02551336),
        Seg::Line(221.45770591, 149.09963447),
        Seg::Line(221.61000325, 149.17120315),
        Seg::Line(221.76183209, 149.24021626),
        Seg::Line(221.91319441, 149.30667047),
        Seg::Line(222.06409207, 149.37056225),
        Seg::Line(222.21452682, 149.43188790),
        Seg::Line(222.36450021, 149.49064349),
        Seg::Line(222.51401365, 149.54682491),
        Seg::Line(222.66306832, 149.60042785),
        Seg::Line(222.81166520, 149.65144780),
        Seg::Line(222.95980500, 149.69988008),
        Seg::Line(223.10748820, 149.74571980),
        Seg::Line(223.25471494, 149.78896194),
        Seg::Line(223.40148508, 149.82960127),
        Seg::Line(223.54779809, 149.86763246),
        Seg::Line(223.69365313, 149.90305002),
        Seg::Line(223.83904892, 149.93584837),
        Seg::Line(223.98398379, 149.96602183),
        Seg::Line(224.16221131, 150.00000000),
        Seg::Line(224.91700000, 150.00000000),
        Seg::Line(224.91700000, 149.20000000),
        Seg::Line(224.23778869, 149.20000000),
        Seg::Line(224.14043027, 149.18143911),
        Seg::Line(224.00860733, 149.15399538),
        Seg::Line(223.87607343, 149.12409841),
        Seg::Line(223.74282691, 149.09174254),
        Seg::Line(223.60886649, 149.05692217),
        Seg::Line(223.47419131, 149.01963181),
        Seg::Line(223.33880086, 148.97986613),
        Seg::Line(223.20269500, 148.93761992),
        Seg::Line(223.06587387, 148.89288813),
        Seg::Line(222.92833793, 148.84566590),
        Seg::Line(222.79008791, 148.79594852),
        Seg::Line(222.65112479, 148.74373151),
        Seg::Line(222.51144974, 148.68901053),
        Seg::Line(222.37106418, 148.63178150),
        Seg::Line(222.22996965, 148.57204047),
        Seg::Line(222.08816791, 148.50978374),
        Seg::Line(221.94566081, 148.44500779),
        Seg::Line(221.80245034, 148.37770928),
        Seg::Line(221.65853858, 148.30788507),
        Seg::Line(221.51392771, 148.23553223),
        Seg::Line(221.36861998, 148.16064797),
        Seg::Line(221.22261769, 148.08322971),
        Seg::Line(221.07592318, 148.00327502),
        Seg::Line(220.92853883, 147.92078165),
        Seg::Line(220.78046705, 147.83574750),
        Seg::Line(220.63171024, 147.74817062),
        Seg::Line(220.48227083, 147.65804922),
        Seg::Line(220.33215122, 147.56538161),
        Seg::Line(220.18135383, 147.47016629),
        Seg::Line(220.02988103, 147.37240182),
        Seg::Line(219.87773519, 147.27208693),
        Seg::Line(219.76339893, 147.19512295),
        Seg::Line(207.63363408, 133.86240622),
        Seg::Line(206.65079271, 133.47196164),
        Seg::Line(205.75000000, 134.71055155),
        Seg::Line(205.75000000, 135.00000000),
        Seg::Line(205.41897466, 135.16571137),
        Seg::Line(204.91700000, 135.85592648),
        Seg::Line(204.91700000, 136.09872726),
        Seg::Line(205.44849443, 136.48526869),
        Seg::Line(206.93254129, 134.44470436),
    ],
    close: true, fill: Some(Ink::Fixed(rgb(0xa5cca4))),
    stroke: None, width: 0.0,
};

const BAND: Prim = Prim::Path {
    x: 219.19271149, y: 147.16059291,
    segs: &[
        Seg::Line(218.37542022, 148.14134243),
        Seg::Line(217.42625176, 149.02723300),
        Seg::Line(215.75000000, 147.35098124),
        Seg::Line(215.75000000, 147.91700000),
        Seg::Line(214.91700000, 147.50000000),
        Seg::Line(214.91700000, 147.64935209),
        Seg::Line(217.14325542, 149.87560751),
        Seg::Line(217.27367695, 149.98970386),
        Seg::Line(217.39813305, 150.09515289),
        Seg::Line(217.52427761, 150.19865070),
        Seg::Line(217.65210682, 150.30019449),
        Seg::Line(217.78161672, 150.39978168),
        Seg::Line(217.91280328, 150.49740987),
        Seg::Line(218.04566237, 150.59307688),
        Seg::Line(218.18018980, 150.68678074),
        Seg::Line(218.31638133, 150.77851969),
        Seg::Line(218.45423267, 150.86829220),
        Seg::Line(218.59373955, 150.95609699),
        Seg::Line(218.73489766, 151.04193295),
        Seg::Line(218.87770273, 151.12579924),
        Seg::Line(219.02215051, 151.20769522),
        Seg::Line(219.16823679, 151.28762046),
        Seg::Line(219.31595744, 151.36557476),
        Seg::Line(219.46530839, 151.44155811),
        Seg::Line(219.61628564, 151.51557070),
        Seg::Line(219.76888531, 151.58761292),
        Seg::Line(219.92310362, 151.65768534),
        Seg::Line(220.07893690, 151.72578869),
        Seg::Line(220.23638161, 151.79192389),
        Seg::Line(220.39543434, 151.85609199),
        Seg::Line(220.55609183, 151.91829419),
        Seg::Line(220.71835093, 151.97853183),
        Seg::Line(220.88220869, 152.03680637),
        Seg::Line(221.04766225, 152.09311938),
        Seg::Line(221.16700000, 152.13194912),
        Seg::Line(221.16700000, 151.66700000),
        Seg::Line(222.00000000, 151.66700000),
        Seg::Line(222.40863502, 152.48329010),
        Seg::Line(224.50000000, 153.04918886),
        Seg::Line(224.50000000, 152.50000000),
        Seg::Line(224.91700000, 152.50000000),
        Seg::Line(224.91700000, 152.33325428),
        Seg::Line(222.10407651, 151.57211027),
        Seg::Line(221.93770243, 151.52746254),
        Seg::Line(221.86962297, 151.50825085),
        Seg::Line(222.38179199, 149.86931000),
        Seg::Line(221.61820801, 149.63069000),
        Seg::Line(221.10668797, 151.26755412),
        Seg::Line(220.99161000, 151.22662767),
        Seg::Line(220.83974151, 151.17024747),
        Seg::Line(220.68952659, 151.11208835),
        Seg::Line(220.54096214, 151.05215163),
        Seg::Line(220.39404487, 150.99043852),
        Seg::Line(220.24877138, 150.92695008),
        Seg::Line(220.10513813, 150.86168721),
        Seg::Line(219.96314144, 150.79465066),
        Seg::Line(219.82277755, 150.72584098),
        Seg::Line(219.68404256, 150.65525858),
        Seg::Line(219.54693248, 150.58290363),
        Seg::Line(219.41144324, 150.50877614),
        Seg::Line(219.27757071, 150.43287589),
        Seg::Line(219.14531067, 150.35520247),
        Seg::Line(219.01465889, 150.27575523),
        Seg::Line(218.88561108, 150.19453332),
        Seg::Line(218.75816295, 150.11153565),
        Seg::Line(218.63231020, 150.02676093),
        Seg::Line(218.50804857, 149.94020762),
        Seg::Line(218.38537380, 149.85187398),
        Seg::Line(218.26428172, 149.76175803),
        Seg::Line(218.14476818, 149.66985759),
        Seg::Line(218.02682916, 149.57617027),
        Seg::Line(218.01818071, 149.56907448),
        Seg::Line(218.95791312, 148.69199090),
        Seg::Line(219.80728851, 147.67274043),
        Seg::Move(215.40784271, 147.00882395),
        Seg::Line(215.33366667, 147.08300000),
        Seg::Line(215.48201876, 147.08300000),
    ],
    close: true, fill: Some(Ink::Fixed(rgb(0x8bac89))),
    stroke: None, width: 0.0,
};
