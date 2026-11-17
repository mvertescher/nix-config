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
//      Regions above the print (`over`): no copy shows inside one, and a
//      region that does not cast leaves no copy of what lies in it; such a
//      region may wear a hatched halo hugging its light marks.
//   2. Bloom: the frame's light part blurred (bloom.wgsl, at half
//      resolution), hatched, screened over.
//   3. Fill scanlines: a darkening multiply by a raised cosine.
//   4. Grain: bilinear value noise, per channel, multiplicative plus additive.

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
    // enclosure reach, enclosure thickness step (device px), far copy blur
    // sigma (device px), grain taps per axis
    u11: vec4<f32>,
    // grain mul per channel
    u12: vec4<f32>,
    // grain add per channel
    u13: vec4<f32>,
    // bloom gain, sigma (device px), knee lo, hi
    u14: vec4<f32>,
    // bloom tint per channel, bloom hatch depth
    u15: vec4<f32>,
    // hatch pitch (device px), unit direction (device), halo hatch depth
    u16: vec4<f32>,
    // halo sigma (device px), live region count, device px per design px
    u17: vec4<f32>,
    // per region: x0, y0, x1, y1 (design px); corner cuts tl, tr, br, bl;
    // cast (0/1), halo per channel
    over: array<vec4<f32>, 36>,
}

@group(0) @binding(0) var u_sampler: sampler;
@group(0) @binding(1) var<uniform> u: Uniforms;
@group(0) @binding(2) var u_frame: texture_2d<f32>;
@group(0) @binding(3) var u_bloom: texture_2d<f32>;

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
// h (0: one bilinear tap). Above 1.6 px the four taps would fall apart into
// separate images of a thin mark, so a 4x4 grid (uniform, pitch h / 1.118,
// the same standard deviation) is used instead.
fn copy_at(p: vec2<f32>, h: f32) -> vec3<f32> {
    if (h <= 0.0) {
        return at(p);
    }
    if (h <= 1.6) {
        return (at(p + vec2<f32>(h, h)) + at(p + vec2<f32>(-h, h))
            + at(p + vec2<f32>(h, -h)) + at(p + vec2<f32>(-h, -h))) * 0.25;
    }
    let d = h / 1.118034;
    var acc = vec3<f32>(0.0);
    for (var j = 0; j < 4; j = j + 1) {
        for (var i = 0; i < 4; i = i + 1) {
            acc = acc + at(p + vec2<f32>(f32(i) - 1.5, f32(j) - 1.5) * d);
        }
    }
    return acc * (1.0 / 16.0);
}

// Encoded luminance of a fill at least two thickness steps thick: the
// least of three samples along `d` from `q`. A thin light line is not a
// fill; its pixels next to dark do not all read light.
fn thick(q: vec2<f32>, d: vec2<f32>, t: f32) -> f32 {
    return min(luma(at(q)), min(luma(at(q + d * t)), luma(at(q + d * (2.0 * t)))));
}

var<private> reach_steps: array<f32, 13> = array<f32, 13>(0.03, 0.06, 0.1, 0.15, 0.21, 0.28, 0.36, 0.45, 0.55, 0.66, 0.78, 0.9, 1.0);

// How completely a light fill encloses `p`: along each of eight directions
// the lightest thick sample within `reach` (13 steps from 0.03 of it, closer
// near), and the least of the eight. A pixel of dark ink inside a
// light fill reads as light as the fill; ground outside it, open in some
// direction, reads dark. Stops early once a direction is open.
fn enclosure(p: vec2<f32>, reach: f32, t: f32, lo: f32, hi: f32) -> f32 {
    var e = 1.0;
    for (var i = 0; i < 8; i = i + 1) {
        let a = f32(i) * (TAU / 8.0);
        let d = vec2<f32>(cos(a), sin(a));
        var best = 0.0;
        for (var j = 0; j < 13; j = j + 1) {
            let f = reach_steps[j];
            best = max(best, thick(p + d * (reach * f), d, t));
            if (best >= hi) {
                break;
            }
        }
        e = min(e, best);
        if (e <= lo) {
            break;
        }
    }
    return e;
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

// Signed distance (design px, > 0 outside) from design point `d` to region
// `i`: a rectangle with 45-degree corner cuts.
fn region_distance(d: vec2<f32>, i: i32) -> f32 {
    let r = u.over[3 * i];
    let b = u.over[3 * i + 1];
    var e = max(max(r.x - d.x, d.x - r.z), max(r.y - d.y, d.y - r.w));
    let k = 0.70710678;
    if (b.x > 0.0) { e = max(e, (b.x - (d.x - r.x) - (d.y - r.y)) * k); }
    if (b.y > 0.0) { e = max(e, (b.y - (r.z - d.x) - (d.y - r.y)) * k); }
    if (b.z > 0.0) { e = max(e, (b.z - (r.z - d.x) - (r.w - d.y)) * k); }
    if (b.w > 0.0) { e = max(e, (b.w - (d.x - r.x) - (r.w - d.y)) * k); }
    return e;
}

// How much of device pixel `p` lies in a region above the print,
// antialiased over one device pixel: `which` 0 any region, 1 those that cast
// no copies, 2 those that do.
fn covered(p: vec2<f32>, which: i32) -> f32 {
    let n = i32(u.u17.y);
    let d = p / u.u0.zw;
    var c = 0.0;
    for (var i = 0; i < n; i = i + 1) {
        let casts = u.over[3 * i + 2].x > 0.5;
        if ((which == 1 && casts) || (which == 2 && !casts)) {
            continue;
        }
        c = max(c, clamp(0.5 - region_distance(d, i) * u.u17.z, 0.0, 1.0));
    }
    return c;
}

// One copy, composited over `out`. `here` is the frame's own colour,
// `tint` the texture's per-channel factor, `gate` the fill gate.
fn composite(out: vec3<f32>, here: vec3<f32>, p: vec2<f32>, k: vec2<f32>, a: f32,
             tint: vec3<f32>, gate: f32, h: f32) -> vec3<f32> {
    if ((k.x == 0.0 && k.y == 0.0) || a <= 0.0) {
        return out;
    }
    let c = u.u3.zw;
    let q = c + (p - c) / (vec2<f32>(1.0) + k);
    let src = copy_at(q, h);
    var lift = 1.0;
    var plated = 0.0;
    if (u.u17.y > 0.0) {
        plated = covered(q, 2);
    }
    if (u.u8.y > u.u8.x) {
        lift = smoothstep(u.u8.x, u.u8.y, luma(src) - luma(here));
        if (plated > 0.0) {
            // A copy of an above-print plate: lifted by the channel in which
            // it is lightest, so a dim plate on a ground of about its own
            // luminance still shows its ghost (still never over lighter ink).
            let dc = src - here;
            lift = mix(lift, smoothstep(u.u8.x, u.u8.y, max(dc.r, max(dc.g, dc.b))), plated);
        }
    }
    if (u.u10.w < 1.0) {
        lift = lift * mix(1.0, u.u10.w, smoothstep(u.u10.y, u.u10.z, luma(here)));
    }
    if (u.u6.w > u.u3.y) {
        let hi = max(src.r, max(src.g, src.b));
        let lo = min(src.r, min(src.g, src.b));
        lift = lift * smoothstep(u.u3.y, u.u6.w, (hi - lo) / max(hi, 1e-4));
    }
    if (u.u17.y > 0.0) {
        // Above the print: nothing shows inside a region, and a region
        // that does not cast leaves no copy of itself.
        lift = lift * (1.0 - covered(p, 0)) * (1.0 - covered(q, 1));
    }
    let w = clamp(a * gate * lift, 0.0, 1.0);
    let col = clamp(src * tint, vec3<f32>(0.0), vec3<f32>(1.0));
    let over = mix(out, col, w);
    return select(over, max(out, over), u.u5.w > 0.5);
}

var<private> halo_rings: array<f32, 4> = array<f32, 4>(0.4, 0.9, 1.5, 2.2);

// The halo of the light marks inside an above-print region with a halo:
// for each channel, `out` moves toward the nearest lighter mark's colour
// by the region's halo fraction times a Gaussian of the distance (taps on
// four rings of 8, 8, 12 and 16 at 0.4, 0.9, 1.5, 2.2 sigma), hatched.
fn halo(out: vec3<f32>, p: vec2<f32>, hatch: f32) -> vec3<f32> {
    let n = i32(u.u17.y);
    let d = p / u.u0.zw;
    var g = vec3<f32>(0.0);
    var inside = 0.0;
    for (var i = 0; i < n; i = i + 1) {
        let h = u.over[3 * i + 2].yzw;
        if (max(h.x, max(h.y, h.z)) <= 0.0) {
            continue;
        }
        let c = clamp(0.5 - region_distance(d, i) * u.u17.z, 0.0, 1.0);
        if (c > inside) {
            inside = c;
            g = h;
        }
    }
    if (inside <= 0.0) {
        return out;
    }
    let s = u.u17.x;
    let here = luma(out);
    var best = 0.0;
    var ink = out;
    for (var r = 0; r < 4; r = r + 1) {
        let rad = halo_rings[r] * s;
        let m = select(8, 4 * r + 4, r > 0);
        let w = exp(-0.5 * halo_rings[r] * halo_rings[r]);
        if (w <= best) {
            break;
        }
        for (var j = 0; j < m; j = j + 1) {
            let a = (f32(j) + 0.5 * f32(r)) * (TAU / f32(m));
            let t = at(p + vec2<f32>(cos(a), sin(a)) * rad);
            let v = w * smoothstep(u.u8.x, u.u8.y, luma(t) - here);
            if (v > best) {
                best = v;
                ink = t;
            }
        }
    }
    let f = clamp(inside * best * g * max(0.0, 1.0 + u.u16.w * hatch), vec3<f32>(0.0), vec3<f32>(1.0));
    return out + f * (ink - out);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let p = pos.xy;
    let src = textureLoad(u_frame, vec2<i32>(p), 0);
    var out = encode(src.rgb);
    let here0 = out;

    // 1. Registration copies.
    if (u.u1.z > 0.0 || u.u2.y > 0.0 || u.u3.x > 0.0) {
        let r = u.u7.z;
        let ring = (luma(at(p + vec2<f32>(r, 0.0))) + luma(at(p - vec2<f32>(r, 0.0)))
            + luma(at(p + vec2<f32>(0.0, r))) + luma(at(p - vec2<f32>(0.0, r)))) * 0.25;
        // A ring radius of 0 leaves the fill gate to the enclosure test.
        let gate = select(1.0 - smoothstep(u.u7.x, u.u7.y, ring), 1.0, u.u7.y <= u.u7.x || r <= 0.0);
        let t = scan(dot(p, u.u4.zw), u.u4.x, u.u4.y);
        let tint = max(vec3<f32>(0.0), mix(u.u6.xyz, u.u5.xyz, 0.5 + 0.5 * t));
        let here = out;
        // The farthest copy (slot a) may be softer than the others.
        out = composite(out, here, p, u.u1.xy, u.u1.z, tint, gate, u.u11.z);
        out = composite(out, here, p, vec2<f32>(u.u1.w, u.u2.x), u.u2.y, tint, gate, u.u7.w);
        out = composite(out, here, p, u.u2.zw, u.u3.x, tint, gate, u.u7.w);
        // Enclosure gate, only where a copy changed the pixel: dark ink
        // enclosed by a light fill (an emblem on a card) keeps no copies.
        let moved = abs(out - here);
        if (u.u11.x > 0.0 && u.u7.y > u.u7.x && max(moved.r, max(moved.g, moved.b)) > 0.25 / 255.0) {
            let e = enclosure(p, u.u11.x, u.u11.y, u.u7.x, u.u7.y);
            // Full suppression by the middle of the fill gate: a fill that
            // encloses ink is a fill even if a little darker than the gate.
            let g2 = 1.0 - smoothstep(u.u7.x, 0.5 * (u.u7.x + u.u7.y), e);
            if (g2 < 1.0) {
                let g = gate * g2;
                out = here;
                out = composite(out, here, p, u.u1.xy, u.u1.z, tint, g, u.u11.z);
                out = composite(out, here, p, vec2<f32>(u.u1.w, u.u2.x), u.u2.y, tint, g, u.u7.w);
                out = composite(out, here, p, u.u2.zw, u.u3.x, tint, g, u.u7.w);
            }
        }
    }

    // Hatching shared by the halo and the bloom: 1 + depth * cos.
    let hatch = scan(dot(p, u.u16.yz), u.u16.x, 0.0);

    // 1b. Halo of an above-print region's light marks.
    if (u.u17.y > 0.0) {
        out = halo(out, p, hatch);
    }

    // 2. Bloom, screened.
    if (u.u14.x > 0.0) {
        // The glow lies around the light parts, not on them: none on a
        // light pixel, nor where the blurred light fraction says the pixel
        // is mostly surrounded by light (dark ink on a fill; beside a
        // fill's straight edge the fraction is at most a half) -- the
        // references' fills and the dark text on them keep their colour.
        let b = textureSampleLevel(u_bloom, u_sampler, p / u.u0.xy, 0.0);
        var open = (1.0 - smoothstep(u.u14.z, u.u14.w, luma(here0))) * (1.0 - smoothstep(0.5, 0.7, b.a));
        if (u.u17.y > 0.0) {
            // An element above the print does not glow and is not glowed
            // on (Kitsch's BRAINDANCE text has its halo and no glow).
            open = open * (1.0 - covered(p, 0));
        }
        let g = b.rgb * open;
        let add = clamp(u.u14.x * g * u.u15.xyz * max(0.0, 1.0 + u.u15.w * hatch), vec3<f32>(0.0), vec3<f32>(1.0));
        out = vec3<f32>(1.0) - (vec3<f32>(1.0) - out) * (vec3<f32>(1.0) - add);
    }

// 3. Fill scanlines: darken by depth at the trough, 1 at the crest.
    if (u.u8.z > 0.0) {
        out = out * (1.0 - u.u8.z * (0.5 - 0.5 * scan(p.y, u.u8.w, u.u9.x)));
    }

    // 4. Grain: value noise of cell u9.w device px, integrated over the
    // pixel on an m x m grid of taps (m = u11.w), so a cell smaller than a
    // pixel averages down the way a downsampled print's grain does.
    if (u.u9.y > 0.0 || u.u9.z > 0.0) {
        let s = bitcast<u32>(i32(u.u10.x));
        let m = i32(u.u11.w);
        let cell = max(u.u9.w, 0.25);
        let corner = floor(p);
        var n = vec3<f32>(0.0);
        for (var j = 0; j < m; j = j + 1) {
            for (var i = 0; i < m; i = i + 1) {
                let q = (corner + (vec2<f32>(f32(i), f32(j)) + 0.5) / f32(m)) / cell;
                n = n + vec3<f32>(noise(q, 0u, s), noise(q, 1u, s), noise(q, 2u, s));
            }
        }
        n = n / f32(m * m);
        out = out + n * (u.u12.xyz * out + u.u13.xyz);
    }

    return vec4<f32>(decode(clamp(out, vec3<f32>(0.0), vec3<f32>(1.0))), src.a);
}
