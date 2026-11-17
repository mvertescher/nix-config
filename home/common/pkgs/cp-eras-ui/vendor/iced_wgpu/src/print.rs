//! Print pass (local patch; see `PATCHES.md`).
//!
//! An optional full-screen pass over the finished frame that gives it the
//! surface of the reference art: registration copies (up to three copies of
//! the frame scaled about a centre, blurred, colour-textured and composited
//! under the art), fill scanlines and grain. The application states the material once with
//! [`set`]; [`Renderer::present`](crate::Renderer::present) then draws the
//! frame into an offscreen target and runs `shader/print.wgsl` from it into
//! the surface. With no material set, or one whose strengths are all zero,
//! `present` takes the upstream path unchanged and nothing here runs.
//!
//! Lengths are in *design* pixels of a `frame` (1600x900 in cp-eras-ui) and
//! are converted to device pixels per axis at present time, so the pattern
//! keeps its size relative to the artwork at any window size.
//!
//! Animation is a matter of calling [`set`] with a new `scan_phase`,
//! `echo_phase` or `grain_seed` and requesting a redraw: nothing is cached on
//! those values.

use std::borrow::Cow;
use std::sync::RwLock;

use crate::core::Size;

/// How a registration copy is laid over the frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blend {
    /// Normal alpha compositing of the (textured) copy at its opacity, so
    /// overlapping copies combine as `1 - (1 - a1)(1 - a2)`: brighter than
    /// either alone, less than their sum. A copy can darken a channel (a
    /// saturated copy over a blue ground lowers its green and blue).
    Over,
    /// Per-channel lighten of the composited copy: never darkens.
    Lighten,
}

/// How many [`Over`] regions a material carries.
pub const MAX_OVER: usize = 12;

/// A region of the frame that sits *above* the printed layer: an element
/// laid after the print, so no registration copy shows inside it. Its
/// outline is a rectangle (design px) with 45-degree corner cuts, the
/// shape of a cp-eras plate.
///
/// `cast`: whether what lies in the region is itself printed -- a plate
/// that casts its own copies outward (true), or text set on top of
/// everything that casts none (false).
///
/// `halo`: a hatched halo hugging the light marks inside the region, per
/// channel the fraction of the way from the ground to the mark's colour
/// at the mark's edge, falling off as a Gaussian of the material's
/// `halo_sigma` (zero: none).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Over {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    /// Corner cuts, top-left, top-right, bottom-right, bottom-left.
    pub bevel: [f32; 4],
    pub cast: bool,
    pub halo: [f32; 3],
}

impl Over {
    pub const NONE: Over = Over { x: 0.0, y: 0.0, w: 0.0, h: 0.0, bevel: [0.0; 4], cast: true, halo: [0.0; 3] };

    pub fn is_live(&self) -> bool {
        self.w > 0.0 && self.h > 0.0
    }
}

/// One era's print material. See the module documentation for units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    /// The design frame the lengths below are measured in.
    pub frame: (f32, f32),
    /// Up to three registration copies, each `(kx, ky, opacity)`: the frame
    /// scaled by `1 + kx` across and `1 + ky` down about `echo_centre`. A
    /// zero scale or opacity is no copy. Composited farthest first.
    pub echo: [(f32, f32, f32); 3],
    /// The point the copies are scaled about (design px).
    pub echo_centre: (f32, f32),
    pub echo_blend: Blend,
    /// Softness of the copies: the standard deviation of their blur.
    pub echo_soft: f32,
    /// Softness of the farthest copy, when larger than `echo_soft` (0: the
    /// same). Above about 1.6 device px it is drawn as a 4x4 grid of taps
    /// instead of the 4-tap box, so it does not break into separate taps.
    pub echo_soft_far: f32,
    /// Lift gate: a copy shows only where it is lighter than the pixel
    /// under it, fading in over this encoded-luminance difference. This is
    /// what puts the copies *under* the art: ink is not covered by a copy
    /// of the ground, nor by a copy of other ink. `(0, 0)` disables it.
    pub echo_lift: (f32, f32),
    /// The copies' colour texture: `t = cos(2pi (d / pitch + phase))` along
    /// the direction `angle` degrees from +x (90 = rows); each channel of a
    /// copy is scaled by `mix(trough, crest, (1 + t) / 2)`.
    pub echo_tex_pitch: f32,
    pub echo_tex_angle: f32,
    pub echo_tex_crest: [f32; 3],
    pub echo_tex_trough: [f32; 3],
    /// Phase of the copies' texture, in cycles.
    pub echo_phase: f32,
    /// Fill gate: the encoded luminance, averaged on a ring of this radius,
    /// at which copies start (`.0`) and finish (`.1`) being suppressed.
    pub echo_gate: (f32, f32),
    pub echo_gate_radius: f32,
    /// Enclosure gate: the reach (design px) of a test for dark ink that a
    /// light fill encloses. Along each of eight directions the lightest
    /// *thick* sample (three samples `echo_enclose_thick` apart, all light)
    /// within this reach is found; the least of the eight is the pixel's
    /// enclosure, and copies are suppressed as it rises through
    /// `echo_gate` like the ring's luminance. Dark ink inside a light fill
    /// stays firm however large the ink is. `0` disables it.
    pub echo_enclose: f32,
    pub echo_enclose_thick: f32,
    /// Ground gate: `(lo, hi, floor)`. A copy's opacity is scaled by
    /// `mix(1, floor, smoothstep(lo, hi, L))`, `L` the encoded luminance of
    /// the pixel the copy lands on: copies of light marks are weaker over a
    /// mid-tone ground than over a dark one. `floor` 1 disables it.
    pub echo_ground: (f32, f32, f32),
    /// Chroma gate: a copy shows only where its own colour is saturated,
    /// fading in as `(max - min) / max` of the copied colour rises from
    /// `.0` to `.1`. For a colour texture fitted on one ink, so a neutral
    /// mark does not get copies the fit never saw. `(0, 0)` disables it.
    pub echo_chroma: (f32, f32),
    /// Fill scanlines: darkening at the trough (0 = none), pitch, phase in
    /// cycles.
    pub scan_depth: f32,
    pub scan_pitch: f32,
    pub scan_phase: f32,
    /// Grain: multiplicative and additive standard deviation per channel
    /// (encoded 0..1 units), cell size (design px), seed. The noise is
    /// integrated over each device pixel (up to 4x4 taps), so a cell
    /// smaller than a pixel averages down as a downsampled image's grain
    /// does, rather than being drawn at one pixel per cell.
    pub grain_mul: [f32; 3],
    pub grain_add: [f32; 3],
    pub grain_cell: f32,
    pub grain_seed: i32,
    /// Regions above the printed layer ([`Over`]); empty slots are
    /// [`Over::NONE`]. Only the live ones are sent to the shader.
    pub over: [Over; MAX_OVER],
    /// Standard deviation of an [`Over`] region's halo, design px.
    pub halo_sigma: f32,
    /// Hatching of the halo and the bloom: pitch (design px) and the angle
    /// of its normal (degrees from +x, y down). `halo_hatch` and
    /// `bloom_hatch` are each one's depth: the halo or glow is scaled by
    /// `1 + depth * cos(2pi d / pitch)` (0: solid).
    pub hatch_pitch: f32,
    pub hatch_angle: f32,
    pub halo_hatch: f32,
    /// Bloom: a soft glow of the frame's light parts, screened over the
    /// result. `bloom_gain` 0 is none. The glow is the frame weighted by
    /// `smoothstep(lo, hi, L)` of its encoded luminance (`bloom_knee`),
    /// blurred by a Gaussian of `bloom_sigma` design px, scaled per channel
    /// by `bloom_tint` (ones: the glow carries the mark's own colour), less
    /// `bloom_chroma` of its grey part.
    pub bloom_gain: f32,
    pub bloom_sigma: f32,
    pub bloom_knee: (f32, f32),
    pub bloom_tint: [f32; 3],
    pub bloom_hatch: f32,
    /// How much of a light colour's grey part (`min(r, g, b)`) is taken
    /// away before it glows: 0 the colour as it is, 1 its chroma only (a
    /// mint mark glows green-blue, a sage one green).
    pub bloom_chroma: f32,
}

impl Params {
    /// No material: every strength zero.
    pub const OFF: Self = Self {
        frame: (1600.0, 900.0),
        echo: [(0.0, 0.0, 0.0); 3],
        echo_centre: (800.0, 450.0),
        echo_blend: Blend::Over,
        echo_soft: 0.0,
        echo_soft_far: 0.0,
        echo_lift: (0.0, 0.0),
        echo_tex_pitch: 2.0,
        echo_tex_angle: 90.0,
        echo_tex_crest: [1.0; 3],
        echo_tex_trough: [1.0; 3],
        echo_phase: 0.0,
        echo_gate: (1.0, 1.0),
        echo_gate_radius: 6.0,
        echo_enclose: 0.0,
        echo_enclose_thick: 2.0,
        echo_ground: (0.0, 0.0, 1.0),
        echo_chroma: (0.0, 0.0),
        scan_depth: 0.0,
        scan_pitch: 2.0,
        scan_phase: 0.0,
        grain_mul: [0.0; 3],
        grain_add: [0.0; 3],
        grain_cell: 1.0,
        grain_seed: 0,
        over: [Over::NONE; MAX_OVER],
        halo_sigma: 1.3,
        hatch_pitch: 1.6,
        hatch_angle: -45.0,
        halo_hatch: 0.0,
        bloom_gain: 0.0,
        bloom_sigma: 2.0,
        bloom_knee: (0.3, 0.6),
        bloom_tint: [1.0; 3],
        bloom_hatch: 0.0,
        bloom_chroma: 0.0,
    };

    /// Whether the bloom draws.
    pub fn has_bloom(&self) -> bool {
        self.bloom_gain > 0.0 && self.bloom_sigma > 0.0
    }

    /// Whether any registration copy would draw.
    pub fn has_echo(&self) -> bool {
        self.echo.iter().any(|&(kx, ky, a)| (kx != 0.0 || ky != 0.0) && a > 0.0)
    }

    /// Whether the pass would change no pixel, so it is skipped.
    pub fn is_off(&self) -> bool {
        !(self.has_echo()
            || self.has_bloom()
            || self.scan_depth > 0.0
            || self.grain_mul.iter().any(|&v| v > 0.0)
            || self.grain_add.iter().any(|&v| v > 0.0))
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::OFF
    }
}

static PARAMS: RwLock<Option<Params>> = RwLock::new(None);

/// State the material for every later frame of this process. `None`, or a
/// material that [`Params::is_off`], restores the upstream present path.
pub fn set(params: Option<Params>) {
    let params = params.filter(|p| !p.is_off());
    if let Ok(mut slot) = PARAMS.write() {
        *slot = params;
    }
}

/// The material in effect, if any.
pub fn get() -> Option<Params> {
    PARAMS.read().ok().and_then(|p| *p)
}

/// A design-pixel pitch in device pixels at `scale` device pixels per
/// design pixel. Below three device pixels a fractional pitch beats
/// against the pixel grid into broad moire bands, so it snaps to a whole
/// number of pixels, never below two.
pub fn device_pitch(pitch: f32, scale: f32) -> f32 {
    let p = pitch * scale;
    if p < 3.0 {
        p.round().max(2.0)
    } else {
        p
    }
}

/// The copies with an effect, farthest (largest `|kx| + |ky|`) first,
/// padded with empty slots: the order the shader composites them in.
pub fn echo_order(echo: &[(f32, f32, f32); 3]) -> [(f32, f32, f32); 3] {
    let mut out = [(0.0, 0.0, 0.0); 3];
    let mut live: Vec<(f32, f32, f32)> = echo
        .iter()
        .copied()
        .filter(|&(kx, ky, a)| (kx != 0.0 || ky != 0.0) && a > 0.0)
        .collect();
    let reach = |c: &(f32, f32, f32)| c.0.abs() + c.1.abs();
    live.sort_by(|a, b| reach(b).total_cmp(&reach(a)));
    for (slot, c) in out.iter_mut().zip(live) {
        *slot = c;
    }
    out
}

/// The shader's uniform block (`shader/print.wgsl`): eighteen `vec4`s of
/// parameters, then three per [`Over`] region (live ones first).
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Uniforms {
    pub u: [[f32; 4]; 18],
    pub over: [[f32; 4]; 3 * MAX_OVER],
}

/// A design-space direction `angle` degrees from +x as a unit direction per
/// device pixel, and a design pitch along it in device pixels (snapped as
/// [`device_pitch`]).
pub fn device_direction(angle: f32, pitch: f32, sx: f32, sy: f32) -> (f32, f32, f32) {
    let (sa, ca) = angle.to_radians().sin_cos();
    let (dx, dy) = (ca / sx, sa / sy);
    let n = (dx * dx + dy * dy).sqrt().max(1e-9);
    (device_pitch(pitch, 1.0 / n), dx / n, dy / n)
}

/// The bloom blur's standard deviation in half-resolution pixels, and how
/// many taps either side of the centre (three sigmas, at most 24).
pub fn bloom_taps(sigma_device: f32) -> (f32, f32) {
    let h = (sigma_device * 0.5).max(0.25);
    (h, (3.0 * h).ceil().clamp(1.0, 24.0))
}

/// Taps per axis with which the grain is integrated over a device pixel,
/// for a grain cell of `cell` device px: enough that a sub-pixel cell
/// averages down (1 from 1.5 px up, at most 4).
pub fn grain_taps(cell: f32) -> f32 {
    (1.5 / cell.max(1e-3)).ceil().clamp(1.0, 4.0)
}

impl Uniforms {
    pub fn new(p: &Params, size: Size<u32>) -> Self {
        let (w, h) = (size.width as f32, size.height as f32);
        let sx = w / p.frame.0.max(1.0);
        let sy = h / p.frame.1.max(1.0);
        let s = sx.min(sy);
        let e = echo_order(&p.echo);
        // The texture's direction per device pixel: a design-space unit
        // direction (c, s) advances (c / sx, s / sy) design px per device
        // px; its length converts the pitch to device pixels.
        let (sa, ca) = p.echo_tex_angle.to_radians().sin_cos();
        let (dx, dy) = (ca / sx, sa / sy);
        let n = (dx * dx + dy * dy).sqrt().max(1e-9);
        let tex_pitch = device_pitch(p.echo_tex_pitch, 1.0 / n);
        let gmax = |v: [f32; 3]| v[0].max(v[1]).max(v[2]);
        let cell = (p.grain_cell * s).max(0.25);
        let blend = match p.echo_blend {
            Blend::Over => 0.0,
            Blend::Lighten => 1.0,
        };
        let (hatch_pitch, hx, hy) = device_direction(p.hatch_angle, p.hatch_pitch, sx, sy);
        let mut over = [[0.0; 4]; 3 * MAX_OVER];
        let mut live = 0;
        for o in p.over.iter().filter(|o| o.is_live()) {
            over[3 * live] = [o.x, o.y, o.x + o.w, o.y + o.h];
            over[3 * live + 1] = o.bevel;
            over[3 * live + 2] = [if o.cast { 1.0 } else { 0.0 }, o.halo[0], o.halo[1], o.halo[2]];
            live += 1;
        }
        let bloom = if p.has_bloom() { p.bloom_gain } else { 0.0 };
        Self {
            over,
            u: [
                [w, h, sx, sy],
                [e[0].0, e[0].1, e[0].2, e[1].0],
                [e[1].1, e[1].2, e[2].0, e[2].1],
                [e[2].2, p.echo_chroma.0, p.echo_centre.0 * sx, p.echo_centre.1 * sy],
                [tex_pitch, p.echo_phase, dx / n, dy / n],
                [
                    p.echo_tex_crest[0],
                    p.echo_tex_crest[1],
                    p.echo_tex_crest[2],
                    blend,
                ],
                [
                    p.echo_tex_trough[0],
                    p.echo_tex_trough[1],
                    p.echo_tex_trough[2],
                    p.echo_chroma.1,
                ],
                [
                    p.echo_gate.0,
                    p.echo_gate.1,
                    if p.echo_gate_radius > 0.0 { (p.echo_gate_radius * s).max(1.0) } else { 0.0 },
                    p.echo_soft * s,
                ],
                [
                    p.echo_lift.0,
                    p.echo_lift.1,
                    p.scan_depth,
                    device_pitch(p.scan_pitch, sy),
                ],
                [p.scan_phase, gmax(p.grain_mul), gmax(p.grain_add), cell],
                [
                    p.grain_seed as f32,
                    p.echo_ground.0,
                    p.echo_ground.1,
                    p.echo_ground.2,
                ],
                [
                    p.echo_enclose * s,
                    (p.echo_enclose_thick * s).max(1.0),
                    p.echo_soft_far.max(p.echo_soft) * s,
                    grain_taps(cell),
                ],
                [p.grain_mul[0], p.grain_mul[1], p.grain_mul[2], 0.0],
                [p.grain_add[0], p.grain_add[1], p.grain_add[2], 0.0],
                [bloom, p.bloom_sigma * s, p.bloom_knee.0, p.bloom_knee.1],
                [p.bloom_tint[0], p.bloom_tint[1], p.bloom_tint[2], p.bloom_hatch],
                [hatch_pitch, hx, hy, p.halo_hatch],
                [p.halo_sigma * s, live as f32, s, 0.0],
            ],
        }
    }
}

struct Target {
    size: Size<u32>,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    /// The print pass's bindings: the frame and the bloom's result.
    bind_group: wgpu::BindGroup,
    /// The bloom's three passes: frame -> half A (threshold and
    /// downsample), A -> B (across), B -> A (down).
    bloom_down: wgpu::BindGroup,
    bloom_across: wgpu::BindGroup,
    bloom_up: wgpu::BindGroup,
    half_a: wgpu::TextureView,
    half_b: wgpu::TextureView,
    half: Size<u32>,
}

/// The format the bloom is blurred in: encoded values, unclamped.
const BLOOM_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

/// One bloom pass's uniforms: half-resolution size, knee; blur direction,
/// sigma (half px) and taps either side.
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
struct BloomUniforms {
    u: [[f32; 4]; 2],
}

/// The pass's pipeline and its offscreen frame, kept by the renderer.
pub(crate) struct Pass {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    bloom_layout: wgpu::BindGroupLayout,
    bloom_down: wgpu::RenderPipeline,
    bloom_blur: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    uniforms: wgpu::Buffer,
    bloom_uniforms: [wgpu::Buffer; 3],
    source_format: wgpu::TextureFormat,
    pub(crate) target_format: wgpu::TextureFormat,
    target: Option<Target>,
}

fn layout_entries(extra: bool) -> Vec<wgpu::BindGroupLayoutEntry> {
    let texture = |binding| wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    };
    let mut entries = vec![
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
        texture(2),
    ];
    if extra {
        entries.push(texture(3));
    }
    entries
}

fn pipeline(
    device: &wgpu::Device,
    label: &str,
    layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule,
    entry: &str,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &[layout],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(entry),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

impl Pass {
    pub(crate) fn new(
        device: &wgpu::Device,
        source_format: wgpu::TextureFormat,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("iced_wgpu.print.layout"),
            entries: &layout_entries(true),
        });
        let bloom_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("iced_wgpu.print.bloom_layout"),
            entries: &layout_entries(false),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("iced_wgpu.print.shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader/print.wgsl"))),
        });
        let bloom_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("iced_wgpu.print.bloom_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader/bloom.wgsl"))),
        });

        let pipeline = pipeline(device, "iced_wgpu.print.pipeline", &layout, &shader, "fs_main", target_format);
        let bloom_down = pipeline_for(device, "iced_wgpu.print.bloom_down", &bloom_layout, &bloom_shader, "fs_down");
        let bloom_blur = pipeline_for(device, "iced_wgpu.print.bloom_blur", &bloom_layout, &bloom_shader, "fs_blur");

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("iced_wgpu.print.sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..wgpu::SamplerDescriptor::default()
        });

        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("iced_wgpu.print.uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bloom_uniform = || {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("iced_wgpu.print.bloom_uniforms"),
                size: std::mem::size_of::<BloomUniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        Self {
            pipeline,
            layout,
            bloom_layout,
            bloom_down,
            bloom_blur,
            sampler,
            uniforms,
            bloom_uniforms: [bloom_uniform(), bloom_uniform(), bloom_uniform()],
            source_format,
            target_format,
            target: None,
        }
    }

    /// The offscreen frame to draw into, (re)created at `size`.
    pub(crate) fn target(
        &mut self,
        device: &wgpu::Device,
        size: Size<u32>,
    ) -> wgpu::TextureView {
        if self.target.as_ref().map(|t| t.size) != Some(size) {
            let make = |label, w: u32, h: u32, format| {
                device.create_texture(&wgpu::TextureDescriptor {
                        label: Some(label),
                        size: wgpu::Extent3d {
                            width: w.max(1),
                            height: h.max(1),
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::TEXTURE_BINDING
                            | wgpu::TextureUsages::COPY_DST,
                        view_formats: &[],
                    })
            };
            let texture = |label, w, h, format| {
                make(label, w, h, format).create_view(&wgpu::TextureViewDescriptor::default())
            };
            let frame = make("iced_wgpu.print.frame", size.width, size.height, self.source_format);
            let view = frame.create_view(&wgpu::TextureViewDescriptor::default());
            let half = Size::new(size.width.div_ceil(2).max(1), size.height.div_ceil(2).max(1));
            let half_a = texture("iced_wgpu.print.bloom_a", half.width, half.height, BLOOM_FORMAT);
            let half_b = texture("iced_wgpu.print.bloom_b", half.width, half.height, BLOOM_FORMAT);
            let group = |label, layout, uniforms: &wgpu::Buffer, views: &[&wgpu::TextureView]| {
                let mut entries = vec![
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: uniforms.as_entire_binding(),
                    },
                ];
                for (i, v) in views.iter().enumerate() {
                    entries.push(wgpu::BindGroupEntry {
                        binding: 2 + i as u32,
                        resource: wgpu::BindingResource::TextureView(v),
                    });
                }
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(label),
                    layout,
                    entries: &entries,
                })
            };
            let bind_group = group("iced_wgpu.print.bind_group", &self.layout, &self.uniforms, &[&view, &half_a]);
            let bloom_down = group("iced_wgpu.print.bloom_down", &self.bloom_layout, &self.bloom_uniforms[0], &[&view]);
            let bloom_across = group("iced_wgpu.print.bloom_across", &self.bloom_layout, &self.bloom_uniforms[1], &[&half_a]);
            let bloom_up = group("iced_wgpu.print.bloom_up", &self.bloom_layout, &self.bloom_uniforms[2], &[&half_b]);
            self.target = Some(Target {
                size,
                texture: frame,
                view,
                bind_group,
                bloom_down,
                bloom_across,
                bloom_up,
                half_a,
                half_b,
                half,
            });
        }
        self.target.as_ref().expect("target was just created").view.clone()
    }

    /// Run the pass from the offscreen frame into `frame`: with a bloom,
    /// first its three half-resolution passes, then the print pass.
    pub(crate) fn encode(
        &mut self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        params: &Params,
    ) {
        let Some(target) = self.target.as_ref() else {
            return;
        };
        let uniforms = Uniforms::new(params, target.size);
        queue.write_buffer(&self.uniforms, 0, bytemuck::bytes_of(&uniforms));

        if params.has_bloom() {
            let (sigma, taps) = bloom_taps(uniforms.u[14][1]);
            let (w, h) = (target.half.width as f32, target.half.height as f32);
            let knee = [w, h, params.bloom_knee.0, params.bloom_knee.1];
            let passes = [
                (&target.bloom_down, &target.half_a, [params.bloom_chroma, 0.0, 0.0, 0.0], &self.bloom_down),
                (&target.bloom_across, &target.half_b, [1.0, 0.0, sigma, taps], &self.bloom_blur),
                (&target.bloom_up, &target.half_a, [0.0, 1.0, sigma, taps], &self.bloom_blur),
            ];
            for (i, (group, out, blur, pipeline)) in passes.into_iter().enumerate() {
                let u = BloomUniforms { u: [knee, blur] };
                queue.write_buffer(&self.bloom_uniforms[i], 0, bytemuck::bytes_of(&u));
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("iced_wgpu.print.bloom_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: out,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, group, &[]);
                pass.draw(0..3, 0..1);
            }
        }

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("iced_wgpu.print.render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: frame,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &target.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}

/// Times the pass alone, for `docs/print-overlay.md` (not used in drawing):
/// `pixels` (RGBA8, sRGB, `size`) is uploaded as the frame, then the pass
/// is encoded into a `format` texture and waited on `frames` times. Returns
/// each frame's wall time.
#[doc(hidden)]
pub fn bench(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    source_format: wgpu::TextureFormat,
    format: wgpu::TextureFormat,
    params: &Params,
    size: Size<u32>,
    pixels: &[u8],
    frames: usize,
) -> Vec<std::time::Duration> {
    let mut pass = Pass::new(device, source_format, format);
    let _ = pass.target(device, size);
    let target = pass.target.as_ref().expect("target");
    queue.write_texture(
        target.texture.as_image_copy(),
        pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width),
            rows_per_image: None,
        },
        wgpu::Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
    );
    let out = device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("iced_wgpu.print.bench"),
            size: wgpu::Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default());
    let _ = device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    (0..frames)
        .map(|_| {
            let t = std::time::Instant::now();
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            pass.encode(queue, &mut encoder, &out, params);
            let index = queue.submit([encoder.finish()]);
            let _ = device.poll(wgpu::PollType::Wait { submission_index: Some(index), timeout: None });
            t.elapsed()
        })
        .collect()
}

fn pipeline_for(
    device: &wgpu::Device,
    label: &str,
    layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule,
    entry: &str,
) -> wgpu::RenderPipeline {
    pipeline(device, label, layout, shader, entry, BLOOM_FORMAT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_is_off_and_default() {
        assert!(Params::OFF.is_off());
        assert_eq!(Params::default(), Params::OFF);
    }

    #[test]
    fn copies_are_ordered_farthest_first() {
        let e = echo_order(&[(0.006, 0.01, 0.38), (0.0, 0.0, 0.3), (0.012, 0.017, 0.19)]);
        assert_eq!(e, [(0.012, 0.017, 0.19), (0.006, 0.01, 0.38), (0.0, 0.0, 0.0)]);
        let p = Params { echo: [(0.005, 0.005, 0.0), (0.0, 0.0, 0.4), (0.0, 0.0, 0.0)], ..Params::OFF };
        assert!(p.is_off());
    }

    #[test]
    fn rows_texture_runs_along_y() {
        let p = Params { echo_tex_pitch: 1.985, echo_tex_angle: 90.0, ..Params::OFF };
        let u = Uniforms::new(&p, Size::new(3840, 2160));
        assert!((u.u[4][0] - 4.764).abs() < 1e-3, "{:?}", u.u[4]);
        assert!(u.u[4][2].abs() < 1e-6 && (u.u[4][3] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn small_pitches_snap_to_whole_pixels() {
        assert_eq!(device_pitch(1.985, 1.0), 2.0);
        assert_eq!(device_pitch(1.0, 1.0), 2.0);
        assert!((device_pitch(1.985, 2.4) - 4.764).abs() < 1e-4);
    }
}
