//! The one primitive every era's screens are built from.
//!
//! A surface is a filled and/or stroked shape whose corner treatment
//! comes from [`Corner`] and whose fill comes from [`Fill`]. Panels,
//! cards, nav pills, badges and list rows are all this widget with
//! different sizes and fills, which is what lets the store screen have a
//! single implementation across four eras.

use crate::style::{Corner, Selection, Style, Ticket};
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{layout, overlay, renderer, Clipboard, Layout, Shell, Widget};
use iced::widget::{canvas, container, stack};
use iced::{event, mouse, Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

/// Which corners a corner treatment applies to.
///
/// A parameter rather than a constant because the amount and the choice
/// come from different places: the era table says *how much* to cut and
/// the widget says *where*. In practice every caller today takes
/// [`default_corners`] -- one era-wide answer -- and the four named
/// constants below exist to serve it. The `Surface::corners` builder
/// that let a widget override it was deleted in the dead-code audit
/// along with `Corners::OPPOSED`: nothing had ever called either, and
/// `OPPOSED`'s own doc claimed it was "the neomil info panel", which
/// the sheet contradicts -- `M 1080 132 h 700 l 24 24 v 560 l -24 24
/// h -700 Z` cuts both *right* corners, not two diagonally opposite
/// ones. The field is still public, so a widget that genuinely needs
/// its own corners sets it; it just does not get a builder and a
/// wrong constant for free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corners {
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_right: bool,
    pub bottom_left: bool,
}

impl Corners {
    pub const ALL: Corners = Corners {
        top_left: true,
        top_right: true,
        bottom_right: true,
        bottom_left: true,
    };
    pub const NONE: Corners = Corners {
        top_left: false,
        top_right: false,
        bottom_right: false,
        bottom_left: false,
    };
    /// The neo-militarism default: cut the bottom-right only.
    pub const BOTTOM_RIGHT: Corners = Corners {
        bottom_right: true,
        ..Corners::NONE
    };
    pub const TOP_RIGHT: Corners = Corners {
        top_right: true,
        ..Corners::NONE
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
    None,
    Solid(Color),
    /// Neokitsch's selection material. Synthesised rather than shipped:
    /// a base tone, banded warp, and grain lines clipped to the shape.
    Veneer {
        base: Color,
        light: Color,
        dark: Color,
        grain: Color,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub corner: Corner,
    pub corners: Corners,
    pub fill: Fill,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    /// The outward wedge on the top-right, for the one widget that
    /// wears one. Default-zero, so every existing caller is unchanged
    /// and the shape falls through to the plain corner walk.
    pub ticket: Ticket,
}

impl Surface {
    /// An outlined surface in the era's border colour.
    pub fn outlined(style: &Style) -> Self {
        Surface {
            corner: style.corner,
            corners: default_corners(style.corner),
            fill: Fill::None,
            stroke: Some(style.palette.border),
            stroke_width: style.metrics.stroke,
            ticket: Ticket::default(),
        }
    }

    /// A filled surface in an arbitrary colour, unstroked.
    pub fn filled(style: &Style, color: Color) -> Self {
        Surface {
            corner: style.corner,
            corners: default_corners(style.corner),
            fill: Fill::Solid(color),
            stroke: None,
            stroke_width: style.metrics.stroke,
            ticket: Ticket::default(),
        }
    }

    /// The selected state, in whatever way this era expresses selection.
    /// This is the call that keeps `Selection::Veneer` from leaking into
    /// every widget as an era check.
    pub fn selected(style: &Style) -> Self {
        let fill = match style.selection {
            Selection::Solid => Fill::Solid(style.palette.select),
            Selection::Veneer => Fill::Veneer {
                base: crate::eras::neokitsch::VENEER,
                light: crate::eras::neokitsch::VENEER_LIGHT,
                dark: crate::eras::neokitsch::VENEER_DARK,
                grain: crate::eras::neokitsch::GRAIN,
            },
        };
        Surface {
            corner: style.corner,
            corners: default_corners(style.corner),
            fill,
            stroke: None,
            stroke_width: style.metrics.stroke,
            ticket: Ticket::default(),
        }
    }

    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = Some(color);
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke = None;
        self
    }

    /// Cut the era's ticket wedge into the top-right.
    ///
    /// A builder rather than a field on the three constructors,
    /// because the wedge belongs to *one widget* and not to the era:
    /// kitsch cuts it into its nav pills and into nothing else it
    /// draws. See [`crate::style::Ticket`].
    pub fn ticket(mut self, ticket: Ticket) -> Self {
        self.ticket = ticket;
        self
    }

    /// Drop the era's corner treatment and draw a plain rectangle.
    ///
    /// Corner treatment is a property of an era's *containers*, not of
    /// every box it draws, and the references are unanimous about it:
    /// kitsch rounds its cards `rx="16"`, its customer panel `rx="15"`
    /// and its nav pills `q0 12` -- and then draws the stat band
    /// (`rect x=536 y=510 width=258 height=26`), the three socket cells
    /// (`width=70 height=28`) and the footnote marker box (`width=26
    /// height=26`) as bare `rect`s with no `rx` at all. Neokitsch clips
    /// its cards' top-right corner and its socket cells likewise have
    /// none. `marker` already hardcoded [`Corner::Square`] for exactly
    /// this reason; this is that decision made sayable.
    ///
    /// It matters more than it sounds because [`outline`] clamps the
    /// corner amount to half the box, so kitsch's `radius: 16` on a
    /// 26px-high cell is not a rounded rectangle -- it is a full pill,
    /// which is what the stat band and the sockets had become.
    pub fn square(mut self) -> Self {
        self.corner = Corner::Square;
        self.corners = Corners::NONE;
        self
    }
}

/// Which corners an era treats by default. Only neo-militarism varies
/// them per widget; the rest apply their treatment uniformly.
pub fn default_corners(corner: Corner) -> Corners {
    match corner {
        Corner::Square => Corners::NONE,
        Corner::Chamfer { .. } => Corners::BOTTOM_RIGHT,
        Corner::Round { .. } => Corners::ALL,
        Corner::ClipTopRight { .. } => Corners::TOP_RIGHT,
    }
}

/// The part of a canvas's box that its own geometry clip will keep.
///
/// A `canvas` hands its geometry a clip rectangle of exactly the widget
/// bounds, and `iced_wgpu` turns that into a scissor rect with
/// `Rectangle::snap`, which *truncates*: `x` and `width` both lose their
/// fraction. A surface laid out at `x = 1262.5` with `width = 297.5` is
/// therefore scissored to `[1262, 1559)`, and a 1px outline whose right
/// edge sits at 1559.5 is not dimmed by the loss -- it disappears
/// entirely.
///
/// That is the whole of "the fourth card has no right border". The card
/// does not overflow anything: it ends exactly on the content edge, and
/// four `FillPortion(1)` cards divide the shelf into a fractional width
/// that puts the last one's stroke in the truncated half-pixel. Cards
/// one and three land on whole pixels and keep half their stroke, which
/// is why only the fourth looks broken. The top bar's third segment
/// fails the same way for the same reason.
///
/// So a stroked shape is built inside the rectangle the scissor will
/// actually keep. It costs at most one pixel of width and makes the
/// outline unconditional instead of a function of where the layout
/// happened to land.
pub fn visible(start: f32, len: f32) -> f32 {
    (len.floor() - (start - start.floor())).clamp(0.0, len)
}

/// The outline of a surface, as a canvas path.
///
/// `ticket` replaces the top-right corner with an outward wedge; pass
/// [`Ticket::default`] for the ordinary shape. The wedge is *inside*
/// `w`, not added to it -- a widget that wants the body at its natural
/// size asks for `body + reach` and lets this cut the difference, the
/// same convention `Banner::overhang` uses.
pub fn outline(corner: Corner, corners: Corners, ticket: Ticket, w: f32, h: f32) -> canvas::Path {
    let amount = corner.inset().min(w / 2.0).min(h / 2.0);
    // The wedge cannot eat more than the box has, and it cannot reach
    // below the bottom edge.
    let cut = ticket.is_cut() && ticket.reach < w && ticket.drop < h;
    let (reach, drop) = if cut {
        // The wedge's point must stay above whatever the bottom-right
        // corner eats, or the outline doubles back on itself. The
        // sampled figures clear it comfortably (drop 15 against a
        // 34-high pill with a 16 radius); this is for the small box
        // some future caller hands it.
        (ticket.reach, ticket.drop.min((h - amount).max(0.0)))
    } else {
        (0.0, 0.0)
    };
    // The body the wedge grows out of.
    let bw = w - reach;

    canvas::Path::new(|b| {
        if cut {
            // The kitsch nav pill, exactly: the top edge runs the body's
            // full width with *no* top-right radius, the wedge carries
            // the outline out and down, and the remaining three corners
            // take the era's treatment.
            //
            //   M172 340 h158 l18 15 v13 q0 12 -12 12 h-164 ...
            //
            // Only kitsch declares a ticket and kitsch is `Round`, but
            // the walk is written for any corner amount so that an era
            // adding one later is a table entry rather than a rewrite.
            let round = matches!(corner, Corner::Round { .. });
            let c = if matches!(corner, Corner::Square) {
                0.0
            } else {
                amount
            };
            let bl = if corners.bottom_left { c } else { 0.0 };
            let br = if corners.bottom_right { c } else { 0.0 };
            let tl = if corners.top_left { c } else { 0.0 };

            b.move_to(Point::new(tl, 0.0));
            b.line_to(Point::new(bw, 0.0));
            b.line_to(Point::new(w, drop));
            b.line_to(Point::new(w, h - br));
            if br > 0.0 && round {
                b.quadratic_curve_to(Point::new(w, h), Point::new(w - br, h));
            } else if br > 0.0 {
                b.line_to(Point::new(w - br, h));
            }
            b.line_to(Point::new(bl, h));
            if bl > 0.0 && round {
                b.quadratic_curve_to(Point::new(0.0, h), Point::new(0.0, h - bl));
            } else if bl > 0.0 {
                b.line_to(Point::new(0.0, h - bl));
            }
            b.line_to(Point::new(0.0, tl));
            if tl > 0.0 && round {
                b.quadratic_curve_to(Point::new(0.0, 0.0), Point::new(tl, 0.0));
            } else if tl > 0.0 {
                b.line_to(Point::new(tl, 0.0));
            }
            b.close();
            return;
        }

        match corner {
            Corner::Round { .. } => {
                let r = amount;
                // Quadratic corners rather than arcs: visually identical
                // at this radius and one less API surface to depend on.
                b.move_to(Point::new(r, 0.0));
                b.line_to(Point::new(w - r, 0.0));
                b.quadratic_curve_to(Point::new(w, 0.0), Point::new(w, r));
                b.line_to(Point::new(w, h - r));
                b.quadratic_curve_to(Point::new(w, h), Point::new(w - r, h));
                b.line_to(Point::new(r, h));
                b.quadratic_curve_to(Point::new(0.0, h), Point::new(0.0, h - r));
                b.line_to(Point::new(0.0, r));
                b.quadratic_curve_to(Point::new(0.0, 0.0), Point::new(r, 0.0));
            }
            _ => {
                // Square, chamfer and clip-top-right are all the same
                // walk with per-corner cuts; square just cuts nothing.
                let c = if matches!(corner, Corner::Square) {
                    0.0
                } else {
                    amount
                };
                let tl = if corners.top_left { c } else { 0.0 };
                let tr = if corners.top_right { c } else { 0.0 };
                let br = if corners.bottom_right { c } else { 0.0 };
                let bl = if corners.bottom_left { c } else { 0.0 };

                b.move_to(Point::new(tl, 0.0));
                b.line_to(Point::new(w - tr, 0.0));
                if tr > 0.0 {
                    b.line_to(Point::new(w, tr));
                }
                b.line_to(Point::new(w, h - br));
                if br > 0.0 {
                    b.line_to(Point::new(w - br, h));
                }
                b.line_to(Point::new(bl, h));
                if bl > 0.0 {
                    b.line_to(Point::new(0.0, h - bl));
                }
                b.line_to(Point::new(0.0, tl));
                if tl > 0.0 {
                    b.line_to(Point::new(tl, 0.0));
                }
            }
        }
        b.close();
    })
}

/// The shape's horizontal extent at height `y`, used to clip grain lines
/// to a surface without needing path clipping in the renderer.
///
/// All four corner treatments are convex and axis-aligned apart from a
/// single corner, so this is exact rather than an approximation. A
/// ticket wedge is convex too, and replaces the top-right corner's
/// contribution rather than adding to it.
pub fn span_at(
    corner: Corner,
    corners: Corners,
    ticket: Ticket,
    w: f32,
    h: f32,
    y: f32,
) -> (f32, f32) {
    let amount = corner.inset().min(w / 2.0).min(h / 2.0);
    let cut = ticket.is_cut() && ticket.reach < w && ticket.drop < h;
    if (amount <= 0.0 && !cut) || y < 0.0 || y > h {
        return (0.0, w);
    }

    let (mut x0, mut x1) = (0.0f32, w);

    // The wedge's hypotenuse: the right edge runs from the body's width
    // at the top edge out to the full width at `drop`, and is flush
    // below that.
    if cut {
        let bw = w - ticket.reach;
        x1 = x1.min(bw + ticket.reach * (y / ticket.drop).clamp(0.0, 1.0));
    }

    // How far in the edge is drawn at `y`, for a corner of the given
    // treatment sitting `d` away from the shape's end.
    let inward = |d: f32| -> f32 {
        if d >= amount {
            return 0.0;
        }
        match corner {
            Corner::Round { .. } => {
                let dy = amount - d;
                amount - (amount * amount - dy * dy).max(0.0).sqrt()
            }
            _ => amount - d,
        }
    };

    if corners.top_left {
        x0 = x0.max(inward(y));
    }
    if corners.bottom_left {
        x0 = x0.max(inward(h - y));
    }
    // A ticket replaces the top-right treatment; applying both would
    // clip the wedge back off again.
    if corners.top_right && !cut {
        x1 = x1.min(w - inward(y));
    }
    if corners.bottom_right {
        x1 = x1.min(w - inward(h - y));
    }
    (x0, x1)
}

/// The part of the shape lying between `y0` and `y1`, as a path.
///
/// [`span_at`] gives the shape's width at one height; a band spans a
/// range of heights, and in the corner regions that width is different
/// at every one of them. Taking a single reading and drawing a
/// rectangle with it is wrong in both directions at once, and it was
/// wrong visibly: neokitsch's veneer put a flat brown wedge *outside*
/// the clipped top-right corner of every selected card, mail row and
/// bar cell, and left a strip of untinted base inside the same corner's
/// lower half. On the store screen that wedge was some 25 by 30 pixels
/// of `(109,88,62)` sitting where the ground should be -- the light
/// warp tone at its own 16% over the page, the arithmetic confirming
/// what the crop showed.
///
/// It read, if you were feeling generous, as a folded flap: the veneer
/// showing its underside where the corner is cut. It is not one.
/// `docs/neokitsch/target-app.svg` draws the selected card as a single
/// path -- `M800 340 h190 l30 30 v410 ...` -- filled with veneer and
/// grain and *nothing else*; the cut corner is empty ground. And a
/// deliberate flap would not stop 4px short of the card's own right
/// edge, which is where `band_h / 2` happened to put it.
///
/// So the band is built from the shape instead, sampled about once a
/// pixel down its height: exact for the three straight corner
/// treatments and close enough for the rounded one, which is the same
/// standard `span_at` already holds the grain lines to.
fn band_path(
    corner: Corner,
    corners: Corners,
    ticket: Ticket,
    w: f32,
    h: f32,
    y0: f32,
    y1: f32,
) -> Option<canvas::Path> {
    let steps = (y1 - y0).ceil().max(1.0) as usize;
    let mut edges = Vec::with_capacity(steps + 1);
    for k in 0..=steps {
        let y = y0 + (y1 - y0) * k as f32 / steps as f32;
        let (x0, x1) = span_at(corner, corners, ticket, w, h, y);
        // A band can start or end outside the shape -- a corner may eat
        // the whole width at the very top -- and the shapes here are
        // convex, so the empty slices are only ever at the ends.
        if x1 > x0 {
            edges.push((x0, x1, y));
        }
    }
    if edges.len() < 2 {
        return None;
    }
    Some(canvas::Path::new(|b| {
        b.move_to(Point::new(edges[0].0, edges[0].2));
        for &(x0, _, y) in &edges[1..] {
            b.line_to(Point::new(x0, y));
        }
        for &(_, x1, y) in edges.iter().rev() {
            b.line_to(Point::new(x1, y));
        }
        b.close();
    }))
}

impl<Message> canvas::Program<Message> for Surface {
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
        // Not `bounds.width`/`bounds.height`: the trailing fraction of
        // both is scissored away before it reaches the screen. See
        // [`visible`].
        let (w, h) = (
            visible(bounds.x, bounds.width),
            visible(bounds.y, bounds.height),
        );
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }

        // A stroke straddles its path, so a shape built flush to the
        // canvas bounds loses the outer half of its outline to
        // clipping -- visible as a box missing its right and bottom
        // edges. Inset the path by half the stroke so the whole line
        // lands inside.
        let inset = match self.stroke {
            Some(_) => self.stroke_width / 2.0,
            None => 0.0,
        };
        let (pw, ph) = (w - inset * 2.0, h - inset * 2.0);
        if pw <= 0.0 || ph <= 0.0 {
            return vec![frame.into_geometry()];
        }
        frame.translate(iced::Vector::new(inset, inset));

        let path = outline(self.corner, self.corners, self.ticket, pw, ph);

        match self.fill {
            Fill::None => {}
            Fill::Solid(color) => frame.fill(&path, color),
            Fill::Veneer {
                base,
                light,
                dark,
                grain,
            } => {
                frame.fill(&path, base);

                // Warp: broad bands across the grain direction, alpha-
                // blended so the plank reads as figured rather than flat.
                //
                // Each band follows the shape rather than being a
                // rectangle placed inside it. That is not fussiness:
                // one reading of `span_at` at the band's own middle,
                // stretched over its whole height, is wrong wherever
                // the shape is not a rectangle -- and the top band
                // always lands on one of those places. See
                // [`band_path`].
                let bands = 7;
                for i in 0..bands {
                    let y0 = i as f32 / bands as f32 * ph;
                    let y1 = (i + 1) as f32 / bands as f32 * ph;
                    let tone = if i % 2 == 0 { light } else { dark };
                    if let Some(band) =
                        band_path(self.corner, self.corners, self.ticket, pw, ph, y0, y1)
                    {
                        frame.fill(
                            &band,
                            Color {
                                a: 0.16,
                                ..tone
                            },
                        );
                    }
                }

                // Grain: fine lines along the plank, clipped to the
                // shape by span rather than by renderer clipping.
                let mut y = 3.0;
                let mut n = 0;
                while y < ph {
                    let (x0, x1) = span_at(self.corner, self.corners, self.ticket, pw, ph, y);
                    if x1 > x0 {
                        let line = canvas::Path::new(|b| {
                            b.move_to(Point::new(x0 + 1.0, y));
                            b.line_to(Point::new(x1 - 1.0, y));
                        });
                        frame.stroke(
                            &line,
                            canvas::Stroke::default()
                                .with_color(Color {
                                    a: if n % 3 == 0 { 0.30 } else { 0.16 },
                                    ..grain
                                })
                                .with_width(if n % 3 == 0 { 0.9 } else { 0.6 }),
                        );
                    }
                    y += 5.0;
                    n += 1;
                }
            }
        }

        if let Some(color) = self.stroke {
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(self.stroke_width),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// Wrap `content` in a surface. The canvas sits behind rather than
/// clipping, so text keeps its own layout and the shape stays cheap.
pub fn surface<'a, Message: 'static>(
    surface: Surface,
    padding: impl Into<iced::Padding>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    stack![
        canvas(surface).width(Length::Fill).height(Length::Fill),
        container(content.into()).padding(padding).width(Length::Fill)
    ]
    .into()
}

/// Wrap `content` in a surface that takes its size from the content.
///
/// [`surface`] paints the box it is handed, which is what a caller who
/// knows the box wants -- a bar cell, a nav pill, a list row. It is the
/// wrong way round when the content is what should decide. The canvas
/// is the first layer of a `stack` and a `stack` takes its size from its
/// first layer, so a surface whose caller does not pin a height grows to
/// whatever space it is offered: the same failure that made bar cells
/// clip their own labels, seen from the other side.
///
/// A [`Backdrop`] inverts that. The content lays out first, against the
/// limits the parent gave, and the surface is then laid out to exactly
/// the size that came back.
pub fn backdrop<'a, Message: 'static>(
    surface: Surface,
    padding: impl Into<iced::Padding>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    layered(
        canvas(surface).width(Length::Fill).height(Length::Fill),
        container(content.into()).padding(padding).width(Length::Fill),
    )
}

/// Two arbitrary layers, sized by the second.
///
/// [`backdrop`] with the background left open. A card whose accent band
/// hangs past its own edge needs the shape inset while the content runs
/// the full width, and a panel whose outline encloses only its first
/// row needs a background that is not a plain [`Surface`] at all --
/// both are this, with a `container` or a `canvas` handed in.
pub fn layered<'a, Message: 'static>(
    background: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Backdrop {
        children: vec![background.into(), content.into()],
    }
    .into()
}

/// Two layers, sized by the second.
///
/// Deliberately not a `stack` with the arguments swapped: the layer that
/// decides the size has to be drawn *last*, or the fill covers the text.
/// `stack` ties those two together -- its first child both sizes the
/// widget and sits at the bottom -- and this is the one place that needs
/// them apart.
struct Backdrop<'a, Message> {
    /// `[background, content]`, in draw order. `content` is the sizer.
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Backdrop<'a, Message> {
    const CONTENT: usize = 1;
}

impl<Message> Widget<Message, Theme, Renderer> for Backdrop<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        self.children[Self::CONTENT].as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self.children[Self::CONTENT].as_widget().layout(
            &mut tree.children[Self::CONTENT],
            renderer,
            limits,
        );
        let size = content.size();

        // Min and max are the same, so the background's `Length::Fill`
        // resolves to the content's size rather than to the parent's.
        let background = self.children[0].as_widget().layout(
            &mut tree.children[0],
            renderer,
            &layout::Limits::new(size, size),
        );

        layout::Node::with_children(size, vec![background, content])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            for ((child, state), layout) in self
                .children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child.as_widget().operate(state, layout, renderer, operation);
            }
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // Topmost layer first, as `stack` does: the content is in front.
        self.children
            .iter_mut()
            .rev()
            .zip(tree.children.iter_mut().rev())
            .zip(layout.children().rev())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .find(|&status| status == event::Status::Captured)
            .unwrap_or(event::Status::Ignored)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .rev()
            .zip(tree.children.iter().rev())
            .zip(layout.children().rev())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .find(|&interaction| interaction != mouse::Interaction::None)
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer, translation)
    }
}

impl<'a, Message: 'a> From<Backdrop<'a, Message>> for Element<'a, Message> {
    fn from(backdrop: Backdrop<'a, Message>) -> Self {
        Element::new(backdrop)
    }
}
