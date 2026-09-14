//! Local dashboard surface measurements from img-07-dashboard.png.
//! Badge gradients/scanlines are independent of their foreground printing.
//! GO HOME retains its existing stripe phase, modulation, contour and motion.
//! Full-bounds layers preserve opacity under fractional scene scaling.
use crate::palette::rgb;
use crate::style::{Ink, Prim, Seg};
use super::{fill_path, fill_rect};

const fn stripes<const N: usize>(x: f32, first: f32, w: f32, pitch: f32, height: f32) -> [Prim; N] {
    let mut rows = [fill_rect(0.0, 0.0, 0.0, 0.0, Ink::Fixed(rgb(0xffffff))); N];
    let mut i = 0;
    while i < N {
        rows[i] = fill_rect(x, first + i as f32 * pitch, w, height, Ink::Fixed(rgb(0xffffff)));
        i += 1;
    }
    rows
}

const BADGE_CUSTOMER_COOL_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x4a373f)), (1.0, rgb(0x493845))];
const BADGE_CUSTOMER_COOL_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x472a2e)), (1.0, rgb(0x482f33))];
const BADGE_CUSTOMER_COOL_MASK_1: &[Prim] = &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_CUSTOMER_COOL: &[Prim] = &[
    Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_CUSTOMER_COOL_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_CUSTOMER_COOL_1 }], mask: BADGE_CUSTOMER_COOL_MASK_1 },
];
const BADGE_CUSTOMER_WARM_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x53343c)), (1.0, rgb(0x523541))];
const BADGE_CUSTOMER_WARM_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x50272a)), (1.0, rgb(0x502c30))];
const BADGE_CUSTOMER_WARM_MASK_1: &[Prim] = &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_CUSTOMER_WARM: &[Prim] = &[
    Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_CUSTOMER_WARM_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_CUSTOMER_WARM_1 }], mask: BADGE_CUSTOMER_WARM_MASK_1 },
];
const BADGE_CUSTOMER_STRIPES: &[Prim] = &stripes::<30>(119.5833, 102.88996733, 56.6667, 1.984934, 1.17);
const BADGE_CUSTOMER: Prim = Prim::Masked {
    prims: &[Prim::At { x: 0.0, y: 0.0, prims: BADGE_CUSTOMER_COOL }, Prim::Masked { prims: BADGE_CUSTOMER_WARM, mask: BADGE_CUSTOMER_STRIPES }],
    mask: &[Prim::Path { x: 119.5833, y: 104.5833, segs: &[Seg::Line(176.25, 104.5833), Seg::Line(176.25, 161.25), Seg::Line(135.0, 161.25), Seg::Line(119.5833, 145.8333)], close: true, fill: Some(Ink::Fixed(rgb(0xffffff))), stroke: None, width: 1.0 }],
};
const BADGE_T1_COOL_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x39234a)), (1.0, rgb(0x382249))];
const BADGE_T1_COOL_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x382448)), (1.0, rgb(0x352549))];
const BADGE_T1_COOL_MASK_1: &[Prim] = &[Prim::Ramp { x: 1132.6667, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T1_COOL: &[Prim] = &[
    Prim::Ramp { x: 1132.6667, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T1_COOL_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1132.6667, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T1_COOL_1 }], mask: BADGE_T1_COOL_MASK_1 },
];
const BADGE_T1_WARM_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x421f46)), (1.0, rgb(0x411e45))];
const BADGE_T1_WARM_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x412045)), (1.0, rgb(0x3e2245))];
const BADGE_T1_WARM_MASK_1: &[Prim] = &[Prim::Ramp { x: 1132.6667, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T1_WARM: &[Prim] = &[
    Prim::Ramp { x: 1132.6667, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T1_WARM_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1132.6667, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T1_WARM_1 }], mask: BADGE_T1_WARM_MASK_1 },
];
const BADGE_T1_STRIPES: &[Prim] = &stripes::<29>(1132.6667, 102.88996733, 56.6667, 1.984934, 1.17);
const BADGE_T1: Prim = Prim::Masked {
    prims: &[Prim::At { x: 0.0, y: 0.0, prims: BADGE_T1_COOL }, Prim::Masked { prims: BADGE_T1_WARM, mask: BADGE_T1_STRIPES }],
    mask: &[Prim::Path { x: 1132.6667, y: 104.1667, segs: &[Seg::Line(1189.3334, 104.1667), Seg::Line(1189.3334, 160.4167), Seg::Line(1148.0834, 160.4167), Seg::Line(1132.6667, 145.0)], close: true, fill: Some(Ink::Fixed(rgb(0xffffff))), stroke: None, width: 1.0 }],
};
const BADGE_T2_COOL_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x782a41)), (1.0, rgb(0x762a41))];
const BADGE_T2_COOL_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x732b41)), (1.0, rgb(0x742c40))];
const BADGE_T2_COOL_MASK_1: &[Prim] = &[Prim::Ramp { x: 1193.5, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T2_COOL: &[Prim] = &[
    Prim::Ramp { x: 1193.5, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T2_COOL_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1193.5, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T2_COOL_1 }], mask: BADGE_T2_COOL_MASK_1 },
];
const BADGE_T2_WARM_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x88233a)), (1.0, rgb(0x86243a))];
const BADGE_T2_WARM_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x83253b)), (1.0, rgb(0x85253a))];
const BADGE_T2_WARM_MASK_1: &[Prim] = &[Prim::Ramp { x: 1193.5, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T2_WARM: &[Prim] = &[
    Prim::Ramp { x: 1193.5, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T2_WARM_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1193.5, y: 104.1667, w: 56.6667, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T2_WARM_1 }], mask: BADGE_T2_WARM_MASK_1 },
];
const BADGE_T2_STRIPES: &[Prim] = &stripes::<29>(1193.5, 102.88996733, 56.6667, 1.984934, 1.17);
const BADGE_T2: Prim = Prim::Masked {
    prims: &[Prim::At { x: 0.0, y: 0.0, prims: BADGE_T2_COOL }, Prim::Masked { prims: BADGE_T2_WARM, mask: BADGE_T2_STRIPES }],
    mask: &[Prim::Path { x: 1193.5, y: 104.1667, segs: &[Seg::Line(1250.1667, 104.1667), Seg::Line(1250.1667, 160.4167), Seg::Line(1208.9167, 160.4167), Seg::Line(1193.5, 145.0)], close: true, fill: Some(Ink::Fixed(rgb(0xffffff))), stroke: None, width: 1.0 }],
};
const BADGE_T3_COOL_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x382449)), (1.0, rgb(0x352449))];
const BADGE_T3_COOL_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x342648)), (1.0, rgb(0x322646))];
const BADGE_T3_COOL_MASK_1: &[Prim] = &[Prim::Ramp { x: 1254.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T3_COOL: &[Prim] = &[
    Prim::Ramp { x: 1254.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T3_COOL_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1254.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T3_COOL_1 }], mask: BADGE_T3_COOL_MASK_1 },
];
const BADGE_T3_WARM_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x412045)), (1.0, rgb(0x3e2145))];
const BADGE_T3_WARM_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x3d2244)), (1.0, rgb(0x3b2343))];
const BADGE_T3_WARM_MASK_1: &[Prim] = &[Prim::Ramp { x: 1254.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T3_WARM: &[Prim] = &[
    Prim::Ramp { x: 1254.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T3_WARM_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1254.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T3_WARM_1 }], mask: BADGE_T3_WARM_MASK_1 },
];
const BADGE_T3_STRIPES: &[Prim] = &stripes::<29>(1254.0, 102.88996733, 56.25, 1.984934, 1.17);
const BADGE_T3: Prim = Prim::Masked {
    prims: &[Prim::At { x: 0.0, y: 0.0, prims: BADGE_T3_COOL }, Prim::Masked { prims: BADGE_T3_WARM, mask: BADGE_T3_STRIPES }],
    mask: &[Prim::Path { x: 1254.0, y: 104.1667, segs: &[Seg::Line(1310.25, 104.1667), Seg::Line(1310.25, 160.4167), Seg::Line(1269.4167, 160.4167), Seg::Line(1254.0, 145.0)], close: true, fill: Some(Ink::Fixed(rgb(0xffffff))), stroke: None, width: 1.0 }],
};
const BADGE_T4_COOL_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x362549)), (1.0, rgb(0x342648))];
const BADGE_T4_COOL_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x332746)), (1.0, rgb(0x322643))];
const BADGE_T4_COOL_MASK_1: &[Prim] = &[Prim::Ramp { x: 1315.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T4_COOL: &[Prim] = &[
    Prim::Ramp { x: 1315.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T4_COOL_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1315.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T4_COOL_1 }], mask: BADGE_T4_COOL_MASK_1 },
];
const BADGE_T4_WARM_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x3f2245)), (1.0, rgb(0x3d2345))];
const BADGE_T4_WARM_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x3c2343)), (1.0, rgb(0x3b233f))];
const BADGE_T4_WARM_MASK_1: &[Prim] = &[Prim::Ramp { x: 1315.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const BADGE_T4_WARM: &[Prim] = &[
    Prim::Ramp { x: 1315.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T4_WARM_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1315.0, y: 104.1667, w: 56.25, h: 56.25, from: (0.0, 0.0), to: (1.0, 0.0), stops: BADGE_T4_WARM_1 }], mask: BADGE_T4_WARM_MASK_1 },
];
const BADGE_T4_STRIPES: &[Prim] = &stripes::<29>(1315.0, 102.88996733, 56.25, 1.984934, 1.17);
const BADGE_T4: Prim = Prim::Masked {
    prims: &[Prim::At { x: 0.0, y: 0.0, prims: BADGE_T4_COOL }, Prim::Masked { prims: BADGE_T4_WARM, mask: BADGE_T4_STRIPES }],
    mask: &[Prim::Path { x: 1315.0, y: 104.1667, segs: &[Seg::Line(1371.25, 104.1667), Seg::Line(1371.25, 160.4167), Seg::Line(1330.4167, 160.4167), Seg::Line(1315.0, 145.0)], close: true, fill: Some(Ink::Fixed(rgb(0xffffff))), stroke: None, width: 1.0 }],
};
pub const BADGES: &[Prim] = &[BADGE_CUSTOMER, BADGE_T1, BADGE_T2, BADGE_T3, BADGE_T4];

const GO_HOME_COOL_FIELD_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x2c2435)), (0.47058824, rgb(0x29222f)), (1.0, rgb(0x231b22))];
const GO_HOME_COOL_FIELD_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x2c212c)), (0.47058824, rgb(0x271d24)), (1.0, rgb(0x211519))];
const GO_HOME_COOL_FIELD_MASK_1: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (0.08144796, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD_2: &[(f32, iced::Color)] = &[(0.0, rgb(0x29191d)), (0.47058824, rgb(0x231216)), (1.0, rgb(0x20090a))];
const GO_HOME_COOL_FIELD_MASK_2: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.08144796, rgb(0x000000)), (0.19457014, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD_3: &[(f32, iced::Color)] = &[(0.0, rgb(0x270c0f)), (0.47058824, rgb(0x220a0b)), (1.0, rgb(0x210a09))];
const GO_HOME_COOL_FIELD_MASK_3: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.19457014, rgb(0x000000)), (0.30769231, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD_4: &[(f32, iced::Color)] = &[(0.0, rgb(0x220a0a)), (0.47058824, rgb(0x210a0a)), (1.0, rgb(0x1b0809))];
const GO_HOME_COOL_FIELD_MASK_4: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.30769231, rgb(0x000000)), (0.51131222, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD_5: &[(f32, iced::Color)] = &[(0.0, rgb(0x220a0a)), (0.47058824, rgb(0x1d0909)), (1.0, rgb(0x1c0908))];
const GO_HOME_COOL_FIELD_MASK_5: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.51131222, rgb(0x000000)), (0.69230769, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD_6: &[(f32, iced::Color)] = &[(0.0, rgb(0x1d090a)), (0.47058824, rgb(0x1c0908)), (1.0, rgb(0x1a0909))];
const GO_HOME_COOL_FIELD_MASK_6: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.69230769, rgb(0x000000)), (0.87330317, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD_7: &[(f32, iced::Color)] = &[(0.0, rgb(0x1d0909)), (0.47058824, rgb(0x1b0909)), (1.0, rgb(0x160808))];
const GO_HOME_COOL_FIELD_MASK_7: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.87330317, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const GO_HOME_COOL_FIELD: &[Prim] = &[
    Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_1 }], mask: GO_HOME_COOL_FIELD_MASK_1 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_2 }], mask: GO_HOME_COOL_FIELD_MASK_2 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_3 }], mask: GO_HOME_COOL_FIELD_MASK_3 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_4 }], mask: GO_HOME_COOL_FIELD_MASK_4 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_5 }], mask: GO_HOME_COOL_FIELD_MASK_5 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_6 }], mask: GO_HOME_COOL_FIELD_MASK_6 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_COOL_FIELD_7 }], mask: GO_HOME_COOL_FIELD_MASK_7 },
];
const GO_HOME_WARM_FIELD_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x322031)), (0.47058824, rgb(0x2f1e2b)), (1.0, rgb(0x29171e))];
const GO_HOME_WARM_FIELD_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x321d28)), (0.47058824, rgb(0x2d1920)), (1.0, rgb(0x271115))];
const GO_HOME_WARM_FIELD_MASK_1: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.0, rgb(0x000000)), (0.08144796, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD_2: &[(f32, iced::Color)] = &[(0.0, rgb(0x2f1519)), (0.47058824, rgb(0x290e12)), (1.0, rgb(0x260506))];
const GO_HOME_WARM_FIELD_MASK_2: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.08144796, rgb(0x000000)), (0.19457014, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD_3: &[(f32, iced::Color)] = &[(0.0, rgb(0x2d080b)), (0.47058824, rgb(0x280607)), (1.0, rgb(0x270605))];
const GO_HOME_WARM_FIELD_MASK_3: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.19457014, rgb(0x000000)), (0.30769231, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD_4: &[(f32, iced::Color)] = &[(0.0, rgb(0x280606)), (0.47058824, rgb(0x270606)), (1.0, rgb(0x210405))];
const GO_HOME_WARM_FIELD_MASK_4: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.30769231, rgb(0x000000)), (0.51131222, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD_5: &[(f32, iced::Color)] = &[(0.0, rgb(0x280606)), (0.47058824, rgb(0x230505)), (1.0, rgb(0x220504))];
const GO_HOME_WARM_FIELD_MASK_5: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.51131222, rgb(0x000000)), (0.69230769, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD_6: &[(f32, iced::Color)] = &[(0.0, rgb(0x230506)), (0.47058824, rgb(0x220504)), (1.0, rgb(0x200505))];
const GO_HOME_WARM_FIELD_MASK_6: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.69230769, rgb(0x000000)), (0.87330317, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD_7: &[(f32, iced::Color)] = &[(0.0, rgb(0x230505)), (0.47058824, rgb(0x210505)), (1.0, rgb(0x1c0404))];
const GO_HOME_WARM_FIELD_MASK_7: &[Prim] = &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: &[(0.87330317, rgb(0x000000)), (1.0, rgb(0xffffff))] }];
const GO_HOME_WARM_FIELD: &[Prim] = &[
    Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_1 }], mask: GO_HOME_WARM_FIELD_MASK_1 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_2 }], mask: GO_HOME_WARM_FIELD_MASK_2 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_3 }], mask: GO_HOME_WARM_FIELD_MASK_3 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_4 }], mask: GO_HOME_WARM_FIELD_MASK_4 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_5 }], mask: GO_HOME_WARM_FIELD_MASK_5 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_6 }], mask: GO_HOME_WARM_FIELD_MASK_6 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1128.0, y: 314.0, w: 238.0, h: 442.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: GO_HOME_WARM_FIELD_7 }], mask: GO_HOME_WARM_FIELD_MASK_7 },
];
const PANEL_STRIPES: &[Prim] = &stripes::<224>(1128.0, 313.180711, 238.0, 1.984934, 0.992467);
pub const PANEL_MATERIAL: &[Prim] = &[Prim::Masked {
    prims: &[Prim::At { x: 0.0, y: 0.0, prims: GO_HOME_COOL_FIELD }, Prim::Masked { prims: GO_HOME_WARM_FIELD, mask: PANEL_STRIPES }],
    mask: &[fill_path(1128.0, 314.0, super::PANEL, Ink::Fixed(rgb(0xffffff)))],
}];
