//! Source-fitted store surfaces. The foreground preserves every hit box,
//! label and inferred feedback drawing from the semantic scene. Static
//! backgrounds select the same card/category and share its opening clip.
//! Custom palettes keep the original scene and materials.
use super::*;
use crate::style::{StoreBackdrop, StoreReference};

const WHITE: Ink = Ink::Fixed(rgb(0xffffff));
const fn alpha(color: iced::Color, a: f32) -> Ink { Ink::Fixed(iced::Color { a, ..color }) }
const fn clear_fill(mut p: Prim) -> Prim {
    match &mut p {
        Prim::Path { fill, .. } | Prim::Rect { fill, .. } => *fill = None,
        _ => panic!("expected store surface"),
    }
    p
}
const fn bare_card<const N: usize>(source: &[Prim], grown: bool, bare_face: &'static [Prim]) -> [Prim; N] {
    assert!(source.len() == N);
    let mut out = [source[0]; N]; let mut i = 0;
    while i < N { out[i] = source[i]; i += 1; }
    out[0] = if grown { clear_fill(source[0]) } else { card_underlay(bare_face) };
    if grown { out[1] = Prim::At { x: 0.0, y: 0.0, prims: &[] }; }
    out
}
const C1_FACE: &[Prim] = &[clear_fill(frame_leaf(CARD1[0]))];
const C1: &[Prim] = &bare_card::<{ CARD1.len() }>(CARD1, false, C1_FACE);
const C3_FACE: &[Prim] = &[clear_fill(frame_leaf(CARD3[0]))];
const C3: &[Prim] = &bare_card::<{ CARD3.len() }>(CARD3, false, C3_FACE);
const C4_FACE: &[Prim] = &[clear_fill(frame_leaf(CARD4[0]))];
const C4: &[Prim] = &bare_card::<{ CARD4.len() }>(CARD4, false, C4_FACE);
const CG: &[Prim] = &bare_card::<{ GROWN.len() }>(GROWN, true, &[]);
const fn unwrap_view(source: &'static [Prim]) -> &'static [Prim] {
    match source[0] { Prim::Viewport { prims, .. } => prims, _ => source }
}
const fn card_branch(source: &'static [Prim], selected: bool, idle: &'static [Prim]) -> [Prim; 1] {
    let Prim::Pick { on, off, .. } = unwrap_view(source)[0] else { panic!("expected store pick") };
    let mut p = if selected { on[0] } else { off[0] };
    let Prim::Plate { on, off, .. } = &mut p else { panic!("expected product plate") };
    *on = CG; *off = idle;
    [p]
}
const fn card_pick(source: &'static [Prim], on: &'static [Prim], off: &'static [Prim]) -> [Prim; 1] {
    let mut p = unwrap_view(source)[0];
    let Prim::Pick { on: a, off: b, .. } = &mut p else { panic!("expected product pick") };
    *a = on; *b = off; [p]
}


const PICK0: &[Prim] = &card_pick(SHELF_0, &card_branch(SHELF_0, true, C1), &card_branch(SHELF_0, false, C1));

const PICK1: &[Prim] = &card_pick(SHELF_1, &card_branch(SHELF_1, true, C1), &card_branch(SHELF_1, false, C1));

const PICK2: &[Prim] = &card_pick(SHELF_2, &card_branch(SHELF_2, true, C3), &card_branch(SHELF_2, false, C3));

const PICK3: &[Prim] = &card_pick(SHELF_3, &card_branch(SHELF_3, true, C4), &card_branch(SHELF_3, false, C4));

const CUT: &[Prim] = &{
    let mut p = SHELF_3[0];
    let Prim::Viewport { prims, .. } = &mut p else { panic!("expected card-four cut") };
    *prims = PICK3; [p]
};
const SHELF_REFERENCE: &[Prim] = &{
    let mut out = [SHELF[0]; 4]; let branches = [PICK0, PICK1, PICK2, CUT]; let mut i = 0;
    while i < 4 {
        out[i] = SHELF[i];
        let Prim::At { prims, .. } = &mut out[i] else { panic!("expected shelf column") };
        *prims = branches[i]; i += 1;
    }
    out
};
const NAV_ROW_BARE: &[Prim] = &[clear_fill(NAV_ROW[0]), NAV_ROW[1]];
const NAV_SELECTED_BARE: &[Prim] = &[clear_fill(NAV_SELECTED[0]), NAV_SELECTED[1]];
const fn nav_branch(source: &[Prim], selected: bool) -> [Prim; 2] {
    assert!(source.len() == 2);
    let mut out = [source[0], source[1]];
    let Prim::At { prims, .. } = &mut out[0] else { panic!("expected nav surface") };
    *prims = if selected { NAV_SELECTED_BARE } else { NAV_ROW_BARE }; out
}


const NAV_ON0: &[Prim] = &nav_branch(NAV_ON_0, true);

const NAV_OFF0: &[Prim] = &nav_branch(NAV_OFF_0, false);

const NAV_ON1: &[Prim] = &nav_branch(NAV_ON_1, true);

const NAV_OFF1: &[Prim] = &nav_branch(NAV_OFF_1, false);

const NAV_ON2: &[Prim] = &nav_branch(NAV_ON_2, true);

const NAV_OFF2: &[Prim] = &nav_branch(NAV_OFF_2, false);

const NAV_ON3: &[Prim] = &nav_branch(NAV_ON_3, true);

const NAV_OFF3: &[Prim] = &nav_branch(NAV_OFF_3, false);

const NAV_ON4: &[Prim] = &nav_branch(NAV_ON_4, true);

const NAV_OFF4: &[Prim] = &nav_branch(NAV_OFF_4, false);

const SCENE: &[Prim] = &{
    let on = [NAV_ON0, NAV_ON1, NAV_ON2, NAV_ON3, NAV_ON4];
    let off = [NAV_OFF0, NAV_OFF1, NAV_OFF2, NAV_OFF3, NAV_OFF4];
    let mut out = [STORE[0]; STORE.len()]; let mut i = 0;
    while i < out.len() {
        out[i] = match STORE[i] {
            Prim::Plate { group: Group::Category, index, x, y, w, h, .. } =>
                Prim::Plate { group: Group::Category, index, x, y, w, h, on: on[index], off: off[index] },
            Prim::Motion { motion, .. } if matches!(motion.change, Change::Clip { .. }) =>
                Prim::Motion { motion, prims: SHELF_REFERENCE },
            // Clear source patches inside this stamp match the surrounding
            // ground. Keep the semantic filled stamp for custom palettes.
            Prim::Rect { x, y, w, h, stroke, width, .. } if x == 153.5 && y == 851.5 =>
                Prim::Rect { x, y, w, h, fill: None, stroke, width },
            // The source's unboxed footer code uses the bright primary ink.
            // The semantic scene retains Dim for custom palettes and variants.
            Prim::Wide { x, y, size, stretch, face, anchor, content, .. } if x == 313.0 =>
                Prim::Wide { x, y, size, stretch, ink: Ink::Fg, face, anchor, content },
            other => other,
        };
        i += 1;
    }
    out
};
const SHELF_OPEN: Motion = {
    let mut i = 0; let mut found = None;
    while i < STORE.len() {
        if let Prim::Motion { motion: motion @ Motion { change: Change::Clip { .. }, .. }, .. } = STORE[i] {
            assert!(found.is_none()); found = Some(motion);
        }
        i += 1;
    }
    match found { Some(motion) => motion, None => panic!("missing shelf motion") }
};
// Only the mask follows the selected contour. Ramp origins below stay at
// y151 so already fitted interior samples keep their world coordinates.
const UPPER_MASK: &[Prim] = &[fill_path(0.0, 154.5833, &[
    Seg::Line(267.25, 154.5833), Seg::Line(279.3333, 166.6667),
    Seg::Line(279.3333, 262.9167), Seg::Line(269.75, 271.25),
    Seg::Line(269.75, 492.0), Seg::Line(246.0, 516.0), Seg::Line(0.0, 516.0),
], WHITE)];
const LOWER_MASK: &[Prim] = &[fill_path(0.0, 154.5833, FRAME_SEL, WHITE)];


const UPPER_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 151.0, w: 282.0, h: 365.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0x562746)), (0.0794521, rgb(0x4f2547)), (0.2301370, rgb(0x472a47)), (0.3397260, rgb(0x412a42)), (0.5863014, rgb(0x362129)), (0.6958904, rgb(0x32191d)), (1.0000000, rgb(0x280809))] }];

const UPPER_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 151.0, w: 282.0, h: 365.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0x622443)), (0.0794521, rgb(0x5d2541)), (0.2301370, rgb(0x562843)), (0.3397260, rgb(0x4d283f)), (0.5863014, rgb(0x45232c)), (0.6958904, rgb(0x3e1c21)), (1.0000000, rgb(0x2f0809))] }];

const UPPER_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 151.0, w: 282.0, h: 365.0, from: (0.00000000, -0.41369863), to: (1.00000000, -0.41369863), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const UPPER: &[Prim] = &[Prim::Masked { prims: &[UPPER_L[0], Prim::Masked { prims: UPPER_R, mask: UPPER_MIX }], mask: UPPER_MASK }];

const LOWER_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 151.0, w: 270.0, h: 646.1, from: (0.00000000, 0.56492803), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0x1a0608)), (1.0000000, rgb(0x130506))] }];

const LOWER_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 151.0, w: 270.0, h: 646.1, from: (0.00000000, 0.56492803), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0x150607)), (1.0000000, rgb(0x100505))] }];

const LOWER_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 151.0, w: 270.0, h: 646.1, from: (0.00000000, -0.23370995), to: (1.00000000, -0.23370995), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const LOWER: &[Prim] = &[Prim::Masked { prims: &[LOWER_L[0], Prim::Masked { prims: LOWER_R, mask: LOWER_MIX }], mask: LOWER_MASK }];

const GROWN_SURFACE: &[Prim] = &[LOWER[0], UPPER[0]];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_material_keeps_non_shelf_motions_and_their_art() {
        let mut checked = 0;
        for (index, original) in STORE.iter().enumerate() {
            if let Prim::Motion { motion, .. } = original {
                if !matches!(motion.change, Change::Clip { .. }) {
                    assert_eq!(*original, SCENE[index]);
                    checked += 1;
                }
            }
        }
        assert!(checked > 0, "exercise independent margin opacity artwork");
    }

    #[test]
    fn material_pixels_match_source_holdouts_along_both_axes() {
        // Independent 11×11 native source medians, not gradient stops.
        // Sampling widely separated rows catches user-space coordinates
        // accidentally supplied to Ramp's bounding-box fractional axis.
        fn check(prims: &[Prim], samples: &[(usize, usize, [u8; 3])]) {
            let pixels = crate::screens::soft::composite(
                prims, &Era::Neomil.style().palette, 300, 800, 1.0,
            );
            for &(x, y, expected) in samples {
                let start = (y * 300 + x) * 4;
                assert_eq!(pixels[start + 3], 255, "opaque interior at {x},{y}");
                for channel in 0..3 {
                    assert!((pixels[start + channel] as i16 - expected[channel] as i16).abs() <= 2,
                        "source patch {x},{y}, channel {channel}: actual {}, expected {}",
                        pixels[start + channel], expected[channel]);
                }
            }
        }
        check(GROWN_SURFACE, &[
            (120, 160, [89, 38, 69]), (100, 255, [72, 42, 68]),
            (180, 375, [62, 33, 40]), (240, 505, [49, 11, 13]),
            (40, 520, [25, 6, 8]), (180, 605, [21, 5, 7]),
            (80, 755, [19, 5, 7]), (220, 765, [17, 5, 5]),
        ]);
        check(NAV_FIELD_OFF1, &[(57, 19, [49, 18, 20]), (57, 52, [48, 18, 19])]);
        check(NAV_FIELD_OFF3, &[(147, 18, [43, 12, 14])]);
        check(NAV_FIELD_OFF4, &[(177, 51, [41, 9, 11])]);
        check(NAV_FIELD_ON0, &[
            (67, 7, [196, 45, 46]), (67, 27, [195, 44, 44]),
            (67, 52, [197, 42, 43]), (172, 62, [197, 43, 43]),
        ]);
    }
}

const ORDINARY1: &[Prim] = &[fill_path(0.0, 151.0, FRAME_STD, alpha(rgb(0xff3c3e), 0.040)),
    Prim::At { x: 0.0, y: 0.0, prims: store_echo::IDLE_LEFT1 }];

const ORDINARY3: &[Prim] = &[fill_path(0.0, 151.0, FRAME_STD, alpha(rgb(0xff312f), 0.043)),
    Prim::At { x: 0.0, y: 0.0, prims: store_echo::IDLE_RIGHT3 }];

const ORDINARY4: &[Prim] = &[fill_rect(0.0, 151.0, 132.0, 462.0, alpha(rgb(0xff312e), 0.041)),
    Prim::At { x: 0.0, y: 0.0, prims: store_echo::IDLE_RIGHT4 }];

const CUT_MASK: &[Prim] = &[fill_rect(0.0, 151.0, 132.0, 646.1, WHITE)];

const GROWN_WITH_ECHO: &[Prim] = &[Prim::At { x: 0.0, y: 0.0, prims: GROWN_SURFACE },
    Prim::At { x: 0.0, y: 0.0, prims: store_echo::IDLE_DOWN }];
const GROWN_CUT: &[Prim] = &[Prim::Masked { prims: GROWN_WITH_ECHO, mask: CUT_MASK }];

const CARDS0: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: GROWN_WITH_ECHO },Prim::At { x: 769.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 1096.0, y: 0.0, prims: ORDINARY3 },Prim::At { x: 1425.0, y: 0.0, prims: ORDINARY4 }];

const CARDS1: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 769.0, y: 0.0, prims: GROWN_WITH_ECHO },Prim::At { x: 1096.0, y: 0.0, prims: ORDINARY3 },Prim::At { x: 1425.0, y: 0.0, prims: ORDINARY4 }];

const CARDS2: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 769.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 1096.0, y: 0.0, prims: GROWN_WITH_ECHO },Prim::At { x: 1425.0, y: 0.0, prims: ORDINARY4 }];

const CARDS3: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 769.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 1096.0, y: 0.0, prims: ORDINARY3 },Prim::At { x: 1425.0, y: 0.0, prims: GROWN_CUT }];

const OFF_MASK0: &[Prim] = &[fill_path(0.0, 0.0, NAV62, WHITE)];

const NAV_FIELD_OFF0_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (0.00000000, 5.45161290), stops: &[(0.0000000, rgb(0x341716)), (1.0000000, rgb(0x2b090c))] }];

const NAV_FIELD_OFF0_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (0.00000000, 5.45161290), stops: &[(0.0000000, rgb(0x301315)), (1.0000000, rgb(0x280809))] }];

const NAV_FIELD_OFF0_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_OFF0: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_OFF0_L[0], Prim::Masked { prims: NAV_FIELD_OFF0_R, mask: NAV_FIELD_OFF0_MIX }], mask: OFF_MASK0 }];

const ON_MASK0: &[Prim] = &[fill_path(0.0, 0.0, NAV67, WHITE)];

const NAV_FIELD_ON0_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc42d2c)), (1.0000000, rgb(0xc5292a))] }];

const NAV_FIELD_ON0_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc12e30)), (1.0000000, rgb(0xc52b2b))] }];

const NAV_FIELD_ON0_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_ON0: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_ON0_L[0], Prim::Masked { prims: NAV_FIELD_ON0_R, mask: NAV_FIELD_ON0_MIX }], mask: ON_MASK0 }];

const OFF_MASK1: &[Prim] = &[fill_path(0.0, 0.0, NAV62, WHITE)];

const NAV_FIELD_OFF1_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -1.12903226), to: (0.00000000, 4.32258065), stops: &[(0.0000000, rgb(0x341716)), (1.0000000, rgb(0x2b090c))] }];

const NAV_FIELD_OFF1_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -1.12903226), to: (0.00000000, 4.32258065), stops: &[(0.0000000, rgb(0x301315)), (1.0000000, rgb(0x280809))] }];

const NAV_FIELD_OFF1_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_OFF1: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_OFF1_L[0], Prim::Masked { prims: NAV_FIELD_OFF1_R, mask: NAV_FIELD_OFF1_MIX }], mask: OFF_MASK1 }];

const ON_MASK1: &[Prim] = &[fill_path(0.0, 0.0, NAV67, WHITE)];

const NAV_FIELD_ON1_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc42d2c)), (1.0000000, rgb(0xc5292a))] }];

const NAV_FIELD_ON1_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc12e30)), (1.0000000, rgb(0xc52b2b))] }];

const NAV_FIELD_ON1_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_ON1: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_ON1_L[0], Prim::Masked { prims: NAV_FIELD_ON1_R, mask: NAV_FIELD_ON1_MIX }], mask: ON_MASK1 }];

const OFF_MASK2: &[Prim] = &[fill_path(0.0, 0.0, NAV62, WHITE)];

const NAV_FIELD_OFF2_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -2.20967742), to: (0.00000000, 3.24193548), stops: &[(0.0000000, rgb(0x341716)), (1.0000000, rgb(0x2b090c))] }];

const NAV_FIELD_OFF2_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -2.20967742), to: (0.00000000, 3.24193548), stops: &[(0.0000000, rgb(0x301315)), (1.0000000, rgb(0x280809))] }];

const NAV_FIELD_OFF2_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_OFF2: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_OFF2_L[0], Prim::Masked { prims: NAV_FIELD_OFF2_R, mask: NAV_FIELD_OFF2_MIX }], mask: OFF_MASK2 }];

const ON_MASK2: &[Prim] = &[fill_path(0.0, 0.0, NAV67, WHITE)];

const NAV_FIELD_ON2_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc42d2c)), (1.0000000, rgb(0xc5292a))] }];

const NAV_FIELD_ON2_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc12e30)), (1.0000000, rgb(0xc52b2b))] }];

const NAV_FIELD_ON2_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_ON2: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_ON2_L[0], Prim::Masked { prims: NAV_FIELD_ON2_R, mask: NAV_FIELD_ON2_MIX }], mask: ON_MASK2 }];

const OFF_MASK3: &[Prim] = &[fill_path(0.0, 0.0, NAV62, WHITE)];

const NAV_FIELD_OFF3_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -3.29032258), to: (0.00000000, 2.16129032), stops: &[(0.0000000, rgb(0x341716)), (1.0000000, rgb(0x2b090c))] }];

const NAV_FIELD_OFF3_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -3.29032258), to: (0.00000000, 2.16129032), stops: &[(0.0000000, rgb(0x301315)), (1.0000000, rgb(0x280809))] }];

const NAV_FIELD_OFF3_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_OFF3: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_OFF3_L[0], Prim::Masked { prims: NAV_FIELD_OFF3_R, mask: NAV_FIELD_OFF3_MIX }], mask: OFF_MASK3 }];

const ON_MASK3: &[Prim] = &[fill_path(0.0, 0.0, NAV67, WHITE)];

const NAV_FIELD_ON3_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc42d2c)), (1.0000000, rgb(0xc5292a))] }];

const NAV_FIELD_ON3_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc12e30)), (1.0000000, rgb(0xc52b2b))] }];

const NAV_FIELD_ON3_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_ON3: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_ON3_L[0], Prim::Masked { prims: NAV_FIELD_ON3_R, mask: NAV_FIELD_ON3_MIX }], mask: ON_MASK3 }];

const OFF_MASK4: &[Prim] = &[fill_path(0.0, 0.0, NAV62, WHITE)];

const NAV_FIELD_OFF4_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -4.37096774), to: (0.00000000, 1.08064516), stops: &[(0.0000000, rgb(0x341716)), (1.0000000, rgb(0x2b090c))] }];

const NAV_FIELD_OFF4_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, -4.37096774), to: (0.00000000, 1.08064516), stops: &[(0.0000000, rgb(0x301315)), (1.0000000, rgb(0x280809))] }];

const NAV_FIELD_OFF4_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 62.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_OFF4: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_OFF4_L[0], Prim::Masked { prims: NAV_FIELD_OFF4_R, mask: NAV_FIELD_OFF4_MIX }], mask: OFF_MASK4 }];

const ON_MASK4: &[Prim] = &[fill_path(0.0, 0.0, NAV67, WHITE)];

const NAV_FIELD_ON4_L: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc42d2c)), (1.0000000, rgb(0xc5292a))] }];

const NAV_FIELD_ON4_R: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (0.00000000, 1.00000000), stops: &[(0.0000000, rgb(0xc12e30)), (1.0000000, rgb(0xc52b2b))] }];

const NAV_FIELD_ON4_MIX: &[Prim] = &[Prim::Ramp { x: 0.0, y: 0.0, w: 208.0, h: 67.0, from: (0.00000000, 0.00000000), to: (1.00000000, 0.00000000), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const NAV_FIELD_ON4: &[Prim] = &[Prim::Masked { prims: &[NAV_FIELD_ON4_L[0], Prim::Masked { prims: NAV_FIELD_ON4_R, mask: NAV_FIELD_ON4_MIX }], mask: ON_MASK4 }];

const NAVS0: &[Prim] = &[Prim::At { x: 153.0, y: 248.0, prims: NAV_FIELD_ON0 },Prim::At { x: 153.0, y: 318.0, prims: NAV_FIELD_OFF1 },Prim::At { x: 153.0, y: 385.0, prims: NAV_FIELD_OFF2 },Prim::At { x: 153.0, y: 452.0, prims: NAV_FIELD_OFF3 },Prim::At { x: 153.0, y: 519.0, prims: NAV_FIELD_OFF4 }];

const NAVS1: &[Prim] = &[Prim::At { x: 153.0, y: 248.0, prims: NAV_FIELD_OFF0 },Prim::At { x: 153.0, y: 318.0, prims: NAV_FIELD_ON1 },Prim::At { x: 153.0, y: 385.0, prims: NAV_FIELD_OFF2 },Prim::At { x: 153.0, y: 452.0, prims: NAV_FIELD_OFF3 },Prim::At { x: 153.0, y: 519.0, prims: NAV_FIELD_OFF4 }];

const NAVS2: &[Prim] = &[Prim::At { x: 153.0, y: 248.0, prims: NAV_FIELD_OFF0 },Prim::At { x: 153.0, y: 318.0, prims: NAV_FIELD_OFF1 },Prim::At { x: 153.0, y: 385.0, prims: NAV_FIELD_ON2 },Prim::At { x: 153.0, y: 452.0, prims: NAV_FIELD_OFF3 },Prim::At { x: 153.0, y: 519.0, prims: NAV_FIELD_OFF4 }];

const NAVS3: &[Prim] = &[Prim::At { x: 153.0, y: 248.0, prims: NAV_FIELD_OFF0 },Prim::At { x: 153.0, y: 318.0, prims: NAV_FIELD_OFF1 },Prim::At { x: 153.0, y: 385.0, prims: NAV_FIELD_OFF2 },Prim::At { x: 153.0, y: 452.0, prims: NAV_FIELD_ON3 },Prim::At { x: 153.0, y: 519.0, prims: NAV_FIELD_OFF4 }];

const NAVS4: &[Prim] = &[Prim::At { x: 153.0, y: 248.0, prims: NAV_FIELD_OFF0 },Prim::At { x: 153.0, y: 318.0, prims: NAV_FIELD_OFF1 },Prim::At { x: 153.0, y: 385.0, prims: NAV_FIELD_OFF2 },Prim::At { x: 153.0, y: 452.0, prims: NAV_FIELD_OFF3 },Prim::At { x: 153.0, y: 519.0, prims: NAV_FIELD_ON4 }];

const BACK00: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS0 }] }, Prim::Soft { prims: NAVS0 }];

const BACK01: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS0 }] }, Prim::Soft { prims: NAVS1 }];

const BACK02: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS0 }] }, Prim::Soft { prims: NAVS2 }];

const BACK03: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS0 }] }, Prim::Soft { prims: NAVS3 }];

const BACK04: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS0 }] }, Prim::Soft { prims: NAVS4 }];

const BACK10: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS1 }] }, Prim::Soft { prims: NAVS0 }];

const BACK11: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS1 }] }, Prim::Soft { prims: NAVS1 }];

const BACK12: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS1 }] }, Prim::Soft { prims: NAVS2 }];

const BACK13: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS1 }] }, Prim::Soft { prims: NAVS3 }];

const BACK14: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS1 }] }, Prim::Soft { prims: NAVS4 }];

const BACK20: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS2 }] }, Prim::Soft { prims: NAVS0 }];

const BACK21: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS2 }] }, Prim::Soft { prims: NAVS1 }];

const BACK22: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS2 }] }, Prim::Soft { prims: NAVS2 }];

const BACK23: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS2 }] }, Prim::Soft { prims: NAVS3 }];

const BACK24: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS2 }] }, Prim::Soft { prims: NAVS4 }];

const BACK30: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS3 }] }, Prim::Soft { prims: NAVS0 }];

const BACK31: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS3 }] }, Prim::Soft { prims: NAVS1 }];

const BACK32: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS3 }] }, Prim::Soft { prims: NAVS2 }];

const BACK33: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS3 }] }, Prim::Soft { prims: NAVS3 }];

const BACK34: &[Prim] = &[Prim::Soft { prims: STORE_GROUND }, Prim::Motion { motion: SHELF_OPEN, prims: &[Prim::Soft { prims: CARDS3 }] }, Prim::Soft { prims: NAVS4 }];

// The rest material is under the opaque feedback coat. Add a semantic
// echo immediately before the unchanged Dots leaf only for reference
// feedback drawings; custom palettes keep STORE_STATES byte-for-byte.
const fn with_echo<const N: usize, const M: usize>(source: &[Prim], at: usize,
    echo: &'static [Prim]) -> [Prim; M] {
    assert!(source.len() == N && M == N + 1 && at < N);
    assert!(matches!(source[at], Prim::Dots { .. }));
    let mut out = [source[0]; M];
    let mut i = 0;
    while i < M {
        out[i] = if i == at { Prim::At { x: 0.0, y: 0.0, prims: echo } }
            else { source[if i < at { i } else { i - 1 }] };
        i += 1;
    }
    out
}
const H1: &[Prim] = &with_echo::<{CARD1.len()}, {CARD1.len()+1}>(STORE_STATES[5].hover, 11, store_echo::ACTIVE_LEFT1_HOVER);
const P1: &[Prim] = &with_echo::<{CARD1.len()}, {CARD1.len()+1}>(STORE_STATES[5].pressed, 11, store_echo::ACTIVE_LEFT1_HELD);
const H3: &[Prim] = &with_echo::<{CARD3.len()}, {CARD3.len()+1}>(STORE_STATES[7].hover, 11, store_echo::ACTIVE_RIGHT3_HOVER);
const P3: &[Prim] = &with_echo::<{CARD3.len()}, {CARD3.len()+1}>(STORE_STATES[7].pressed, 11, store_echo::ACTIVE_RIGHT3_HELD);
// The cutoff card keeps its three outer leaves; its Dots are inside the
// recolored CARD_CUT at outer slot 1. Insert there, before the Dots.
const H4_CUT: &[Prim] = &with_echo::<{CARD_CUT.len()}, {CARD_CUT.len()+1}>(card_hover::CUT, 13, store_echo::ACTIVE_RIGHT4_HOVER);
const P4_CUT: &[Prim] = &with_echo::<{CARD_CUT.len()}, {CARD_CUT.len()+1}>(card_held::CUT, 13, store_echo::ACTIVE_RIGHT4_HELD);
const fn cut_state(source: &[Prim], cut: &'static [Prim]) -> [Prim; 3] {
    assert!(source.len() == 3);
    let mut out = [source[0], source[1], source[2]];
    match out[1] {
        Prim::At { x, y, .. } => out[1] = Prim::At { x, y, prims: cut },
        _ => panic!("cut card content wrapper"),
    }
    out
}
const H4: &[Prim] = &cut_state(STORE_STATES[8].hover, H4_CUT);
const P4: &[Prim] = &cut_state(STORE_STATES[8].pressed, P4_CUT);
const HG: &[Prim] = &with_echo::<{GROWN.len()}, {GROWN.len()+1}>(CARD_GROWN_HOVER, 31, store_echo::ACTIVE_DOWN_HOVER);
const PG: &[Prim] = &with_echo::<{GROWN.len()}, {GROWN.len()+1}>(CARD_GROWN_HELD, 31, store_echo::ACTIVE_DOWN_HELD);
const fn reference_states() -> [crate::style::PlateStates; 9] {
    let mut out = [STORE_STATES[0]; 9];
    let mut i = 0;
    while i < 9 { out[i] = STORE_STATES[i]; i += 1; }
    out[5].hover = H1; out[5].pressed = P1;
    out[6].hover = H1; out[6].pressed = P1;
    out[7].hover = H3; out[7].pressed = P3;
    out[8].hover = H4; out[8].pressed = P4;
    i = 5;
    while i < 9 { out[i].selected_hover = Some(HG); out[i].selected_pressed = Some(PG); i += 1; }
    out
}
const REFERENCE_STATES: &[crate::style::PlateStates] = &reference_states();

pub(super) const REFERENCE: StoreReference = StoreReference { states: Some(REFERENCE_STATES), scene: SCENE, backdrops: &[StoreBackdrop { category: 0, card: 0, prims: BACK00 },StoreBackdrop { category: 1, card: 0, prims: BACK01 },StoreBackdrop { category: 2, card: 0, prims: BACK02 },StoreBackdrop { category: 3, card: 0, prims: BACK03 },StoreBackdrop { category: 4, card: 0, prims: BACK04 },StoreBackdrop { category: 0, card: 1, prims: BACK10 },StoreBackdrop { category: 1, card: 1, prims: BACK11 },StoreBackdrop { category: 2, card: 1, prims: BACK12 },StoreBackdrop { category: 3, card: 1, prims: BACK13 },StoreBackdrop { category: 4, card: 1, prims: BACK14 },StoreBackdrop { category: 0, card: 2, prims: BACK20 },StoreBackdrop { category: 1, card: 2, prims: BACK21 },StoreBackdrop { category: 2, card: 2, prims: BACK22 },StoreBackdrop { category: 3, card: 2, prims: BACK23 },StoreBackdrop { category: 4, card: 2, prims: BACK24 },StoreBackdrop { category: 0, card: 3, prims: BACK30 },StoreBackdrop { category: 1, card: 3, prims: BACK31 },StoreBackdrop { category: 2, card: 3, prims: BACK32 },StoreBackdrop { category: 3, card: 3, prims: BACK33 },StoreBackdrop { category: 4, card: 3, prims: BACK34 }] };

#[cfg(test)]
mod echo_tests {
    use super::*;

    #[test]
    fn reference_echoes_precede_unchanged_cells_in_every_feedback_state() {
        assert_eq!(REFERENCE_STATES.len(), STORE_STATES.len());
        assert_eq!(&REFERENCE_STATES[..5], &STORE_STATES[..5]);
        for (card, dot) in [(0, 11), (1, 11), (2, 11)] {
            let ordinary = STORE_STATES[5 + card];
            let reference = REFERENCE_STATES[5 + card];
            for (base, with_echo) in [(ordinary.hover, reference.hover),
                (ordinary.pressed, reference.pressed)] {
                assert_eq!(with_echo.len(), base.len() + 1);
                assert_eq!(&with_echo[..dot], &base[..dot]);
                assert!(matches!(with_echo[dot], Prim::At { prims, .. } if prims.len() == 25));
                assert_eq!(&with_echo[dot + 1..], &base[dot..]);
                let Prim::Dots { rows, cell, pitch, .. } = with_echo[dot + 1] else { panic!("primary cells moved") };
                assert_eq!((rows, cell, pitch), (QR, 3.0, 3.6667));
            }
        }
        let ordinary = STORE_STATES[8];
        let reference = REFERENCE_STATES[8];
        for (base, with_echo) in [(ordinary.hover, reference.hover),
            (ordinary.pressed, reference.pressed)] {
            assert_eq!(with_echo.len(), base.len());
            assert_eq!(with_echo[0], base[0], "outer coat stays intact");
            assert_eq!(with_echo[2], base[2], "open edge stays intact");
            let Prim::At { x: bx, y: by, prims: inner } = base[1] else { panic!("cut content") };
            let Prim::At { x: ax, y: ay, prims: echoed } = with_echo[1] else { panic!("echoed cut content") };
            assert_eq!((ax, ay), (bx, by));
            assert_eq!(&echoed[..13], &inner[..13]);
            assert!(matches!(echoed[13], Prim::At { prims, .. } if prims.len() == 25));
            assert_eq!(&echoed[14..], &inner[13..]);
        }
        for card in 0..4 {
            let ordinary = STORE_STATES[5 + card];
            let reference = REFERENCE_STATES[5 + card];
            for (base, with_echo) in [
                (ordinary.selected_hover.unwrap(), reference.selected_hover.unwrap()),
                (ordinary.selected_pressed.unwrap(), reference.selected_pressed.unwrap()),
            ] {
                assert_eq!(with_echo.len(), base.len() + 1);
                assert_eq!(&with_echo[..31], &base[..31]);
                assert!(matches!(with_echo[31], Prim::At { prims, .. } if prims.len() == 25));
                assert_eq!(&with_echo[32..], &base[31..]);
            }
        }
        assert!(matches!(CARDS3[3], Prim::At { prims, .. } if matches!(prims[0], Prim::Masked { .. })));
        let Prim::At { prims: cells, .. } = store_echo::ACTIVE_DOWN_HELD[0] else { panic!("selected echo cell") };
        let Prim::Motion { prims: inked, .. } = cells[0] else { panic!("selected echo band") };
        assert!(matches!(inked[0], Prim::Rect { fill: Some(Ink::Fixed(c)), .. } if c == rgb(0x4a0f10)));
    }
}
