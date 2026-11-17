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
//! Every strength zero ([`Print::OFF`], or `CP_ERAS_UI_PRINT=0`) and the
//! pass does not run at all: the frame is drawn exactly as without it. The
//! era-drawn copies the pass replaced (Neomil's dashboard, login and store
//! echoes) are gone, so with the pass off those screens show no copies.

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

/// An element that sits *above* the printed layer: laid after the print,
/// so no registration copy shows inside it. The references draw some
/// elements so -- Neomil's header badge plates (nothing of the label
/// printed on a plate shows doubled inside it), its inactive login cards
/// (their notices carry no copies, while mailbox rows of the same colour
/// do), Kitsch's BRAINDANCE body text (no outward copy, a hatched halo
/// instead) -- and no gate on the frame's colours can tell them from
/// elements that are printed (`docs/print-overlay.md`, "Above the print").
///
/// The outline is a rectangle with 45-degree corner cuts in design px,
/// the shape of a [`Plate`](crate::style::Plate) ([`AbovePrint::plate`]).
/// `cast`: whether the element is itself printed underneath, so it still
/// casts its own copies outward (a plate), or set over everything and casts
/// none (a text block, [`AbovePrint::block`]). `halo`: a hatched halo
/// hugging the light marks inside it ([`Halo`]).
///
/// A screen states its elements above the print with its material
/// ([`Print::above`]); the reference screens' lists are on the era tables
/// beside the elements they cover.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbovePrint {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    /// Corner cuts: top-left, top-right, bottom-right, bottom-left.
    pub bevel: [f32; 4],
    pub cast: bool,
    pub halo: Halo,
}

/// The halo an [`AbovePrint`] region wears around its light marks: per
/// channel, the fraction of the way from the ground to the mark's colour
/// at the mark's edge. It falls off as a Gaussian of the material's
/// `halo_sigma` and is hatched by its `hatch` at `halo_hatch` depth.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Halo {
    pub gain: [f32; 3],
}

impl Halo {
    pub const NONE: Halo = Halo { gain: [0.0; 3] };
}

impl AbovePrint {
    /// A plate laid above the print: it covers the copies of what is
    /// printed on it and casts its own copies outward.
    pub const fn plate(plate: &crate::style::Plate) -> AbovePrint {
        let b = plate.bevel;
        AbovePrint {
            x: plate.at.x,
            y: plate.at.y,
            w: plate.at.w,
            h: plate.at.h,
            bevel: [b.tl, b.tr, b.br, b.bl],
            cast: true,
            halo: Halo::NONE,
        }
    }

    /// A block (of text) set over everything: no copy shows inside it and
    /// none of it is copied outward.
    pub const fn block(x: f32, y: f32, w: f32, h: f32) -> AbovePrint {
        AbovePrint { x, y, w, h, bevel: [0.0; 4], cast: false, halo: Halo::NONE }
    }

    pub const fn with_halo(self, halo: Halo) -> AbovePrint {
        AbovePrint { halo, ..self }
    }

    fn region(&self) -> iced_wgpu::print::Over {
        iced_wgpu::print::Over {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            bevel: self.bevel,
            cast: self.cast,
            halo: self.halo.gain,
        }
    }
}

/// The bloom: a soft glow around the frame's light parts, screened over
/// the result (`docs/print-overlay.md`, "Bloom"). `gain` 0 is none.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bloom {
    /// Strength of the glow relative to the light part it comes from.
    pub gain: f32,
    /// Standard deviation of the glow, design px.
    pub sigma: f32,
    /// Encoded luminance over which a pixel becomes a source of glow:
    /// none below `.0`, all of its colour from `.1`.
    pub knee: (f32, f32),
    /// Per channel factor on the glow's colour: ones keep the mark's hue.
    pub tint: [f32; 3],
    /// Depth of the material's `hatch` on the glow (0: smooth).
    pub hatch: f32,
    /// How much of a light colour's grey part (`min(r, g, b)`) is taken
    /// away before it glows: 1 glows its chroma only.
    pub chroma: f32,
}

impl Bloom {
    pub const NONE: Bloom = Bloom { gain: 0.0, sigma: 2.0, knee: (0.3, 0.6), tint: [1.0; 3], hatch: 0.0, chroma: 0.0 };
}

/// A diagonal hatching: pitch (design px) and the angle of its normal in
/// degrees from +x, y down. Shared by the halo and the bloom.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hatch {
    pub pitch: f32,
    pub angle: f32,
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
    /// Softness of the farthest copy, if larger (0: `echo_soft`).
    pub echo_soft_far: f32,
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
    /// Enclosure gate: reach (design px) of the test for dark ink enclosed
    /// by a light fill, and the thickness step a fill must hold over three
    /// samples to count. Copies are suppressed through `echo_gate` as the
    /// enclosing fill's luminance rises. `0` reach is off.
    pub echo_enclose: f32,
    pub echo_enclose_thick: f32,
    /// Ground gate `(lo, hi, floor)`: a copy landing on a ground of encoded
    /// luminance `L` is laid at `mix(1, floor, smoothstep(lo, hi, L))` of its
    /// opacity. The references' copies of light marks are about 0.6 as
    /// strong over a mid-tone ground (the Kitsch tiles and pink field, the
    /// Neo-kitsch sky) as over a dark one, measured as a fraction of the
    /// mark's contrast against that ground. `floor` 1 is no gate.
    pub echo_ground: (f32, f32, f32),
    /// Chroma gate: copies fade in as the copied colour's saturation
    /// `(max - min) / max` rises from `.0` to `.1`. For a colour texture
    /// fitted on a saturated ink (Neomil's red): a neutral mark gets no
    /// copies the references never show. `(0, 0)` is no gate.
    pub echo_chroma: (f32, f32),
    /// Fill scanlines over everything: darkening at the trough, pitch.
    pub scan_depth: f32,
    pub scan_pitch: f32,
    /// Grain per channel: standard deviation relative to the colour, plus
    /// an absolute floor (encoded 0..1), and the size of one grain cell in
    /// design px. The noise is integrated over each device pixel, so its
    /// size and strength relative to the art hold at any output size.
    pub grain_mul: [f32; 3],
    pub grain_add: [f32; 3],
    pub grain_cell: f32,
    /// Elements above the printed layer on this screen ([`AbovePrint`]).
    /// At most [`MAX_ABOVE`]; more are dropped.
    pub above: &'static [AbovePrint],
    /// The above-print halo's Gaussian standard deviation, design px.
    pub halo_sigma: f32,
    /// Depth of `hatch` on the above-print halo.
    pub halo_hatch: f32,
    /// The hatching of the halo and the bloom.
    pub hatch: Hatch,
    pub bloom: Bloom,
}

/// How many [`AbovePrint`] elements a material carries.
pub const MAX_ABOVE: usize = iced_wgpu::print::MAX_OVER;

impl Print {
    /// No material. The pass is skipped and the frame is unchanged.
    pub const OFF: Print = Print {
        echo: [Echo::NONE; 3],
        echo_centre: (800.0, 450.0),
        echo_blend: Blend::Over,
        echo_soft: 0.0,
        echo_soft_far: 0.0,
        echo_lift: (0.0, 0.0),
        echo_texture: Texture::FLAT,
        echo_gate: (1.0, 1.0),
        echo_gate_radius: 6.0,
        echo_enclose: 0.0,
        echo_enclose_thick: 2.0,
        echo_ground: (0.0, 0.0, 1.0),
        echo_chroma: (0.0, 0.0),
        scan_depth: 0.0,
        scan_pitch: 2.0,
        grain_mul: [0.0; 3],
        grain_add: [0.0; 3],
        grain_cell: 1.0,
        above: &[],
        halo_sigma: 1.3,
        halo_hatch: 0.0,
        hatch: Hatch { pitch: 1.6, angle: -45.0 },
        bloom: Bloom::NONE,
    };

    /// Whether this material changes no pixel.
    pub fn is_off(&self) -> bool {
        let echo = self.echo.iter().any(Echo::is_live);
        let grain = self.grain_mul.iter().chain(&self.grain_add).any(|&v| v > 0.0);
        !(echo || self.scan_depth > 0.0 || grain || self.bloom.gain > 0.0)
    }

    /// This material with `above` as the screen's elements above the print.
    pub const fn above(self, above: &'static [AbovePrint]) -> Print {
        Print { above, ..self }
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
            bloom: Bloom { gain: self.bloom.gain * s, ..self.bloom },
            scan_depth: (self.scan_depth * s).min(1.0),
            grain_mul: self.grain_mul.map(|v| v * s),
            grain_add: self.grain_add.map(|v| v * s),
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
            echo_soft_far: self.echo_soft_far,
            echo_lift: self.echo_lift,
            echo_tex_pitch: t.pitch,
            echo_tex_angle: t.angle,
            echo_tex_crest: t.crest,
            echo_tex_trough: t.trough,
            echo_phase: t.phase + echo_phase,
            echo_gate: self.echo_gate,
            echo_gate_radius: self.echo_gate_radius,
            echo_enclose: self.echo_enclose,
            echo_enclose_thick: self.echo_enclose_thick,
            echo_ground: self.echo_ground,
            echo_chroma: self.echo_chroma,
            scan_depth: self.scan_depth,
            scan_pitch: self.scan_pitch,
            scan_phase,
            grain_mul: self.grain_mul,
            grain_add: self.grain_add,
            grain_cell: self.grain_cell,
            grain_seed: seed,
            over: {
                let mut over = [iced_wgpu::print::Over::NONE; MAX_ABOVE];
                for (slot, a) in over.iter_mut().zip(self.above) {
                    *slot = a.region();
                }
                over
            },
            halo_sigma: self.halo_sigma,
            hatch_pitch: self.hatch.pitch,
            hatch_angle: self.hatch.angle,
            halo_hatch: self.halo_hatch,
            bloom_gain: self.bloom.gain,
            bloom_sigma: self.bloom.sigma,
            bloom_knee: self.bloom.knee,
            bloom_tint: self.bloom.tint,
            bloom_hatch: self.bloom.hatch,
            bloom_chroma: self.bloom.chroma,
        }
    }

    /// Apply `key=value` overrides separated by `;` (lists as `a,b,c`). For
    /// tuning by eye without a rebuild; unknown keys are ignored. Keys:
    /// `echo1`/`echo2`/`echo3=kx,ky,gain`, `echo_centre=x,y`,
    /// `echo_blend=over|lighten`, `echo_soft`, `echo_lift=lo,hi`,
    /// `tex_pitch`, `tex_angle`, `tex_crest=r,g,b`, `tex_trough=r,g,b`,
    /// `tex_phase`,
    /// `echo_gate=lo,hi`, `echo_gate_radius`, `echo_ground=lo,hi,floor`,
    /// `echo_chroma=lo,hi`, `echo_soft_far`, `echo_enclose=reach[,thick]`,
    /// `scan_depth`, `scan_pitch`, `grain_mul=v|r,g,b`,
    /// `grain_add=v|r,g,b`, `grain_cell`; the bloom's `bloom=off` (gain 0:
    /// the bloom alone off, for measuring), `bloom_gain`, `bloom_sigma`,
    /// `bloom_knee=lo,hi`, `bloom_tint=r,g,b`, `bloom_hatch`; `hatch=pitch,angle`,
    /// `halo_sigma`, `halo_hatch`; `above=off` drops the screen's
    /// above-print elements.
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
                ("echo_soft_far", Some(v), ..) => self.echo_soft_far = v,
                ("echo_enclose", Some(v), ..) => self.echo_enclose = v,
                ("echo_enclose", _, Some((r, t)), _) => (self.echo_enclose, self.echo_enclose_thick) = (r, t),
                ("echo_lift", _, Some(p), _) => self.echo_lift = p,
                ("tex_pitch", Some(v), ..) => self.echo_texture.pitch = v,
                ("tex_angle", Some(v), ..) => self.echo_texture.angle = v,
                ("tex_crest", _, _, Some(d)) => self.echo_texture.crest = d,
                ("tex_trough", _, _, Some(d)) => self.echo_texture.trough = d,
                ("tex_phase", Some(v), ..) => self.echo_texture.phase = v,
                ("echo_gate", _, Some(p), _) => self.echo_gate = p,
                ("echo_gate_radius", Some(v), ..) => self.echo_gate_radius = v,
                ("echo_ground", _, _, Some([lo, hi, f])) => self.echo_ground = (lo, hi, f),
                ("echo_chroma", _, Some(p), _) => self.echo_chroma = p,
                ("scan_depth", Some(v), ..) => self.scan_depth = v,
                ("scan_pitch", Some(v), ..) => self.scan_pitch = v,
                ("grain_mul", Some(v), ..) => self.grain_mul = [v; 3],
                ("grain_mul", _, _, Some(d)) => self.grain_mul = d,
                ("grain_add", Some(v), ..) => self.grain_add = [v; 3],
                ("grain_add", _, _, Some(d)) => self.grain_add = d,
                ("grain_cell", Some(v), ..) => self.grain_cell = v,
                ("bloom", ..) if matches!(value.trim(), "off" | "0") => self.bloom.gain = 0.0,
                ("bloom_gain", Some(v), ..) => self.bloom.gain = v,
                ("bloom_sigma", Some(v), ..) => self.bloom.sigma = v,
                ("bloom_knee", _, Some(p), _) => self.bloom.knee = p,
                ("bloom_tint", Some(v), ..) => self.bloom.tint = [v; 3],
                ("bloom_tint", _, _, Some(d)) => self.bloom.tint = d,
                ("bloom_hatch", Some(v), ..) => self.bloom.hatch = v,
                ("bloom_chroma", Some(v), ..) => self.bloom.chroma = v,
                ("hatch", _, Some((p, a)), _) => self.hatch = Hatch { pitch: p, angle: a },
                ("halo_sigma", Some(v), ..) => self.halo_sigma = v,
                ("halo_hatch", Some(v), ..) => self.halo_hatch = v,
                ("above", ..) if value.trim() == "off" => self.above = &[],
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

/// The ground gate every era wears ([`Print::echo_ground`]): full opacity
/// over grounds darker than encoded luminance 0.22, `0.6` of it from 0.30
/// up. Measured 2026-11-15 as the copy-side minus mirror-side contrast of
/// copy 1 next to light marks, reference against our render, on 21 Kitsch
/// and 10 Neo-kitsch sites (`docs/print-overlay.md`, "Copies
/// over mid-tone grounds"). Light fills are handled by the fill gate.
pub const GROUND: (f32, f32, f32) = (0.22, 0.30, 0.6);

/// Reach of the enclosure gate every era wears ([`Print::echo_enclose`]),
/// design px: dark ink that a light fill encloses within this distance in
/// all eight directions keeps no copies, however large the ink (Neomil's
/// login emblem, about 70 dp across, is crisp black in the reference).
/// It replaces the 6 dp ring, which also cut the copies of a fill's own
/// edge short on the ground just outside it.
pub const ENCLOSE: f32 = 90.0;

/// Softness of the far copy for the glowing eras, design px (4 native px):
/// measured 2026-11-16 beside Neo-kitsch's ENTER bar (the copy side minus
/// the mirror side reaches 22-26 native px out where the near-copy-only
/// band ends at 16) and matching the earlier finding that the reference's
/// far copy keeps its strength beside fills but falls to 0.1-0.2 on thin
/// strokes. Neomil's two copies were measured equally soft (header rule).
pub const SOFT_FAR: f32 = 1.7;

/// The grain of Kitsch, Neo-kitsch and Entropism, which measure as one
/// grain (2026-11-16, 4K references, high-passed, per channel): std
/// `mul * v + add` with mul about (0.021, 0.024, 0.046) -- blue twice the
/// others -- and add about 0.5 levels (Entropism sage 3.5/4.8/7.1 on
/// 138/172/140, dark ground 0.7/0.65/0.56; Kitsch pink 3.4/1.8/4.2 on
/// 139/57/79, ENTER bar 3.7/5.0/10.5; Neo-kitsch sky 2.4/2.0/5.5 on
/// 91/68/107), neighbour correlation 0.45-0.6 at 4K. Values here are the
/// noise field's: the per-pixel integration lowers them about 0.9x at 4K.
pub const GRAIN_MUL: [f32; 3] = [0.0233, 0.0267, 0.0511];
pub const GRAIN_ADD: [f32; 3] = [0.0018; 3];
pub const GRAIN_CELL: f32 = 0.55;

/// The bloom shared by the glowing eras before their own gain, sigma and
/// hatch: a light colour (encoded luminance from 0.35, fully from 0.55)
/// glows with its chroma only -- the references' glows carry the mark's
/// hue, saturated (Kitsch's mint ENTER bar glows green and blue with no
/// red, Entropism's sage footer green alone, Neo-kitsch's amber bar red and
/// green with no blue). Measured 2026-11-16, `docs/print-overlay.md`, "Bloom".
pub const GLOW: Bloom = Bloom { gain: 0.0, sigma: 2.5, knee: (0.35, 0.55), tint: [1.0; 3], hatch: 0.0, chroma: 1.0 };

/// The strength [`wear`] applies when `CP_ERAS_UI_PRINT` is unset: the
/// fit itself. On for every era since the owner's decision of 2026-11-14
/// ("On for all eras"); `CP_ERAS_UI_PRINT=0` still turns the pass off.
pub const DEFAULT_STRENGTH: f32 = 1.0;

/// The era's material as its tables state it.

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
        Era::Kitsch => crate::eras::kitsch::print_for(screen),
        _ => of(era),
    }
}

/// State `era`'s material, as fitted on `screen`, for every later frame
/// of this process. As [`wear`], which it extends.
pub fn wear_screen(era: Era, screen: Screen) {
    wear_material(of_screen(era, screen));
}

/// State `era`'s material to the renderer for every later frame of this
/// process. The four reference-screen binaries wear their screen's fit
/// ([`wear_screen`]; the dashboard's hub re-wears it on each route), the
/// mail client the era's treatment for a new screen (this). The status bar
/// never calls it, so it never gets the pass: it is a live desktop surface
/// designed separately and draws its own rule echo.
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
    fn ground_and_chroma_gates_reach_the_shader() {
        for era in Era::ALL {
            let p = of(era);
            assert_eq!(p.echo_ground, GROUND, "{era:?}");
            let u = iced_wgpu::print::Uniforms::new(&p.params(0.0, 0.0, 0), iced_wgpu::core::Size::new(3840, 2160));
            assert_eq!(&u.u[10][1..4], &[GROUND.0, GROUND.1, GROUND.2], "{era:?}");
            assert_eq!((u.u[3][1], u.u[6][3]), p.echo_chroma, "{era:?}");
        }
        // Only Neomil's texture was fitted on one ink, so only it drops
        // copies of neutral marks.
        assert!(of(Era::Neomil).echo_chroma.1 > of(Era::Neomil).echo_chroma.0);
        for era in [Era::Kitsch, Era::Neokitsch, Era::Entropism] {
            assert_eq!(of(era).echo_chroma, (0.0, 0.0), "{era:?}");
        }
        // A floor of 1 is no gate; OFF has none.
        assert_eq!(Print::OFF.echo_ground.2, 1.0);
        let (lo, hi, floor) = GROUND;
        assert!(lo < hi && (0.0..1.0).contains(&floor));
    }

    #[test]
    fn enclosure_soft_far_and_grain_reach_the_shader() {
        for era in Era::ALL {
            let p = of(era);
            assert_eq!(p.echo_enclose, ENCLOSE, "{era:?}");
            // The ring gate is retired: the enclosure test replaces it.
            assert_eq!(p.echo_gate_radius, 0.0, "{era:?}");
            let u = iced_wgpu::print::Uniforms::new(&p.params(0.0, 0.0, 0), iced_wgpu::core::Size::new(3840, 2160));
            assert_eq!(u.u[7][2], 0.0, "{era:?}");
            assert!((u.u[11][0] - ENCLOSE * 2.4).abs() < 1e-3, "{era:?}");
            assert_eq!(&u.u[12][..3], &p.grain_mul, "{era:?}");
            assert_eq!(&u.u[13][..3], &p.grain_add, "{era:?}");
            // Grain cells are sub-pixel at 1600x900 and integrated, not
            // clamped up to a whole pixel.
            let small = iced_wgpu::print::Uniforms::new(&p.params(0.0, 0.0, 0), iced_wgpu::core::Size::new(1600, 900));
            assert!((small.u[9][3] - p.grain_cell).abs() < 1e-6, "{era:?}");
            assert!(small.u[11][3] >= 2.0, "{era:?}");
        }
        for era in [Era::Kitsch, Era::Neokitsch, Era::Entropism] {
            let p = of(era);
            assert_eq!((p.grain_mul, p.grain_add, p.echo_soft_far), (GRAIN_MUL, GRAIN_ADD, SOFT_FAR), "{era:?}");
        }
        // Neomil's grain is additive only, blue strongest.
        let n = of(Era::Neomil);
        assert_eq!(n.grain_mul, [0.0; 3]);
        assert!(n.grain_add[2] > n.grain_add[0]);
        assert_eq!(iced_wgpu::print::grain_taps(0.33), 4.0);
        assert_eq!(iced_wgpu::print::grain_taps(1.32), 2.0);
        assert_eq!(iced_wgpu::print::grain_taps(2.0), 1.0);
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
            "echo1=0.01,0.02,0.4; echo2=0.02,0.03,0.2;bogus=1;scan_depth=x;tex_crest=1,0.3,0.3;echo_blend=lighten;echo_soft=0.5;echo_ground=0.2,0.3,0.6;echo_chroma=0.25,0.45;grain_mul=0.01,0.02,0.04;grain_add=0.002;echo_enclose=50,1.5;echo_soft_far=1.7",
        );
        assert_eq!(p.echo_ground, (0.2, 0.3, 0.6));
        assert_eq!(p.echo_chroma, (0.25, 0.45));
        assert_eq!((p.grain_mul, p.grain_add), ([0.01, 0.02, 0.04], [0.002; 3]));
        assert_eq!((p.echo_enclose, p.echo_enclose_thick, p.echo_soft_far), (50.0, 1.5, 1.7));
        assert_eq!(p.echo[0], Echo { kx: 0.01, ky: 0.02, gain: 0.4 });
        assert_eq!(p.echo[1], Echo { kx: 0.02, ky: 0.03, gain: 0.2 });
        assert_eq!(p.echo_texture.crest, [1.0, 0.3, 0.3]);
        assert_eq!(p.echo_blend, Blend::Lighten);
        assert_eq!(p.echo_soft, 0.5);
        assert_eq!(p.scan_depth, 0.0);
    }
}

#[cfg(test)]
mod above_and_bloom_tests {
    use super::*;
    use iced_wgpu::core::Size;
    use iced_wgpu::print::Uniforms;

    fn uniforms(p: &Print, w: u32, h: u32) -> Uniforms {
        Uniforms::new(&p.params(0.0, 0.0, 0), Size::new(w, h))
    }

    #[test]
    fn print_shaders_parse() {
        for (name, src) in [
            ("print.wgsl", include_str!("../vendor/iced_wgpu/src/shader/print.wgsl")),
            ("bloom.wgsl", include_str!("../vendor/iced_wgpu/src/shader/bloom.wgsl")),
        ] {
            let module = naga::front::wgsl::parse_str(src).unwrap_or_else(|e| panic!("{name}: {}", e.emit_to_string(src)));
            let mut v = naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::empty());
            v.validate(&module).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        }
        // The uniform block the shader declares is the one Rust writes.
        let wgsl = include_str!("../vendor/iced_wgpu/src/shader/print.wgsl");
        assert!(wgsl.contains(&format!("array<vec4<f32>, {}>", 3 * MAX_ABOVE)));
        assert_eq!(std::mem::size_of::<Uniforms>(), (18 + 3 * MAX_ABOVE) * 16);
    }

    #[test]
    fn above_print_regions_reach_the_shader() {
        // Neomil's login: the five header badges and the two inactive cards.
        let login = of_screen(Era::Neomil, Screen::Login);
        assert_eq!(login.above.len(), 7);
        let u = uniforms(&login, 3840, 2160);
        assert_eq!(u.u[17][1], 7.0);
        assert!((u.u[17][2] - 2.4).abs() < 1e-6);
        let badge = login.above[0];
        assert_eq!((badge.x, badge.y, badge.w, badge.h), (117.0, 104.0, 59.0, 57.0));
        assert_eq!(badge.bevel, [0.0, 0.0, 0.0, 15.0]);
        assert_eq!(u.over[0], [117.0, 104.0, 176.0, 161.0]);
        assert_eq!(u.over[1], [0.0, 0.0, 0.0, 15.0]);
        // Plates cast their own copies and wear no halo.
        assert_eq!(u.over[2], [1.0, 0.0, 0.0, 0.0]);
        let card = login.above[5];
        assert_eq!((card.x, card.w, card.bevel), (692.0, 255.0, [0.0, 51.0, 0.0, 22.0]));
        // A region's geometry is the plate's own.
        let plate = crate::style::Plate::filled(crate::style::Plot::new(1.0, 2.0, 3.0, 4.0), crate::style::Ink::Fg)
            .bevelled(crate::style::Bevel::tr(0.5));
        let a = AbovePrint::plate(&plate);
        assert_eq!((a.x, a.y, a.w, a.h, a.bevel, a.cast), (1.0, 2.0, 3.0, 4.0, [0.0, 0.5, 0.0, 0.0], true));

        // Kitsch's dashboard: the BRAINDANCE body text casts nothing and
        // wears a halo.
        let dash = of_screen(Era::Kitsch, Screen::Dashboard);
        assert_eq!(dash.above.len(), 1);
        let u = uniforms(&dash, 1600, 900);
        assert_eq!(u.u[17][1], 1.0);
        assert_eq!(u.over[2][0], 0.0);
        assert!(u.over[2][1] > u.over[2][2] && u.over[2][2] > u.over[2][3] && u.over[2][3] > 0.0);
        assert!((u.u[17][0] - dash.halo_sigma).abs() < 1e-6);
        assert_eq!(u.u[16][3], dash.halo_hatch);

        // Only those two screens carry regions; the era's treatment for a
        // new screen carries none; `above=off` drops them.
        for era in Era::ALL {
            assert!(of(era).above.is_empty(), "{era:?}");
            for screen in [Screen::Login, Screen::Dashboard, Screen::Mailbox, Screen::Store] {
                let n = of_screen(era, screen).above.len();
                let expect = match (era, screen) {
                    (Era::Neomil, Screen::Login) => 7,
                    (Era::Kitsch, Screen::Dashboard) => 1,
                    _ => 0,
                };
                assert_eq!(n, expect, "{era:?} {screen:?}");
                assert!(n <= MAX_ABOVE);
            }
        }
        let off = login.with_overrides("above=off");
        assert!(off.above.is_empty());
        assert_eq!(uniforms(&off, 1600, 900).u[17][1], 0.0);
    }

    #[test]
    fn bloom_reaches_the_shader_and_turns_off_alone() {
        // Neomil measured no glow.
        assert_eq!(of(Era::Neomil).bloom.gain, 0.0);
        for era in [Era::Kitsch, Era::Neokitsch, Era::Entropism] {
            let p = of(era);
            assert!(p.bloom.gain > 0.0, "{era:?}");
            let u = uniforms(&p, 3840, 2160);
            assert_eq!(u.u[14][0], p.bloom.gain);
            // Sigma is in design px: 2.4 device px each at 4K, 1 at 1600.
            assert!((u.u[14][1] - p.bloom.sigma * 2.4).abs() < 1e-4, "{era:?}");
            assert!((uniforms(&p, 1600, 900).u[14][1] - p.bloom.sigma).abs() < 1e-5);
            assert_eq!((u.u[14][2], u.u[14][3]), p.bloom.knee);
            assert_eq!(&u.u[15][..3], &p.bloom.tint);
            assert_eq!(u.u[15][3], p.bloom.hatch);
            let rp = p.params(0.0, 0.0, 0);
            assert_eq!(rp.bloom_chroma, p.bloom.chroma);
            assert!(rp.has_bloom());
            // `bloom=off` removes the glow and nothing else.
            let off = p.with_overrides("bloom=off");
            assert_eq!(off.bloom.gain, 0.0);
            assert_eq!(Print { bloom: p.bloom, ..off }, p);
            assert_eq!(uniforms(&off, 3840, 2160).u[14][0], 0.0);
            // Strength scales the glow; zero strength is off.
            assert!((p.at(0.5).bloom.gain - 0.5 * p.bloom.gain).abs() < 1e-6);
            assert!(p.at(0.0).is_off());
        }
        let p = Print::OFF.with_overrides("bloom_gain=0.8;bloom_sigma=3;bloom_knee=0.3,0.5;bloom_tint=1,0.5,0;bloom_hatch=0.2;bloom_chroma=0.5;hatch=1.7,40;halo_sigma=1.1;halo_hatch=0.6");
        assert_eq!((p.bloom.gain, p.bloom.sigma, p.bloom.knee, p.bloom.tint, p.bloom.hatch, p.bloom.chroma), (0.8, 3.0, (0.3, 0.5), [1.0, 0.5, 0.0], 0.2, 0.5));
        assert_eq!((p.hatch.pitch, p.hatch.angle, p.halo_sigma, p.halo_hatch), (1.7, 40.0, 1.1, 0.6));
        // A material with only a bloom still runs the pass.
        assert!(!p.is_off() && !p.params(0.0, 0.0, 0).is_off());
        // The blur's taps cover three sigmas at half resolution.
        assert_eq!(iced_wgpu::print::bloom_taps(6.96), (3.48, 11.0));
    }
}

/// Frame cost of the print pass alone (`docs/print-overlay.md`, "Cost"),
/// on whatever adapter wgpu picks (lavapipe headless). Not run by default:
///
/// `CP_ERAS_UI_BENCH="era:screen:frame.png ..." cargo test --lib time_print_pass -- --ignored --nocapture`
///
/// Each frame (a capture of that screen, any size) is run with the era's
/// material as is, with `bloom=off`, and with no regions above the print;
/// best of 8 after 2 warm-up frames.
#[cfg(test)]
mod bench {
    use super::*;

    fn decode(path: &str) -> (u32, u32, Vec<u8>) {
        let decoder = png::Decoder::new(std::io::BufReader::new(std::fs::File::open(path).expect(path)));
        let mut reader = decoder.read_info().expect("png");
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).expect("frame");
        let rgba = match info.color_type {
            png::ColorType::Rgba => buf[..info.buffer_size()].to_vec(),
            png::ColorType::Rgb => buf[..info.buffer_size()].chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
            other => panic!("{other:?}"),
        };
        (info.width, info.height, rgba)
    }

    #[test]
    #[ignore]
    fn time_print_pass() {
        let Ok(spec) = std::env::var("CP_ERAS_UI_BENCH") else { return };
        use iced_wgpu::wgpu;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN, ..Default::default() });
        let adapter = futures_lite::future::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).expect("adapter");
        eprintln!("adapter: {:?}", adapter.get_info().name);
        let (device, queue) = futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");
        for item in spec.split_whitespace() {
            let mut parts = item.splitn(3, ':');
            let (era, screen, path) = (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap());
            let era = Era::ALL.into_iter().find(|e| format!("{e:?}").eq_ignore_ascii_case(era)).expect("era");
            let screen = match screen {
                "login" => Screen::Login,
                "dashboard" => Screen::Dashboard,
                "mailbox" => Screen::Mailbox,
                _ => Screen::Store,
            };
            let (w, h, pixels) = decode(path);
            let p = of_screen(era, screen);
            for (label, m) in [("as is", p), ("bloom=off", p.with_overrides("bloom=off")), ("above=off", p.with_overrides("above=off"))] {
                let mut t = iced_wgpu::print::bench(
                    &device,
                    &queue,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    wgpu::TextureFormat::Bgra8UnormSrgb,
                    &m.params(0.0, 0.0, 0),
                    iced_wgpu::core::Size::new(w, h),
                    &pixels,
                    10,
                );
                t.drain(..2);
                let best = t.iter().min().unwrap();
                eprintln!("{era:?} {screen:?} {w}x{h} {label}: best {:.1} ms", best.as_secs_f64() * 1e3);
            }
        }
    }
}
