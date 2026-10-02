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
//! row of that list. Entropism reads a message away from its cursor and
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
use crate::screens::scene::{blend_over, Backdrop, Pointer, PointerAction};
use crate::widgets::ground;
use crate::Element;
use iced::widget::{canvas, stack, Action};
use iced::{mouse, Color, Event, Length, Point, Rectangle, Renderer, Size, Subscription, Vector};
use std::cell::Cell;
use std::time::{Duration, Instant};

/// The frame every trace measures in.
const DW: f32 = 1600.0;
const DH: f32 = 900.0;

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
    /// The initial design target may use a heading absent from the
    /// inbox. Once a row is opened, its content replaces that fixture.
    initial_fixture: bool,
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
            initial_fixture: true,
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
                self.initial_fixture = false;
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
            initial_fixture: self.initial_fixture,
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
    initial_fixture: bool,
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
    /// Monochrome artwork adopts its owning row's resolved printing.
    ink_override: Option<Ink>,
}

impl Sheet<'_> {
    /// Only the list's cursor moves; the reader's selection and message
    /// remain owned by Mail. No row has the index used while held.
    fn cursor_row(&self, pointer: &Pointer<usize>) -> usize {
        if self.style.mailbox_cursor {
            if let Some((row, held)) = pointer.interaction(self.style.mailbox.list.rows) {
                return if held { self.style.mailbox.list.rows.len() } else { row };
            }
        }
        self.selected
    }

    /// The inks as the region being drawn wants them.
    fn paint(&self) -> Paint<'_> {
        Paint { style: self.style, alpha: self.alpha.get(), ink_override: None }
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

#[cfg(test)]
mod interaction_tests {
    use super::*;
    use iced::widget::canvas::Program;

    fn sheet(style: &Style) -> Sheet<'_> {
        Sheet { style, selected: style.mailbox.list.selected,
            showing: style.mailbox.panel.message, initial_fixture: true,
            at: motion::REST, alpha: Cell::new(1.0) }
    }

    fn click(pressed: bool) -> Event {
        Event::Mouse(if pressed { mouse::Event::ButtonPressed(mouse::Button::Left) }
            else { mouse::Event::ButtonReleased(mouse::Button::Left) })
    }

    fn reader(mail: &MailBox) -> Sheet<'_> {
        Sheet {
            style: &mail.style,
            selected: mail.selected,
            showing: mail.showing,
            initial_fixture: mail.initial_fixture,
            at: mail.at(),
            alpha: Cell::new(1.0),
        }
    }

    fn baselines(panel: &MailPanel) -> Vec<f32> {
        let mut result = Vec::new();
        body_lines(panel, |_, y| result.push(y));
        result
    }

    #[test]
    fn paragraph_origins_preserve_prior_lines_and_fallback_pitch() {
        let panel = crate::style::Era::Entropism.style().mailbox.panel;
        let actual = baselines(&panel);
        let expected = [325.0, 346.7, 368.4, 390.1, 429.1, 450.8, 472.5, 494.2, 535.0, 556.7];
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(expected) {
            assert!((actual - expected).abs() < 0.001, "{actual} vs {expected}");
        }

        let mut fallback = panel;
        fallback.paragraph_baselines = &[];
        assert!((baselines(&fallback)[8] - 533.2).abs() < 0.001);
        fallback.paragraph_baselines = &[335.0];
        let partial = baselines(&fallback);
        assert!((partial[0] - 335.0).abs() < 0.001);
        assert!((partial[4] - 429.1).abs() < 0.001);
        assert!((partial[8] - 533.2).abs() < 0.001);

        for (era, first, second, third) in [
            (crate::style::Era::Kitsch, 411.0, 525.0, 582.0),
            (crate::style::Era::Neokitsch, 333.0, 397.5, 526.5),
            (
                crate::style::Era::Neomil,
                347.5 - 1.0 / 2.4,
                452.5 - 1.0 / 2.4,
                557.5 - 1.0 / 2.4,
            ),
        ] {
            let other = era.style().mailbox.panel;
            assert!(other.paragraph_baselines.is_empty(), "{era:?}");
            let ys = baselines(&other);
            let second_index = other.paragraphs[0].len();
            let third_index = second_index + other.paragraphs[1].len();
            for (actual, expected) in [(ys[0], first), (ys[second_index], second), (ys[third_index], third)] {
                assert!((actual - expected).abs() < 0.001, "{era:?}: {actual} vs {expected}");
            }
        }
    }

    #[test]
    fn opening_any_row_replaces_the_fixture_even_after_returning_to_its_index() {
        for era in crate::style::Era::ALL {
            let mut mail = MailBox::new(era.style());
            let panel = mail.style.mailbox.panel;
            let list = mail.style.mailbox.list;
            let initial = reader(&mail).panel_text(&panel, &list).unwrap();
            assert_eq!(initial.0, panel.heading.map(str::to_string)
                .unwrap_or_else(|| cased(list.rows[panel.message].subject, panel.title_upper)));
            if panel.from.is_some() {
                assert_eq!(initial.1, Some(panel.sender.map(str::to_string).unwrap_or_else(||
                    format!("{}{}", list.from_prefix, cased(list.rows[panel.message].from, list.from_upper)))));
            }

            // Open the fixture's own index first, then every row and
            // return to that index. Neither path restores fixture text.
            for row in std::iter::once(panel.message).chain(0..list.rows.len()).chain(std::iter::once(panel.message)) {
                mail.update(Message::Select(row));
                let (heading, sender) = reader(&mail).panel_text(&panel, &list).unwrap();
                assert_eq!(heading, cased(list.rows[row].subject, panel.title_upper));
                assert_eq!(sender, panel.from.map(|_|
                    format!("{}{}", list.from_prefix, cased(list.rows[row].from, list.from_upper))));
                assert!(!mail.initial_fixture);
            }
            mail.enter();
            assert!(!mail.initial_fixture, "reopening the screen must preserve the reader");
        }
    }

    #[test]
    fn keyboard_selection_leaves_fixture_and_keeps_index_bounds() {
        let mut mail = MailBox::new(crate::style::Era::Neomil.style());
        mail.update(Message::Move(Dir::Left));
        assert!(mail.initial_fixture);
        mail.update(Message::Move(Dir::Down));
        assert!(!mail.initial_fixture);
        let panel = mail.style.mailbox.panel;
        let list = mail.style.mailbox.list;
        assert_eq!(reader(&mail).panel_text(&panel, &list).unwrap().0, "I'm worried man");
        mail.update(Message::Select(usize::MAX));
        assert_eq!(mail.selected, list.rows.len() - 1);
        mail.update(Message::Move(Dir::Down));
        assert_eq!(mail.showing, list.rows.len() - 1);
        mail.update(Message::Select(0));
        mail.update(Message::Move(Dir::Up));
        assert_eq!(mail.showing, 0);
    }

    #[test]
    fn opening_clock_reveals_panel_overlay_and_stops_after_rest() {
        let mut mail = MailBox::new(crate::style::Era::Neomil.style());
        mail.now = mail.origin;
        let opening = reader(&mail);
        assert!(!opening.cover(MailPart::List).shown());
        assert!(!opening.cover(MailPart::Panel).shown());
        assert!(!opening.cover(MailPart::Overlay).shown());
        assert_eq!(mail.subscription().units(), usize::from(!motion::frozen()));

        mail.update(Message::Tick(mail.origin + Duration::from_millis(350)));
        let during = reader(&mail);
        assert!(during.cover(MailPart::Panel).shown());
        assert_eq!(during.cover(MailPart::Panel), during.cover(MailPart::Overlay));

        mail.update(Message::Tick(mail.origin + motion::REST));
        let rest = reader(&mail);
        assert!(rest.cover(MailPart::List).shown());
        assert!(rest.cover(MailPart::Panel).shown());
        assert!(rest.cover(MailPart::Overlay).shown());
        assert_eq!(mail.subscription().units(), 0);
    }

    #[test]
    fn individual_row_fills_yield_to_feedback_and_keep_fallbacks() {
        let style = crate::style::Era::Neomil.style();
        let sheet = self::sheet(&style);
        let mut list = style.mailbox.list;
        list.row_fills = &[Ink::Dim, Ink::Border];
        list.row_fill = Some(Ink::Bg);
        assert_eq!(list.row_fill_at(0), Some(Ink::Dim));
        assert_eq!(list.row_fill_at(1), Some(Ink::Border));
        assert_eq!(list.row_fill_at(2), Some(Ink::Bg));
        for coat in [list.feedback.unwrap().hover, list.feedback.unwrap().pressed] {
            let dressed = sheet.row_material(&list, Some(coat));
            assert_eq!(dressed.row_fill_at(1), coat.fill);
            assert_eq!(dressed.sel_fill, coat.fill.unwrap());
            assert_eq!(dressed.row, list.row);
            assert_eq!(dressed.sel, list.sel);
        }
        list.row_fills = &[];
        assert_eq!(list.row_fill_at(0), Some(Ink::Bg));
        list.row_fill = None;
        assert_eq!(list.row_fill_at(0), None);
    }

    #[test]
    fn icon_origins_support_individual_placement_and_regular_fallbacks() {
        let icons = crate::style::Icons {
            x: 5.0, y: 12.0, pitch: 70.0,
            positions: &[(21.0, 13.0), (19.0, 86.0)], normal: &[], selected: &[],
        };
        assert_eq!(icons.origin(0), (21.0, 13.0));
        assert_eq!(icons.origin(1), (19.0, 86.0));
        assert_eq!(icons.origin(2), (5.0, 152.0));
    }

    #[test]
    fn row_printing_keeps_feedback_and_selected_sender_contrast_above_resting_inks() {
        for (era, selected_sender) in [
            (crate::style::Era::Neomil, Ink::Mid),
            (crate::style::Era::Kitsch, Ink::Select),
        ] {
            let mut list = era.style().mailbox.list;
            list.title_ink = Ink::Dim;
            list.from_ink = Ink::Border;
            list.selected_ink = Ink::Mid;
            let row = list.row.shifted(0.0, list.selected as f32 * list.pitch);
            assert_eq!(Sheet::printing_inks(&list, list.selected, row, false, None, None), (Ink::Dim, Ink::Border));
            assert_eq!(Sheet::printing_inks(&list, list.selected, row, true, None, None), (Ink::Mid, selected_sender));
            for selected in [false, true] {
                assert_eq!(Sheet::printing_inks(&list, list.selected, row, selected, Some(Ink::Fg), None), (Ink::Fg, Ink::Fg));
                assert_eq!(Sheet::printing_inks(&list, list.selected, row, selected, Some(Ink::Fg), Some(Ink::Bg)), (Ink::Fg, Ink::Bg));
            }
        }
    }

    #[test]
    fn neokitsch_selected_printing_keeps_distinct_inks_across_selection_and_press() {
        let style = crate::style::Era::Neokitsch.style();
        let sheet = self::sheet(&style);
        let list = &style.mailbox.list;
        let inks = list.selected_printing.unwrap();
        assert_eq!((inks.title, inks.sender, inks.envelope), (
            Ink::Fixed(crate::palette::rgb(0x7b5438)),
            Ink::Fixed(crate::palette::rgb(0x895f3b)),
            Ink::Fixed(crate::palette::rgb(0x865c39)),
        ));
        for index in [sheet.selected, 0, list.rows.len() - 1] {
            let row = list.row.shifted(0.0, index as f32 * list.pitch);
            let (title, sender) = Sheet::printing_inks(list, index, row, true, None, None);
            assert_eq!((title, sender, Sheet::envelope_ink(list, true, None, title)),
                (inks.title, inks.sender, inks.envelope));
        }
        let held = sheet.row_coat(0, Some((0, true))).unwrap();
        assert!(held.selection);
        assert_eq!(held.printing, None);
        assert_eq!(held.sender, None);
        assert_eq!(sheet.row_material(list, Some(held)).veneer, list.veneer);
        let row = list.row;
        let (title, sender) = Sheet::printing_inks(list, 0, row, true, Some(Ink::Fg), None);
        assert_eq!((title, sender, Sheet::envelope_ink(list, true, Some(Ink::Fg), title)),
            (Ink::Fg, Ink::Fg, Ink::Fg));
        let (title, sender) = Sheet::printing_inks(list, 0, row, true, Some(Ink::Fg), Some(Ink::Bg));
        assert_eq!((title, sender, Sheet::envelope_ink(list, true, Some(Ink::Fg), title)),
            (Ink::Fg, Ink::Bg, Ink::Fg));
        let (title, sender) = Sheet::printing_inks(list, 0, row, false, None, None);
        assert_eq!((title, sender, Sheet::envelope_ink(list, false, None, title)),
            (list.title_ink, list.from_ink, list.title_ink));

        let mut custom = style;
        custom.palette.on_select = crate::palette::rgb(0x102030);
        let custom_sheet = self::sheet(&custom);
        let custom_list = custom_sheet.row_material(&custom.mailbox.list, None);
        assert_eq!(custom_list.selected_printing, None);
        let (title, sender) = Sheet::printing_inks(&custom_list, 0, custom_list.row, true, None, None);
        assert_eq!((title, sender, Sheet::envelope_ink(&custom_list, true, None, title)),
            (Ink::OnSelect, Ink::OnSelect, Ink::OnSelect));
        assert_eq!(title.of(&custom.palette), custom.palette.on_select);

        // Values published by home/themes/neokitsch/palettes.nix. Its
        // derived panel differs from the crate's standalone bloom.
        let published = crate::theme::Theme::parse(r##"
            era = "neokitsch"
            variant = "reference"
            [colors]
            bg = "#0a0a0a"
            panel = "#16161f"
            border = "#916424"
            dim = "#8a7048"
            fg = "#e7c686"
            alert = "#fcc474"
            tape = "#e3af5f"
            banner = "#d3b279"
            onBanner = "#3a2410"
            bevel = "#c69a55"
            shade = "#5e3414"
            ornament = "#634427"
            inset = "#2c1c14"
        "##).unwrap();
        let reference = crate::style::Style::from_theme(&published);
        assert!(reference.mailbox_reference_palette());
        assert_eq!(reference.mailbox.list.selected_printing, Some(inks));
        let reference_sheet = self::sheet(&reference);
        assert_eq!(reference_sheet.row_material(&reference.mailbox.list, None).selected_printing, Some(inks));
        let mut variant = published.clone();
        variant.variant = "bleach".into();
        assert_eq!(crate::style::Style::from_theme(&variant).mailbox.list.selected_printing, None);
        let mut changed_role = reference;
        changed_role.palette.dim = crate::palette::rgb(0x554433);
        assert!(!changed_role.mailbox_reference_palette());
        assert_eq!(self::sheet(&changed_role).row_material(&changed_role.mailbox.list, None).selected_printing, None);
    }

    #[test]
    fn unread_vectors_share_row_contrast_and_motion_alpha() {
        let style = crate::style::Era::Neomil.style();
        let list = &style.mailbox.list;
        for alpha in [0.0, 0.5, 1.0] {
            let paint = Paint { style: &style, alpha, ink_override: None };
            for selected in [false, true] {
                for feedback in [None, Some(Ink::Bg)] {
                    let (title, _) = Sheet::printing_inks(list, 0, list.row, selected, feedback, None);
                    let monochrome = Paint { ink_override: Some(title), ..paint };
                    for source_role in [Ink::Fg, Ink::Border, Ink::OnSelect] {
                        assert_eq!(ink(monochrome, source_role), ink(paint, title));
                    }
                }
            }
        }
    }

    #[test]
    fn selected_typography_follows_selection_and_retains_row_fallback() {
        let mut list = crate::style::Era::Kitsch.style().mailbox.list;
        for row in 0..list.rows.len() {
            let idle = list.row_type_at(row, false).unwrap();
            let selected = list.row_type_at(row, true).unwrap();
            assert!(idle.from.bold && !idle.from.semibold);
            assert!(selected.from.semibold && !selected.from.bold);
            assert_eq!((idle.from.x, idle.from.y, idle.from.size),
                (selected.from.x, selected.from.y, selected.from.size));
            assert_eq!(idle.title, selected.title);
        }
        list.selected_row_type = None;
        for row in 0..=list.rows.len() {
            assert_eq!(list.row_type_at(row, true), list.row_type_at(row, false));
        }
        for era in [crate::style::Era::Entropism, crate::style::Era::Neokitsch,
            crate::style::Era::Neomil] {
            let list = era.style().mailbox.list;
            assert!(list.selected_row_type.is_none());
            for row in 0..list.rows.len() {
                assert_eq!(list.row_type_at(row, true), list.row_type_at(row, false));
            }
        }
    }

    #[test]
    fn measured_sender_position_controls_fill_contrast_without_overriding_feedback() {
        use crate::style::MailRowType;
        let mut list = crate::style::Era::Kitsch.style().mailbox.list;
        list.selected_ink = Ink::Mid;
        const ROW_TYPE: &[MailRowType] = &[
            MailRowType { title: Run::new(0.0, 10.0, 20.0, Ink::Bg),
                from: Run::new(0.0, 10.0, 16.0, Ink::Bg).medium().stretched(1.2) },
            MailRowType { title: Run::new(0.0, 10.0, 20.0, Ink::Bg),
                from: Run::new(0.0, 100.0, 16.0, Ink::Bg) },
        ];
        list.row_type = ROW_TYPE;
        list.selected_row_type = None;
        let row = list.row;
        assert_eq!(Sheet::printing_inks(&list, 0, row, true, None, None), (Ink::Mid, Ink::Mid));
        assert_eq!(Sheet::printing_inks(&list, 1, row, true, None, None), (Ink::Mid, Ink::Select));
        assert_eq!(Sheet::printing_inks(&list, 0, row, false, None, None), (list.title_ink, list.from_ink));
        for index in [0, 1] {
            assert_eq!(Sheet::printing_inks(&list, index, row, true, Some(Ink::Fg), Some(Ink::Dim)),
                (Ink::Fg, Ink::Dim));
        }
    }

    #[test]
    fn selected_unread_placement_preserves_normal_and_fallback_frames() {
        let mut list = crate::style::Era::Neomil.style().mailbox.list;
        let normal = crate::style::Frame::new(162.0, 44.5, 74.0, 10.8);
        let selected = normal.shifted(0.0, 2.5);
        list.new_pill = Some(normal);
        list.new_pill_selected = Some(selected);
        assert_eq!(list.unread_frame(false), Some(normal));
        assert_eq!(list.unread_frame(true), Some(selected));
        list.new_pill_selected = None;
        assert_eq!(list.unread_frame(true), Some(normal));
        list.new_pill = None;
        assert_eq!(list.unread_frame(false), None);
        assert_eq!(list.unread_frame(true), None);
    }

    #[test]
    fn every_mail_row_activates_only_on_release_at_stretched_sizes() {
        for era in crate::style::Era::ALL {
            let style = era.style();
            let sheet = self::sheet(&style);
            for (sx, sy) in [(0.5, 0.5), (1.0, 1.0), (2.0, 1.5)] {
                let bounds = Rectangle { x: 19.0, y: 31.0, width: DW * sx, height: DH * sy };
                for row in 0..style.mailbox.list.rows.len() {
                    let band = sheet.row_at(row);
                    let cursor = mouse::Cursor::Available(Point::new(
                        bounds.x + (band.x + band.w / 2.0) * sx,
                        bounds.y + (band.y + band.h / 2.0) * sy,
                    ));
                    let mut pointer = Pointer::default();
                    assert!(sheet.update(&mut pointer, &click(false), bounds, cursor).is_none());
                    let (message, _, status) = sheet.update(&mut pointer, &click(true), bounds, cursor).unwrap().into_inner();
                    assert!(message.is_none());
                    assert_eq!(status, iced::event::Status::Captured);
                    let (message, _, _) = sheet.update(&mut pointer, &click(false), bounds, cursor).unwrap().into_inner();
                    assert!(matches!(message, Some(Message::Select(i)) if i == row));
                }
            }
        }
    }

    #[test]
    fn mail_drag_focus_and_era_changes_cancel_activation() {
        let style = crate::style::Era::Entropism.style();
        let sheet = self::sheet(&style);
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(DW, DH));
        let cursor = |i| {
            let row = sheet.row_at(i);
            mouse::Cursor::Available(Point::new(row.x + row.w / 2.0, row.y + row.h / 2.0))
        };
        for end in [cursor(1), mouse::Cursor::Unavailable] {
            let mut pointer = Pointer::default();
            sheet.update(&mut pointer, &click(true), bounds, cursor(0));
            let (message, _, _) = sheet.update(&mut pointer, &click(false), bounds, end).unwrap().into_inner();
            assert!(message.is_none());
        }
        for cancel in [Event::Window(iced::window::Event::Unfocused), Event::Mouse(mouse::Event::CursorLeft)] {
            let mut pointer = Pointer::default();
            sheet.update(&mut pointer, &click(true), bounds, cursor(0));
            sheet.update(&mut pointer, &cancel, bounds, cursor(0));
            assert!(sheet.update(&mut pointer, &click(false), bounds, cursor(0)).is_none());
        }
        let mut pointer = Pointer::default();
        sheet.update(&mut pointer, &click(true), bounds, cursor(0));
        let other = crate::style::Era::Neomil.style();
        pointer.sync(other.mailbox.list.rows);
        assert!(sheet.update(&mut pointer, &click(false), bounds, cursor(0)).is_none());
    }

    #[test]
    fn only_entropism_moves_the_list_cursor_without_opening_mail() {
        for era in crate::style::Era::ALL {
            let style = era.style();
            let sheet = self::sheet(&style);
            let selected = sheet.selected;
            let showing = sheet.showing;
            let row = (selected + 1) % style.mailbox.list.rows.len();
            let mut pointer = Pointer::default();
            pointer.sync(style.mailbox.list.rows);
            pointer.event(&Event::Mouse(mouse::Event::CursorMoved { position: Point::ORIGIN }), Some(row));
            assert_eq!(sheet.cursor_row(&pointer), if era == crate::style::Era::Entropism { row } else { selected });
            pointer.event(&click(true), Some(row));
            assert_eq!(sheet.cursor_row(&pointer), if era == crate::style::Era::Entropism { style.mailbox.list.rows.len() } else { selected });
            pointer.event(&Event::Window(iced::window::Event::Unfocused), None);
            assert_eq!(sheet.cursor_row(&pointer), selected);
            assert_eq!((sheet.selected, sheet.showing), (selected, showing));
        }
    }

    #[test]
    fn row_feedback_tracks_gesture_without_changing_reader_or_unread_content() {
        for era in [crate::style::Era::Neomil, crate::style::Era::Neokitsch, crate::style::Era::Kitsch] {
            let style = era.style();
            let sheet = self::sheet(&style);
            let list = &style.mailbox.list;
            let target = (sheet.selected + 1) % list.rows.len();
            let initial = (sheet.selected, sheet.showing);
            let mut pointer = Pointer::default();
            pointer.sync(list.rows);
            pointer.event(&Event::Mouse(mouse::Event::CursorMoved { position: Point::ORIGIN }), Some(target));
            let hover = sheet.row_coat(target, pointer.interaction(list.rows)).unwrap();
            assert_eq!(hover, list.feedback.unwrap().hover);
            assert!(sheet.row_coat(sheet.selected, pointer.interaction(list.rows)).is_none());
            pointer.event(&click(true), Some(target));
            let held = sheet.row_coat(target, pointer.interaction(list.rows)).unwrap();
            assert_eq!(held, list.feedback.unwrap().pressed);
            let dressed = sheet.row_material(list, Some(held));
            assert_eq!((dressed.row, dressed.sel, dressed.row_trim, dressed.sel_trim),
                (list.row, list.sel, list.row_trim, list.sel_trim));
            assert_eq!(dressed.rows, list.rows);
            assert_eq!((dressed.glyph_x, dressed.glyph_dy, dressed.glyph_offsets, dressed.glyph_w, dressed.new_pill, dressed.icons),
                (list.glyph_x, list.glyph_dy, list.glyph_offsets, list.glyph_w, list.new_pill, list.icons));
            pointer.event(&Event::Window(iced::window::Event::Unfocused), None);
            assert!(sheet.row_coat(target, pointer.interaction(list.rows)).is_none());
            assert_eq!(sheet.row_material(list, None), *list);
            assert_eq!((sheet.selected, sheet.showing), initial);
            pointer.event(&click(false), Some(target));
            assert!(sheet.row_coat(target, pointer.interaction(list.rows)).is_none());
        }
    }

    #[test]
    fn selected_row_feedback_preserves_material_and_geometry() {
        let style = crate::style::Era::Neokitsch.style();
        let sheet = self::sheet(&style);
        let list = &style.mailbox.list;
        assert!(sheet.row_coat(sheet.selected, Some((sheet.selected, false))).is_none());
        let held = sheet.row_coat(sheet.selected, Some((sheet.selected, true))).unwrap();
        assert!(held.selection);
        assert_eq!(sheet.row_material(list, Some(held)).veneer, list.veneer);
        let hover = sheet.row_coat(0, Some((0, false))).unwrap();
        assert_eq!(hover.echo.unwrap().rings, 7);
        assert!(!hover.selection);
        let style = crate::style::Era::Neomil.style();
        let sheet = self::sheet(&style);
        let hover = sheet.row_coat(sheet.selected, Some((sheet.selected, false))).unwrap();
        let held = sheet.row_coat(sheet.selected, Some((sheet.selected, true))).unwrap();
        assert_ne!(hover.fill, held.fill);
        assert!(held.printing.is_some());
        assert_eq!(sheet.row_material(&style.mailbox.list, Some(held)).sel, style.mailbox.list.sel);
    }

    #[test]
    fn kitsch_row_lift_keeps_two_piece_geometry_and_flat_press_keeps_sender_legible() {
        let style = crate::style::Era::Kitsch.style();
        let sheet = self::sheet(&style);
        let list = &style.mailbox.list;
        let target = sheet.selected + 1;
        let hover = sheet.row_coat(target, Some((target, false))).unwrap();
        let held = sheet.row_coat(target, Some((target, true))).unwrap();
        let echo = hover.echo.unwrap();
        assert_eq!(echo.rings, 1);
        assert_eq!(echo.step, crate::style::Frame::new(20.0, -20.0, 0.0, 0.0));
        assert!(echo.fill.is_some());
        assert_eq!((echo.fill_alpha, echo.alpha), (0.58, 0.80));
        assert!(held.echo.is_none());
        for coat in [hover, held] {
            assert!(coat.selection);
            let dressed = sheet.row_material(list, Some(coat));
            let [(body, body_trim), (icon, icon_trim)] = Sheet::row_faces(&dressed, list.pitch);
            assert_eq!(body, Some(list.sel.shifted(0.0, list.pitch)));
            assert_eq!(icon, list.sel_icon.map(|at| at.shifted(0.0, list.pitch)));
            assert_eq!((body_trim, icon_trim), (list.sel_trim, list.sel_icon_trim));
            let (body, icon) = (body.unwrap(), icon.unwrap());
            assert_eq!(body.x - (icon.x + icon.w), 2.0);
            assert_eq!(dressed.rows, list.rows);
            assert_ne!(coat.sender, coat.printing, "sender sits below the colored face");
        }
        assert_ne!(hover.sender, held.sender);
        assert_eq!(sheet.row_material(list, Some(held)).sel_fill, list.sel_fill);
        let selected = sheet.row_coat(sheet.selected, Some((sheet.selected, false))).unwrap();
        assert_eq!(selected.echo, hover.echo);
        assert_eq!(sheet.row_material(list, Some(selected)).sel_fill, list.sel_fill);
        assert!(selected.printing.is_none());
        assert!(selected.sender.is_none());
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
    let c = s.ink_override.unwrap_or(role).of(&s.style.palette);
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
fn label(frame: &mut canvas::Frame, scale: Scale, s: Paint, at: Run, color: Color, content: &str) {
    let _ = label_keep(frame, scale, s, at, color, content, 0);
}

// Retain original shaping and advances after an era-provided outline prefix.
fn label_keep(
    frame: &mut canvas::Frame, scale: Scale, s: Paint, at: Run,
    color: Color, content: &str, skip_prefix: usize,
) -> bool {
    if content.is_empty() || at.size <= 0.0 {
        return false;
    }
    let size = scale.len(at.size);
    let text = canvas::Text {
        content: content.to_string(),
        position: Point::new(at.x * scale.sx, at.y * scale.sy - size * s.style.mailbox.text_baseline),
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
    };
    if skip_prefix > 0 {
        // Stretched labels already use Iced's outline branch. Keep their
        // original shaping, phase and per-glyph paths.
        // Unexpected shaping or the cached uniform-scale path falls back.
        if !content.is_ascii() || (at.stretch - 1.0).abs() <= 1e-4 {
            return false;
        }
        let origin = text.position;
        let mut glyphs = Vec::new();
        canvas::Text { position: Point::ORIGIN, ..text }
            .draw_with(|path, color| glyphs.push((path, color)));
        if glyphs.len() != content.len() || skip_prefix > glyphs.len() {
            return false;
        }
        frame.with_save(|f| {
            f.translate(Vector::new(origin.x, origin.y));
            f.scale_nonuniform(Vector::new(at.stretch, 1.0));
            for (path, color) in glyphs.into_iter().skip(skip_prefix) {
                f.fill(&path, color);
            }
        });
    } else if (at.stretch - 1.0).abs() <= 1e-4 {
        frame.fill_text(text);
    } else {
        // Scale glyphs about the positioned anchor, not the canvas
        // origin. Canvas text alignment then retains its left/center/
        // right meaning while the baseline remains at the same y.
        frame.with_save(|frame| {
            frame.translate(iced::Vector::new(text.position.x, text.position.y));
            frame.scale_nonuniform(iced::Vector::new(at.stretch, 1.0));
            frame.fill_text(canvas::Text { position: Point::ORIGIN, ..text });
        });
    }
    true
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

/// Visit the trace's explicit lines at their canvas baselines. An
/// overridden paragraph start does not alter the pitch of later ones.
fn body_lines(panel: &MailPanel, mut emit: impl FnMut(&str, f32)) {
    let mut pitched_y = panel.body.y;
    for (index, paragraph) in panel.paragraphs.iter().enumerate() {
        let offset = panel.paragraph_baselines.get(index).map(|start| start - pitched_y).unwrap_or(0.0);
        for line in *paragraph {
            emit(line, pitched_y + offset);
            pitched_y += panel.line;
        }
        pitched_y += panel.para - panel.line;
    }
}

impl Sheet<'_> {
    /// Region A: the list frame, its rows, their glyphs and their text.
    fn row_coat(&self, i: usize, interaction: Option<(usize, bool)>) -> Option<crate::style::MailRowCoat> {
        let (target, held) = interaction?;
        if target != i { return None; }
        let states = self.style.mailbox.list.feedback?;
        if held { Some(states.pressed) }
        else if i == self.selected { states.selected_hover }
        else { Some(states.hover) }
    }

    fn row_material(&self, list: &MailList, coat: Option<crate::style::MailRowCoat>) -> MailList {
        let mut dressed = *list;
        // Source-sampled marks belong to the reference palette. A custom
        // theme, including a direct palette edit, keeps its OnSelect role.
        if !self.style.mailbox_reference_palette() {
            dressed.selected_printing = None;
        }
        if let Some(c) = coat {
            if let Some(fill) = c.fill {
                dressed.row_fill = Some(fill);
                dressed.row_fills = &[];
                dressed.sel_fill = fill;
                dressed.veneer = None;
            }
            dressed.row_stroke = c.outline;
            if let Some(spine) = c.spine { dressed.rule_ink = spine; }
        }
        dressed
    }

    fn row_faces(list: &MailList, shift: f32) -> [(Option<crate::style::Frame>, Trim); 2] {
        [
            (Some(list.sel.shifted(0.0, shift)), list.sel_trim),
            (list.sel_icon.map(|at| at.shifted(0.0, shift)), list.sel_icon_trim),
        ]
    }

    fn list(&self, frame: &mut canvas::Frame, scale: Scale, list: &MailList, interaction: Option<(usize, bool)>) {
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
                let (x, y) = icons.origin(i);
                let art = if i == self.selected { icons.selected } else { icons.normal };
                frame.with_save(|f| {
                    f.translate(Vector::new(x * scale.sx, y * scale.sy));
                    pieces(f, scale, self.paint(), art);
                });
            }
        }

        for (i, mail) in list.rows.iter().enumerate() {
            let row = self.row_at(i);
            let coat = self.row_coat(i, interaction);
            let selected = i == self.selected || coat.is_some_and(|c| c.selection);
            let shift = (i as f32 - list.selected as f32) * list.pitch;
            let dressed = self.row_material(list, coat);
            if let Some(c) = coat {
                if let Some(echo) = c.echo {
                    for ring in (1..=echo.rings).rev() {
                        let n = f32::from(ring);
                        let mut color = ink(s, echo.ink);
                        color.a *= (echo.alpha - (n - 1.0) * echo.fade).max(0.0);
                        let fill = echo.fill.map(|fill| {
                            let mut color = ink(s, fill);
                            color.a *= (echo.fill_alpha - (n - 1.0) * echo.fade).max(0.0);
                            color
                        });
                        for (at, trim) in Self::row_faces(list, shift) {
                            if let Some(at) = at {
                                let outline = crate::style::Frame::new(
                                    at.x + n * echo.step.x, at.y + n * echo.step.y,
                                    at.w + n * echo.step.w, at.h + n * echo.step.h,
                                );
                                box_at(frame, scale, outline, trim, fill, Some((color, echo.width)));
                            }
                        }
                    }
                }
            }
            let list = &dressed;
            let printing = coat.and_then(|c| c.printing);

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
                    if let Some(outline) = coat.and_then(|c| c.outline) {
                        for (at, trim) in Self::row_faces(list, shift) {
                            if let Some(at) = at {
                                box_at(f, scale, at, trim, None,
                                    Some((ink(me.paint(), outline), list.row_width.max(1.1))));
                            }
                        }
                    }
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
                    me.printing(f, scale, list, i, mail, row, true, printing, coat.and_then(|c| c.sender));
                });
            } else {
                if coat.is_some_and(|c| c.echo.is_some()) {
                    let at = list.sel.shifted(0.0, shift);
                    box_at(frame, scale, at, list.sel_trim, None,
                        coat.and_then(|c| c.outline).map(|ink_| (ink(s, ink_), 1.1)));
                }
                if list.decor == RowDecor::Boxed {
                    box_at(
                        frame,
                        scale,
                        row,
                        list.row_trim,
                        list.row_fill_at(i).map(|r| ink(s, r)),
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
                self.printing(frame, scale, list, i, mail, row, false, printing, coat.and_then(|c| c.sender));
            }
        }
        pieces(frame, scale, self.paint(), list.footer);
    }

    /// Feedback wins over selection contrast, which wins over resting
    /// printing. A sender outside the selected fill keeps bright ink.
    fn printing_inks(list: &MailList, index: usize, row: crate::style::Frame, selected: bool, printing: Option<Ink>, sender: Option<Ink>) -> (Ink, Ink) {
        let shift = row.y - list.row.y - list.selected as f32 * list.pitch;
        let title = printing.unwrap_or(if selected {
            list.selected_printing.map_or(list.selected_ink, |inks| inks.title)
        } else { list.title_ink });
        // Kitsch's bar ends above its sender line, so contrast follows
        // the actual fill geometry rather than the selected index alone.
        let from_dy = list.row_type_at(index, selected).map_or(list.from_dy, |t| t.from.y);
        let on_fill = selected && row.y + from_dy <= list.sel.y + shift + list.sel.h;
        let from = sender.or(printing).unwrap_or(match (selected, on_fill) {
            (true, true) => list.selected_printing.map_or(list.selected_ink, |inks| inks.sender),
            (true, false) => Ink::Select,
            _ => list.from_ink,
        });
        (title, from)
    }

    fn envelope_ink(list: &MailList, selected: bool, printing: Option<Ink>, title_ink: Ink) -> Ink {
        printing.unwrap_or_else(|| {
            if selected { list.selected_printing.map_or(title_ink, |inks| inks.envelope) }
            else { title_ink }
        })
    }

    /// One row's printing: the envelope, the subject, the sender and
    /// the NEW pill the era marks unread rows with. The selected row's
    /// is drawn under [`MailPart::Printing`], from [`Sheet::list`].
    fn printing(&self, frame: &mut canvas::Frame, scale: Scale, list: &MailList, index: usize, mail: &crate::style::Mail, row: crate::style::Frame, selected: bool, printing: Option<Ink>, sender: Option<Ink>) {
        let s = self.paint();
        let (title_ink, from_ink) = Self::printing_inks(list, index, row, selected, printing, sender);
        let envelope_ink = Self::envelope_ink(list, selected, printing, title_ink);
        let glyph_y = row.y + list.glyph_dy + list.glyph_offsets.get(index).copied().unwrap_or(0.0);

        let art = list.envelope.map(|art| if mail.unread { art.open } else { art.normal })
            .filter(|art| !art.is_empty());
        if let Some(art) = art {
            frame.with_save(|frame| {
                let origin = scale.point(list.glyph_x, glyph_y);
                frame.translate(iced::Vector::new(origin.x, origin.y));
                pieces(frame, scale, Paint { ink_override: Some(envelope_ink), ..s }, art);
            });
        } else {
            envelope(
                frame,
                scale,
                list.glyph_x,
                glyph_y,
                list.glyph_w,
                mail.unread,
                ink(s, envelope_ink),
                1.2,
            );
        }

        let mut title = Run {
            bold: list.title_bold,
            ..Run::new(
                list.text_x,
                row.y + list.title_dy,
                list.title_size,
                title_ink,
            )
        };
        if let Some(t) = list.row_type_at(index, selected) {
            title = Run { y: row.y + t.title.y, ink: title_ink, ..t.title };
        }
        label(
            frame,
            scale,
            s,
            title,
            ink(s, title_ink),
            &cased(mail.subject, list.title_upper),
        );

        let sender = format!("{}{}", list.from_prefix, cased(mail.from, list.from_upper));
        let mut at = match list.from_at {
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
        if let Some(t) = list.row_type_at(index, selected) {
            at = Run { y: row.y + t.from.y, ink: from_ink, ..t.from };
        }
        label(frame, scale, s, at, ink(s, from_ink), &sender);

        // The NEW pill, on the rows the trace puts one on -- its
        // unread ones, in the era that marks them this way.
        if let Some(pill) = list.unread_frame(selected) {
            if mail.unread {
                let at = pill.shifted(row.x, row.y);
                if !list.new_pill_art.is_empty() {
                    frame.with_save(|f| {
                        f.translate(Vector::new(at.x * scale.sx, at.y * scale.sy));
                        pieces(f, scale, Paint { ink_override: Some(title_ink), ..s }, list.new_pill_art);
                    });
                    return;
                }
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
                    s,
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
        body_lines(panel, |line, y| {
            label(frame, scale, s, Run { y, ..panel.body }, ink(s, panel.body.ink), line);
        });
    }

    /// The panel's heading and sender line: [`MailPart::Title`], drawn
    /// after the panel so an era whose heading stands outside the
    /// panel's clip (neomil's, above `#message-open`) can leave it out
    /// of that motion.
    fn title(&self, frame: &mut canvas::Frame, scale: Scale, panel: &MailPanel, list: &MailList) {
        let s = self.paint();
        let Some((heading, sender)) = self.panel_text(panel, list) else { return; };
        label(frame, scale, s, panel.title, ink(s, panel.title.ink), &heading);
        if let (Some(at), Some(sender)) = (panel.from, sender) {
            label(frame, scale, s, at, ink(s, at.ink), &sender);
        }
    }

    /// Initial source wording is independent of the selected row index.
    /// Returning to the fixture's original row still reads that row's mail.
    fn panel_text(&self, panel: &MailPanel, list: &MailList) -> Option<(String, Option<String>)> {
        let mail = list.rows.get(self.showing).or(list.rows.last())?;
        let heading = match panel.heading.filter(|_| self.initial_fixture) {
            Some(text) => text.to_string(),
            None => cased(mail.subject, panel.title_upper),
        };
        let sender = panel.from.map(|_| match panel.sender.filter(|_| self.initial_fixture) {
            Some(text) => text.to_string(),
            None => format!("{}{}", list.from_prefix, cased(mail.from, list.from_upper)),
        });
        Some((heading, sender))
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
            let at_label = b.label_runs.get(i).copied().unwrap_or(b.label);
            let role = if filled { Ink::OnSelect } else { at_label.ink };
            label(
                frame,
                scale,
                s,
                Run {
                    x: at.x + at_label.x,
                    y: at.y + at_label.y,
                    ink: role,
                    ..at_label
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
                    s,
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
            let at_label = b.label_runs.get(i).copied().unwrap_or(b.label);
            let role = if selected { Ink::OnSelect } else { at_label.ink };
            let content = b.labels.get(i).copied().unwrap_or("");
            let full_run = Run {
                x: at.x + at_label.x,
                y: at.y + at_label.y,
                ink: role,
                ..at_label
            };
            if let Some(art) = b.label_art.get(i).filter(|art|
                art.text == content && art.replace_prefix > 0 && !art.pieces.is_empty()
            ) {
                if label_keep(frame, scale, s, full_run, ink(s, role), content, art.replace_prefix) {
                    // Match text sizing: uniform glyph scale about the label's
                    // responsive anchor, including in non-16:9 windows.
                    let k = scale.sx.min(scale.sy);
                    frame.with_save(|f| {
                        f.translate(Vector::new(full_run.x * scale.sx, full_run.y * scale.sy));
                        pieces(f, Scale { sx: k, sy: k },
                            Paint { ink_override: Some(role), ..s }, art.pieces);
                    });
                    continue;
                }
            }
            label(frame, scale, s, full_run, ink(s, role), content);
        }
    }
}

impl canvas::Program<Message, Style> for Sheet<'_> {
    type State = Pointer<usize>;

    /// Select only on release over the row originally pressed.
    ///
    /// The plates are drawn from the era table in design coordinates,
    /// so the hit test is the same arithmetic run backwards -- no
    /// second copy of the geometry, and no `mouse_area` per row over a
    /// canvas that already knows where every row is.
    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        state.sync(self.style.mailbox.list.rows);
        let row = cursor.position_in(bounds).and_then(|p| self.hit(p, bounds));
        match state.event(event, row) {
            PointerAction::Ignore => None,
            PointerAction::Redraw => Some(Action::request_redraw()),
            PointerAction::Capture => Some(Action::request_redraw().and_capture()),
            PointerAction::Activate(row) => Some(Action::publish(Message::Select(row)).and_capture()),
        }
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
        state: &Self::State,
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
        let list_sheet = Sheet {
            selected: self.cursor_row(state),
            alpha: Cell::new(1.0),
            ..*self
        };
        list_sheet.under(&mut frame, scale, self.cover(MailPart::List), |me, f| me.list(f, scale, &m.list, state.interaction(m.list.rows)));
        self.under(&mut frame, scale, self.cover(MailPart::Panel), |me, f| me.panel(f, scale, &m.panel, &m.list));
        self.under(&mut frame, scale, self.cover(MailPart::Title), |me, f| me.title(f, scale, &m.panel, &m.list));
        self.under(&mut frame, scale, self.cover(MailPart::Buttons), |me, f| me.buttons(f, scale, &m.buttons));
        self.under(&mut frame, scale, self.cover(MailPart::Badges), |me, f| me.badges(f, scale, &m.badges));
        self.under(&mut frame, scale, self.cover(MailPart::Overlay), |me, f| pieces(f, scale, me.paint(), m.overlay));

        vec![frame.into_geometry()]
    }
}

/// Draw an era's free-standing pieces -- [`Mailbox::chrome`] under the
/// four regions, [`Mailbox::overlay`] over them.
fn pieces(frame: &mut canvas::Frame, scale: Scale, s: Paint, artwork: &[Piece]) {
    for piece in artwork {
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
            Piece::Label(note) => label(frame, scale, s, note.at, ink(s, note.at.ink), note.text),
            Piece::LabelArt { note, pieces: art } => {
                if art.is_empty() {
                    label(frame, scale, s, note.at, ink(s, note.at.ink), note.text);
                } else if !note.text.is_empty() && note.at.size > 0.0 {
                    let k = scale.sx.min(scale.sy);
                    frame.with_save(|f| {
                        f.translate(Vector::new(note.at.x * scale.sx, note.at.y * scale.sy));
                        pieces(f, Scale { sx: k, sy: k }, Paint {
                            ink_override: Some(s.ink_override.unwrap_or(note.at.ink)),
                            ..s
                        }, art);
                    });
                }
            },
        }
    }
}
