// Print pass (local patch, see PATCHES.md and src/print.rs): one full-screen
// fragment pass over the finished frame, applying an era's print material.
//
// All arithmetic is in encoded (sRGB) values, the space the reference art was
// composited in. The source is an sRGB texture, so samples arrive linear and
// are encoded here; the result is decoded again for the sRGB target.
//
// Three terms, each zero when its strength is zero:
//   1. Registration copies: the frame scaled about the screen centre by
//      (1 + n*k), n = 2 then 1, lighten-composited at alpha a_n, the copy's
//      rows modulated by a cosine at the echo pitch. A fill gate suppresses
//      copies where the neighbourhood is a light fill, so dark ink on a fill
//      is not lightened by a copy of the fill around it.
//   2. Fill scanlines: a darkening multiply by a raised cosine.
//   3. Grain: bilinear value noise, per channel, multiplicative plus additive.

struct Uniforms {
    // size.x, size.y (device px), sx, sy (device px per design px)
    u0: vec4<f32>,
    // echo k, alpha (far copy), alpha (near copy), echo scan depth
    u1: vec4<f32>,
    // echo pitch (device px), echo phase (cycles), gate lo, gate hi
    u2: vec4<f32>,
    // gate radius (device px), scan depth, scan pitch (device px), scan phase
    u3: vec4<f32>,
    // grain mul, grain add (encoded units), grain cell (device px), seed
    u4: vec4<f32>,
    // copy softness: half-spread of the 4-tap box (device px), unused x3
    u5: vec4<f32>,
}

@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var<uniform> u: Uniforms;
@group(0) @binding(2) var u_frame: texture_2d<f32>;

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

fn decode(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((max(c, vec3<f32>(0.0)) + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, c <= vec3<f32>(0.04045));
}

fn luma(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// Encoded colour at a device-pixel position (pixel centres at +0.5).
fn at(p: vec2<f32>) -> vec3<f32> {
    let uv = p / u.u0.xy;
    return encode(textureSampleLevel(u_frame, u_sampler, uv, 0.0).rgb);
}

// A copy sampled as a 4-tap box of half-spread h (0: one bilinear tap).
fn copy_at(p: vec2<f32>, h: f32) -> vec3<f32> {
    if (h <= 0.0) {
        return at(p);
    }
    return (at(p + vec2<f32>(h, h)) + at(p + vec2<f32>(-h, h))
        + at(p + vec2<f32>(h, -h)) + at(p + vec2<f32>(-h, -h))) * 0.25;
}

const TAU: f32 = 6.283185307179586;

// Cosine modulation of a row: 1 + depth * cos(2pi (y / pitch + phase)).
// Mirrored by `print::scan` in cp-eras-ui (src/print.rs).
fn scan(y: f32, pitch: f32, phase: f32) -> f32 {
    return cos(TAU * (y / pitch + phase));
}

// Integer hash (lowbias32). Mirrored in cp-eras-ui src/print.rs.
fn hash(x: u32) -> u32 {
    var h = x;
    h = h ^ (h >> 16u);
    h = h * 0x7feb352du;
    h = h ^ (h >> 15u);
    h = h * 0x846ca68bu;
    h = h ^ (h >> 16u);
    return h;
}

// A lattice value in [-1, 1] for cell (x, y), channel c, seed s.
fn lattice(x: i32, y: i32, c: u32, s: u32) -> f32 {
    let h = hash((bitcast<u32>(x) * 0x9e3779b1u) ^ hash(bitcast<u32>(y) ^ (c * 0x85ebca6bu) ^ s));
    return f32(h) / 2147483647.5 - 1.0;
}

fn noise(p: vec2<f32>, c: u32, s: u32) -> f32 {
    let i = floor(p);
    let f = p - i;
    let x = i32(i.x);
    let y = i32(i.y);
    let a = lattice(x, y, c, s);
    let b = lattice(x + 1, y, c, s);
    let d = lattice(x, y + 1, c, s);
    let e = lattice(x + 1, y + 1, c, s);
    // Normalised to unit standard deviation at every sub-cell position, so
    // the grain has no visible lattice when a cell spans several pixels.
    let wx = (1.0 - f.x) * (1.0 - f.x) + f.x * f.x;
    let wy = (1.0 - f.y) * (1.0 - f.y) + f.y * f.y;
    return mix(mix(a, b, f.x), mix(d, e, f.x), f.y) / (0.57735027 * sqrt(wx * wy));
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let p = pos.xy;
    let src = textureLoad(u_frame, vec2<i32>(p), 0);
    var out = encode(src.rgb);
    let size = u.u0.xy;
    let centre = size * 0.5;

    // 1. Registration copies.
    let k = u.u1.x;
    if (k > 0.0 && (u.u1.y > 0.0 || u.u1.z > 0.0)) {
        let r = u.u3.x;
        let ring = (luma(at(p + vec2<f32>(r, 0.0))) + luma(at(p - vec2<f32>(r, 0.0)))
            + luma(at(p + vec2<f32>(0.0, r))) + luma(at(p - vec2<f32>(0.0, r)))) * 0.25;
        let gate = select(1.0 - smoothstep(u.u2.z, u.u2.w, ring), 1.0, u.u2.w <= u.u2.z);
        let m = 1.0 + u.u1.w * scan(p.y, u.u2.x, u.u2.y);
        let base = out;
        // The far copy is painted first, as the fainter print under the nearer.
        let far = copy_at(centre + (p - centre) / (1.0 + 2.0 * k), u.u5.x);
        let a2 = clamp(u.u1.y * m * gate, 0.0, 1.0);
        out = max(out, mix(out, far, a2));
        let near = copy_at(centre + (p - centre) / (1.0 + k), u.u5.x);
        let a1 = clamp(u.u1.z * m * gate, 0.0, 1.0);
        out = max(out, mix(out, near, a1));
        // Copies only ever lighten what is under them.
        out = max(out, base);
    }

    // 2. Fill scanlines: darken by depth at the trough, 1 at the crest.
    if (u.u3.y > 0.0) {
        out = out * (1.0 - u.u3.y * (0.5 - 0.5 * scan(p.y, u.u3.z, u.u3.w)));
    }

    // 3. Grain.
    if (u.u4.x > 0.0 || u.u4.y > 0.0) {
        let q = p / max(u.u4.z, 1.0);
        let s = bitcast<u32>(i32(u.u4.w));
        let n = vec3<f32>(noise(q, 0u, s), noise(q, 1u, s), noise(q, 2u, s));
        out = out + n * (u.u4.x * out + vec3<f32>(u.u4.y));
    }

    return vec4<f32>(decode(clamp(out, vec3<f32>(0.0), vec3<f32>(1.0))), src.a);
}
