//! The one primitive every era's screens are built from.
//!
//! A surface is a filled and/or stroked shape whose corner treatment
//! comes from [`Corners`] and whose fill comes from [`Fill`]. Panels,
//! cards, nav pills, badges and list rows are all this widget with
//! different sizes and fills, which is what lets the store screen have a
//! single implementation across four eras.

use crate::style::{Corner, Selection, Style, Ticket};
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{layout, overlay, renderer, Clipboard, Layout, Shell, Widget};
use iced::widget::{canvas, container, stack};
use iced::{mouse, Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

/// One corner's treatment.
///
/// A kind *and* an amount, both belonging to the corner rather than to
/// the era: a `Chamfer` carries its own width and height, so a cut can
/// be shallower than it is wide -- which the era table's single `cut`
/// figure could not say at all.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cut {
    Square,
    /// A diagonal cut `x` wide along the horizontal edge and `y` tall
    /// along the vertical one.
    Chamfer { x: f32, y: f32 },
    Round { radius: f32 },
    /// Kitsch's nav chevron (`mailbox-trace.svg` `#chev`, `bar.svg`
    /// item 4): the vertical edge rises from `y` to a peak `x` along the
    /// top, then drops `brow.0` further along to `brow.1` *below* the
    /// box's top, and the rest of the top edge runs at that height. A
    /// shoulder rather than a corner: the box's top is not where the
    /// shape's top is. Top-left only -- on any other corner it is the
    /// chamfer `{ x, y }`, brow and all ignored.
    Peak { x: f32, y: f32, brow: (f32, f32) },
}

impl Cut {
    /// This cut as it fits a `w` by `h` box: how far it eats along the
    /// horizontal edge, and how far along the vertical one.
    ///
    /// No corner may eat more than half of either dimension, or two of
    /// them meet in the middle and the outline doubles back on itself
    /// -- the clamp the old single `amount` applied to all four at
    /// once. Both numbers shrink by the *same* factor, so a squeezed
    /// chamfer keeps the slope it was asked for and a squeezed radius
    /// stays circular. That is also what makes this arithmetic a pure
    /// refactor: with `x == y` the scaled pair is exactly the old
    /// `min(amount, w / 2, h / 2)`.
    fn extent(self, w: f32, h: f32) -> (f32, f32) {
        let (x, y) = match self {
            Cut::Square => return (0.0, 0.0),
            Cut::Chamfer { x, y } => (x, y),
            Cut::Round { radius } => (radius, radius),
            // The rising edge alone; the brow is the top-left walk's
            // business, see `outline`.
            Cut::Peak { x, y, .. } => (x, y),
        };
        if x <= 0.0 || y <= 0.0 {
            return (0.0, 0.0);
        }
        let scale = 1.0f32.min(w / 2.0 / x).min(h / 2.0 / y).max(0.0);
        (x * scale, y * scale)
    }

    fn is_round(self) -> bool {
        matches!(self, Cut::Round { .. })
    }

    /// How far below the box's top the top edge runs: the drop of a
    /// [`Cut::Peak`], and nothing for every other cut.
    fn brow(self) -> f32 {
        match self {
            Cut::Peak { brow, .. } => brow.1,
            _ => 0.0,
        }
    }
}

/// The four corners of a surface, each with its own [`Cut`].
///
/// Four booleans until the bar redesign: the era table said *how much*
/// to cut and the widget picked one of four named subsets to apply it
/// to, so everything an era drew wore one treatment at one size. The
/// redesigned bars are not sayable that way. A neokitsch bar cell
/// rounds three corners at `r=3` and chamfers the fourth 10 wide by 7
/// tall; a kitsch menu foot chamfers 12 by 4; a neomil cell chamfers
/// the bottom-*left* at 6 where the era's own default chamfers the
/// bottom-right at 15. Mixed kinds, non-square cuts, and amounts the
/// widget rather than the table chooses -- three things a bool cannot
/// carry.
///
/// That the *choice* belongs at the call site was always true, and the
/// sheets said so before the bars did: the neomil info panel is
/// `M 1080 132 h 700 l 24 24 v 560 l -24 24 h -700 Z`, which cuts both
/// *right* corners, and no era-wide rule produces that. What is new is
/// that the amount can come from the call site too.
/// [`default_corners`] keeps the era's declared [`Corner`] as the
/// answer every caller took before, and still takes today.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Corners {
    pub top_left: Cut,
    pub top_right: Cut,
    pub bottom_right: Cut,
    pub bottom_left: Cut,
}

impl Corners {
    /// The same cut on all four corners.
    pub const fn all(cut: Cut) -> Corners {
        Corners {
            top_left: cut,
            top_right: cut,
            bottom_right: cut,
            bottom_left: cut,
        }
    }

    /// A plain rectangle, and the base the builders below start from.
    pub const fn square() -> Corners {
        Corners::all(Cut::Square)
    }

    pub fn with_top_left(mut self, cut: Cut) -> Corners {
        self.top_left = cut;
        self
    }

    pub fn with_top_right(mut self, cut: Cut) -> Corners {
        self.top_right = cut;
        self
    }

    pub fn with_bottom_right(mut self, cut: Cut) -> Corners {
        self.bottom_right = cut;
        self
    }

    pub fn with_bottom_left(mut self, cut: Cut) -> Corners {
        self.bottom_left = cut;
        self
    }
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
        self.corners = Corners::square();
        self
    }
}

/// The era's one declared treatment, as a [`Cut`] a corner can wear.
///
/// [`Corner`] is the era table's vocabulary and stays that: four
/// variants, one amount, no per-corner geometry. This is the bridge to
/// the widget's vocabulary, and it is total -- an era that declares a
/// chamfer hands the same cut to whichever corner a caller puts it on,
/// which is what let the since-deleted `widgets::charts` ask for "the era's
/// treatment, bottom-right only" without naming an era.
pub fn era_cut(corner: Corner) -> Cut {
    match corner {
        Corner::Square => Cut::Square,
        // Square by construction: the table carries one figure, so the
        // diagonal it describes is at 45 degrees until a widget says
        // otherwise.
        Corner::Chamfer { cut } | Corner::ClipTopRight { cut } => Cut::Chamfer { x: cut, y: cut },
        Corner::Round { radius } => Cut::Round { radius },
    }
}

/// Which corners an era treats by default. Only neo-militarism varies
/// them per widget; the rest apply their treatment uniformly.
pub fn default_corners(corner: Corner) -> Corners {
    let cut = era_cut(corner);
    match corner {
        Corner::Square => Corners::square(),
        Corner::Chamfer { .. } => Corners::square().with_bottom_right(cut),
        Corner::Round { .. } => Corners::all(cut),
        Corner::ClipTopRight { .. } => Corners::square().with_top_right(cut),
    }
}

/// The part of a canvas's box that its own geometry clip will keep.
///
/// A `canvas` hands its geometry a clip rectangle of exactly the widget
/// bounds, and `iced_wgpu` turns that into a scissor rect with
/// `Rectangle::snap`. On iced 0.13 that *truncated* -- `x` and `width`
/// both lost their fraction -- so a surface laid out at `x = 1262.5`
/// with `width = 297.5` was scissored to `[1262, 1559)`, and a 1px
/// outline whose right edge sat at 1559.5 was not dimmed by the loss,
/// it disappeared entirely.
///
/// iced 0.14 rounds both corners to the nearest whole pixel instead,
/// and `iced_wgpu`'s triangle pipeline translates the mesh by the same
/// rounding delta so the canvas's own origin lands on the scissor's --
/// which crispens every canvas hairline by up to half a pixel and is
/// the one rendering change this crate could not hold still across the
/// migration. The kept span became `round(start + len) - round(start)`.
///
/// The arithmetic below is *not* that expression, deliberately. It is
/// always less than or equal to it, so a shape built inside it is still
/// inside the scissor on 0.14; and being conservative by that half
/// pixel puts the stroke closer to where 0.13 drew it than the exact
/// span does. Measured, not assumed: on `store.neomil` the exact form
/// scores 99.939% against the golden and this one 99.991%, and on
/// `dashboard.kitsch` 99.703% against 99.739%.
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
pub fn outline(corners: Corners, ticket: Ticket, w: f32, h: f32) -> canvas::Path {
    let tl = corners.top_left.extent(w, h);
    let tr = corners.top_right.extent(w, h);
    let br = corners.bottom_right.extent(w, h);
    let bl = corners.bottom_left.extent(w, h);

    // The wedge cannot eat more than the box has, and it cannot reach
    // below the bottom edge.
    let cut = ticket.is_cut() && ticket.reach < w && ticket.drop < h;
    let (reach, drop) = if cut {
        // The wedge's point must stay above whatever the bottom-right
        // corner eats, or the outline doubles back on itself. The
        // sampled figures clear it comfortably (drop 15 against a
        // 34-high pill with a 16 radius); this is for the small box
        // some future caller hands it.
        (ticket.reach, ticket.drop.min((h - br.1).max(0.0)))
    } else {
        (0.0, 0.0)
    };
    // The body the wedge grows out of.
    let bw = w - reach;
    // Where the top edge runs: the box's top, unless the top-left is
    // a peak standing above it -- and then it starts past the peak's
    // drop, scaled as the peak itself was if the box squeezed it.
    let top = corners.top_left.brow().min(h / 2.0);
    let brow_x = brow_run(corners.top_left, tl);

    canvas::Path::new(|b| {
        // One walk for every shape: down the four edges, turning each
        // corner the way its own `Cut` says. A rounded corner is a
        // quadratic through the box corner rather than an arc --
        // visually identical at these radii and one less API surface
        // to depend on -- a chamfer is the straight line between the
        // two offsets, and a square corner is no segment at all.
        let turn = |b: &mut canvas::path::Builder,
                    cut: Cut,
                    (ex, ey): (f32, f32),
                    pivot: Point,
                    end: Point| {
            if ex <= 0.0 && ey <= 0.0 {
                return;
            }
            if cut.is_round() {
                b.quadratic_curve_to(pivot, end);
            } else {
                b.line_to(end);
            }
        };

        b.move_to(Point::new(tl.0 + brow_x, top));
        if cut {
            // The kitsch nav pill, exactly: the top edge runs the
            // body's full width with *no* top-right treatment, and the
            // wedge carries the outline out and down in its place.
            //
            //   M172 340 h158 l18 15 v13 q0 12 -12 12 h-164 ...
            b.line_to(Point::new(bw, top));
            b.line_to(Point::new(w, top + drop));
        } else {
            b.line_to(Point::new(w - tr.0, top));
            turn(
                b,
                corners.top_right,
                tr,
                Point::new(w, top),
                Point::new(w, top + tr.1),
            );
        }
        b.line_to(Point::new(w, h - br.1));
        turn(
            b,
            corners.bottom_right,
            br,
            Point::new(w, h),
            Point::new(w - br.0, h),
        );
        b.line_to(Point::new(bl.0, h));
        turn(
            b,
            corners.bottom_left,
            bl,
            Point::new(0.0, h),
            Point::new(0.0, h - bl.1),
        );
        b.line_to(Point::new(0.0, tl.1));
        if let Cut::Peak { .. } = corners.top_left {
            // Up to the peak and down onto the brow.
            b.line_to(Point::new(tl.0, 0.0));
            b.line_to(Point::new(tl.0 + brow_x, top));
        } else {
            turn(
                b,
                corners.top_left,
                tl,
                Point::new(0.0, 0.0),
                Point::new(tl.0, 0.0),
            );
        }
        b.close();
    })
}

/// How far along the top a [`Cut::Peak`]'s drop runs, scaled by the
/// same factor its rising edge was (`extent` is the scaled pair).
fn brow_run(cut: Cut, (ex, _): (f32, f32)) -> f32 {
    match cut {
        Cut::Peak { x, brow, .. } if x > 0.0 => brow.0 * ex / x,
        _ => 0.0,
    }
}

/// The shape's horizontal extent at height `y`, used to clip grain lines
/// to a surface without needing path clipping in the renderer.
///
/// Every [`Cut`] is convex and eats only its own corner, so this is
/// exact rather than an approximation -- including an asymmetric
/// chamfer, whose hypotenuse is just a steeper or shallower line. A
/// ticket wedge is convex too, and replaces the top-right corner's
/// contribution rather than adding to it.
pub fn span_at(corners: Corners, ticket: Ticket, w: f32, h: f32, y: f32) -> (f32, f32) {
    if y < 0.0 || y > h {
        return (0.0, w);
    }
    let cut = ticket.is_cut() && ticket.reach < w && ticket.drop < h;

    let (mut x0, mut x1) = (0.0f32, w);

    // Above the brow only the peak exists: its far side is the drop
    // from the peak onto the brow, and the rest of the top edge has not
    // started yet.
    let top = corners.top_left.brow().min(h / 2.0);
    if y < top && top > 0.0 {
        let tl = corners.top_left.extent(w, h);
        x1 = x1.min(tl.0 + brow_run(corners.top_left, tl) * (y / top));
    }

    // The wedge's hypotenuse: the right edge runs from the body's width
    // at the top edge out to the full width at `drop`, and is flush
    // below that.
    if cut {
        let bw = w - ticket.reach;
        x1 = x1.min(bw + ticket.reach * (y / ticket.drop).clamp(0.0, 1.0));
    }

    // How far in a corner's edge is drawn when the reading sits `d`
    // away from that corner along the vertical.
    let inward = |cut: Cut, d: f32| -> f32 {
        let (ex, ey) = cut.extent(w, h);
        if ey <= 0.0 || d >= ey {
            return 0.0;
        }
        if cut.is_round() {
            // `ex == ey == r` for a round cut, so this is the circle.
            let dy = ey - d;
            ex - (ex * ex - dy * dy).max(0.0).sqrt()
        } else {
            // The chamfer's hypotenuse, in the general case where it is
            // not at 45 degrees: full width at the corner, nothing at
            // `ey` away from it.
            ex * (1.0 - d / ey)
        }
    };

    x0 = x0.max(inward(corners.top_left, y));
    x0 = x0.max(inward(corners.bottom_left, h - y));
    // A ticket replaces the top-right treatment; applying both would
    // clip the wedge back off again.
    if !cut {
        // Measured from the brow, where a peaked shape's top edge runs.
        x1 = x1.min(w - inward(corners.top_right, (y - top).max(0.0)));
    }
    x1 = x1.min(w - inward(corners.bottom_right, h - y));
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
/// `docs/neokitsch/target-app.svg` (deleted 2026-09-03; `store-trace.svg`
/// draws card 2 solid gold across y 411..653 the same way) draws the
/// selected card as a single
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
        let (x0, x1) = span_at(corners, ticket, w, h, y);
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

        let path = outline(self.corners, self.ticket, pw, ph);

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
                        band_path(self.corners, self.ticket, pw, ph, y0, y1)
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
                    let (x0, x1) = span_at(self.corners, self.ticket, pw, ph, y);
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
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self.children[Self::CONTENT].as_widget_mut().layout(
            &mut tree.children[Self::CONTENT],
            renderer,
            limits,
        );
        let size = content.size();

        // Min and max are the same, so the background's `Length::Fill`
        // resolves to the content's size rather than to the parent's.
        let background = self.children[0].as_widget_mut().layout(
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
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((child, state), layout) in self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(state, layout, renderer, operation);
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // Topmost layer first, as `stack` does: the content is in front.
        // Capture is reported through the shell now rather than
        // returned, so stopping means asking it whether the event has
        // been taken.
        for ((child, state), layout) in self
            .children
            .iter_mut()
            .rev()
            .zip(tree.children.iter_mut().rev())
            .zip(layout.children().rev())
        {
            child.as_widget_mut().update(
                state, event, layout, cursor, renderer, clipboard, shell,
                viewport,
            );

            if shell.is_event_captured() {
                return;
            }
        }
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
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Backdrop<'a, Message>> for Element<'a, Message> {
    fn from(backdrop: Backdrop<'a, Message>) -> Self {
        Element::new(backdrop)
    }
}

#[cfg(test)]
mod tests {
    //! The shapes the redesigned bars ask for, built through the public
    //! API and read back through [`span_at`].
    //!
    //! `bar.rs` cannot wear them yet -- expressing them there is a
    //! separate change -- so this is what proves they are *sayable*
    //! today rather than a plan. Every figure comes from an
    //! IMPLEMENTATION DELTA block in `docs/<era>/bar.svg`.
    //!
    //! [`span_at`] is the right probe because it reads the same
    //! per-corner geometry [`outline`] walks: a chamfer's hypotenuse
    //! sampled at a height is exactly the point the path would put
    //! there. A test that only built a `Corners` would prove the struct
    //! compiles and nothing about the shape.

    use super::*;
    use crate::style::Corner;

    /// Canvas coordinates are `f32` and the arithmetic is a couple of
    /// multiplies deep, so compare at a tolerance a pixel could not
    /// hide in.
    #[track_caller]
    fn span_eq(got: (f32, f32), want: (f32, f32)) {
        assert!(
            (got.0 - want.0).abs() < 1e-4 && (got.1 - want.1).abs() < 1e-4,
            "span {got:?}, want {want:?}"
        );
    }

    fn span(corners: Corners, w: f32, h: f32, y: f32) -> (f32, f32) {
        span_at(corners, Ticket::default(), w, h, y)
    }

    /// neomil bar cell: chamfer on the *bottom-left* only, cut 6 on a
    /// 25px cell -- the corner the era's own default does not cut, at
    /// an amount the era table does not carry.
    #[test]
    fn neomil_bar_cell_cuts_the_bottom_left() {
        let c = Corners::square().with_bottom_left(Cut::Chamfer { x: 6.0, y: 6.0 });
        assert_eq!(c.bottom_right, Cut::Square);

        span_eq(span(c, 35.0, 25.0, 0.0), (0.0, 35.0));
        span_eq(span(c, 35.0, 25.0, 19.0), (0.0, 35.0));
        span_eq(span(c, 35.0, 25.0, 22.0), (3.0, 35.0));
        span_eq(span(c, 35.0, 25.0, 25.0), (6.0, 35.0));
    }

    /// neokitsch bar cell: rounded `r=3` on three corners and a
    /// chamfer 10 wide by 7 tall on the fourth. Mixed kinds *and* a
    /// non-square chamfer, which is the pair of things four booleans
    /// and one amount could not say.
    #[test]
    fn neokitsch_bar_cell_mixes_round_and_a_wide_chamfer() {
        let c = Corners::all(Cut::Round { radius: 3.0 })
            .with_bottom_left(Cut::Chamfer { x: 10.0, y: 7.0 });
        let (w, h) = (60.0, 25.0);

        // Top edge: the two radii bite 3 in from each end.
        span_eq(span(c, w, h, 0.0), (3.0, 57.0));
        // Below both radii and above the chamfer: the full width.
        span_eq(span(c, w, h, 3.0), (0.0, 60.0));
        // Bottom edge: the chamfer's full 10 on the left, the
        // bottom-right radius's 3 on the right.
        span_eq(span(c, w, h, h), (10.0, 57.0));
        // Halfway up the chamfer it has eaten half its *width*, not
        // half its height -- the whole point of carrying two numbers.
        span_eq(span(c, w, h, h - 3.5), (5.0, 60.0));
    }

    /// neokitsch tape and the alert plate: square but for one corner,
    /// on opposite diagonals.
    #[test]
    fn neokitsch_plates_cut_one_corner_each() {
        let tape = Corners::square().with_top_right(Cut::Chamfer { x: 10.0, y: 10.0 });
        span_eq(span(tape, 71.0, 25.0, 0.0), (0.0, 61.0));
        span_eq(span(tape, 71.0, 25.0, 5.0), (0.0, 66.0));
        span_eq(span(tape, 71.0, 25.0, 10.0), (0.0, 71.0));
        span_eq(span(tape, 71.0, 25.0, 25.0), (0.0, 71.0));

        let plate = Corners::square().with_bottom_left(Cut::Chamfer { x: 10.0, y: 7.0 });
        span_eq(span(plate, 50.0, 25.0, 0.0), (0.0, 50.0));
        span_eq(span(plate, 50.0, 25.0, 25.0), (10.0, 50.0));
    }

    /// kitsch menu foot: a chamfer 12 wide by 4 tall on the
    /// bottom-right -- shallow and broad, the opposite proportion to
    /// neokitsch's.
    #[test]
    fn kitsch_menu_foot_chamfers_wide_and_shallow() {
        let c = Corners::square().with_bottom_right(Cut::Chamfer { x: 12.0, y: 4.0 });
        let (w, h) = (160.0, 25.6);

        span_eq(span(c, w, h, h - 4.0), (0.0, 160.0));
        span_eq(span(c, w, h, h - 2.0), (0.0, 154.0));
        span_eq(span(c, w, h, h), (0.0, 148.0));
    }

    /// kitsch workspace chevron: `#chev` scaled 25/46, `M 0,25 V 13
    /// L 12,0 L 15.2,4.9 H 36.7 Q 40,4.9 40,7.6 V 13 L 28,25 Z`. The
    /// top edge runs at 4.9, not 0, and above it only the peak exists.
    #[test]
    fn kitsch_chevron_peaks_above_its_own_top_edge() {
        let c = Corners::square()
            .with_top_left(Cut::Peak {
                x: 12.0,
                y: 13.0,
                brow: (3.2, 4.9),
            })
            .with_top_right(Cut::Round { radius: 3.0 })
            .with_bottom_right(Cut::Chamfer { x: 12.0, y: 12.0 });
        let (w, h) = (40.0, 25.0);
        // The rising edge takes 13 of a 25px cell, which is more than
        // the half `extent` lets any corner eat, so the peak and its
        // brow are squeezed by 12.5/13 -- under half a pixel.
        let s = 12.5 / 13.0;
        let (px, py, bx) = (12.0 * s, 13.0 * s, 3.2 * s);
        let rise = |y: f32| px * (1.0 - y / py);

        // The peak itself: a point.
        span_eq(span(c, w, h, 0.0), (px, px));
        // Halfway down the drop: the rising edge on the left, the
        // drop on the right, and no top edge yet.
        span_eq(span(c, w, h, 2.45), (rise(2.45), px + bx / 2.0));
        // At the brow the top edge begins, and the top-right radius
        // bites from *here*, not from the box's top.
        span_eq(span(c, w, h, 4.9), (rise(4.9), 37.0));
        span_eq(span(c, w, h, 7.9), (rise(7.9), 40.0));
        // The rising edge lands; the bottom chamfer as before.
        span_eq(span(c, w, h, py), (0.0, 40.0));
        span_eq(span(c, w, h, h), (0.0, 28.0));
    }

    /// kitsch cells: round 8 on all four, and entropism: square. Both
    /// were expressible before; they have to stay so.
    #[test]
    fn kitsch_rounds_and_entropism_squares() {
        let round = Corners::all(Cut::Round { radius: 8.0 });
        let (w, h) = (100.0, 25.0);
        span_eq(span(round, w, h, 0.0), (8.0, 92.0));
        span_eq(span(round, w, h, 8.0), (0.0, 100.0));
        span_eq(span(round, w, h, h), (8.0, 92.0));
        // On the circle: 8 - sqrt(64 - 48) = 4 in, at 8 - 8*sin(60) up.
        let y = 8.0 - 8.0 * (3.0f32).sqrt() / 2.0;
        span_eq(span(round, w, h, y), (4.0, 96.0));

        let square = Corners::square();
        for y in [0.0, 1.0, 12.5, 25.0] {
            span_eq(span(square, w, h, y), (0.0, w));
        }
    }

    /// The era table's [`Corner`] still says what it always said, and
    /// [`default_corners`] puts it where it always went.
    #[test]
    fn default_corners_bridges_the_era_table() {
        assert_eq!(default_corners(Corner::Square), Corners::square());
        assert_eq!(
            default_corners(Corner::Chamfer { cut: 15.0 }),
            Corners::square().with_bottom_right(Cut::Chamfer { x: 15.0, y: 15.0 })
        );
        assert_eq!(
            default_corners(Corner::Round { radius: 16.0 }),
            Corners::all(Cut::Round { radius: 16.0 })
        );
        assert_eq!(
            default_corners(Corner::ClipTopRight { cut: 30.0 }),
            Corners::square().with_top_right(Cut::Chamfer { x: 30.0, y: 30.0 })
        );
    }

    /// A cut too big for its box shrinks in *both* directions by one
    /// factor. That is what keeps a symmetric cut symmetric under the
    /// clamp, and so what keeps this refactor pixel-identical: neomil's
    /// 15 on a 25-high cell is the old `min(15, w/2, h/2)` of 12.5 in
    /// both axes, not 15 wide by 12.5 tall.
    #[test]
    fn an_oversized_cut_keeps_its_slope() {
        let c = default_corners(Corner::Chamfer { cut: 15.0 });
        span_eq(span(c, 35.0, 25.0, 25.0), (0.0, 22.5));
        span_eq(span(c, 35.0, 25.0, 12.5), (0.0, 35.0));

        // A radius stays circular for the same reason.
        assert_eq!(Cut::Round { radius: 16.0 }.extent(60.0, 25.0), (12.5, 12.5));
    }
}
