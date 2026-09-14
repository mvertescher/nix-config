//! Dashboard-only sampled composite ground from img-07-dashboard.png.
//! Horizontal row ramps are blended across eleven vertical intervals.
//! Keep the shared Neomil glow constants unchanged for other screens.
use crate::palette::rgb;
use crate::style::Prim;

const ROW_0: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x2b363e)),
    (0.03125, rgb(0x2c3b4c)),
    (0.0625, rgb(0x2c3c56)),
    (0.125, rgb(0x2c3b59)),
    (0.1875, rgb(0x2c3759)),
    (0.25, rgb(0x293058)),
    (0.375, rgb(0x232757)),
    (0.5, rgb(0x1e2254)),
    (0.625, rgb(0x191e51)),
    (0.75, rgb(0x141d4f)),
    (0.875, rgb(0x0f1f4d)),
    (0.9375, rgb(0x0d244c)),
    (0.96875, rgb(0x0d2644)),
    (1.0, rgb(0x0c1d27)),
];
const ROW_1: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x28272a)),
    (0.03125, rgb(0x272e34)),
    (0.0625, rgb(0x283946)),
    (0.125, rgb(0x283950)),
    (0.1875, rgb(0x253855)),
    (0.25, rgb(0x243257)),
    (0.375, rgb(0x1e2755)),
    (0.5, rgb(0x1a2153)),
    (0.625, rgb(0x151e51)),
    (0.75, rgb(0x101d4f)),
    (0.875, rgb(0x0b214d)),
    (0.9375, rgb(0x092448)),
    (0.96875, rgb(0x08223a)),
    (1.0, rgb(0x080e13)),
];
const ROW_2: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x251b1d)),
    (0.03125, rgb(0x251c1e)),
    (0.0625, rgb(0x242327)),
    (0.125, rgb(0x202d35)),
    (0.1875, rgb(0x1f2f3d)),
    (0.25, rgb(0x1e324c)),
    (0.375, rgb(0x182a53)),
    (0.5, rgb(0x152352)),
    (0.625, rgb(0x112050)),
    (0.75, rgb(0x0b214e)),
    (0.875, rgb(0x072345)),
    (0.9375, rgb(0x05223a)),
    (0.96875, rgb(0x041520)),
    (1.0, rgb(0x060305)),
];
const ROW_3: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x201617)),
    (0.03125, rgb(0x201617)),
    (0.0625, rgb(0x201618)),
    (0.125, rgb(0x1f191c)),
    (0.1875, rgb(0x1a1c20)),
    (0.25, rgb(0x18272f)),
    (0.375, rgb(0x142c47)),
    (0.5, rgb(0x11284e)),
    (0.625, rgb(0x0e254d)),
    (0.75, rgb(0x092345)),
    (0.875, rgb(0x042032)),
    (0.9375, rgb(0x03171e)),
    (0.96875, rgb(0x030709)),
    (1.0, rgb(0x030203)),
];
const ROW_4: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1e1214)),
    (0.03125, rgb(0x1e1314)),
    (0.0625, rgb(0x1d1214)),
    (0.125, rgb(0x1b1113)),
    (0.1875, rgb(0x191114)),
    (0.25, rgb(0x15161a)),
    (0.375, rgb(0x112532)),
    (0.5, rgb(0x0e2841)),
    (0.625, rgb(0x0b2641)),
    (0.75, rgb(0x062136)),
    (0.875, rgb(0x03171f)),
    (0.9375, rgb(0x020507)),
    (0.96875, rgb(0x020203)),
    (1.0, rgb(0x020203)),
];
const ROW_5: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1b0f11)),
    (0.03125, rgb(0x1b0f11)),
    (0.0625, rgb(0x1a0f11)),
    (0.125, rgb(0x190e10)),
    (0.1875, rgb(0x170d0e)),
    (0.25, rgb(0x150c0e)),
    (0.375, rgb(0x0f161b)),
    (0.5, rgb(0x0c1f2a)),
    (0.625, rgb(0x091f2c)),
    (0.75, rgb(0x041922)),
    (0.875, rgb(0x02080a)),
    (0.9375, rgb(0x030203)),
    (0.96875, rgb(0x020303)),
    (1.0, rgb(0x030202)),
];
const ROW_6: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x190c0e)),
    (0.03125, rgb(0x190c0e)),
    (0.0625, rgb(0x180c0e)),
    (0.125, rgb(0x170b0e)),
    (0.1875, rgb(0x15090c)),
    (0.25, rgb(0x12080a)),
    (0.375, rgb(0x110b0d)),
    (0.5, rgb(0x0c1014)),
    (0.625, rgb(0x071116)),
    (0.75, rgb(0x020c12)),
    (0.875, rgb(0x020303)),
    (0.9375, rgb(0x030302)),
    (0.96875, rgb(0x030203)),
    (1.0, rgb(0x020202)),
];
const ROW_7: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x170a0c)),
    (0.03125, rgb(0x170a0b)),
    (0.0625, rgb(0x170a0c)),
    (0.125, rgb(0x15090b)),
    (0.1875, rgb(0x14070a)),
    (0.25, rgb(0x110609)),
    (0.375, rgb(0x110204)),
    (0.5, rgb(0x0d0609)),
    (0.625, rgb(0x080509)),
    (0.75, rgb(0x030306)),
    (0.875, rgb(0x020302)),
    (0.9375, rgb(0x030203)),
    (0.96875, rgb(0x030302)),
    (1.0, rgb(0x020202)),
];
const ROW_8: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x160608)),
    (0.03125, rgb(0x150709)),
    (0.0625, rgb(0x150709)),
    (0.125, rgb(0x130608)),
    (0.1875, rgb(0x110508)),
    (0.25, rgb(0x0f0507)),
    (0.375, rgb(0x0d0407)),
    (0.5, rgb(0x0a0305)),
    (0.625, rgb(0x060304)),
    (0.75, rgb(0x020102)),
    (0.875, rgb(0x020302)),
    (0.9375, rgb(0x030302)),
    (0.96875, rgb(0x030202)),
    (1.0, rgb(0x020203)),
];
const ROW_9: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x130305)),
    (0.03125, rgb(0x120305)),
    (0.0625, rgb(0x120305)),
    (0.125, rgb(0x110305)),
    (0.1875, rgb(0x0e0304)),
    (0.25, rgb(0x0d0305)),
    (0.375, rgb(0x0a0304)),
    (0.5, rgb(0x070304)),
    (0.625, rgb(0x030203)),
    (0.75, rgb(0x030403)),
    (0.875, rgb(0x030202)),
    (0.9375, rgb(0x030202)),
    (0.96875, rgb(0x030303)),
    (1.0, rgb(0x020202)),
];
const ROW_10: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x0f0304)),
    (0.03125, rgb(0x0f0304)),
    (0.0625, rgb(0x0e0304)),
    (0.125, rgb(0x0c0304)),
    (0.1875, rgb(0x0a0304)),
    (0.25, rgb(0x080304)),
    (0.375, rgb(0x060304)),
    (0.5, rgb(0x030203)),
    (0.625, rgb(0x020202)),
    (0.75, rgb(0x020302)),
    (0.875, rgb(0x030303)),
    (0.9375, rgb(0x020202)),
    (0.96875, rgb(0x020302)),
    (1.0, rgb(0x020203)),
];
const ROW_11: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x0a0303)),
    (0.03125, rgb(0x090304)),
    (0.0625, rgb(0x090304)),
    (0.125, rgb(0x080304)),
    (0.1875, rgb(0x060304)),
    (0.25, rgb(0x050204)),
    (0.375, rgb(0x030203)),
    (0.5, rgb(0x020202)),
    (0.625, rgb(0x020203)),
    (0.75, rgb(0x030303)),
    (0.875, rgb(0x020302)),
    (0.9375, rgb(0x020303)),
    (0.96875, rgb(0x020302)),
    (1.0, rgb(0x030302)),
];

const BLEND: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x000000)),
    (1.0, rgb(0xffffff)),
];

/// Opaque dashboard background at every scale. A complete first row is
/// followed by complete horizontal rows under clamped vertical masks.
/// Adjacent band rectangles would source-over their antialiased edges,
/// leaving translucent seams at fractional canvas scales. Full-frame
/// mask coverage avoids multiplying edge coverage at internal boundaries.
/// Paint inside the dashboard's existing leading `Prim::Soft` group.
pub const BACKGROUND: &[Prim] = &[
    Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
        from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_0 },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_1 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (0.0, 0.088888889), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_2 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.088888889), to: (0.0, 0.177777778), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_3 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.177777778), to: (0.0, 0.266666667), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_4 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.266666667), to: (0.0, 0.333333333), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_5 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.333333333), to: (0.0, 0.4), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_6 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.4), to: (0.0, 0.466666667), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_7 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.466666667), to: (0.0, 0.533333333), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_8 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.533333333), to: (0.0, 0.622222222), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_9 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.622222222), to: (0.0, 0.755555556), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_10 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.755555556), to: (0.0, 0.888888889), stops: BLEND }],
    },
    Prim::Masked {
        prims: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.0), to: (1.0, 0.0), stops: ROW_11 }],
        mask: &[Prim::Ramp { x: 0.0, y: 0.0, w: 1600.0, h: 900.0,
            from: (0.0, 0.888888889), to: (0.0, 1.0), stops: BLEND }],
    },
];


#[cfg(test)]
mod tests {
    use super::BACKGROUND;
    use crate::{screens::soft, style::Era};

    #[test]
    fn background_has_no_translucent_seams_at_fractional_scales() {
        let palette = Era::Neomil.style().palette;
        // A narrow vertical strip crosses every gradient join. Exclude
        // the fractional outer bottom edge; all interior pixels must
        // remain opaque regardless of where a join falls within a pixel.
        for k in [0.37_f32, 0.4875, 0.83, 1.0] {
            let h = (900.0 * k).floor() as u32;
            let bytes = soft::composite(BACKGROUND, &palette, 4, h, k);
            for (i, pixel) in bytes.chunks_exact(4).enumerate() {
                assert_eq!(pixel[3], 255, "ground seam at scale {k}, row {}", i / 4);
            }
        }
    }
}
