//! Neo-kitsch wood veneer as a generated texture.
//!
//! Owner decision 2026-11-13 (`docs/review-2026-11-13/DECISIONS.md`, 3):
//! the selected surfaces wear a pre-rendered texture clipped to their
//! shapes instead of drawn vector grain, accepting an image asset and
//! less clean scaling at odd sizes. The textures are *generated* by
//! `scripts/make-veneer.py` from measured parameters -- the reference art
//! is third-party and is never redistributed -- and committed as
//! `docs/neokitsch/veneer/*.png`, where the traces reference the same
//! files. `docs/neokitsch/veneer-texture.md` has the measurements, the
//! model and the costs.
//!
//! Each texture is one surface's bounding box at 2.4 texels per design
//! px (3840x2160 is 2.4x the 1600x900 design, so a 4K frame samples it
//! 1:1), stored as an 8-bit indexed PNG whose palette is the surface's
//! colour ramp. [`image`] clips it to the surface's own fill prims in
//! software (`screens/soft.rs`'s rasteriser, so the edge coverage is the
//! one every composited fill gets), area-filters it to the canvas scale,
//! adds the thin light rim, and hands the scene a premultiplied RGBA
//! image of just the surface's pixel box. Images are cached per surface,
//! position and scale.
//!
//! The canvas draws images above every mesh in its layer, so a scene
//! that holds a veneer is painted in layers, one per veneer
//! (`screens/scene.rs`, [`crate::widgets::layered`]).

use crate::style::Prim;
use iced::widget::image::Handle;
use iced::{Color, Rectangle};
use std::sync::{Arc, Mutex, OnceLock};

/// Texels per design px.
pub const TEXELS: f32 = 2.4;

/// The generated textures, one per veneered surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Texture {
    /// Dashboard: the selected cascade card (`#ncardsel`), card-local.
    DashboardEmail,
    /// Dashboard: the right-hand detail panel's body.
    DashboardPanel,
    /// Mailbox: the selected row's bar.
    MailboxRow,
    /// Store: the selected category button.
    StoreNav,
    /// Store: the selected product card's body and socket band.
    StoreCard,
    /// Store: the BASKET plate's lower band.
    StoreBasket,
}

impl Texture {
    pub const ALL: [Texture; 6] = [
        Texture::DashboardEmail,
        Texture::DashboardPanel,
        Texture::MailboxRow,
        Texture::StoreNav,
        Texture::StoreCard,
        Texture::StoreBasket,
    ];

    /// The asset's file stem in `docs/neokitsch/veneer/`.
    pub fn name(self) -> &'static str {
        match self {
            Texture::DashboardEmail => "dashboard-email",
            Texture::DashboardPanel => "dashboard-panel",
            Texture::MailboxRow => "mailbox-row",
            Texture::StoreNav => "store-nav",
            Texture::StoreCard => "store-card",
            Texture::StoreBasket => "store-basket",
        }
    }

    fn png(self) -> &'static [u8] {
        match self {
            Texture::DashboardEmail => include_bytes!("../docs/neokitsch/veneer/dashboard-email.png"),
            Texture::DashboardPanel => include_bytes!("../docs/neokitsch/veneer/dashboard-panel.png"),
            Texture::MailboxRow => include_bytes!("../docs/neokitsch/veneer/mailbox-row.png"),
            Texture::StoreNav => include_bytes!("../docs/neokitsch/veneer/store-nav.png"),
            Texture::StoreCard => include_bytes!("../docs/neokitsch/veneer/store-card.png"),
            Texture::StoreBasket => include_bytes!("../docs/neokitsch/veneer/store-basket.png"),
        }
    }

    /// The surface's bounding box in design px: the texture's extent,
    /// and `SURFACES[..]['size']` in the generator.
    pub fn size(self) -> (f32, f32) {
        match self {
            Texture::DashboardEmail => (90.5, 325.2),
            Texture::DashboardPanel => (230.4, 309.0),
            Texture::MailboxRow => (477.0, 55.0),
            Texture::StoreNav => (200.6, 38.6),
            Texture::StoreCard => (262.1, 241.7),
            Texture::StoreBasket => (204.6, 45.0),
        }
    }

    /// The thin light rim inside the edge: colour and width in design
    /// px. Measured as a 1-2 native px highlight, +10..15 green levels,
    /// along the mailbox bar and the store card's edges; the EMAIL card
    /// keeps its own traced 1.2 outline instead.
    fn rim(self) -> Option<(Color, f32)> {
        let light = Color::from_rgba8(0xff, 0xe2, 0xa6, 0.55);
        match self {
            Texture::DashboardEmail => None,
            _ => Some((light, 0.5)),
        }
    }

    fn decoded(self) -> &'static Decoded {
        static CELLS: [OnceLock<Decoded>; 6] = [const { OnceLock::new() }; 6];
        let i = Texture::ALL.iter().position(|&t| t == self).unwrap_or(0);
        CELLS[i].get_or_init(|| Decoded::from_png(self.png()))
    }
}

/// An indexed texture: one palette index per texel, the palette in
/// encoded sRGB, and a chain of 2x2 box-filtered RGB levels below it.
/// 1600x900, which reads 2.4 texels per pixel, box-filters level 0
/// directly ([`Decoded::area`]); the levels serve smaller scales.
struct Decoded {
    w: usize,
    h: usize,
    idx: Vec<u8>,
    pal: Vec<[f32; 3]>,
    /// Level 1, 2, ...: half, quarter, ... size, 8-bit encoded RGB.
    mips: Vec<Mip>,
}

struct Mip {
    w: usize,
    h: usize,
    rgb: Vec<[u8; 3]>,
}

impl Decoded {
    /// Decode an 8-bit indexed PNG without expanding it: the indices and
    /// the palette are kept apart, a quarter of the memory of RGBA.
    fn from_png(bytes: &[u8]) -> Self {
        let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
        decoder.set_transformations(png::Transformations::IDENTITY);
        let mut reader = decoder.read_info().expect("veneer texture is a PNG");
        let info = reader.info();
        assert_eq!(info.color_type, png::ColorType::Indexed, "veneer texture is indexed");
        assert_eq!(info.bit_depth, png::BitDepth::Eight, "veneer texture is 8-bit");
        let pal = info
            .palette
            .as_ref()
            .expect("veneer texture has a palette")
            .chunks_exact(3)
            .map(|c| [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0])
            .collect::<Vec<_>>();
        let mut idx = vec![0; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut idx).expect("veneer texture decodes");
        idx.truncate(frame.buffer_size());
        let mut d = Decoded { w: frame.width as usize, h: frame.height as usize, idx, pal, mips: Vec::new() };
        // Mips down to a few texels: each the 2x2 mean of the one above
        // (edge texels repeat on odd sizes), in encoded sRGB as rsvg's
        // own image filtering averages.
        let (mut w, mut h) = (d.w, d.h);
        while w > 4 && h > 4 {
            let (nw, nh) = (w.div_ceil(2), h.div_ceil(2));
            let level = d.mips.len();
            let rgb = (0..nh)
                .flat_map(|y| (0..nw).map(move |x| (x, y)))
                .map(|(x, y)| {
                    let mut sum = [0.0f32; 3];
                    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                        let c = d.level_texel(level, (2 * x + dx) as isize, (2 * y + dy) as isize);
                        for i in 0..3 {
                            sum[i] += c[i];
                        }
                    }
                    sum.map(|v| (v / 4.0 * 255.0).round().clamp(0.0, 255.0) as u8)
                })
                .collect();
            d.mips.push(Mip { w: nw, h: nh, rgb });
            (w, h) = (nw, nh);
        }
        d
    }

    /// Texel `(x, y)` of `level` (0 is the indexed original), clamped.
    fn level_texel(&self, level: usize, x: isize, y: isize) -> [f32; 3] {
        if level == 0 {
            let x = x.clamp(0, self.w as isize - 1) as usize;
            let y = y.clamp(0, self.h as isize - 1) as usize;
            return self.pal.get(self.idx[y * self.w + x] as usize).copied().unwrap_or([0.0; 3]);
        }
        let m = &self.mips[level - 1];
        let x = x.clamp(0, m.w as isize - 1) as usize;
        let y = y.clamp(0, m.h as isize - 1) as usize;
        m.rgb[y * m.w + x].map(|c| c as f32 / 255.0)
    }

    /// Bilinear on `level` at its texel coordinates `(u, v)`, texel
    /// centres at `i + 0.5`.
    fn bilinear(&self, level: usize, u: f32, v: f32) -> [f32; 3] {
        let (u, v) = (u - 0.5, v - 0.5);
        let (x0, y0) = (u.floor(), v.floor());
        let (fx, fy) = (u - x0, v - y0);
        let (x0, y0) = (x0 as isize, y0 as isize);
        let a = self.level_texel(level, x0, y0);
        let b = self.level_texel(level, x0 + 1, y0);
        let c = self.level_texel(level, x0, y0 + 1);
        let d = self.level_texel(level, x0 + 1, y0 + 1);
        std::array::from_fn(|i| {
            let top = a[i] + (b[i] - a[i]) * fx;
            let bot = c[i] + (d[i] - c[i]) * fx;
            top + (bot - top) * fy
        })
    }

    /// The mean over a `foot`-texel square centred on `(u, v)` (level-0
    /// texel coordinates): plain bilinear at or above 1:1; below it, an
    /// exact box -- every texel the footprint touches, weighted by the
    /// area it covers -- on level 0 while the footprint is under four
    /// texels (1600x900 reads 2.4), and on the mip that brings it to two
    /// to four texels beyond that.
    ///
    /// The exact box is the downscale the reference itself gets at
    /// 1600x900 and keeps a line's contrast where its pitch allows: the
    /// previous mip-then-tap path blurred over about 3.5 texels, which
    /// turned bunched grain to mud.
    fn area(&self, u: f32, v: f32, foot: f32) -> [f32; 3] {
        if foot <= 1.0 {
            return self.bilinear(0, u, v);
        }
        let level = if foot < 4.0 { 0 } else { ((foot / 2.0).log2().floor() as usize).min(self.mips.len()) };
        let scale = (1u32 << level) as f32;
        let (u, v, foot) = (u / scale, v / scale, foot / scale);
        let half = foot / 2.0;
        // Per-axis coverage of texel i (spanning [i, i + 1]) by the
        // footprint [c - half, c + half].
        let weights = |c: f32| {
            let (lo, hi) = (c - half, c + half);
            let first = lo.floor() as isize;
            let last = (hi.ceil() as isize - 1).max(first);
            (first..=last).map(move |i| {
                let w = (hi.min(i as f32 + 1.0) - lo.max(i as f32)).max(0.0);
                (i, w)
            })
        };
        let mut sum = [0.0f32; 3];
        let mut total = 0.0f32;
        for (y, wy) in weights(v) {
            if wy <= 0.0 {
                continue;
            }
            for (x, wx) in weights(u) {
                let w = wx * wy;
                if w <= 0.0 {
                    continue;
                }
                let c = self.level_texel(level, x, y);
                for ch in 0..3 {
                    sum[ch] += c[ch] * w;
                }
                total += w;
            }
        }
        sum.map(|s| s / total.max(1e-6))
    }
}

/// A veneer clipped to its surface at one canvas scale: the surface's
/// pixel box in canvas coordinates and its premultiplied RGBA image.
#[derive(Debug)]
pub struct Image {
    pub bounds: Rectangle,
    pub handle: Handle,
    bytes: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Key {
    texture: Texture,
    shape: (usize, usize),
    /// Texture origin, translation and scale, as bits: a fractional
    /// position is a different raster.
    at: [u32; 5],
}

/// Retained images: a handful of surfaces at a few sizes. The limit is
/// generous for that and small beside `SoftCache`'s 384 MiB.
const CACHE_BYTES: usize = 64 * 1024 * 1024;
const CACHE_ENTRIES: usize = 64;

fn cache() -> &'static Mutex<Vec<(Key, Arc<Image>)>> {
    static CACHE: OnceLock<Mutex<Vec<(Key, Arc<Image>)>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(Vec::new()))
}

/// `texture` with its top-left at design `(x, y)`, clipped to the fill
/// prims of `shape` (rects, rounded rects and paths, in the same local
/// coordinates), all translated by `(ox, oy)` and scaled by `k` to
/// canvas pixels. `None` when the shape covers no pixel.
pub fn image(texture: Texture, x: f32, y: f32, shape: &'static [Prim], ox: f32, oy: f32, k: f32) -> Option<Arc<Image>> {
    if !(k > 0.0) || !k.is_finite() {
        return None;
    }
    let key = Key {
        texture,
        shape: (shape.as_ptr() as usize, shape.len()),
        at: [x.to_bits(), y.to_bits(), ox.to_bits(), oy.to_bits(), k.to_bits()],
    };
    {
        let mut cache = cache().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(i) = cache.iter().position(|(k, _)| *k == key) {
            let entry = cache.remove(i);
            let image = entry.1.clone();
            cache.push(entry);
            return Some(image);
        }
    }
    let tex = texture.decoded();
    let (tw, th) = texture.size();
    // Texels per canvas pixel.
    let foot = TEXELS / k;
    let (sx, sy) = (tex.w as f32 / tw, tex.h as f32 / th);
    let paint = |px: f32, py: f32| {
        let u = (px / k - ox - x) * sx;
        let v = (py / k - oy - y) * sy;
        let [r, g, b] = tex.area(u, v, foot);
        Color { r, g, b, a: 1.0 }
    };
    let (x0, y0, w, h, rgba) = crate::screens::soft::clipped(shape, ox, oy, k, texture.rim(), &paint)?;
    let bytes = rgba.len();
    let image = Arc::new(Image {
        bounds: Rectangle { x: x0 as f32, y: y0 as f32, width: w as f32, height: h as f32 },
        handle: Handle::from_rgba(w as u32, h as u32, rgba),
        bytes,
    });
    let mut cache = cache().lock().unwrap_or_else(|e| e.into_inner());
    let mut total: usize = cache.iter().map(|(_, i)| i.bytes).sum();
    while !cache.is_empty() && (total + bytes > CACHE_BYTES || cache.len() >= CACHE_ENTRIES) {
        total -= cache.remove(0).1.bytes;
    }
    if bytes <= CACHE_BYTES {
        cache.push((key, image.clone()));
    }
    Some(image)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::fill_rect;
    use crate::style::Ink;

    #[test]
    fn every_texture_decodes_at_its_design_size_and_resolution() {
        for t in Texture::ALL {
            let d = t.decoded();
            let (w, h) = t.size();
            // The generator rounds `size * 2.4` up in f64; f32 lands a hair over.
            let texels = |v: f32| (v * TEXELS - 1e-3).ceil() as usize;
            assert_eq!((d.w, d.h), (texels(w), texels(h)), "{}", t.name());
            assert_eq!(d.pal.len(), 256, "{}", t.name());
            assert_eq!(d.idx.len(), d.w * d.h, "{}", t.name());
        }
    }

    #[test]
    fn the_ramp_runs_light_to_dark_and_the_fill_stays_light() {
        for t in Texture::ALL {
            let d = t.decoded();
            let lum = |c: [f32; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
            assert!(lum(d.pal[0]) > lum(d.pal[255]), "{}", t.name());
            // Most texels are fill, not line: the median index stays in
            // the light third of the ramp so dark ink keeps its contrast.
            let mut sorted = d.idx.clone();
            sorted.sort_unstable();
            assert!(sorted[sorted.len() / 2] < 85, "{} median index {}", t.name(), sorted[sorted.len() / 2]);
        }
    }

    /// Cold cost of each surface's image (decode + clip + filter), at
    /// 1600x900 and 3840x2160. `cargo test --release --lib -- --ignored
    /// veneer_cold_cost --nocapture`; numbers in veneer-texture.md.
    #[test]
    #[ignore]
    fn veneer_cold_cost() {
        let shapes: [(Texture, f32, f32, &'static [Prim]); 6] = [
            (Texture::DashboardEmail, 0.0, 0.0, &BOXES[0..1]),
            (Texture::DashboardPanel, 0.0, 0.0, &BOXES[1..2]),
            (Texture::MailboxRow, 0.0, 0.0, &BOXES[2..3]),
            (Texture::StoreNav, 0.0, 0.0, &BOXES[3..4]),
            (Texture::StoreCard, 0.0, 0.0, &BOXES[4..5]),
            (Texture::StoreBasket, 0.0, 0.0, &BOXES[5..6]),
        ];
        let t = std::time::Instant::now();
        for (tex, ..) in shapes {
            tex.decoded();
        }
        println!("decode all six: {:.2} ms", t.elapsed().as_secs_f64() * 1e3);
        for k in [1.0f32, 2.4] {
            for (tex, x, y, shape) in shapes {
                let t = std::time::Instant::now();
                let img = image(tex, x, y, shape, 10.0, 10.0, k).unwrap();
                println!("{:16} k={k}: {:7.2} ms, {}x{} px, {} KiB RGBA", tex.name(), t.elapsed().as_secs_f64() * 1e3,
                    img.bounds.width, img.bounds.height, img.bytes / 1024);
            }
        }
    }

    static BOXES: [Prim; 6] = [
        fill_rect(0.0, 0.0, 90.5, 325.2, Ink::Bg),
        fill_rect(0.0, 0.0, 230.4, 309.0, Ink::Bg),
        fill_rect(0.0, 0.0, 477.0, 55.0, Ink::Bg),
        fill_rect(0.0, 0.0, 200.6, 38.6, Ink::Bg),
        fill_rect(0.0, 0.0, 262.1, 241.7, Ink::Bg),
        fill_rect(0.0, 0.0, 204.6, 45.0, Ink::Bg),
    ];

    static SQUARE: [Prim; 1] = [fill_rect(0.0, 0.0, 10.0, 10.0, Ink::Bg)];

    #[test]
    fn an_image_covers_its_shape_box_and_is_cached() {
        let a = image(Texture::StoreNav, 0.0, 0.0, &SQUARE, 100.0, 50.0, 2.4).expect("image");
        assert_eq!(a.bounds.x.floor(), a.bounds.x);
        assert!(a.bounds.x <= 240.0 && a.bounds.x + a.bounds.width >= 264.0);
        assert!(a.bounds.y <= 120.0 && a.bounds.y + a.bounds.height >= 144.0);
        let b = image(Texture::StoreNav, 0.0, 0.0, &SQUARE, 100.0, 50.0, 2.4).expect("image");
        assert!(Arc::ptr_eq(&a, &b), "a second lookup reuses the raster and its handle");
        let c = image(Texture::StoreNav, 0.0, 0.0, &SQUARE, 100.0, 50.0, 1.0).expect("image");
        assert!(!Arc::ptr_eq(&a, &c), "another scale is another raster");
    }
}
