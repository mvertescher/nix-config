//! Print pass (local patch; see `PATCHES.md`).
//!
//! An optional full-screen pass over the finished frame that gives it the
//! surface of the reference art: registration copies (the frame scaled about
//! the screen centre and lighten-composited, with scanlined rows), fill
//! scanlines and grain. The application states the material once with
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

/// One era's print material. See the module documentation for units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Params {
    /// The design frame the lengths below are measured in.
    pub frame: (f32, f32),
    /// Registration copies: scale step `k`; copies at `1 + k` and
    /// `1 + 2k` about the screen centre.
    pub echo_scale: f32,
    /// Opacity of the near (`1 + k`) and far (`1 + 2k`) copy.
    pub echo_alpha: (f32, f32),
    /// Row modulation of the copies: depth (0 = flat) and pitch.
    pub echo_scan_depth: f32,
    pub echo_scan_pitch: f32,
    /// Phase of the copies' rows, in cycles.
    pub echo_phase: f32,
    /// Fill gate: the encoded luminance, averaged on a ring of this radius,
    /// at which copies start (`.0`) and finish (`.1`) being suppressed.
    pub echo_gate: (f32, f32),
    pub echo_gate_radius: f32,
    /// Softness of the copies: the spread of a 4-tap box blur.
    pub echo_soft: f32,
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
        echo_scale: 0.0,
        echo_alpha: (0.0, 0.0),
        echo_scan_depth: 0.0,
        echo_scan_pitch: 2.0,
        echo_phase: 0.0,
        echo_gate: (1.0, 1.0),
        echo_gate_radius: 6.0,
        echo_soft: 0.0,
        scan_depth: 0.0,
        scan_pitch: 2.0,
        scan_phase: 0.0,
        grain_mul: 0.0,
        grain_add: 0.0,
        grain_cell: 1.0,
        grain_seed: 0,
    };

    /// Whether the pass would change no pixel, so it is skipped.
    pub fn is_off(&self) -> bool {
        let echo = self.echo_scale > 0.0
            && (self.echo_alpha.0 > 0.0 || self.echo_alpha.1 > 0.0);
        !(echo || self.scan_depth > 0.0 || self.grain_mul > 0.0 || self.grain_add > 0.0)
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

/// The shader's uniform block, six `vec4`s (`shader/print.wgsl`).
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Uniforms {
    pub u: [[f32; 4]; 6],
}

impl Uniforms {
    pub fn new(p: &Params, size: Size<u32>) -> Self {
        let (w, h) = (size.width as f32, size.height as f32);
        let sx = w / p.frame.0.max(1.0);
        let sy = h / p.frame.1.max(1.0);
        let s = sx.min(sy);
        Self {
            u: [
                [w, h, sx, sy],
                [p.echo_scale, p.echo_alpha.1, p.echo_alpha.0, p.echo_scan_depth],
                [
                    device_pitch(p.echo_scan_pitch, sy),
                    p.echo_phase,
                    p.echo_gate.0,
                    p.echo_gate.1,
                ],
                [
                    (p.echo_gate_radius * s).max(1.0),
                    p.scan_depth,
                    device_pitch(p.scan_pitch, sy),
                    p.scan_phase,
                ],
                [p.grain_mul, p.grain_add, (p.grain_cell * s).max(1.0), p.grain_seed as f32],
                [p.echo_soft * s * 0.5, 0.0, 0.0, 0.0],
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
    fn small_pitches_snap_to_whole_pixels() {
        assert_eq!(device_pitch(1.985, 1.0), 2.0);
        assert_eq!(device_pitch(1.0, 1.0), 2.0);
        assert!((device_pitch(1.985, 2.4) - 4.764).abs() < 1e-4);
    }
}
