var<private> uvs: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 0.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 1.0)
);

@group(0) @binding(0) var u_sampler: sampler;
// xy: the region's share of the targets; z: the light-edge mode
// (0 off, 1 on, 2 debug).
@group(0) @binding(1) var<uniform> u_ratio: vec4<f32>;
@group(1) @binding(0) var u_texture: texture_2d<f32>;
// cp-eras-ui local patch (`src/light_edges.rs`, the CPU mirror and the
// derivation): the unresolved colour samples and their mark counts.
@group(1) @binding(1) var u_samples: texture_multisampled_2d<f32>;
@group(1) @binding(2) var u_marks: texture_multisampled_2d<f32>;
// A copy of the frame under this pass (fs_ground only).
@group(1) @binding(3) var u_ground: texture_2d<f32>;

// Michelson contrast in luminance, (Y_mark - Y_under) / (Y_mark + Y_under),
// at and above which a light mark's edge is resolved in full in encoded
// space. Zero and below (a dark mark, or no contrast): untouched.
const CONTRAST_FULL = 0.1;
// Alpha at and above which a group of samples counts as opaque.
const OPAQUE = 0.998;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let uv = uvs[input.vertex_index];

    var out: VertexOutput;
    out.uv = uv * u_ratio.xy;
    out.position = vec4<f32>(uv * vec2(2.0, -2.0) + vec2(-1.0, 1.0), 0.0, 1.0);

    return out;
}

fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    return select(pow((c + 0.055) / 1.055, vec3(2.4)), c / 12.92, c <= vec3(0.04045));
}

fn linear_to_srgb(l: vec3<f32>) -> vec3<f32> {
    let c = max(l, vec3(0.0));
    return select(1.055 * pow(c, vec3(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3(0.0031308));
}

fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3(0.2126, 0.7152, 0.0722));
}

// How much of the encoded resolve a mark of luminance `ym` drawn over
// `yu` takes: 0 for a mark no lighter than what it covers, smoothstep to 1
// at CONTRAST_FULL.
fn light_weight(ym: f32, yu: f32) -> f32 {
    let c = (ym - yu) / max(ym + yu, 1e-6);
    let t = clamp(c / CONTRAST_FULL, 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

// The resolve of a pixel at `p` whose hardware (linear) resolve is `base`.
// With `ground`, every sample is first composited over the frame under
// this pass, `g`, so marks drawn over nothing here are resolved against
// what they will cover; the result is then opaque.
fn light_resolve(p: vec2<i32>, base: vec4<f32>, ground: bool) -> vec4<f32> {
    let mode = u_ratio.z;
    let n = i32(textureNumSamples(u_samples));

    // A pixel with one mark count throughout has no mark edge in it.
    var lo = 1e30;
    var hi = -1e30;
    for (var k = 0; k < n; k++) {
        let m = textureLoad(u_marks, p, k).r;
        lo = min(lo, m);
        hi = max(hi, m);
    }
    if hi - lo < 0.5 {
        return base;
    }

    var g = vec3(0.0);
    if ground {
        g = textureLoad(u_ground, p, 0).rgb;
    }

    // Samples above the lowest count are the mark; the rest are what it
    // was drawn over.
    var mark = vec4(0.0);
    var under = vec4(0.0);
    var nm = 0.0;
    var nu = 0.0;
    var encoded = vec3(0.0);
    for (var k = 0; k < n; k++) {
        var c = textureLoad(u_samples, p, k);
        if ground {
            c = vec4(c.rgb + (1.0 - c.a) * g, 1.0);
        }
        if textureLoad(u_marks, p, k).r > lo + 0.5 {
            mark += c;
            nm += 1.0;
        } else {
            under += c;
            nu += 1.0;
        }
        encoded += linear_to_srgb(c.rgb);
    }
    mark /= nm;
    under /= nu;

    // Without the ground, a mark over nothing drawn in this pass covers
    // the frame below, which this blend cannot read: left linear.
    if under.a < OPAQUE || mark.a < OPAQUE {
        if mode > 1.5 {
            return vec4(0.5, 0.0, 0.5, 1.0);
        }
        return base;
    }

    let w = light_weight(luminance(mark.rgb), luminance(under.rgb));
    if w <= 0.0 {
        if mode > 1.5 {
            return vec4(0.0, 0.0, 0.6, 1.0);
        }
        return base;
    }
    if mode > 1.5 {
        return vec4(0.0, w, 0.0, 1.0);
    }

    var linear = base.rgb;
    if ground {
        linear = base.rgb + (1.0 - base.a) * g;
    }
    let resolved = srgb_to_linear(encoded / f32(n));
    return vec4(mix(linear, resolved, w), 1.0);
}

// The frame under this pass could not be copied.
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(u_texture, u_sampler, input.uv);
    if u_ratio.z < 0.5 {
        return base;
    }
    return light_resolve(vec2<i32>(floor(input.position.xy)), base, false);
}

@fragment
fn fs_ground(input: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(u_texture, u_sampler, input.uv);
    if u_ratio.z < 0.5 {
        return base;
    }
    return light_resolve(vec2<i32>(floor(input.position.xy)), base, true);
}
