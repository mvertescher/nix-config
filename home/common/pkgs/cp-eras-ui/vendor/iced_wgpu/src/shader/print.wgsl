// Print pass (local patch, see PATCHES.md and src/print.rs): one full-screen
// fragment pass over the finished frame, applying an era's print material.
//
// All arithmetic is in encoded (sRGB) values, the space the reference art was
// composited in. The source is an sRGB texture, so samples arrive linear and
// are encoded here; the result is decoded again for the sRGB target.
//
// Three terms, each zero when its strength is zero:
//   1. Registration copies: up to three copies of the frame, each scaled
//      about a centre by (1 + kx, 1 + ky), blurred (4-tap box, sigma = h),
//      its colour scaled per channel between a texture's crest and trough
//      factors along a cosine (rows for Neomil), and
//      composited farthest first at its opacity: alpha-over (or lighten).
//      Two gates keep the copies *under* the art: the lift gate shows a copy
//      only where it is lighter than the pixel under it, and the fill gate
//      suppresses copies where the neighbourhood is a light fill, so dark
//      ink on a fill is not lightened by a copy of the fill around it.
//      Two more weigh them: the ground gate weakens a copy that lands on a
//      mid-tone ground (to `floor` of its opacity), and the chroma gate
//      drops copies of unsaturated colour when the texture was fitted on a
//      saturated ink.
//   2. Fill scanlines: a darkening multiply by a raised cosine.
//   3. Grain: bilinear value noise, per channel, multiplicative plus additive.

struct Uniforms {
    // size.x, size.y (device px), sx, sy (device px per design px)
    u0: vec4<f32>,
    // copy a: kx, ky, opacity; copy b: kx (farthest first)
    u1: vec4<f32>,
    // copy b: ky, opacity; copy c: kx, ky
    u2: vec4<f32>,
    // copy c: opacity, chroma gate lo, scale centre (device px)
    u3: vec4<f32>,
    // texture pitch (device px), phase (cycles), unit direction (device)
    u4: vec4<f32>,
    // texture crest factor per channel, blend (0 over, 1 lighten)
    u5: vec4<f32>,
    // texture trough factor per channel, chroma gate hi
    u6: vec4<f32>,
    // fill gate lo, hi, ring radius (device px), copy blur sigma (device px)
    u7: vec4<f32>,
    // lift gate lo, hi, fill scan depth, fill scan pitch (device px)
    u8: vec4<f32>,
    // fill scan phase, grain mul, grain add (encoded), grain cell (device px)
    u9: vec4<f32>,
    // grain seed, ground gate lo, hi, floor
    u10: vec4<f32>,
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

// A copy sampled as a 4-tap box at (+-h, +-h): per-axis standard deviation
// h (0: one bilinear tap).
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

// One copy, composited over `out`. `here` is the frame's own colour,
// `tint` the texture's per-channel factor, `gate` the fill gate.
fn composite(out: vec3<f32>, here: vec3<f32>, p: vec2<f32>, k: vec2<f32>, a: f32,
             tint: vec3<f32>, gate: f32) -> vec3<f32> {
    if ((k.x == 0.0 && k.y == 0.0) || a <= 0.0) {
        return out;
    }
    let c = u.u3.zw;
    let src = copy_at(c + (p - c) / (vec2<f32>(1.0) + k), u.u7.w);
    var lift = 1.0;
    if (u.u8.y > u.u8.x) {
        lift = smoothstep(u.u8.x, u.u8.y, luma(src) - luma(here));
    }
    if (u.u10.w < 1.0) {
        lift = lift * mix(1.0, u.u10.w, smoothstep(u.u10.y, u.u10.z, luma(here)));
    }
    if (u.u6.w > u.u3.y) {
        let hi = max(src.r, max(src.g, src.b));
        let lo = min(src.r, min(src.g, src.b));
        lift = lift * smoothstep(u.u3.y, u.u6.w, (hi - lo) / max(hi, 1e-4));
    }
    let w = clamp(a * gate * lift, 0.0, 1.0);
    let col = clamp(src * tint, vec3<f32>(0.0), vec3<f32>(1.0));
    let over = mix(out, col, w);
    return select(over, max(out, over), u.u5.w > 0.5);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let p = pos.xy;
    let src = textureLoad(u_frame, vec2<i32>(p), 0);
    var out = encode(src.rgb);

    // 1. Registration copies.
    if (u.u1.z > 0.0 || u.u2.y > 0.0 || u.u3.x > 0.0) {
        let r = u.u7.z;
        let ring = (luma(at(p + vec2<f32>(r, 0.0))) + luma(at(p - vec2<f32>(r, 0.0)))
            + luma(at(p + vec2<f32>(0.0, r))) + luma(at(p - vec2<f32>(0.0, r)))) * 0.25;
        let gate = select(1.0 - smoothstep(u.u7.x, u.u7.y, ring), 1.0, u.u7.y <= u.u7.x);
        let t = scan(dot(p, u.u4.zw), u.u4.x, u.u4.y);
        let tint = max(vec3<f32>(0.0), mix(u.u6.xyz, u.u5.xyz, 0.5 + 0.5 * t));
        let here = out;
        out = composite(out, here, p, u.u1.xy, u.u1.z, tint, gate);
        out = composite(out, here, p, vec2<f32>(u.u1.w, u.u2.x), u.u2.y, tint, gate);
        out = composite(out, here, p, u.u2.zw, u.u3.x, tint, gate);
    }

// 2. Fill scanlines: darken by depth at the trough, 1 at the crest.
    if (u.u8.z > 0.0) {
        out = out * (1.0 - u.u8.z * (0.5 - 0.5 * scan(p.y, u.u8.w, u.u9.x)));
    }

    // 3. Grain.
    if (u.u9.y > 0.0 || u.u9.z > 0.0) {
        let q = p / max(u.u9.w, 1.0);
        let s = bitcast<u32>(i32(u.u10.x));
        let n = vec3<f32>(noise(q, 0u, s), noise(q, 1u, s), noise(q, 2u, s));
        out = out + n * (u.u9.y * out + vec3<f32>(u.u9.z));
    }

    return vec4<f32>(decode(clamp(out, vec3<f32>(0.0), vec3<f32>(1.0))), src.a);
}
