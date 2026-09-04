//! Software compositing for translucent stacks: a [`Prim::Soft`] group
//! is rasterised here, in sRGB, and handed to the canvas as one opaque
//! image.
//!
//! Why this exists. The traces are sRGB documents and rsvg composites
//! `fill-opacity` on the *encoded* channel values, `r = a*c + (1-a)*b`.
//! wgpu blends in linear light. A single translucent fill can be
//! rebased to land the same pixel (see `scene::blend_over`), but only
//! at one backdrop: the rebased layer adds a fixed amount of linear
//! light, where the trace's layer adds *more* over a brighter backdrop
//! and less over a darker one, and it does so per channel -- teal over
//! pink has to darken R while it brightens G, which no one alpha does.
//! Kitsch's dashboard stacks up to seven ghost cards over a haze and
//! every one of them is that case; measured, the rebase left them a
//! hue off (design `68 49 55`, painted `76 47 52`) and split each card
//! into a too-bright and a too-dark half where the layer count under
//! it changed. G2i's shape gate flips between four and six levels on
//! those faint tails, so the screen read 45% matched while looking
//! right at a glance.
//!
//! Rasterising the stack in software is the one exact answer, and it is
//! the answer `login.rs` already gives for its washes. The cost is a
//! frame-sized buffer per group, built once per canvas size and palette
//! (`SoftCache` in `scene.rs`) -- about 60ms for the kitsch dashboard
//! in a release build.
//!
//! Scope. A group holds fills: rects, rounded rects, paths, ellipses,
//! circles, lobes, washes, and `At`/`Turn` around those. Text, grain,
//! dots and plates are not rasterised here -- they never carry an
//! opacity, iced draws them well, and a plate's `on`/`off` switch would
//! defeat the cache. `soft_groups_hold_only_fills` in the test module
//! walks every era table to keep it so.

use crate::palette::Palette;
use crate::style::{Prim, Seg};
use iced::Color;

/// Vertical sub-scanlines per pixel row for polygon coverage; the
/// horizontal extent of a span is exact.
const SUB: usize = 4;

/// The colour a gradient stop table holds at `t`, interpolated in sRGB
/// between the two stops that bracket it -- what SVG does with a
/// gradient's stops.
pub fn stop(stops: &[(f32, Color)], t: f32) -> Color {
    let Some(&(_, first)) = stops.first() else {
        return Color::TRANSPARENT;
    };
    let mut prev = (0.0, first);
    for &(offset, color) in stops {
        if t <= offset {
            let span = offset - prev.0;
            let f = if span > 0.0 { (t - prev.0) / span } else { 0.0 };
            return lerp(prev.1, color, f);
        }
        prev = (offset, color);
    }
    prev.1
}

/// `a` towards `b` by `f`, every channel including alpha, in sRGB.
fn lerp(a: Color, b: Color, f: f32) -> Color {
    let mix = |p: f32, q: f32| p + (q - p) * f;
    Color { r: mix(a.r, b.r), g: mix(a.g, b.g), b: mix(a.b, b.b), a: mix(a.a, b.a) }
}

/// A design-space to pixel-space transform: translate, rotate, scale.
/// `At` moves the origin, `Turn` moves it and turns, and `k` is the
/// frame-to-pixel scale for the whole group.
#[derive(Debug, Clone, Copy)]
struct Xf {
    ox: f32,
    oy: f32,
    k: f32,
    sin: f32,
    cos: f32,
}

impl Xf {
    fn scaled(k: f32) -> Self {
        Xf { ox: 0.0, oy: 0.0, k, sin: 0.0, cos: 1.0 }
    }

    /// Design point to pixel point.
    fn at(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.ox + self.k * (x * self.cos - y * self.sin),
            self.oy + self.k * (x * self.sin + y * self.cos),
        )
    }

    /// Pixel point back to design point.
    fn inv(&self, px: f32, py: f32) -> (f32, f32) {
        let (dx, dy) = ((px - self.ox) / self.k, (py - self.oy) / self.k);
        (dx * self.cos + dy * self.sin, -dx * self.sin + dy * self.cos)
    }

    /// The transform with its origin moved to design `(x, y)`.
    fn moved(&self, x: f32, y: f32) -> Self {
        let (ox, oy) = self.at(x, y);
        Xf { ox, oy, ..*self }
    }

    /// The transform with its origin moved to design `(x, y)` and then
    /// turned `angle` degrees clockwise, SVG's sense.
    fn turned(&self, x: f32, y: f32, angle: f32) -> Self {
        let (ox, oy) = self.at(x, y);
        let (s, c) = angle.to_radians().sin_cos();
        Xf {
            ox,
            oy,
            k: self.k,
            sin: self.sin * c + self.cos * s,
            cos: self.cos * c - self.sin * s,
        }
    }
}

/// A premultiplied RGBA float buffer.
struct Buf {
    w: usize,
    h: usize,
    px: Vec<[f32; 4]>,
}

impl Buf {
    fn new(w: usize, h: usize) -> Self {
        Buf { w, h, px: vec![[0.0; 4]; w * h] }
    }

    /// Composite `c` over pixel `(x, y)` at coverage `cov`: the sRGB
    /// "over" rsvg does, on encoded values.
    fn lay(&mut self, x: usize, y: usize, cov: f32, c: Color) {
        let a = (cov * c.a).clamp(0.0, 1.0);
        if a <= 0.0 {
            return;
        }
        let p = &mut self.px[y * self.w + x];
        let keep = 1.0 - a;
        p[0] = p[0] * keep + c.r * a;
        p[1] = p[1] * keep + c.g * a;
        p[2] = p[2] * keep + c.b * a;
        p[3] = p[3] * keep + a;
    }

    /// Premultiplied 8-bit RGBA, which is what the image pipeline
    /// blends.
    fn bytes(&self) -> Vec<u8> {
        let enc = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        self.px.iter().flat_map(|p| p.map(enc)).collect()
    }

    /// Fill the even-odd interior of `rings` (pixel-space polygons),
    /// colouring each pixel by `paint` at its centre.
    fn fill(&mut self, rings: &[Vec<(f32, f32)>], paint: &dyn Fn(f32, f32) -> Color) {
        let mut edges: Vec<((f32, f32), (f32, f32))> = Vec::new();
        for ring in rings {
            for i in 0..ring.len() {
                let a = ring[i];
                let b = ring[(i + 1) % ring.len()];
                if a.1 != b.1 {
                    edges.push((a, b));
                }
            }
        }
        let Some((x0, x1, y0, y1)) = bbox(rings.iter().flatten().copied()) else {
            return;
        };
        let (col0, col1) = (clamp_lo(x0, self.w), clamp_hi(x1, self.w));
        let (row0, row1) = (clamp_lo(y0, self.h), clamp_hi(y1, self.h));
        if col0 >= col1 || row0 >= row1 {
            return;
        }
        let mut cov = vec![0.0f32; col1 - col0];
        let mut xs: Vec<f32> = Vec::new();
        for row in row0..row1 {
            cov.iter_mut().for_each(|c| *c = 0.0);
            for s in 0..SUB {
                let sy = row as f32 + (s as f32 + 0.5) / SUB as f32;
                xs.clear();
                for &((ax, ay), (bx, by)) in &edges {
                    if (ay <= sy) != (by <= sy) {
                        xs.push(ax + (sy - ay) * (bx - ax) / (by - ay));
                    }
                }
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
                for pair in xs.chunks_exact(2) {
                    span(&mut cov, col0, pair[0], pair[1], 1.0 / SUB as f32);
                }
            }
            for (i, &c) in cov.iter().enumerate() {
                if c > 0.0 {
                    let (px, py) = ((col0 + i) as f32 + 0.5, row as f32 + 0.5);
                    self.lay(col0 + i, row, c.min(1.0), paint(px, py));
                }
            }
        }
    }

    /// Stroke `rings` (pixel-space polylines, closed if `closed`) with a
    /// `width`-pixel line: round joins and caps, as a distance field.
    /// Pixels well inside or outside the band are decided at their
    /// centre and only the edge is supersampled.
    fn stroke(&mut self, rings: &[Vec<(f32, f32)>], closed: bool, width: f32, c: Color) {
        let mut segs: Vec<((f32, f32), (f32, f32))> = Vec::new();
        for ring in rings {
            let n = ring.len();
            let last = if closed { n } else { n.saturating_sub(1) };
            for i in 0..last {
                segs.push((ring[i], ring[(i + 1) % n]));
            }
        }
        let hw = width / 2.0;
        let Some((x0, x1, y0, y1)) = bbox(rings.iter().flatten().copied()) else {
            return;
        };
        let pad = hw + 1.0;
        let (col0, col1) = (clamp_lo(x0 - pad, self.w), clamp_hi(x1 + pad, self.w));
        let (row0, row1) = (clamp_lo(y0 - pad, self.h), clamp_hi(y1 + pad, self.h));
        // Only the segments within reach of a row are measured against
        // it; a rounded card is a few dozen chords and most of them are
        // on the far side.
        let mut active: Vec<((f32, f32), (f32, f32))> = Vec::new();
        for row in row0..row1 {
            let (top, bottom) = (row as f32 - pad, row as f32 + 1.0 + pad);
            active.clear();
            active.extend(
                segs.iter()
                    .filter(|&&(a, b)| a.1.min(b.1) <= bottom && a.1.max(b.1) >= top),
            );
            if active.is_empty() {
                continue;
            }
            let dist = |px: f32, py: f32| {
                active
                    .iter()
                    .map(|&(a, b)| seg_distance(px, py, a, b))
                    .fold(f32::INFINITY, f32::min)
            };
            for col in col0..col1 {
                let (px, py) = (col as f32 + 0.5, row as f32 + 0.5);
                let d = dist(px, py);
                // Half the pixel's diagonal is as far as any point in it
                // is from the centre.
                let cov = if d > hw + 0.71 {
                    continue;
                } else if d < hw - 0.71 {
                    1.0
                } else {
                    let mut inside = 0;
                    for sy in 0..SUB {
                        for sx in 0..SUB {
                            let qx = col as f32 + (sx as f32 + 0.5) / SUB as f32;
                            let qy = row as f32 + (sy as f32 + 0.5) / SUB as f32;
                            if dist(qx, qy) <= hw {
                                inside += 1;
                            }
                        }
                    }
                    inside as f32 / (SUB * SUB) as f32
                };
                self.lay(col, row, cov, c);
            }
        }
    }
}

/// Add `weight` times the overlap of `[x0, x1)` with each pixel column
/// into `cov`, whose first entry is column `col0`.
fn span(cov: &mut [f32], col0: usize, x0: f32, x1: f32, weight: f32) {
    let lo = x0.max(col0 as f32);
    let hi = x1.min((col0 + cov.len()) as f32);
    if hi <= lo {
        return;
    }
    let (i0, i1) = (lo.floor() as usize, (hi.ceil() as usize).max(lo.floor() as usize + 1));
    for i in i0..i1 {
        let (l, r) = (i as f32, i as f32 + 1.0);
        let overlap = (hi.min(r) - lo.max(l)).max(0.0);
        if let Some(c) = cov.get_mut(i - col0) {
            *c += overlap * weight;
        }
    }
}

fn bbox(pts: impl Iterator<Item = (f32, f32)>) -> Option<(f32, f32, f32, f32)> {
    let mut b: Option<(f32, f32, f32, f32)> = None;
    for (x, y) in pts {
        b = Some(match b {
            None => (x, x, y, y),
            Some((x0, x1, y0, y1)) => (x0.min(x), x1.max(x), y0.min(y), y1.max(y)),
        });
    }
    b
}

fn clamp_lo(v: f32, n: usize) -> usize {
    (v.floor().max(0.0) as usize).min(n)
}

fn clamp_hi(v: f32, n: usize) -> usize {
    (v.ceil().max(0.0) as usize).min(n)
}

/// Distance from `(px, py)` to the segment `a`-`b`.
fn seg_distance(px: f32, py: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let t = if len2 > 0.0 {
        (((px - a.0) * dx + (py - a.1) * dy) / len2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let (cx, cy) = (a.0 + t * dx, a.1 + t * dy);
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}

/// Chords per curve segment and per quarter arc. Sixteen keeps a
/// 500px-radius ellipse within a third of a pixel of the true curve.
const CHORDS: usize = 16;

/// Rounded-rectangle outline in design space.
fn round_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Vec<(f32, f32)> {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    if r <= 0.0 {
        return vec![(x, y), (x + w, y), (x + w, y + h), (x, y + h)];
    }
    let mut pts = Vec::with_capacity(4 * (CHORDS + 1));
    let corners = [
        (x + w - r, y + r, -90.0f32),
        (x + w - r, y + h - r, 0.0),
        (x + r, y + h - r, 90.0),
        (x + r, y + r, 180.0),
    ];
    for (cx, cy, start) in corners {
        for i in 0..=CHORDS {
            let a = (start + 90.0 * i as f32 / CHORDS as f32).to_radians();
            pts.push((cx + r * a.cos(), cy + r * a.sin()));
        }
    }
    pts
}

/// Ellipse outline in design space.
fn ellipse(x: f32, y: f32, rx: f32, ry: f32) -> Vec<(f32, f32)> {
    let n = 8 * CHORDS;
    (0..n)
        .map(|i| {
            let a = std::f32::consts::TAU * i as f32 / n as f32;
            (x + rx * a.cos(), y + ry * a.sin())
        })
        .collect()
}

/// A [`Prim::Path`]'s subpaths as polygons in design space, curves
/// flattened to chords.
fn path_rings(x: f32, y: f32, segs: &[Seg]) -> Vec<Vec<(f32, f32)>> {
    let mut rings = vec![vec![(x, y)]];
    for seg in segs {
        let ring = rings.last_mut().unwrap();
        let &(x0, y0) = ring.last().unwrap();
        match *seg {
            Seg::Move(mx, my) => rings.push(vec![(mx, my)]),
            Seg::Line(lx, ly) => ring.push((lx, ly)),
            Seg::Quad { cx, cy, x: qx, y: qy } => {
                for i in 1..=CHORDS {
                    let t = i as f32 / CHORDS as f32;
                    let u = 1.0 - t;
                    ring.push((
                        u * u * x0 + 2.0 * u * t * cx + t * t * qx,
                        u * u * y0 + 2.0 * u * t * cy + t * t * qy,
                    ));
                }
            }
            Seg::Cubic { c1x, c1y, c2x, c2y, x: bx, y: by } => {
                for i in 1..=CHORDS {
                    let t = i as f32 / CHORDS as f32;
                    let u = 1.0 - t;
                    let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
                    ring.push((
                        a * x0 + b * c1x + c * c2x + d * bx,
                        a * y0 + b * c1y + c * c2y + d * by,
                    ));
                }
            }
        }
    }
    rings.retain(|r| r.len() > 1);
    rings
}

/// Can `prim` be rasterised by [`composite`]? What `soft_groups_hold_only_fills`
/// asserts of every era table.
pub fn supported(prim: &Prim) -> bool {
    match prim {
        Prim::Rect { .. }
        | Prim::Path { .. }
        | Prim::Round { .. }
        | Prim::Lobe { .. }
        | Prim::Ellipse { .. }
        | Prim::Circle { .. }
        | Prim::Wash { .. } => true,
        Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Soft { prims } => {
            prims.iter().all(supported)
        }
        Prim::Text { .. }
        | Prim::Wide { .. }
        | Prim::Spaced { .. }
        | Prim::Tracked { .. }
        | Prim::Grain { .. }
        | Prim::Dots { .. }
        | Prim::Plate { .. } => false,
    }
}

/// Rasterise `prims` over a transparent `w`x`h` buffer, the design's
/// 1600x900 frame scaled by `k`, and return it as premultiplied RGBA8.
pub fn composite(prims: &[Prim], palette: &Palette, w: u32, h: u32, k: f32) -> Vec<u8> {
    let mut buf = Buf::new(w as usize, h as usize);
    walk(&mut buf, prims, palette, Xf::scaled(k));
    buf.bytes()
}

fn walk(buf: &mut Buf, prims: &[Prim], palette: &Palette, xf: Xf) {
    let map = |pts: Vec<(f32, f32)>| -> Vec<(f32, f32)> {
        pts.into_iter().map(|(x, y)| xf.at(x, y)).collect()
    };
    let flat = |c: Color| move |_: f32, _: f32| c;
    for prim in prims {
        match *prim {
            Prim::Rect { x, y, w, h, fill, stroke, width } => {
                let ring = map(round_rect(x, y, w, h, 0.0));
                if let Some(ink) = fill {
                    buf.fill(&[ring.clone()], &flat(ink.of(palette)));
                }
                if let Some(ink) = stroke {
                    // Mitred like iced's default stroke: the band between
                    // the rectangle grown and shrunk by half the width.
                    let hw = width / 2.0;
                    let outer = map(round_rect(x - hw, y - hw, w + width, h + width, 0.0));
                    let inner = map(round_rect(x + hw, y + hw, w - width, h - width, 0.0));
                    buf.fill(&[outer, inner], &flat(ink.of(palette)));
                }
            }
            Prim::Round { x, y, w, h, r, fill, stroke, width } => {
                let ring = map(round_rect(x, y, w, h, r));
                if let Some(ink) = fill {
                    buf.fill(&[ring.clone()], &flat(ink.of(palette)));
                }
                if let Some(ink) = stroke {
                    buf.stroke(&[ring], true, width * xf.k, ink.of(palette));
                }
            }
            Prim::Path { x, y, segs, close, fill, stroke, width } => {
                let rings: Vec<_> = path_rings(x, y, segs).into_iter().map(map).collect();
                if let Some(ink) = fill {
                    buf.fill(&rings, &flat(ink.of(palette)));
                }
                if let Some(ink) = stroke {
                    buf.stroke(&rings, close, width * xf.k, ink.of(palette));
                }
            }
            Prim::Ellipse { x, y, rx, ry, fill, stroke, width } => {
                let ring = map(ellipse(x, y, rx, ry));
                if let Some(ink) = fill {
                    buf.fill(&[ring.clone()], &flat(ink.of(palette)));
                }
                if let Some(ink) = stroke {
                    buf.stroke(&[ring], true, width * xf.k, ink.of(palette));
                }
            }
            Prim::Circle { x, y, r, fill, stroke, width } => {
                let ring = map(ellipse(x, y, r, r));
                if let Some(ink) = fill {
                    buf.fill(&[ring.clone()], &flat(ink.of(palette)));
                }
                if let Some(ink) = stroke {
                    buf.stroke(&[ring], true, width * xf.k, ink.of(palette));
                }
            }
            Prim::Lobe { x, y, rx, ry, stops } => {
                // The gradient's own ellipse, its stops read off the
                // normalised distance from the centre -- the radial
                // gradient itself rather than the ring approximation
                // the canvas path draws.
                let ring = map(ellipse(x, y, rx, ry));
                buf.fill(&[ring], &|px, py| {
                    let (lx, ly) = xf.inv(px, py);
                    let t = (((lx - x) / rx).powi(2) + ((ly - y) / ry).powi(2)).sqrt();
                    stop(stops, t.min(1.0))
                });
            }
            Prim::Wash { x, y, w, h, top, foot } => {
                let ring = map(round_rect(x, y, w, h, 0.0));
                let (top, foot) = (top.of(palette), foot.of(palette));
                buf.fill(&[ring], &|px, py| {
                    let (_, ly) = xf.inv(px, py);
                    lerp(top, foot, ((ly - y) / h).clamp(0.0, 1.0))
                });
            }
            Prim::At { x, y, prims } => walk(buf, prims, palette, xf.moved(x, y)),
            Prim::Turn { x, y, angle, prims } => {
                walk(buf, prims, palette, xf.turned(x, y, angle))
            }
            Prim::Soft { prims } => walk(buf, prims, palette, xf),
            Prim::Text { .. }
            | Prim::Wide { .. }
            | Prim::Spaced { .. }
            | Prim::Tracked { .. }
            | Prim::Grain { .. }
            | Prim::Dots { .. }
            | Prim::Plate { .. } => {
                debug_assert!(false, "Prim::Soft holds fills only; see soft.rs");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{fill_rect, Ink};

    fn palette() -> Palette {
        crate::style::Era::Kitsch.style().palette
    }

    fn px(bytes: &[u8], w: u32, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * w + x) * 4) as usize;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    }

    const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    const HALF_GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 0.5 };

    #[test]
    fn a_rect_on_the_grid_covers_its_pixels_and_no_others() {
        let prims = [fill_rect(2.0, 1.0, 3.0, 2.0, Ink::Fixed(RED))];
        let out = composite(&prims, &palette(), 8, 4, 1.0);
        assert_eq!(px(&out, 8, 2, 1), [255, 0, 0, 255]);
        assert_eq!(px(&out, 8, 4, 2), [255, 0, 0, 255]);
        assert_eq!(px(&out, 8, 1, 1), [0, 0, 0, 0]);
        assert_eq!(px(&out, 8, 5, 1), [0, 0, 0, 0]);
        assert_eq!(px(&out, 8, 2, 3), [0, 0, 0, 0]);
    }

    #[test]
    fn a_half_pixel_edge_is_half_covered() {
        let prims = [fill_rect(0.5, 0.0, 2.0, 1.0, Ink::Fixed(RED))];
        let out = composite(&prims, &palette(), 4, 1, 1.0);
        assert_eq!(px(&out, 4, 0, 0), [128, 0, 0, 128]);
        assert_eq!(px(&out, 4, 1, 0), [255, 0, 0, 255]);
        assert_eq!(px(&out, 4, 2, 0), [128, 0, 0, 128]);
        assert_eq!(px(&out, 4, 3, 0), [0, 0, 0, 0]);
    }

    #[test]
    fn translucent_layers_composite_in_srgb() {
        // Half green over red: rsvg's `r = a*c + (1-a)*b` on encoded
        // values gives (128, 128, 0); a linear blend would give the
        // much brighter (188, 188, 0).
        let prims = [
            fill_rect(0.0, 0.0, 1.0, 1.0, Ink::Fixed(RED)),
            fill_rect(0.0, 0.0, 1.0, 1.0, Ink::Fixed(HALF_GREEN)),
        ];
        let out = composite(&prims, &palette(), 1, 1, 1.0);
        assert_eq!(px(&out, 1, 0, 0), [128, 128, 0, 255]);
    }

    #[test]
    fn two_subpaths_fill_even_odd() {
        const SEGS: &[Seg] = &[
            Seg::Line(6.0, 0.0),
            Seg::Line(6.0, 6.0),
            Seg::Line(0.0, 6.0),
            Seg::Move(2.0, 2.0),
            Seg::Line(4.0, 2.0),
            Seg::Line(4.0, 4.0),
            Seg::Line(2.0, 4.0),
        ];
        let prims = [crate::style::fill_path(0.0, 0.0, SEGS, Ink::Fixed(RED))];
        let out = composite(&prims, &palette(), 6, 6, 1.0);
        assert_eq!(px(&out, 6, 0, 0), [255, 0, 0, 255]);
        assert_eq!(px(&out, 6, 2, 2), [0, 0, 0, 0], "the hole is cut out");
        assert_eq!(px(&out, 6, 3, 3), [0, 0, 0, 0]);
        assert_eq!(px(&out, 6, 4, 4), [255, 0, 0, 255]);
    }

    #[test]
    fn a_turn_places_its_child_where_the_scene_does() {
        // A 1x1 square at (2, 0) under a 90-degree clockwise turn about
        // (5, 5) lands at (5, 7): x' = -y, y' = x on a y-down screen.
        const SQUARE: &[Prim] = &[fill_rect(2.0, 0.0, 1.0, 1.0, Ink::Fixed(RED))];
        let prims = [Prim::Turn { x: 5.0, y: 5.0, angle: 90.0, prims: SQUARE }];
        let out = composite(&prims, &palette(), 10, 10, 1.0);
        assert_eq!(px(&out, 10, 4, 7), [255, 0, 0, 255]);
        assert_eq!(px(&out, 10, 7, 2), [0, 0, 0, 0]);
    }

    #[test]
    fn a_stroke_sits_on_the_outline() {
        let prims = [Prim::Round {
            x: 2.0,
            y: 2.0,
            w: 6.0,
            h: 6.0,
            r: 0.0,
            fill: None,
            stroke: Some(Ink::Fixed(RED)),
            width: 2.0,
        }];
        let out = composite(&prims, &palette(), 10, 10, 1.0);
        // Centred on the edge at x=2: columns 1 and 2 are covered, 0
        // and 3 are not.
        assert_eq!(px(&out, 10, 1, 5), [255, 0, 0, 255]);
        assert_eq!(px(&out, 10, 2, 5), [255, 0, 0, 255]);
        assert_eq!(px(&out, 10, 0, 5), [0, 0, 0, 0]);
        assert_eq!(px(&out, 10, 3, 5), [0, 0, 0, 0]);
    }

    #[test]
    fn a_lobe_reads_its_stops_off_the_radius() {
        const STOPS: &[(f32, Color)] = &[
            (0.0, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
            (1.0, Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
        ];
        let prims = [Prim::Lobe { x: 10.0, y: 10.0, rx: 10.0, ry: 5.0, stops: STOPS }];
        let out = composite(&prims, &palette(), 20, 20, 1.0);
        // Half way out along the long axis and along the short axis
        // are the same stop.
        let along = px(&out, 20, 15, 10)[0]; // t = 5.5/10
        let across = px(&out, 20, 10, 12)[0]; // t = 2.5/5
        assert!((along as i32 - 112).abs() <= 2, "{along}");
        assert!((across as i32 - 127).abs() <= 2, "{across}");
        assert_eq!(px(&out, 20, 10, 2)[3], 0, "outside the ellipse");
    }

    #[test]
    fn stop_interpolates_in_srgb_between_bracketing_stops() {
        const STOPS: &[(f32, Color)] = &[
            (0.2, Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
            (0.6, Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 }),
        ];
        assert_eq!(stop(STOPS, 0.0), STOPS[0].1, "before the first stop");
        assert_eq!(stop(STOPS, 1.0), STOPS[1].1, "past the last");
        let mid = stop(STOPS, 0.4);
        assert!((mid.r - 0.5).abs() < 1e-6 && (mid.a - 0.5).abs() < 1e-6);
    }
}
