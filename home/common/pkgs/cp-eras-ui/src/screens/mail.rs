//! The mailbox, in any era.
//!
//! Present in all four sets of design targets, and the only screen with
//! a photo-shaped trace per era: `docs/<era>/mailbox-trace.svg`. Those
//! four traces disagree about more than dress. Entropism frames its
//! list and its message in two outlined boxes; neomil boxes every row,
//! chamfers its bottom-left corner and sets a column of isometric
//! cartridge icons beside them; kitsch hangs five bare rows inside a
//! teal bracket and stacks four chevron tabs down the right where the
//! others put buttons; neokitsch rules its rows, puts the envelope on
//! the *right*, and prints the message as plain text with no panel at
//! all.
//!
//! None of that is four dressed rectangles, and none of it is an era
//! test either: it is [`crate::style::Mailbox`], a table in each era's
//! file carrying the trace's own geometry at its own 1600x900 frame,
//! its colours by palette role, and its line art as polylines. This
//! file is the single reader of that table. Nothing below asks which
//! era it is in, which is the standing test for `screens/` and not a
//! comment.
//!
//! The *content* -- subjects, senders, body copy -- is the table's too,
//! because the four traces do not agree about it. An earlier version of
//! this file said they did and kept one inbox and three lorem
//! paragraphs here; read as text, the traces say otherwise. Neomil's
//! list is "List of messages / I'm worried man / Heist data sent to
//! you / ..." with every row from Jackie, not the inbox the other three
//! show, and its panel is headed "Urgent Information (!)", which is no
//! row of that list. Entropism reads a message it has not selected and
//! heads it "from: Mom" over a list that says "FROM: MOM". Kitsch and
//! neokitsch split the lorem three ways with no "Nemo enim" paragraph;
//! entropism and neomil keep it. And every trace sets each body line
//! explicitly, hyphenating where it breaks ("incidi-" / "dunt"), so
//! there is nothing to wrap: [`crate::style::MailList::rows`] and
//! [`crate::style::MailPanel::paragraphs`] carry the text verbatim and
//! this file draws one run per entry. What is still the screen's is
//! casing -- an era that shouts its subjects stores them in sentence
//! case and `title_upper` / `from_upper` say so -- and what happens on
//! a click, when the panel leaves the trace's resting message and reads
//! the clicked row instead.
//!
//! Drawn as one canvas rather than composed out of layout, for the same
//! reason [`crate::screens::dashboard`]'s trace-shaped arms are: a
//! trace measures absolute coordinates, and the gate that judges this
//! screen (`scripts/fidelity_check.sh --implementation <era> mailbox`)
//! matches shapes by bounding box. Layout that lands a frame two pixels
//! out is layout that fails a gate the trace passes.
//!
//! Motion. A trace's boot-in `<animate>`s are on the table too, as
//! [`crate::style::MailMotion`]s ([`crate::style::Mailbox::motions`]): each
//! names the parts of the sheet it moves, and `draw` paints every
//! region under the clip its motions have come to and the alpha they
//! have faded to, off the screen's own clock. The rest frame is the
//! sheet as it was before any of it: a clip at its full rect and an
//! alpha of 1 draw exactly what an unclipped, unfaded region did.
//! Two parts are finer than a region because the traces disagree on
//! where they ride: the panel's heading ([`MailPart::Title`], its own
//! step after the panel, since neomil's stands above the panel's
//! clip) and the selected row's printing ([`MailPart::Printing`],
//! drawn inside the list under a second cover, since neokitsch's
//! fades in with the bar). An era that names neither gets them where
//! the region draws them.

use crate::motion;
use crate::style::{
    FromAt, Ink, MailBadges, MailButtons, MailList, MailPanel, MailPart, Piece, RowDecor, Run,
    Seg, Style, Ticket, Trim, BL, BR, TL, TR,
};
use crate::widgets::surface::{outline, Corners, Cut};
use crate::screens::nav::{Dir, Stroke};
use crate::screens::scene::{blend_over, Backdrop};
use crate::widgets::ground;
use crate::Element;
use iced::widget::{canvas, stack, Action};
use iced::{mouse, Color, Event, Length, Point, Rectangle, Renderer, Size, Subscription, Vector};
use std::cell::Cell;
use std::time::{Duration, Instant};

/// The frame every trace measures in.
const DW: f32 = 1600.0;
const DH: f32 = 900.0;

/// Where a Rajdhani baseline sits below the top of its line box, as a
/// fraction of the font size. The traces position text by baseline and
/// canvas text by the top of its line box, so every run converts
/// through this. It carries iced's own default line height as well as
/// the face's ascent, which is why it is a measured constant rather
/// than a font metric.
const BASELINE: f32 = 0.95;

pub struct MailBox {
    pub style: Style,
    /// Which row is picked out. Starts on the row the era's trace
    /// selects and moves with a click, so the screen is a design target
    /// *and* a working list rather than a poster of one.
    selected: usize,
    /// Which message the panel is reading. Starts where the trace puts
    /// it -- which is not always the selected row: entropism selects
    /// row 0 and reads row 1 -- and follows the selection thereafter.
    showing: usize,
    /// The screen's t = 0: the process origin, or the moment the hub
    /// opened it (`motion::onset`, [`MailBox::enter`]).
    origin: Instant,
    /// The moment the sheet is drawn at, for the era's `MailMotion`s.
    /// Advanced by [`Message::Tick`] while the boot-in runs, then left
    /// where it is.
    now: Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// A row was clicked. Carries the row's index in the visible list.
    Select(usize),
    /// A key moved the selection: `j`/`k` walk the list, and the list
    /// is the only thing here to walk, so `h`/`l` do nothing.
    Move(Dir),
    /// The clock, while the boot-in runs.
    Tick(Instant),
}

impl crate::shell::Wears for MailBox {
    fn wears(&self) -> Style {
        self.style
    }
}

impl MailBox {
    pub fn new(style: Style) -> Self {
        let selected = style.mailbox.list.selected;
        let showing = style.mailbox.panel.message;
        MailBox {
            style,
            selected,
            showing,
            origin: motion::origin(),
            now: motion::now(),
        }
    }

    pub fn title(&self) -> String {
        format!("MAIL BOX — {}", self.style.era.name())
    }

    /// The screen is coming up: start its clock here, so its boot-in
    /// plays from now (`motion`, the module note). Under a pinned
    /// clock this changes nothing.
    pub fn enter(&mut self) {
        self.origin = motion::onset();
        self.now = motion::now();
    }

    /// Where the sheet's clock is, counted from the screen's origin.
    pub(crate) fn at(&self) -> Duration {
        self.now.saturating_duration_since(self.origin)
    }

    /// A redraw every frame until the sheet is at rest, and none when
    /// the clock is pinned: as `Dashboard::subscription`. An era whose
    /// mailbox has no motion never asks for one.
    pub fn subscription(&self) -> Subscription<Message> {
        if motion::frozen() || self.style.mailbox.motions.is_empty() || self.at() >= motion::REST {
            return Subscription::none();
        }
        iced::time::every(Duration::from_millis(16)).map(Message::Tick)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick(at) => self.now = at,
            Message::Select(row) => {
                self.selected = row.min(self.style.mailbox.list.rows.len().saturating_sub(1));
                self.showing = self.selected;
            }
            Message::Move(Dir::Down) => self.update(Message::Select(self.selected + 1)),
            Message::Move(Dir::Up) => self.update(Message::Select(self.selected.saturating_sub(1))),
            Message::Move(Dir::Left | Dir::Right) => {}
        }
    }

    /// The keyboard's part in this screen: moves. Enter and Esc are the
    /// hub's, so on its own the mailbox drops them.
    pub fn stroke(stroke: Stroke) -> Option<Message> {
        match stroke {
            Stroke::Move(dir) => Some(Message::Move(dir)),
            Stroke::Open | Stroke::Back => None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sheet = canvas(Sheet {
            style: &self.style,
            selected: self.selected,
            showing: self.showing,
            at: self.at(),
            alpha: Cell::new(1.0),
        })
        .width(Length::Fill)
        .height(Length::Fill);

        // An era whose mailbox trace measures its own ground composites
        // it from `backdrop`; the rest take the era's `Ground`. Stacking
        // two would double the bloom.
        let m = &self.style.mailbox;
        if m.backdrop.is_empty() {
            stack![ground(&self.style), sheet].into()
        } else {
            let backdrop = canvas(Backdrop { style: self.style, prims: m.backdrop, stretch: true, at: self.at() })
                .width(Length::Fill)
                .height(Length::Fill);
            stack![backdrop, sheet].into()
        }
    }
}

/// The whole screen, drawn from the era's [`crate::style::Mailbox`].
struct Sheet<'a> {
    style: &'a Style,
    selected: usize,
    showing: usize,
    /// The moment to draw at, counted from the screen's origin: what
    /// the era's `MailMotion`s are read at.
    at: Duration,
    /// The alpha the region being drawn has faded to, 1 outside any
    /// motion. Set by [`Sheet::under`] around each region, read by
    /// [`Sheet::paint`] on the way to every ink -- a `Cell` because the
    /// drawing methods take `&self` and a region's fills fade under
    /// their own cover inside it.
    alpha: Cell<f32>,
}

/// What the motions naming a part of the sheet have come to: the clip
/// they intersect to, in design coordinates (`None` for no clip at
/// all), and the alpha they multiply to.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cover {
    clip: Option<crate::style::Frame>,
    alpha: f32,
}

impl Cover {
    const OPEN: Cover = Cover { clip: None, alpha: 1.0 };

    /// Both covers at once: the clips intersected -- `with_clip`
    /// drafts a fresh frame, so a nested clip does not intersect with
    /// its outer one by itself -- and the alphas multiplied.
    fn and(self, other: Cover) -> Cover {
        let clip = match (self.clip, other.clip) {
            (None, c) | (c, None) => c,
            (Some(a), Some(b)) => {
                let x0 = a.x.max(b.x);
                let y0 = a.y.max(b.y);
                let x1 = (a.x + a.w).min(b.x + b.w);
                let y1 = (a.y + a.h).min(b.y + b.h);
                Some(crate::style::Frame::new(x0, y0, (x1 - x0).max(0.0), (y1 - y0).max(0.0)))
            }
        };
        Cover { clip, alpha: self.alpha * other.alpha }
    }

    /// Whether anything under this cover can show at all.
    fn shown(&self) -> bool {
        self.alpha > 0.0 && self.clip.map_or(true, |c| c.w > 0.0 && c.h > 0.0)
    }
}

/// The style and the alpha every ink is resolved through: what the
/// drawing functions below take where they took a `&Style`.
#[derive(Clone, Copy)]
struct Paint<'a> {
    style: &'a Style,
    alpha: f32,
}

impl Sheet<'_> {
    /// The inks as the region being drawn wants them.
    fn paint(&self) -> Paint<'_> {
        Paint { style: self.style, alpha: self.alpha.get() }
    }

    /// The cover the era's motions put over `part` at this moment.
    fn cover(&self, part: MailPart) -> Cover {
        self.style
            .mailbox
            .motions
            .iter()
            .filter(|m| m.parts.contains(&part))
            .fold(Cover::OPEN, |cover, m| {
                let t = motion::progress(&m.motion, self.at);
                cover.and(match m.motion.change {
                    crate::style::Change::Clip { x, y, w, h } => Cover {
                        clip: Some(crate::style::Frame::new(
                            x,
                            y,
                            crate::style::Change::lerp(w, t),
                            crate::style::Change::lerp(h, t),
                        )),
                        alpha: 1.0,
                    },
                    crate::style::Change::Opacity { alpha } => Cover {
                        clip: None,
                        alpha: crate::style::Change::lerp(alpha, t),
                    },
                })
            })
    }

    /// Draw something under a cover: clipped to it, and with its alpha
    /// on every ink, folded into the alpha already in force. Nothing is
    /// drawn under a cover nothing can show through. The clip maps to
    /// the canvas axis by axis, as the sheet does.
    ///
    /// Every call drafts a frame, clipped or not. iced's `with_clip`
    /// pastes the draft's meshes into the frame *ahead of* everything
    /// the frame drew directly -- a frame's own geometry is only
    /// batched when it is finished -- so a clipped region would land
    /// under chrome drawn before it, and its inner edge would show the
    /// outline it was meant to cover (the entropism REPORT SPAM ring,
    /// 2026-09-07). Drafting every region keeps the sheet's order the
    /// paint order: each is pasted as it is drawn and the frame's own
    /// buffer stays empty. A draft nested in a draft is pasted the same
    /// way, under its parent's own drawing, which is why [`Sheet::fill`]
    /// drafts only when it has a clip of its own.
    fn under(&self, frame: &mut canvas::Frame, scale: Scale, cover: Cover, draw: impl FnOnce(&Self, &mut canvas::Frame)) {
        if !cover.shown() {
            return;
        }
        let was = self.alpha.replace(self.alpha.get() * cover.alpha);
        let region = match cover.clip {
            Some(c) => Rectangle::new(scale.point(c.x, c.y), scale.size(c.w, c.h)),
            None => Rectangle::with_size(frame.size()),
        };
        frame.with_clip(region, |f| draw(self, f));
        self.alpha.set(was);
    }

    /// Draw part of a region under a second cover, `part`'s, on top of
    /// the one already in force. Called from inside the region's own
    /// draw; a cover with a clip drafts a frame of its own (see
    /// [`Sheet::under`] for what that does to the order), one with
    /// only an alpha does not.
    fn also(&self, frame: &mut canvas::Frame, scale: Scale, part: MailPart, draw: impl FnOnce(&Self, &mut canvas::Frame)) {
        let cover = self.cover(part);
        match cover.clip {
            Some(_) => self.under(frame, scale, cover, draw),
            None if cover.shown() => {
                let was = self.alpha.replace(self.alpha.get() * cover.alpha);
                draw(self, frame);
                self.alpha.set(was);
            }
            None => {}
        }
    }

    /// One of a region's reverse-video fills, under the
    /// [`MailPart::Fills`] cover.
    fn fill(&self, frame: &mut canvas::Frame, scale: Scale, draw: impl FnOnce(&Self, &mut canvas::Frame)) {
        self.also(frame, scale, MailPart::Fills, draw);
    }
}

impl Sheet<'_> {
    /// One row's box in design coordinates. The hit target and the
    /// drawn plate are the same rectangle by construction, which is the
    /// point of keeping the geometry in the table rather than in the
    /// layout.
    fn row_at(&self, i: usize) -> crate::style::Frame {
        let list = &self.style.mailbox.list;
        list.row.shifted(0.0, i as f32 * list.pitch)
    }

    /// How far the selection has moved from where the table parks it.
    fn sel_offset(&self) -> f32 {
        let list = &self.style.mailbox.list;
        (self.selected as f32 - list.selected as f32) * list.pitch
    }

    /// Which row a cursor position lands on, in the canvas's own
    /// coordinates. The selected row is tested against its own plate,
    /// which in two eras is wider than the band the other rows take.
    fn hit(&self, at: Point, bounds: Rectangle) -> Option<usize> {
        let list = &self.style.mailbox.list;
        let (sx, sy) = (bounds.width / DW, bounds.height / DH);
        let inside = |f: crate::style::Frame| {
            at.x >= f.x * sx
                && at.x <= (f.x + f.w) * sx
                && at.y >= f.y * sy
                && at.y <= (f.y + f.h) * sy
        };
        (0..list.rows.len()).find(|&i| {
            let band = self.row_at(i);
            inside(band)
                || (i == self.selected && inside(list.sel.shifted(0.0, self.sel_offset())))
        })
    }
}

/// Design coordinates to device coordinates.
#[derive(Debug, Clone, Copy)]
struct Scale {
    sx: f32,
    sy: f32,
}

impl Scale {
    fn point(self, x: f32, y: f32) -> Point {
        Point::new(x * self.sx, y * self.sy)
    }

    fn size(self, w: f32, h: f32) -> Size {
        Size::new(w * self.sx, h * self.sy)
    }

    /// A length along the smaller axis: stroke widths and type sizes,
    /// which must not stretch when the window is not 16:9.
    fn len(self, v: f32) -> f32 {
        v * self.sx.min(self.sy)
    }
}

/// A role's colour, faded as the region being drawn is: the fade is a
/// `fill-opacity` on the ink, rebased for wgpu's linear blend the way
/// `scene::Scene::ink` rebases one (`blend_over`), so the fade lands
/// the trace's pixel over the era's ground. Untouched at 1.
fn ink(s: Paint, role: Ink) -> Color {
    let c = role.of(&s.style.palette);
    if s.alpha >= 1.0 {
        c
    } else {
        blend_over(Color { a: c.a * s.alpha, ..c }, s.style.palette.bg)
    }
}

/// A [`Trim`] as the corner set [`outline`] walks.
fn corners(trim: Trim, scale: Scale) -> Corners {
    if trim.corners == 0 || trim.cut <= 0.0 {
        return Corners::square();
    }
    let cut = if trim.round {
        Cut::Round {
            radius: scale.len(trim.cut),
        }
    } else {
        Cut::Chamfer {
            x: trim.cut * scale.sx,
            y: trim.cut * scale.sy,
        }
    };
    let mut c = Corners::square();
    if trim.corners & TL != 0 {
        c = c.with_top_left(cut);
    }
    if trim.corners & TR != 0 {
        c = c.with_top_right(cut);
    }
    if trim.corners & BR != 0 {
        c = c.with_bottom_right(cut);
    }
    if trim.corners & BL != 0 {
        c = c.with_bottom_left(cut);
    }
    c
}

/// Fill and/or stroke one box of the table, corners and all.
fn box_at(
    frame: &mut canvas::Frame,
    scale: Scale,
    at: crate::style::Frame,
    trim: Trim,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
) {
    let size = scale.size(at.w, at.h);
    if size.width <= 0.0 || size.height <= 0.0 {
        return;
    }
    let path = outline(
        corners(trim, scale),
        Ticket::default(),
        size.width,
        size.height,
    );
    frame.with_save(|f| {
        f.translate(Vector::new(at.x * scale.sx, at.y * scale.sy));
        if let Some(color) = fill {
            f.fill(&path, color);
        }
        if let Some((color, width)) = stroke {
            f.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(scale.len(width)),
            );
        }
    });
}

/// A polyline in design coordinates: the era's line art.
fn poly_at(
    frame: &mut canvas::Frame,
    scale: Scale,
    points: &[(f32, f32)],
    close: bool,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
) {
    if points.len() < 2 {
        return;
    }
    let path = canvas::Path::new(|b| {
        b.move_to(scale.point(points[0].0, points[0].1));
        for (x, y) in &points[1..] {
            b.line_to(scale.point(*x, *y));
        }
        if close {
            b.close();
        }
    });
    if let Some(color) = fill {
        frame.fill(&path, color);
    }
    if let Some((color, width)) = stroke {
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(color)
                .with_width(scale.len(width))
                .with_line_join(canvas::LineJoin::Round),
        );
    }
}

/// A curved outline in design coordinates.
///
/// The straight-line half of the era's art goes through [`poly_at`];
/// this is for the runs the trace actually curves, transcribed segment
/// by segment rather than sampled -- a polyline through a cubic's
/// endpoints turns a corner the material eases through.
fn curve_at(
    frame: &mut canvas::Frame,
    scale: Scale,
    start: (f32, f32),
    steps: &[Seg],
    close: bool,
    fill: Option<Color>,
    stroke: Option<(Color, f32)>,
) {
    if steps.is_empty() {
        return;
    }
    let path = canvas::Path::new(|b| {
        b.move_to(scale.point(start.0, start.1));
        for step in steps {
            match *step {
                Seg::Move(x, y) => b.move_to(scale.point(x, y)),
                Seg::Line(x, y) => b.line_to(scale.point(x, y)),
                Seg::Cubic { c1x, c1y, c2x, c2y, x, y } => b.bezier_curve_to(
                    scale.point(c1x, c1y),
                    scale.point(c2x, c2y),
                    scale.point(x, y),
                ),
                Seg::Quad { cx, cy, x, y } => {
                    b.quadratic_curve_to(scale.point(cx, cy), scale.point(x, y))
                }
            }
        }
        if close {
            b.close();
        }
    });
    if let Some(color) = fill {
        frame.fill(&path, color);
    }
    if let Some((color, width)) = stroke {
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(color)
                .with_width(scale.len(width))
                .with_line_join(canvas::LineJoin::Round),
        );
    }
}

/// One run of text, positioned by the baseline the trace gives.
fn label(frame: &mut canvas::Frame, scale: Scale, at: Run, color: Color, content: &str) {
    if content.is_empty() || at.size <= 0.0 {
        return;
    }
    let size = scale.len(at.size);
    frame.fill_text(canvas::Text {
        content: content.to_string(),
        position: Point::new(at.x * scale.sx, at.y * scale.sy - size * BASELINE),
        color,
        size: size.into(),
        font: if at.bold {
            crate::fonts::FONT_RAJDHANI_BOLD
        } else if at.semibold {
            crate::fonts::FONT_RAJDHANI_SEMIBOLD
        } else if at.medium {
            crate::fonts::FONT_RAJDHANI_MEDIUM
        } else {
            crate::fonts::FONT_RAJDHANI_REGULAR
        },
        align_x: if at.center {
            iced::advanced::text::Alignment::Center
        } else if at.right {
            iced::advanced::text::Alignment::Right
        } else {
            iced::advanced::text::Alignment::Left
        },
        ..Default::default()
    });
}

/// The envelope beside a row, drawn rather than set -- Rajdhani has no
/// U+2709 and neither does any era's UI face.
///
/// Both variants come off the traces: the closed one is a rect with the
/// flap folded down as a V and two back-fold lines; the open one raises
/// the flap into a diamond peaking above the body's top edge.
fn envelope(
    frame: &mut canvas::Frame,
    scale: Scale,
    x: f32,
    y: f32,
    w: f32,
    open: bool,
    color: Color,
    width: f32,
) {
    if w <= 0.0 {
        return;
    }
    let h = w * 0.65;
    let stroke = canvas::Stroke::default()
        .with_color(color)
        .with_width(scale.len(width))
        .with_line_join(canvas::LineJoin::Round);
    let path = canvas::Path::new(|b| {
        if open {
            let top = y + h * 0.42;
            b.move_to(scale.point(x, top));
            b.line_to(scale.point(x + w, top));
            b.line_to(scale.point(x + w, top + h));
            b.line_to(scale.point(x, top + h));
            b.close();
            b.move_to(scale.point(x, top));
            b.line_to(scale.point(x + w / 2.0, y));
            b.line_to(scale.point(x + w, top));
            b.line_to(scale.point(x + w / 2.0, top + h * 0.55));
            b.close();
        } else {
            b.move_to(scale.point(x, y));
            b.line_to(scale.point(x + w, y));
            b.line_to(scale.point(x + w, y + h));
            b.line_to(scale.point(x, y + h));
            b.close();
            b.move_to(scale.point(x, y));
            b.line_to(scale.point(x + w / 2.0, y + h * 0.68));
            b.line_to(scale.point(x + w, y));
            b.move_to(scale.point(x, y + h));
            b.line_to(scale.point(x + w * 0.32, y + h * 0.44));
            b.move_to(scale.point(x + w, y + h));
            b.line_to(scale.point(x + w * 0.68, y + h * 0.44));
        }
    });
    frame.stroke(&path, stroke);
}

fn cased(content: &str, upper: bool) -> String {
    if upper {
        content.to_uppercase()
    } else {
        content.to_string()
    }
}

impl Sheet<'_> {
    /// Region A: the list frame, its rows, their glyphs and their text.
    fn list(&self, frame: &mut canvas::Frame, scale: Scale, list: &MailList) {
        let s = self.paint();

        if let Some(at) = list.frame {
            box_at(
                frame,
                scale,
                at,
                Trim::NONE,
                None,
                Some((ink(s, list.frame_ink), list.frame_width)),
            );
        }

        if let Some(icons) = list.icons {
            for i in 0..list.rows.len() {
                self.cartridge(
                    frame,
                    scale,
                    icons.x,
                    icons.y + i as f32 * icons.pitch,
                    i == self.selected,
                );
            }
        }

        for (i, mail) in list.rows.iter().enumerate() {
            let row = self.row_at(i);
            let selected = i == self.selected;
            let shift = self.sel_offset();

            if selected {
                self.fill(frame, scale, |me, f| {
                    if let Some(cell) = list.sel_icon {
                        box_at(
                            f,
                            scale,
                            cell.shifted(0.0, shift),
                            list.sel_icon_trim,
                            Some(ink(me.paint(), list.sel_fill)),
                            None,
                        );
                    }
                    me.selection(f, scale, list, shift);
                });
                // What the row prints -- [`MailPart::Printing`], for the
                // era whose bar fades in with its ink rather than
                // lighting under it.
                self.also(frame, scale, MailPart::Printing, |me, f| {
                    let s = me.paint();
                    if let Some(n) = list.sel_notch.map(|n| n.shifted(0.0, shift)) {
                        // The tab motif inverted: a dark trapezoid cut up
                        // into the bar's bottom edge, outlined in the ink.
                        poly_at(
                            f,
                            scale,
                            &[
                                (n.x, n.y + n.h),
                                (n.x + n.h * 0.55, n.y),
                                (n.x + n.w - n.h * 0.55, n.y),
                                (n.x + n.w, n.y + n.h),
                            ],
                            true,
                            Some(ink(s, Ink::Bg)),
                            Some((ink(s, list.rule_ink), 1.0)),
                        );
                    }
                    me.printing(f, scale, list, mail, row, true);
                });
            } else {
                if list.decor == RowDecor::Boxed {
                    box_at(
                        frame,
                        scale,
                        row,
                        list.row_trim,
                        list.row_fill.map(|r| ink(s, r)),
                        list.row_stroke.map(|r| (ink(s, r), list.row_width)),
                    );
                }
                if let Some(spine) = list.spine {
                    box_at(
                        frame,
                        scale,
                        spine.shifted(row.x, row.y),
                        Trim::NONE,
                        Some(ink(s, list.rule_ink)),
                        None,
                    );
                }
            }

            // The divider at a row's foot, which the selected row does
            // not take: its own fill is the edge there. Entropism draws
            // them inside one frame, neokitsch as free hairlines with a
            // small filled tab riding each.
            if !selected {
                if let Some(rule) = list.rule {
                    let at = rule.shifted(row.x, row.y);
                    box_at(frame, scale, at, Trim::NONE, Some(ink(s, list.rule_ink)), None);
                    if let Some(tab) = list.tab {
                        let t = tab.shifted(row.x, row.y);
                        poly_at(
                            frame,
                            scale,
                            &[
                                (t.x, t.y + t.h),
                                (t.x + t.h * 0.7, t.y),
                                (t.x + t.w - t.h * 0.7, t.y),
                                (t.x + t.w, t.y + t.h),
                            ],
                            true,
                            Some(ink(s, list.tab_ink)),
                            None,
                        );
                    }
                }
            }

            if !selected {
                self.printing(frame, scale, list, mail, row, false);
            }
        }
    }

    /// One row's printing: the envelope, the subject, the sender and
    /// the NEW pill the era marks unread rows with. The selected row's
    /// is drawn under [`MailPart::Printing`], from [`Sheet::list`].
    fn printing(&self, frame: &mut canvas::Frame, scale: Scale, list: &MailList, mail: &crate::style::Mail, row: crate::style::Frame, selected: bool) {
        let s = self.paint();
        let shift = self.sel_offset();
        let title_ink = if selected { Ink::OnSelect } else { Ink::Fg };
        // A selected row's sender is dark *only where it sits on
        // the selection*. Kitsch's bar ends above its own from-line
        // and the trace sets that line in the bright yellow, so the
        // rule is geometric rather than another table field.
        let on_fill = selected && row.y + list.from_dy <= list.sel.y + shift + list.sel.h;
        let from_ink = match (selected, on_fill) {
            (true, true) => Ink::OnSelect,
            (true, false) => Ink::Select,
            _ => Ink::Mid,
        };

        envelope(
            frame,
            scale,
            list.glyph_x,
            row.y + list.glyph_dy,
            list.glyph_w,
            mail.unread,
            ink(s, title_ink),
            1.2,
        );

        let title = Run {
            bold: list.title_bold,
            ..Run::new(
                list.text_x,
                row.y + list.title_dy,
                list.title_size,
                title_ink,
            )
        };
        label(
            frame,
            scale,
            title,
            ink(s, title_ink),
            &cased(mail.subject, list.title_upper),
        );

        let sender = format!("{}{}", list.from_prefix, cased(mail.from, list.from_upper));
        let at = match list.from_at {
            FromAt::Beneath => Run::new(
                list.text_x,
                row.y + list.from_dy,
                list.from_size,
                from_ink,
            ),
            // The one era that sets the sender as a second column,
            // right-aligned on the subject's own line: neomil's
            // trace anchors every name's end 7px inside the row's
            // right edge (x 504 on rows x 241..511).
            FromAt::Trailing => Run::new(
                row.x + row.w - 7.0,
                row.y + list.title_dy,
                list.from_size,
                from_ink,
            )
            .right(),
        };
        label(frame, scale, at, ink(s, from_ink), &sender);

        // The NEW pill, on the rows the trace puts one on -- its
        // unread ones, in the era that marks them this way.
        if let Some(pill) = list.new_pill {
            if mail.unread {
                let at = pill.shifted(row.x, row.y);
                box_at(
                    frame,
                    scale,
                    at,
                    Trim::round(TL | TR | BR | BL, 4.0),
                    None,
                    Some((ink(s, title_ink), 1.5)),
                );
                label(
                    frame,
                    scale,
                    Run::new(at.x + at.w / 2.0, at.y + at.h - 3.0, 10.0, title_ink)
                        .bold()
                        .centered(),
                    ink(s, title_ink),
                    "NEW",
                );
            }
        }
    }

    /// The selection fill, in whatever the era means by selection --
    /// [`crate::widgets::surface::Surface::selected`]'s decision, taken
    /// on a canvas this screen already owns.
    fn selection(&self, frame: &mut canvas::Frame, scale: Scale, list: &MailList, shift: f32) {
        let s = self.paint();
        let at = list.sel.shifted(0.0, shift);
        let fill = match list.veneer {
            Some(v) => v.base,
            None => ink(s, list.sel_fill),
        };
        box_at(frame, scale, at, list.sel_trim, Some(fill), None);

        // The grain, drawn at the measured pitch, width and contrast
        // rather than blended into the base. `Surface` clips its own
        // grain with `span_at`; here the shape is one box in design
        // coordinates, so each line is clamped to the bar's span at its
        // own height -- which for this era is the top-right chamfer.
        let Some(v) = list.veneer else { return };
        let mut y = at.y + v.pitch / 2.0;
        while y < at.y + at.h {
            let d = y - at.y;
            let inset = if list.sel_trim.corners & TR != 0 && d < list.sel_trim.cut {
                list.sel_trim.cut - d
            } else {
                0.0
            };
            // The zigzag: a vertex every `turn`, alternating the sway
            // about the line's own height, which is what gives the
            // plank its book-matched chevron -- and the period the
            // extractor splits the bar on.
            let right = at.x + at.w - inset;
            let line = canvas::Path::new(|b| {
                b.move_to(scale.point(at.x, y + v.sway / 2.0));
                let mut x = v.phase;
                while x < at.x {
                    x += v.turn;
                }
                let mut up = true;
                while x < right {
                    b.line_to(scale.point(x, y + if up { -v.sway / 2.0 } else { v.sway / 2.0 }));
                    up = !up;
                    x += v.turn;
                }
                b.line_to(scale.point(right, y + if up { -v.sway / 2.0 } else { v.sway / 2.0 }));
            });
            frame.stroke(
                &line,
                canvas::Stroke::default()
                    .with_color(v.grain)
                    .with_width(scale.len(v.width)),
            );
            y += v.pitch;
        }
    }

    /// Neomil's isometric disc cartridge, one per row. Geometry off
    /// `docs/neomil/mailbox-trace.svg`'s `#cart` / `#tile` / `#disc`,
    /// relative to the icon's top vertex.
    fn cartridge(&self, frame: &mut canvas::Frame, scale: Scale, x: f32, y: f32, selected: bool) {
        let s = self.paint();
        let shift = |p: &[(f32, f32)]| -> Vec<(f32, f32)> {
            p.iter().map(|(a, b)| (x + a, y + b)).collect()
        };
        let (shell, face) = if selected {
            (Some(ink(s, Ink::Select)), ink(s, Ink::Border))
        } else {
            (None, ink(s, Ink::Select))
        };
        poly_at(
            frame,
            scale,
            &shift(&[(0.0, 0.0), (37.0, 14.0), (-15.0, 54.0), (-52.0, 41.0)]),
            true,
            shell,
            Some((ink(s, Ink::Dim), 2.0)),
        );
        poly_at(
            frame,
            scale,
            &shift(&[(-26.0, 28.0), (20.0, 37.0), (-7.0, 52.0), (-41.0, 41.0)]),
            true,
            Some(face),
            None,
        );
        let ellipse = |rx: f32, ry: f32| {
            canvas::Path::new(|b| {
                b.ellipse(canvas::path::arc::Elliptical {
                    center: scale.point(x + 6.0, y + 23.0),
                    radii: Vector::new(rx * scale.sx, ry * scale.sy),
                    rotation: iced::Radians(-0.35),
                    start_angle: iced::Radians(0.0),
                    end_angle: iced::Radians(std::f32::consts::TAU),
                });
            })
        };
        frame.fill(&ellipse(18.0, 12.0), face);
        frame.stroke(
            &ellipse(9.0, 6.0),
            canvas::Stroke::default()
                .with_color(if selected {
                    ink(s, Ink::Select)
                } else {
                    ink(s, Ink::Border)
                })
                .with_width(scale.len(1.5)),
        );
    }

    /// Region B: the message.
    fn panel(&self, frame: &mut canvas::Frame, scale: Scale, panel: &MailPanel, list: &MailList) {
        let s = self.paint();
        if let Some(at) = panel.frame {
            box_at(
                frame,
                scale,
                at,
                panel.frame_trim,
                panel.frame_fill.map(|r| ink(s, r)),
                panel.frame_stroke.map(|r| (ink(s, r), panel.frame_width)),
            );
        }
        if let Some(at) = panel.head {
            self.fill(frame, scale, |me, f| {
                box_at(f, scale, at, panel.head_trim, Some(ink(me.paint(), panel.head_ink)), None);
            });
        }

        if list.rows.is_empty() {
            return;
        }

        // One run per line the trace sets; nothing is wrapped here.
        let mut y = panel.body.y;
        for para in panel.paragraphs {
            for line in *para {
                label(
                    frame,
                    scale,
                    Run { y, ..panel.body },
                    ink(s, panel.body.ink),
                    line,
                );
                y += panel.line;
            }
            y += panel.para - panel.line;
        }
    }

    /// The panel's heading and sender line: [`MailPart::Title`], drawn
    /// after the panel so an era whose heading stands outside the
    /// panel's clip (neomil's, above `#message-open`) can leave it out
    /// of that motion.
    fn title(&self, frame: &mut canvas::Frame, scale: Scale, panel: &MailPanel, list: &MailList) {
        let s = self.paint();
        // At rest the panel says what the trace says, which two eras
        // pin explicitly; once a click has moved it off `message` the
        // heading and sender are the shown row's own.
        let Some(mail) = list.rows.get(self.showing).or(list.rows.last()) else {
            return;
        };
        let at_rest = self.showing == panel.message;
        let heading = match panel.heading.filter(|_| at_rest) {
            Some(text) => text.to_string(),
            None => cased(mail.subject, panel.title_upper),
        };
        label(frame, scale, panel.title, ink(s, panel.title.ink), &heading);
        if let Some(at) = panel.from {
            let sender = match panel.sender.filter(|_| at_rest) {
                Some(text) => text.to_string(),
                None => format!("{}{}", list.from_prefix, cased(mail.from, list.from_upper)),
            };
            label(frame, scale, at, ink(s, at.ink), &sender);
        }
    }

    /// Region C: the action buttons, or the chevron tabs an era stacks
    /// down the right where they would go.
    fn buttons(&self, frame: &mut canvas::Frame, scale: Scale, b: &MailButtons) {
        let s = self.paint();
        if b.count == 0 {
            return;
        }

        // Entropism's four are one outlined strip with two dividers,
        // not four boxes -- which is what its `#btn-chrome` path says.
        if b.joined {
            box_at(
                frame,
                scale,
                crate::style::Frame::new(b.first.x, b.first.y, b.dx * b.count as f32, b.first.h),
                b.trim,
                None,
                Some((ink(s, b.stroke), b.width)),
            );
            for i in 1..b.count {
                box_at(
                    frame,
                    scale,
                    crate::style::Frame::new(
                        b.first.x + b.dx * i as f32,
                        b.first.y,
                        b.width,
                        b.first.h,
                    ),
                    Trim::NONE,
                    Some(ink(s, b.stroke)),
                    None,
                );
            }
        }

        for i in 0..b.count {
            let at = b.first.shifted(b.dx * i as f32, b.dy * i as f32);
            let filled = b.filled == Some(i);
            if filled {
                // The reverse-video button is a fill the eras light
                // separately (`MailPart::Fills`).
                self.fill(frame, scale, |me, f| {
                    let fill = ink(me.paint(), b.fill);
                    if b.chevron {
                        me.chevron(f, scale, at, Some(fill), b.width);
                    } else {
                        box_at(f, scale, at, b.trim, Some(fill), None);
                    }
                });
            } else if b.chevron {
                self.chevron(frame, scale, at, None, b.width);
            } else if !b.joined {
                let fill = b.idle_fill.map(|i| ink(s, i));
                box_at(frame, scale, at, b.trim, fill, Some((ink(s, b.stroke), b.width)));
            }
            if let Some(tab) = b.tab {
                let t = tab.shifted(at.x, at.y);
                poly_at(
                    frame,
                    scale,
                    &[
                        (t.x, t.y + t.h),
                        (t.x + t.h * 0.5, t.y),
                        (t.x + t.w - t.h * 0.5, t.y),
                        (t.x + t.w, t.y + t.h),
                    ],
                    true,
                    Some(ink(s, Ink::Alert)),
                    None,
                );
            }
            let role = if filled { Ink::OnSelect } else { b.label.ink };
            label(
                frame,
                scale,
                Run {
                    x: at.x + b.label.x,
                    y: at.y + b.label.y,
                    ink: role,
                    ..b.label
                },
                ink(s, role),
                b.labels.get(i).copied().unwrap_or(""),
            );
        }
    }

    /// Kitsch's tab: a peak rising out of the leading edge onto the top
    /// rail, and a cut trailing corner. `M 0,46 V 24 L 22,0 L 28,9 H
    /// 155 q 6,0 6,5 V 24 L 139,46 Z` in the trace, straightened.
    fn chevron(
        &self,
        frame: &mut canvas::Frame,
        scale: Scale,
        at: crate::style::Frame,
        fill: Option<iced::Color>,
        width: f32,
    ) {
        let s = self.paint();
        let (x, y, w, h) = (at.x, at.y, at.w, at.h);
        poly_at(
            frame,
            scale,
            &[
                (x, y + h),
                (x, y + h * 0.52),
                (x + 22.0, y),
                (x + 28.0, y + 9.0),
                (x + w - 6.0, y + 9.0),
                (x + w, y + 14.0),
                (x + w, y + h * 0.52),
                (x + w - 22.0, y + h),
            ],
            true,
            fill,
            if fill.is_some() { None } else { Some((ink(s, Ink::Fg), width)) },
        );
    }

    /// Region D: the clearance badges.
    fn badges(&self, frame: &mut canvas::Frame, scale: Scale, b: &MailBadges) {
        let s = self.paint();
        let cols = b.cols.max(1);
        for i in 0..b.count {
            let at = b
                .first
                .shifted(b.dx * (i % cols) as f32, b.dy * (i / cols) as f32);
            let selected = b.selected == Some(i);
            if selected {
                self.fill(frame, scale, |me, f| {
                    box_at(f, scale, at, b.trim, Some(ink(me.paint(), Ink::Select)), None);
                });
            } else {
                box_at(
                    frame,
                    scale,
                    at,
                    b.trim,
                    b.fill.map(|r| ink(s, r)),
                    Some((ink(s, b.stroke), b.width)),
                );
            }
            if let Some(cap) = b.caption {
                let role = if selected { Ink::OnSelect } else { cap.ink };
                label(
                    frame,
                    scale,
                    Run {
                        x: at.x + cap.x,
                        y: at.y + cap.y,
                        ink: role,
                        ..cap
                    },
                    ink(s, role),
                    b.caption_text,
                );
            }
            let role = if selected { Ink::OnSelect } else { b.label.ink };
            label(
                frame,
                scale,
                Run {
                    x: at.x + b.label.x,
                    y: at.y + b.label.y,
                    ink: role,
                    ..b.label
                },
                ink(s, role),
                b.labels.get(i).copied().unwrap_or(""),
            );
        }
    }
}

impl canvas::Program<Message, Style> for Sheet<'_> {
    type State = ();

    /// Click a row to select it.
    ///
    /// The plates are drawn from the era table in design coordinates,
    /// so the hit test is the same arithmetic run backwards -- no
    /// second copy of the geometry, and no `mouse_area` per row over a
    /// canvas that already knows where every row is.
    fn update(
        &self,
        _state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        if !matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        ) {
            return None;
        }
        let row = self.hit(cursor.position_in(bounds)?, bounds)?;
        Some(Action::publish(Message::Select(row)))
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        match cursor.position_in(bounds).and_then(|p| self.hit(p, bounds)) {
            Some(_) => mouse::Interaction::Pointer,
            None => mouse::Interaction::default(),
        }
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
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        let scale = Scale {
            sx: w / DW,
            sy: h / DH,
        };
        let m = &self.style.mailbox;

        // The chrome is static; the pieces an era moves are in its
        // motions, and come up under those covers after it. Then the
        // regions, each under its own cover, and the overlay. All of
        // it through `under`, so the order here is the paint order.
        self.under(&mut frame, scale, Cover::OPEN, |me, f| pieces(f, scale, me.paint(), m.chrome));
        for motion in self.style.mailbox.motions {
            for part in motion.parts {
                if let MailPart::Pieces(moving) = *part {
                    self.under(&mut frame, scale, self.cover(*part), |me, f| pieces(f, scale, me.paint(), moving));
                }
            }
        }
        self.under(&mut frame, scale, self.cover(MailPart::List), |me, f| me.list(f, scale, &m.list));
        self.under(&mut frame, scale, self.cover(MailPart::Panel), |me, f| me.panel(f, scale, &m.panel, &m.list));
        self.under(&mut frame, scale, self.cover(MailPart::Title), |me, f| me.title(f, scale, &m.panel, &m.list));
        self.under(&mut frame, scale, self.cover(MailPart::Buttons), |me, f| me.buttons(f, scale, &m.buttons));
        self.under(&mut frame, scale, self.cover(MailPart::Badges), |me, f| me.badges(f, scale, &m.badges));
        self.under(&mut frame, scale, Cover::OPEN, |me, f| pieces(f, scale, me.paint(), m.overlay));

        vec![frame.into_geometry()]
    }
}

/// Draw an era's free-standing pieces -- [`Mailbox::chrome`] under the
/// four regions, [`Mailbox::overlay`] over them.
fn pieces(frame: &mut canvas::Frame, scale: Scale, s: Paint, pieces: &[Piece]) {
    for piece in pieces {
        match piece {
            Piece::Box {
                at,
                fill,
                stroke,
                width,
                trim,
            } => box_at(
                frame,
                scale,
                *at,
                *trim,
                fill.map(|r| ink(s, r)),
                stroke.map(|r| (ink(s, r), *width)),
            ),
            Piece::Poly {
                points,
                fill,
                stroke,
                width,
                close,
            } => poly_at(
                frame,
                scale,
                points,
                *close,
                fill.map(|r| ink(s, r)),
                stroke.map(|r| (ink(s, r), *width)),
            ),
            Piece::Curve {
                start,
                steps,
                fill,
                stroke,
                width,
                close,
            } => curve_at(
                frame,
                scale,
                *start,
                steps,
                *close,
                fill.map(|r| ink(s, r)),
                stroke.map(|r| (ink(s, r), *width)),
            ),
            Piece::Label(note) => label(frame, scale, note.at, ink(s, note.at.ink), note.text),
        }
    }
}
