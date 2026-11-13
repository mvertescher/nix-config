struct VertexInput {
    @builtin(vertex_index) vertex_idx: u32,
    @location(0) pos: vec2<i32>,
    @location(1) dim: u32,
    @location(2) uv: u32,
    @location(3) color: u32,
    @location(4) content_type_with_srgb: u32,
    @location(5) depth: f32,
}

struct VertexOutput {
    @invariant @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) @interpolate(flat) content_type: u32,
    // cp-eras-ui: (strength, encoded ink lightness, ink luminance, 0); see
    // `dark_ink_coverage`. Strength 0 means the upstream path.
    @location(3) @interpolate(flat) dark_ink: vec4<f32>,
};

struct Params {
    screen_resolution: vec2<u32>,
    _pad: vec2<u32>,
};

@group(0) @binding(0)
var color_atlas_texture: texture_2d<f32>;

@group(0) @binding(1)
var mask_atlas_texture: texture_2d<f32>;

@group(0) @binding(2)
var atlas_sampler: sampler;

@group(1) @binding(0)
var<uniform> params: Params;

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        return c / 12.92;
    } else {
        return pow((c + 0.055) / 1.055, 2.4);
    }
}

// cp-eras-ui local patch (PATCHES.md): dark-ink coverage correction. The
// CPU mirror, with the reasoning and the unit tests, is `src/ink.rs`; the
// three constants must stay equal to the ones there (a test checks).
const ASSUMED_FILL = 0.78;
const FULL_Y = 0.04;
const OFF_Y = 0.085;

fn linear_to_srgb(l: f32) -> f32 {
    if l <= 0.0031308 {
        return l * 12.92;
    } else {
        return 1.055 * pow(l, 1.0 / 2.4) - 0.055;
    }
}

// (strength, encoded lightness, luminance, 0) of a linear ink colour.
fn dark_ink(rgb: vec3<f32>) -> vec4<f32> {
    let y = dot(rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let t = clamp((y - FULL_Y) / (OFF_Y - FULL_Y), 0.0, 1.0);
    let w = 1.0 - t * t * (3.0 - 2.0 * t);
    return vec4<f32>(w, linear_to_srgb(y), y, 0.0);
}

// The coverage at which a linear-light blend lands where an encoded (sRGB)
// blend of the same coverage would, on a fill of lightness ASSUMED_FILL,
// mixed in by the ink's strength.
fn dark_ink_coverage(a: f32, ink: vec4<f32>) -> f32 {
    let fill = srgb_to_linear(ASSUMED_FILL);
    let target_value = srgb_to_linear(a * ink.y + (1.0 - a) * ASSUMED_FILL);
    let encoded = clamp((fill - target_value) / (fill - ink.z), 0.0, 1.0);
    return a + (encoded - a) * ink.x;
}

@vertex
fn vs_main(in_vert: VertexInput) -> VertexOutput {
    var pos = in_vert.pos;
    let width = in_vert.dim & 0xffffu;
    let height = (in_vert.dim & 0xffff0000u) >> 16u;
    let color = in_vert.color;
    var uv = vec2<u32>(in_vert.uv & 0xffffu, (in_vert.uv & 0xffff0000u) >> 16u);
    let v = in_vert.vertex_idx;

    let corner_position = vec2<u32>(
        in_vert.vertex_idx & 1u,
        (in_vert.vertex_idx >> 1u) & 1u,
    );

    let corner_offset = vec2<u32>(width, height) * corner_position;

    uv = uv + corner_offset;
    pos = pos + vec2<i32>(corner_offset);

    var vert_output: VertexOutput;

    vert_output.position = vec4<f32>(
        2.0 * vec2<f32>(pos) / vec2<f32>(params.screen_resolution) - 1.0,
        in_vert.depth,
        1.0,
    );

    vert_output.position.y *= -1.0;

    let content_type = in_vert.content_type_with_srgb & 0xffffu;
    let srgb = (in_vert.content_type_with_srgb & 0xffff0000u) >> 16u;

    switch srgb {
        case 0u: {
            vert_output.color = vec4<f32>(
                f32((color & 0x00ff0000u) >> 16u) / 255.0,
                f32((color & 0x0000ff00u) >> 8u) / 255.0,
                f32(color & 0x000000ffu) / 255.0,
                f32((color & 0xff000000u) >> 24u) / 255.0,
            );
        }
        case 1u: {
            vert_output.color = vec4<f32>(
                srgb_to_linear(f32((color & 0x00ff0000u) >> 16u) / 255.0),
                srgb_to_linear(f32((color & 0x0000ff00u) >> 8u) / 255.0),
                srgb_to_linear(f32(color & 0x000000ffu) / 255.0),
                f32((color & 0xff000000u) >> 24u) / 255.0,
            );
        }
        default: {}
    }

    // Only in Accurate mode (linear blending); Web mode already blends
    // encoded values.
    vert_output.dark_ink = vec4<f32>(0.0);
    if srgb == 1u {
        vert_output.dark_ink = dark_ink(vert_output.color.rgb);
    }

    var dim: vec2<u32> = vec2(0u);
    switch content_type {
        case 0u: {
            dim = textureDimensions(color_atlas_texture);
            break;
        }
        case 1u: {
            dim = textureDimensions(mask_atlas_texture);
            break;
        }
        default: {}
    }

    vert_output.content_type = content_type;

    vert_output.uv = vec2<f32>(uv) / vec2<f32>(dim);

    return vert_output;
}

@fragment
fn fs_main(in_frag: VertexOutput) -> @location(0) vec4<f32> {
    switch in_frag.content_type {
        case 0u: {
            return textureSampleLevel(color_atlas_texture, atlas_sampler, in_frag.uv, 0.0);
        }
        case 1u: {
            let coverage = textureSampleLevel(mask_atlas_texture, atlas_sampler, in_frag.uv, 0.0).x;
            if in_frag.dark_ink.x <= 0.0 {
                return vec4<f32>(in_frag.color.rgb, in_frag.color.a * coverage);
            }
            return vec4<f32>(in_frag.color.rgb, in_frag.color.a * dark_ink_coverage(coverage, in_frag.dark_ink));
        }
        default: {
            return vec4<f32>(0.0);
        }
    }
}
