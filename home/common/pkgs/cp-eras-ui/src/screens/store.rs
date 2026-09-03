//! The 4ST store: the toolkit's acceptance test.
//!
//! All four eras' references show this screen, and
//! `docs/<era>/store-trace.svg` measures each of them. This module is
//! the *renderer* for those four traces: it walks
//! [`crate::style::Style::store`] -- the era's scene, as data -- and
//! paints it. Nothing here names an era, and there is no `if era ==`:
//! the four screens differ by their table entry, the way
//! [`crate::style::Layout`] and [`crate::style::Menu`] already work,
//! just with a richer value. `src/style.rs`'s store section records why
//! this one screen carries geometry rather than a composition; the
//! short version is that the four traces do not disagree about a
//! *shape*, they disagree about the furniture around it, and no corner
//! radius turns entropism's segmented header strip into neokitsch's
//! eight-strand wire band.
//!
//! The scene is drawn on a single canvas at the trace's own 1600x900
//! coordinates, so a figure in an era table can be diffed against the
//! SVG line it came from and `scripts/fidelity_check.sh --implementation
//! <era> store` compares like with like.
//!
//! Run it with `cp-eras-ui-store --era <name>`; with no flag it
//! follows the desktop theme.

use crate::style::{Anchor, Face, Group, Ink, Prim, Seg, Style};
use crate::widgets::ground;
use iced::widget::{canvas, stack};
use iced::mouse::Interaction;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};

/// The frame every store trace is measured in. The scene is painted at
/// this size and scaled to the canvas, so the screen holds its
/// proportions in a window that is not exactly 1600x900 -- the golden
/// matrix and G2i both use exactly that, and nothing else should have
/// to.
const FRAME: (f32, f32) = (1600.0, 900.0);

/// Where a glyph's baseline sits inside the line box iced lays out when
/// the text is top-aligned: the leading above the ascender plus the
/// ascender itself, as a fraction of the font size. The traces give
/// every text run its SVG baseline, so the scene has to convert.
const BASELINE: f32 = 0.84;

pub struct Store {
    pub style: Style,
    /// The chosen category and card, as indices into the era's plates.
    /// Seeded from [`Style::store_selection`], which is what makes the
    /// opening state match each era's own material.
    pub category: usize,
    pub card: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// A plate was clicked: pick it for its group.
    Select { group: Group, index: usize },
}

impl Store {
    pub fn new(style: Style) -> Self {
        let (category, card) = style.store_selection;
        Store {
            style,
            category,
            card,
        }
    }

    pub fn title(&self) -> String {
        format!("4ST STORE — {}", self.style.era.name())
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Select {
                group: Group::Category,
                index,
            } => self.category = index,
            Message::Select {
                group: Group::Card,
                index,
            } => self.card = index,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            ground(&self.style),
            canvas(Scene {
                style: self.style,
                category: self.category,
                card: self.card,
            })
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .into()
    }
}

/// The era's store scene, painted.
#[derive(Debug, Clone, Copy)]
struct Scene {
    style: Style,
    category: usize,
    card: usize,
}

impl Scene {
    /// Resolve one of the scene's inks against the live palette, so a
    /// published theme still re-dresses the screen.
    fn ink(&self, ink: Ink) -> Color {
        ink.of(&self.style.palette)
    }

    /// The colour a gradient stop table holds at `t`, interpolated in
    /// sRGB between the two stops that bracket it.
    fn stop(stops: &[(f32, Color)], t: f32) -> Color {
        let Some(&(_, first)) = stops.first() else {
            return Color::TRANSPARENT;
        };
        let mut prev = (0.0, first);
        for &(offset, color) in stops {
            if t <= offset {
                let span = offset - prev.0;
                let f = if span > 0.0 { (t - prev.0) / span } else { 0.0 };
                let mix = |a: f32, b: f32| a + (b - a) * f;
                return Color {
                    r: mix(prev.1.r, color.r),
                    g: mix(prev.1.g, color.g),
                    b: mix(prev.1.b, color.b),
                    a: mix(prev.1.a, color.a),
                };
            }
            prev = (offset, color);
        }
        prev.1
    }

    /// The current selection for a group.
    fn picked(&self, group: Group) -> usize {
        match group {
            Group::Category => self.category,
            Group::Card => self.card,
        }
    }

    /// The frame-to-canvas scale, and the same one `draw` paints with.
    fn scale(bounds: Rectangle) -> f32 {
        (bounds.width / FRAME.0).min(bounds.height / FRAME.1)
    }

    /// Which plate, if any, sits under a point in canvas coordinates.
    ///
    /// Walks the scene the way `paint` does so the hit boxes cannot
    /// drift from the drawing: both come from the same table, through
    /// the same `Prim::At` translations, at the same scale.
    fn hit(prims: &[Prim], ox: f32, oy: f32, k: f32, at: Point) -> Option<(Group, usize)> {
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
                    if let Some(f) = Self::hit(prims, ox + x, oy + y, k, at) {
                        return Some(f);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn font(face: Face) -> iced::Font {
        match face {
            Face::Regular => crate::fonts::FONT_RAJDHANI_REGULAR,
            // No published semibold face; see `bar::era_face`.
            Face::Medium | Face::SemiBold => crate::fonts::FONT_RAJDHANI_MEDIUM,
            Face::Bold => crate::fonts::FONT_RAJDHANI_BOLD,
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
    /// [`Prim::At`] accumulates and `k` the frame-to-canvas scale.
    fn paint(
        &self,
        frame: &mut canvas::Frame,
        prims: &[Prim],
        ox: f32,
        oy: f32,
        k: f32,
    ) {
        for prim in prims {
            match *prim {
                Prim::Rect { x, y, w, h, fill, stroke, width } => {
                    let path = canvas::Path::rectangle(
                        Point::new((ox + x) * k, (oy + y) * k),
                        Size::new(w * k, h * k),
                    );
                    self.paint_path(frame, &path, fill, stroke, width, k);
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
                    self.paint_path(frame, &path, fill, stroke, width, k);
                }
                Prim::Text { x, y, size, ink, face, anchor, content } => {
                    self.paint_text(frame, content, (ox + x) * k, (oy + y) * k, size * k, ink, face, anchor);
                }
                Prim::Wide { x, y, size, stretch, ink, face, content } => {
                    // A non-uniform transform makes iced convert the run
                    // to filled glyph outlines, which is exactly what
                    // `lengthAdjust="spacingAndGlyphs"` asks for.
                    let color = self.ink(ink);
                    let (px, py) = ((ox + x) * k, (oy + y) * k - size * k * BASELINE);
                    let font = Self::font(face);
                    let size = size * k;
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
                        );
                    }
                }
                Prim::Grain { x, y, w, h, pitch, width, ink } => {
                    let color = self.ink(ink);
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
                    let color = self.ink(ink);
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
                    self.paint_path(frame, &path, fill, stroke, width, k);
                }
                Prim::Lobe { x, y, rx, ry, stops } => {
                    // Enough rings that the steps are under a level per
                    // ring at the gradient's steepest, which is what it
                    // takes for the banding to go away by eye.
                    const RINGS: usize = 96;
                    let centre = Point::new((ox + x) * k, (oy + y) * k);
                    for i in 0..RINGS {
                        let outer = 1.0 - i as f32 / RINGS as f32;
                        let inner = 1.0 - (i + 1) as f32 / RINGS as f32;
                        // Disjoint annuli, not stacked discs. A stop
                        // table may carry *opacities* -- kitsch's left
                        // margin is one colour at three alphas over the
                        // bloom -- and overlapping translucent discs
                        // composite each other 96 times over. Two
                        // ellipses in one even-odd path is an exact
                        // elliptical annulus, so every pixel is painted
                        // once and alpha lands on the backdrop.
                        let path = canvas::Path::new(|b| {
                            b.ellipse(elliptical(centre, rx * outer * k, ry * outer * k));
                            if inner > 0.0 {
                                b.ellipse(elliptical(centre, rx * inner * k, ry * inner * k));
                            }
                        });
                        frame.fill(
                            &path,
                            canvas::Fill {
                                style: canvas::Style::Solid(Self::stop(
                                    stops,
                                    (outer + inner) * 0.5,
                                )),
                                rule: canvas::fill::Rule::EvenOdd,
                            },
                        );
                    }
                }
                Prim::Ellipse { x, y, rx, ry, fill, stroke, width } => {
                    let path = ellipse(
                        Point::new((ox + x) * k, (oy + y) * k),
                        rx * k,
                        ry * k,
                    );
                    self.paint_path(frame, &path, fill, stroke, width, k);
                }
                Prim::Circle { x, y, r, fill, stroke, width } => {
                    let path =
                        canvas::Path::circle(Point::new((ox + x) * k, (oy + y) * k), r * k);
                    self.paint_path(frame, &path, fill, stroke, width, k);
                }
                Prim::Wash { x, y, w, h, top, foot } => {
                    let (x0, y0) = ((ox + x) * k, (oy + y) * k);
                    let gradient = iced::advanced::graphics::gradient::Linear::new(
                        Point::new(x0, y0),
                        Point::new(x0, y0 + h * k),
                    )
                    .add_stop(0.0, self.ink(top))
                    .add_stop(1.0, self.ink(foot));
                    frame.fill(
                        &canvas::Path::rectangle(Point::new(x0, y0), Size::new(w * k, h * k)),
                        canvas::Fill {
                            style: canvas::Style::Gradient(gradient.into()),
                            rule: canvas::fill::Rule::NonZero,
                        },
                    );
                }
                Prim::Plate { group, index, on, off, .. } => {
                    let prims = if self.picked(group) == index { on } else { off };
                    self.paint(frame, prims, ox, oy, k);
                }
                Prim::At { x, y, prims } => self.paint(frame, prims, ox + x, oy + y, k),
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
    ) {
        if let Some(ink) = fill {
            frame.fill(
                path,
                canvas::Fill {
                    style: canvas::Style::Solid(self.ink(ink)),
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
                    .with_color(self.ink(ink))
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
    ) {
        frame.fill_text(canvas::Text {
            content: content.to_string(),
            position: Point::new(x, baseline - size * BASELINE),
            color: self.ink(ink),
            size: size.into(),
            font: Self::font(face),
            align_x: Self::align(anchor),
            align_y: iced::alignment::Vertical::Top,
            ..Default::default()
        });
    }
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

impl canvas::Program<Message> for Scene {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event else {
            return None;
        };
        let at = cursor.position_in(bounds)?;
        let (group, index) = Self::hit(self.style.store, 0.0, 0.0, Self::scale(bounds), at)?;
        Some(canvas::Action::publish(Message::Select { group, index }).and_capture())
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Interaction {
        cursor
            .position_in(bounds)
            .and_then(|at| Self::hit(self.style.store, 0.0, 0.0, Self::scale(bounds), at))
            .map_or(Interaction::default(), |_| Interaction::Pointer)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let k = Self::scale(bounds);
        if k > 0.0 {
            self.paint(&mut frame, self.style.store, 0.0, 0.0, k);
        }
        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Era;

    /// The centre of a plate, in canvas coordinates at `k = 1`.
    fn plates(prims: &[Prim], ox: f32, oy: f32, out: &mut Vec<(Group, usize, Point)>) {
        for prim in prims {
            match *prim {
                Prim::Plate { group, index, x, y, w, h, .. } => out.push((
                    group,
                    index,
                    Point::new(ox + x + w / 2.0, oy + y + h / 2.0),
                )),
                Prim::At { x, y, prims } => plates(prims, ox + x, oy + y, out),
                _ => {}
            }
        }
    }

    /// Every era offers the same two choices -- five categories and
    /// four cards -- however differently it draws them. An era table
    /// that forgot to wrap its shelf in plates would render fine and be
    /// dead to the mouse, which is exactly the failure this catches.
    #[test]
    fn every_era_offers_five_categories_and_four_cards() {
        for era in Era::ALL {
            let mut found = Vec::new();
            plates(era.style().store, 0.0, 0.0, &mut found);
            let cats: Vec<_> = found
                .iter()
                .filter(|(g, ..)| *g == Group::Category)
                .map(|(_, i, _)| *i)
                .collect();
            let cards: Vec<_> = found
                .iter()
                .filter(|(g, ..)| *g == Group::Card)
                .map(|(_, i, _)| *i)
                .collect();
            assert_eq!(cats, vec![0, 1, 2, 3, 4], "{} categories", era.name());
            assert_eq!(cards, vec![0, 1, 2, 3], "{} cards", era.name());
        }
    }

    /// Hit-testing walks the scene the same way painting does, so a
    /// click at a plate's own centre has to come back as that plate.
    #[test]
    fn a_click_at_a_plates_centre_selects_that_plate() {
        for era in Era::ALL {
            let store = era.style().store;
            let mut found = Vec::new();
            plates(store, 0.0, 0.0, &mut found);
            for (group, index, centre) in found {
                assert_eq!(
                    Scene::hit(store, 0.0, 0.0, 1.0, centre),
                    Some((group, index)),
                    "{} {:?} {}",
                    era.name(),
                    group,
                    index
                );
            }
        }
    }

    /// The opening selection is era data, and the traces disagree about
    /// it: entropism grows its first card, the other three their
    /// second. A screen that hardcoded either would match one trace and
    /// miss three.
    #[test]
    fn the_screen_opens_on_the_selection_its_era_was_traced_with() {
        assert_eq!(Store::new(Era::Entropism.style()).card, 0);
        for era in [Era::Kitsch, Era::Neomil, Era::Neokitsch] {
            assert_eq!(Store::new(era.style()).card, 1, "{}", era.name());
        }
        for era in Era::ALL {
            let store = Store::new(era.style());
            assert!(store.category < 5, "{}", era.name());
        }
    }

    /// Selecting moves only its own group.
    #[test]
    fn selecting_a_card_leaves_the_category_alone() {
        let mut store = Store::new(Era::Kitsch.style());
        let category = store.category;
        store.update(Message::Select {
            group: Group::Card,
            index: 3,
        });
        assert_eq!(store.card, 3);
        assert_eq!(store.category, category);
        store.update(Message::Select {
            group: Group::Category,
            index: 4,
        });
        assert_eq!(store.category, 4);
        assert_eq!(store.card, 3);
    }
}
