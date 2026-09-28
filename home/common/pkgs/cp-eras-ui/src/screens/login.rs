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
//!
//! Since 2026-09-06 the screen is also a working greeter. The live
//! slot's field takes the keyboard -- printable characters append,
//! Backspace deletes, Escape clears, Enter submits -- and draws the
//! secret as the era's mask glyphs the way [`crate::style::Entry`]
//! says to. Nothing typed shows: the field at rest is the trace's
//! mock, so the goldens are the idle frame. Run with a
//! [`Greeter`], Enter hands the secret to greetd on a thread of its
//! own (`crate::greetd`) and the screen exits when the session
//! starts; without one it is the demo it always was and Enter
//! clears the field. The live slot's action also submits on a complete
//! pointer click; dragging away or losing focus cancels that gesture.

use crate::greetd::{self, Refusal, Secret};
use crate::motion;
use crate::style::{
    Access, Blink, Caret, Coat, Colophon, Emblem, Entry, Fixture, Ink, Legend, Masthead, Plate, Plot,
    Seg, Slot,
    Style,
};
use crate::screens::scene::{Backdrop, Pointer, PointerAction};
use crate::widgets::ground;
use crate::Element;
use iced::keyboard::{self, key::Named, Key};
use iced::widget::{canvas, stack};
use iced::{mouse, Color, Length, Point, Rectangle, Renderer, Size, Subscription, Task, Vector};
use std::time::Instant;

/// The greetd side of the screen: whom to sign in, and what to start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeter {
    pub user: String,
    /// The session command, as greetd's `start_session` wants it: the
    /// elements are joined with spaces and run by the user's shell, so
    /// one element holding the whole command line is the usual shape.
    pub cmd: Vec<String>,
}

/// Where a sign-in is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Idle,
    /// The secret is with greetd; keys are ignored until it answers.
    Submitting,
    /// greetd said no. The next key clears the notice.
    Failed,
    /// The session is starting; the screen is on its way out.
    Success,
}

pub struct Login {
    pub style: Style,
    secret: Secret,
    /// Whether the keyboard has been touched. Until it has, the live
    /// field shows the trace's mock rather than an empty run, so the
    /// idle frame is the golden.
    awake: bool,
    phase: Phase,
    greeter: Option<Greeter>,
    /// The moment the last frame was asked for: what the caret blink
    /// (`motion::CARET_BLINK`) is read at. Advanced by [`Message::Tick`]
    /// while the clock runs; pinned when it is frozen.
    now: Instant,
    /// Invalidates an in-flight pointer gesture when keyboard or
    /// authentication state changes, including a disabled interval
    /// during which the canvas receives no pointer events.
    input_epoch: u64,
}

#[derive(Clone)]
pub enum Message {
    /// Printable text from a key press. The `Debug` impl below does
    /// not show it: a message log must never carry the secret.
    Typed(String),
    Backspace,
    Clear,
    Submit,
    Outcome(Result<(), Refusal>),
    /// The clock moved far enough for the caret to flip.
    Tick(Instant),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Typed(_) => f.write_str("Typed(..)"),
            Message::Backspace => f.write_str("Backspace"),
            Message::Clear => f.write_str("Clear"),
            Message::Submit => f.write_str("Submit"),
            Message::Outcome(o) => write!(f, "Outcome({o:?})"),
            Message::Tick(_) => f.write_str("Tick"),
        }
    }
}

impl crate::shell::Wears for Login {
    fn wears(&self) -> Style {
        self.style
    }
}

impl Login {
    /// The demo: every key works, and Enter clears the field.
    pub fn new(style: Style) -> Self {
        Login::greeting(style, None)
    }

    /// The greeter, when `greeter` is `Some`.
    pub fn greeting(style: Style, greeter: Option<Greeter>) -> Self {
        Login {
            style,
            secret: Secret::new(),
            awake: false,
            phase: Phase::Idle,
            greeter,
            now: motion::now(),
            input_epoch: 0,
        }
    }

    pub fn title(&self) -> String {
        format!("ACCESS — {}", self.style.era.name())
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// How many characters are in the field.
    pub fn typed(&self) -> usize {
        self.secret.len()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        if !matches!(message, Message::Tick(_)) {
            self.input_epoch = self.input_epoch.wrapping_add(1);
        }
        match message {
            Message::Tick(at) => {
                self.now = at;
                return Task::none();
            }
            Message::Outcome(Ok(())) => {
                self.phase = Phase::Success;
                return iced::exit();
            }
            Message::Outcome(Err(refusal)) => {
                // greetd's wording, never the secret. `Denied` is the
                // expected kind and stays quiet; a broken socket is
                // worth a line in the journal.
                if let Refusal::Broken(_) = refusal {
                    eprintln!("cp-eras-ui-login: {refusal}");
                }
                self.secret.clear();
                self.phase = Phase::Failed;
                return Task::none();
            }
            _ => {}
        }
        if matches!(self.phase, Phase::Submitting | Phase::Success) {
            return Task::none();
        }
        self.awake = true;
        self.phase = Phase::Idle;
        match message {
            Message::Typed(text) => self.secret.push_str(&text),
            Message::Backspace => self.secret.pop(),
            Message::Clear => self.secret.clear(),
            Message::Submit => return self.submit(),
            Message::Outcome(_) | Message::Tick(_) => unreachable!(),
        }
        Task::none()
    }

    fn submit(&mut self) -> Task<Message> {
        let Some(greeter) = self.greeter.clone() else {
            self.secret.clear();
            return Task::none();
        };
        self.phase = Phase::Submitting;
        let secret = std::mem::take(&mut self.secret);
        // A thread rather than a task on the runtime: `greetd::login`
        // blocks on a PAM stack, which is nobody's executor's business.
        // The answer comes back over a channel the task awaits.
        let (tx, rx) = async_channel::bounded(1);
        std::thread::spawn(move || {
            let outcome = match greetd::socket() {
                Some(sock) => greetd::login(&sock, &greeter.user, &secret, &greeter.cmd),
                None => Err(Refusal::Broken("GREETD_SOCK is not set".into())),
            };
            let _ = tx.send_blocking(outcome);
        });
        Task::perform(
            async move {
                rx.recv()
                    .await
                    .unwrap_or_else(|_| Err(Refusal::Broken("greetd thread went away".into())))
            },
            Message::Outcome,
        )
    }

    /// The keyboard, whole: there is one field and it always has the
    /// focus, so no widget captures anything and every key is ours.
    /// And the caret's tick -- one per half-period, which is every
    /// time the blink changes -- unless the clock is frozen, when the
    /// frame never changes and a redraw is only work.
    pub fn subscription(&self) -> Subscription<Message> {
        let keys = iced::event::listen_with(|event, _status, _window| match event {
            iced::Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                text,
                ..
            }) => key_message(key, modifiers, text.as_deref()),
            _ => None,
        });
        if motion::frozen() {
            return keys;
        }
        Subscription::batch([
            keys,
            iced::time::every(motion::CARET_BLINK / 2).map(Message::Tick),
        ])
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Three layers, and the backdrop has to be its own: `iced_wgpu`
        // buckets a canvas's geometry into meshes, images and text and
        // draws the buckets in that order, so an image is painted over
        // every shape in the same canvas no matter when it was asked
        // for. The backdrop is an image (`scene::Backdrop`, the same
        // canvas the mailbox and the scenes put under themselves), so
        // it goes in a canvas of its own, under the one that draws
        // the screen. Stretched, because `Grid` stretches the art.
        stack![
            ground(&self.style),
            canvas(Backdrop {
                style: self.style,
                prims: self.style.access_backdrop(),
                stretch: true,
                at: self.now.saturating_duration_since(motion::origin()),
            })
            .width(Length::Fill)
            .height(Length::Fill),
            canvas(Art {
                style: self.style,
                input_epoch: self.input_epoch,
                shown: Shown {
                    awake: self.awake,
                    typed: self.secret.len(),
                    phase: self.phase,
                    lit: motion::blink(motion::CARET_BLINK, self.now),
                },
            })
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .into()
    }
}

/// What a key press means to the field. `None` is a key the field
/// has no use for -- a modifier on its own, an arrow, a chord.
fn key_message(key: Key, modifiers: keyboard::Modifiers, text: Option<&str>) -> Option<Message> {
    match key.as_ref() {
        Key::Named(Named::Enter) => return Some(Message::Submit),
        Key::Named(Named::Backspace) => return Some(Message::Backspace),
        Key::Named(Named::Escape) => return Some(Message::Clear),
        Key::Character("u") if modifiers.control() => return Some(Message::Clear),
        _ => {}
    }
    if modifiers.control() || modifiers.alt() || modifiers.logo() {
        return None;
    }
    let text: String = text?.chars().filter(|c| !c.is_control()).collect();
    (!text.is_empty()).then_some(Message::Typed(text))
}

/// The state the art draws with: what the field shows, what the
/// prompt says, and which half of the caret's blink this frame is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Shown {
    awake: bool,
    typed: usize,
    phase: Phase,
    /// `#caret-blink` is in its lit half. Frame 0 is lit.
    lit: bool,
}

impl Shown {
    /// The run the live field carries: the trace's mock until the
    /// keyboard is touched, then the typed count in masks. Without its
    /// tail in the dark half of the blink when the tail is the caret
    /// when a design uses a text tail as its caret.
    fn run(&self, entry: &Entry) -> String {
        let run = if self.awake {
            let mut run: String = std::iter::repeat(entry.mask).take(self.typed).collect();
            run.push_str(entry.tail);
            run
        } else {
            entry.rest.text.to_string()
        };
        if entry.blink == Blink::Tail && !self.lit {
            return run.strip_suffix(entry.tail).unwrap_or(&run).to_string();
        }
        run
    }

    /// The masks alone -- what a trailing caret stands after.
    fn masks(&self, entry: &Entry) -> String {
        std::iter::repeat(entry.mask).take(self.typed).collect()
    }

    fn word(&self, entry: &Entry) -> Option<&'static str> {
        match self.phase {
            Phase::Submitting | Phase::Success => Some(entry.busy),
            Phase::Failed => Some(entry.failed),
            Phase::Idle => None,
        }
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
fn plate_vertices(plate: &Plate) -> Vec<Point> {
    let Plot { x, y, w, h } = plate.at;
    let b = plate.bevel;
    let (top, step) = match plate.step {
        Some(s) => (y + s.drop, Some(s)),
        None => (y, None),
    };

    let mut points = vec![Point::new(x + b.tl, top)];
    if let Some(s) = step {
        points.push(Point::new(s.x, top));
        points.push(Point::new(s.x + s.run, y));
    }
    points.push(Point::new(x + w - b.tr, y));
    if b.tr > 0.0 { points.push(Point::new(x + w, y + b.tr)); }
    points.push(Point::new(x + w, y + h - b.br));
    if b.br > 0.0 { points.push(Point::new(x + w - b.br, y + h)); }
    points.push(Point::new(x + b.bl, y + h));
    if b.bl > 0.0 { points.push(Point::new(x, y + h - b.bl)); }
    points.push(Point::new(x, top + b.tl));
    if b.tl > 0.0 { points.push(Point::new(x + b.tl, top)); }
    points
}

fn plate_path(g: Grid, plate: &Plate) -> canvas::Path {
    canvas::Path::new(|p| {
        if let Some(path) = plate.path {
            p.move_to(g.at(path.start.0, path.start.1));
            for step in path.steps {
                match *step {
                    Seg::Move(x, y) => {
                        if path.close { p.close(); }
                        p.move_to(g.at(x, y));
                    }
                    Seg::Line(x, y) => p.line_to(g.at(x, y)),
                    Seg::Quad { cx, cy, x, y } => p.quadratic_curve_to(g.at(cx, cy), g.at(x, y)),
                    Seg::Cubic { c1x, c1y, c2x, c2y, x, y } =>
                        p.bezier_curve_to(g.at(c1x, c1y), g.at(c2x, c2y), g.at(x, y)),
                }
            }
            if path.close { p.close(); }
            return;
        }
        for (i, point) in plate_vertices(plate).iter().enumerate() {
            if i == 0 { p.move_to(g.at(point.x, point.y)); }
            else { p.line_to(g.at(point.x, point.y)); }
        }
        p.close();
    })
}

/// The same outline used for drawing, including the cut corners and
/// stepped top. A clipped corner must not submit an invisible button.
fn plate_contains(plate: &Plate, at: Point) -> bool {
    use canvas::path::lyon_path::{iterator::PathIterator, Event};
    // Flatten the drawing path to a bounded design-space tolerance,
    // including its closing edges. No invisible rounded corner or
    // shoulder can activate the control's enclosing rectangle.
    let path = plate_path(Grid { sx: 1.0, sy: 1.0 }, plate);
    let mut inside = false;
    for event in path.raw().iter().flattened(0.025) {
        let (a, b) = match event {
            Event::Line { from, to } => (from, to),
            Event::End { last, first, close: true } => (last, first),
            _ => continue,
        };
        if (a.y > at.y) != (b.y > at.y)
            && at.x < (b.x - a.x) * (at.y - a.y) / (b.y - a.y) + a.x
        {
            inside = !inside;
        }
    }
    inside
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

/// The face a legend is set in.
fn font_of(legend: &Legend) -> iced::Font {
    iced::Font {
        family: iced::font::Family::Name("Rajdhani"),
        weight: legend.weight,
        ..iced::Font::DEFAULT
    }
}

/// The width of `content` set as `legend` would set it, in design
/// units: the natural advances plus the legend's tracking between
/// them, measured in the window and scaled back through the grid.
fn run_extent(g: Grid, legend: &Legend, content: &str) -> f32 {
    if content.is_empty() {
        return 0.0;
    }
    let size = g.span(legend.size);
    let advances = advances(content, size, font_of(legend));
    let window = advances.iter().sum::<f32>()
        + g.span(legend.tracking) * (advances.len().max(1) - 1) as f32;
    window * legend.stretch / g.sx
}

fn field_interior(field: &Plate) -> Plot {
    let inset = if field.stroke.is_some() { field.weight / 2.0 } else { 0.0 } + 0.5;
    Plot::new(field.at.x + inset, field.at.y + inset,
        (field.at.w - 2.0 * inset).max(0.0), (field.at.h - 2.0 * inset).max(0.0))
}

/// The displayed suffix has the same masks as the complete secret, but
/// only the count that fits is shaped. Reserve the lit tail/caret even
/// in the dark blink half, so blinking never shifts the displayed run.
/// `Shown` carries only a count; the secret is neither copied nor cut.
fn field_shown(g: Grid, slot: &Slot, shown: Shown) -> Shown {
    let (Some(field), Some(entry)) = (slot.field, slot.entry) else { return shown };
    let interior = field_interior(&field);
    let right = interior.x + interior.w;
    let mut visible = Shown { typed: 0, ..shown };
    let mut previous_width = 0.0;
    for count in 1..=shown.typed {
        let trial = Shown { typed: count, lit: true, ..shown };
        let masks = trial.masks(&entry);
        let width = run_extent(g, &entry.rest, &masks);
        // A missing/zero-advance face must not turn a very long secret
        // into an unbounded shaping job.
        if width <= previous_width { break; }
        previous_width = width;
        let text_right = entry.rest.x + run_extent(g, &entry.rest, &trial.run(&entry));
        let caret_right = slot.caret.filter(|_| matches!(entry.caret, Caret::Trails | Caret::AfterMasks))
            .map_or(text_right, |caret| caret.at.x + width + caret.at.w);
        if text_right.max(caret_right) > right { break; }
        visible.typed = count;
    }
    visible
}

struct Pen<'a> {
    frame: &'a mut canvas::Frame,
    grid: Grid,
    style: &'a Style,
}

fn access_ink(style: &Style, ink: Ink) -> Color {
    if ink == Ink::Fg && style.access_reference_palette() {
        if let Some(reference) = style.access.reference_fg {
            return reference;
        }
    }
    ink.of(&style.palette)
}

fn plate_fill(style: &Style, plate: &Plate) -> Option<Color> {
    let fill = plate.fill?;
    if let Some(reference) = plate.reference_fill {
        if style.access_reference_palette() {
            return Some(reference);
        }
    }
    Some(access_ink(style, fill))
}

fn plate_stroke(style: &Style, plate: &Plate) -> Option<(Color, f32)> {
    let stroke = plate.stroke?;
    if style.access_reference_palette() {
        if let Some(reference) = plate.reference_stroke {
            return Some(reference);
        }
    }
    Some((access_ink(style, stroke), plate.weight))
}

impl Pen<'_> {
    fn ink(&self, ink: Ink) -> Color {
        access_ink(self.style, ink)
    }

    fn plate(&mut self, plate: &Plate) {
        let path = plate_path(self.grid, plate);
        match (plate_fill(&self.style, plate), plate.foot) {
            (Some(fill), None) => self.frame.fill(&path, fill),
            (Some(fill), Some(foot)) => {
                // Neomil's unselected cards are translucent over the
                // screen's glow and so grade darker downward; the trace
                // samples both stops down the card's centre.
                let top = self.grid.at(plate.at.x, plate.at.y);
                let bottom = self.grid.at(plate.at.x, plate.at.y + plate.at.h);
                let gradient = canvas::gradient::Linear::new(top, bottom)
                    .add_stop(0.0, fill)
                    .add_stop(1.0, self.ink(foot));
                self.frame.fill(&path, gradient);
            }
            (None, _) => {}
        }
        if let Some((color, weight)) = plate_stroke(self.style, plate) {
            self.frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(self.grid.span(weight)),
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
        self.legend_text(legend, legend.text);
    }

    /// `legend`'s geometry, face and ink carrying `content` instead of
    /// its own text: the live field's run, the prompt's notice.
    fn legend_text(&mut self, legend: &Legend, content: &str) {
        let g = self.grid;
        let size = g.span(legend.size);
        let color = self.ink(legend.ink);
        let font = font_of(legend);
        let text = canvas::Text {
            content: content.to_string(),
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

            let advances = advances(content, size, font);
            let total: f32 = advances.iter().sum::<f32>()
                + tracking * (advances.len().max(1) - 1) as f32;
            let mut x = if legend.centred { -total / 2.0 } else { 0.0 };
            for (glyph, advance) in content.chars().zip(advances) {
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

struct Art {
    style: Style,
    shown: Shown,
    input_epoch: u64,
}

#[derive(Default)]
struct ActionState {
    pointer: Pointer<()>,
    input_epoch: u64,
}

impl Art {
    fn enabled(&self) -> bool {
        !matches!(self.shown.phase, Phase::Submitting | Phase::Success)
    }

    fn live_slot(&self) -> Option<&Slot> {
        self.style.access.slots.iter().find(|slot| slot.entry.is_some())
    }

    fn target(&self, bounds: Rectangle, cursor: mouse::Cursor) -> Option<()> {
        if !self.enabled() || bounds.width <= 0.0 || bounds.height <= 0.0 { return None; }
        let at = cursor.position_in(bounds)?;
        let grid = Grid::new(bounds.size());
        let action = self.live_slot()?.action?;
        plate_contains(&action, Point::new(at.x / grid.sx, at.y / grid.sy)).then_some(())
    }

    fn pointer_event(&self, state: &mut ActionState, event: &iced::Event,
        bounds: Rectangle, cursor: mouse::Cursor) -> PointerAction<()> {
        if state.input_epoch != self.input_epoch || !self.enabled() {
            state.pointer = Pointer::default();
            state.input_epoch = self.input_epoch;
        }
        state.pointer.sync(self.style.access.slots);
        if !self.enabled() { return PointerAction::Ignore; }
        let target = self.target(bounds, cursor);
        // Leaving the action cancels the press. Returning over it can
        // hover again, but cannot revive a cancelled submission.
        if target.is_none() && matches!(event, iced::Event::Mouse(mouse::Event::CursorMoved { .. })) {
            return state.pointer.event(&iced::Event::Mouse(mouse::Event::CursorLeft), None);
        }
        state.pointer.event(event, target)
    }

    fn action_coat(&self, state: &ActionState) -> Option<Coat> {
        if !self.enabled() { return Some(self.style.controls.disabled); }
        if state.input_epoch != self.input_epoch { return None; }
        let (_, held) = state.pointer.interaction(self.style.access.slots)?;
        let coats = self.style.controls.primary_states;
        if held { coats.pressed } else { coats.hover }
    }
}

impl canvas::Program<Message, Style> for Art {
    type State = ActionState;

    fn update(&self, state: &mut Self::State, event: &iced::Event,
        bounds: Rectangle, cursor: mouse::Cursor) -> Option<canvas::Action<Message>> {
        match self.pointer_event(state, event, bounds, cursor) {
            PointerAction::Ignore => None,
            PointerAction::Redraw => Some(canvas::Action::request_redraw()),
            PointerAction::Capture => Some(canvas::Action::request_redraw().and_capture()),
            PointerAction::Activate(()) => Some(canvas::Action::publish(Message::Submit).and_capture()),
        }
    }

    fn mouse_interaction(&self, _state: &Self::State, bounds: Rectangle,
        cursor: mouse::Cursor) -> mouse::Interaction {
        if self.target(bounds, cursor).is_some() { mouse::Interaction::Pointer }
        else { mouse::Interaction::default() }
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
        // The keyboard goes to the first live slot. Neokitsch offers two
        // (A and B, both with a field); the second stays at rest.
        let mut live = true;
        for slot in access.slots {
            let shown = if live && slot.entry.is_some() {
                live = false;
                Some(&self.shown)
            } else {
                None
            };
            draw_slot(&mut pen, slot, shown, shown.and(self.action_coat(state)));
        }
        colophon(&mut pen, &access.colophon);

        let mut geometry = vec![frame.into_geometry()];
        if self.shown.awake {
            if let Some(slot) = self.live_slot() {
                if let Some(field) = slot.field {
                    let grid = Grid::new(bounds.size());
                    let inner = field_interior(&field);
                    let region = Rectangle::new(grid.at(inner.x, inner.y), grid.size(inner.w, inner.h));
                    if region.width > 0.0 && region.height > 0.0 {
                        // A separate geometry puts the clipped content
                        // above the well: with_clip drafts are pasted
                        // under the direct shapes of their own frame.
                        let shown = field_shown(grid, slot, self.shown);
                        let mut input = canvas::Frame::new(renderer, bounds.size());
                        input.with_clip(region, |frame| {
                            let mut pen = Pen { frame, grid, style: &self.style };
                            draw_entry(&mut pen, slot, Some(&shown));
                        });
                        geometry.push(input.into_geometry());
                    }
                }
            }
        }
        geometry
    }
}

// -------------------------------------------------------------- masthead

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
            art,
            rule,
            labels,
        } => {
            for badge in *badges {
                pen.plate(badge);
            }
            for plate in *art {
                pen.plate(plate);
            }
            pen.plate(rule);
            pen.legends(labels);
        }
        Masthead::Clock { labels } => pen.legends(labels),
        Masthead::Logotype {
            cell,
            divider,
            art,
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
            for plate in *art {
                pen.plate(plate);
            }
            pen.legends(labels);
        }
    }
}

// ------------------------------------------------------------------ slot

/// One slot. `shown` is `Some` on the live slot the keyboard is
/// writing into, and says what its field carries.
fn slot_body<'a>(style: &Style, slot: &'a Slot) -> Option<&'a Plate> {
    if style.access_reference_palette() {
        slot.reference_body.as_ref().or(slot.body.as_ref())
    } else {
        slot.body.as_ref()
    }
}

fn draw_slot(pen: &mut Pen, slot: &Slot, shown: Option<&Shown>, coat: Option<Coat>) {
    if let Some(body) = slot_body(pen.style, slot) {
        pen.plate(body);
    }
    if let Some(foot) = &slot.foot {
        pen.plate(foot);
    }
    if let Some(notch) = &slot.notch {
        pen.plate(notch);
    }
    if let Some(rail) = &slot.notch_rail {
        pen.plate(rail);
    }
    if let Some(tab) = &slot.mark_tab {
        pen.plate(tab);
    }
    if let Some(mark) = &slot.mark {
        pen.plate(mark);
        if slot.emblem_art.is_empty() {
            emblem(pen, slot.emblem, mark.at);
        } else {
            for plate in slot.emblem_art {
                pen.plate(plate);
            }
        }
    }
    if let Some(name) = &slot.name {
        pen.legend(name);
    }
    // What the prompt says: its own text, or -- on the live slot, while
    // greetd is asked and after it refused -- the entry's notice. An
    // era with no prompt (kitsch, neokitsch) puts the notice on its
    // action label instead. No new geometry: the word takes the
    // legend's place in the legend's face.
    let word = shown.zip(slot.entry.as_ref()).and_then(|(s, e)| s.word(e));
    match (&slot.prompt, word) {
        (Some(prompt), Some(word)) => pen.legend_text(prompt, word),
        (Some(prompt), None) => pen.legend(prompt),
        (None, _) => {}
    }
    if let Some(field) = &slot.field {
        pen.plate(field);
    }
    // Live input is a clipped geometry above the painted well. The
    // untouched mock keeps the exact trace's original draw order.
    if !shown.is_some_and(|shown| shown.awake) {
        draw_entry(pen, slot, shown);
    }
    if let Some(action) = &slot.action {
        pen.plate(&coated_action(*action, coat));
    }
    let label = slot.action_label.map(|mut label| {
        if let Some(coat) = coat { label.ink = coat.ink; }
        label
    });
    match (&label, word, slot.prompt.is_some()) {
        (Some(label), Some(word), false) => pen.legend_text(label, word),
        (Some(label), _, _) => pen.legend(label),
        (None, _, _) => {}
    }
    for mark in slot.action_marks {
        pen.plate(mark);
    }
    if let Some(badge) = &slot.badge {
        badge_plate(pen, badge);
    }
    if let Some(letter) = &slot.badge_letter {
        pen.legend(letter);
    }
    pen.legends(slot.notes);
}

fn coated_action(mut action: Plate, coat: Option<Coat>) -> Plate {
    if let Some(coat) = coat {
        action.fill = (coat.fill != Ink::None).then_some(coat.fill);
        action.reference_fill = None;
        action.foot = None;
        action.stroke = (coat.edge != Ink::None).then_some(coat.edge);
        action.reference_stroke = None;
        action.weight = coat.weight;
    }
    action
}

fn draw_entry(pen: &mut Pen, slot: &Slot, shown: Option<&Shown>) {
    if let Some(entry) = &slot.entry {
        match shown {
            Some(shown) => pen.legend_text(&entry.rest, &shown.run(entry)),
            None => pen.legend(&entry.rest),
        }
    }
    if let Some(caret) = entry_caret(pen.grid, slot, shown) {
        pen.plate(&caret);
    }
}

/// Resolve the measured caret for both the untouched fixture and live
/// input. Its travel depends on masks, never on the secret's contents.
fn entry_caret(grid: Grid, slot: &Slot, shown: Option<&Shown>) -> Option<Plate> {
    let dark = shown.zip(slot.entry.as_ref()).is_some_and(|(s, e)| e.blink == Blink::Caret && !s.lit);
    let mut caret = slot.caret.filter(|_| !dark)?;
    if let Some(entry) = &slot.entry {
        if let Some(shown) = shown.filter(|s| s.awake && matches!(entry.caret, Caret::Trails | Caret::AfterMasks)) {
            caret.at.x += run_extent(grid, &entry.rest, &shown.masks(entry));
        } else if entry.caret == Caret::AfterMasks {
            let masks = entry.rest.text.strip_suffix(entry.tail).unwrap_or(entry.rest.text);
            caret.at.x += run_extent(grid, &entry.rest, masks);
        }
    }
    Some(caret)
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
        Fixture::Margins { chips, marks, labels } => {
            for mark in *marks {
                pen.plate(mark);
            }
            for chip in *chips {
                pen.box_at(*chip, Ink::Fg);
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
/// descending from above into independent rounded feet.
///
/// Every figure is `docs/neokitsch/login-trace.svg`'s: the outer
/// plateau spaced 3.9, the centre one tightened to 3.03 so the bends
/// fan, the departure walking right 1.9 a strand and the landing 2.2,
/// mirrored about x=808, and the brightness stepping from 0.30 at the
/// top strand to 1.0 at the bottom.
fn wire_band(pen: &mut Pen, outer: f32, inner: f32, end: f32, strands: usize) {
    const X0: f32 = 34.0;
    const X1: f32 = 1565.0;
    const MIRROR: f32 = 1616.0;
    // Source feet descend from above; `end` is strand zero's terminal.
    let curl = outer - end;
    let control = curl * 0.55228475;
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

    let strand = |p: &mut canvas::path::Builder, i: usize, g: Grid| {
        let (oy, iy, lx, rx) = geometry(i);
        let bow = 0.55 * (rx - lx);
        p.move_to(g.at(X0, oy - curl));
        p.bezier_curve_to(
            g.at(X0, oy - curl + control),
            g.at(X0 + curl - control, oy),
            g.at(X0 + curl, oy),
        );
        p.line_to(g.at(lx, oy));
        p.bezier_curve_to(g.at(lx + bow, oy), g.at(rx - bow, iy), g.at(rx, iy));
        p.line_to(g.at(MIRROR - rx, iy));
        p.bezier_curve_to(
            g.at(MIRROR - rx + bow, iy),
            g.at(MIRROR - lx - bow, oy),
            g.at(MIRROR - lx, oy),
        );
        p.line_to(g.at(X1 - curl, oy));
        p.bezier_curve_to(
            g.at(X1 - curl + control, oy),
            g.at(X1, oy - curl + control),
            g.at(X1, oy - curl),
        );
    };

    // The floor between the strands glows, black at the top strand and
    // rising to the warm brown at the bottom one, with no spill outside
    // the band.
    let g = pen.grid;
    let last = strands.saturating_sub(1);
    let glow = canvas::Path::new(|p| {
        strand(p, 0, g);
        let (oy_last, iy_last, lx_last, rx_last) = geometry(last);
        let bow = 0.55 * (rx_last - lx_last);
        p.line_to(g.at(X1, oy_last - curl));
        p.bezier_curve_to(
            g.at(X1, oy_last - curl + control),
            g.at(X1 - curl + control, oy_last),
            g.at(X1 - curl, oy_last),
        );
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
        p.line_to(g.at(X0 + curl, oy_last));
        p.bezier_curve_to(
            g.at(X0 + curl - control, oy_last),
            g.at(X0, oy_last - curl + control),
            g.at(X0, oy_last - curl),
        );
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
            let path = canvas::Path::new(|p| strand(p, i, g));
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
        let path = canvas::Path::new(|p| strand(p, i, g));
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

    #[test]
    fn ornamental_strokes_stay_open_without_changing_closed_surfaces() {
        use canvas::path::lyon_path::Event;
        const LETTERS: &[Seg] = &[
            Seg::Line(1.0, 9.0), Seg::Line(5.0, 9.0),
            Seg::Move(7.0, 1.0), Seg::Line(9.0, 9.0), Seg::Line(11.0, 1.0),
        ];
        let plate = Plate::outlined(Plot::new(1.0, 1.0, 10.0, 8.0), Ink::Fg, 0.625);
        let endings = |plate| {
            let path = plate_path(Grid { sx: 1.0, sy: 1.0 }, &plate);
            path.raw().iter().filter_map(|event| match event {
                Event::End { close, .. } => Some(close),
                _ => None,
            }).collect::<Vec<_>>()
        };
        assert_eq!(endings(plate.open_path((1.0, 1.0), LETTERS)), [false, false]);
        assert_eq!(endings(plate.outlined_path((1.0, 1.0), LETTERS)), [true, true]);
        assert_eq!(endings(plate), [true]);
    }

    #[test]
    fn source_plate_fill_preserves_custom_palette_roles_and_feedback() {
        use crate::palette::rgb;
        let style = Era::Neomil.style();
        let source = rgb(0xf63333);
        let plate = Plate::filled(Plot::new(0.0, 0.0, 20.0, 20.0), Ink::Fg)
            .reference_fill(source);
        assert_eq!(plate_fill(&style, &plate), Some(source));

        let mut custom = style;
        custom.palette.fg = rgb(0x37c8a0);
        assert_eq!(plate_fill(&custom, &plate), Some(custom.palette.fg));
        // A custom palette that retains the reference foreground is still
        // custom: checking only fg would incorrectly brighten its card.
        custom = style;
        custom.palette.dim = rgb(0x123456);
        assert_eq!(plate_fill(&custom, &plate), Some(style.palette.fg));

        let held = coated_action(plate, Some(Coat::filled(Ink::Select, Ink::OnSelect)));
        assert_eq!(plate_fill(&style, &held), Some(style.palette.select));
        let outlined = coated_action(plate, Some(Coat::outlined(Ink::Dim, 1.0, Ink::Fg)));
        assert_eq!(plate_fill(&style, &outlined), None);
        assert_eq!(plate_fill(&style, &plate.over(Ink::Dim)), Some(style.palette.dim));
    }

    #[test]
    fn source_plate_edges_preserve_custom_weight_and_feedback() {
        use crate::palette::rgb;
        let style = Era::Neomil.style();
        let source = (rgb(0x792a31), 0.75);
        let plate = Plate::outlined(Plot::new(0.0, 0.0, 20.0, 20.0), Ink::Fg, 1.5)
            .reference_edge(source.0, source.1);
        assert_eq!(plate_stroke(&style, &plate), Some(source));
        let mut custom = style;
        custom.palette.panel = rgb(0x183638);
        assert_eq!(plate_stroke(&custom, &plate), Some((custom.palette.fg, 1.5)));
        custom.palette.fg = rgb(0x37c8a0);
        assert_eq!(plate_stroke(&custom, &plate), Some((custom.palette.fg, 1.5)));

        let edged = plate.edged(Ink::Dim, 2.0);
        assert_eq!(plate_stroke(&style, &edged), Some((style.palette.dim, 2.0)));
        let held = coated_action(plate, Some(Coat::outlined(Ink::Select, 3.0, Ink::Fg)));
        assert_eq!(plate_stroke(&style, &held), Some((style.palette.select, 3.0)));
        let filled = coated_action(plate, Some(Coat::filled(Ink::Select, Ink::OnSelect)));
        assert_eq!(plate_stroke(&style, &filled), None);
    }

    #[test]
    fn access_reference_ink_does_not_recolor_custom_themes_or_explicit_plate_fills() {
        use crate::palette::rgb;
        let style = Era::Neomil.style();
        assert_eq!(access_ink(&style, Ink::Fg), rgb(0xf63333));
        assert_eq!(access_ink(&style, Ink::Dim), style.palette.dim);
        let plate = Plate::filled(Plot::new(0.0, 0.0, 10.0, 10.0), Ink::Fg)
            .reference_fill(rgb(0x9c2527));
        assert_eq!(plate_fill(&style, &plate), Some(rgb(0x9c2527)));
        let mut custom = style;
        custom.palette.panel = rgb(0x183638);
        assert_eq!(access_ink(&custom, Ink::Fg), custom.palette.fg);
        assert_eq!(plate_fill(&custom, &plate), Some(custom.palette.fg));
    }

    #[test]
    fn reference_card_material_retains_frames_and_custom_palette_surfaces() {
        use crate::palette::rgb;
        let source = Era::Neomil.style();
        assert_ne!(source.access_backdrop(), source.access.backdrop);
        let mut custom = source;
        custom.palette.panel = rgb(0x183638);
        assert_eq!(custom.access_backdrop(), source.access.backdrop);
        for slot in source.access.slots.iter().skip(1) {
            let body = slot.body.as_ref().expect("inactive card body");
            let reference = slot_body(&source, slot).expect("source frame");
            assert!(reference.fill.is_none(), "material below must stay visible");
            assert_eq!(reference.at, body.at);
            assert_eq!(reference.path, body.path);
            assert_eq!(reference.bevel, body.bevel);
            assert_eq!(reference.step, body.step);
            assert_eq!(reference.stroke, body.stroke);
            assert_eq!(reference.weight, body.weight);
            assert_eq!(slot_body(&custom, slot), Some(body));
        }
        for era in [Era::Entropism, Era::Kitsch, Era::Neokitsch] {
            let style = era.style();
            assert_eq!(style.access_backdrop(), style.access.backdrop);
            for slot in style.access.slots {
                assert_eq!(slot_body(&style, slot), slot.body.as_ref());
            }
        }
    }

    #[test]
    fn published_entropism_footer_uses_source_fill_and_custom_roles_stay_semantic() {
        use crate::palette::rgb;
        let builtin = Era::Entropism.style();
        let Colophon::Band { plate, .. } = builtin.access.colophon else { panic!("footer band"); };
        let mut published = builtin;
        published.palette.panel = rgb(0x181109);
        for style in [builtin, published] {
            assert_eq!(plate_fill(&style, &plate), Some(rgb(0x8aac8c)));
            let mut custom = style;
            custom.palette.select = rgb(0xe8bf63);
            assert_eq!(plate_fill(&custom, &plate), Some(custom.palette.select));
            custom = style;
            custom.palette.fg = rgb(0xe8bf63);
            assert_eq!(plate_fill(&custom, &plate), Some(style.palette.select));
        }
    }

    fn plots(access: &Access) -> Vec<(&'static str, Plot)> {
        let mut out = Vec::new();
        match &access.masthead {
            Masthead::Strip { plate, .. } => out.push(("masthead", plate.at)),
            Masthead::Dossier { badges, art, rule, .. } => {
                for badge in *badges {
                    out.push(("badge", badge.at));
                }
                out.extend(art.iter().map(|plate| ("dossier art", plate.at)));
                out.push(("rule", rule.at));
            }
            Masthead::Clock { .. } => {}
            Masthead::Logotype { cell, art, .. } => {
                out.push(("header cell", cell.at));
                out.extend(art.iter().map(|plate| ("header art", plate.at)));
            }
        }
        for slot in access.slots {
            for (name, plate) in [
                ("body", &slot.body),
                ("reference body", &slot.reference_body),
                ("foot", &slot.foot),
                ("notch", &slot.notch),
                ("notch rail", &slot.notch_rail),
                ("mark", &slot.mark),
                ("mark tab", &slot.mark_tab),
                ("field", &slot.field),
                ("caret", &slot.caret),
                ("action", &slot.action),
                ("badge", &slot.badge),
            ] {
                if let Some(plate) = plate {
                    out.push((name, plate.at));
                }
            }
            for mark in slot.action_marks {
                out.push(("action mark", mark.at));
            }
            out.extend(slot.emblem_art.iter().map(|plate| ("emblem art", plate.at)));
        }
        if let Fixture::Bracket { barcode, .. } = &access.fixture {
            out.push(("barcode", *barcode));
        }
        if let Fixture::Margins { chips, marks, .. } = &access.fixture {
            for chip in chips.iter() {
                out.push(("chip", *chip));
            }
            out.extend(marks.iter().map(|plate| ("margin mark", plate.at)));
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

    /// A field is something you type into, so every slot with one
    /// says how typed input looks, and no slot without one does.
    #[test]
    fn every_field_has_an_entry() {
        for era in Era::ALL {
            let style = era.style();
            for (i, slot) in style.access.slots.iter().enumerate() {
                assert_eq!(
                    slot.field.is_some(),
                    slot.entry.is_some(),
                    "{}: slot {i} has a field without an entry, or the reverse",
                    era.name()
                );
            }
        }
    }

    /// The rest run is the trace's mock of the same field with some
    /// number of characters in it: `mask` repeated, then `tail`. So
    /// typing that many characters reproduces the trace exactly, and
    /// the golden is one state of the live field rather than a picture
    /// the field replaces.
    #[test]
    fn the_rest_run_is_a_typed_run() {
        for era in Era::ALL {
            for slot in era.style().access.slots {
                let Some(entry) = slot.entry else { continue };
                let stars = entry
                    .rest
                    .text
                    .strip_suffix(entry.tail)
                    .unwrap_or_else(|| panic!("{}: rest run does not end in its tail", era.name()));
                assert!(
                    stars.chars().all(|c| c == entry.mask),
                    "{}: rest run {:?} is not masks then tail",
                    era.name(),
                    entry.rest.text
                );
                let shown = Shown {
                    awake: true,
                    typed: stars.chars().count(),
                    phase: Phase::Idle,
                    lit: true,
                };
                assert_eq!(shown.run(&entry), entry.rest.text);
                assert!(!entry.busy.is_empty() && !entry.failed.is_empty());
            }
        }
    }

    #[test]
    fn measured_trailing_caret_preserves_fixture_and_live_input_positions() {
        let slot = Era::Neomil.style().access.slots[0];
        let entry = slot.entry.unwrap();
        assert_eq!(entry.caret, Caret::AfterMasks);
        assert_eq!(entry.tail, "");
        let rest = Shown { awake: false, typed: 0, phase: Phase::Idle, lit: true };
        for size in [Size::new(1600.0, 900.0), Size::new(1537.0, 947.0), Size::new(3840.0, 2160.0)] {
            let grid = Grid::new(size);
            let fixture = entry_caret(grid, &slot, Some(&rest)).unwrap();
            assert_eq!(entry_caret(grid, &slot, None), Some(fixture));
            assert_eq!(entry_caret(grid, &slot, Some(&Shown { lit: false, ..rest })), None);
            let ten = Shown { awake: true, typed: 10, ..rest };
            assert_eq!(entry_caret(grid, &slot, Some(&ten)), Some(fixture));
            let empty = Shown { typed: 0, ..ten };
            let empty_mark = entry_caret(grid, &slot, Some(&empty)).unwrap();
            assert_eq!(empty_mark, slot.caret.unwrap());
            assert!(fixture.at.x > empty_mark.at.x);
            assert_eq!(fixture.at.w, empty_mark.at.w);
            assert_eq!(fixture.at.y, empty_mark.at.y);
            let dark = Shown { lit: false, ..ten };
            assert_eq!(entry_caret(grid, &slot, Some(&dark)), None);
            assert_eq!(dark.run(&entry), ten.run(&entry));
        }
    }


    /// Neomil and kitsch blink measured caret plates without changing
    /// their mask runs; neokitsch has no visible caret.
    #[test]
    fn the_dark_half_hides_what_the_trace_animates() {
        let neomil = Era::Neomil.style().access.slots[0].entry.unwrap();
        assert_eq!(neomil.blink, Blink::Caret);
        let dark = Shown {
            awake: false,
            typed: 0,
            phase: Phase::Idle,
            lit: false,
        };
        assert_eq!(dark.run(&neomil), "**********");
        assert_eq!(Shown { awake: true, typed: 3, ..dark }.run(&neomil), "***");
        assert_eq!(Shown { lit: true, ..dark }.run(&neomil), "**********");

        let kitsch = Era::Kitsch.style().access.slots[0].entry.unwrap();
        assert_eq!(kitsch.blink, Blink::Caret);
        assert_eq!(Shown { awake: true, typed: 2, ..dark }.run(&kitsch), "**");
        for era in Era::ALL {
            for slot in era.style().access.slots {
                if let Some(entry) = slot.entry {
                    // A blinking plate needs a plate to blink.
                    assert!(entry.blink != Blink::Caret || slot.caret.is_some(), "{}", era.name());
                    assert!(entry.blink != Blink::Tail || !entry.tail.is_empty(), "{}", era.name());
                }
            }
        }
    }

    /// Asleep, the field shows the mock whatever the count says;
    /// awake, it shows the count.
    #[test]
    fn asleep_is_the_mock_and_awake_is_the_count() {
        let entry = Era::Neomil.style().access.slots[0].entry.unwrap();
        let asleep = Shown {
            awake: false,
            typed: 0,
            phase: Phase::Idle,
            lit: true,
        };
        assert_eq!(asleep.run(&entry), "**********");
        let awake = Shown {
            awake: true,
            typed: 4,
            phase: Phase::Idle,
            lit: true,
        };
        assert_eq!(awake.run(&entry), "****");
        assert_eq!(awake.masks(&entry), "****");
        let empty = Shown {
            awake: true,
            typed: 0,
            phase: Phase::Idle,
            lit: true,
        };
        assert_eq!(empty.run(&entry), "");
        assert_eq!(empty.word(&entry), None);
        assert_eq!(
            Shown {
                phase: Phase::Failed,
                ..empty
            }
            .word(&entry),
            Some("access denied:")
        );
    }

    #[test]
    fn keys_become_field_messages() {
        use iced::keyboard::Modifiers;
        let none = Modifiers::empty();
        let typed = |m: Option<Message>| match m {
            Some(Message::Typed(t)) => t,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            typed(key_message(Key::Character("a".into()), none, Some("a"))),
            "a"
        );
        assert_eq!(
            typed(key_message(Key::Character("A".into()), Modifiers::SHIFT, Some("A"))),
            "A"
        );
        assert!(matches!(
            key_message(Key::Named(Named::Enter), none, Some("\r")),
            Some(Message::Submit)
        ));
        assert!(matches!(
            key_message(Key::Named(Named::Backspace), none, Some("\u{8}")),
            Some(Message::Backspace)
        ));
        assert!(matches!(
            key_message(Key::Named(Named::Escape), none, Some("\u{1b}")),
            Some(Message::Clear)
        ));
        assert!(matches!(
            key_message(Key::Character("u".into()), Modifiers::CTRL, Some("\u{15}")),
            Some(Message::Clear)
        ));
        assert!(key_message(Key::Character("c".into()), Modifiers::CTRL, None).is_none());
        assert!(key_message(Key::Named(Named::Shift), Modifiers::SHIFT, None).is_none());
        assert!(key_message(Key::Named(Named::ArrowLeft), none, None).is_none());
    }

    /// The demo: keys edit the field, Enter empties it, and the secret
    /// is never in a message's `Debug`.
    #[test]
    fn the_demo_edits_and_enter_clears() {
        let mut login = Login::new(Era::Entropism.style());
        assert!(!login.awake);
        let _ = login.update(Message::Typed("ab".into()));
        let _ = login.update(Message::Typed("c".into()));
        assert!(login.awake);
        assert_eq!(login.typed(), 3);
        let _ = login.update(Message::Backspace);
        assert_eq!(login.typed(), 2);
        let _ = login.update(Message::Submit);
        assert_eq!(login.typed(), 0);
        assert_eq!(login.phase(), Phase::Idle);
        let _ = login.update(Message::Typed("z".into()));
        let _ = login.update(Message::Clear);
        assert_eq!(login.typed(), 0);
        assert_eq!(format!("{:?}", Message::Typed("hunter2".into())), "Typed(..)");
    }

    /// The greeter: Enter takes the secret away and waits; a refusal
    /// shows until the next key; keys while waiting are dropped.
    #[test]
    fn the_greeter_waits_and_shows_a_refusal() {
        let mut login = Login::greeting(
            Era::Kitsch.style(),
            Some(Greeter {
                user: "mverte".into(),
                cmd: vec!["true".into()],
            }),
        );
        let _ = login.update(Message::Typed("x".into()));
        let _ = login.update(Message::Submit);
        assert_eq!(login.phase(), Phase::Submitting);
        assert_eq!(login.typed(), 0, "the secret left with the request");
        let _ = login.update(Message::Typed("y".into()));
        assert_eq!(login.typed(), 0, "keys are ignored while greetd is asked");
        let _ = login.update(Message::Outcome(Err(Refusal::Denied("no".into()))));
        assert_eq!(login.phase(), Phase::Failed);
        let _ = login.update(Message::Typed("y".into()));
        assert_eq!(login.phase(), Phase::Idle);
        assert_eq!(login.typed(), 1);
        let _ = login.update(Message::Outcome(Ok(())));
        assert_eq!(login.phase(), Phase::Success);
    }

    fn art(login: &Login) -> Art {
        Art {
            style: login.style,
            input_epoch: login.input_epoch,
            shown: Shown { awake: login.awake, typed: login.typed(), phase: login.phase, lit: true },
        }
    }

    fn action_cursor(art: &Art, bounds: Rectangle) -> mouse::Cursor {
        let action = art.live_slot().unwrap().action.unwrap().at;
        let grid = Grid::new(bounds.size());
        let local = grid.at(action.x + action.w / 2.0, action.y + action.h / 2.0);
        mouse::Cursor::Available(Point::new(bounds.x + local.x, bounds.y + local.y))
    }

    #[test]
    fn long_unicode_input_only_limits_the_display_and_recovers_when_deleted() {
        let secret = "é中🦀".repeat(80);
        for era in Era::ALL {
            let mut login = Login::new(era.style());
            let _ = login.update(Message::Typed(secret.clone()));
            let slot = login.style.access.slots.iter().find(|slot| slot.entry.is_some()).unwrap();
            let entry = slot.entry.unwrap();
            for size in [Size::new(1600.0, 900.0), Size::new(1237.0, 697.0), Size::new(3840.0, 2160.0)] {
                let grid = Grid::new(size);
                let shown = art(&login).shown;
                let visible = field_shown(grid, slot, shown);
                assert!(visible.typed > 0 && visible.typed < login.typed(), "{}", era.name());
                let inside = field_interior(&slot.field.unwrap());
                let right = inside.x + inside.w;
                assert!(entry.rest.x + run_extent(grid, &entry.rest, &visible.run(&entry)) <= right);
                if let Some(caret) = slot.caret.filter(|_| matches!(entry.caret, Caret::Trails | Caret::AfterMasks)) {
                    assert!(caret.at.x + run_extent(grid, &entry.rest, &visible.masks(&entry)) + caret.at.w <= right);
                }
                let dark = field_shown(grid, slot, Shown { lit: false, ..shown });
                assert_eq!(visible.typed, dark.typed, "blink must not change display capacity");
            }
            assert_eq!(login.secret.expose(), secret, "layout must retain the complete Unicode secret");
            for _ in 0..secret.chars().count() - 2 { let _ = login.update(Message::Backspace); }
            assert_eq!(login.secret.expose(), "é中");
            let slot = art(&login).live_slot().copied().unwrap();
            assert_eq!(field_shown(Grid::new(Size::new(1600.0, 900.0)), &slot, art(&login).shown).typed, 2);
            let _ = login.update(Message::Clear);
            assert_eq!(login.typed(), 0);
            assert_eq!(field_shown(Grid::new(Size::new(1600.0, 900.0)), &slot, art(&login).shown).typed, 0);
        }
    }

    #[test]
    fn pointer_submission_uses_the_same_message_and_demo_semantics_as_enter() {
        use canvas::Program;
        let mut login = Login::new(Era::Neomil.style());
        let _ = login.update(Message::Typed("é中🦀".repeat(40)));
        let art = art(&login);
        let bounds = Rectangle::new(Point::new(13.0, 27.0), Size::new(1237.0, 697.0));
        let cursor = action_cursor(&art, bounds);
        let mut state = ActionState::default();
        assert_eq!(art.mouse_interaction(&state, bounds, cursor), mouse::Interaction::Pointer);
        let down = iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let up = iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let (message, _, status) = art.update(&mut state, &down, bounds, cursor).unwrap().into_inner();
        assert!(message.is_none());
        assert_eq!(status, iced::event::Status::Captured);
        assert_eq!(login.typed(), 120, "press must not submit");
        let (message, _, status) = art.update(&mut state, &up, bounds, cursor).unwrap().into_inner();
        assert!(matches!(message, Some(Message::Submit)));
        assert_eq!(status, iced::event::Status::Captured);
        let _ = login.update(message.unwrap());
        assert_eq!(login.typed(), 0);
        assert_eq!(login.phase, Phase::Idle);
        assert!(art.update(&mut state, &up, bounds, cursor).is_none(), "release submits once");
    }

    #[test]
    fn pointer_release_outside_drag_and_focus_loss_cancel_submission() {
        let login = Login::new(Era::Neomil.style());
        let art = art(&login);
        let bounds = Rectangle::with_size(Size::new(1600.0, 900.0));
        let cursor = action_cursor(&art, bounds);
        let outside = mouse::Cursor::Available(Point::new(900.0, 700.0));
        let down = iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let up = iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let mut state = ActionState::default();
        assert_eq!(art.pointer_event(&mut state, &down, bounds, cursor), PointerAction::Capture);
        assert_eq!(art.pointer_event(&mut state, &up, bounds, outside), PointerAction::Capture);
        assert_eq!(art.pointer_event(&mut state, &up, bounds, cursor), PointerAction::Ignore);
        for cancel in [
            iced::Event::Mouse(mouse::Event::CursorLeft),
            iced::Event::Window(iced::window::Event::Unfocused),
            iced::Event::Mouse(mouse::Event::CursorMoved { position: Point::new(900.0, 700.0) }),
        ] {
            art.pointer_event(&mut state, &down, bounds, cursor);
            art.pointer_event(&mut state, &cancel, bounds, outside);
            art.pointer_event(&mut state, &iced::Event::Mouse(mouse::Event::CursorMoved {
                position: cursor.position().unwrap(),
            }), bounds, cursor);
            assert_eq!(art.pointer_event(&mut state, &up, bounds, cursor), PointerAction::Ignore);
        }
    }

    #[test]
    fn disabled_and_keyboard_transitions_discard_held_gestures() {
        use canvas::Program;
        let mut login = Login::new(Era::Neomil.style());
        let bounds = Rectangle::with_size(Size::new(1600.0, 900.0));
        let cursor = action_cursor(&art(&login), bounds);
        let down = iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let up = iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        let mut state = ActionState::default();
        art(&login).pointer_event(&mut state, &down, bounds, cursor);
        let _ = login.update(Message::Typed("x".into()));
        assert!(art(&login).action_coat(&state).is_none());
        assert_eq!(art(&login).pointer_event(&mut state, &up, bounds, cursor), PointerAction::Ignore);
        art(&login).pointer_event(&mut state, &down, bounds, cursor);
        // Simulate the phase while a greetd worker owns the request;
        // no real socket or credential is needed for this event test.
        login.phase = Phase::Submitting;
        login.input_epoch += 1;
        assert_ne!(art(&login).mouse_interaction(&state, bounds, cursor), mouse::Interaction::Pointer);
        assert_eq!(art(&login).action_coat(&state), Some(login.style.controls.disabled));
        assert_eq!(art(&login).pointer_event(&mut state, &down, bounds, cursor), PointerAction::Ignore);
        assert_eq!(art(&login).pointer_event(&mut state, &up, bounds, cursor), PointerAction::Ignore);
        let _ = login.update(Message::Typed("ignored".into()));
        assert_eq!(login.typed(), 1);
        let _ = login.update(Message::Outcome(Err(Refusal::Denied("synthetic".into()))));
        assert_eq!(art(&login).pointer_event(&mut state, &up, bounds, cursor), PointerAction::Ignore);
        // An entire disabled interval can pass without canvas events.
        art(&login).pointer_event(&mut state, &down, bounds, cursor);
        login.phase = Phase::Submitting;
        login.input_epoch += 1;
        let _ = login.update(Message::Outcome(Err(Refusal::Denied("synthetic".into()))));
        assert_eq!(art(&login).pointer_event(&mut state, &up, bounds, cursor), PointerAction::Ignore);
        art(&login).pointer_event(&mut state, &down, bounds, cursor);
        assert_eq!(art(&login).pointer_event(&mut state, &up, bounds, cursor), PointerAction::Activate(()));
    }

    #[test]
    fn rounded_login_shoulder_rejects_gap_and_cancels_drag_out_at_any_scale() {
        let login = Login::new(Era::Kitsch.style());
        let art = art(&login);
        let action = art.live_slot().unwrap().action.unwrap();
        for size in [Size::new(1600.0, 900.0), Size::new(1537.0, 947.0), Size::new(3840.0, 2160.0)] {
            let bounds = Rectangle { x: 17.0, y: 23.0, ..Rectangle::with_size(size) };
            let grid = Grid::new(size);
            let cursor = |x, y| {
                let at = grid.at(x, y);
                mouse::Cursor::Available(Point::new(bounds.x + at.x, bounds.y + at.y))
            };
            // The gap occupies part of the action's enclosing rectangle;
            // its rounded upper-left corner is also visibly empty.
            for (x, y) in [(300.0, 466.0), (500.0, 459.0), (257.1, 470.5)] {
                assert_eq!(art.target(bounds, cursor(x, y)), None);
            }
            let inside = cursor(500.0, 480.0);
            assert_eq!(art.target(bounds, inside), Some(()));
            let down = iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
            let up = iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
            let mut state = ActionState::default();
            art.pointer_event(&mut state, &down, bounds, inside);
            let gap = cursor(300.0, 466.0);
            art.pointer_event(&mut state, &iced::Event::Mouse(mouse::Event::CursorMoved {
                position: gap.position().unwrap(),
            }), bounds, gap);
            assert_ne!(art.pointer_event(&mut state, &up, bounds, inside), PointerAction::Activate(()));
            art.pointer_event(&mut state, &down, bounds, inside);
            assert_eq!(art.pointer_event(&mut state, &up, bounds, inside), PointerAction::Activate(()));
        }
        for coat in [login.style.controls.primary_states.hover, login.style.controls.primary_states.pressed,
            Some(login.style.controls.disabled)] {
            assert_eq!(coated_action(action, coat).path, action.path);
        }
    }

    #[test]
    fn only_the_live_action_is_hit_and_feedback_preserves_its_outline() {
        let login = Login::new(Era::Neomil.style());
        let art = art(&login);
        let bounds = Rectangle::with_size(Size::new(1600.0, 900.0));
        let action = art.live_slot().unwrap().action.unwrap();
        let cut = Point::new(action.at.x + action.at.w - 1.0, action.at.y + action.at.h - 1.0);
        assert_eq!(art.target(bounds, mouse::Cursor::Available(cut)), None);
        for slot in login.style.access.slots.iter().skip(1) {
            if let Some(action) = slot.action {
                let at = action.at;
                assert_eq!(art.target(bounds, mouse::Cursor::Available(Point::new(at.x + at.w / 2.0, at.y + at.h / 2.0))), None);
            }
        }
        let mut state = ActionState::default();
        assert!(art.action_coat(&state).is_none());
        let cursor = action_cursor(&art, bounds);
        art.pointer_event(&mut state, &iced::Event::Mouse(mouse::Event::CursorMoved {
            position: cursor.position().unwrap(),
        }), bounds, cursor);
        assert_eq!(art.action_coat(&state), login.style.controls.primary_states.hover);
        art.pointer_event(&mut state, &iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)), bounds, cursor);
        assert_eq!(art.action_coat(&state), login.style.controls.primary_states.pressed);
        for coat in [login.style.controls.primary_states.hover, login.style.controls.primary_states.pressed,
            Some(login.style.controls.disabled)] {
            assert_eq!(plate_vertices(&action), plate_vertices(&coated_action(action, coat)));
        }
    }
}
