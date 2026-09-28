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
const fn bare_card<const N: usize>(source: &[Prim], grown: bool) -> [Prim; N] {
    assert!(source.len() == N);
    let mut out = [source[0]; N]; let mut i = 0;
    while i < N { out[i] = source[i]; i += 1; }
    out[0] = clear_fill(out[0]);
    if grown { out[1] = Prim::At { x: 0.0, y: 0.0, prims: &[] }; }
    out
}
const C1: &[Prim] = &bare_card::<{ CARD1.len() }>(CARD1, false);
const C3: &[Prim] = &bare_card::<{ CARD3.len() }>(CARD3, false);
const C4: &[Prim] = &bare_card::<{ CARD4.len() }>(CARD4, false);
const CG: &[Prim] = &bare_card::<{ GROWN.len() }>(GROWN, true);
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
            Prim::Motion { motion, .. } => Prim::Motion { motion, prims: SHELF_REFERENCE },
            other => other,
        };
        i += 1;
    }
    out
};
const SHELF_OPEN: Motion = {
    let mut i = 0; let mut found = None;
    while i < STORE.len() {
        if let Prim::Motion { motion, .. } = STORE[i] {
            assert!(found.is_none()); found = Some(motion);
        }
        i += 1;
    }
    match found { Some(motion) => motion, None => panic!("missing shelf motion") }
};
const UPPER_MASK: &[Prim] = &[fill_path(0.0, 151.0, &[
    Seg::Line(269.0, 151.0), Seg::Line(282.0, 164.0), Seg::Line(282.0, 270.0),
    Seg::Line(270.0, 282.0), Seg::Line(270.0, 492.0), Seg::Line(246.0, 516.0), Seg::Line(0.0, 516.0),
], WHITE)];
const LOWER_MASK: &[Prim] = &[fill_path(0.0, 151.0, FRAME_SEL, WHITE)];


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

const ORDINARY1: &[Prim] = &[fill_path(0.0, 151.0, FRAME_STD, alpha(rgb(0xff3c3e), 0.040))];

const ORDINARY3: &[Prim] = &[fill_path(0.0, 151.0, FRAME_STD, alpha(rgb(0xff312f), 0.043))];

const ORDINARY4: &[Prim] = &[fill_rect(0.0, 151.0, 132.0, 462.0, alpha(rgb(0xff312e), 0.041))];

const CUT_MASK: &[Prim] = &[fill_rect(0.0, 151.0, 132.0, 646.1, WHITE)];

const GROWN_CUT: &[Prim] = &[Prim::Masked { prims: GROWN_SURFACE, mask: CUT_MASK }];

const CARDS0: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: GROWN_SURFACE },Prim::At { x: 769.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 1096.0, y: 0.0, prims: ORDINARY3 },Prim::At { x: 1425.0, y: 0.0, prims: ORDINARY4 }];

const CARDS1: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 769.0, y: 0.0, prims: GROWN_SURFACE },Prim::At { x: 1096.0, y: 0.0, prims: ORDINARY3 },Prim::At { x: 1425.0, y: 0.0, prims: ORDINARY4 }];

const CARDS2: &[Prim] = &[Prim::At { x: 437.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 769.0, y: 0.0, prims: ORDINARY1 },Prim::At { x: 1096.0, y: 0.0, prims: GROWN_SURFACE },Prim::At { x: 1425.0, y: 0.0, prims: ORDINARY4 }];

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

pub(super) const REFERENCE: StoreReference = StoreReference { scene: SCENE, backdrops: &[StoreBackdrop { category: 0, card: 0, prims: BACK00 },StoreBackdrop { category: 1, card: 0, prims: BACK01 },StoreBackdrop { category: 2, card: 0, prims: BACK02 },StoreBackdrop { category: 3, card: 0, prims: BACK03 },StoreBackdrop { category: 4, card: 0, prims: BACK04 },StoreBackdrop { category: 0, card: 1, prims: BACK10 },StoreBackdrop { category: 1, card: 1, prims: BACK11 },StoreBackdrop { category: 2, card: 1, prims: BACK12 },StoreBackdrop { category: 3, card: 1, prims: BACK13 },StoreBackdrop { category: 4, card: 1, prims: BACK14 },StoreBackdrop { category: 0, card: 2, prims: BACK20 },StoreBackdrop { category: 1, card: 2, prims: BACK21 },StoreBackdrop { category: 2, card: 2, prims: BACK22 },StoreBackdrop { category: 3, card: 2, prims: BACK23 },StoreBackdrop { category: 4, card: 2, prims: BACK24 },StoreBackdrop { category: 0, card: 3, prims: BACK30 },StoreBackdrop { category: 1, card: 3, prims: BACK31 },StoreBackdrop { category: 2, card: 3, prims: BACK32 },StoreBackdrop { category: 3, card: 3, prims: BACK33 },StoreBackdrop { category: 4, card: 3, prims: BACK34 }] };
