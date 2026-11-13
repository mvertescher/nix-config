//! Dark-ink coverage correction: the CPU mirror of `dark_ink_coverage` in
//! `shader.wgsl`, kept here so the curve can be unit-tested. This module is
//! the cp-eras-ui local patch; see `PATCHES.md` beside this crate.
//!
//! # Why
//!
//! With [`ColorMode::Accurate`](crate::ColorMode) on an sRGB surface, a
//! glyph's edge coverage is blended in *linear* light. Reference art,
//! browsers and rsvg blend in the encoded (sRGB) space. For light ink on a
//! dark ground the two are close enough that cp-eras-ui tuned its weights
//! against the linear look; for *dark* ink on a light fill, linear blending
//! makes every partly covered pixel much lighter than the encoded blend, so
//! the text comes out a stop thinner and paler than the art it reproduces.
//!
//! # What
//!
//! For a glyph pixel with coverage `a`, ink of encoded lightness `i` and a
//! fill of encoded lightness `B`, the encoded blend lands on
//! `L(a i + (1 - a) B)` in linear light (`L` = the sRGB transfer function).
//! A linear blend reaches the same value at coverage
//!
//! ```text
//! a_s = (L(B) - L(a i + (1 - a) B)) / (L(B) - L(i))
//! ```
//!
//! which is `>= a` because `L` is convex, `0` at `a = 0` and `1` at `a = 1`.
//! The shader cannot see the fill, so `B` is a fixed assumption,
//! [`ASSUMED_FILL`]: the lightness of the eras' selection fills (sage,
//! gold), which are what dark ink sits on.
//!
//! The shader also cannot tell dark ink on a light fill from dark ink on a
//! dark ground, or which of two colours is the "ink". It therefore applies
//! the correction by the ink's own luminance, with a smooth window:
//! full strength up to [`FULL_Y`], fading by a smoothstep to exactly zero at
//! [`OFF_Y`] (relative luminance, linear). Inks at or above `OFF_Y` take the
//! original code path untouched, so light text on a dark ground is
//! bit-identical to upstream. No hard threshold: two near-identical inks get
//! near-identical coverage curves.
//!
//! `OFF_Y` sits just under the darkest ink cp-eras-ui draws on a *dark*
//! ground (Neomil's dim red `#a32226`, Y = 0.0907), and above every ink it
//! draws on a fill but two (`#8a3f28` and `#6b4d08`, Kitsch's store and
//! dashboard), which therefore keep the linear look. A custom palette that
//! puts ink darker than `OFF_Y` on a dark ground would see that text drawn
//! slightly heavier; that is the price of a correction keyed on the ink.

/// Encoded (sRGB) lightness assumed for the fill under dark ink.
pub const ASSUMED_FILL: f32 = 0.78;
/// Ink luminance (linear Y) at and below which the correction is full.
pub const FULL_Y: f32 = 0.04;
/// Ink luminance (linear Y) at and above which the correction is off.
pub const OFF_Y: f32 = 0.085;

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

/// How much of the correction an ink of luminance `y` receives: 1 up to
/// [`FULL_Y`], 0 from [`OFF_Y`], a smoothstep between.
pub fn strength(y: f32) -> f32 {
    let t = ((y - FULL_Y) / (OFF_Y - FULL_Y)).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
}

/// The coverage a glyph pixel is drawn with, given its rasterised
/// coverage `a` and the ink's linear RGB. Identity when `strength` is 0.
pub fn dark_ink_coverage(a: f32, ink_linear: [f32; 3]) -> f32 {
    let y = luminance(ink_linear);
    let w = strength(y);
    if w <= 0.0 {
        return a;
    }
    let i = linear_to_srgb(y);
    let fill = srgb_to_linear(ASSUMED_FILL);
    let target = srgb_to_linear(a * i + (1.0 - a) * ASSUMED_FILL);
    let encoded = ((fill - target) / (fill - y)).clamp(0.0, 1.0);
    a + (encoded - a) * w
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEPS: usize = 256;

    fn grey(y: f32) -> [f32; 3] {
        [y, y, y]
    }

    fn inks() -> impl Iterator<Item = f32> {
        (0..=200).map(|n| n as f32 * 0.001)
    }

    #[test]
    fn identity_for_light_ink() {
        for y in [OFF_Y, 0.0907, 0.2, 0.44, 0.83, 1.0] {
            for n in 0..=STEPS {
                let a = n as f32 / STEPS as f32;
                assert_eq!(dark_ink_coverage(a, grey(y)).to_bits(), a.to_bits(), "y={y} a={a}");
            }
        }
        // Neomil's dim red, the darkest ink drawn on a dark ground.
        let red = [0xa3, 0x22, 0x26].map(|c| srgb_to_linear(c as f32 / 255.0));
        assert!(luminance(red) > OFF_Y);
        assert_eq!(dark_ink_coverage(0.5, red), 0.5);
    }

    #[test]
    fn endpoints_fixed_and_bounded() {
        for y in inks() {
            assert_eq!(dark_ink_coverage(0.0, grey(y)), 0.0, "y={y}");
            assert!((dark_ink_coverage(1.0, grey(y)) - 1.0).abs() < 1e-6, "y={y}");
            for n in 0..=STEPS {
                let a = n as f32 / STEPS as f32;
                let c = dark_ink_coverage(a, grey(y));
                assert!((0.0..=1.0).contains(&c), "y={y} a={a} c={c}");
                // Only ever thickens.
                assert!(c >= a - 1e-6, "y={y} a={a} c={c}");
            }
        }
    }

    #[test]
    fn monotonic_in_coverage() {
        for y in inks() {
            let mut last = 0.0;
            for n in 0..=STEPS {
                let a = n as f32 / STEPS as f32;
                let c = dark_ink_coverage(a, grey(y));
                assert!(c >= last - 1e-6, "y={y} a={a}: {c} < {last}");
                last = c;
            }
        }
    }

    #[test]
    fn weakens_smoothly_as_ink_lightens() {
        // At fixed coverage, a lighter ink never gets more correction (to
        // 1e-4: near black, the linear toe of the sRGB curve makes the exact
        // curve rise by ~2.5e-5, a hundredth of one 8-bit level), and
        // neighbouring inks (0.001 apart in Y) differ only slightly.
        for n in 1..STEPS {
            let a = n as f32 / STEPS as f32;
            let mut last = f32::INFINITY;
            for y in inks() {
                let c = dark_ink_coverage(a, grey(y));
                assert!(c <= last + 1e-4, "a={a} y={y}: {c} > {last}");
                if last.is_finite() {
                    assert!(last - c < 0.03, "a={a} y={y}: step {}", last - c);
                }
                last = c;
            }
        }
        assert_eq!(strength(0.0), 1.0);
        assert_eq!(strength(FULL_Y), 1.0);
        assert_eq!(strength(OFF_Y), 0.0);
    }

    #[test]
    fn black_ink_matches_the_encoded_blend() {
        // For black ink the curve is the exact encoded blend on the fill.
        let a = 0.5;
        let c = dark_ink_coverage(a, grey(0.0));
        let want = 1.0 - srgb_to_linear(0.5 * ASSUMED_FILL) / srgb_to_linear(ASSUMED_FILL);
        assert!((c - want).abs() < 1e-6);
        assert!(c > 0.7 && c < 0.85, "{c}");
    }

    #[test]
    fn shader_constants_match() {
        let wgsl = include_str!("shader.wgsl");
        for (name, value) in [("ASSUMED_FILL", ASSUMED_FILL), ("FULL_Y", FULL_Y), ("OFF_Y", OFF_Y)] {
            let line = wgsl
                .lines()
                .find(|l| l.starts_with(&format!("const {name}")))
                .unwrap_or_else(|| panic!("{name} missing from shader.wgsl"));
            let v: f32 = line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap();
            assert_eq!(v, value, "{name}");
        }
    }
}
