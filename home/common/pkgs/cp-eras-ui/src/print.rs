//! An era's print material: the surface the reference art has and crisp
//! vector drawing does not -- registration copies, scanlines, grain.
//!
//! Owner decision 6 (`docs/review-2026-11-13/DECISIONS.md`): one shared pass
//! per era over the whole screen, tuned by eye against the reference, able to
//! animate later, instead of fitting each echo and texture patch element by
//! element. This module is the era-side description (plain numbers in the
//! 1600x900 design frame, no toolkit types) and the CPU mirror of the
//! pattern; the pass itself runs on the GPU over the finished frame
//! (`vendor/iced_wgpu/src/print.rs`, `shader/print.wgsl`).
//!
//! What the numbers are measured from is in `docs/print-overlay.md`. In
//! short, for both Neomil and Entropism the reference's faint copies of every
//! bright element are offset *outward from the screen centre* in proportion
//! to the distance from it, i.e. the frame scaled about its centre by about
//! 0.55 %, which is a whole-screen effect a post-process can draw; the copies'
//! rows are scanlined; clear ground and the large flat fills carry no
//! scanlines; Entropism's light fills carry grain.
//!
//! Every strength zero ([`Print::OFF`]) and the pass does not run at all: the
//! frame is drawn exactly as before. Kitsch and Neo-kitsch are OFF.

use crate::style::Era;

/// One era's print material, in design pixels of the 1600x900 frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Print {
    /// Registration copies: the frame scaled about its centre by `1 + k`
    /// (near copy) and `1 + 2k` (far copy), lighten-composited.
    pub echo_scale: f32,
    /// Opacity of the near and the far copy.
    pub echo_alpha: (f32, f32),
    /// Row modulation of the copies: `1 + depth * cos(2pi y / pitch)`.
    pub echo_scan_depth: f32,
    pub echo_scan_pitch: f32,
    /// Encoded luminance, averaged on a ring of `echo_gate_radius`, at which
    /// copies begin and finish being suppressed: inside a light fill a copy
    /// of the fill would otherwise pale the dark ink printed on it.
    pub echo_gate: (f32, f32),
    pub echo_gate_radius: f32,
    /// Softness of the copies (design px): the spread of a small box blur.
    pub echo_soft: f32,
    /// Fill scanlines over everything: darkening at the trough, pitch.
    pub scan_depth: f32,
    pub scan_pitch: f32,
    /// Grain: standard deviation relative to the colour, plus an absolute
    /// floor (encoded 0..1), and the size of one grain cell.
    pub grain_mul: f32,
    pub grain_add: f32,
    pub grain_cell: f32,
}

impl Print {
    /// No material. The pass is skipped and the frame is unchanged.
    pub const OFF: Print = Print {
        echo_scale: 0.0,
        echo_alpha: (0.0, 0.0),
        echo_scan_depth: 0.0,
        echo_scan_pitch: 2.0,
        echo_gate: (1.0, 1.0),
        echo_gate_radius: 6.0,
        echo_soft: 0.0,
        scan_depth: 0.0,
        scan_pitch: 2.0,
        grain_mul: 0.0,
        grain_add: 0.0,
        grain_cell: 1.0,
    };

    /// Whether this material changes no pixel.
    pub fn is_off(&self) -> bool {
        let echo = self.echo_scale > 0.0 && (self.echo_alpha.0 > 0.0 || self.echo_alpha.1 > 0.0);
        !(echo || self.scan_depth > 0.0 || self.grain_mul > 0.0 || self.grain_add > 0.0)
    }

    /// The material at `strength` times its fitted strength: opacities,
    /// scanline depth and grain scale; shapes (scale, pitches, gate) do not.
    pub fn at(&self, strength: f32) -> Print {
        let s = strength.max(0.0);
        Print {
            echo_alpha: ((self.echo_alpha.0 * s).min(1.0), (self.echo_alpha.1 * s).min(1.0)),
            scan_depth: (self.scan_depth * s).min(1.0),
            grain_mul: self.grain_mul * s,
            grain_add: self.grain_add * s,
            ..*self
        }
    }

    /// The renderer's parameters, with the moment's phases and seed. A
    /// later animation varies only these three arguments.
    pub fn params(&self, echo_phase: f32, scan_phase: f32, seed: i32) -> iced_wgpu::print::Params {
        iced_wgpu::print::Params {
            frame: crate::screens::scene::FRAME,
            echo_scale: self.echo_scale,
            echo_alpha: self.echo_alpha,
            echo_scan_depth: self.echo_scan_depth,
            echo_scan_pitch: self.echo_scan_pitch,
            echo_phase,
            echo_gate: self.echo_gate,
            echo_gate_radius: self.echo_gate_radius,
            echo_soft: self.echo_soft,
            scan_depth: self.scan_depth,
            scan_pitch: self.scan_pitch,
            scan_phase,
            grain_mul: self.grain_mul,
            grain_add: self.grain_add,
            grain_cell: self.grain_cell,
            grain_seed: seed,
        }
    }

    /// Apply `key=value` overrides separated by `;` (pairs as `a,b`). For
    /// tuning by eye without a rebuild; unknown keys are ignored.
    pub fn with_overrides(mut self, spec: &str) -> Print {
        for item in spec.split(';') {
            let Some((key, value)) = item.split_once('=') else { continue };
            let nums: Vec<f32> = value.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            let one = nums.first().copied();
            let pair = (nums.len() == 2).then(|| (nums[0], nums[1]));
            match (key.trim(), one, pair) {
                ("echo_scale", Some(v), _) => self.echo_scale = v,
                ("echo_alpha", _, Some(p)) => self.echo_alpha = p,
                ("echo_scan_depth", Some(v), _) => self.echo_scan_depth = v,
                ("echo_scan_pitch", Some(v), _) => self.echo_scan_pitch = v,
                ("echo_gate", _, Some(p)) => self.echo_gate = p,
                ("echo_gate_radius", Some(v), _) => self.echo_gate_radius = v,
                ("echo_soft", Some(v), _) => self.echo_soft = v,
                ("scan_depth", Some(v), _) => self.scan_depth = v,
                ("scan_pitch", Some(v), _) => self.scan_pitch = v,
                ("grain_mul", Some(v), _) => self.grain_mul = v,
                ("grain_add", Some(v), _) => self.grain_add = v,
                ("grain_cell", Some(v), _) => self.grain_cell = v,
                _ => {}
            }
        }
        self
    }
}

impl Default for Print {
    fn default() -> Self {
        Print::OFF
    }
}

/// The era's material as its tables state it.
/// The strength [`wear`] applies when `CP_ERAS_UI_PRINT` is unset. See the
/// note in `wear`.
pub const DEFAULT_STRENGTH: f32 = 0.0;

pub fn of(era: Era) -> Print {
    match era {
        Era::Neomil => crate::eras::neomil::PRINT,
        Era::Entropism => crate::eras::entropism::PRINT,
        Era::Kitsch | Era::Neokitsch => Print::OFF,
    }
}

/// State `era`'s material to the renderer for every later frame of this
/// process. Called by the four reference-screen binaries; the bar and the
/// mail client never call it, so they never get the pass.
///
/// Two capture/tuning variables, read here only:
/// - `CP_ERAS_UI_PRINT=<strength>` scales the fitted strengths (0 turns the
///   pass off; 1 is the fit; the strength ladder uses 0.5 and 1.5).
/// - `CP_ERAS_UI_PRINT_SET=<key=value;...>`: [`Print::with_overrides`].
pub fn wear(era: Era) {
    let mut print = of(era);
    if let Ok(spec) = std::env::var("CP_ERAS_UI_PRINT_SET") {
        print = print.with_overrides(&spec);
    }
    // DEFAULT_STRENGTH is 0 while the look awaits the owner's review (2026-11-13,
    // todo/review-2026-11-13.md section 4): the pass is built and fitted but off,
    // so every golden is unchanged. Set it to 1.0 to wear the fit by default.
    let strength = std::env::var("CP_ERAS_UI_PRINT").ok().and_then(|v| v.trim().parse::<f32>().ok()).unwrap_or(DEFAULT_STRENGTH);
    let print = print.at(strength);
    iced_wgpu::print::set((!print.is_off()).then(|| print.params(0.0, 0.0, 0)));
}

// --- CPU mirror of `shader/print.wgsl` (the parts with numbers in them) ---

/// The copies' and fill scanlines' row cosine, at device row `y` (pixel
/// centre), device pitch and phase in cycles. Mirrors `scan` in the shader.
pub fn scan(y: f32, pitch: f32, phase: f32) -> f32 {
    (std::f32::consts::TAU * (y / pitch + phase)).cos()
}

/// Darkening factor of the fill scanlines at row `y`: 1 at a crest,
/// `1 - depth` at a trough. Mirrors term 2 of the shader.
pub fn fill_scan(y: f32, pitch: f32, phase: f32, depth: f32) -> f32 {
    1.0 - depth * (0.5 - 0.5 * scan(y, pitch, phase))
}

/// lowbias32, the shader's integer hash.
pub fn hash(x: u32) -> u32 {
    let mut h = x;
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846ca68b);
    h ^= h >> 16;
    h
}

/// The shader's lattice value in [-1, 1] for cell (x, y), channel c, seed s.
pub fn lattice(x: i32, y: i32, c: u32, s: u32) -> f32 {
    let h = hash((x as u32).wrapping_mul(0x9e3779b1) ^ hash((y as u32) ^ c.wrapping_mul(0x85ebca6b) ^ s));
    h as f32 / 2147483647.5 - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_is_identity_and_default() {
        assert!(Print::OFF.is_off());
        assert_eq!(Print::default(), Print::OFF);
        assert!(of(Era::Kitsch).is_off());
        assert!(of(Era::Neokitsch).is_off());
        // OFF converts to renderer parameters the renderer also skips.
        assert!(Print::OFF.params(0.3, 0.7, 9).is_off());
        // Zero strength is OFF for every era, whatever its fit.
        for era in Era::ALL {
            assert!(of(era).at(0.0).is_off(), "{era:?}");
        }
    }

    #[test]
    fn fitted_eras_are_on() {
        assert!(!of(Era::Neomil).is_off());
        assert!(!of(Era::Entropism).is_off());
        let n = of(Era::Neomil);
        assert_eq!(n.at(1.0), n);
        assert!((n.at(0.5).echo_alpha.0 - n.echo_alpha.0 * 0.5).abs() < 1e-6);
        assert_eq!(n.at(2.0).echo_scale, n.echo_scale);
    }

    #[test]
    fn row_pattern_is_periodic_at_its_pitch() {
        for &pitch in &[2.0f32, 4.764, 7.94] {
            for i in 0..50 {
                let y = 0.5 + i as f32 * 1.37;
                let a = scan(y, pitch, 0.25);
                let b = scan(y + pitch, pitch, 0.25);
                assert!((a - b).abs() < 1e-3, "pitch {pitch} y {y}");
                let f = fill_scan(y, pitch, 0.0, 0.3);
                assert!((0.7 - 1e-6..=1.0 + 1e-6).contains(&f));
            }
            // Half a period away it is the opposite phase.
            assert!((scan(0.0, pitch, 0.0) + scan(pitch / 2.0, pitch, 0.0)).abs() < 1e-3);
        }
        // At the 1600x900 design size the 1.985 px pitch snaps to 2 device
        // pixels: rows alternate exactly, with no moire beat.
        let p = iced_wgpu::print::device_pitch(1.985, 1.0);
        assert_eq!(p, 2.0);
        for row in 0..400 {
            let y = row as f32 + 0.5;
            assert!((scan(y, p, 0.0) - scan(y + 2.0, p, 0.0)).abs() < 1e-3);
        }
    }

    #[test]
    fn grain_lattice_is_bounded_centred_and_seeded() {
        let mut sum = 0.0f64;
        let mut sq = 0.0f64;
        let n = 64 * 64;
        for y in 0..64 {
            for x in 0..64 {
                let v = lattice(x, y, 0, 0);
                assert!((-1.0..=1.0).contains(&v));
                sum += v as f64;
                sq += (v * v) as f64;
            }
        }
        let mean = sum / n as f64;
        let std = (sq / n as f64 - mean * mean).sqrt();
        assert!(mean.abs() < 0.03, "{mean}");
        assert!((std - 0.577).abs() < 0.03, "{std}");
        assert_ne!(lattice(3, 4, 0, 0), lattice(3, 4, 0, 1));
        assert_ne!(lattice(3, 4, 0, 0), lattice(3, 4, 1, 0));
    }

    #[test]
    fn shader_mirror_constants_match() {
        let wgsl = include_str!("../vendor/iced_wgpu/src/shader/print.wgsl");
        for k in ["0x7feb352du", "0x846ca68bu", "0x9e3779b1u", "0x85ebca6bu", "2147483647.5"] {
            assert!(wgsl.contains(k), "{k} missing from print.wgsl");
        }
    }

    #[test]
    fn overrides_parse() {
        let p = Print::OFF.with_overrides("echo_scale=0.01; echo_alpha=0.3,0.2;bogus=1;scan_depth=x");
        assert_eq!(p.echo_scale, 0.01);
        assert_eq!(p.echo_alpha, (0.3, 0.2));
        assert_eq!(p.scan_depth, 0.0);
    }
}
