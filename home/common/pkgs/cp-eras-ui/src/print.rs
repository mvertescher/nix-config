//! An era's print material: the surface the reference art has and crisp
//! vector drawing does not -- registration copies, scanlines, grain.
//!
//! Owner decision 6 (`docs/review-2026-11-13/DECISIONS.md`): one shared pass
//! per era over the whole screen, tuned against the reference, able to
//! animate later, instead of fitting each echo and texture patch element by
//! element. This module is the era-side description (plain numbers in the
//! 1600x900 design frame, no toolkit types) and the CPU mirror of the
//! pattern; the pass itself runs on the GPU over the finished frame
//! (`vendor/iced_wgpu/src/print.rs`, `shader/print.wgsl`).
//!
//! What the numbers are measured from is in `docs/print-overlay.md`. In
//! short, every era's reference has faint *registration copies* of its art:
//! two (not one, not three) copies of the whole frame, each scaled about a
//! point near the screen centre -- Neomil by 0.50 % and 0.92 % -- softly
//! blurred, composited *under* the art with normal alpha (where the two
//! overlap they combine as `1 - (1 - a1)(1 - a2)`, brighter than either,
//! less than their sum), and coloured by a fine texture (Neomil: rows at
//! 1.985 px that swing the copy between saturated and pale red). Clear
//! ground and the large flat fills carry no texture of their own.
//!
//! Every strength zero ([`Print::OFF`]) and the pass does not run at all: the
//! frame is drawn exactly as before.

use crate::style::Era;

pub use iced_wgpu::print::Blend;

/// One registration copy: the frame scaled about the material's centre by
/// `1 + kx` across and `1 + ky` down, laid at `gain` opacity. A point at
/// distance `d` from the centre is copied `k * d` farther out.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Echo {
    pub kx: f32,
    pub ky: f32,
    pub gain: f32,
}

impl Echo {
    pub const NONE: Echo = Echo { kx: 0.0, ky: 0.0, gain: 0.0 };

    /// Whether this copy draws anything.
    pub fn is_live(&self) -> bool {
        (self.kx != 0.0 || self.ky != 0.0) && self.gain > 0.0
    }
}

/// The copies' colour texture: `t = cos(2pi (d / pitch + phase))` with `d`
/// measured in design px along the direction `angle` degrees from +x (90:
/// it varies down the screen, i.e. rows). Each channel of a copy is scaled
/// by `mix(trough, crest, (1 + t) / 2)`. At phase 0 a crest lies on the line
/// `d = 0`. Equal crest and trough with all ones is no texture.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Texture {
    pub pitch: f32,
    pub angle: f32,
    pub crest: [f32; 3],
    pub trough: [f32; 3],
    pub phase: f32,
}

impl Texture {
    pub const FLAT: Texture = Texture { pitch: 2.0, angle: 90.0, crest: [1.0; 3], trough: [1.0; 3], phase: 0.0 };
}

/// The reference screens, for the per-screen fits of [`of_screen`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Login,
    Dashboard,
    Mailbox,
    Store,
}

/// One era's print material, in design pixels of the 1600x900 frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Print {
    /// Registration copies (up to three; a zero gain is no copy).
    pub echo: [Echo; 3],
    /// The point the copies are scaled about.
    pub echo_centre: (f32, f32),
    /// How each copy is laid down; overlapping copies follow from it.
    pub echo_blend: Blend,
    /// Softness of the copies: the standard deviation of their blur.
    pub echo_soft: f32,
    /// Lift gate (encoded luminance difference): a copy shows only where it
    /// is lighter than the pixel under it, i.e. under the art. `(0, 0)` off.
    pub echo_lift: (f32, f32),
    /// The copies' colour texture.
    pub echo_texture: Texture,
    /// Encoded luminance, averaged on a ring of `echo_gate_radius`, at which
    /// copies begin and finish being suppressed: inside a light fill a copy
    /// of the fill would otherwise pale the dark ink printed on it.
    pub echo_gate: (f32, f32),
    pub echo_gate_radius: f32,
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
        echo: [Echo::NONE; 3],
        echo_centre: (800.0, 450.0),
        echo_blend: Blend::Over,
        echo_soft: 0.0,
        echo_lift: (0.0, 0.0),
        echo_texture: Texture::FLAT,
        echo_gate: (1.0, 1.0),
        echo_gate_radius: 6.0,
        scan_depth: 0.0,
        scan_pitch: 2.0,
        grain_mul: 0.0,
        grain_add: 0.0,
        grain_cell: 1.0,
    };

    /// Whether this material changes no pixel.
    pub fn is_off(&self) -> bool {
        let echo = self.echo.iter().any(Echo::is_live);
        !(echo || self.scan_depth > 0.0 || self.grain_mul > 0.0 || self.grain_add > 0.0)
    }

    /// The material at `strength` times its fitted strength: copy gains,
    /// scanline depth and grain scale; shapes (scales, centre, pitches,
    /// texture, gates) do not.
    pub fn at(&self, strength: f32) -> Print {
        let s = strength.max(0.0);
        let mut echo = self.echo;
        for e in &mut echo {
            e.gain = (e.gain * s).min(1.0);
        }
        Print {
            echo,
            scan_depth: (self.scan_depth * s).min(1.0),
            grain_mul: self.grain_mul * s,
            grain_add: self.grain_add * s,
            ..*self
        }
    }

    /// The renderer's parameters, with the moment's phases and seed. A
    /// later animation varies only these three arguments; `echo_phase` adds
    /// to the texture's own phase.
    pub fn params(&self, echo_phase: f32, scan_phase: f32, seed: i32) -> iced_wgpu::print::Params {
        let t = self.echo_texture;
        iced_wgpu::print::Params {
            frame: crate::screens::scene::FRAME,
            echo: self.echo.map(|e| (e.kx, e.ky, e.gain)),
            echo_centre: self.echo_centre,
            echo_blend: self.echo_blend,
            echo_soft: self.echo_soft,
            echo_lift: self.echo_lift,
            echo_tex_pitch: t.pitch,
            echo_tex_angle: t.angle,
            echo_tex_crest: t.crest,
            echo_tex_trough: t.trough,
            echo_phase: t.phase + echo_phase,
            echo_gate: self.echo_gate,
            echo_gate_radius: self.echo_gate_radius,
            scan_depth: self.scan_depth,
            scan_pitch: self.scan_pitch,
            scan_phase,
            grain_mul: self.grain_mul,
            grain_add: self.grain_add,
            grain_cell: self.grain_cell,
            grain_seed: seed,
        }
    }

    /// Apply `key=value` overrides separated by `;` (lists as `a,b,c`). For
    /// tuning by eye without a rebuild; unknown keys are ignored. Keys:
    /// `echo1`/`echo2`/`echo3=kx,ky,gain`, `echo_centre=x,y`,
    /// `echo_blend=over|lighten`, `echo_soft`, `echo_lift=lo,hi`,
    /// `tex_pitch`, `tex_angle`, `tex_crest=r,g,b`, `tex_trough=r,g,b`,
    /// `tex_phase`,
    /// `echo_gate=lo,hi`, `echo_gate_radius`, `scan_depth`, `scan_pitch`,
    /// `grain_mul`, `grain_add`, `grain_cell`.
    pub fn with_overrides(mut self, spec: &str) -> Print {
        for item in spec.split(';') {
            let Some((key, value)) = item.split_once('=') else { continue };
            let nums: Vec<f32> = value.split(',').filter_map(|v| v.trim().parse().ok()).collect();
            let one = (nums.len() == 1).then(|| nums[0]);
            let pair = (nums.len() == 2).then(|| (nums[0], nums[1]));
            let triple = (nums.len() == 3).then(|| [nums[0], nums[1], nums[2]]);
            match (key.trim(), one, pair, triple) {
                ("echo1", _, _, Some([x, y, g])) => self.echo[0] = Echo { kx: x, ky: y, gain: g },
                ("echo2", _, _, Some([x, y, g])) => self.echo[1] = Echo { kx: x, ky: y, gain: g },
                ("echo3", _, _, Some([x, y, g])) => self.echo[2] = Echo { kx: x, ky: y, gain: g },
                ("echo_centre", _, Some(p), _) => self.echo_centre = p,
                ("echo_blend", ..) => match value.trim() {
                    "over" => self.echo_blend = Blend::Over,
                    "lighten" => self.echo_blend = Blend::Lighten,
                    _ => {}
                },
                ("echo_soft", Some(v), ..) => self.echo_soft = v,
                ("echo_lift", _, Some(p), _) => self.echo_lift = p,
                ("tex_pitch", Some(v), ..) => self.echo_texture.pitch = v,
                ("tex_angle", Some(v), ..) => self.echo_texture.angle = v,
                ("tex_crest", _, _, Some(d)) => self.echo_texture.crest = d,
                ("tex_trough", _, _, Some(d)) => self.echo_texture.trough = d,
                ("tex_phase", Some(v), ..) => self.echo_texture.phase = v,
                ("echo_gate", _, Some(p), _) => self.echo_gate = p,
                ("echo_gate_radius", Some(v), ..) => self.echo_gate_radius = v,
                ("scan_depth", Some(v), ..) => self.scan_depth = v,
                ("scan_pitch", Some(v), ..) => self.scan_pitch = v,
                ("grain_mul", Some(v), ..) => self.grain_mul = v,
                ("grain_add", Some(v), ..) => self.grain_add = v,
                ("grain_cell", Some(v), ..) => self.grain_cell = v,
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
        Era::Kitsch => crate::eras::kitsch::PRINT,
        Era::Neokitsch => crate::eras::neokitsch::PRINT,
    }
}

/// `era`'s material as measured on one reference screen. The references
/// were not all printed with the same registration: Neomil's dashboard copies
/// sit at 0.50 % / 0.93 % in both axes, its login, mailbox and store at
/// about 0.55-0.60 % across and 1.0-1.07 % down (copy 2 about twice that).
/// [`of`] is the era's treatment for a new screen; this is the per-screen
/// fit, for reproducing the reference screens themselves. Eras without a
/// per-screen fit return [`of`].
pub fn of_screen(era: Era, screen: Screen) -> Print {
    match era {
        Era::Neomil => crate::eras::neomil::print_for(screen),
        _ => of(era),
    }
}

/// State `era`'s material, as fitted on `screen`, for every later frame
/// of this process. As [`wear`], which it extends.
pub fn wear_screen(era: Era, screen: Screen) {
    wear_material(of_screen(era, screen));
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
    wear_material(of(era));
}

fn wear_material(mut print: Print) {
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
        // OFF converts to renderer parameters the renderer also skips.
        assert!(Print::OFF.params(0.3, 0.7, 9).is_off());
        // Zero strength is OFF for every era, whatever its fit.
        for era in Era::ALL {
            assert!(of(era).at(0.0).is_off(), "{era:?}");
        }
    }

    #[test]
    fn fitted_eras_are_on() {
        for era in Era::ALL {
            let p = of(era);
            assert!(!p.is_off(), "{era:?}");
            assert_eq!(p.at(1.0), p);
            assert!((p.at(0.5).echo[0].gain - p.echo[0].gain * 0.5).abs() < 1e-6);
            assert_eq!(p.at(2.0).echo[0].kx, p.echo[0].kx);
            assert_eq!(p.at(2.0).echo_texture, p.echo_texture);
        }
    }

    #[test]
    fn neomil_has_two_measured_copies() {
        for screen in [Screen::Login, Screen::Dashboard, Screen::Mailbox, Screen::Store] {
            let n = of_screen(Era::Neomil, screen);
            assert!(!n.is_off());
            assert_eq!(n.echo.iter().filter(|e| e.is_live()).count(), 2, "{screen:?}");
            assert_eq!(n.echo_blend, Blend::Over);
            // The far copy is the fainter one.
            assert!(n.echo[1].ky > n.echo[0].ky && n.echo[1].gain < n.echo[0].gain, "{screen:?}");
            assert_eq!(n.at(0.0).is_off(), true);
        }
        assert_eq!(of(Era::Neomil), of_screen(Era::Neomil, Screen::Mailbox));
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
        let p = Print::OFF.with_overrides(
            "echo1=0.01,0.02,0.4; echo2=0.02,0.03,0.2;bogus=1;scan_depth=x;tex_crest=1,0.3,0.3;echo_blend=lighten;echo_soft=0.5",
        );
        assert_eq!(p.echo[0], Echo { kx: 0.01, ky: 0.02, gain: 0.4 });
        assert_eq!(p.echo[1], Echo { kx: 0.02, ky: 0.03, gain: 0.2 });
        assert_eq!(p.echo_texture.crest, [1.0, 0.3, 0.3]);
        assert_eq!(p.echo_blend, Blend::Lighten);
        assert_eq!(p.echo_soft, 0.5);
        assert_eq!(p.scan_depth, 0.0);
    }
}
