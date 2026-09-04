//! The access screen, in any era.
//!
//! The sharpest test of the vocabulary in the crate, and not for the
//! reason the old version of this file claimed. It is not "a label, a
//! field, one button and the era's chrome": the four
//! `docs/<era>/login-trace.svg` traces are four different compositions.
//! Entropism sets one field alone in an empty frame over a solid sage
//! band; neomil deals three dossier cards under a badge header;
//! kitsch stands three chip-headed guest rows inside a full-height
//! bracket with a barcode in its foot; neokitsch offers two identical
//! entry groups over a band of twenty-two wires.
//!
//! What they *do* share is a grammar -- some number of account slots,
//! exactly one of which you may sign into, each with a mark, a name, a
//! footnote and a control -- and that grammar is
//! [`crate::style::Slot`]. Everything below reads it off the era table
//! and draws it; nothing here names an era. The measured coordinates
//! live in the tables' `--- login ---` blocks because they are
//! sampled facts about an era, the same way [`crate::style::Style::store`]
//! and [`crate::style::Style::dashboard`] are.
//!
//! Why one canvas rather than a column of widgets: the traces carry
//! measured coordinates -- "field, x 563..922, y 414..447" -- and the
//! gate this screen is built against (`scripts/fidelity_check.sh
//! --implementation <era> login`, see `docs/PIPELINE.md`) matches
//! bounding boxes at an IoU of 0.65. Flow layout cannot hit a
//! transcribed rectangle to five pixels, and the crate already draws a
//! whole screen this way -- `screens::dashboard`'s ops backdrop. The
//! design frame is 1600x900 and everything scales from it, so the
//! screen is not pinned to that size.

use crate::style::{
    Access, Colophon, Emblem, Fixture, Ink, Legend, Masthead, Plate, Plot, Slot, Style, Wash,
};
use crate::widgets::ground;
use iced::widget::{canvas, stack};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

pub struct Login {
    pub style: Style,
    /// The wash is rasterised a pixel at a time (see [`wash_image`]),
    /// which is work worth doing once rather than every frame.
    backdrop: canvas::Cache,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Login {
    pub fn new(style: Style) -> Self {
        Login {
            style,
            backdrop: canvas::Cache::new(),
        }
    }

    pub fn title(&self) -> String {
        format!("ACCESS — {}", self.style.era.name())
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        // Three layers, and the wash has to be its own: `iced_wgpu`
        // buckets a canvas's geometry into meshes, images and text and
        // draws the buckets in that order, so an image is painted over
        // every shape in the same canvas no matter when it was asked
        // for. The wash is an image (see `wash_image`), so it goes in a
        // canvas of its own, under the one that draws the screen.
        stack![
            ground(&self.style),
            canvas(Backdrop {
                style: self.style,
                cache: &self.backdrop,
            })
            .width(Length::Fill)
            .height(Length::Fill),
            canvas(Art { style: self.style })
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .into()
    }
}

// ------------------------------------------------------------- the frame

/// The trace's frame. Every coordinate in the era tables is in these
/// units and this maps them onto whatever window the screen is given.
#[derive(Debug, Clone, Copy)]
struct Grid {
    sx: f32,
    sy: f32,
}

const DESIGN_W: f32 = 1600.0;
const DESIGN_H: f32 = 900.0;

impl Grid {
    fn new(bounds: Size) -> Grid {
        Grid {
            sx: bounds.width / DESIGN_W,
            sy: bounds.height / DESIGN_H,
        }
    }

    fn at(self, x: f32, y: f32) -> Point {
        Point::new(x * self.sx, y * self.sy)
    }

    fn size(self, w: f32, h: f32) -> Size {
        Size::new(w * self.sx, h * self.sy)
    }

    /// A length that is neither horizontal nor vertical -- a stroke
    /// width, a text size. The mean keeps a hairline a hairline under a
    /// non-square window.
    fn span(self, v: f32) -> f32 {
        v * (self.sx + self.sy) / 2.0
    }
}

/// Rajdhani's ascent, in ems, read off the shipped face's `hhea`
/// (930/1000). The traces position text by its baseline, because that
/// is what an SVG `<text y=...>` means; `fill_text` positions the top
/// of the line box. Pinning the line height to the face's own natural
/// line (1.276em, `ascent - descent`) means the two differ by exactly
/// the ascent with no centring term in between.
const ASCENT: f32 = 0.93;
const LINE: f32 = 1.276;

// ---------------------------------------------------------------- plates

/// The outline of a [`Plate`], as a path in window coordinates.
///
/// One walk for every shape on the screen, the way
/// [`crate::widgets::surface::outline`] does it for the widget set: down
/// the four edges, cutting each corner by its own bevel, with a
/// shoulder in the top edge where the era's bar has one.
fn plate_path(g: Grid, plate: &Plate) -> canvas::Path {
    let Plot { x, y, w, h } = plate.at;
    let b = plate.bevel;
    let (top, step) = match plate.step {
        Some(s) => (y + s.drop, Some(s)),
        None => (y, None),
    };

    canvas::Path::new(|p| {
        p.move_to(g.at(x + b.tl, top));
        if let Some(s) = step {
            p.line_to(g.at(s.x, top));
            p.line_to(g.at(s.x + s.run, y));
        }
        p.line_to(g.at(x + w - b.tr, y));
        if b.tr > 0.0 {
            p.line_to(g.at(x + w, y + b.tr));
        }
        p.line_to(g.at(x + w, y + h - b.br));
        if b.br > 0.0 {
            p.line_to(g.at(x + w - b.br, y + h));
        }
        p.line_to(g.at(x + b.bl, y + h));
        if b.bl > 0.0 {
            p.line_to(g.at(x, y + h - b.bl));
        }
        p.line_to(g.at(x, top + b.tl));
        if b.tl > 0.0 {
            p.line_to(g.at(x + b.tl, top));
        }
        p.close();
    })
}

/// The natural width of a run, measured through the same shaper that
/// will draw it.
///
/// `canvas::Text` has no letter-spacing and no width fitting -- the SVG
/// `textLength` the traces used to reach for is a no-op in librsvg too,
/// which is why the polished traces carry transforms instead. Measuring
/// is what lets a `tracking` figure be honoured as an extent rather
/// than ignored.
/// Each character's advance in a run, by prefix measurement.
///
/// Prefixes rather than characters on their own, because a lone space
/// measures zero -- the shaper trims it -- and because a difference the
/// shaper makes between neighbours belongs to the pair, not to either
/// glyph.
use super::scene::advances;

struct Pen<'a> {
    frame: &'a mut canvas::Frame,
    grid: Grid,
    style: &'a Style,
}

impl Pen<'_> {
    fn ink(&self, ink: Ink) -> Color {
        ink.of(&self.style.palette)
    }

    fn plate(&mut self, plate: &Plate) {
        let path = plate_path(self.grid, plate);
        match (plate.fill, plate.foot) {
            (Some(fill), None) => self.frame.fill(&path, self.ink(fill)),
            (Some(fill), Some(foot)) => {
                // Neomil's unselected cards are translucent over the
                // screen's glow and so grade darker downward; the trace
                // samples both stops down the card's centre.
                let top = self.grid.at(plate.at.x, plate.at.y);
                let bottom = self.grid.at(plate.at.x, plate.at.y + plate.at.h);
                let gradient = canvas::gradient::Linear::new(top, bottom)
                    .add_stop(0.0, self.ink(fill))
                    .add_stop(1.0, self.ink(foot));
                self.frame.fill(&path, gradient);
            }
            (None, _) => {}
        }
        if let Some(stroke) = plate.stroke {
            self.frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(self.ink(stroke))
                    .with_width(self.grid.span(plate.weight)),
            );
        }
    }

    fn box_at(&mut self, at: Plot, ink: Ink) {
        let color = self.ink(ink);
        self.frame.fill_rectangle(
            self.grid.at(at.x, at.y),
            self.grid.size(at.w, at.h),
            color,
        );
    }

    fn rule(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, ink: Ink, weight: f32) {
        let (a, b) = (self.grid.at(x0, y0), self.grid.at(x1, y1));
        let path = canvas::Path::line(a, b);
        self.frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(self.ink(ink))
                .with_width(self.grid.span(weight)),
        );
    }

    fn poly(&mut self, points: &[(f32, f32)], ink: Ink) {
        let g = self.grid;
        let path = canvas::Path::new(|p| {
            for (i, &(x, y)) in points.iter().enumerate() {
                if i == 0 {
                    p.move_to(g.at(x, y));
                } else {
                    p.line_to(g.at(x, y));
                }
            }
            p.close();
        });
        let color = self.ink(ink);
        self.frame.fill(&path, color);
    }

    fn legend(&mut self, legend: &Legend) {
        let g = self.grid;
        let size = g.span(legend.size);
        let color = self.ink(legend.ink);
        let font = iced::Font {
            family: iced::font::Family::Name("Rajdhani"),
            weight: legend.weight,
            ..iced::Font::DEFAULT
        };
        let text = canvas::Text {
            content: legend.text.to_string(),
            position: Point::ORIGIN,
            color,
            size: size.into(),
            line_height: iced::widget::text::LineHeight::Absolute((size * LINE).into()),
            font,
            align_x: if legend.centred {
                iced::advanced::text::Alignment::Center
            } else {
                iced::advanced::text::Alignment::Left
            },
            align_y: iced::alignment::Vertical::Top,
            ..Default::default()
        };
        // Tracking is per glyph, so it is drawn per glyph: the run is
        // split, each character measured through the same shaper, and
        // the advances walked with the trace's spacing added between
        // them. Stretching the whole run instead would land the same
        // ink extent, but it takes the text off the glyph pipeline --
        // a non-uniform transform makes `iced` fall back to filling
        // glyph outlines as meshes -- and the crisper edges that comes
        // with are a visible difference on a screen this empty.
        let tracking = g.span(legend.tracking);
        let anchor = g.at(legend.x, legend.baseline);
        let stretch = legend.stretch;
        self.frame.with_save(|frame| {
            frame.translate(Vector::new(anchor.x, anchor.y));
            if legend.turned {
                frame.rotate(-std::f32::consts::FRAC_PI_2);
            }
            if (stretch - 1.0).abs() > 1e-4 {
                frame.scale_nonuniform(Vector::new(stretch, 1.0));
            }
            frame.translate(Vector::new(0.0, -size * ASCENT));

            if tracking == 0.0 {
                frame.fill_text(text);
                return;
            }

            let advances = advances(legend.text, size, font);
            let total: f32 = advances.iter().sum::<f32>()
                + tracking * (advances.len().max(1) - 1) as f32;
            let mut x = if legend.centred { -total / 2.0 } else { 0.0 };
            for (glyph, advance) in legend.text.chars().zip(advances) {
                frame.fill_text(canvas::Text {
                    content: glyph.to_string(),
                    position: Point::new(x, 0.0),
                    align_x: iced::advanced::text::Alignment::Left,
                    ..text.clone()
                });
                x += advance + tracking;
            }
        });
    }

    fn legends(&mut self, legends: &[Legend]) {
        for legend in legends {
            self.legend(legend);
        }
    }
}

// ------------------------------------------------------------------- art

/// The era's wash, on its own layer. See [`Login::view`].
struct Backdrop<'a> {
    style: Style,
    cache: &'a canvas::Cache,
}

impl<Message> canvas::Program<Message> for Backdrop<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let style = self.style;
        vec![self.cache.draw(renderer, bounds.size(), move |frame| {
            let mut pen = Pen {
                frame,
                grid: Grid::new(bounds.size()),
                style: &style,
            };
            wash(&mut pen, style.access.wash);
        })]
    }
}

struct Art {
    style: Style,
}

impl<Message> canvas::Program<Message> for Art {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let mut pen = Pen {
            frame: &mut frame,
            grid: Grid::new(bounds.size()),
            style: &self.style,
        };
        let access: &Access = &self.style.access;

        masthead(&mut pen, &access.masthead);
        // The fixture goes under the slots: kitsch's bracket runs the
        // full height of the frame and the first guest row sits inside
        // it, and neokitsch's wire band is the floor the entry groups
        // stand on.
        fixture(&mut pen, &access.fixture);
        for slot in access.slots {
            draw_slot(&mut pen, slot);
        }
        colophon(&mut pen, &access.colophon);

        vec![frame.into_geometry()]
    }
}

// ------------------------------------------------------------------ wash

/// A wash is a smooth function of position, and the way to draw one is
/// to sample it.
///
/// The first version of this stepped each wash out as a grid of filled
/// cells, and at the cell size that kept the geometry count sane the
/// steps were plainly visible -- 25x20px blocks across the whole right
/// half of the neokitsch screen, against a design that is smooth. Every
/// alternative inside the canvas vocabulary has the same shape of
/// problem: `iced` has linear gradients and no radial one, nested
/// ellipses band in the radial direction and cannot carry a horizontal
/// fade at all, and a grid fine enough to hide the steps is tens of
/// thousands of rectangles a frame.
///
/// So the wash is rasterised into an RGBA buffer and drawn as an image.
/// `image-without-codecs` is already on for the bar's tray icons, so
/// this adds nothing to the build, and `Login` keeps a `canvas::Cache`
/// so the buffer is built once rather than every frame.
///
/// One pixel of buffer per pixel of screen, not a small buffer scaled
/// up. A scaled one is geometrically smooth but *tonally* banded: at
/// 480x270 the entropism lift came out in 36 distinct colours across
/// the whole page where the design's has 234, because bilinear
/// interpolation between two neighbouring 8-bit samples keeps landing
/// on the same rounded value. That is visible as contour rings, and the
/// extractor saw it too -- the design's backdrop is rich enough to hold
/// a palette cluster of its own and the scaled one was not.
const WASH_MAX_PX: u32 = 1 << 23;

fn wash_image(pen: &mut Pen, sample: impl Fn(f32, f32) -> Color) {
    let (mut w, mut h) = (
        (DESIGN_W * pen.grid.sx).round().max(1.0) as u32,
        (DESIGN_H * pen.grid.sy).round().max(1.0) as u32,
    );
    while w * h > WASH_MAX_PX {
        w = (w / 2).max(1);
        h = (h / 2).max(1);
    }
    let (wash_w, wash_h) = (w, h);

    let mut pixels = Vec::with_capacity((wash_w * wash_h * 4) as usize);
    for row in 0..wash_h {
        let y = (row as f32 + 0.5) / wash_h as f32 * DESIGN_H;
        for col in 0..wash_w {
            let x = (col as f32 + 0.5) / wash_w as f32 * DESIGN_W;
            let c = sample(x, y);
            // Premultiplied, because that is what the image pipeline
            // blends: a straight-alpha buffer darkens every partially
            // transparent pixel towards black on the way in.
            let a = c.a.clamp(0.0, 1.0);
            let enc = |v: f32| (v.clamp(0.0, 1.0) * a * 255.0).round() as u8;
            pixels.extend_from_slice(&[enc(c.r), enc(c.g), enc(c.b), (a * 255.0).round() as u8]);
        }
    }
    let handle = iced::widget::image::Handle::from_rgba(wash_w, wash_h, pixels);
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: DESIGN_W * pen.grid.sx,
        height: DESIGN_H * pen.grid.sy,
    };
    pen.frame.draw_image(
        bounds,
        canvas::Image::new(handle).filter_method(iced::widget::image::FilterMethod::Linear),
    );
}

/// Distance from an ellipse's centre in units of its own radii: 0 at the
/// centre, 1 on the ellipse. The one shape every wash in the four traces
/// is built from, and what an SVG `radialGradient` measures its stops
/// along.
fn radial(x: f32, y: f32, cx: f32, cy: f32, rx: f32, ry: f32) -> f32 {
    (((x - cx) / rx).powi(2) + ((y - cy) / ry).powi(2)).sqrt()
}

fn ramp(stops: &[(f32, f32)], t: f32) -> f32 {
    let mut prev = stops[0];
    for &stop in stops {
        if t <= stop.0 {
            let span = stop.0 - prev.0;
            let k = if span <= 0.0 { 0.0 } else { (t - prev.0) / span };
            return prev.1 + (stop.1 - prev.1) * k;
        }
        prev = stop;
    }
    prev.1
}

fn ramp_color(stops: &[(f32, u32)], t: f32) -> Color {
    let mut prev = stops[0];
    for &stop in stops {
        if t <= stop.0 {
            let span = stop.0 - prev.0;
            let k = if span <= 0.0 { 0.0 } else { (t - prev.0) / span };
            let a = crate::palette::rgb(prev.1);
            let b = crate::palette::rgb(stop.1);
            return Color {
                r: a.r + (b.r - a.r) * k,
                g: a.g + (b.g - a.g) * k,
                b: a.b + (b.b - a.b) * k,
                a: 1.0,
            };
        }
        prev = stop;
    }
    crate::palette::rgb(prev.1)
}

/// The page ground each trace paints before its wash: entropism's lift
/// covers the frame on its own, the other three fill first and wash
/// over the top.
///
/// The wash carries it rather than leaning on `widgets::ground`
/// underneath, because the two disagree -- the era's declared
/// `Ground::Bloom` is a different disc from the one this screen's photo
/// shows, and where the wash went transparent the era's bloom came
/// through it. Kitsch's rose ran to the bottom edge of the frame that
/// way, against a design that is black below y 620.
const GROUND_KITSCH: u32 = 0x0a0907;
const GROUND_NEOMIL: u32 = 0x080405;

/// `over`, as the compositor means it: `top` at its own alpha on `under`.
fn over(top: Color, under: Color) -> Color {
    let a = top.a + under.a * (1.0 - top.a);
    if a <= 0.0 {
        return Color::TRANSPARENT;
    }
    let mix = |t: f32, u: f32| (t * top.a + u * under.a * (1.0 - top.a)) / a;
    Color {
        r: mix(top.r, under.r),
        g: mix(top.g, under.g),
        b: mix(top.b, under.b),
        a,
    }
}

/// Neomil's cold-blue glow: a horizontal ramp under a vertical falloff,
/// straight off `docs/neomil/login-trace.svg`'s `glowh` and `glowv`.
const GLOW_H: [(f32, u32); 10] = [
    (0.000, 0x282824),
    (0.063, 0x273743),
    (0.188, 0x263953),
    (0.313, 0x202b56),
    (0.438, 0x1b2253),
    (0.563, 0x171f51),
    (0.688, 0x121f51),
    (0.813, 0x0d1f4e),
    (0.938, 0x082447),
    (1.000, 0x080b0e),
];
const GLOW_V: [(f32, f32); 9] = [
    (0.00, 1.000),
    (0.25, 1.000),
    (0.30, 0.890),
    (0.35, 0.729),
    (0.40, 0.549),
    (0.45, 0.329),
    (0.50, 0.169),
    (0.55, 0.071),
    (0.60, 0.000),
];
/// The warm near-black vignette down its left margin.
const VIGNETTE: u32 = 0x241012;

/// Entropism's warm lift: a faint radial over the near-black ground,
/// `docs/entropism/login-trace.svg`'s `lift`.
const LIFT: [(f32, u32); 3] = [(0.0, 0x1a1810), (0.7, 0x141107), (1.0, 0x0f0a04)];

/// Kitsch's rose bloom out of the top edge, and the grey-green cast down
/// the left margin.
const BLOOM: [(f32, u32); 5] = [
    (0.00, 0xa84f62),
    (0.35, 0x8e3b52),
    (0.60, 0x5a2236),
    (0.85, 0x1e0f14),
    (1.00, 0x0a0907),
];
const LEFTWASH: u32 = 0x262a24;
const LEFTWASH_A: [(f32, f32); 3] = [(0.0, 1.0), (0.6, 0.5), (1.0, 0.0)];

/// Neokitsch's violet haze, its brighter top-left lobe, and the
/// cold-blue band inside it. Retuned to this photo on the 2026-09-03
/// trace pass: the haze centre moved 825 -> 770 with `r` 1030 -> 1000
/// and the y-scale 0.515 -> 0.49, its stops to 0.35/0.66/0.85, the blue
/// band to 850/1000/0.47, and `hazelobe` is new.
const HAZE: [(f32, u32); 4] = [
    (0.00, 0x574568),
    (0.35, 0x574568),
    (0.66, 0x3a3853),
    (0.85, 0x16121a),
];
const HAZE_BASE: u32 = 0x0e0a0d;
const HAZE_LOBE: u32 = 0x7a5288;
const HAZE_LOBE_A: [(f32, f32); 3] = [(0.00, 0.85), (0.45, 0.55), (1.00, 0.0)];
const HAZE_BLUE: [(f32, u32); 5] = [
    (0.00, 0x223350),
    (0.60, 0x223350),
    (0.68, 0x223350),
    (0.76, 0x1a2c46),
    (0.84, 0x101d30),
];
const HAZE_BLUE_A: [(f32, f32); 5] =
    [(0.00, 0.0), (0.60, 0.0), (0.68, 0.85), (0.76, 0.80), (0.84, 0.0)];
const HAZE_BLUE_FADE: [(f32, f32); 5] = [
    (0.00, 0.000),
    (0.12, 0.102),
    (0.22, 0.478),
    (0.40, 1.000),
    (1.00, 1.000),
];

fn wash(pen: &mut Pen, wash: Wash) {
    match wash {
        Wash::Plain => {}
        Wash::WarmLift => wash_image(pen, |x, y| {
            ramp_color(&LIFT, radial(x, y, 0.45 * DESIGN_W, 0.4 * DESIGN_H, 0.8 * DESIGN_W, 0.8 * DESIGN_H))
        }),
        Wash::ColdGlow => wash_image(pen, |x, y| {
            let glow = Color {
                a: ramp(&GLOW_V, y / DESIGN_H),
                ..ramp_color(&GLOW_H, x / DESIGN_W)
            };
            let vignette = Color {
                a: (1.0
                    - radial(x, y, 0.02 * DESIGN_W, 0.60 * DESIGN_H, 0.34 * DESIGN_W, 0.34 * DESIGN_H))
                .clamp(0.0, 1.0)
                .powi(2),
                ..crate::palette::rgb(VIGNETTE)
            };
            over(vignette, over(glow, crate::palette::rgb(GROUND_NEOMIL)))
        }),
        Wash::RoseBloom => wash_image(pen, |x, y| {
            let bloom = if y <= 620.0 {
                ramp_color(&BLOOM, radial(x, y, 800.0, -155.0, 1520.0, 589.0))
            } else {
                Color::TRANSPARENT
            };
            let wash = Color {
                a: ramp(&LEFTWASH_A, radial(x, y, 0.0, 450.0, 300.0, 390.0)),
                ..crate::palette::rgb(LEFTWASH)
            };
            over(wash, over(bloom, crate::palette::rgb(GROUND_KITSCH)))
        }),
        Wash::VioletHaze => wash_image(pen, |x, y| {
            let haze = {
                let t = radial(x, y, 770.0, -120.0, 1000.0, 490.0);
                if t >= 0.85 {
                    // The trace's last stop runs out to the page ground.
                    ramp_color(
                        &[(0.85, 0x16121a), (1.00, HAZE_BASE)],
                        t.min(1.0),
                    )
                } else {
                    ramp_color(&HAZE, t)
                }
            };
            let lobe = Color {
                a: if y <= 300.0 {
                    ramp(&HAZE_LOBE_A, radial(x, y, 430.0, -40.0, 560.0, 168.0))
                } else {
                    0.0
                },
                ..crate::palette::rgb(HAZE_LOBE)
            };
            let blue = {
                let t = radial(x, y, 850.0, -120.0, 1000.0, 470.0);
                Color {
                    a: if y <= 560.0 {
                        ramp(&HAZE_BLUE_A, t) * ramp(&HAZE_BLUE_FADE, x / DESIGN_W)
                    } else {
                        0.0
                    },
                    ..ramp_color(&HAZE_BLUE, t)
                }
            };
            over(blue, over(lobe, haze))
        }),
    }
}

// -------------------------------------------------------------- masthead

/// The neomil dossier's protocol block: four rows of barcode dashes
/// over the code tape. Constants of one drawing, so they live here and
/// not in the era table -- the same division [`Masthead`] documents.
const PROTOCOL_DASHES: [(f32, f32, f32); 8] = [
    (257.0, 107.0, 18.0),
    (279.0, 107.0, 21.0),
    (257.0, 111.0, 26.0),
    (287.0, 111.0, 13.0),
    (257.0, 115.0, 12.0),
    (273.0, 115.0, 27.0),
    (257.0, 119.0, 22.0),
    (283.0, 119.0, 17.0),
];
const TAPE_TICKS: [(f32, f32); 5] = [
    (261.0, 1.5),
    (264.0, 1.0),
    (267.0, 2.0),
    (271.0, 1.0),
    (274.0, 1.5),
];

fn masthead(pen: &mut Pen, masthead: &Masthead) {
    match masthead {
        Masthead::Strip {
            plate,
            dividers,
            labels,
        } => {
            pen.plate(plate);
            for &x in *dividers {
                pen.rule(
                    x,
                    plate.at.y,
                    x,
                    plate.at.y + plate.at.h,
                    plate.stroke.unwrap_or(Ink::Border),
                    plate.weight,
                );
            }
            pen.legends(labels);
        }
        Masthead::Dossier {
            badges,
            rule,
            labels,
        } => {
            for badge in *badges {
                pen.plate(badge);
            }
            for &(x, y, w) in &PROTOCOL_DASHES {
                pen.box_at(Plot::new(x, y, w, 2.5), Ink::Dim);
            }
            // The code tape under the block: a filled label with its
            // leading corner nicked off, carrying dark ticks and a
            // registration string.
            pen.poly(
                &[
                    (259.0, 151.0),
                    (376.0, 151.0),
                    (376.0, 160.0),
                    (259.0, 160.0),
                    (257.0, 158.0),
                    (257.0, 153.0),
                ],
                Ink::Fg,
            );
            for &(x, w) in &TAPE_TICKS {
                pen.box_at(
                    Plot::new(x, 153.0, w, 5.0),
                    Ink::Fixed(crate::eras::neomil::ON_CARD),
                );
            }
            pen.plate(rule);
            pen.legends(labels);
        }
        Masthead::Clock { labels } => pen.legends(labels),
        Masthead::Logotype {
            cell,
            divider,
            labels,
        } => {
            pen.plate(cell);
            pen.rule(
                *divider,
                cell.at.y,
                *divider,
                cell.at.y + cell.at.h,
                cell.stroke.unwrap_or(Ink::Border),
                cell.weight,
            );
            pen.legends(labels);
        }
    }
}

// ------------------------------------------------------------------ slot

/// The tab every neomil avatar box wears on its top-left corner:
/// 46 wide, 7 tall, with a short diagonal off its trailing end.
const TAB: (f32, f32, f32) = (46.0, 7.0, 6.0);

fn draw_slot(pen: &mut Pen, slot: &Slot) {
    if let Some(body) = &slot.body {
        pen.plate(body);
    }
    if let Some(foot) = &slot.foot {
        pen.plate(foot);
    }
    if let Some(notch) = &slot.notch {
        // A dark bite out of the card's leading edge, chamfered top and
        // bottom, with the era's hairline rail standing in it.
        let Plot { x, y, w, h } = *notch;
        pen.poly(
            &[
                (x, y),
                (x + w, y + 10.0),
                (x + w, y + h - 11.0),
                (x, y + h),
            ],
            Ink::Bg,
        );
        pen.box_at(Plot::new(x, y, 1.5, h), Ink::Dim);
    }
    if let Some(mark) = &slot.mark {
        if matches!(slot.emblem, Emblem::Hexagon | Emblem::Portrait) {
            let (tw, th, cut) = TAB;
            let (x, y) = (mark.at.x, mark.at.y);
            pen.poly(
                &[
                    (x, y - th),
                    (x + tw, y - th),
                    (x + tw + cut, y),
                    (x, y),
                ],
                mark.fill.unwrap_or(Ink::Fg),
            );
        }
        pen.plate(mark);
        emblem(pen, slot.emblem, mark.at);
    }
    if let Some(name) = &slot.name {
        pen.legend(name);
    }
    if let Some(prompt) = &slot.prompt {
        pen.legend(prompt);
    }
    if let Some(field) = &slot.field {
        pen.plate(field);
    }
    if let Some(value) = &slot.value {
        pen.legend(value);
    }
    if let Some(caret) = &slot.caret {
        pen.plate(caret);
    }
    if let Some(action) = &slot.action {
        pen.plate(action);
    }
    if let Some(label) = &slot.action_label {
        pen.legend(label);
    }
    if let Some(badge) = &slot.badge {
        badge_plate(pen, badge);
    }
    if let Some(letter) = &slot.badge_letter {
        pen.legend(letter);
    }
    pen.legends(slot.notes);
}

/// The boxed footnote letter.
///
/// Square in three of the four references (`rect x=380 y=796 width=26
/// height=26` even in kitsch, which rounds its containers; the deleted
/// `widgets::marker` was built to that) -- and neokitsch is the
/// exception: its box is the era's mini-SIM
/// plate, rounded and with one corner folded in. Which one an era draws
/// follows its declared [`crate::style::Corner`] rather than its name.
fn badge_plate(pen: &mut Pen, badge: &Plate) {
    let Plot { x, y, w, h } = badge.at;
    let ink = badge.stroke.unwrap_or(Ink::Border);
    match pen.style.corner {
        crate::style::Corner::ClipTopRight { .. } => {
            let r = 3.0;
            let fold = 7.0;
            let g = pen.grid;
            let path = canvas::Path::new(|p| {
                p.move_to(g.at(x + r, y));
                p.line_to(g.at(x + w - r, y));
                p.quadratic_curve_to(g.at(x + w, y), g.at(x + w, y + r));
                p.line_to(g.at(x + w, y + h - fold));
                p.line_to(g.at(x + w - fold, y + h));
                p.line_to(g.at(x + r, y + h));
                p.quadratic_curve_to(g.at(x, y + h), g.at(x, y + h - r));
                p.line_to(g.at(x, y + r));
                p.quadratic_curve_to(g.at(x, y), g.at(x + r, y));
                p.close();
            });
            let color = pen.ink(ink);
            pen.frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(pen.grid.span(badge.weight)),
            );
            pen.rule(x + w, y + h - fold, x + w - fold, y + h - fold, ink, 1.0);
            pen.rule(x + w - fold, y + h - fold, x + w - fold, y + h, ink, 1.0);
        }
        _ => pen.plate(badge),
    }
}

fn emblem(pen: &mut Pen, emblem: Emblem, at: Plot) {
    let (x, y) = (at.x, at.y);
    match emblem {
        Emblem::None => {}
        Emblem::Hexagon => {
            // The wire hexagon and its satellites, dark on the plate.
            let ink = Ink::Fixed(crate::eras::neomil::GLYPH_INK);
            let hex = [
                (56.0, 17.0),
                (76.0, 28.0),
                (76.0, 52.0),
                (56.0, 63.0),
                (36.0, 52.0),
                (36.0, 28.0),
            ];
            let g = pen.grid;
            let path = canvas::Path::new(|p| {
                for (i, &(dx, dy)) in hex.iter().enumerate() {
                    if i == 0 {
                        p.move_to(g.at(x + dx, y + dy));
                    } else {
                        p.line_to(g.at(x + dx, y + dy));
                    }
                }
                p.close();
            });
            let color = pen.ink(ink);
            pen.frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(pen.grid.span(3.0)),
            );
            pen.rule(x + 40.0, y + 56.0, x + 72.0, y + 24.0, ink, 4.0);
            for (i, wide) in [14.0f32, 12.0, 14.0, 10.0].iter().enumerate() {
                pen.box_at(
                    Plot::new(x + 4.0, y + 9.0 + 4.0 * i as f32, *wide, 1.5),
                    ink,
                );
            }
            pen.box_at(Plot::new(x + 4.0, y + 77.0, 6.0, 7.0), ink);
            pen.box_at(Plot::new(x + 15.0, y + 78.0, 6.0, 6.0), ink);
            pen.box_at(Plot::new(x + 32.0, y + 75.0, 12.0, 2.0), ink);
            pen.box_at(Plot::new(x + 32.0, y + 85.0, 12.0, 2.0), ink);
            pen.box_at(Plot::new(x + 32.0, y + 75.0, 2.0, 12.0), ink);
            pen.box_at(Plot::new(x + 42.0, y + 75.0, 2.0, 12.0), ink);
            pen.rule(x + 32.0, y + 75.0, x + 44.0, y + 87.0, ink, 2.0);
            pen.box_at(Plot::new(x + 47.0, y + 75.0, 3.0, 3.0), ink);
            pen.box_at(Plot::new(x + 47.0, y + 80.0, 3.0, 3.0), ink);
        }
        Emblem::Portrait => {
            // Hair, face and shoulders, filling the lower two thirds of
            // the plate. Traced as three filled runs.
            let ink = Ink::Fixed(crate::eras::neomil::PORTRAIT);
            let g = pen.grid;
            let head = canvas::Path::new(|p| {
                p.ellipse(canvas::path::arc::Elliptical {
                    center: g.at(x + 51.0, y + 38.0),
                    radii: Vector::new(16.0 * g.sx, 21.0 * g.sy),
                    rotation: iced::Radians(0.0),
                    start_angle: iced::Radians(0.0),
                    end_angle: iced::Radians(std::f32::consts::TAU),
                });
            });
            let color = pen.ink(ink);
            pen.frame.fill(&head, color);
            let hair = canvas::Path::new(|p| {
                p.move_to(g.at(x + 31.0, y + 27.0));
                p.quadratic_curve_to(g.at(x + 34.0, y + 5.0), g.at(x + 54.0, y + 5.0));
                p.quadratic_curve_to(g.at(x + 78.0, y + 7.0), g.at(x + 80.0, y + 33.0));
                p.line_to(g.at(x + 78.0, y + 65.0));
                p.line_to(g.at(x + 66.0, y + 53.0));
                p.line_to(g.at(x + 68.0, y + 21.0));
                p.quadratic_curve_to(g.at(x + 56.0, y + 13.0), g.at(x + 42.0, y + 25.0));
                p.line_to(g.at(x + 40.0, y + 53.0));
                p.line_to(g.at(x + 28.0, y + 59.0));
                p.close();
            });
            pen.frame.fill(&hair, color);
            let shoulders = canvas::Path::new(|p| {
                p.move_to(g.at(x + 16.0, y + 94.0));
                p.line_to(g.at(x + 22.0, y + 71.0));
                p.quadratic_curve_to(g.at(x + 51.0, y + 55.0), g.at(x + 80.0, y + 71.0));
                p.line_to(g.at(x + 86.0, y + 94.0));
                p.close();
            });
            pen.frame.fill(&shoulders, color);
        }
        Emblem::Chip => {
            // A printed chip: a dark hexagon split by a lit slash, a
            // hatched wedge in the top-left corner, and a row of marks
            // along the foot.
            let ink = Ink::Fixed(crate::eras::kitsch::CHIP_INK);
            let lit = Ink::Fixed(crate::eras::kitsch::CHIP);
            pen.poly(
                &[
                    (x + 36.8, y + 3.7),
                    (x + 54.0, y + 13.7),
                    (x + 54.0, y + 33.0),
                    (x + 36.8, y + 43.7),
                    (x + 17.4, y + 33.0),
                    (x + 17.4, y + 13.7),
                ],
                ink,
            );
            pen.rule(x + 17.4, y + 33.0, x + 54.0, y + 13.7, lit, 2.0);
            pen.poly(
                &[
                    (x, y + 4.0),
                    (x + 13.0, y + 4.0),
                    (x + 12.5, y + 13.0),
                    (x + 10.0, y + 13.0),
                    (x, y + 44.0),
                ],
                ink,
            );
            for (i, w) in [12.6f32, 12.4, 12.2].iter().enumerate() {
                pen.rule(
                    x,
                    y + 6.5 + 2.5 * i as f32,
                    x + w,
                    y + 6.5 + 2.5 * i as f32,
                    lit,
                    0.9,
                );
            }
            for &(dx, dy, w, h) in &[
                (1.0, 52.0, 3.0, 3.0),
                (1.0, 56.0, 3.0, 3.5),
                (30.0, 52.5, 3.7, 3.2),
                (30.0, 56.3, 3.7, 3.0),
                (34.5, 51.0, 1.0, 1.0),
                (35.0, 53.5, 9.0, 0.8),
                (35.0, 55.5, 7.0, 0.8),
                (35.0, 57.5, 8.0, 0.8),
                (11.6, 53.1, 2.8, 2.8),
            ] {
                pen.box_at(Plot::new(x + dx, y + dy, w, h), ink);
            }
            pen.rule(x + 23.0, y + 52.6, x + 29.0, y + 52.6, ink, 1.0);
            pen.rule(x + 29.0, y + 52.6, x + 29.0, y + 59.0, ink, 1.0);
            pen.rule(x + 29.0, y + 59.0, x + 23.0, y + 59.0, ink, 1.0);
            pen.rule(x + 23.0, y + 59.0, x + 23.0, y + 52.6, ink, 1.0);
            pen.rule(x + 23.0, y + 52.6, x + 29.0, y + 59.0, ink, 1.0);
        }
    }
}

// --------------------------------------------------------------- fixture

/// The bars of kitsch's barcode: a column profile of the photo,
/// thresholded at the bar/gap midpoint. Fifty bars, x 377.5..590.4.
const BARS: [(f32, f32); 50] = [
    (377.5, 7.5), (385.8, 1.7), (389.2, 1.7), (392.1, 1.7), (395.0, 1.7),
    (401.2, 3.3), (405.8, 2.9), (410.4, 1.7), (415.0, 1.7), (419.6, 2.9),
    (425.8, 1.2), (428.8, 1.7), (431.2, 3.3), (436.2, 1.3), (440.4, 1.7),
    (443.8, 2.9), (449.6, 1.7), (452.5, 3.3), (457.1, 2.1), (460.4, 3.3),
    (464.6, 3.3), (469.2, 1.7), (473.8, 1.7), (478.3, 3.3), (484.6, 1.7),
    (487.5, 1.7), (490.4, 3.3), (495.0, 1.7), (499.6, 1.7), (504.2, 3.3),
    (510.0, 1.7), (513.3, 1.7), (516.2, 3.3), (520.8, 1.7), (525.4, 1.7),
    (528.3, 2.9), (534.2, 2.1), (537.5, 2.9), (542.1, 1.7), (545.0, 3.3),
    (549.6, 2.9), (554.2, 1.7), (558.3, 2.1), (563.3, 2.9), (567.5, 2.1),
    (572.1, 1.7), (575.0, 3.3), (581.2, 1.7), (584.2, 3.3), (589.2, 1.2),
];

fn fixture(pen: &mut Pen, fixture: &Fixture) {
    match fixture {
        Fixture::None => {}
        Fixture::Margins { chips, labels } => {
            for (i, chip) in chips.iter().enumerate() {
                let (x, y) = (chip.x, chip.y);
                pen.box_at(Plot::new(x - 21.0, y + 3.0, 8.0, 1.5), Ink::Fg);
                pen.box_at(Plot::new(x - 21.0, y + 7.0, 6.0, 1.5), Ink::Fg);
                pen.box_at(Plot::new(x - 11.0, y + 3.0, 6.0, 6.0), Ink::Fg);
                pen.box_at(Plot::new(x, y, chip.w, chip.h), Ink::Fg);
                // The right margin carries a down-arrow under its chip.
                if i + 1 == chips.len() {
                    pen.box_at(Plot::new(x + 6.0, y + 64.0, 2.0, 18.0), Ink::Fg);
                    pen.poly(
                        &[
                            (x + 2.0, y + 80.0),
                            (x + 12.0, y + 80.0),
                            (x + 7.0, y + 88.0),
                        ],
                        Ink::Fg,
                    );
                    pen.box_at(Plot::new(x + 13.0, y + 66.0, 1.5, 4.0), Ink::Dim);
                    pen.box_at(Plot::new(x + 13.0, y + 73.0, 1.5, 4.0), Ink::Dim);
                }
            }
            pen.legends(labels);
        }
        Fixture::Bracket {
            left,
            right,
            knee,
            foot,
            barcode,
            labels,
        } => {
            let g = pen.grid;
            let (l, r, k, f) = (*left, *right, *knee, *foot);
            // The lobe outside the diagonal, filled.
            let lobe = canvas::Path::new(|p| {
                p.move_to(g.at(l + 0.5, k + 8.0));
                p.line_to(g.at(330.0, 626.0));
                p.quadratic_curve_to(g.at(338.0, 632.0), g.at(338.0, 646.0));
                p.line_to(g.at(338.0, f));
                p.line_to(g.at(262.0, f));
                p.quadratic_curve_to(g.at(l + 0.5, f), g.at(l + 0.5, 698.0));
                p.close();
            });
            let lobe_ink = pen.ink(Ink::Fixed(crate::eras::kitsch::LOBE));
            pen.frame.fill(&lobe, lobe_ink);
            // The outline: full height, breaking into the diagonal at
            // the knee and rounding into its foot.
            let outline = canvas::Path::new(|p| {
                p.move_to(g.at(l, 0.0));
                p.line_to(g.at(l, k - 12.0));
                p.quadratic_curve_to(g.at(l, k + 7.0), g.at(241.0, k + 16.0));
                p.line_to(g.at(324.0, 619.0));
                p.quadratic_curve_to(g.at(338.0, 630.0), g.at(338.0, 650.0));
                p.line_to(g.at(338.0, 700.0));
                p.quadratic_curve_to(g.at(338.0, f), g.at(369.0, f));
                p.line_to(g.at(581.0, f));
                p.quadratic_curve_to(g.at(r, f), g.at(r, 700.0));
                p.line_to(g.at(r, 0.0));
            });
            let edge = pen.ink(Ink::Fixed(crate::eras::kitsch::BARCODE));
            pen.frame.stroke(
                &outline,
                canvas::Stroke::default()
                    .with_color(edge)
                    .with_width(pen.grid.span(1.3)),
            );
            // The barcode standing in the bracket's foot: a teal label
            // strip, fifty bars, and the digits under them.
            pen.box_at(
                Plot::new(barcode.x, barcode.y, 7.0, barcode.h),
                Ink::Fixed(crate::eras::kitsch::BARCODE_TAB),
            );
            for &(x, w) in &BARS {
                pen.box_at(
                    Plot::new(x, barcode.y, w, 51.0),
                    Ink::Fixed(crate::eras::kitsch::BARCODE),
                );
            }
            pen.legends(labels);
        }
        Fixture::WireBand {
            outer,
            inner,
            end,
            strands,
        } => wire_band(pen, *outer, *inner, *end, *strands),
    }
}

/// The wire band: `strands` hairlines running the two outer plateaus,
/// S-bending down onto the low centre one and back up, both ends
/// curling into a vertical.
///
/// Every figure is `docs/neokitsch/login-trace.svg`'s: the outer
/// plateau spaced 3.9, the centre one tightened to 3.03 so the bends
/// fan, the departure walking right 1.9 a strand and the landing 2.2,
/// mirrored about x=808, and the brightness stepping from 0.30 at the
/// top strand to 1.0 at the bottom.
fn wire_band(pen: &mut Pen, outer: f32, inner: f32, end: f32, strands: usize) {
    const X0: f32 = 35.0;
    const X1: f32 = 1564.0;
    const MIRROR: f32 = 1616.0;
    const CURL: f32 = 8.0;
    let n = strands.max(2) as f32 - 1.0;

    let geometry = |i: usize| -> (f32, f32, f32, f32) {
        let t = i as f32;
        (
            outer + 3.9 * t,
            inner + (845.7 - inner) / n * t,
            358.0 + 1.9 * t,
            406.0 + 2.2 * t,
        )
    };

    let strand = |p: &mut canvas::path::Builder, i: usize, g: Grid, closed: bool| {
        let (oy, iy, lx, rx) = geometry(i);
        let bow = 0.55 * (rx - lx);
        if !closed {
            p.move_to(g.at(X0, end));
            p.line_to(g.at(X0, oy + CURL));
            p.quadratic_curve_to(g.at(X0, oy), g.at(X0 + CURL, oy));
        } else {
            p.move_to(g.at(X0, oy));
        }
        p.line_to(g.at(lx, oy));
        p.bezier_curve_to(g.at(lx + bow, oy), g.at(rx - bow, iy), g.at(rx, iy));
        p.line_to(g.at(MIRROR - rx, iy));
        p.bezier_curve_to(
            g.at(MIRROR - rx + bow, iy),
            g.at(MIRROR - lx - bow, oy),
            g.at(MIRROR - lx, oy),
        );
        if !closed {
            p.line_to(g.at(X1 - CURL, oy));
            p.quadratic_curve_to(g.at(X1, oy), g.at(X1, oy + CURL));
            p.line_to(g.at(X1, end));
        } else {
            p.line_to(g.at(X1, oy));
        }
    };

    // The floor between the strands glows, black at the top strand and
    // rising to the warm brown at the bottom one, with no spill outside
    // the band.
    let g = pen.grid;
    let last = strands.saturating_sub(1);
    let glow = canvas::Path::new(|p| {
        strand(p, 0, g, true);
        let (oy_last, iy_last, lx_last, rx_last) = geometry(last);
        let bow = 0.55 * (rx_last - lx_last);
        p.line_to(g.at(X1, oy_last));
        p.line_to(g.at(MIRROR - lx_last, oy_last));
        p.bezier_curve_to(
            g.at(MIRROR - lx_last - bow, oy_last),
            g.at(MIRROR - rx_last + bow, iy_last),
            g.at(MIRROR - rx_last, iy_last),
        );
        p.line_to(g.at(rx_last, iy_last));
        p.bezier_curve_to(
            g.at(rx_last - bow, iy_last),
            g.at(lx_last + bow, oy_last),
            g.at(lx_last, oy_last),
        );
        p.line_to(g.at(X0, oy_last));
        p.close();
    });
    let tone = pen.ink(Ink::Fixed(crate::eras::neokitsch::WIRE_GLOW));
    let (top, bottom) = (pen.grid.at(0.0, outer), pen.grid.at(0.0, 845.7));
    let gradient = canvas::gradient::Linear::new(top, bottom)
        .add_stop(0.0, Color { a: 0.0, ..tone })
        .add_stop(0.45, Color { a: 0.55, ..tone })
        .add_stop(1.0, Color { a: 1.0, ..tone });
    pen.frame.fill(&glow, gradient);

    let wire = pen.ink(Ink::Fixed(crate::eras::neokitsch::WIRE));
    // Each wire sits in a soft vertical smear of its own light, and it
    // is not the trace's `halo` -- that is tagged `class="photo"` and
    // G2i hides it. This is the band's own interior glow, which the
    // trace measures directly ("red 13 at y725, 56 at y805, 14 at
    // y820" at x=200) and which the `bandglow` fill alone does not
    // reach: with only the gradient the floor comes out at red ~32
    // where the design renders ~56, and the shape gate reads 14% of
    // the design's area against 90% with these passes.
    //
    // Drawn as two wide, low-alpha passes under the crisp stroke --
    // the same "close enough at UI scale" call `widgets::ground` makes
    // for its bloom -- in the wire's own hue taken down to the dim
    // brown of the band's dark family.
    let bloom = Color {
        r: wire.r * 0.43,
        g: wire.g * 0.32,
        b: wire.b * 0.21,
        a: 1.0,
    };
    for (width, weight) in [(9.0f32, 0.05f32), (4.5, 0.09)] {
        for i in 0..strands {
            let path = canvas::Path::new(|p| strand(p, i, g, false));
            let alpha = (0.30 + (1.0 - 0.30) * i as f32 / n) * weight;
            pen.frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(Color { a: alpha, ..bloom })
                    .with_width(pen.grid.span(width)),
            );
        }
    }
    for i in 0..strands {
        let path = canvas::Path::new(|p| strand(p, i, g, false));
        let alpha = 0.30 + (1.0 - 0.30) * i as f32 / n;
        pen.frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(Color { a: alpha, ..wire })
                .with_width(pen.grid.span(1.2)),
        );
    }
}

// -------------------------------------------------------------- colophon

fn colophon(pen: &mut Pen, colophon: &Colophon) {
    match colophon {
        Colophon::None => {}
        Colophon::Band { plate, labels } => {
            pen.plate(plate);
            pen.legends(labels);
        }
        Colophon::Notice { labels } => pen.legends(labels),
    }
}

#[cfg(test)]
mod tests {
    //! The era tables' login blocks are transcriptions, and the failure
    //! mode of a transcription is a typo -- a coordinate off by a
    //! decimal point, a slot that lost its control in an edit. Neither
    //! shows up as a compile error and both show up as a screen with a
    //! hole in it, so they are checked here rather than found by eye.

    use super::*;
    use crate::style::{Era, Fixture, Masthead};

    fn plots(access: &Access) -> Vec<(&'static str, Plot)> {
        let mut out = Vec::new();
        match &access.masthead {
            Masthead::Strip { plate, .. } => out.push(("masthead", plate.at)),
            Masthead::Dossier { badges, rule, .. } => {
                for badge in *badges {
                    out.push(("badge", badge.at));
                }
                out.push(("rule", rule.at));
            }
            Masthead::Clock { .. } => {}
            Masthead::Logotype { cell, .. } => out.push(("header cell", cell.at)),
        }
        for slot in access.slots {
            for (name, plate) in [
                ("body", &slot.body),
                ("foot", &slot.foot),
                ("mark", &slot.mark),
                ("field", &slot.field),
                ("caret", &slot.caret),
                ("action", &slot.action),
                ("badge", &slot.badge),
            ] {
                if let Some(plate) = plate {
                    out.push((name, plate.at));
                }
            }
            if let Some(notch) = slot.notch {
                out.push(("notch", notch));
            }
        }
        if let Fixture::Bracket { barcode, .. } = &access.fixture {
            out.push(("barcode", *barcode));
        }
        if let Fixture::Margins { chips, .. } = &access.fixture {
            for chip in chips.iter() {
                out.push(("chip", *chip));
            }
        }
        if let Colophon::Band { plate, .. } = &access.colophon {
            out.push(("footer band", plate.at));
        }
        out
    }

    /// Nothing an era's table places is outside the frame the traces
    /// measure it in, and nothing has collapsed to zero.
    #[test]
    fn every_era_places_its_access_screen_inside_the_frame() {
        for era in Era::ALL {
            let style = era.style();
            for (what, plot) in plots(&style.access) {
                assert!(
                    plot.w > 0.0 && plot.h > 0.0,
                    "{}: {what} is empty: {plot:?}",
                    era.name()
                );
                assert!(
                    plot.x >= 0.0
                        && plot.y >= 0.0
                        && plot.x + plot.w <= DESIGN_W
                        && plot.y + plot.h <= DESIGN_H,
                    "{}: {what} leaves the frame: {plot:?}",
                    era.name()
                );
            }
        }
    }

    /// Every era offers at least one account, and every account it
    /// offers can be told from the ground: a slot with neither a
    /// control nor a mark is a slot that draws nothing.
    #[test]
    fn every_slot_carries_something() {
        for era in Era::ALL {
            let style = era.style();
            assert!(
                !style.access.slots.is_empty(),
                "{}: no access slots at all",
                era.name()
            );
            for (i, slot) in style.access.slots.iter().enumerate() {
                assert!(
                    slot.action.is_some() || slot.mark.is_some() || slot.field.is_some(),
                    "{}: slot {i} draws nothing",
                    era.name()
                );
            }
        }
    }

    /// Exactly one slot per era is the live one -- the one with a field
    /// to type into. It is what separates an access screen from a list
    /// of names, and three of the four traces put two locked accounts
    /// beside it. Neokitsch is the exception the traces record: it
    /// offers A *and* B, both live.
    #[test]
    fn the_live_slots_are_the_ones_with_a_field() {
        let live = |era: Era| {
            era.style()
                .access
                .slots
                .iter()
                .filter(|s| s.field.is_some())
                .count()
        };
        assert_eq!(live(Era::Entropism), 1);
        assert_eq!(live(Era::Neomil), 1);
        assert_eq!(live(Era::Kitsch), 1);
        assert_eq!(live(Era::Neokitsch), 2);
    }
}
