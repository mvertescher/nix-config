//! Light-edge resolve: light marks on dark ground are antialiased in
//! encoded (sRGB) space, as the reference art and rsvg do. The CPU mirror
//! of `fs_main` in `shader/blit.wgsl`, kept here so the rule can be
//! unit-tested. This module is a cp-eras-ui local patch; see `PATCHES.md`
//! beside this crate.
//!
//! # Why
//!
//! The triangle (mesh) pipeline draws every canvas fill and stroke into a
//! multisampled target of the surface's sRGB format; the hardware resolve
//! averages the samples in *linear* light and the frame is composited in
//! linear light. Reference art, browsers and rsvg average and blend
//! *encoded* values. Because the sRGB transfer is convex, the linear
//! average of a partly covered pixel is lighter than the encoded one, so
//! the partly covered rows of a light line on a dark ground come out
//! brighter and every thin light line reads heavy: Neomil's 0.7 dp header
//! rule measures 2.21 px where the reference's measures 1.80 px (edge rows
//! at encoded coverage 0.50 / 0.71 against 0.31 / 0.49).
//!
//! # What
//!
//! Each fragment of the MSAA pass also adds one to a per-sample *mark
//! count* (a second, `R16Float` attachment with additive blending) when its
//! colour is opaque. Within a pixel, samples above the pixel's lowest count
//! are covered by something drawn over what the other samples hold: the
//! *mark*, and what it was drawn over, the *under*. The resolve
//! (`blit.wgsl`) then, per pixel:
//!
//! - uses the hardware-resolved value unchanged when every sample has the
//!   same count (no mark edge: interiors, abutting fills, translucent
//!   layers), when mark or under is not opaque (the under is in the frame
//!   below this pass and cannot be read), or when the mark is not lighter
//!   than the under;
//! - otherwise averages the samples' *encoded* values (exactly what an
//!   encoded rasteriser's coverage blend gives for those samples) and mixes
//!   that over the linear result by [`light_weight`]: zero for a mark no
//!   lighter than its under (Michelson contrast of luminance <= 0), a
//!   smoothstep to one at [`CONTRAST_FULL`].
//!
//! Dark marks on light fills (dark text outlines, Neomil's dark insets on
//! red tiles, Entropism's caption frames) have negative contrast and keep
//! the linear resolve bit for bit: their correction, where there is one,
//! is `widgets::stem` in cp-eras-ui, on the geometry.
//!
//! Text is excluded: light text was tuned against the linear blend and is
//! left alone (cryoglyph's `PATCHES.md`). Glyph outlines that reach this
//! pipeline (canvas text under a non-uniform transform) are filled in
//! [`text_ink`], whose alpha is a hair under one, so they never count as
//! marks; cp-eras-ui uses the same function where it fills outlines itself.

use std::sync::OnceLock;

/// Michelson contrast of luminance, `(Y_mark - Y_under) / (Y_mark +
/// Y_under)`, from which a light mark's edge is resolved in full.
pub const CONTRAST_FULL: f32 = 0.1;
/// Alpha from which a group of samples counts as opaque.
pub const OPAQUE: f32 = 0.998;
/// The alpha of [`text_ink`]: below one by 2^-20, so its fragments are not
/// marks, and a millionth of the colour under it shows through, far below
/// one 8-bit level.
pub const TEXT_ALPHA: f32 = 1.0 - 1.0 / 1_048_576.0;

/// The mark-count attachment's format.
pub const MARK_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R16Float;

/// The mark-count attachment: one per opaque fragment, summed.
pub fn mark_target() -> wgpu::ColorTargetState {
    let add = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    };
    wgpu::ColorTargetState {
        format: MARK_FORMAT,
        blend: Some(wgpu::BlendState { color: add, alpha: add }),
        write_mask: wgpu::ColorWrites::RED,
    }
}

/// Text colour for glyph outlines filled through the mesh pipeline: an
/// opaque colour gets [`TEXT_ALPHA`], so its fragments are not marks and
/// its edges keep the linear resolve. Translucent colours are not marks
/// anyway and are returned unchanged.
pub fn text_ink(color: crate::core::Color) -> crate::core::Color {
    if color.a >= 1.0 {
        crate::core::Color { a: TEXT_ALPHA, ..color }
    } else {
        color
    }
}

/// Whether a vertex colour with this alpha counts as a mark (the vertex
/// shader's `mark`).
pub fn is_mark(alpha: f32) -> bool {
    alpha >= 1.0
}

/// The resolve's mode, from `ICED_WGPU_LIGHT_EDGES`: unset or anything else
/// is on; `off` (or `0`) restores the upstream resolve exactly; `debug`
/// paints each mark edge pixel by its class instead (green: corrected, by
/// weight; blue: a dark mark, untouched; purple: over nothing drawn in
/// the pass, untouched).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Upstream resolve.
    Off,
    /// Light edges resolved in encoded space.
    On,
    /// Classify instead of drawing.
    Debug,
}

impl Mode {
    /// The process's mode, read once.
    pub fn current() -> Mode {
        static MODE: OnceLock<Mode> = OnceLock::new();
        *MODE.get_or_init(|| {
            match std::env::var("ICED_WGPU_LIGHT_EDGES").as_deref() {
                Ok("off" | "0") => Mode::Off,
                Ok("debug") => Mode::Debug,
                _ => Mode::On,
            }
        })
    }

    /// The value `blit.wgsl` reads as `u_ratio.z`.
    pub fn uniform(self) -> f32 {
        match self {
            Mode::Off => 0.0,
            Mode::On => 1.0,
            Mode::Debug => 2.0,
        }
    }
}

/// The sRGB transfer function, encoded -> linear.
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// The inverse transfer function, linear -> encoded.
pub fn linear_to_srgb(l: f32) -> f32 {
    let l = l.max(0.0);
    if l <= 0.0031308 {
        l * 12.92
    } else {
        1.055 * l.powf(1.0 / 2.4) - 0.055
    }
}

/// Relative luminance of a linear RGB colour (Rec. 709 weights).
pub fn luminance(rgb: [f32; 3]) -> f32 {
    0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2]
}

/// How much of the encoded resolve a mark of luminance `ym` over an under
/// of luminance `yu` takes: 0 when the mark is no lighter, a smoothstep in
/// Michelson contrast to 1 at [`CONTRAST_FULL`].
pub fn light_weight(ym: f32, yu: f32) -> f32 {
    let c = (ym - yu) / (ym + yu).max(1e-6);
    let t = (c / CONTRAST_FULL).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// One sample: premultiplied linear RGBA and its mark count.
pub type Sample = ([f32; 4], f32);

/// The resolved pixel for `samples`, given the hardware's linear resolve
/// `base` (the mean of the samples, up to the resolve target's rounding)
/// and, when the frame could be copied, the linear colour `ground` of the
/// frame under the pass. `base` itself (premultiplied, composited by the
/// blend) whenever the correction does not apply; otherwise an opaque
/// colour that replaces the frame's pixel.
pub fn resolve(samples: &[Sample], base: [f32; 4], ground: Option<[f32; 3]>) -> [f32; 4] {
    let lo = samples.iter().map(|s| s.1).fold(f32::INFINITY, f32::min);
    let hi = samples.iter().map(|s| s.1).fold(f32::NEG_INFINITY, f32::max);
    if samples.is_empty() || hi - lo < 0.5 {
        return base;
    }
    let (mut mark, mut under, mut nm, mut nu) = ([0.0f32; 4], [0.0f32; 4], 0.0f32, 0.0f32);
    let mut encoded = [0.0f32; 3];
    for (c, m) in samples {
        let c = match ground {
            Some(g) => &over(*c, g),
            None => c,
        };
        let (acc, n) = if *m > lo + 0.5 { (&mut mark, &mut nm) } else { (&mut under, &mut nu) };
        for i in 0..4 {
            acc[i] += c[i];
        }
        *n += 1.0;
        for i in 0..3 {
            encoded[i] += linear_to_srgb(c[i]);
        }
    }
    let mark = mark.map(|v| v / nm);
    let under = under.map(|v| v / nu);
    if under[3] < OPAQUE || mark[3] < OPAQUE {
        return base;
    }
    let w = light_weight(luminance([mark[0], mark[1], mark[2]]), luminance([under[0], under[1], under[2]]));
    if w <= 0.0 {
        return base;
    }
    let n = samples.len() as f32;
    let linear = match ground {
        Some(g) => over(base, g),
        None => base,
    };
    let mut out = [0.0, 0.0, 0.0, 1.0];
    for i in 0..3 {
        let e = srgb_to_linear(encoded[i] / n);
        out[i] = linear[i] + (e - linear[i]) * w;
    }
    out
}

/// The quad pipeline's border: the colour of a pixel a fraction `t` covered
/// by `border` over `fill` (both linear, opaque), which upstream mixes
/// linearly (`mix(fill, border, t)`). Where the border is the lighter, the
/// encoded mix by [`light_weight`]; the mirror of `light_border` in
/// `shader/quad/solid.wgsl`. Translucent colours and fully covered or
/// uncovered pixels keep the upstream mix.
pub fn quad_border(fill: [f32; 4], border: [f32; 4], t: f32) -> [f32; 4] {
    let mixed: [f32; 4] = std::array::from_fn(|i| fill[i] + (border[i] - fill[i]) * t);
    if t <= 0.0 || t >= 1.0 || fill[3] < 1.0 || border[3] < 1.0 {
        return mixed;
    }
    let w = light_weight(luminance([border[0], border[1], border[2]]), luminance([fill[0], fill[1], fill[2]]));
    if w <= 0.0 {
        return mixed;
    }
    let mut out = mixed;
    for i in 0..3 {
        let e = srgb_to_linear(linear_to_srgb(fill[i]) + (linear_to_srgb(border[i]) - linear_to_srgb(fill[i])) * t);
        out[i] = mixed[i] + (e - mixed[i]) * w;
    }
    out
}

/// Premultiplied `c` composited over opaque `g`, in linear light.
fn over(c: [f32; 4], g: [f32; 3]) -> [f32; 4] {
    [c[0] + (1.0 - c[3]) * g[0], c[1] + (1.0 - c[3]) * g[1], c[2] + (1.0 - c[3]) * g[2], 1.0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lin(hex: u32) -> [f32; 4] {
        let c = |s: u32| srgb_to_linear(((hex >> s) & 0xff) as f32 / 255.0);
        [c(16), c(8), c(0), 1.0]
    }

    /// `k` of four samples hold `ink` over `ground`, as one opaque mark
    /// drawn over one opaque fill.
    fn pixel(ink: [f32; 4], ground: [f32; 4], k: usize) -> (Vec<Sample>, [f32; 4]) {
        let s: Vec<Sample> = (0..4).map(|i| if i < k { (ink, 2.0) } else { (ground, 1.0) }).collect();
        let mut base = [0.0; 4];
        for (c, _) in &s {
            for i in 0..4 {
                base[i] += c[i] / 4.0;
            }
        }
        (s, base)
    }

    fn enc_cov(v: [f32; 4], ink: [f32; 4], ground: [f32; 4], ch: usize) -> f32 {
        (linear_to_srgb(v[ch]) - linear_to_srgb(ground[ch])) / (linear_to_srgb(ink[ch]) - linear_to_srgb(ground[ch]))
    }

    // Neomil's header rule and ground; Kitsch mint on its dark ground;
    // Entropism's sage on its ground; Neo-kitsch's gold.
    const LIGHT_ON_DARK: [(u32, u32); 4] =
        [(0xfb3535, 0x14244d), (0x67e0c4, 0x101414), (0x8bb08f, 0x15130e), (0xf0a65a, 0x100b0c)];
    // Dark ink on fills: ON_CARD on red, the tile inset, Entropism's
    // selected caption ink on sage, black on gold.
    const DARK_ON_LIGHT: [(u32, u32); 4] =
        [(0x1a0405, 0xef3333), (0x5f1112, 0xef3333), (0x223c1b, 0x8bb08f), (0x000000, 0xf0a65a)];

    #[test]
    fn interiors_and_uniform_pixels_unchanged() {
        for (ink, ground) in LIGHT_ON_DARK {
            for k in [0, 4] {
                let (s, base) = pixel(lin(ink), lin(ground), k);
                // One count throughout: nothing to resolve.
                let s: Vec<Sample> = s.into_iter().map(|(c, _)| (c, 1.0)).collect();
                assert_eq!(resolve(&s, base, None), base);
            }
        }
    }

    #[test]
    fn dark_marks_untouched() {
        for (ink, fill) in DARK_ON_LIGHT {
            for k in 1..4 {
                let (s, base) = pixel(lin(ink), lin(fill), k);
                assert_eq!(resolve(&s, base, None), base, "{ink:06x} on {fill:06x}, {k}/4");
            }
        }
        assert_eq!(light_weight(0.01, 0.5), 0.0);
        assert_eq!(light_weight(0.2, 0.2), 0.0);
    }

    #[test]
    fn light_marks_land_on_the_encoded_blend() {
        for (ink, ground) in LIGHT_ON_DARK {
            let (i, g) = (lin(ink), lin(ground));
            assert_eq!(light_weight(luminance([i[0], i[1], i[2]]), luminance([g[0], g[1], g[2]])), 1.0);
            for k in 1..4 {
                let (s, base) = pixel(i, g, k);
                let out = resolve(&s, base, None);
                let f = k as f32 / 4.0;
                for ch in 0..3 {
                    if (linear_to_srgb(i[ch]) - linear_to_srgb(g[ch])).abs() < 0.05 {
                        continue;
                    }
                    let c = enc_cov(out, i, g, ch);
                    assert!((c - f).abs() < 1e-3, "{ink:06x} on {ground:06x} {k}/4 ch{ch}: {c}");
                }
            }
        }
    }

    #[test]
    fn header_rule_edge_rows() {
        // A row a quarter covered by #fb3535 over #14244d: linear reads
        // 0.50 encoded (measured 0.50), the encoded resolve 0.25.
        let (i, g) = (lin(0xfb3535), lin(0x14244d));
        let (s, base) = pixel(i, g, 1);
        assert!((enc_cov(base, i, g, 0) - 0.50).abs() < 0.02, "{}", enc_cov(base, i, g, 0));
        assert!((enc_cov(resolve(&s, base, None), i, g, 0) - 0.25).abs() < 1e-3);
    }

    #[test]
    fn monotonic_and_only_ever_darker() {
        for (ink, ground) in LIGHT_ON_DARK {
            let (i, g) = (lin(ink), lin(ground));
            let mut last = luminance([g[0], g[1], g[2]]);
            for k in 0..=4 {
                let (s, base) = pixel(i, g, k);
                let out = resolve(&s, base, None);
                let y = luminance([out[0], out[1], out[2]]);
                assert!(y >= last - 1e-6, "{ink:06x}: {k}/4 {y} < {last}");
                // Encoded averaging never lightens a pixel.
                for ch in 0..3 {
                    assert!(out[ch] <= base[ch] + 1e-6);
                }
                last = y;
            }
        }
    }

    #[test]
    fn weight_is_continuous_and_bounded() {
        let mut last = 0.0;
        for n in 0..=2000 {
            let ym = 0.02 + n as f32 * 0.00005;
            let w = light_weight(ym, 0.02);
            assert!((0.0..=1.0).contains(&w));
            assert!(w >= last - 1e-6 && w - last < 0.03, "ym={ym}");
            last = w;
        }
        assert_eq!(light_weight(0.5, 0.0), 1.0);
    }

    #[test]
    fn transparent_under_is_left_linear() {
        let i = lin(0xfb3535);
        let s: Vec<Sample> = vec![(i, 1.0), ([0.0; 4], 0.0), ([0.0; 4], 0.0), ([0.0; 4], 0.0)];
        let base = [i[0] / 4.0, i[1] / 4.0, i[2] / 4.0, 0.25];
        assert_eq!(resolve(&s, base, None), base);
    }

    #[test]
    fn over_the_copied_ground() {
        // The same mark drawn over nothing in the pass resolves, given the
        // frame under it, exactly as if the ground had been in the pass.
        for (ink, ground) in LIGHT_ON_DARK {
            let (i, g) = (lin(ink), lin(ground));
            for k in 1..4 {
                let (s, base) = pixel(i, g, k);
                let want = resolve(&s, base, None);
                let bare: Vec<Sample> = (0..4).map(|n| if n < k { (i, 1.0) } else { ([0.0; 4], 0.0) }).collect();
                let f = k as f32 / 4.0;
                let bare_base = [i[0] * f, i[1] * f, i[2] * f, f];
                let got = resolve(&bare, bare_base, Some([g[0], g[1], g[2]]));
                for ch in 0..4 {
                    assert!((got[ch] - want[ch]).abs() < 1e-5, "{ink:06x} {k}/4 ch{ch}: {got:?} {want:?}");
                }
            }
        }
        // A dark mark over nothing, on a light ground: the blend's own result.
        for (ink, fill) in DARK_ON_LIGHT {
            let i = lin(ink);
            let bare: Vec<Sample> = (0..4).map(|n| if n < 2 { (i, 1.0) } else { ([0.0; 4], 0.0) }).collect();
            let base = [i[0] / 2.0, i[1] / 2.0, i[2] / 2.0, 0.5];
            let f = lin(fill);
            assert_eq!(resolve(&bare, base, Some([f[0], f[1], f[2]])), base);
        }
    }

    #[test]
    fn text_is_not_a_mark() {
        let c = crate::core::Color::from_rgb8(0xfb, 0x35, 0x35);
        assert!(is_mark(c.a));
        assert!(!is_mark(text_ink(c).a));
        assert!(text_ink(c).a > 0.999_99);
        let faint = crate::core::Color { a: 0.5, ..c };
        assert_eq!(text_ink(faint), faint);
        assert!(!is_mark(faint.a));
    }

    #[test]
    fn quad_border_light_on_dark_only() {
        for (ink, ground) in LIGHT_ON_DARK {
            let (i, g) = (lin(ink), lin(ground));
            let mut last = -1.0;
            for n in 0..=64 {
                let t = n as f32 / 64.0;
                let out = quad_border(g, i, t);
                for ch in 0..3 {
                    if (linear_to_srgb(i[ch]) - linear_to_srgb(g[ch])).abs() > 0.05 {
                        assert!((enc_cov(out, i, g, ch) - t).abs() < 1e-3, "{ink:06x} t={t} ch{ch}");
                    }
                }
                let y = luminance([out[0], out[1], out[2]]);
                assert!(y >= last - 1e-6);
                last = y;
            }
            assert_eq!(quad_border(g, i, 0.0), g);
            let one = quad_border(g, i, 1.0);
            assert!((0..4).all(|k| (one[k] - i[k]).abs() < 1e-6));
        }
        for (ink, fill) in DARK_ON_LIGHT {
            let (i, f) = (lin(ink), lin(fill));
            for n in 0..=16 {
                let t = n as f32 / 16.0;
                let up: [f32; 4] = std::array::from_fn(|k| f[k] + (i[k] - f[k]) * t);
                assert_eq!(quad_border(f, i, t), up);
            }
        }
    }

    #[test]
    fn shader_constants_match() {
        let wgsl = include_str!("shader/blit.wgsl");
        for (name, value) in [("CONTRAST_FULL", CONTRAST_FULL), ("OPAQUE", OPAQUE)] {
            let line = wgsl
                .lines()
                .find(|l| l.starts_with(&format!("const {name}")))
                .unwrap_or_else(|| panic!("{name} missing from blit.wgsl"));
            let v: f32 = line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap();
            assert_eq!(v, value, "{name}");
        }
        let quad = include_str!("shader/quad/solid.wgsl");
        let line = quad.lines().find(|l| l.starts_with("const QUAD_CONTRAST_FULL")).unwrap();
        let v: f32 = line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap();
        assert_eq!(v, CONTRAST_FULL, "QUAD_CONTRAST_FULL");
        let solid = include_str!("shader/triangle/solid.wgsl");
        assert!(solid.contains("select(0.0, 1.0, input.color.a >= 1.0)"));
    }
}
