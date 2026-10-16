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
//! circles, lobes, washes, multi-stop ramps, luminance-masked
//! sub-groups, and `At`/`Turn` around those. Text, grain,
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

/// A pixel rectangle in absolute frame coordinates. Bounds are rounded
/// before intersection: two disjoint shapes can still cover parts of
/// the same pixel, and a luminance mask multiplies those coverages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Area {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Area {
    const EMPTY: Self = Self { x0: 0, y0: 0, x1: 0, y1: 0 };

    fn empty(self) -> bool {
        self.x0 >= self.x1 || self.y0 >= self.y1
    }

    fn intersect(self, other: Self) -> Self {
        Self {
            x0: self.x0.max(other.x0),
            y0: self.y0.max(other.y0),
            x1: self.x1.min(other.x1),
            y1: self.y1.min(other.y1),
        }
    }

    fn union(self, other: Self) -> Self {
        if self.empty() {
            return other;
        }
        if other.empty() {
            return self;
        }
        Self {
            x0: self.x0.min(other.x0),
            y0: self.y0.min(other.y0),
            x1: self.x1.max(other.x1),
            y1: self.y1.max(other.y1),
        }
    }

    fn around(self, bounds: (f32, f32, f32, f32), pad: f32) -> Self {
        let (x0, x1, y0, y1) = bounds;
        // Invalid geometry is not a reason to reject a draw here. Keep
        // the existing walker responsible for its handling instead.
        if ![x0, x1, y0, y1, pad].iter().all(|v| v.is_finite()) {
            return self;
        }
        Self {
            x0: clamp_lo(x0 - pad, self.x1).max(self.x0),
            y0: clamp_lo(y0 - pad, self.y1).max(self.y0),
            x1: clamp_hi(x1 + pad, self.x1).max(self.x0),
            y1: clamp_hi(y1 + pad, self.y1).max(self.y0),
        }
    }
}

/// A premultiplied RGBA float buffer over an absolute pixel rectangle.
/// Frame bands have `x0 = 0`; mask scratch buffers keep only the pixels
/// their two sides can touch. Geometry and samples remain in frame
/// coordinates, so cropping changes neither coverage nor gradients.
struct Buf {
    w: usize,
    h: usize,
    x0: usize,
    y0: usize,
    px: Vec<[f32; 4]>,
}

impl Buf {
    fn band(w: usize, y0: usize, h: usize) -> Self {
        Self::region(Area { x0: 0, y0, x1: w, y1: y0 + h })
    }

    fn region(area: Area) -> Self {
        let (w, h) = (area.x1 - area.x0, area.y1 - area.y0);
        Buf { w, h, x0: area.x0, y0: area.y0, px: vec![[0.0; 4]; w * h] }
    }

    fn area(&self) -> Area {
        Area { x0: self.x0, y0: self.y0, x1: self.x0 + self.w, y1: self.y0 + self.h }
    }

    fn cols(&self, x0: f32, x1: f32) -> (usize, usize) {
        let right = self.x0 + self.w;
        (clamp_lo(x0, right).max(self.x0), clamp_hi(x1, right).max(self.x0))
    }

    /// The rows this buffer holds, clipped to `y0..y1` of the frame.
    fn rows(&self, y0: f32, y1: f32) -> (usize, usize) {
        let top = self.y0;
        let bottom = self.y0 + self.h;
        (clamp_lo(y0, bottom).max(top), clamp_hi(y1, bottom).max(top))
    }

    /// Composite `c` over pixel `(x, y)` at coverage `cov`: the sRGB
    /// "over" rsvg does, on encoded values.
    fn lay(&mut self, x: usize, y: usize, cov: f32, c: Color) {
        let a = (cov * c.a).clamp(0.0, 1.0);
        if a <= 0.0 {
            return;
        }
        let p = &mut self.px[(y - self.y0) * self.w + x - self.x0];
        let keep = 1.0 - a;
        p[0] = p[0] * keep + c.r * a;
        p[1] = p[1] * keep + c.g * a;
        p[2] = p[2] * keep + c.b * a;
        p[3] = p[3] * keep + a;
    }

    /// Multiply every pixel by the luminance of `mask`'s: SVG's
    /// luminance mask, `0.2125 R + 0.7154 G + 0.0721 B` of the encoded
    /// colour times its alpha -- which, on a premultiplied pixel, is
    /// the luminance of the stored channels as they are. rsvg does not
    /// linearise first (see `Prim::Masked`).
    fn mask(&mut self, mask: &Buf) {
        debug_assert_eq!(self.area(), mask.area());
        for (p, m) in self.px.iter_mut().zip(&mask.px) {
            let lum = (0.2125 * m[0] + 0.7154 * m[1] + 0.0721 * m[2]).clamp(0.0, 1.0);
            p.iter_mut().for_each(|v| *v *= lum);
        }
    }

    /// Composite `layer` over this buffer, premultiplied "over".
    fn over(&mut self, layer: &Buf) {
        debug_assert_eq!(self.area().intersect(layer.area()), layer.area());
        for row in 0..layer.h {
            let start = (row + layer.y0 - self.y0) * self.w + layer.x0 - self.x0;
            let dest = &mut self.px[start..start + layer.w];
            let src = &layer.px[row * layer.w..(row + 1) * layer.w];
            for (p, l) in dest.iter_mut().zip(src) {
                let keep = 1.0 - l[3];
                for i in 0..4 {
                    p[i] = p[i] * keep + l[i];
                }
            }
        }
    }

    /// Premultiplied 8-bit RGBA, which is what the image pipeline
    /// blends.
    fn bytes(&self) -> Vec<u8> {
        self.px.iter().flat_map(|p| Self::encode(*p)).collect()
    }

    /// One pixel of [`bytes`](Self::bytes).
    fn encode(p: [f32; 4]) -> [u8; 4] {
        p.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
    }

    /// Fill the even-odd interior of `rings` (pixel-space polygons),
    /// colouring each pixel by `paint` at its centre.
    fn fill(&mut self, rings: &[Vec<(f32, f32)>], paint: &dyn Fn(f32, f32) -> Color) {
        let Some((x0, x1, y0, y1)) = bbox(rings.iter().flatten().copied()) else {
            return;
        };
        let (col0, col1) = self.cols(x0, x1);
        let (row0, row1) = self.rows(y0, y1);
        if col0 >= col1 || row0 >= row1 {
            return;
        }
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
        let (col0, col1) = self.cols(x0 - pad, x1 + pad);
        let (row0, row1) = self.rows(y0 - pad, y1 + pad);
        // Only the segments within reach of a row are measured against
        // it; a rounded card is a few dozen chords and most of them are
        // on the far side.
        let mut active: Vec<((f32, f32), (f32, f32))> = Vec::new();
        let mut spans: Vec<(usize, usize)> = Vec::new();
        // Unusual input keeps the original full-column traversal. Finite
        // endpoint bounds are conservative for every point on a segment.
        let bounded = width.is_finite()
            && width >= 0.0
            && pad.is_finite()
            // At much larger coordinates the legacy f32 segment-distance
            // reconstruction can cancel near an endpoint. Fall back rather
            // than assuming the geometric endpoint interval still contains it.
            && width <= 65_536.0
            && segs.iter().all(|&(a, b)| {
                [a.0, a.1, b.0, b.1]
                    .into_iter()
                    .all(|v| v.is_finite() && v.abs() <= 65_536.0)
            });
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
            spans.clear();
            if bounded {
                for &(a, b) in &active {
                    let (left, right) = self.cols(a.0.min(b.0) - pad, a.0.max(b.0) + pad);
                    let (left, right) = (left.max(col0), right.min(col1));
                    if left < right {
                        spans.push((left, right));
                    }
                }
                spans.sort_unstable_by_key(|&(left, _)| left);
                let mut merged = 0;
                for i in 0..spans.len() {
                    let (left, right) = spans[i];
                    if merged > 0 && left <= spans[merged - 1].1 {
                        spans[merged - 1].1 = spans[merged - 1].1.max(right);
                    } else {
                        spans[merged] = (left, right);
                        merged += 1;
                    }
                }
                spans.truncate(merged);
            } else {
                spans.push((col0, col1));
            }
            for &(left, right) in &spans {
                for col in left..right {
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

    /// Stream consecutive identical white paths. The literal `stroke` above
    /// remains the reference and handles every ineligible input unchanged.
    fn stroke_run<const BOUNDED: bool>(&mut self, prims: &[Prim], palette: &Palette, xf: Xf) -> bool {
        type Edge = ((f32, f32), (f32, f32));
        struct Band {
            hw: f32, pad: f32, color: Color,
            col0: usize, col1: usize, row0: usize, row1: usize,
        }
        struct RowBand {
            band: usize,
            active: Vec<usize>,
            spans: Vec<(usize, usize)>,
        }
        let Prim::Path { x, y, segs, close, .. } = prims[0] else { return false };
        let rings: Vec<_> = path_rings(x, y, segs).into_iter()
            .map(|ring| ring.into_iter().map(|(x, y)| xf.at(x, y)).collect::<Vec<_>>())
            .collect();
        let mut edges: Vec<Edge> = Vec::new();
        for ring in &rings {
            let n = ring.len();
            let last = if close { n } else { n.saturating_sub(1) };
            for i in 0..last {
                edges.push((ring[i], ring[(i + 1) % n]));
            }
        }
        if edges.is_empty() || edges.iter().any(|&(a, b)| {
            [a.0, a.1, b.0, b.1].into_iter()
                .any(|v| !v.is_finite() || v.abs() > 65_536.0)
        }) {
            return false;
        }
        let Some((x0, x1, y0, y1)) = bbox(rings.iter().flatten().copied()) else {
            return false;
        };
        let mut bands = Vec::with_capacity(prims.len());
        for prim in prims {
            let Prim::Path { stroke: Some(ink), width, .. } = *prim else { return false };
            if BOUNDED && prim_area(prim, xf, self.area()).empty() { continue; }
            let pixel_width = width * xf.k;
            let hw = pixel_width / 2.0;
            let pad = hw + 1.0;
            if !pad.is_finite() || pixel_width <= 0.0 || pixel_width > 65_536.0 {
                return false;
            }
            let (col0, col1) = self.cols(x0 - pad, x1 + pad);
            let (row0, row1) = self.rows(y0 - pad, y1 + pad);
            if col0 < col1 && row0 < row1 {
                bands.push(Band { hw, pad, color: ink.of(palette), col0, col1, row0, row1 });
            }
        }
        if bands.is_empty() { return true; }
        let first_row = bands.iter().map(|b| b.row0).min().unwrap();
        let last_row = bands.iter().map(|b| b.row1).max().unwrap();
        // Keep per-band row workspaces alive across all rows. These long
        // contours would otherwise allocate active/spans vectors 123 times
        // per row even though their segment counts never grow.
        let mut row_bands: Vec<RowBand> = (0..bands.len()).map(|band| RowBand {
            band,
            active: Vec::with_capacity(edges.len()),
            spans: Vec::with_capacity(edges.len()),
        }).collect();
        let mut row_indices = Vec::with_capacity(bands.len());
        let mut union = Vec::with_capacity(bands.len() * edges.len());
        for row in first_row..last_row {
            row_indices.clear();
            for (index, band) in bands.iter().enumerate() {
                if row < band.row0 || row >= band.row1 { continue; }
                let (top, bottom) = (row as f32 - band.pad, row as f32 + 1.0 + band.pad);
                let slot = &mut row_bands[index];
                slot.active.clear();
                slot.active.extend(edges.iter().enumerate().filter_map(|(i, &(a, b))| {
                    (a.1.min(b.1) <= bottom && a.1.max(b.1) >= top).then_some(i)
                }));
                if slot.active.is_empty() { continue; }
                slot.spans.clear();
                for &i in &slot.active {
                    let (a, b) = edges[i];
                    let (left, right) = self.cols(a.0.min(b.0) - band.pad, a.0.max(b.0) + band.pad);
                    let (left, right) = (left.max(band.col0), right.min(band.col1));
                    if left < right { slot.spans.push((left, right)); }
                }
                slot.spans.sort_unstable_by_key(|&(left, _)| left);
                let mut merged = 0;
                for i in 0..slot.spans.len() {
                    let (left, right) = slot.spans[i];
                    if merged > 0 && left <= slot.spans[merged - 1].1 {
                        slot.spans[merged - 1].1 = slot.spans[merged - 1].1.max(right);
                    } else {
                        slot.spans[merged] = (left, right);
                        merged += 1;
                    }
                }
                slot.spans.truncate(merged);
                row_indices.push(index);
            }
            if row_indices.is_empty() { continue; }
            if row_indices.iter().any(|&i| row_bands[i].active != row_bands[row_indices[0]].active) {
                // At joins, width-dependent row membership can change the
                // distance minimum. Execute those rows band-first, literally.
                for &index in &row_indices {
                    let r = &row_bands[index];
                    let band = &bands[r.band];
                    let dist = |px: f32, py: f32| r.active.iter()
                        .map(|&i| seg_distance(px, py, edges[i].0, edges[i].1))
                        .fold(f32::INFINITY, f32::min);
                    for &(left, right) in &r.spans {
                        for col in left..right {
                            let (px, py) = (col as f32 + 0.5, row as f32 + 0.5);
                            let d = dist(px, py);
                            let cov = if d > band.hw + 0.71 { continue; }
                                else if d < band.hw - 0.71 { 1.0 }
                                else {
                                    let mut inside = 0;
                                    for sy in 0..SUB { for sx in 0..SUB {
                                        let qx = col as f32 + (sx as f32 + 0.5) / SUB as f32;
                                        let qy = row as f32 + (sy as f32 + 0.5) / SUB as f32;
                                        if dist(qx, qy) <= band.hw { inside += 1; }
                                    }}
                                    inside as f32 / (SUB * SUB) as f32
                                };
                            self.lay(col, row, cov, band.color);
                        }
                    }
                }
                continue;
            }
            let active = &row_bands[row_indices[0]].active;
            let dist = |px: f32, py: f32| active.iter()
                .map(|&i| seg_distance(px, py, edges[i].0, edges[i].1))
                .fold(f32::INFINITY, f32::min);
            union.clear();
            for &index in &row_indices { union.extend_from_slice(&row_bands[index].spans); }
            union.sort_unstable_by_key(|&(left, _)| left);
            let mut merged = 0;
            for i in 0..union.len() {
                let (left, right) = union[i];
                if merged > 0 && left <= union[merged - 1].1 {
                    union[merged - 1].1 = union[merged - 1].1.max(right);
                } else {
                    union[merged] = (left, right);
                    merged += 1;
                }
            }
            union.truncate(merged);
            for &(left, right) in &union {
                for col in left..right {
                    let (px, py) = (col as f32 + 0.5, row as f32 + 0.5);
                    let d = dist(px, py);
                    let mut samples = [0.0f32; SUB * SUB];
                    let mut sampled = false;
                    for &index in &row_indices {
                        let r = &row_bands[index];
                        if !r.spans.iter().any(|&(a, b)| a <= col && col < b) { continue; }
                        let band = &bands[r.band];
                        let cov = if d > band.hw + 0.71 { continue; }
                            else if d < band.hw - 0.71 { 1.0 }
                            else {
                                if !sampled {
                                    for sy in 0..SUB { for sx in 0..SUB {
                                        let qx = col as f32 + (sx as f32 + 0.5) / SUB as f32;
                                        let qy = row as f32 + (sy as f32 + 0.5) / SUB as f32;
                                        samples[sy * SUB + sx] = dist(qx, qy);
                                    }}
                                    sampled = true;
                                }
                                let inside = samples.iter().filter(|&&sample| sample <= band.hw).count();
                                inside as f32 / (SUB * SUB) as f32
                            };
                        self.lay(col, row, cov, band.color);
                    }
                }
            }
        }
        true
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
        | Prim::Ramp { .. } => true,
        Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Soft { prims } => {
            prims.iter().all(supported)
        }
        Prim::Masked { prims, mask } => prims.iter().all(supported) && mask.iter().all(supported),
        // A composited group is rasterised once and cached; it has
        // no clock to move against. The way round is the other
        // nesting -- a `Soft` group under a `Motion`, which `Backdrop`
        // rasterises with `composite_over` and clips as an image.
        Prim::Motion { .. } | Prim::Pick { .. } | Prim::Viewport { .. } => false,
        Prim::Text { .. }
        | Prim::Wide { .. }
        | Prim::ReusableWide { .. }
        | Prim::Outlined { .. }
        | Prim::Spaced { .. }
        | Prim::Tracked { .. }
        | Prim::TrackedWords { .. }
        | Prim::Grain { .. }
        | Prim::Dots { .. }
        | Prim::Plate { .. } => false,
    }
}

/// One horizontal band of a composite: rows `y..y + h` of the frame,
/// as premultiplied RGBA8. A composite is handed to the canvas as
/// bands rather than one image for two reasons, both measured on a
/// 3840x2160 window (the desk report of 2026-09-07):
///
/// - **Time.** One buffer walked on one thread took 560-610 ms for
///   neokitsch's dashboard ground at that size, paid inside `draw`
///   with the whole app stalled; the bands are walked in parallel.
/// - **The first frame.** iced 0.14 uploads an image over 2 MiB on a
///   worker thread (`MAX_SYNC_SIZE` in `iced_wgpu/src/image/cache.rs`)
///   and draws the frame that first names it *without* it, so a
///   33 MB ground came up one frame late: a boot-in over a bare page,
///   then the haze popped in. A band stays under that limit
///   ([`BAND_BYTES`]) and is uploaded in the frame it is drawn in.
///
/// Each band is drawn with the frame's whole geometry and keeps only
/// its rows -- the same edges, the same scanlines, the same float
/// walk -- so the bands laid end to end are the one-buffer composite
/// byte for byte (`bands_are_the_one_buffer_composite`).
#[derive(Debug, Clone)]
pub struct Band {
    pub y: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

/// The most bytes a band carries: under iced's synchronous-upload
/// limit, so it lands in the frame that draws it.
const BAND_BYTES: usize = 2 * 1024 * 1024 - 1;

/// The bands a `w`x`h` frame is walked in: at most [`BAND_BYTES`]
/// each, and no fewer than there are threads to walk them, as
/// `(y, h)` pairs covering `0..h` in order.
fn bands(w: u32, h: u32) -> Vec<(u32, u32)> {
    let per_row = (w as usize * 4).max(1);
    let cap = (BAND_BYTES / per_row).max(1) as u32;
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get() as u32);
    let rows = cap.min(h.div_ceil(threads.max(1))).max(1);
    (0..h).step_by(rows as usize).map(|y| (y, rows.min(h - y))).collect()
}

/// Walk one band per thread, in `f`, and return them in frame order.
fn in_bands<F>(w: u32, h: u32, f: F) -> Vec<Band>
where
    F: Fn(&mut Buf) + Sync,
{
    let plan = bands(w, h);
    std::thread::scope(|scope| {
        let workers: Vec<_> = plan
            .iter()
            .map(|&(y, rows)| {
                let f = &f;
                scope.spawn(move || {
                    let mut buf = Buf::band(w as usize, y as usize, rows as usize);
                    f(&mut buf);
                    Band { y, h: rows, rgba: buf.bytes() }
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join().expect("a band walker panicked")).collect()
    })
}

/// [`Band`]s laid end to end: the frame, top to bottom.
#[cfg(test)]
fn join(bands: Vec<Band>) -> Vec<u8> {
    let mut out = Vec::with_capacity(bands.iter().map(|b| b.rgba.len()).sum());
    for band in bands {
        out.extend_from_slice(&band.rgba);
    }
    out
}

/// Rasterise `prims` over a transparent `w`x`h` frame, the design's
/// 1600x900 scaled by `k`, as premultiplied RGBA8 [`Band`]s.
pub fn composite_bands(prims: &[Prim], palette: &Palette, w: u32, h: u32, k: f32) -> Vec<Band> {
    in_bands(w, h, |buf| walk(buf, prims, palette, Xf::scaled(k)))
}

/// One group in a software-layer stack. All preceding groups contribute
/// to a cut, including groups whose cached image does not need rendering.
/// Standalone images contain only their own group over transparency.
pub struct SoftLayer<'a> {
    pub prims: &'a [Prim],
    pub cut: bool,
    pub render: bool,
}

/// Render the requested images without replaying their prefixes. Retain
/// the original group/image boundaries: merging them changes the GPU's
/// filtering around fractional edges even when the CPU pixels agree.
/// Each band's prefix remains in float sRGB throughout; quantization and
/// coverage cuts affect only the returned image, never the next layer.
/// Output indices match `layers`, with empty vectors for cache hits.
pub fn composite_layers_bands(
    layers: &[SoftLayer<'_>], palette: &Palette, w: u32, h: u32, k: f32,
) -> Vec<Vec<Band>> {
    let mut images: Vec<Vec<Band>> = (0..layers.len()).map(|_| Vec::new()).collect();
    if !layers.iter().any(|layer| layer.render) {
        return images;
    }
    let plan = bands(w, h);
    std::thread::scope(|scope| {
        let workers: Vec<_> = plan.iter().map(|&(y, rows)| {
            scope.spawn(move || {
                let area = Area { x0: 0, y0: y as usize, x1: w as usize, y1: (y + rows) as usize };
                layers_rgba(layers, palette, area, k)
            })
        }).collect();
        for ((y, rows), worker) in plan.into_iter().zip(workers) {
            for (image, rgba) in images.iter_mut().zip(worker.join().expect("a layer-band walker panicked")) {
                if let Some(rgba) = rgba {
                    image.push(Band { y, h: rows, rgba });
                }
            }
        }
    });
    images
}

/// The per-band walk is separate so absolute-coordinate regression ROIs
/// can exercise exactly the production accumulation and encoding path.
fn layers_rgba(layers: &[SoftLayer<'_>], palette: &Palette, area: Area, k: f32) -> Vec<Option<Vec<u8>>> {
    let mut images: Vec<Option<Vec<u8>>> = (0..layers.len()).map(|_| None).collect();
    let Some(last) = layers.iter().rposition(|layer| layer.render) else {
        return images;
    };
    let mut prefix = Buf::region(area);
    for (index, layer) in layers[..=last].iter().enumerate() {
        walk(&mut prefix, layer.prims, palette, Xf::scaled(k));
        if !layer.render {
            continue;
        }
        if index == 0 && !layer.cut {
            images[index] = Some(prefix.bytes());
            continue;
        }
        let mut own = Buf::region(area);
        walk(&mut own, layer.prims, palette, Xf::scaled(k));
        images[index] = Some(if layer.cut {
            prefix.px.iter().zip(&own.px).flat_map(|(pixel, coverage)| {
                if coverage[3] <= 0.0 { [0; 4] } else { Buf::encode(*pixel) }
            }).collect()
        } else {
            own.bytes()
        });
    }
    images
}

/// [`composite_bands`] as one image, for the tests that read pixels.
#[cfg(test)]
pub(crate) fn composite(prims: &[Prim], palette: &Palette, w: u32, h: u32, k: f32) -> Vec<u8> {
    join(composite_bands(prims, palette, w, h, k))
}

/// Rasterise `prims` over the groups `under` it -- all of them, in
/// order, as one composite -- and return only the pixels `prims`
/// itself touches, as premultiplied RGBA8 [`Band`]s: the composite's
/// own value (opaque, where the ground is) there, transparent
/// everywhere else.
///
/// This is how a translucent group gets to *move* over a composited
/// backdrop without leaving sRGB. Drawn as its own image over the
/// ground's, a stack of ghosts would be blended by wgpu in linear
/// light -- the very thing the module note says cannot land the
/// trace. Drawn as the full composite cut to its own coverage, every
/// pixel it puts down is the pixel the one-buffer composite would have
/// put there (same prims, same order, same float walk), and its clip
/// can then move across it as a scissor. The cut is exact at rest
/// wherever the groups over it touch none of its pixels; where they
/// do, the later group's cut includes this one and paints last, so
/// the rest frame is still the one-buffer composite -- only a frame
/// mid-wipe could show a later group's pixel early. Kitsch's two fans
/// share a bounding box but no pixel (`the_kitsch_fans_share_no_pixel`
/// in `scene.rs`).
pub fn composite_over_bands(
    under: &[&[Prim]],
    prims: &[Prim],
    palette: &Palette,
    w: u32,
    h: u32,
    k: f32,
) -> Vec<Band> {
    in_bands(w, h, |all| {
        for group in under {
            walk(all, group, palette, Xf::scaled(k));
        }
        walk(all, prims, palette, Xf::scaled(k));
        let mut own = Buf::band(all.w, all.y0, all.h);
        walk(&mut own, prims, palette, Xf::scaled(k));
        for (p, o) in all.px.iter_mut().zip(&own.px) {
            if o[3] <= 0.0 {
                *p = [0.0; 4];
            }
        }
    })
}

/// [`composite_over_bands`] as one image, for the tests.
#[cfg(test)]
pub(crate) fn composite_over(under: &[&[Prim]], prims: &[Prim], palette: &Palette, w: u32, h: u32, k: f32) -> Vec<u8> {
    join(composite_over_bands(under, prims, palette, w, h, k))
}

/// Which pixels `prims` touches at all -- the cut [`composite_over`]
/// makes, for a test to check two moving groups against.
#[cfg(test)]
pub(crate) fn touched(prims: &[Prim], palette: &Palette, w: u32, h: u32, k: f32) -> Vec<bool> {
    let mut own = Buf::band(w as usize, 0, h as usize);
    walk(&mut own, prims, palette, Xf::scaled(k));
    own.px.iter().map(|o| o[3] > 0.0).collect()
}

/// Conservative pixel coverage, without flattening paths. A Bézier is
/// inside its control-point hull; rectangles enclosing ellipses and
/// rounded rectangles also enclose their chord approximations. One
/// extra pixel protects floating-point edge rounding. Stroke padding
/// matches the distance-field walk (rectangles have mitred corners).
fn prim_area(prim: &Prim, xf: Xf, clip: Area) -> Area {
    let rect = |x: f32, y: f32, w: f32, h: f32, pad: f32| {
        let pts = [xf.at(x, y), xf.at(x + w, y), xf.at(x + w, y + h), xf.at(x, y + h)];
        clip.around(bbox(pts.into_iter()).unwrap(), pad)
    };
    let stroke_pad = |stroke: Option<crate::style::Ink>, width: f32| {
        1.0 + if stroke.is_some() { width.abs() * xf.k.abs() / 2.0 } else { 0.0 }
    };
    match *prim {
        Prim::Rect { x, y, w, h, stroke, width, .. } => {
            let hw = if stroke.is_some() { width.abs() / 2.0 } else { 0.0 };
            rect(x.min(x + w) - hw, y.min(y + h) - hw, w.abs() + 2.0 * hw, h.abs() + 2.0 * hw, 1.0)
        }
        Prim::Round { x, y, w, h, stroke, width, .. } => {
            rect(x, y, w, h, stroke_pad(stroke, width))
        }
        Prim::Path { x, y, segs, stroke, width, .. } => {
            let points = std::iter::once(xf.at(x, y)).chain(segs.iter().flat_map(|seg| {
                // Repeated endpoints keep this iterator allocation-free.
                match *seg {
                    Seg::Move(x, y) | Seg::Line(x, y) => [xf.at(x, y); 3],
                    Seg::Quad { cx, cy, x, y } => [xf.at(cx, cy), xf.at(x, y), xf.at(x, y)],
                    Seg::Cubic { c1x, c1y, c2x, c2y, x, y } => {
                        [xf.at(c1x, c1y), xf.at(c2x, c2y), xf.at(x, y)]
                    }
                }
            }));
            clip.around(bbox(points).unwrap(), stroke_pad(stroke, width))
        }
        Prim::Ellipse { x, y, rx, ry, stroke, width, .. } => {
            rect(x - rx.abs(), y - ry.abs(), 2.0 * rx.abs(), 2.0 * ry.abs(), stroke_pad(stroke, width))
        }
        Prim::Circle { x, y, r, stroke, width, .. } => {
            rect(x - r.abs(), y - r.abs(), 2.0 * r.abs(), 2.0 * r.abs(), stroke_pad(stroke, width))
        }
        Prim::Lobe { x, y, rx, ry, .. } => {
            rect(x - rx.abs(), y - ry.abs(), 2.0 * rx.abs(), 2.0 * ry.abs(), 1.0)
        }
        Prim::Ramp { x, y, w, h, .. } => rect(x, y, w, h, 1.0),
        Prim::Masked { prims, mask } => {
            let content = group_area(prims, xf, clip);
            if content.empty() { content } else { group_area(mask, xf, content) }
        }
        Prim::At { x, y, prims } => group_area(prims, xf.moved(x, y), clip),
        Prim::Turn { x, y, angle, prims } => group_area(prims, xf.turned(x, y, angle), clip),
        Prim::Soft { prims } => group_area(prims, xf, clip),
        // Unsupported primitives must still reach the walk's assertion.
        _ => clip,
    }
}

fn group_area(prims: &[Prim], xf: Xf, clip: Area) -> Area {
    prims.iter().fold(Area::EMPTY, |area, prim| area.union(prim_area(prim, xf, clip)))
}

fn walk(buf: &mut Buf, prims: &[Prim], palette: &Palette, xf: Xf) {
    walk_inner::<true>(buf, prims, palette, xf);
}

/// The uncropped specialization is used only by equivalence tests: it
/// follows the original full-band mask path with the same rasterizer.
fn walk_inner<const BOUNDED: bool>(buf: &mut Buf, prims: &[Prim], palette: &Palette, xf: Xf) {
    walk_engine::<BOUNDED, true>(buf, prims, palette, xf);
}

/// Only adjacent, identical fixed-white paths can share distances. Every
/// other primitive follows the literal walker, including semantic inks.
fn white_stroke_run(prims: &[Prim], xf: Xf) -> usize {
    let Some(Prim::Path { x, y, segs, close, fill: None,
        stroke: Some(crate::style::Ink::Fixed(first)), width }) = prims.first() else {
        return 0;
    };
    let white = |c: &Color| c.r.to_bits() == 1.0f32.to_bits()
        && c.g.to_bits() == 1.0f32.to_bits()
        && c.b.to_bits() == 1.0f32.to_bits() && c.a.is_finite();
    let mut prev = *width * xf.k;
    if !white(first) || !prev.is_finite() || prev <= 0.0 || prev > 65_536.0
        || !x.is_finite() || !y.is_finite() {
        return 0;
    }
    let mut n = 1;
    for prim in &prims[1..] {
        let Prim::Path { x: xx, y: yy, segs: ss, close: cc, fill: None,
            stroke: Some(crate::style::Ink::Fixed(color)), width: ww } = prim else { break };
        let pixel_width = *ww * xf.k;
        if xx.to_bits() != x.to_bits() || yy.to_bits() != y.to_bits()
            || cc != close || !std::ptr::eq(*ss, *segs) || !white(color)
            || !pixel_width.is_finite() || pixel_width <= 0.0
            || pixel_width >= prev || pixel_width > 65_536.0 { break }
        prev = pixel_width;
        n += 1;
    }
    n
}

fn walk_engine<const BOUNDED: bool, const BATCH: bool>(
    buf: &mut Buf, prims: &[Prim], palette: &Palette, xf: Xf,
) {
    let map = |pts: Vec<(f32, f32)>| -> Vec<(f32, f32)> {
        pts.into_iter().map(|(x, y)| xf.at(x, y)).collect()
    };
    let flat = |c: Color| move |_: f32, _: f32| c;
    let mut rest = prims;
    while let Some((prim, tail)) = rest.split_first() {
        if BATCH {
            let count = white_stroke_run(rest, xf);
            if count >= 2 && buf.stroke_run::<BOUNDED>(&rest[..count], palette, xf) {
                rest = &rest[count..];
                continue;
            }
        }
        rest = tail;
        // Wrappers recurse naturally. Reject leaves before allocating
        // flattened geometry, and masks before allocating either side.
        let wrapper = matches!(prim, Prim::At { .. } | Prim::Turn { .. } | Prim::Soft { .. });
        let area = if BOUNDED && !wrapper { prim_area(prim, xf, buf.area()) } else { buf.area() };
        if area.empty() {
            continue;
        }
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
            Prim::Ramp { x, y, w, h, from, to, stops } => {
                // The stop offset of a point is its projection onto the
                // gradient vector, in bounding-box fractions: SVG's
                // `objectBoundingBox` axis, so a diagonal on a wide box
                // skews the way the trace's does.
                let ring = map(round_rect(x, y, w, h, 0.0));
                let (dx, dy) = (to.0 - from.0, to.1 - from.1);
                let len2 = (dx * dx + dy * dy).max(f32::EPSILON);
                buf.fill(&[ring], &|px, py| {
                    let (lx, ly) = xf.inv(px, py);
                    let (fx, fy) = ((lx - x) / w - from.0, (ly - y) / h - from.1);
                    stop(stops, ((fx * dx + fy * dy) / len2).clamp(0.0, 1.0))
                });
            }
            Prim::Masked { prims, mask } => {
                // Outside this intersection at least one side is zero.
                // Keep the absolute origin, sample locations and draw
                // order: only temporary allocation and loops shrink.
                let mut layer = Buf::region(area);
                walk_engine::<BOUNDED, BATCH>(&mut layer, prims, palette, xf);
                let mut lum = Buf::region(area);
                walk_engine::<BOUNDED, BATCH>(&mut lum, mask, palette, xf);
                layer.mask(&lum);
                buf.over(&layer);
            }
            Prim::At { x, y, prims } => walk_engine::<BOUNDED, BATCH>(buf, prims, palette, xf.moved(x, y)),
            Prim::Turn { x, y, angle, prims } => {
                walk_engine::<BOUNDED, BATCH>(buf, prims, palette, xf.turned(x, y, angle))
            }
            Prim::Soft { prims } => walk_engine::<BOUNDED, BATCH>(buf, prims, palette, xf),
            Prim::Text { .. }
            | Prim::Wide { .. }
            | Prim::ReusableWide { .. }
            | Prim::Outlined { .. }
            | Prim::Spaced { .. }
            | Prim::Tracked { .. }
            | Prim::TrackedWords { .. }
            | Prim::Grain { .. }
            | Prim::Dots { .. }
            | Prim::Motion { .. }
            | Prim::Pick { .. }
            | Prim::Viewport { .. }
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
    fn a_ramp_reads_its_stops_along_its_axis() {
        const STOPS: &[(f32, Color)] = &[
            (0.0, Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
            (1.0, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
        ];
        // Horizontal over a 10x2 box: column 2 sits at t = 0.25.
        let prims = [Prim::Ramp {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 2.0,
            from: (0.0, 0.0),
            to: (1.0, 0.0),
            stops: STOPS,
        }];
        let out = composite(&prims, &palette(), 10, 2, 1.0);
        assert_eq!(px(&out, 10, 2, 1)[0], 64);
        assert_eq!(px(&out, 10, 7, 0)[0], 191);
        // Vertical over the same box: the row, not the column, decides.
        let prims = [Prim::Ramp {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 2.0,
            from: (0.0, 0.0),
            to: (0.0, 1.0),
            stops: STOPS,
        }];
        let out = composite(&prims, &palette(), 10, 2, 1.0);
        assert_eq!(px(&out, 10, 2, 0)[0], 64);
        assert_eq!(px(&out, 10, 2, 1)[0], 191);
    }

    #[test]
    fn a_mask_passes_its_luminance_as_rsvg_measures_it() {
        // White through #808080, pure red, pure green and half-alpha
        // white: 128, 54, 183, 128 -- the numbers rsvg-convert gives
        // for the same document (see `Prim::Masked`).
        const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        const GREY: Color = Color { r: 128.0 / 255.0, g: 128.0 / 255.0, b: 128.0 / 255.0, a: 1.0 };
        const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
        const HALF_WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 };
        const LAYER: &[Prim] = &[fill_rect(0.0, 0.0, 4.0, 1.0, Ink::Fixed(WHITE))];
        const MASK: &[Prim] = &[
            fill_rect(0.0, 0.0, 1.0, 1.0, Ink::Fixed(GREY)),
            fill_rect(1.0, 0.0, 1.0, 1.0, Ink::Fixed(RED)),
            fill_rect(2.0, 0.0, 1.0, 1.0, Ink::Fixed(GREEN)),
            fill_rect(3.0, 0.0, 1.0, 1.0, Ink::Fixed(HALF_WHITE)),
        ];
        let prims = [
            fill_rect(0.0, 0.0, 5.0, 1.0, Ink::Fixed(Color::BLACK)),
            Prim::Masked { prims: LAYER, mask: MASK },
        ];
        let out = composite(&prims, &palette(), 5, 1, 1.0);
        assert_eq!(px(&out, 5, 0, 0)[0], 128);
        assert_eq!(px(&out, 5, 1, 0)[0], 54);
        assert!((px(&out, 5, 2, 0)[0] as i32 - 183).abs() <= 1);
        assert!((px(&out, 5, 3, 0)[0] as i32 - 128).abs() <= 1);
        assert_eq!(px(&out, 5, 4, 0), [0, 0, 0, 255], "nothing where the mask draws nothing");
    }

    #[test]
    fn cropped_masks_preserve_float_pixels_under_nested_transforms() {
        const STOPS: &[(f32, Color)] = &[
            (0.0, Color { r: 0.3, g: 0.8, b: 0.6, a: 0.35 }),
            (0.43, Color { r: 0.9, g: 0.2, b: 0.4, a: 0.85 }),
            (1.0, Color { r: 0.6, g: 0.7, b: 0.3, a: 0.6 }),
        ];
        const CURVES: &[Seg] = &[
            Seg::Quad { cx: -9.0, cy: 27.0, x: 19.0, y: 28.0 },
            Seg::Cubic { c1x: 44.0, c1y: 52.0, c2x: 40.0, c2y: -15.0, x: 30.0, y: 3.0 },
            Seg::Line(7.0, 3.0),
            Seg::Move(16.0, 11.0),
            Seg::Line(23.0, 10.0),
            Seg::Line(19.0, 18.0),
        ];
        const CONTENT: &[Prim] = &[
            Prim::Round {
                x: 3.125, y: 5.75, w: 34.0, h: 22.0, r: 5.5,
                fill: Some(Ink::Fixed(HALF_GREEN)), stroke: Some(Ink::Fixed(RED)), width: 2.75,
            },
            Prim::Path {
                x: 7.0, y: 3.0, segs: CURVES, close: false,
                fill: Some(Ink::Fixed(HALF_GREEN)), stroke: Some(Ink::Fixed(RED)), width: 3.25,
            },
            Prim::Lobe { x: 21.0, y: 18.0, rx: 14.5, ry: 12.75, stops: STOPS },
            Prim::Ellipse {
                x: 21.5, y: 15.25, rx: 8.75, ry: 5.5,
                fill: Some(Ink::Fixed(HALF_GREEN)), stroke: Some(Ink::Fixed(RED)), width: 2.5,
            },
            Prim::Circle {
                x: 15.25, y: 18.5, r: 4.75,
                fill: Some(Ink::Fixed(HALF_GREEN)), stroke: Some(Ink::Fixed(RED)), width: 1.75,
            },
            Prim::Rect {
                x: 2.5, y: 8.5, w: 15.5, h: 8.0,
                fill: None, stroke: Some(Ink::Fixed(RED)), width: 6.5,
            },
        ];
        const MASK: &[Prim] = &[
            Prim::Ramp {
                x: 5.25, y: 2.75, w: 28.5, h: 32.25,
                from: (0.1, 0.2), to: (0.9, 0.8), stops: STOPS,
            },
            Prim::Turn { x: 8.125, y: 4.5, angle: -19.5, prims: &[
                Prim::Masked { prims: CONTENT, mask: &[
                    Prim::Circle {
                        x: 18.5, y: 15.25, r: 11.5,
                        fill: Some(Ink::Fixed(Color::WHITE)), stroke: None, width: 0.0,
                    },
                ] },
            ] },
        ];
        const MASKED: &[Prim] = &[
            Prim::Masked { prims: CONTENT, mask: MASK },
            // A nested wrapper exercises cropped buffers with a second
            // absolute origin and both an inner and outer mask.
            Prim::At { x: 12.375, y: -1.75, prims: &[
                Prim::Masked { prims: CONTENT, mask: MASK },
            ] },
        ];
        for k in [0.37, 1.0, 1.375, 2.4] {
            for angle in [-31.5, 0.0, 63.75] {
                let scene = [
                    fill_rect(0.0, 0.0, 160.0, 160.0, Ink::Fixed(HALF_GREEN)),
                    Prim::Turn { x: 15.25, y: 5.75, angle, prims: MASKED },
                ];
                let mut reference = Buf::band(96, 0, 80);
                walk_inner::<false>(&mut reference, &scene, &palette(), Xf::scaled(k));
                for (y, h) in [(0, 80), (0, 7), (7, 19), (26, 1), (27, 53)] {
                    let mut cropped = Buf::band(96, y, h);
                    walk(&mut cropped, &scene, &palette(), Xf::scaled(k));
                    assert_eq!(
                        cropped.px, reference.px[y * 96..(y + h) * 96],
                        "k={k}, angle={angle}, band={y}..{}", y + h,
                    );
                }
            }
        }
    }

    #[test]
    fn disjoint_mask_shapes_can_still_share_fractional_pixel_coverage() {
        // The two rectangles do not overlap geometrically, but each
        // covers one quarter of pixel (4, 2). Pixel coverage is masked
        // after rasterization, so the product remains 1/16 there.
        const CONTENT: &[Prim] = &[fill_rect(4.125, 2.0, 0.25, 1.0, Ink::Fixed(RED))];
        const MASK: &[Prim] = &[fill_rect(4.625, 2.0, 0.25, 1.0, Ink::Fixed(Color::WHITE))];
        let scene = [Prim::Masked { prims: CONTENT, mask: MASK }];
        let mut reference = Buf::band(12, 0, 8);
        walk_inner::<false>(&mut reference, &scene, &palette(), Xf::scaled(1.0));
        let mut cropped = Buf::band(12, 0, 8);
        walk(&mut cropped, &scene, &palette(), Xf::scaled(1.0));
        assert_eq!(cropped.px, reference.px);
        assert_eq!(px(&cropped.bytes(), 12, 4, 2), [16, 0, 0, 16]);
    }

    #[test]
    fn cropped_masks_preserve_rect_strokes_with_negative_extents() {
        const MASK: &[Prim] = &[fill_rect(-100.0, -100.0, 300.0, 300.0, Ink::Fixed(Color::WHITE))];
        const RECTANGLES: &[Prim] = &[
            Prim::Rect {
                x: 20.0, y: 20.0, w: -10.0, h: -8.0,
                fill: Some(Ink::Fixed(HALF_GREEN)), stroke: Some(Ink::Fixed(RED)), width: 4.0,
            },
            Prim::Rect {
                x: 25.25, y: 23.5, w: -12.75, h: 5.25,
                fill: None, stroke: Some(Ink::Fixed(HALF_GREEN)), width: 6.5,
            },
        ];
        const MASKED: &[Prim] = &[Prim::Masked { prims: RECTANGLES, mask: MASK }];
        for k in [1.0, 2.4] {
            for angle in [0.0, 33.5] {
                let scene = [Prim::Turn { x: 16.0, y: 4.0, angle, prims: MASKED }];
                let mut reference = Buf::band(128, 0, 128);
                let mut cropped = Buf::band(128, 0, 128);
                walk_inner::<false>(&mut reference, &scene, &palette(), Xf::scaled(k));
                walk(&mut cropped, &scene, &palette(), Xf::scaled(k));
                assert_eq!(cropped.px, reference.px, "k={k}, angle={angle}");
            }
        }
    }

    #[test]
    fn mask_scratch_is_local_and_disjoint_bands_are_empty() {
        const CONTENT: &[Prim] = &[fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(RED))];
        const MASK: &[Prim] = &[fill_rect(402.25, 203.5, 8.5, 6.25, Ink::Fixed(Color::WHITE))];
        let masked = Prim::Masked { prims: CONTENT, mask: MASK };
        let frame = Area { x0: 0, y0: 0, x1: 3840, y1: 2160 };
        let local = prim_area(&masked, Xf::scaled(2.4), frame);
        assert!(!local.empty());
        assert!((local.x1 - local.x0) * (local.y1 - local.y0) < 500);
        let far_band = Area { y0: 1700, y1: 1768, ..frame };
        assert!(prim_area(&masked, Xf::scaled(2.4), far_band).empty());
        let empty = Prim::Masked { prims: CONTENT, mask: &[] };
        assert!(prim_area(&empty, Xf::scaled(2.4), frame).empty());
    }

    #[test]
    fn dashboard_masks_match_the_uncropped_compositor() {
        // Cover real material trees from every era, including the deep
        // Neomil scan/glyph masks. Compare float pixels before encoding,
        // not a tolerance that could conceal a compositing-order change.
        fn check(prims: &[Prim], palette: &Palette) {
            for prim in prims {
                match prim {
                    Prim::Soft { prims } => {
                        let mut reference = Buf::band(160, 0, 90);
                        let mut cropped = Buf::band(160, 0, 90);
                        walk_inner::<false>(&mut reference, prims, palette, Xf::scaled(0.1));
                        walk(&mut cropped, prims, palette, Xf::scaled(0.1));
                        assert_eq!(cropped.px, reference.px);
                    }
                    Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Motion { prims, .. } => {
                        check(prims, palette);
                    }
                    _ => {}
                }
            }
        }
        for era in [crate::style::Era::Entropism, crate::style::Era::Kitsch,
                    crate::style::Era::Neomil, crate::style::Era::Neokitsch] {
            let style = era.style().dashboard_style();
            check(style.dashboard, &style.palette);
        }
    }

    #[test]
    fn neomil_4k_maker_pixels_preserve_separate_quantized_panel_cuts() {
        // A headless 4K comparison found one-channel, one-step differences
        // here after merging the panel images. Keep the true coordinates:
        // translating this ROI changes the gradient and coverage samples.
        // The split at 1768 also crosses the actual 4K image-band boundary.
        let style = crate::style::Era::Neomil.style().dashboard_style();
        let Prim::Soft { prims: ground } = style.dashboard[0] else {
            panic!("the dashboard starts with its composited ground")
        };
        let Prim::Motion { prims: motion, .. } = style.dashboard[1] else {
            panic!("the panel is under its opening motion")
        };
        let groups: Vec<_> = motion.iter().map(|prim| {
            let Prim::Soft { prims } = prim else {
                panic!("the panel preserves its original software-image boundaries")
            };
            *prims
        }).collect();
        assert_eq!(groups.len(), 7);
        let mut layers = vec![SoftLayer { prims: ground, cut: false, render: true }];
        layers.extend(groups.iter().map(|prims| SoftLayer { prims, cut: true, render: true }));

        fn cut<const BOUNDED: bool>(
            area: Area, under: &[&[Prim]], prims: &[Prim], palette: &Palette,
        ) -> Vec<u8> {
            let mut all = Buf::region(area);
            for group in under {
                walk_inner::<BOUNDED>(&mut all, group, palette, Xf::scaled(2.4));
            }
            walk_inner::<BOUNDED>(&mut all, prims, palette, Xf::scaled(2.4));
            let mut own = Buf::region(area);
            walk_inner::<BOUNDED>(&mut own, prims, palette, Xf::scaled(2.4));
            for (pixel, coverage) in all.px.iter_mut().zip(&own.px) {
                if coverage[3] <= 0.0 {
                    *pixel = [0.0; 4];
                }
            }
            all.bytes()
        }

        fn overlay_opaque_cut(dest: &mut [u8], cut: &[u8]) {
            for (pixel, layer) in dest.chunks_exact_mut(4).zip(cut.chunks_exact(4)) {
                // Over this opaque ground each quantized cut pixel is
                // either transparent or a complete sRGB composite.
                assert!(layer[3] == 0 || layer[3] == 255);
                if layer[3] == 255 {
                    pixel.copy_from_slice(layer);
                }
            }
        }

        let roi = Area { x0: 2910, y0: 1720, x1: 3030, y1: 1790 };
        let mut parts = Vec::new();
        let mut whole = Vec::new();
        for area in [roi, Area { y1: 1768, ..roi }, Area { y0: 1768, ..roi }] {
            let mut background = Buf::region(area);
            walk_inner::<false>(&mut background, ground, &style.palette, Xf::scaled(2.4));
            let mut legacy = background.bytes();
            let batched = layers_rgba(&layers, &style.palette, area, 2.4);
            assert_eq!(batched[0].as_ref().unwrap(), &legacy);
            let mut current = legacy.clone();
            let mut under = vec![ground];
            for (index, group) in groups.iter().enumerate() {
                let old_cut = cut::<false>(area, &under, group, &style.palette);
                let bounded_cut = cut::<true>(area, &under, group, &style.palette);
                assert_eq!(bounded_cut, old_cut, "cropping changes original cut in {area:?}");
                let batched_cut = batched[index + 1].as_ref().unwrap();
                assert_eq!(batched_cut, &old_cut, "batching changes original cut {index} in {area:?}");
                overlay_opaque_cut(&mut legacy, &old_cut);
                overlay_opaque_cut(&mut current, batched_cut);
                under.push(*group);
            }
            assert_ne!(current, background.bytes(), "the ROI must exercise panel material");
            assert_eq!(current, legacy, "batching changes separately quantized cuts in {area:?}");
            if area == roi {
                whole = current;
            } else {
                parts.extend(current);
            }
        }
        assert_eq!(whole, parts, "the upload-band boundary must preserve the same pixels");
    }

    #[test]
    fn batched_layers_match_independent_images_and_cuts_with_partial_cache_hits() {
        const STOPS: &[(f32, Color)] = &[
            (0.0, Color { r: 0.2, g: 0.5, b: 0.9, a: 0.7 }),
            (1.0, Color { r: 0.8, g: 0.3, b: 0.1, a: 0.2 }),
        ];
        const GROUPS: &[&[Prim]] = &[
            &[fill_rect(0.0, 0.0, 40.0, 32.0, Ink::Fixed(HALF_GREEN))],
            &[Prim::Masked { prims: &[
                Prim::Ramp {
                    x: 3.125, y: 2.75, w: 20.5, h: 18.25,
                    from: (0.0, 0.1), to: (1.0, 0.9), stops: STOPS,
                },
            ], mask: &[
                fill_rect(5.5, 3.75, 16.25, 13.5, Ink::Fixed(Color::WHITE)),
            ] }],
            // A standalone image after a cut must exclude the prefix,
            // while the next cut must still include this group's paint.
            &[fill_rect(8.375, 7.5, 16.75, 15.25, Ink::Fixed(HALF_GREEN))],
            &[Prim::Circle {
                x: 23.25, y: 19.5, r: 8.75,
                fill: Some(Ink::Fixed(HALF_GREEN)), stroke: Some(Ink::Fixed(RED)), width: 2.75,
            }],
            // Covers pixels outside the earlier cuts. Destructively
            // cutting the retained prefix would lose the ground here.
            &[fill_rect(0.25, 0.75, 38.5, 29.5, Ink::Fixed(HALF_GREEN))],
        ];
        for k in [0.37, 1.375, 2.4] {
            for requested in [
                [true, true, true, true, true],
                [false, true, false, false, true],
                [false, false, true, false, false],
                [false, false, false, false, false],
            ] {
                let cuts = [false, true, false, true, true];
                let layers: Vec<_> = GROUPS.iter().enumerate().map(|(i, prims)| {
                    SoftLayer { prims, cut: cuts[i], render: requested[i] }
                }).collect();
                let actual = composite_layers_bands(&layers, &palette(), 96, 72, k);
                assert_eq!(actual.len(), layers.len());
                let mut under = Vec::new();
                for (i, group) in GROUPS.iter().enumerate() {
                    if !requested[i] {
                        assert!(actual[i].is_empty());
                    } else {
                        let expected = if cuts[i] {
                            composite_over_bands(&under, group, &palette(), 96, 72, k)
                        } else {
                            composite_bands(group, &palette(), 96, 72, k)
                        };
                        assert_eq!(actual[i].len(), expected.len());
                        for (a, e) in actual[i].iter().zip(&expected) {
                            assert_eq!((a.y, a.h), (e.y, e.h));
                            assert!(a.rgba.len() <= BAND_BYTES);
                            assert_eq!(a.rgba, e.rgba, "layer={i}, k={k}, requested={requested:?}");
                        }
                    }
                    under.push(*group);
                }
            }
            // A cut at index zero still applies its own coverage test.
            let first = [SoftLayer { prims: GROUPS[1], cut: true, render: true }];
            assert_eq!(
                join(composite_layers_bands(&first, &palette(), 96, 72, k).remove(0)),
                join(composite_over_bands(&[], GROUPS[1], &palette(), 96, 72, k)),
            );
        }
        assert!(composite_layers_bands(&[], &palette(), 96, 72, 1.0).is_empty());
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
    #[test]
    fn bands_are_the_one_buffer_composite() {
        // Neokitsch's dashboard ground: a page, a turned haze lobe and a
        // masked layer, every kind of walk a band can clip. At 160x90
        // the plan is a few rows a band, so nearly every scanline
        // borders another band's; one buffer walked alone must match.
        let style = crate::style::Era::Neokitsch.style();
        let Prim::Soft { prims } = style.dashboard[0] else {
            panic!("neokitsch's dashboard opens with its soft ground")
        };
        let (w, h, k) = (160, 90, 0.1);
        let plan = bands(w, h);
        assert!(plan.len() > 1, "one band would test nothing: {plan:?}");
        assert_eq!(plan.iter().map(|b| b.1).sum::<u32>(), h);
        let mut one = Buf::band(w as usize, 0, h as usize);
        walk(&mut one, prims, &style.palette, Xf::scaled(k));
        assert_eq!(composite(prims, &style.palette, w, h, k), one.bytes());
        // And the cut, which is where a band's own scratch buffers must
        // share its rows.
        let under: &[&[Prim]] = &[&[fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(RED))]];
        let mut all = Buf::band(w as usize, 0, h as usize);
        for group in under {
            walk(&mut all, group, &style.palette, Xf::scaled(k));
        }
        walk(&mut all, prims, &style.palette, Xf::scaled(k));
        let mut own = Buf::band(w as usize, 0, h as usize);
        walk(&mut own, prims, &style.palette, Xf::scaled(k));
        for (p, o) in all.px.iter_mut().zip(&own.px) {
            if o[3] <= 0.0 {
                *p = [0.0; 4];
            }
        }
        assert_eq!(composite_over(under, prims, &style.palette, w, h, k), all.bytes());
    }
}

#[cfg(test)]
mod bounded_stroke_reference_tests {
    use super::*;

    fn stroke_reference(buf: &mut Buf, rings: &[Vec<(f32, f32)>], closed: bool, width: f32, c: Color) {
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
        let (col0, col1) = buf.cols(x0 - pad, x1 + pad);
        let (row0, row1) = buf.rows(y0 - pad, y1 + pad);
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
                buf.lay(col, row, cov, c);
            }
        }
    }

    fn check(name: &str, rings: Vec<Vec<(f32, f32)>>, closed: bool, widths: &[f32], area: Area) {
        let colors = [
            Color { r: 0.92, g: 0.18, b: 0.3, a: 1.0 },
            Color { r: 0.12, g: 0.68, b: 0.86, a: 0.37 },
        ];
        for &width in widths {
            let mut expected = Buf::region(area);
            let mut actual = Buf::region(area);
            for color in colors {
                stroke_reference(&mut expected, &rings, closed, width, color);
                actual.stroke(&rings, closed, width, color);
            }
            assert_eq!(actual.bytes(), expected.bytes(), "{name}, width {width}");
            assert_eq!(actual.px, expected.px, "float buffer: {name}, width {width}");
        }
    }

    #[test]
    fn bounded_columns_match_full_columns_for_separated_joined_and_clipped_paths() {
        let whole = Area { x0: 0, y0: 0, x1: 120, y1: 100 };
        let clip = Area { x0: 18, y0: 7, x1: 76, y1: 72 };
        check("disjoint long sides", vec![vec![(10.2, 2.3), (11.3, 95.7), (108.4, 95.7), (109.7, 2.3)]], false, &[0.4, 1.0, 3.6, 18.0], whole);
        check("crossing and overlapping diagonals", vec![vec![(2.7, 4.1), (105.3, 90.6)], vec![(107.4, 3.3), (2.1, 92.1)], vec![(20.1, 30.6), (90.2, 55.4)]], false, &[0.35, 1.7, 9.5], whole);
        check("closed joins", vec![vec![(14.1, 10.4), (93.7, 12.2), (108.2, 59.8), (88.2, 86.4), (11.4, 78.8)]], true, &[0.6, 2.3, 12.2], whole);
        check("clipped paths", vec![vec![(-15.3, 10.2), (50.4, 3.2), (130.7, 40.1), (13.3, 91.2)]], true, &[0.3, 2.5, 16.0], clip);
        check("short and zero length", vec![vec![(20.2, 30.7), (20.2, 30.7), (21.1, 31.4)], vec![(22.2, 31.1), (22.2, 31.1)]], false, &[0.4, 1.2, 6.0], whole);
    }

    #[test]
    fn bounded_columns_match_full_columns_for_transforms_and_fallbacks() {
        let original = vec![(2.2, 3.8), (35.1, 4.7), (36.5, 22.1), (3.3, 23.9)];
        let angle = 0.4f32;
        let ring: Vec<_> = original.into_iter().map(|(x, y)| {
            let (x, y) = (x * 1.37 - 12.6, y * 0.83 + 19.1);
            (x * angle.cos() - y * angle.sin() + 44.0, x * angle.sin() + y * angle.cos() - 10.0)
        }).collect();
        check("affine rotation and shift", vec![ring], true, &[0.6, 2.1, 8.2], Area { x0: 0, y0: 0, x1: 100, y1: 90 });
        check("negative width fallback", vec![vec![(12.2, 12.1), (41.2, 54.4)]], false, &[-0.5], Area { x0: 0, y0: 0, x1: 80, y1: 80 });
        check("nonfinite width fallback", vec![vec![(12.2, 12.1), (41.2, 54.4)]], false, &[f32::NAN], Area { x0: 0, y0: 0, x1: 80, y1: 80 });
        check("nonfinite endpoint fallback", vec![vec![(f32::NAN, 12.1), (41.2, 54.4)]], false, &[2.0], Area { x0: 0, y0: 0, x1: 80, y1: 80 });
        check("infinite width fallback", vec![vec![(12.2, 12.1), (41.2, 54.4)]], false, &[f32::INFINITY], Area { x0: 0, y0: 0, x1: 80, y1: 80 });
        check("huge finite endpoint cancellation", vec![vec![(100_000_000.0, 50.0), (4.0, 50.0)], vec![(0.0, 90.0), (0.0, 90.0)]], false, &[0.4], Area { x0: 0, y0: 0, x1: 120, y1: 100 });
    }
}

#[cfg(test)]
mod stroke_run_reference_tests {
    use super::*;
    use crate::style::{fill_rect, Ink};

    const CLOSED: &[Seg] = &[
        Seg::Line(83.4, 9.2), Seg::Line(85.1, 24.3), Seg::Line(80.2, 29.7),
        Seg::Line(79.3, 88.1), Seg::Line(8.6, 88.1),
    ];
    const OPEN: &[Seg] = &[
        Seg::Line(17.2, 88.4), Seg::Line(99.8, 88.4),
    ];
    const OTHER: &[Seg] = &[Seg::Line(20.2, 88.4), Seg::Line(98.1, 88.4)];
    const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0042 };
    const STOPS: &[(f32, Color)] = &[
        (0.0, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }),
        (1.0, Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 }),
    ];
    const OPEN_BANDS: &[Prim] = &[
        Prim::Path { x: 99.8, y: 9.2, segs: OPEN, close: false, fill: None,
            stroke: Some(Ink::Fixed(WHITE)), width: 3.18 },
        Prim::Path { x: 99.8, y: 9.2, segs: OPEN, close: false, fill: None,
            stroke: Some(Ink::Fixed(Color { a: 0.0045, ..WHITE })), width: 1.54 },
        Prim::Path { x: 99.8, y: 9.2, segs: OPEN, close: false, fill: None,
            stroke: Some(Ink::Fixed(Color { a: 0.0048, ..WHITE })), width: 0.17 },
    ];
    const C4_MASKED: &[Prim] = &[Prim::Masked {
        prims: &[Prim::Masked {
            prims: &[fill_rect(6.0, 7.0, 97.0, 85.0, Ink::Fixed(Color { r: 0.74, g: 0.13, b: 0.12, a: 1.0 }))],
            mask: OPEN_BANDS,
        }],
        mask: &[Prim::Ramp { x: 6.0, y: 7.0, w: 97.0, h: 85.0,
            from: (0.8, 0.0), to: (1.0, 0.0), stops: STOPS }],
    }];

    fn path(segs: &'static [Seg], closed: bool, width: f32, color: Color) -> Prim {
        Prim::Path { x: if closed { 8.6 } else { 99.8 }, y: 9.2, segs,
            close: closed, fill: None, stroke: Some(Ink::Fixed(color)), width }
    }

    fn equal(prims: &[Prim], area: Area, xf: Xf) {
        let palette = crate::style::Era::Neomil.style().palette;
        let mut literal = Buf::region(area);
        let mut batched = Buf::region(area);
        walk_engine::<true, false>(&mut literal, prims, &palette, xf);
        walk_engine::<true, true>(&mut batched, prims, &palette, xf);
        assert_eq!(literal.px.iter().flat_map(|rgba| rgba.iter().map(|v| v.to_bits())).collect::<Vec<_>>(),
            batched.px.iter().flat_map(|rgba| rgba.iter().map(|v| v.to_bits())).collect::<Vec<_>>(), "float buffer bits");
        assert_eq!(literal.bytes(), batched.bytes(), "RGBA bytes");
    }

    #[test]
    fn closed_joins_and_open_caps_match_at_fractional_rotated_and_clipped_origins() {
        let mut closed = Vec::new();
        for (i, width) in [3.18, 2.75, 1.9, 1.1, 0.45, 0.17].into_iter().enumerate() {
            closed.push(path(CLOSED, true, width, Color { a: 0.0039 + i as f32 * 0.00013, ..WHITE }));
        }
        let clipped = Area { x0: 7, y0: 13, x1: 99, y1: 91 };
        for xf in [Xf::scaled(1.0), Xf::scaled(0.41666666).moved(14.2, -3.7),
            Xf::scaled(0.9).turned(12.3, 11.7, 17.5)] {
            assert_eq!(white_stroke_run(&closed, xf), closed.len(), "normal band run is eligible");
            equal(&closed, Area { x0: 0, y0: 0, x1: 120, y1: 110 }, xf);
            equal(&closed, clipped, xf);
            equal(C4_MASKED, Area { x0: 3, y0: 0, x1: 112, y1: 104 }, xf);
        }
    }

    #[test]
    fn interruptions_and_ineligible_strokes_keep_literal_semantics() {
        let white = path(OPEN, false, 3.18, WHITE);
        let thin = path(OPEN, false, 0.17, Color { a: 0.0048, ..WHITE });
        let variants = [
            vec![white, path(OTHER, false, 1.0, WHITE), thin],
            vec![white, path(OPEN, false, 1.0, Color { r: 0.9, ..WHITE }), thin],
            vec![white, path(OPEN, false, 0.0, WHITE), thin],
            vec![white, path(OPEN, false, -0.4, WHITE), thin],
            vec![white, path(OPEN, false, f32::INFINITY, WHITE), thin],
            vec![white, path(OPEN, false, f32::NAN, WHITE), thin],
            vec![white, path(OPEN, false, 100_000.0, WHITE), thin],
        ];
        let area = Area { x0: 0, y0: 0, x1: 120, y1: 110 };
        for seq in variants { equal(&seq, area, Xf::scaled(1.0)); }
        let huge = path(OPEN, false, 3.0, WHITE);
        assert_eq!(white_stroke_run(&[huge, thin], Xf::scaled(100_000.0)), 0);
        assert_eq!(white_stroke_run(&[white, thin], Xf::scaled(f32::INFINITY)), 0);
        assert_eq!(white_stroke_run(&[white, thin], Xf::scaled(f32::NAN)), 0);
    }

    #[test]
    fn active_rows_spans_and_unsafe_coordinates_match_literal_strokes() {
        const ROWS: &[Seg] = &[
            Seg::Line(100.0, 12.5), Seg::Move(8.0, 16.25), Seg::Line(100.0, 16.25),
        ];
        const APART: &[Seg] = &[
            Seg::Line(24.0, 12.5), Seg::Move(82.0, 16.25), Seg::Line(102.0, 16.25),
        ];
        let area = Area { x0: 0, y0: 0, x1: 120, y1: 100 };
        for segs in [ROWS, APART] {
            let bands = [6.0, 2.5, 0.8].map(|width| Prim::Path {
                x: 8.0, y: 0.0, segs, close: false, fill: None,
                stroke: Some(Ink::Fixed(WHITE)), width,
            });
            assert_eq!(white_stroke_run(&bands, Xf::scaled(1.0)), 3);
            equal(&bands, area, Xf::scaled(1.0));
            equal(&bands, Area { x0: 13, y0: 8, x1: 91, y1: 24 }, Xf::scaled(1.0));
        }

        // This finite cancellation witness has eligible widths but an
        // unsafe transformed endpoint. The batch must take the literal
        // traversal: f32 endpoint reconstruction can reach column zero.
        const HUGE: &[Seg] = &[
            Seg::Line(4.0, 50.0), Seg::Move(0.0, 90.0), Seg::Line(0.0, 90.0),
        ];
        let pair = [0.4, 0.2].map(|width| Prim::Path {
            x: 100_000_000.0, y: 50.0, segs: HUGE, close: false, fill: None,
            stroke: Some(Ink::Fixed(WHITE)), width,
        });
        assert_eq!(white_stroke_run(&pair, Xf::scaled(1.0)), 2);
        let palette = crate::style::Era::Neomil.style().palette;
        let mut literal = Buf::region(area);
        walk_engine::<true, false>(&mut literal, &pair, &palette, Xf::scaled(1.0));
        assert!(literal.px.iter().any(|rgba| rgba.iter().any(|&v| v != 0.0)), "literal witness has coverage");
        equal(&pair, area, Xf::scaled(1.0));

        for bad in [f32::NAN, f32::INFINITY] {
            let segs: &'static [Seg] = Box::leak(vec![Seg::Line(bad, 50.0)].into_boxed_slice());
            let pair = [0.4, 0.2].map(|width| Prim::Path {
                x: 4.0, y: 50.0, segs, close: false, fill: None,
                stroke: Some(Ink::Fixed(WHITE)), width,
            });
            assert_eq!(white_stroke_run(&pair, Xf::scaled(1.0)), 2);
            equal(&pair, area, Xf::scaled(1.0));
        }
    }
}
