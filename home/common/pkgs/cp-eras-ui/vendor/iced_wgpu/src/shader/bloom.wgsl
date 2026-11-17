// Bloom passes of the print pass (local patch, see PATCHES.md and
// src/print.rs). Three full-screen passes at half resolution, in encoded
// (sRGB) values kept unclamped in a float texture:
//   fs_down: the frame's light part, `(c - chroma * min(c)) * smoothstep(lo,
//            hi, L(c))` per
//            full-resolution pixel, averaged over each 2x2 block (so a thin
//            light stroke is weighed before it is averaged away);
//   fs_blur: a normalised Gaussian along one axis (run across, then down).
// Alpha carries the light fraction, blurred alike: the print pass keeps the
// glow off pixels mostly surrounded by light (a fill and the ink on it).
// The print pass samples the result bilinearly and screens it over.

struct Uniforms {
    // half-resolution size (px), knee lo, hi
    a: vec4<f32>,
    // blur: direction (unit, half px), sigma (half px), taps either side;
    // down: chroma (how much of the colour's grey part is taken away)
    b: vec4<f32>,
}

@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var<uniform> u: Uniforms;
@group(0) @binding(2) var u_source: texture_2d<f32>;

var<private> corners: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(3.0, -1.0),
    vec2<f32>(-1.0, 3.0),
);

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(corners[i], 0.0, 1.0);
}

fn encode(c: vec3<f32>) -> vec3<f32> {
    let lo = c * 12.92;
    let hi = 1.055 * pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, c <= vec3<f32>(0.0031308));
}

// The glow's source at a pixel, and (alpha) how much of it is light.
fn light(q: vec2<i32>, size: vec2<i32>) -> vec4<f32> {
    let c = encode(textureLoad(u_source, clamp(q, vec2<i32>(0), size - vec2<i32>(1)), 0).rgb);
    let l = dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
    let grey = min(c.r, min(c.g, c.b));
    let w = smoothstep(u.a.z, u.a.w, l);
    return vec4<f32>((c - vec3<f32>(u.b.x * grey)) * w, w);
}

@fragment
fn fs_down(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(u_source));
    let q = vec2<i32>(floor(pos.xy)) * 2;
    let c = light(q, size) + light(q + vec2<i32>(1, 0), size)
        + light(q + vec2<i32>(0, 1), size) + light(q + vec2<i32>(1, 1), size);
    return c * 0.25;
}

@fragment
fn fs_blur(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(u_source));
    let q = vec2<i32>(floor(pos.xy));
    let d = vec2<i32>(u.b.xy);
    let n = i32(u.b.w);
    let k = -0.5 / (u.b.z * u.b.z);
    var acc = vec4<f32>(0.0);
    var sum = 0.0;
    for (var i = -n; i <= n; i = i + 1) {
        let w = exp(k * f32(i * i));
        let t = clamp(q + d * i, vec2<i32>(0), size - vec2<i32>(1));
        acc = acc + w * textureLoad(u_source, t, 0);
        sum = sum + w;
    }
    return acc / sum;
}
