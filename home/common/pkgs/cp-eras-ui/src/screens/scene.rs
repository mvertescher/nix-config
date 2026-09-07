//! The scene renderer: one `canvas::Program` that walks a
//! [`Prim`] list and paints it.
//!
//! An era's trace-driven screens -- the store and the dashboard -- are
//! data on the era table: `&'static [Prim]` transcribed from
//! `docs/<era>/<screen>-trace.svg` at the trace's own 1600x900. This
//! module is the one interpreter for that data. It was the store's
//! private engine until the dashboard folded onto the same model
//! (2026-09-03), at which point it moved here unchanged so both screens
//! walk one copy: the store's render and hit boxes are byte-identical
//! before and after the move, and that is the standing test for any
//! edit here.
//!
//! Nothing in this file names an era or a screen. A screen owns its
//! selection state and hands the scene a [`Picked`] snapshot; the scene
//! paints a [`Prim::Plate`]'s `on` drawing when the plate's index is
//! the pick for its [`Group`] and `off` otherwise, and reports clicks
//! on plates back through the screen's own message constructor.

use crate::motion;
use crate::screens::soft;
use crate::style::{Anchor, Change, Face, Group, Ink, Prim, Seg, Style};
use crate::Element;
use std::cell::RefCell;
use std::time::Duration;
use iced::widget::canvas;
use iced::mouse::Interaction;
use iced::{mouse, Color, Point, Rectangle, Renderer, Size};

/// The frame every trace is measured in. The scene is painted at this
/// size and scaled to the canvas, so the screen holds its proportions
/// in a window that is not exactly 1600x900 -- the golden matrix and
/// G2i both use exactly that, and nothing else should have to.
pub const FRAME: (f32, f32) = (1600.0, 900.0);

/// Where a glyph's baseline sits inside the line box iced lays out when
/// the text is top-aligned: the leading above the ascender plus the
/// ascender itself, as a fraction of the font size. The traces give
/// every text run its SVG baseline, so the scene has to convert.
const BASELINE: f32 = 0.84;

/// [`BASELINE`] for Orbitron, whose ascender stands taller (hhea
/// 1011 / -243 against Rajdhani's 930 / -346): the canvas centres
/// the glyph box in its 1.2 line, so the baseline sits at
/// `0.6 + (ascent - descent) / 2`. Measured as well as computed: at
/// `BASELINE` the neomil `next` sat 6px under the trace's, at this it
/// lands on it.
const ORBITRON_BASELINE: f32 = 0.984;

/// The baseline fraction of a face.
fn baseline(face: Face) -> f32 {
    match face {
        Face::OrbitronBold => ORBITRON_BASELINE,
        _ => BASELINE,
    }
}

/// The current selection, one index per [`Group`]. A screen fills in
/// the groups its scene uses and leaves the rest at zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Picked {
    pub category: usize,
    pub card: usize,
    pub module: usize,
}

impl Picked {
    /// The current selection for a group.
    pub fn get(&self, group: Group) -> usize {
        match group {
            Group::Category => self.category,
            Group::Card => self.card,
            Group::Module => self.module,
        }
    }
}

/// A scene, ready to paint: the era (for its palette), the display
/// list, the selection, and how a click on a plate becomes the owning
/// screen's message.
#[derive(Debug, Clone, Copy)]
pub struct Scene<M> {
    pub style: Style,
    pub prims: &'static [Prim],
    pub picked: Picked,
    /// The moment to paint, counted from `motion::origin()`: what every
    /// [`Prim::Motion`] in the list is read at. A screen with nothing
    /// moving still says where its clock is; `motion::REST` is the
    /// trace as drawn.
    pub at: Duration,
    pub on_select: fn(Group, usize) -> M,
}

impl<M: 'static> Scene<M> {
    /// The scene as a widget: its [`Prim::Soft`] groups in a canvas of
    /// their own under the canvas that paints everything else.
    ///
    /// Two canvases because of how iced batches a canvas layer: every
    /// mesh in it, then every image, then every text run. A composited
    /// group is an image, so drawn in the same canvas as the fills it
    /// is meant to sit under it would cover them -- measured, it did,
    /// leaving only the text of the kitsch dashboard showing. Each
    /// child of a `stack` after the first gets a layer of its own, so
    /// the split puts the image where the era table says it goes.
    pub fn view(self) -> Element<'static, M> {
        iced::widget::stack![
            canvas(Backdrop { style: self.style, prims: self.prims, stretch: false, at: self.at })
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
            canvas(self).width(iced::Length::Fill).height(iced::Length::Fill),
        ]
        .into()
    }
}

/// The [`Prim::Soft`] groups of a scene, as their own canvas: see
/// [`Scene::view`] for why they cannot share the scene's. It paints
/// only the groups that *lead* the scene's list -- a composited
/// backdrop is what `Soft` is for, and `soft_groups_lead_their_scene`
/// keeps every era table to that. A leading group may sit inside a
/// [`Prim::Motion`] with a [`Change::Clip`]: it is then composited
/// over the groups before it and cut to its own coverage
/// (`soft::composite_over`), so the image can be clipped as the
/// motion says without wgpu blending its translucency in linear
/// light. That is how kitsch's ghost fans extrude.
#[derive(Debug, Clone, Copy)]
pub struct Backdrop {
    pub style: Style,
    pub prims: &'static [Prim],
    /// Whether the composite is drawn over the whole canvas rather
    /// than letterboxed at the frame's aspect. A [`Scene`] paints at
    /// one uniform scale and its backdrop matches; the mailbox and the
    /// login map the frame onto the window axis by axis, and a
    /// letterboxed haze under a stretched sheet would put the design's
    /// features beside the art they belong under. The image is still
    /// composited at the frame's aspect and stretched on the way out --
    /// a gradient filtered linearly loses nothing to that.
    pub stretch: bool,
    /// The moment to paint, as [`Scene::at`]: what a motion over a
    /// leading group is read at.
    pub at: Duration,
}

/// Whether `prim` is one [`Backdrop`] paints: a [`Prim::Soft`] group,
/// or a [`Prim::Motion`] holding nothing but those.
fn is_soft(prim: &Prim) -> bool {
    match prim {
        Prim::Soft { .. } => true,
        Prim::Motion { prims, .. } => !prims.is_empty() && prims.iter().all(is_soft),
        _ => false,
    }
}

/// The leading run of [`Prim::Soft`] groups in `prims`, moving ones
/// included.
fn leading_soft(prims: &'static [Prim]) -> &'static [Prim] {
    let n = prims.iter().take_while(|p| is_soft(p)).count();
    &prims[..n]
}

impl Backdrop {
    /// Paint one leading group, or a motion of them, `under` the
    /// groups already painted. `region` is the clip the enclosing
    /// motions have come to, in canvas coordinates, `None` when there
    /// is none; an empty one paints nothing.
    fn paint(
        &self,
        frame: &mut canvas::Frame,
        soft: &SoftCache,
        prim: &'static Prim,
        under: &mut Vec<&'static [Prim]>,
        size: (u32, u32),
        k: f32,
        region: Option<Rectangle>,
    ) {
        match *prim {
            Prim::Soft { prims } => {
                let handle = if region.is_some() {
                    soft.cut(under, prims, &self.style.palette, size, k)
                } else {
                    soft.image(prims, &self.style.palette, size, k)
                };
                under.push(prims);
                let (width, height) = if self.stretch {
                    (frame.width(), frame.height())
                } else {
                    (size.0 as f32, size.1 as f32)
                };
                let image = canvas::Image::new(handle)
                    .filter_method(iced::widget::image::FilterMethod::Linear);
                let bounds = Rectangle { x: 0.0, y: 0.0, width, height };
                match region {
                    Some(region) => frame.with_clip(region, |f| f.draw_image(bounds, image)),
                    None => frame.draw_image(bounds, image),
                }
            }
            Prim::Motion { motion, prims } => {
                let t = motion::progress(&motion, self.at);
                let region = match motion.change {
                    Change::Clip { x, y, w, h } => {
                        // The same rectangle `Scene::paint` clips to; a
                        // stretched backdrop maps it axis by axis as
                        // the sheet over it does.
                        let (sx, sy) = if self.stretch {
                            (frame.width() / FRAME.0, frame.height() / FRAME.1)
                        } else {
                            (k, k)
                        };
                        let own = Rectangle {
                            x: x * sx,
                            y: y * sy,
                            width: Change::lerp(w, t) * sx,
                            height: Change::lerp(h, t) * sy,
                        };
                        match region {
                            Some(outer) => outer.intersection(&own).unwrap_or(Rectangle::new(Point::ORIGIN, Size::ZERO)),
                            None => own,
                        }
                    }
                    // A fade of a composited group would want the
                    // image drawn at an opacity, which lands in linear
                    // light; `soft_motions_only_clip` keeps the tables
                    // off it until something needs it.
                    Change::Opacity { .. } => region.unwrap_or(Rectangle::new(
                        Point::ORIGIN,
                        Size::new(frame.width(), frame.height()),
                    )),
                };
                if region.width <= 0.0 || region.height <= 0.0 {
                    // Nothing shows, but the groups are still under
                    // whatever follows.
                    for p in prims {
                        push_soft(p, under);
                    }
                    return;
                }
                for p in prims {
                    self.paint(frame, soft, p, under, size, k, Some(region));
                }
            }
            _ => {}
        }
    }
}

/// Record the groups of a leading prim as painted, without painting.
fn push_soft(prim: &'static Prim, under: &mut Vec<&'static [Prim]>) {
    match *prim {
        Prim::Soft { prims } => under.push(prims),
        Prim::Motion { prims, .. } => prims.iter().for_each(|p| push_soft(p, under)),
        _ => {}
    }
}

impl<M> canvas::Program<M, Style> for Backdrop {
    type State = SoftCache;

    fn draw(
        &self,
        soft: &Self::State,
        renderer: &Renderer,
        _theme: &Style,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let k = scale(bounds);
        if k > 0.0 {
            // One pixel of image per pixel of canvas, so at the golden
            // matrix's 1600x900 the composite lands byte-for-byte;
            // scaled, it is filtered like any other image.
            let size = ((FRAME.0 * k).round().max(1.0) as u32, (FRAME.1 * k).round().max(1.0) as u32);
            let mut under = Vec::new();
            for prim in leading_soft(self.prims) {
                self.paint(&mut frame, soft, prim, &mut under, size, k, None);
            }
        }
        vec![frame.into_geometry()]
    }
}

/// The rasterised [`Prim::Soft`] groups of one [`Backdrop`] canvas, the
/// widget's `Program::State`. A group is rebuilt only when the canvas
/// size or the palette changes (a published theme re-dresses the screen
/// through the palette), so the software composite is paid once, not
/// per frame -- clicks and hovers redraw the scene without it.
#[derive(Debug, Default)]
pub struct SoftCache(RefCell<Vec<SoftEntry>>);

#[derive(Debug)]
struct SoftEntry {
    prims: usize,
    len: usize,
    size: (u32, u32),
    palette: crate::palette::Palette,
    handle: iced::widget::image::Handle,
}

impl SoftCache {
    /// The image for `prims` at `size` under `palette`, composited on a
    /// miss. Entries for another size or palette are dropped on the
    /// way: a resize or a theme change invalidates every group at once.
    pub(crate) fn image(
        &self,
        prims: &'static [Prim],
        palette: &crate::palette::Palette,
        size: (u32, u32),
        k: f32,
    ) -> iced::widget::image::Handle {
        if let Some(handle) = self.find(prims, palette, size) {
            return handle;
        }
        let pixels = soft::composite(prims, palette, size.0, size.1, k);
        self.keep(prims, palette, size, pixels)
    }

    /// The image for a *moving* `prims`: the composite of `under` and
    /// `prims` cut to what `prims` touches (`soft::composite_over`).
    /// Keyed like [`image`](Self::image) on `prims` alone -- what is
    /// under a group is fixed by its table.
    pub(crate) fn cut(
        &self,
        under: &[&'static [Prim]],
        prims: &'static [Prim],
        palette: &crate::palette::Palette,
        size: (u32, u32),
        k: f32,
    ) -> iced::widget::image::Handle {
        if let Some(handle) = self.find(prims, palette, size) {
            return handle;
        }
        let pixels = soft::composite_over(under, prims, palette, size.0, size.1, k);
        self.keep(prims, palette, size, pixels)
    }

    fn find(
        &self,
        prims: &'static [Prim],
        palette: &crate::palette::Palette,
        size: (u32, u32),
    ) -> Option<iced::widget::image::Handle> {
        let mut cache = self.0.borrow_mut();
        cache.retain(|e| e.size == size && e.palette == *palette);
        cache
            .iter()
            .find(|e| e.prims == prims.as_ptr() as usize && e.len == prims.len())
            .map(|e| e.handle.clone())
    }

    fn keep(
        &self,
        prims: &'static [Prim],
        palette: &crate::palette::Palette,
        size: (u32, u32),
        pixels: Vec<u8>,
    ) -> iced::widget::image::Handle {
        let handle = iced::widget::image::Handle::from_rgba(size.0, size.1, pixels);
        self.0.borrow_mut().push(SoftEntry {
            prims: prims.as_ptr() as usize,
            len: prims.len(),
            size,
            palette: *palette,
            handle: handle.clone(),
        });
        handle
    }
}

/// The frame-to-canvas scale, and the same one `draw` paints with.
pub fn scale(bounds: Rectangle) -> f32 {
    (bounds.width / FRAME.0).min(bounds.height / FRAME.1)
}

/// Which plate, if any, sits under a point in canvas coordinates, with
/// `k` the frame-to-canvas scale.
///
/// Walks the scene the way `paint` does so the hit boxes cannot drift
/// from the drawing: both come from the same table, through the same
/// `Prim::At` translations, at the same scale.
pub fn hit(prims: &[Prim], k: f32, at: Point) -> Option<(Group, usize)> {
    hit_at(prims, 0.0, 0.0, k, at)
}

fn hit_at(prims: &[Prim], ox: f32, oy: f32, k: f32, at: Point) -> Option<(Group, usize)> {
    for prim in prims {
        match *prim {
            Prim::Plate { group, index, x, y, w, h, .. } => {
                let box_ = Rectangle {
                    x: (ox + x) * k,
                    y: (oy + y) * k,
                    width: w * k,
                    height: h * k,
                };
                if box_.contains(at) {
                    return Some((group, index));
                }
            }
            Prim::At { x, y, prims } => {
                if let Some(f) = hit_at(prims, ox + x, oy + y, k, at) {
                    return Some(f);
                }
            }
            // A plate under a wipe is hit whether or not the wipe has
            // uncovered it: the scene has no plates that move yet, and
            // a hit box that fades in with its drawing is a decision for
            // the first one that does.
            Prim::Motion { prims, .. } => {
                if let Some(f) = hit_at(prims, ox, oy, k, at) {
                    return Some(f);
                }
            }
            Prim::Turn { x, y, angle, prims } => {
                // Carry the point into the turned frame: subtract the
                // pivot, undo the rotation, and the sub-scene's own
                // coordinates apply with the pivot at the origin.
                let (px, py) = ((ox + x) * k, (oy + y) * k);
                let (lx, ly) = turned(at.x - px, at.y - py, -angle);
                if let Some(f) = hit_at(prims, 0.0, 0.0, k, Point::new(lx, ly)) {
                    return Some(f);
                }
            }
            _ => {}
        }
    }
    None
}

/// `(x, y)` rotated `angle` degrees about the origin, SVG's sense:
/// positive is clockwise on a y-down screen (`x' = x cos - y sin`,
/// `y' = x sin + y cos`).
fn turned(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let (sin, cos) = angle.to_radians().sin_cos();
    (x * cos - y * sin, x * sin + y * cos)
}

/// Every plate in a scene as `(group, index, centre)`, the centre in
/// frame coordinates. The keyboard walks these centres
/// (`screens::nav`), and the screens' tests use them to prove a table
/// wrapped its choosers in plates and that a click at each centre lands.
pub(crate) fn plates(prims: &[Prim], ox: f32, oy: f32, out: &mut Vec<(Group, usize, Point)>) {
    for prim in prims {
        match *prim {
            Prim::Plate { group, index, x, y, w, h, .. } => out.push((
                group,
                index,
                Point::new(ox + x + w / 2.0, oy + y + h / 2.0),
            )),
            Prim::At { x, y, prims } => plates(prims, ox + x, oy + y, out),
            Prim::Motion { prims, .. } => plates(prims, ox, oy, out),
            Prim::Turn { x, y, angle, prims } => {
                // Collect the sub-scene's centres about its own origin,
                // then turn them out about the pivot.
                let mut inner = Vec::new();
                plates(prims, 0.0, 0.0, &mut inner);
                out.extend(inner.into_iter().map(|(g, i, c)| {
                    let (tx, ty) = turned(c.x, c.y, angle);
                    (g, i, Point::new(ox + x + tx, oy + y + ty))
                }));
            }
            _ => {}
        }
    }
}

impl<M> Scene<M> {
    /// Resolve one of the scene's inks against the live palette, so a
    /// published theme still re-dresses the screen.
    fn ink(&self, ink: Ink, alpha: f32) -> Color {
        let c = ink.of(&self.style.palette);
        self.blend(Color { a: c.a * alpha, ..c })
    }

    /// Rebase a translucent colour for wgpu's linear-light compositing.
    ///
    /// The traces are sRGB documents: rsvg composites `fill-opacity` as
    /// `r = a*c + (1-a)*b` on the *encoded* channel values, and every
    /// alpha in an era table was read off one. iced/wgpu blends in
    /// linear light onto an sRGB surface, so the same alpha lands far
    /// brighter over a dark ground -- kitsch's faintest ghost, `#0f9f80`
    /// at .12, painted G 61 where the trace has 33. This keeps the ink
    /// and rescales the alpha so a linear blend over the era's ground
    /// `b` reproduces the trace's pixel: per channel
    /// `a' = (lin(r) - lin(b)) / (lin(c) - lin(b))`, collapsed to one
    /// alpha with the luminance weights (the channels agree to within a
    /// level over a near-black ground).
    ///
    /// `b` is `palette.bg`, exact over flat ground and an approximation
    /// wherever the prim sits on a haze or on another translucent prim
    /// (measured: a ghost over the kitsch bloom lands within 8 levels,
    /// a five-deep stack 23 levels dark where it was 9 bright). That is
    /// as far as one alpha can go, and it is why translucent *stacks*
    /// are not drawn this way at all: an era table wraps them in a
    /// [`Prim::Soft`] group and `soft.rs` composites them in sRGB. The
    /// test module below pins the numbers; TODO.md § Design pipeline
    /// records the alternatives and why they lost.
    fn blend(&self, c: Color) -> Color {
        blend_over(c, self.style.palette.bg)
    }

    fn font(face: Face) -> iced::Font {
        match face {
            Face::Regular => crate::fonts::FONT_RAJDHANI_REGULAR,
            Face::Medium => crate::fonts::FONT_RAJDHANI_MEDIUM,
            Face::SemiBold => crate::fonts::FONT_RAJDHANI_SEMIBOLD,
            Face::Bold => crate::fonts::FONT_RAJDHANI_BOLD,
            Face::OrbitronBold => crate::fonts::FONT_ORBITRON_BOLD,
        }
    }

    fn align(anchor: Anchor) -> iced::advanced::text::Alignment {
        match anchor {
            Anchor::Start => iced::advanced::text::Alignment::Left,
            Anchor::Middle => iced::advanced::text::Alignment::Center,
            Anchor::End => iced::advanced::text::Alignment::Right,
        }
    }

    /// Paint one primitive. `(ox, oy)` is the running translation
    /// [`Prim::At`] accumulates, `k` the frame-to-canvas scale, and
    /// `alpha` the fade the enclosing [`Change::Opacity`] groups have
    /// come to, 1 at the top.
    fn paint(
        &self,
        frame: &mut canvas::Frame,
        prims: &[Prim],
        ox: f32,
        oy: f32,
        k: f32,
        alpha: f32,
    ) {
        for prim in prims {
            match *prim {
                Prim::Rect { x, y, w, h, fill, stroke, width } => {
                    let path = canvas::Path::rectangle(
                        Point::new((ox + x) * k, (oy + y) * k),
                        Size::new(w * k, h * k),
                    );
                    self.paint_path(frame, &path, fill, stroke, width, k, alpha);
                }
                Prim::Path { x, y, segs, close, fill, stroke, width } => {
                    let path = canvas::Path::new(|b| {
                        b.move_to(Point::new((ox + x) * k, (oy + y) * k));
                        for seg in segs {
                            match *seg {
                                Seg::Move(mx, my) => {
                                    if close {
                                        b.close();
                                    }
                                    b.move_to(Point::new((ox + mx) * k, (oy + my) * k));
                                }
                                Seg::Line(lx, ly) => {
                                    b.line_to(Point::new((ox + lx) * k, (oy + ly) * k))
                                }
                                Seg::Quad { cx, cy, x: qx, y: qy } => b.quadratic_curve_to(
                                    Point::new((ox + cx) * k, (oy + cy) * k),
                                    Point::new((ox + qx) * k, (oy + qy) * k),
                                ),
                                Seg::Cubic { c1x, c1y, c2x, c2y, x: bx, y: by } => b
                                    .bezier_curve_to(
                                        Point::new((ox + c1x) * k, (oy + c1y) * k),
                                        Point::new((ox + c2x) * k, (oy + c2y) * k),
                                        Point::new((ox + bx) * k, (oy + by) * k),
                                    ),
                            }
                        }
                        if close {
                            b.close();
                        }
                    });
                    self.paint_path(frame, &path, fill, stroke, width, k, alpha);
                }
                Prim::Text { x, y, size, ink, face, anchor, content } => {
                    self.paint_text(frame, content, (ox + x) * k, (oy + y) * k, size * k, ink, face, anchor, alpha);
                }
                Prim::Outlined { x, y, size, ink, face, anchor, width, content } => {
                    // The canvas fills text and cannot stroke it, but
                    // the outlines it falls back to under a stretch or
                    // a turn are reachable directly: `draw_with` lays
                    // the run out exactly as `fill_text` would and
                    // hands over one path per glyph.
                    let text =
                        Self::text(content, (ox + x) * k, (oy + y) * k, size * k, Color::TRANSPARENT, face, anchor);
                    let stroke = canvas::Stroke::default()
                        .with_color(self.ink(ink, alpha))
                        .with_width(width * k);
                    text.draw_with(|path, _| frame.stroke(&path, stroke));
                }
                Prim::Wide { x, y, size, stretch, ink, face, anchor, content } => {
                    // A non-uniform transform makes iced convert the run
                    // to filled glyph outlines, which is exactly what
                    // `lengthAdjust="spacingAndGlyphs"` asks for. The
                    // anchor is taken on the stretched run, as SVG
                    // applies `text-anchor` inside the `scale(sx,1)`.
                    let color = self.ink(ink, alpha);
                    let font = Self::font(face);
                    let size = size * k;
                    let run = run_width(content, size, font) * stretch;
                    let px = anchored((ox + x) * k, anchor, run);
                    let py = (oy + y) * k - size * baseline(face);
                    frame.with_save(|f| {
                        f.translate(iced::Vector::new(px, py));
                        f.scale_nonuniform(iced::Vector::new(stretch, 1.0));
                        f.fill_text(canvas::Text {
                            content: content.to_string(),
                            position: Point::ORIGIN,
                            color,
                            size: size.into(),
                            font,
                            align_x: iced::advanced::text::Alignment::Left,
                            align_y: iced::alignment::Vertical::Top,
                            ..Default::default()
                        });
                    });
                }
                Prim::Tracked { x, y, size, ink, face, anchor, tracking, content } => {
                    let (size, tracking) = (size * k, tracking * k);
                    let advances = advances(content, size, Self::font(face));
                    let total: f32 = advances.iter().sum::<f32>()
                        + tracking * advances.len().saturating_sub(1) as f32;
                    let mut gx = anchored((ox + x) * k, anchor, total);
                    for (glyph, advance) in content.chars().zip(advances) {
                        let mut buf = [0u8; 4];
                        self.paint_text(
                            frame,
                            glyph.encode_utf8(&mut buf),
                            gx,
                            (oy + y) * k,
                            size,
                            ink,
                            face,
                            Anchor::Start,
                            alpha,
                        );
                        gx += advance + tracking;
                    }
                }
                Prim::Spaced { x, y, size, ink, face, pitch, content } => {
                    for (i, ch) in content.chars().enumerate() {
                        let mut buf = [0u8; 4];
                        let glyph: &str = ch.encode_utf8(&mut buf);
                        self.paint_text(
                            frame,
                            glyph,
                            (ox + x + i as f32 * pitch) * k,
                            (oy + y) * k,
                            size * k,
                            ink,
                            face,
                            Anchor::Start,
                            alpha,
                        );
                    }
                }
                Prim::Grain { x, y, w, h, pitch, width, ink } => {
                    let color = self.ink(ink, alpha);
                    let mut gy = y + pitch;
                    while gy < y + h {
                        frame.fill_rectangle(
                            Point::new((ox + x) * k, (oy + gy) * k),
                            Size::new(w * k, width * k),
                            color,
                        );
                        gy += pitch;
                    }
                }
                Prim::Dots { x, y, cell, pitch, ink, rows } => {
                    let color = self.ink(ink, alpha);
                    for (r, row) in rows.iter().enumerate() {
                        for (c, mark) in row.chars().enumerate() {
                            if mark == '.' || mark == ' ' {
                                continue;
                            }
                            frame.fill_rectangle(
                                Point::new(
                                    (ox + x + c as f32 * pitch) * k,
                                    (oy + y + r as f32 * pitch) * k,
                                ),
                                Size::new(cell * k, cell * k),
                                color,
                            );
                        }
                    }
                }
                Prim::Round { x, y, w, h, r, fill, stroke, width } => {
                    let path = canvas::Path::rounded_rectangle(
                        Point::new((ox + x) * k, (oy + y) * k),
                        Size::new(w * k, h * k),
                        (r * k).into(),
                    );
                    self.paint_path(frame, &path, fill, stroke, width, k, alpha);
                }
                Prim::Ellipse { x, y, rx, ry, fill, stroke, width } => {
                    let path = ellipse(
                        Point::new((ox + x) * k, (oy + y) * k),
                        rx * k,
                        ry * k,
                    );
                    self.paint_path(frame, &path, fill, stroke, width, k, alpha);
                }
                Prim::Circle { x, y, r, fill, stroke, width } => {
                    let path =
                        canvas::Path::circle(Point::new((ox + x) * k, (oy + y) * k), r * k);
                    self.paint_path(frame, &path, fill, stroke, width, k, alpha);
                }
                Prim::Ramp { x, y, w, h, from, to, stops } => {
                    // Flat strips at design-pixel pitch along the axis,
                    // each the stop table read in sRGB the way rsvg reads
                    // it. Not iced's gradient: that one `mix`es in linear
                    // light and `smoothstep`s between stops, so it can
                    // only land the trace *at* a stop -- neomil's
                    // `#c2upper` drawn that way was 14 levels off a third
                    // of the way down. Axis-aligned only here (the test
                    // `soft_only_prims_stay_soft` keeps it so); a
                    // diagonal ramp lives in a `Soft` group.
                    let vertical = from.0 == to.0;
                    let n = (if vertical { h } else { w }).ceil().max(1.0) as usize;
                    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
                    let len2 = (dx * dx + dy * dy).max(f32::EPSILON);
                    for i in 0..n {
                        let f = (i as f32 + 0.5) / n as f32;
                        let (fx, fy) = if vertical { (0.5, f) } else { (f, 0.5) };
                        let t = ((fx - from.0) * dx + (fy - from.1) * dy) / len2;
                        let colour = self.blend(soft::stop(stops, t.clamp(0.0, 1.0)));
                        let (sx, sy, sw, sh) = if vertical {
                            (x, y + h * i as f32 / n as f32, w, h / n as f32)
                        } else {
                            (x + w * i as f32 / n as f32, y, w / n as f32, h)
                        };
                        frame.fill_rectangle(
                            Point::new((ox + sx) * k, (oy + sy) * k),
                            Size::new(sw * k, sh * k),
                            colour,
                        );
                    }
                }
                Prim::Plate { group, index, on, off, .. } => {
                    let prims = if self.picked.get(group) == index { on } else { off };
                    self.paint(frame, prims, ox, oy, k, alpha);
                }
                Prim::At { x, y, prims } => self.paint(frame, prims, ox + x, oy + y, k, alpha),
                Prim::Motion { motion, prims } => {
                    let t = motion::progress(&motion, self.at);
                    match motion.change {
                        Change::Clip { x, y, w, h } => {
                            // `with_clip` drafts a frame whose
                            // coordinates are this one's and applies
                            // the region as a scissor when it pastes,
                            // so the sub-scene paints with the same
                            // running translation. An empty region
                            // is skipped rather than handed to wgpu
                            // as a zero-sized scissor.
                            //
                            // The paste lands the draft's meshes
                            // *under* everything this frame draws
                            // directly, before or after: a frame's
                            // own geometry is batched only when it
                            // is finished. So a clipped group must
                            // not overlap a prim outside it that is
                            // meant to paint under it. No table does
                            // today (the goldens would show it);
                            // `mail.rs`'s `Sheet::under` says how a
                            // scene that needed to would be drawn.
                            let region = Rectangle {
                                x: (ox + x) * k,
                                y: (oy + y) * k,
                                width: Change::lerp(w, t) * k,
                                height: Change::lerp(h, t) * k,
                            };
                            if region.width > 0.0 && region.height > 0.0 {
                                frame.with_clip(region, |f| self.paint(f, prims, ox, oy, k, alpha));
                            }
                        }
                        Change::Opacity { alpha: fade } => {
                            // A group alpha the canvas does not have:
                            // each prim's ink is faded on its way to
                            // the linear rebase (`ink`), which is what
                            // rsvg's group opacity comes to for prims
                            // that do not overlap. Where they do, the
                            // stack shows through more than the trace's
                            // -- the same limit `blend` states, and a
                            // fading group is at most a few frames from
                            // rest. Nothing is painted at 0.
                            let a = alpha * Change::lerp(fade, t);
                            if a > 0.0 {
                                self.paint(frame, prims, ox, oy, k, a);
                            }
                        }
                    }
                }
                Prim::Turn { x, y, angle, prims } => {
                    // iced's `Frame::rotate` composes euclid's
                    // `[cos sin; -sin cos]`, the same matrix as SVG's
                    // `rotate(a)`: a positive angle is clockwise on
                    // screen, so the trace's degrees pass straight
                    // through. Text under a rotation is rendered by
                    // iced as filled glyph outlines, so a label turns
                    // with its group rather than staying upright.
                    frame.with_save(|f| {
                        f.translate(iced::Vector::new((ox + x) * k, (oy + y) * k));
                        f.rotate(iced::Radians(angle.to_radians()));
                        self.paint(f, prims, 0.0, 0.0, k, alpha);
                    });
                }
                // Painted by the `Backdrop` canvas underneath; see
                // `Scene::view`.
                Prim::Soft { .. } => {}
                // Composited only: a luminance mask has no canvas
                // drawing, and a radial gradient has no exact one --
                // until 2026-09-05 a `Lobe` here was 96 even-odd
                // annuli, each rebased for the linear blend, which
                // banded where `soft.rs` reads the gradient at every
                // pixel. `soft_only_prims_stay_soft` keeps both inside
                // a `Soft` group where `Backdrop` finds them.
                Prim::Masked { .. } | Prim::Lobe { .. } => {}
            }
        }
    }

    fn paint_path(
        &self,
        frame: &mut canvas::Frame,
        path: &canvas::Path,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
        k: f32,
        alpha: f32,
    ) {
        if let Some(ink) = fill {
            frame.fill(
                path,
                canvas::Fill {
                    style: canvas::Style::Solid(self.ink(ink, alpha)),
                    // Even-odd throughout: it is what a one-subpath
                    // shape already does, and it is what cuts the
                    // counter out of a logotype glyph.
                    rule: canvas::fill::Rule::EvenOdd,
                },
            );
        }
        if let Some(ink) = stroke {
            frame.stroke(
                path,
                canvas::Stroke::default()
                    .with_color(self.ink(ink, alpha))
                    .with_width(width * k),
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_text(
        &self,
        frame: &mut canvas::Frame,
        content: &str,
        x: f32,
        baseline: f32,
        size: f32,
        ink: Ink,
        face: Face,
        anchor: Anchor,
        alpha: f32,
    ) {
        frame.fill_text(Self::text(content, x, baseline, size, self.ink(ink, alpha), face, anchor));
    }

    /// A run on its SVG baseline, as the canvas lays it out: what
    /// `paint_text` fills and what `Prim::Outlined` strokes, built in
    /// one place so the two land on the same pixels.
    fn text(
        content: &str,
        x: f32,
        baseline: f32,
        size: f32,
        color: Color,
        face: Face,
        anchor: Anchor,
    ) -> canvas::Text {
        canvas::Text {
            content: content.to_string(),
            position: Point::new(x, baseline - size * self::baseline(face)),
            color,
            size: size.into(),
            font: Self::font(face),
            align_x: Self::align(anchor),
            align_y: iced::alignment::Vertical::Top,
            ..Default::default()
        }
    }
}

/// Where a run of width `run` starts when it is anchored at `x`: SVG's
/// `text-anchor`, for the runs the scene lays out itself
/// ([`Prim::Tracked`], whose glyphs it places one by one, and
/// [`Prim::Wide`], whose `run` is the stretched width).
pub(crate) fn anchored(x: f32, anchor: Anchor, run: f32) -> f32 {
    x - match anchor {
        Anchor::Start => 0.0,
        Anchor::Middle => run / 2.0,
        Anchor::End => run,
    }
}

/// The advance of each glyph of `content` at `size` in `font`, measured
/// through the shaper as prefix widths so kerning inside the run is
/// kept. What a tracked run is walked with, here and on the login.
pub(crate) fn advances(content: &str, size: f32, font: iced::Font) -> Vec<f32> {
    let mut out = Vec::with_capacity(content.chars().count());
    let mut previous = 0.0;
    let mut end = 0;
    for glyph in content.chars() {
        end += glyph.len_utf8();
        let width = run_width(&content[..end], size, font);
        out.push((width - previous).max(0.0));
        previous = width;
    }
    out
}

fn run_width(content: &str, size: f32, font: iced::Font) -> f32 {
    use iced::advanced::text::Paragraph as _;

    iced::advanced::graphics::text::Paragraph::with_text(iced::advanced::text::Text {
        content,
        bounds: Size::INFINITE,
        size: size.into(),
        line_height: iced::widget::text::LineHeight::Relative(1.0),
        font,
        align_x: iced::advanced::text::Alignment::Left,
        align_y: iced::alignment::Vertical::Top,
        shaping: iced::advanced::text::Shaping::Advanced,
        wrapping: iced::advanced::text::Wrapping::None,
    })
    .min_bounds()
    .width
}

/// The sRGB decode, on one channel: what wgpu does to a vertex colour
/// before it blends (`Color::into_linear`, spelled out so the test can
/// hold the inverse next to it).
fn to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// `c` composited in sRGB at its own alpha over `b`, as rsvg paints it.
fn srgb_over(c: Color, b: Color) -> [f32; 3] {
    let mix = |x: f32, y: f32| c.a * x + (1.0 - c.a) * y;
    [mix(c.r, b.r), mix(c.g, b.g), mix(c.b, b.b)]
}

/// The linear-blend colour that lands where `c` would under sRGB
/// compositing over `b`: the ink unchanged, the alpha rescaled. Opaque
/// and fully transparent colours pass through, so a table of solid
/// inks -- including one that pre-mixes its opacities by hand, as
/// neokitsch's `RING_*` do -- is unaffected.
///
/// Why the alpha and not the ink: the two ways to be exact over `b`
/// are to keep `c` and shrink `a`, or to keep `a` and darken `c`. Both
/// are approximate where prims stack, because sRGB compositing adds
/// *more* linear light over a brighter backdrop and a fixed linear
/// layer adds less; the darkened ink is the worse of the two there
/// (kitsch's five-deep WEAPONS ghosts: trace G 139, none 148, alpha
/// 116, darkened ink 102) and it also shifts the hue, so the alpha is
/// what moves. Measured on the kitsch dashboard, G2i 31% -> 45% with
/// the alpha, 32% with the ink.
pub(crate) fn blend_over(c: Color, b: Color) -> Color {
    if c.a <= 0.0 || c.a >= 1.0 {
        return c;
    }
    const LUMA: [f32; 3] = [0.2126, 0.7152, 0.0722];
    let r = srgb_over(c, b);
    let (mut num, mut den) = (0.0, 0.0);
    for (i, (cc, bb)) in [(c.r, b.r), (c.g, b.g), (c.b, b.b)].into_iter().enumerate() {
        num += LUMA[i] * (to_linear(r[i]) - to_linear(bb));
        den += LUMA[i] * (to_linear(cc) - to_linear(bb));
    }
    if den.abs() < 1e-6 {
        // The ink is the ground: any alpha paints the same pixel.
        return c;
    }
    Color { a: (num / den).clamp(0.0, 1.0), ..c }
}

/// A whole-turn elliptical arc, for the path builder.
fn elliptical(center: Point, rx: f32, ry: f32) -> canvas::path::arc::Elliptical {
    canvas::path::arc::Elliptical {
        center,
        radii: iced::Vector::new(rx, ry),
        rotation: iced::Radians(0.0),
        start_angle: iced::Radians(0.0),
        end_angle: iced::Radians(std::f32::consts::TAU),
    }
}

/// An ellipse as a closed path.
fn ellipse(center: Point, rx: f32, ry: f32) -> canvas::Path {
    canvas::Path::new(|b| b.ellipse(elliptical(center, rx, ry)))
}

impl<M> canvas::Program<M, Style> for Scene<M> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<M>> {
        let iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event else {
            return None;
        };
        let at = cursor.position_in(bounds)?;
        let (group, index) = hit(self.prims, scale(bounds), at)?;
        Some(canvas::Action::publish((self.on_select)(group, index)).and_capture())
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Interaction {
        cursor
            .position_in(bounds)
            .and_then(|at| hit(self.prims, scale(bounds), at))
            .map_or(Interaction::default(), |_| Interaction::Pointer)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Style,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let k = scale(bounds);
        if k > 0.0 {
            self.paint(&mut frame, self.prims, 0.0, 0.0, k, 1.0);
        }
        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sRGB encode, inverse of `to_linear`.
    fn to_srgb(l: f32) -> f32 {
        if l <= 0.003_130_8 {
            l * 12.92
        } else {
            1.055 * l.powf(1.0 / 2.4) - 0.055
        }
    }

    fn rgb(h: u32) -> Color {
        Color::from_rgb8((h >> 16) as u8, (h >> 8) as u8, h as u8)
    }

    /// What wgpu paints: `c` blended in linear light over `b`, back in
    /// 8-bit sRGB.
    fn linear_blend(c: Color, b: Color) -> [u8; 3] {
        let ch = |x: f32, y: f32| {
            (to_srgb(c.a * to_linear(x) + (1.0 - c.a) * to_linear(y)) * 255.0).round() as u8
        };
        [ch(c.r, b.r), ch(c.g, b.g), ch(c.b, b.b)]
    }

    /// What rsvg paints: the same blend on the encoded values.
    fn srgb_blend(c: Color, b: Color) -> [u8; 3] {
        srgb_over(c, b).map(|v| (v * 255.0).round() as u8)
    }

    /// Every `Prim::Soft` group in every era table holds only what
    /// `soft.rs` rasterises: text, grain, dots and plates go in the
    /// scene proper. See the module note there for why.
    #[test]
    fn soft_groups_hold_only_fills() {
        fn check(prims: &[Prim], where_: &str) {
            for prim in prims {
                match *prim {
                    Prim::Soft { prims } => {
                        assert!(
                            prims.iter().all(soft::supported),
                            "{where_}: a Soft group holds a prim soft.rs does not rasterise"
                        );
                    }
                    Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Motion { prims, .. } => check(prims, where_),
                    Prim::Plate { on, off, .. } => {
                        check(on, where_);
                        check(off, where_);
                    }
                    _ => {}
                }
            }
        }
        for era in crate::style::Era::ALL {
            let style = era.style();
            check(style.dashboard, &format!("{era:?} dashboard"));
            check(style.store, &format!("{era:?} store"));
            check(style.mailbox.backdrop, &format!("{era:?} mailbox backdrop"));
            check(style.access.backdrop, &format!("{era:?} login backdrop"));
        }
    }

    /// `Prim::Masked` and `Prim::Lobe` have no canvas drawing (`paint`
    /// skips them), so one outside a `Soft` group would vanish without
    /// a word; and the canvas draws `Prim::Ramp` as strips along one
    /// axis, so a diagonal one outside a `Soft` group would come out
    /// wrong rather than vanish.
    #[test]
    fn soft_only_prims_stay_soft() {
        fn check(prims: &[Prim], where_: &str) {
            for prim in prims {
                match *prim {
                    Prim::Masked { .. } | Prim::Lobe { .. } => {
                        panic!("{where_}: a composited-only prim outside a Soft group")
                    }
                    Prim::Ramp { from, to, .. } => {
                        assert!(
                            from.0 == to.0 || from.1 == to.1,
                            "{where_}: a diagonal Ramp outside a Soft group"
                        );
                    }
                    Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Motion { prims, .. } => check(prims, where_),
                    Prim::Plate { on, off, .. } => {
                        check(on, where_);
                        check(off, where_);
                    }
                    Prim::Soft { .. } => {}
                    _ => {}
                }
            }
        }
        for era in crate::style::Era::ALL {
            let style = era.style();
            check(style.dashboard, &format!("{era:?} dashboard"));
            check(style.store, &format!("{era:?} store"));
            check(style.mailbox.backdrop, &format!("{era:?} mailbox backdrop"));
            check(style.access.backdrop, &format!("{era:?} login backdrop"));
        }
    }

    /// A `Prim::Soft` group is a backdrop, and `Backdrop` paints only
    /// the groups that lead a scene's list: one after any other prim
    /// would be silently dropped, and one nested in an `At`, `Turn` or
    /// plate would be too. The one nesting `Backdrop` does see is a
    /// leading `Motion` holding nothing but `Soft` groups (`is_soft`);
    /// a `Motion` that mixes a `Soft` group with anything else is
    /// neither backdrop nor scene, and is caught here as nested.
    #[test]
    fn soft_groups_lead_their_scene() {
        fn none_nested(prims: &[Prim], where_: &str) {
            for prim in prims {
                match *prim {
                    Prim::Soft { .. } => panic!("{where_}: a Soft group is not at the top level"),
                    Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Motion { prims, .. } => none_nested(prims, where_),
                    Prim::Plate { on, off, .. } => {
                        none_nested(on, where_);
                        none_nested(off, where_);
                    }
                    _ => {}
                }
            }
        }
        for era in crate::style::Era::ALL {
            let style = era.style();
            for (prims, screen) in [
                (style.dashboard, "dashboard"),
                (style.store, "store"),
                (style.mailbox.backdrop, "mailbox backdrop"),
                (style.access.backdrop, "login backdrop"),
            ] {
                let where_ = format!("{era:?} {screen}");
                let lead = leading_soft(prims).len();
                for prim in &prims[lead..] {
                    assert!(!matches!(prim, Prim::Soft { .. }), "{where_}: a Soft group follows a non-Soft prim");
                    match *prim {
                        Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Motion { prims, .. } => none_nested(prims, &where_),
                        Prim::Plate { on, off, .. } => {
                            none_nested(on, &where_);
                            none_nested(off, &where_);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// A motion over a leading `Soft` group is a clip: `Backdrop` cuts
    /// the composite and scissors it, and has no exact way to fade an
    /// image (the opacity would blend in linear light). Nothing needs
    /// a fade there yet; this is where to start if something does.
    #[test]
    fn soft_motions_only_clip() {
        for era in crate::style::Era::ALL {
            let style = era.style();
            for (prims, screen) in [
                (style.dashboard, "dashboard"),
                (style.store, "store"),
                (style.mailbox.backdrop, "mailbox backdrop"),
                (style.access.backdrop, "login backdrop"),
            ] {
                for prim in leading_soft(prims) {
                    if let Prim::Motion { motion, .. } = prim {
                        assert!(
                            matches!(motion.change, Change::Clip { .. }),
                            "{era:?} {screen}: #{} fades a Soft group",
                            motion.id
                        );
                    }
                }
            }
        }
    }

    /// The cut `Backdrop` draws a moving `Soft` group as is exact at
    /// rest only where the groups after it touch none of its pixels
    /// (`soft::composite_over`). Kitsch's two ghost fans overlap in
    /// their clip boxes -- x 668..774 -- but no ghost of one lands on
    /// a pixel of the other, so the rest frame is the one-buffer
    /// composite and a frame mid-wipe shows nothing early.
    #[test]
    fn the_kitsch_fans_share_no_pixel() {
        let style = crate::style::Era::Kitsch.style();
        let fans: Vec<&'static [Prim]> = leading_soft(style.dashboard)
            .iter()
            .filter_map(|p| match p {
                Prim::Motion { prims, .. } => Some(prims.iter()),
                _ => None,
            })
            .flatten()
            .filter_map(|p| match *p {
                Prim::Soft { prims } => Some(prims),
                _ => None,
            })
            .collect();
        assert_eq!(fans.len(), 2, "two fans");
        let (w, h) = (FRAME.0 as u32, FRAME.1 as u32);
        let left = soft::touched(fans[0], &style.palette, w, h, 1.0);
        let right = soft::touched(fans[1], &style.palette, w, h, 1.0);
        let shared = left.iter().zip(&right).filter(|(l, r)| **l && **r).count();
        assert_eq!(shared, 0, "{shared} pixels under both fans");
        assert!(left.iter().any(|&t| t) && right.iter().any(|&t| t), "both fans paint");
    }

    /// A `Prim::Motion`'s clip is a rectangle in the scene's frame:
    /// `with_clip` drafts a fresh frame, so a transform set by an
    /// enclosing `Turn` would not reach it and the clip would land
    /// unrotated over a rotated drawing. Nothing needs one yet; the
    /// day something does, this is where to start.
    #[test]
    fn motion_groups_are_not_turned() {
        fn check(prims: &[Prim], turned: bool, where_: &str) {
            for prim in prims {
                match *prim {
                    Prim::Motion { motion, prims } => {
                        assert!(!turned, "{where_}: #{} is inside a Turn", motion.id);
                        check(prims, turned, where_);
                    }
                    Prim::Turn { prims, .. } => check(prims, true, where_),
                    Prim::At { prims, .. } | Prim::Soft { prims } => check(prims, turned, where_),
                    Prim::Plate { on, off, .. } => {
                        check(on, turned, where_);
                        check(off, turned, where_);
                    }
                    _ => {}
                }
            }
        }
        for era in crate::style::Era::ALL {
            let style = era.style();
            check(style.dashboard, false, &format!("{era:?} dashboard"));
            check(style.store, false, &format!("{era:?} store"));
        }
    }

    /// A `Change::Opacity` fade is the ink at that `fill-opacity`: a
    /// solid ink faded to .4 is rebased exactly as the trace's `.4`
    /// would be, and at 1 it is untouched.
    #[test]
    fn a_faded_ink_is_the_translucent_one() {
        let style = crate::style::Era::Neokitsch.style();
        let scene: Scene<()> = Scene {
            style,
            prims: &[],
            picked: Picked { category: 0, card: 0, module: 0 },
            at: crate::motion::REST,
            on_select: |_, _| (),
        };
        let gold = rgb(0xf2b463);
        let ink = Ink::Fixed(gold);
        assert_eq!(scene.ink(ink, 1.0), gold);
        let faded = scene.ink(ink, 0.4);
        let translucent = blend_over(Color { a: 0.4, ..gold }, style.palette.bg);
        assert_eq!(faded, translucent);
        assert!(faded.a < 0.4, "the linear rebase darkens a fade over a dark ground: {}", faded.a);
        assert_eq!(scene.ink(ink, 0.0).a, 0.0);
    }

    /// Every boot-in in every era table is over by `motion::REST`, so
    /// the goldens -- taken at REST -- are the traces at rest and not
    /// a frame of something still moving.
    #[test]
    fn every_motion_rests_before_rest() {
        fn rests(motion: &crate::style::Motion, where_: &str) {
            let ends = std::time::Duration::from_millis(u64::from(motion.begin + motion.dur));
            assert!(ends <= crate::motion::REST, "{where_}: #{} ends at {ends:?}", motion.id);
        }
        fn check(prims: &[Prim], where_: &str) {
            for prim in prims {
                match *prim {
                    Prim::Motion { motion, prims } => {
                        rests(&motion, where_);
                        check(prims, where_);
                    }
                    Prim::At { prims, .. } | Prim::Turn { prims, .. } | Prim::Soft { prims } => check(prims, where_),
                    Prim::Plate { on, off, .. } => {
                        check(on, where_);
                        check(off, where_);
                    }
                    _ => {}
                }
            }
        }
        for era in crate::style::Era::ALL {
            let style = era.style();
            check(style.dashboard, &format!("{era:?} dashboard"));
            check(style.store, &format!("{era:?} store"));
            check(style.mailbox.backdrop, &format!("{era:?} mailbox backdrop"));
            check(style.access.backdrop, &format!("{era:?} login backdrop"));
            for m in style.mailbox.motions {
                rests(&m.motion, &format!("{era:?} mailbox"));
                assert!(!m.parts.is_empty(), "{era:?} mailbox: #{} moves nothing", m.motion.id);
            }
        }
    }

    /// Kitsch's faintest ghost, `#0f9f80` at .12 over the hub ground:
    /// the trace has G 31 (measured 33 on the haze), a naive linear
    /// blend paints 60, and the rebased alpha lands within a level.
    #[test]
    fn ghost_over_flat_ground_matches_the_trace() {
        let ground = rgb(0x0d0d0c);
        let ghost = Color { a: 0.12, ..rgb(0x0f9f80) };
        let design = srgb_blend(ghost, ground);
        let naive = linear_blend(ghost, ground);
        let fixed = linear_blend(blend_over(ghost, ground), ground);
        assert_eq!(design[1], 31);
        assert!(naive[1] >= 58, "naive linear blend was {naive:?}");
        for i in 0..3 {
            assert!(
                (fixed[i] as i16 - design[i] as i16).abs() <= 1,
                "channel {i}: design {design:?}, rebased {fixed:?}"
            );
        }
    }

    /// The alpha only ever shrinks for an ink brighter than its ground,
    /// and the ink itself is never touched.
    #[test]
    fn rebasing_moves_only_the_alpha() {
        let ground = rgb(0x0a0a0a);
        for a in [0.07, 0.12, 0.5, 0.85] {
            let c = Color { a, ..rgb(0x6cc4bd) };
            let out = blend_over(c, ground);
            assert_eq!((out.r, out.g, out.b), (c.r, c.g, c.b));
            assert!(out.a < a && out.a > 0.0, "alpha {a} -> {}", out.a);
        }
    }

    /// Opaque and clear colours pass through, so neokitsch's hand
    /// pre-mixed `RING_*` inks are not corrected twice.
    #[test]
    fn opaque_and_clear_pass_through() {
        let ground = rgb(0x0e0a0d);
        let ring = rgb(0x3a2a1e);
        assert_eq!(blend_over(ring, ground), ring);
        let clear = Color { a: 0.0, ..ring };
        assert_eq!(blend_over(clear, ground), clear);
        // An ink equal to its ground has no alpha to solve for.
        let same = Color { a: 0.3, ..ground };
        assert_eq!(blend_over(same, ground), same);
    }

    /// SVG's `text-anchor` on a run the scene lays out itself: a start
    /// anchor is the pen position, a middle anchor halves the run
    /// about it, an end anchor ends there. For a stretched run the
    /// anchor is taken on the stretched width -- kitsch's boxed `A`,
    /// a 9.3 advance under `scale(1.7 1)` centred at 176.7, starts at
    /// 176.7 - 15.8 / 2 and its far end mirrors that about the centre.
    #[test]
    fn anchors_place_a_run_as_text_anchor_does() {
        assert_eq!(anchored(100.0, Anchor::Start, 40.0), 100.0);
        assert_eq!(anchored(100.0, Anchor::Middle, 40.0), 80.0);
        assert_eq!(anchored(100.0, Anchor::End, 40.0), 60.0);
        // An empty run anchors at its position whatever the anchor.
        for anchor in [Anchor::Start, Anchor::Middle, Anchor::End] {
            assert_eq!(anchored(12.5, anchor, 0.0), 12.5);
        }
        let (cx, advance, stretch) = (176.7, 9.3, 1.7);
        let run = advance * stretch;
        let start = anchored(cx, Anchor::Middle, run);
        assert!((start - (176.7 - 15.81 / 2.0)).abs() < 1e-3, "start {start}");
        assert!(((start + run) - cx - (cx - start)).abs() < 1e-4, "not symmetric about cx");
        // The stretch is in the run, not the anchor: at 1.0 the same
        // advance centres a narrower run about the same point.
        let plain = anchored(cx, Anchor::Middle, advance);
        assert!(plain > start && (plain + advance) < start + run);
    }

    /// A tall plate standing 60 right of a `Turn` pivot, turned +30: its
    /// centre lands below and to the right (SVG's clockwise on a y-down
    /// screen), `plates` and `hit_at` agree on where, and a point that
    /// only the *unturned* box would cover misses.
    #[test]
    fn a_plate_inside_a_turn_is_hit_at_its_turned_centre() {
        const TURNED: &[Prim] = &[Prim::Turn {
            x: 400.0,
            y: 300.0,
            angle: 30.0,
            prims: &[Prim::Plate {
                group: Group::Module,
                index: 3,
                x: 50.0,
                y: -50.0,
                w: 20.0,
                h: 100.0,
                on: &[],
                off: &[],
            }],
        }];
        let k = 0.5;
        let mut centres = Vec::new();
        plates(TURNED, 0.0, 0.0, &mut centres);
        assert_eq!(centres.len(), 1);
        let (group, index, c) = centres[0];
        assert_eq!((group, index), (Group::Module, 3));
        // (60, 0) turned 30 clockwise: (60 cos 30, 60 sin 30).
        let (ex, ey) = (400.0 + 60.0 * 0.866_025_4, 300.0 + 30.0);
        assert!((c.x - ex).abs() < 1e-3 && (c.y - ey).abs() < 1e-3, "centre {c:?}");
        assert!(c.y > 300.0, "positive angle must turn clockwise (y down)");
        assert_eq!(hit(TURNED, k, Point::new(c.x * k, c.y * k)), Some((Group::Module, 3)));
        // The unturned box's top end (460, 255) maps to local
        // (29.5, -69): off the end of the turned plate, so a miss.
        assert_eq!(hit(TURNED, k, Point::new(460.0 * k, 255.0 * k)), None);
        // The far end of the plate, in its own frame (60, 45), turned.
        let (fx, fy) = turned(60.0, 45.0, 30.0);
        assert_eq!(
            hit(TURNED, k, Point::new((400.0 + fx) * k, (300.0 + fy) * k)),
            Some((Group::Module, 3))
        );
    }
}
