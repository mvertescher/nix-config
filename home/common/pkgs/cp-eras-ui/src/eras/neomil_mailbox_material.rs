//! Mailbox-specific surface measurements from img-08-main.png.
//! Read docs/neomil/mailbox-material.md for source masks and validation.
//! Compose these slices inside Prim::Soft; PANEL also needs message-open.
use crate::palette::rgb;
use crate::style::{fill_path, fill_rect, Ink, Prim, Seg};

/// The shared source background matches the measured dashboard composite.
/// Reuse its immutable drawing without changing any other screen.
pub const GROUND: &[Prim] = super::dashboard_ground::BACKGROUND;

const BLEND: &[(f32, iced::Color)] = &[(0.0, rgb(0x000000)), (1.0, rgb(0xffffff))];

const SCAN_CYCLE: &[(f32, iced::Color)] = &[(0.0, rgb(0x000000)), (0.0625, rgb(0x0a0a0a)), (0.125, rgb(0x252525)), (0.1875, rgb(0x4f4f4f)), (0.25, rgb(0x7f7f7f)), (0.3125, rgb(0xb0b0b0)), (0.375, rgb(0xdadada)), (0.4375, rgb(0xf5f5f5)), (0.5, rgb(0xffffff)), (0.5625, rgb(0xf5f5f5)), (0.625, rgb(0xdadada)), (0.6875, rgb(0xb0b0b0)), (0.75, rgb(0x808080)), (0.8125, rgb(0x4f4f4f)), (0.875, rgb(0x252525)), (0.9375, rgb(0x0a0a0a)), (1.0, rgb(0x000000))];

const fn scans<const N: usize>(x: f32, y: f32, w: f32, pitch: f32) -> [Prim; N] {
    let mut rows = [fill_rect(0.0, 0.0, 0.0, 0.0, Ink::Fixed(rgb(0x000000))); N];
    let mut i = 0;
    while i < N {
        rows[i] = Prim::Ramp { x, y: y + i as f32 * pitch, w, h: pitch,
            from: (0.0, 0.0), to: (0.0, 1.0), stops: SCAN_CYCLE };
        i += 1;
    }
    rows
}

const MAIL_PANEL_LOW_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x24283a)), (0.16782247, rgb(0x23293e)), (0.37586685, rgb(0x20273d)), (0.58391123, rgb(0x242436)), (0.79195562, rgb(0x33202a)), (1.0, rgb(0x3a191b))];
const MAIL_PANEL_LOW_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x23242f)), (0.16782247, rgb(0x212635)), (0.37586685, rgb(0x1f2534)), (0.58391123, rgb(0x20212d)), (0.79195562, rgb(0x2f1c22)), (1.0, rgb(0x391314))];
const MAIL_PANEL_LOW_ROW_2: &[(f32, iced::Color)] = &[(0.0, rgb(0x231e24)), (0.16782247, rgb(0x21212a)), (0.37586685, rgb(0x1e202a)), (0.58391123, rgb(0x1e1c24)), (0.79195562, rgb(0x2a161a)), (1.0, rgb(0x370c0c))];
const MAIL_PANEL_LOW_ROW_3: &[(f32, iced::Color)] = &[(0.0, rgb(0x21151a)), (0.16782247, rgb(0x211a1f)), (0.37586685, rgb(0x1e191f)), (0.58391123, rgb(0x18151a)), (0.79195562, rgb(0x220d10)), (1.0, rgb(0x360b0a))];
const MAIL_PANEL_LOW_ROW_4: &[(f32, iced::Color)] = &[(0.0, rgb(0x230e12)), (0.16782247, rgb(0x1f1013)), (0.37586685, rgb(0x1c1114)), (0.58391123, rgb(0x1a0b0f)), (0.79195562, rgb(0x1b0707)), (1.0, rgb(0x290b0b))];
const MAIL_PANEL_LOW_ROW_5: &[(f32, iced::Color)] = &[(0.0, rgb(0x22090c)), (0.16782247, rgb(0x21090b)), (0.37586685, rgb(0x1e080a)), (0.58391123, rgb(0x1a0808)), (0.79195562, rgb(0x180807)), (1.0, rgb(0x1d0808))];
const MAIL_PANEL_LOW_ROW_6: &[(f32, iced::Color)] = &[(0.0, rgb(0x200809)), (0.16782247, rgb(0x1e0709)), (0.37586685, rgb(0x1b0709)), (0.58391123, rgb(0x190808)), (0.79195562, rgb(0x190808)), (1.0, rgb(0x180707))];
const MAIL_PANEL_LOW_ROW_7: &[(f32, iced::Color)] = &[(0.0, rgb(0x1e070a)), (0.16782247, rgb(0x1c0809)), (0.37586685, rgb(0x190708)), (0.58391123, rgb(0x180707)), (0.79195562, rgb(0x190708)), (1.0, rgb(0x190707))];
const MAIL_PANEL_LOW_ROW_8: &[(f32, iced::Color)] = &[(0.0, rgb(0x1c0809)), (0.16782247, rgb(0x1b0809)), (0.37586685, rgb(0x190708)), (0.58391123, rgb(0x190807)), (0.79195562, rgb(0x190808)), (1.0, rgb(0x180708))];
const MAIL_PANEL_LOW: &[Prim] = &[
    Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_1 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (0.0, 0.07235142), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_2 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.07235142), to: (0.0, 0.1498708), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_3 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.1498708), to: (0.0, 0.22739018), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_4 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.22739018), to: (0.0, 0.33074935), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_5 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.33074935), to: (0.0, 0.43410853), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_6 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.43410853), to: (0.0, 0.64082687), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_7 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.64082687), to: (0.0, 0.84754522), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_LOW_ROW_8 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.84754522), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_PANEL_HIGH_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x282839)), (0.16782247, rgb(0x27283d)), (0.37586685, rgb(0x24263c)), (0.58391123, rgb(0x282335)), (0.79195562, rgb(0x372029)), (1.0, rgb(0x3e191a))];
const MAIL_PANEL_HIGH_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x27232e)), (0.16782247, rgb(0x252534)), (0.37586685, rgb(0x232434)), (0.58391123, rgb(0x24202c)), (0.79195562, rgb(0x331b21)), (1.0, rgb(0x3d1213))];
const MAIL_PANEL_HIGH_ROW_2: &[(f32, iced::Color)] = &[(0.0, rgb(0x271d24)), (0.16782247, rgb(0x252029)), (0.37586685, rgb(0x221f29)), (0.58391123, rgb(0x221c23)), (0.79195562, rgb(0x2e1519)), (1.0, rgb(0x3b0b0b))];
const MAIL_PANEL_HIGH_ROW_3: &[(f32, iced::Color)] = &[(0.0, rgb(0x251419)), (0.16782247, rgb(0x25191e)), (0.37586685, rgb(0x22191e)), (0.58391123, rgb(0x1c1419)), (0.79195562, rgb(0x260d0f)), (1.0, rgb(0x3a0a09))];
const MAIL_PANEL_HIGH_ROW_4: &[(f32, iced::Color)] = &[(0.0, rgb(0x270e11)), (0.16782247, rgb(0x230f13)), (0.37586685, rgb(0x211013)), (0.58391123, rgb(0x1e0a0e)), (0.79195562, rgb(0x1f0606)), (1.0, rgb(0x2d0b0a))];
const MAIL_PANEL_HIGH_ROW_5: &[(f32, iced::Color)] = &[(0.0, rgb(0x27090b)), (0.16782247, rgb(0x25080a)), (0.37586685, rgb(0x220809)), (0.58391123, rgb(0x1e0708)), (0.79195562, rgb(0x1c0707)), (1.0, rgb(0x210708))];
const MAIL_PANEL_HIGH_ROW_6: &[(f32, iced::Color)] = &[(0.0, rgb(0x240709)), (0.16782247, rgb(0x220708)), (0.37586685, rgb(0x1f0708)), (0.58391123, rgb(0x1d0707)), (0.79195562, rgb(0x1d0707)), (1.0, rgb(0x1c0607))];
const MAIL_PANEL_HIGH_ROW_7: &[(f32, iced::Color)] = &[(0.0, rgb(0x220709)), (0.16782247, rgb(0x200708)), (0.37586685, rgb(0x1d0608)), (0.58391123, rgb(0x1d0707)), (0.79195562, rgb(0x1d0707)), (1.0, rgb(0x1d0707))];
const MAIL_PANEL_HIGH_ROW_8: &[(f32, iced::Color)] = &[(0.0, rgb(0x200708)), (0.16782247, rgb(0x1f0708)), (0.37586685, rgb(0x1d0607)), (0.58391123, rgb(0x1d0707)), (0.79195562, rgb(0x1d0707)), (1.0, rgb(0x1d0707))];
const MAIL_PANEL_HIGH: &[Prim] = &[
    Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_1 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (0.0, 0.07235142), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_2 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.07235142), to: (0.0, 0.1498708), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_3 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.1498708), to: (0.0, 0.22739018), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_4 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.22739018), to: (0.0, 0.33074935), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_5 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.33074935), to: (0.0, 0.43410853), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_6 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.43410853), to: (0.0, 0.64082687), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_7 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.64082687), to: (0.0, 0.84754522), stops: BLEND }] },
    Prim::Masked { prims: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_PANEL_HIGH_ROW_8 }], mask: &[Prim::Ramp { x: 729.0, y: 312.0, w: 721.0, h: 387.0, from: (0.0, 0.84754522), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_PANEL_SCAN: &[Prim] = &scans::<196>(729.0, 310.90537809, 721.0, 1.984934);
// The source lower right is square, following the side-bar inward step.
// Shared with the ordinary canvas contour so the sampled field cannot leak.
pub const PANEL_CONTOUR: &[Seg] = &[
    Seg::Line(1040.0, 313.33), Seg::Line(1240.0, 312.5),
    Seg::Line(1439.17, 312.5), Seg::Line(1450.0, 323.33),
    Seg::Line(1450.0, 544.5833), Seg::Line(1440.5548, 554.0285),
    Seg::Line(1440.5548, 699.3177), Seg::Line(729.17, 699.3177),
];
pub const PANEL_ORIGIN: (f32, f32) = (729.17, 313.33);
const MAIL_PANEL: Prim = Prim::Masked { prims: &[Prim::At { x: 0.0, y: 0.0, prims: MAIL_PANEL_LOW }, Prim::Masked { prims: MAIL_PANEL_HIGH, mask: MAIL_PANEL_SCAN }], mask: &[fill_path(PANEL_ORIGIN.0, PANEL_ORIGIN.1, PANEL_CONTOUR, Ink::Fixed(rgb(0xffffff)))] };

/// Opaque panel material, clipped to the source contour.
pub const PANEL: &[Prim] = &[MAIL_PANEL];

const MAIL_BADGE_CUSTOMER_LOW_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x463740)), (1.0, rgb(0x453845))];
const MAIL_BADGE_CUSTOMER_LOW_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x44292d)), (1.0, rgb(0x442f34))];
const MAIL_BADGE_CUSTOMER_LOW: &[Prim] = &[
    Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_CUSTOMER_LOW_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_CUSTOMER_LOW_ROW_1 }], mask: &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_CUSTOMER_HIGH_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x4c353d)), (1.0, rgb(0x4b3643))];
const MAIL_BADGE_CUSTOMER_HIGH_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x4a272b)), (1.0, rgb(0x492d32))];
const MAIL_BADGE_CUSTOMER_HIGH: &[Prim] = &[
    Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_CUSTOMER_HIGH_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_CUSTOMER_HIGH_ROW_1 }], mask: &[Prim::Ramp { x: 119.5833, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_CUSTOMER_SCAN: &[Prim] = &scans::<29>(119.5833, 104.4797115, 56.6667, 1.984934);
const MAIL_BADGE_CUSTOMER: Prim = Prim::Masked { prims: &[Prim::At { x: 0.0, y: 0.0, prims: MAIL_BADGE_CUSTOMER_LOW }, Prim::Masked { prims: MAIL_BADGE_CUSTOMER_HIGH, mask: MAIL_BADGE_CUSTOMER_SCAN }], mask: &[fill_path(119.5833, 104.5833, &[Seg::Line(176.25, 104.5833), Seg::Line(176.25, 161.25), Seg::Line(135.0, 161.25), Seg::Line(119.5833, 145.8333)], Ink::Fixed(rgb(0xffffff)))] };

const MAIL_BADGE_T1_LOW_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x34214a)), (1.0, rgb(0x352149))];
const MAIL_BADGE_T1_LOW_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x32234a)), (1.0, rgb(0x31244a))];
const MAIL_BADGE_T1_LOW: &[Prim] = &[
    Prim::Ramp { x: 1133.3333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T1_LOW_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1133.3333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T1_LOW_ROW_1 }], mask: &[Prim::Ramp { x: 1133.3333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T1_HIGH_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x3a1f48)), (1.0, rgb(0x3b1f47))];
const MAIL_BADGE_T1_HIGH_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x382148)), (1.0, rgb(0x372248))];
const MAIL_BADGE_T1_HIGH: &[Prim] = &[
    Prim::Ramp { x: 1133.3333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T1_HIGH_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1133.3333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T1_HIGH_ROW_1 }], mask: &[Prim::Ramp { x: 1133.3333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T1_SCAN: &[Prim] = &scans::<29>(1133.3333, 104.48118923, 56.6667, 1.984934);
const MAIL_BADGE_T1: Prim = Prim::Masked { prims: &[Prim::At { x: 0.0, y: 0.0, prims: MAIL_BADGE_T1_LOW }, Prim::Masked { prims: MAIL_BADGE_T1_HIGH, mask: MAIL_BADGE_T1_SCAN }], mask: &[fill_path(1133.3333, 104.5833, &[Seg::Line(1190.0, 104.5833), Seg::Line(1190.0, 161.25), Seg::Line(1148.75, 161.25), Seg::Line(1133.3333, 145.8333)], Ink::Fixed(rgb(0xffffff)))] };

const MAIL_BADGE_T2_FIELD_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x712744)), (1.0, rgb(0x702844))];
const MAIL_BADGE_T2_FIELD_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x702944)), (1.0, rgb(0x6f2943))];
const MAIL_BADGE_T2_FIELD: &[Prim] = &[
    Prim::Ramp { x: 1194.1667, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T2_FIELD_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1194.1667, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T2_FIELD_ROW_1 }], mask: &[Prim::Ramp { x: 1194.1667, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T2: Prim = Prim::Masked { prims: &[Prim::At { x: 0.0, y: 0.0, prims: MAIL_BADGE_T2_FIELD }], mask: &[fill_path(1194.1667, 104.5833, &[Seg::Line(1250.8334, 104.5833), Seg::Line(1250.8334, 161.25), Seg::Line(1209.5834, 161.25), Seg::Line(1194.1667, 145.8333)], Ink::Fixed(rgb(0xffffff)))] };

const MAIL_BADGE_T3_LOW_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x342349)), (1.0, rgb(0x332349))];
const MAIL_BADGE_T3_LOW_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x2e254a)), (1.0, rgb(0x2e2647))];
const MAIL_BADGE_T3_LOW: &[Prim] = &[
    Prim::Ramp { x: 1255.0, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T3_LOW_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1255.0, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T3_LOW_ROW_1 }], mask: &[Prim::Ramp { x: 1255.0, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T3_HIGH_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x3a2047)), (1.0, rgb(0x382147))];
const MAIL_BADGE_T3_HIGH_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x332348)), (1.0, rgb(0x342445))];
const MAIL_BADGE_T3_HIGH: &[Prim] = &[
    Prim::Ramp { x: 1255.0, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T3_HIGH_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1255.0, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T3_HIGH_ROW_1 }], mask: &[Prim::Ramp { x: 1255.0, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T3_SCAN: &[Prim] = &scans::<29>(1255.0, 104.49469901, 56.6667, 1.984934);
const MAIL_BADGE_T3: Prim = Prim::Masked { prims: &[Prim::At { x: 0.0, y: 0.0, prims: MAIL_BADGE_T3_LOW }, Prim::Masked { prims: MAIL_BADGE_T3_HIGH, mask: MAIL_BADGE_T3_SCAN }], mask: &[fill_path(1255.0, 104.5833, &[Seg::Line(1311.6667, 104.5833), Seg::Line(1311.6667, 161.25), Seg::Line(1270.4167, 161.25), Seg::Line(1255.0, 145.8333)], Ink::Fixed(rgb(0xffffff)))] };

const MAIL_BADGE_T4_LOW_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x32244a)), (1.0, rgb(0x322548))];
const MAIL_BADGE_T4_LOW_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x2c2647)), (1.0, rgb(0x2e2643))];
const MAIL_BADGE_T4_LOW: &[Prim] = &[
    Prim::Ramp { x: 1315.8333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T4_LOW_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1315.8333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T4_LOW_ROW_1 }], mask: &[Prim::Ramp { x: 1315.8333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T4_HIGH_ROW_0: &[(f32, iced::Color)] = &[(0.0, rgb(0x382248)), (1.0, rgb(0x372346))];
const MAIL_BADGE_T4_HIGH_ROW_1: &[(f32, iced::Color)] = &[(0.0, rgb(0x312445)), (1.0, rgb(0x332341))];
const MAIL_BADGE_T4_HIGH: &[Prim] = &[
    Prim::Ramp { x: 1315.8333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T4_HIGH_ROW_0 },
    Prim::Masked { prims: &[Prim::Ramp { x: 1315.8333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (1.0, 0.0), stops: MAIL_BADGE_T4_HIGH_ROW_1 }], mask: &[Prim::Ramp { x: 1315.8333, y: 104.5833, w: 56.6667, h: 56.6667, from: (0.0, 0.0), to: (0.0, 1.0), stops: BLEND }] }
];

const MAIL_BADGE_T4_SCAN: &[Prim] = &scans::<29>(1315.8333, 104.50086792, 56.6667, 1.984934);
const MAIL_BADGE_T4: Prim = Prim::Masked { prims: &[Prim::At { x: 0.0, y: 0.0, prims: MAIL_BADGE_T4_LOW }, Prim::Masked { prims: MAIL_BADGE_T4_HIGH, mask: MAIL_BADGE_T4_SCAN }], mask: &[fill_path(1315.8333, 104.5833, &[Seg::Line(1372.5, 104.5833), Seg::Line(1372.5, 161.25), Seg::Line(1331.25, 161.25), Seg::Line(1315.8333, 145.8333)], Ink::Fixed(rgb(0xffffff)))] };

/// Badge fields precede their separate frame and primary printing.
pub const BADGES: &[Prim] = &[MAIL_BADGE_CUSTOMER, MAIL_BADGE_T1, MAIL_BADGE_T2, MAIL_BADGE_T3, MAIL_BADGE_T4];

#[cfg(test)]
mod tests {
    use super::{PANEL, PANEL_ORIGIN};
    use crate::{screens::soft, style::{Era, Prim}};

    #[test]
    fn panel_material_has_no_translucent_internal_seams() {
        let palette = Era::Neomil.style().palette;
        // A narrow interior strip crosses every field/scan join while
        // avoiding the contour's intentionally antialiased outer edge.
        // The source top at x900 is y313.33. Start four design units below
        // it and stop before the square bottom, excluding only outer-edge AA.
        let strip = [Prim::At { x: -900.0, y: -(PANEL_ORIGIN.1 + 4.0), prims: PANEL }];
        for scale in [0.37_f32, 0.4875, 0.83, 1.0, 2.4] {
            let height = (380.0 * scale).floor() as u32;
            let bytes = soft::composite(&strip, &palette, 4, height, scale);
            for (index, pixel) in bytes.chunks_exact(4).enumerate() {
                assert_eq!(pixel[3], 255, "panel seam at scale {scale}, row {}", index / 4);
            }
        }
    }
}
