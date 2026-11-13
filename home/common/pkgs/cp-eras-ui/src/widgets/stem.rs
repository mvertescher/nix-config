//! Stem darkening: dark ink on a fill, on the outline (MSAA) route.
//!
//! # Why
//!
//! wgpu blends coverage in linear light; the reference art and the traces
//! (rsvg) blend encoded values. For dark ink on a light fill that makes
//! every partly covered edge pixel lighter, so text reads thinner than the
//! trace. Text drawn through the glyph atlas has that corrected in the
//! glyph shader (`vendor/cryoglyph`, `src/ink.rs`). Text drawn as filled
//! glyph *outlines* -- anything under a non-uniform transform (a
//! stretched run, a turned label) and the scene's cached outline runs --
//! is resolved from MSAA samples holding ink and fill together, where no
//! shader can tell which is the ink. There the outline itself is grown:
//! after filling a glyph, its contours are also stroked, moving every
//! edge outward by half the stroke width.
//!
//! # How much
//!
//! The thinning lives entirely in the antialiased edge, which is about
//! one *device* pixel wide whatever the text size or the scene scale. On
//! a pixel whose ink and fill mix at coverage `a`, the encoded blend
//! reaches the ink/fill midpoint at `a = 0.5`; the linear blend reaches it
//! only at the coverage [`midpoint_coverage`] returns (0.80 for black ink
//! on [`ASSUMED_FILL`]), so each edge reads as if it sat
//! `midpoint_coverage - 0.5` of an edge ramp further in. The stroke width
//! is therefore [`GAIN`] times that shift, in device pixels -- *not* in
//! design pixels: at 4K the stems are 2.4 times wider but the edge is not,
//! so a width that scaled with the scene would overshoot by 2.4x there.
//! The canvas frame is in logical pixels, which are device pixels at
//! output scale 1 (the golden matrix and terra's 3840x2160 output both).
//!
//! `GAIN` is fitted to the traces, not derived. Measured 2026-11-13 as ink
//! mass (encoded coverage summed over the run) of all 27 dark-on-fill
//! outline runs in the golden matrix against the same SVG traces
//! rasterised by rsvg, at both sizes: undarkened, the runs sit at a median
//! 0.92 of their trace at 1600x900 and 0.96 at 3840x2160. With `GAIN` 0.5
//! the medians are 0.99 and 0.97; with 1.0 they are 1.05 and 0.99, and at
//! 1600x900 eleven runs then overshoot their trace by more than 5% (up to
//! 20%), against four (up to 8%) at 0.5. Overshooting is the worse error,
//! so 0.5. Individual runs at 1600x900 scatter by +-15% around the median
//! because a stem one or two pixels wide loses more or less to the blend
//! depending on where it falls on the pixel grid; at 4K that scatter is
//! a few percent.
//!
//! # Which ink
//!
//! Only ink on a fill is darkened, and the outline route cannot see what
//! is under it. The ink's role can say: the palette's ground roles
//! (`Fg`, `Dim`, `Mid`, `Alert` ...) are drawn on the era's dark ground
//! and are never touched -- Neomil's dim red `#a32226` (Y 0.0907) is
//! `Dim`. `OnSelect` sits on the selection fill by definition. A
//! [`Ink::Fixed`] colour carries no role, so for it (and for `OnSelect`,
//! in case a custom palette makes it light) the ink's luminance decides,
//! with a smooth window: full up to [`FULL_Y`], fading to exactly zero at
//! [`OFF_Y`]. The window sits between the lightest fixed ink this route
//! draws on a fill (Neo-kitsch's `#895f3b`, Y 0.138) and the darkest fixed
//! ink it draws on dark ground (Neomil's `#f83333`, Y 0.226). It is wider
//! than the glyph shader's (`cryoglyph::ink::OFF_Y` 0.085) because the
//! shader has no role to exclude `#a32226` with; and the shift itself
//! already falls smoothly towards zero as the ink lightens.
//!
//! Light ink, ground roles and everything with zero width take the
//! original code path, so their pixels are unchanged.

use crate::style::Ink;
use cryoglyph::ink::{linear_to_srgb, luminance, srgb_to_linear, ASSUMED_FILL};
use iced::widget::canvas;
use iced::Color;

/// Stroke width, in device pixels, per unit of edge shift
/// ([`midpoint_coverage`] - 0.5): 0.15 px for black ink, 0.10 px for
/// Entropism's `#1f2a1c`. An analytic one-pixel linear ramp would want 2
/// (half the stroke on each side of an edge); the measured MSAA edge wants
/// far less (module docs).
pub const GAIN: f32 = 0.5;
/// Ink luminance (linear Y) up to which a fixed or on-select ink is
/// darkened in full.
pub const FULL_Y: f32 = 0.15;
/// Ink luminance (linear Y) from which it is not darkened at all.
pub const OFF_Y: f32 = 0.20;

/// The coverage at which a linear blend of ink of luminance `y` over
/// [`ASSUMED_FILL`] lands where an encoded blend at coverage 0.5 does:
/// the ink/fill midpoint the eye (and the coverage metric) reads as the
/// glyph's edge. 0.5 for ink as light as the fill; above it for darker
/// ink. The same expression as `cryoglyph::ink::dark_ink_coverage` at
/// `a = 0.5`, without its luminance window.
pub fn midpoint_coverage(y: f32) -> f32 {
    let i = linear_to_srgb(y);
    let fill = srgb_to_linear(ASSUMED_FILL);
    if fill - y <= 1e-6 {
        return 0.5;
    }
    let target = srgb_to_linear(0.5 * i + 0.5 * ASSUMED_FILL);
    ((fill - target) / (fill - y)).clamp(0.5, 1.0)
}

/// 1 up to [`FULL_Y`], 0 from [`OFF_Y`], a smoothstep between.
pub fn window(y: f32) -> f32 {
    let t = ((y - FULL_Y) / (OFF_Y - FULL_Y)).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
}

/// Whether ink of this role can sit on a fill: `OnSelect`, or a fixed
/// colour whose luminance then decides. Every other role is the
/// palette's ink for the era's ground, or a fill colour.
pub fn may_sit_on_fill(role: Ink) -> bool {
    matches!(role, Ink::OnSelect | Ink::Fixed(_))
}

/// The stroke width, in canvas (device) pixels, to grow outline text of
/// `role` resolved to `color` by. Zero for anything not darkened.
///
/// Scaled by the colour's alpha so a fading run thins continuously
/// rather than dropping its stroke at some opacity; a translucent run
/// composites its stroke over its fill, which at that width and opacity
/// is a sub-level difference.
pub fn width(role: Ink, color: Color) -> f32 {
    if !may_sit_on_fill(role) {
        return 0.0;
    }
    let y = luminance([color.r, color.g, color.b].map(srgb_to_linear));
    let w = window(y);
    if w <= 0.0 {
        return 0.0;
    }
    GAIN * (midpoint_coverage(y) - 0.5) * w * color.a.clamp(0.0, 1.0)
}

/// The stroke that darkens a run: its own ink, bevel joins so a sharp
/// glyph corner grows no miter spike.
pub fn stroke(color: Color, width: f32) -> canvas::Stroke<'static> {
    canvas::Stroke {
        style: canvas::Style::Solid(color),
        width,
        line_cap: canvas::LineCap::Butt,
        line_join: canvas::LineJoin::Bevel,
        ..canvas::Stroke::default()
    }
}

/// Fill `text` as glyph outlines and, when `width` is positive, stroke
/// each outline in the ink by `width`.
///
/// With `width` zero this is exactly `frame.fill_text(text)`. With it
/// positive it does what `fill_text` does under a non-uniform or
/// rotated transform -- `draw_with`, filling each path -- plus the
/// stroke, so the caller must only pass a positive width where the
/// frame's transform already sends the text down that route; under a
/// uniform transform `fill_text` would have used the glyph atlas.
pub fn fill_text(frame: &mut canvas::Frame, text: canvas::Text, width: f32) {
    if width <= 0.0 {
        frame.fill_text(text);
        return;
    }
    let ink = text.color;
    let darken = stroke(ink, width);
    text.draw_with(|path, color| {
        frame.fill(&path, color);
        // A raster glyph (no outline) comes back in its own colours;
        // only outlines painted in the run's ink are grown.
        if color == ink {
            frame.stroke(&path, darken);
        }
    });
}

/// Stroke already-filled outline `paths` by `width` in `color`; nothing
/// when `width` is zero. For the scene's cached outline runs.
pub fn stroke_paths(frame: &mut canvas::Frame, paths: &[canvas::Path], color: Color, width: f32) {
    if width <= 0.0 {
        return;
    }
    let darken = stroke(color, width);
    for path in paths {
        frame.stroke(path, darken);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(v: u32) -> Color {
        Color::from_rgb8((v >> 16) as u8, (v >> 8) as u8, v as u8)
    }

    fn y_of(c: Color) -> f32 {
        luminance([c.r, c.g, c.b].map(srgb_to_linear))
    }

    /// Every dark ink the outline route draws on a fill, as of the
    /// 2026-11-13 survey of the 25 golden cases.
    const ON_FILL: [u32; 11] = [
        0x0e2b2a, 0x1f2a1c, 0x22301f, 0x0f3a33, 0x4a3a05, 0x5a3a08, 0x35462e, 0x2e4a2c, 0x6b3f1e,
        0x7b5438, 0x895f3b,
    ];

    #[test]
    fn every_on_fill_ink_is_darkened() {
        for v in ON_FILL {
            let c = hex(v);
            assert!(width(Ink::Fixed(c), c) > 0.0, "#{v:06x}");
            assert_eq!(window(y_of(c)), 1.0, "#{v:06x} should get the full window");
            assert!(width(Ink::OnSelect, c) > 0.0, "#{v:06x}");
        }
    }

    #[test]
    fn ground_roles_are_never_darkened() {
        // Neomil's dim red is darker than two on-fill inks; its role
        // is what keeps it out.
        let dim = hex(0xa32226);
        assert!(y_of(dim) < y_of(hex(0x7b5438)));
        for role in [Ink::Dim, Ink::Fg, Ink::Mid, Ink::Border, Ink::Alert, Ink::Bg, Ink::Tape,
            Ink::Select, Ink::Ornament, Ink::Relief, Ink::Banner, Ink::Emphasis, Ink::Cta,
            Ink::Inset, Ink::None]
        {
            for c in [dim, hex(0x000000), hex(0x1f2a1c)] {
                assert_eq!(width(role, c), 0.0, "{role:?}");
            }
        }
    }

    #[test]
    fn light_fixed_inks_are_not_darkened() {
        // The darkest fixed inks the route draws on dark ground (Neomil's
        // reds), and light text generally.
        for v in [0xf63333, 0xf83333, 0xf93333, 0xfb3535, 0xef3333, 0xacddb4, 0x7cffe5, 0xffffff] {
            let c = hex(v);
            assert!(y_of(c) >= OFF_Y, "#{v:06x}");
            assert_eq!(width(Ink::Fixed(c), c), 0.0, "#{v:06x}");
            assert_eq!(width(Ink::OnSelect, c), 0.0, "#{v:06x}");
        }
    }

    #[test]
    fn width_is_smooth_and_falls_as_ink_lightens() {
        let mut last = f32::INFINITY;
        for n in 0..=255u8 {
            let c = Color::from_rgb8(n, n, n);
            let w = width(Ink::Fixed(c), c);
            assert!(w <= last + 1e-6, "grey {n}: {w} > {last}");
            if last.is_finite() {
                // Neighbouring 8-bit greys differ by well under a
                // hundredth of a pixel: no threshold.
                assert!(last - w < 0.01, "grey {n}: step {}", last - w);
            }
            last = w;
        }
        assert_eq!(window(FULL_Y), 1.0);
        assert_eq!(window(OFF_Y), 0.0);
    }

    #[test]
    fn black_width_matches_the_encoded_midpoint() {
        // For black ink the midpoint is the exact encoded blend on the
        // fill: 1 - L(0.39) / L(0.78) ~ 0.80, i.e. a 0.3 coverage shift.
        let m = midpoint_coverage(0.0);
        let want = 1.0 - srgb_to_linear(0.5 * ASSUMED_FILL) / srgb_to_linear(ASSUMED_FILL);
        assert!((m - want).abs() < 1e-6);
        let w = width(Ink::Fixed(Color::BLACK), Color::BLACK);
        assert!((w - GAIN * (want - 0.5)).abs() < 1e-6);
        // And the width is a constant in device pixels: nothing in it
        // depends on text size or the scene's scale.
        assert!(w > 0.1 && w < 0.2, "{w}");
    }

    #[test]
    fn width_fades_with_alpha() {
        let c = hex(0x1f2a1c);
        let full = width(Ink::Fixed(c), c);
        let half = width(Ink::Fixed(c), Color { a: 0.5, ..c });
        assert!((half - 0.5 * full).abs() < 1e-6);
        assert_eq!(width(Ink::Fixed(c), Color { a: 0.0, ..c }), 0.0);
    }
}
