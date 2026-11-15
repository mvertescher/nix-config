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
    /// Grain: multiplicative and additive standard deviation (encoded
    /// 0..1 units), cell size, seed.
    pub grain_mul: f32,
    pub grain_add: f32,
    pub grain_cell: f32,
    pub grain_seed: i32,
}

impl Params {
    /// No material: every strength zero.
    pub const OFF: Self = Self {
        frame: (1600.0, 900.0),
        echo: [(0.0, 0.0, 0.0); 3],
        echo_centre: (800.0, 450.0),
        echo_blend: Blend::Over,
        echo_soft: 0.0,
        echo_lift: (0.0, 0.0),
        echo_tex_pitch: 2.0,
        echo_tex_angle: 90.0,
        echo_tex_crest: [1.0; 3],
        echo_tex_trough: [1.0; 3],
        echo_phase: 0.0,
        echo_gate: (1.0, 1.0),
        echo_gate_radius: 6.0,
        echo_ground: (0.0, 0.0, 1.0),
        echo_chroma: (0.0, 0.0),
        scan_depth: 0.0,
        scan_pitch: 2.0,
        scan_phase: 0.0,
        grain_mul: 0.0,
        grain_add: 0.0,
        grain_cell: 1.0,
        grain_seed: 0,
    };

    /// Whether any registration copy would draw.
    pub fn has_echo(&self) -> bool {
        self.echo.iter().any(|&(kx, ky, a)| (kx != 0.0 || ky != 0.0) && a > 0.0)
    }

    /// Whether the pass would change no pixel, so it is skipped.
    pub fn is_off(&self) -> bool {
        !(self.has_echo()
            || self.scan_depth > 0.0
            || self.grain_mul > 0.0
            || self.grain_add > 0.0)
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

/// The shader's uniform block, eleven `vec4`s (`shader/print.wgsl`).
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Uniforms {
    pub u: [[f32; 4]; 11],
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
        let blend = match p.echo_blend {
            Blend::Over => 0.0,
            Blend::Lighten => 1.0,
        };
        Self {
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
                    (p.echo_gate_radius * s).max(1.0),
                    p.echo_soft * s,
                ],
                [
                    p.echo_lift.0,
                    p.echo_lift.1,
                    p.scan_depth,
                    device_pitch(p.scan_pitch, sy),
                ],
                [
                    p.scan_phase,
                    p.grain_mul,
                    p.grain_add,
                    (p.grain_cell * s).max(1.0),
                ],
                [
                    p.grain_seed as f32,
                    p.echo_ground.0,
                    p.echo_ground.1,
                    p.echo_ground.2,
                ],
            ],
        }
    }
}

struct Target {
    size: Size<u32>,
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

/// The pass's pipeline and its offscreen frame, kept by the renderer.
pub(crate) struct Pass {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    uniforms: wgpu::Buffer,
    source_format: wgpu::TextureFormat,
    pub(crate) target_format: wgpu::TextureFormat,
    target: Option<Target>,
}

impl Pass {
    pub(crate) fn new(
        device: &wgpu::Device,
        source_format: wgpu::TextureFormat,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("iced_wgpu.print.layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("iced_wgpu.print.pipeline_layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });

        let shader =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("iced_wgpu.print.shader"),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "shader/print.wgsl"
                ))),
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("iced_wgpu.print.pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target_format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

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

        Self {
            pipeline,
            layout,
            sampler,
            uniforms,
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
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("iced_wgpu.print.frame"),
                size: wgpu::Extent3d {
                    width: size.width.max(1),
                    height: size.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.source_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view =
                texture.create_view(&wgpu::TextureViewDescriptor::default());
            let bind_group =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("iced_wgpu.print.bind_group"),
                    layout: &self.layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(
                                &self.sampler,
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: self.uniforms.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(
                                &view,
                            ),
                        },
                    ],
                });
            self.target = Some(Target {
                size,
                view,
                bind_group,
            });
        }
        self.target.as_ref().expect("target was just created").view.clone()
    }

    /// Run the pass from the offscreen frame into `frame`.
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
