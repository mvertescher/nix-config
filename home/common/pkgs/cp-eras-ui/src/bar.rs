//! The status bar, in any era.
//!
//! This is the screen that made building our own bar worth it. waybar
//! styles with CSS and ashell with a closed `Islands | Solid | Gradient`
//! enum, and neither can express a chamfered or clipped corner -- so on
//! the one surface that is always visible, two of the four eras could
//! never look like themselves. Here a module is a [`Surface`], so it
//! wears whatever `Corner` the era declares, for free.
//!
//! Everything here is a pure function of [`Style`] and [`Readings`].
//! Collecting the readings is the binary's job, which keeps this file
//! testable and keeps the layer-shell dependency out of the library.

use crate::screens::scene::SoftCache;
use crate::style::{
    BarChrome, BarGround, BarOrnament, Dress, Face, Ink, MenuMarker, MenuRule, PanelEcho, Prim,
    Selection, Style, Tab, Ticket,
};
use crate::widgets::surface::{layered, outline, span_at, Corners, Cut, Fill, Surface};
use iced::widget::canvas::{Frame, Path, Stroke};
use iced::widget::image;
use iced::widget::{canvas, container, mouse_area, row, stack, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Size, Theme};


/// One workspace as the compositor reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub id: i32,
    pub active: bool,
}

/// The default audio sink, as the sound server reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Audio {
    /// Whole percent of the server's notion of normal volume. May
    /// exceed 100: PulseAudio allows amplification, and a bar that
    /// clamps it silently is lying about the state of the machine.
    pub volume: u16,
    pub muted: bool,
}

/// How the machine reaches the outside world.
///
/// `Unknown` and `Offline` are deliberately not the same variant. A bar
/// that announces "offline" because its first probe has not landed yet
/// is worse than one that shows nothing for a second, so `Unknown`
/// draws no module at all.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Network {
    #[default]
    Unknown,
    /// Nothing holds a default route.
    Offline,
    Wired {
        interface: String,
    },
    /// `ssid` is empty when the name was not cheaply available; the
    /// interface stands in for it rather than the module vanishing.
    Wireless {
        interface: String,
        ssid: String,
    },
}

/// One tray icon, as StatusNotifierItem describes it.
///
/// Deliberately not the protocol's own shape. `IconName`,
/// `IconPixmap`, `AttentionIconName`, `OverlayIconName` and
/// `IconThemePath` are five ways of saying one thing -- *these pixels*
/// -- and resolving between them means an icon-theme lookup, a PNG
/// decoder and an SVG rasteriser. All of that is the binary's problem
/// (`examples/bar/tray.rs`, `examples/bar/icon.rs`); what arrives here
/// is already decoded, already composited with its overlay, and
/// already the right size, so `bar()` stays a pure function of what it
/// is handed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayItem {
    /// Four characters of the item's own name, drawn when no icon
    /// could be resolved. Not a placeholder for one that is coming:
    /// plenty of items name an icon no installed theme has, and a cell
    /// that says `SYNC` is worth more than a blank one.
    pub label: String,
    /// The resolved icon, RGBA, or `None` to fall back to [`label`].
    ///
    /// [`label`]: TrayItem::label
    pub icon: Option<image::Handle>,
    /// The item is asking to be looked at (`Status = NeedsAttention`).
    /// The host has already swapped in `AttentionIcon*` where the item
    /// offered one; this is what makes the *cell* shout as well.
    pub attention: bool,
}

/// What a pointer did to a tray cell.
///
/// Named for the protocol methods rather than for the buttons, because
/// the mapping from button to method is the host's convention and the
/// method names are the fixed part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    /// `Activate`. Left button, the usual "open this application".
    Activate,
    /// `SecondaryActivate`. Middle button.
    ///
    /// Reachable on a layer surface as of `iced_layershell` 0.19. Up to
    /// 0.13.7 that crate mapped every Wayland button code except
    /// `BTN_RIGHT` to `mouse::Button::Left` (`src/event.rs`), so a
    /// middle click on `cp-eras-ui-bar` arrived as
    /// [`Activate`](TrayAction::Activate); this was wired anyway
    /// because `bar()` is not a layer-shell function and was correct in
    /// an ordinary window even then. 0.19.1's `src/event.rs` maps `274`
    /// to `mouse::Button::Middle`, which is the third arm that comment
    /// was waiting for. Still unexercised against a running item.
    Secondary,
    /// `ContextMenu`. Right button.
    Context,
    /// `Scroll` along the vertical axis, in whole wheel detents, never
    /// zero.
    ///
    /// The sign is whatever the host's scroll event carried, passed
    /// through rather than normalised, and it is the one thing here
    /// that has *not* been checked against a running item: no
    /// StatusNotifierItem on this desktop acts on `Scroll`, and the
    /// virtual-pointer tools available for driving one produce the same
    /// sign in both directions.
    ///
    /// An earlier version of this comment claimed the two hosts of
    /// `bar()` disagree, on the grounds that `iced_layershell` forwards
    /// the raw `wl_pointer` axis. **That is wrong.** Its `src/event.rs`
    /// negates on all four paths -- `-vertical.discrete`,
    /// `-vertical.absolute` and the horizontal pair -- so it already
    /// matches iced's own `ScrollDelta` convention, positive away from
    /// the user. That was true of 0.13.7 and is still true of 0.19.1.
    /// What remains unverified is only whether a real item interprets a
    /// detent the way we send it.
    Scroll(i32),
}

/// One row of an item's `com.canonical.dbusmenu`.
///
/// Deliberately not the protocol's own shape, for the same reason
/// [`TrayItem`] is not: a menu row over that interface is a numeric id
/// and a bag of thirty optional properties, of which a bar draws six.
/// Reading them is the host's problem (`examples/bar/tray.rs`); what
/// arrives here is already a row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuEntry {
    /// The dbusmenu id, sent back with `Event` when the row is
    /// clicked. Meaningless to the bar and never displayed -- though a
    /// [`Submenu`](MenuKind::Submenu) row's id is also what the host
    /// sends `AboutToShow` for when the row is opened.
    pub id: i32,
    pub label: String,
    /// `enabled = false`. Drawn in the quiet ink and not clickable,
    /// rather than hidden: an application greys a row to say the
    /// command exists and is not available now, and dropping it would
    /// say something else.
    pub enabled: bool,
    pub kind: MenuKind,
    /// The row's own icon, already decoded and already the size
    /// [`menu_icon_size`] asks for.
    ///
    /// `icon-name` and `icon-data` are two ways of saying *these
    /// pixels*, and resolving between them is the same icon-theme
    /// search and the same PNG decoder a tray cell needs -- so it
    /// happens in the host, for exactly the reason [`TrayItem::icon`]
    /// does.
    pub icon: Option<image::Handle>,
    /// The rows of this row's submenu; empty for every other kind.
    ///
    /// A tree rather than a fetch-by-id, because `GetLayout` hands the
    /// whole thing over in one reply: by the time a panel is on screen
    /// its submenus are data the host already has.
    pub children: Vec<MenuEntry>,
}

/// What kind of row it is. The protocol's `type`, `toggle-type`,
/// `toggle-state` and `children-display` collapse to this, because
/// those are four ways of describing one thing: what the row does when
/// it is clicked and what it looks like before it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuKind {
    /// A plain command.
    Command,
    /// A rule between groups. Carries no id worth dispatching.
    Separator,
    /// A checkmark or a radio, and whether it is set. Drawn as the
    /// era's own *selection* rather than as a tick glyph: selection is
    /// the thing the era vocabulary already has four opinions about,
    /// and a `[x]` would be the same box in all four.
    Toggle(bool),
    /// A row whose [`children`](MenuEntry::children) open beside it.
    /// See [`tray_menu`] for where they are drawn and why there.
    Submenu,
}

/// An item's context menu: a tree of rows, however deep the item nests
/// them.
///
/// Which branch is *open* is deliberately not here. That is state of
/// the panel and not of the item -- it moves when nothing about the
/// menu has -- so it travels beside this as a [`MenuPath`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrayMenu {
    pub entries: Vec<MenuEntry>,
}

/// Which submenu of a [`TrayMenu`] is open, as indices into successive
/// [`MenuEntry::children`] vectors. Empty is the root panel alone.
///
/// Indices rather than dbusmenu ids because the index *is* the
/// geometry: how far down its panel the *n*th row sits is what places
/// the panel hanging off it, and an id would have to be searched for
/// to get there. Safe because a menu on screen is a snapshot, and
/// nothing edits it while it is up.
pub type MenuPath = Vec<usize>;

/// Characters of a menu label drawn before it is clipped.
///
/// An application chooses its own labels and some of them are
/// sentences; the panel is placed against the pointer, so one long row
/// would push the whole menu off the side of the screen rather than
/// just looking untidy.
const MENU_LABEL_CHARS: usize = 40;

/// Air above and below a separator, which is a rule rather than a row.
///
/// The one figure in the menu that is still a constant: all four
/// designs put their rule 4px clear of the rows either side, so there
/// is nothing for the era table to disagree about. Everything else a
/// row measures now comes from [`crate::style::BarMenu`].
const MENU_RULE_AIR: f32 = 4.0;

/// Resolve a named ink, falling back to the era's body colour.
///
/// [`Ink::None`] is the caller's problem rather than this function's:
/// it means "draw nothing", and everywhere it can appear the drawing
/// code asks [`Style::ink`] directly and matches on the `Option`.
fn ink_of(style: &Style, ink: Ink) -> Color {
    style.ink(ink).unwrap_or(style.palette.fg)
}

/// The font one of the era's bar labels is set in.
fn era_face(style: &Style, bold: bool) -> iced::Font {
    if bold {
        return font_of(Face::Bold);
    }
    font_of(style.bar.face)
}

/// The loaded Rajdhani file a [`Face`] names.
fn font_of(face: Face) -> iced::Font {
    use crate::fonts::{
        FONT_RAJDHANI_BOLD, FONT_RAJDHANI_MEDIUM, FONT_RAJDHANI_REGULAR, FONT_RAJDHANI_SEMIBOLD,
    };
    match face {
        Face::Regular => FONT_RAJDHANI_REGULAR,
        Face::Medium => FONT_RAJDHANI_MEDIUM,
        // A true 600 only if the binary loaded `RAJDHANI_SEMIBOLD`;
        // otherwise the shaper hands back Bold (see `fonts.rs`).
        Face::SemiBold => FONT_RAJDHANI_SEMIBOLD,
        Face::Bold => FONT_RAJDHANI_BOLD,
    }
}

/// A bar label, in the era's face and one of its inks.
fn ink_text<'a>(
    style: &Style,
    ink: Ink,
    bold: bool,
    label: impl Into<String>,
) -> iced::widget::Text<'a> {
    iced::widget::text(label.into())
        .size(f32::from(style.metrics.text_body))
        .color(ink_of(style, ink))
        .font(era_face(style, bold))
}

/// How far inside the bar's padding the module row starts: half a
/// stroke under [`BarChrome::Frame`], where the modules are segments of
/// a frame measured from its centreline, and nothing where they stand
/// loose.
fn frame_edge(b: &crate::style::Bar) -> f32 {
    match b.chrome {
        BarChrome::Frame => b.stroke / 2.0,
        BarChrome::Loose => 0.0,
    }
}

/// The [`Surface`] a [`Dress`] describes.
///
/// [`Ink::Select`] goes through [`Surface::selected`] rather than
/// through [`Style::ink`], which is what keeps neokitsch's veneer out
/// of this file: an era whose selection is a material says so once, in
/// [`crate::style::Selection`], and every dress that names `Select`
/// gets it.
fn face_of(style: &Style, dress: &Dress) -> Surface {
    let fill = match dress.fill {
        Ink::None => Fill::None,
        Ink::Select => Surface::selected(style).fill,
        ink => Fill::Solid(ink_of(style, ink)),
    };
    Surface {
        corners: dress.corners,
        fill,
        stroke: style.ink(dress.stroke),
        stroke_width: style.bar.stroke,
        ticket: Ticket::default(),
    }
}

/// The trapezoid tab of [`crate::style::Tab`], standing on the bottom
/// edge of a box `w` by `h`.
fn tab_path(tab: Tab, w: f32, h: f32, stroke: f32) -> Path {
    let (base, top) = if w < tab.narrow_below {
        (tab.narrow_base, tab.narrow_top)
    } else {
        (tab.base, tab.top)
    };
    let right = (w - tab.inset).min(w);
    let left = right - base;
    let shoulder = (base - top) / 2.0;
    // Inside the outline and standing on the bottom stroke, as on the
    // store nav's RIFLES: the base sits on the stroke's *inner* edge,
    // so the outline stays an unbroken loop under it.
    let y1 = h - stroke;
    let y0 = y1 - tab.height;
    Path::new(|b| {
        b.move_to(Point::new(left, y1));
        b.line_to(Point::new(left + shoulder, y0));
        b.line_to(Point::new(right - shoulder, y0));
        b.line_to(Point::new(right, y1));
        b.close();
    })
}

/// A module whose bottom edge steps, which [`Surface`] cannot draw.
///
/// Only kitsch asks for one, and only for its two boxes -- the USER
/// tape and the DESCRIPTION window label -- both of which are a flat
/// fill or a flat outline, so nothing here has to know about veneer.
/// `widgets/surface.rs` is not ours to grow a step on; see the report.
#[derive(Debug, Clone, Copy)]
struct Stepped {
    /// `(run, rise)`.
    step: (f32, f32),
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: f32,
}

impl<Message> canvas::Program<Message> for Stepped {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        const DIAGONAL: f32 = 6.0;
        let mut frame = Frame::new(renderer, bounds.size());
        let inset = if self.stroke.is_some() {
            self.stroke_width / 2.0
        } else {
            0.0
        };
        let (w, h) = (bounds.width - inset * 2.0, bounds.height - inset * 2.0);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        frame.translate(iced::Vector::new(inset, inset));

        let (run, rise) = self.step;
        let run = run.min((w - DIAGONAL).max(0.0));
        let path = Path::new(|b| {
            b.move_to(Point::new(0.0, 0.0));
            b.line_to(Point::new(w, 0.0));
            b.line_to(Point::new(w, h - rise));
            b.line_to(Point::new(run + DIAGONAL, h - rise));
            b.line_to(Point::new(run, h));
            b.line_to(Point::new(0.0, h));
            b.close();
        });
        if let Some(fill) = self.fill {
            frame.fill(&path, fill);
        }
        if let Some(color) = self.stroke {
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(color)
                    .with_width(self.stroke_width),
            );
        }
        vec![frame.into_geometry()]
    }
}

/// The canvas that paints a module's silhouette, whichever of the two
/// shapes it is, outlined at `stroke` -- the bar's everywhere but the
/// window box, which may carry its own
/// ([`crate::style::WindowLabel::stroke`]).
fn face_canvas<'a, Message: 'static>(
    style: &Style,
    dress: &Dress,
    stroke: f32,
) -> Element<'a, Message> {
    match dress.step {
        None => canvas(Surface {
            stroke_width: stroke,
            ..face_of(style, dress)
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
        Some(step) => canvas(Stepped {
            step,
            fill: match dress.fill {
                Ink::None => None,
                ink => Some(ink_of(style, ink)),
            },
            stroke: style.ink(dress.stroke),
            stroke_width: stroke,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
    }
}

/// The trim a module carries on top of its surface: neokitsch's tab and
/// neomil's barcode ticks.
///
/// A second canvas rather than part of [`Surface`], because neither is
/// a property of the *shape* -- the tab rides the outline it is given
/// and the ticks are cargo. `widgets/surface.rs` is not ours to grow a
/// tab field on, so this is the bar-local version; see the report.
#[derive(Debug, Clone, Copy)]
struct Trim {
    tab: Option<(Tab, Color)>,
    /// The five ticks of the neomil code tape, in this ink.
    ticks: Option<Color>,
    /// Neokitsch's book-match grain: hairlines at 1.7px pitch and a
    /// seam down the plate's midpoint, clipped to the silhouette.
    ///
    /// On top of the veneer [`Surface`] already synthesises, not
    /// instead of it: that one is a warp gradient with grain at a 5px
    /// pitch, and the design measures 14 lines on a 25px plate. The
    /// dense pass is what makes a plate read as a plank at bar scale
    /// rather than as a brown fill.
    grain: Option<(Corners, Color)>,
    stroke: f32,
}

impl Trim {
    fn is_empty(&self) -> bool {
        self.tab.is_none() && self.ticks.is_none() && self.grain.is_none()
    }

    /// Whether this dress is a plate of the era's selection material.
    fn grain_of(style: &Style, dress: &Dress) -> Option<(Corners, Color)> {
        match (style.selection, dress.fill) {
            (Selection::Veneer, Ink::Select) => Some((
                dress.corners,
                style.palette.relief().0,
            )),
            _ => None,
        }
    }
}

impl<Message> canvas::Program<Message> for Trim {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }

        if let Some((tab, color)) = self.tab {
            frame.fill(&tab_path(tab, w, h, self.stroke), color);
        }

        if let Some((corners, ink)) = self.grain {
            const PITCH: f32 = 1.7;
            let mut y = PITCH;
            let mut n = 0u32;
            while y < h {
                // The wobble is deterministic: a plank is not a ruled
                // page, and a golden has to be byte-identical run to
                // run.
                let wobble = ((n * 7 % 5) as f32 - 2.0) * 0.45;
                let yy = (y + wobble).clamp(0.0, h);
                let (x0, x1) = span_at(corners, Ticket::default(), w, h, yy);
                if x1 - x0 > 2.0 {
                    frame.stroke(
                        &Path::new(|b| {
                            b.move_to(Point::new(x0 + 1.0, yy));
                            b.line_to(Point::new(x1 - 1.0, yy));
                        }),
                        Stroke::default()
                            .with_color(Color { a: 0.5, ..ink })
                            .with_width(0.6),
                    );
                }
                y += PITCH;
                n += 1;
            }
            // The book-match seam at the plate's midpoint, which is
            // where the mailbox bar's 32 hairlines chevron into each
            // other. The trace draws it 0.9 wide; rsvg puts that on one
            // pixel and an antialiased canvas spreads it over two at
            // half strength, which on a grained plank is nothing at
            // all -- so it is drawn at 1.5, the width that lands a
            // whole pixel on the line.
            frame.stroke(
                &Path::new(|b| {
                    b.move_to(Point::new(w / 2.0, 0.0));
                    b.line_to(Point::new(w / 2.0, h));
                }),
                Stroke::default()
                    .with_color(Color { a: 0.8, ..ink })
                    .with_width(1.5),
            );
        }

        if let Some(color) = self.ticks {
            // login-trace's code tape, scaled to 25: five ticks of
            // 2 / 1.5 / 3 / 1.5 / 2 starting 9 in, 13 tall.
            for &(x, tw) in &[(9.0, 2.0), (13.0, 1.5), (16.5, 3.0), (21.5, 1.5), (25.0, 2.0)] {
                frame.fill(
                    &Path::rectangle(Point::new(x, 6.0), Size::new(tw, (h - 12.0).max(1.0))),
                    color,
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

/// The strip itself: its ground, its chrome, and whatever ornament the
/// era hangs on it. Drawn on one canvas beneath every module.
///
/// It is handed the run of module widths rather than measuring them,
/// because a divider sits *on* a module boundary and iced has already
/// forgotten where those were by the time anything is drawn. The left
/// run accumulates from the strip's left padding and the right run from
/// its right, which is also what makes the tray keep a fixed distance
/// from the edge of the screen.
#[derive(Debug, Clone)]
struct Strip {
    ground: BarGround,
    chrome: BarChrome,
    ornament: BarOrnament,
    chrome_ink: Color,
    ornament_ink: Color,
    stroke: f32,
    pad_left: f32,
    pad_right: f32,
    pad_y: f32,
    /// `(gap before, width)` per module, in document order.
    left: Vec<(f32, f32)>,
    right: Vec<(f32, f32)>,
}

/// The strip's ground when it is a [`BarGround::Haze`]: the era's
/// screen ground composited by `screens::soft` at the strip's own
/// pixels, one image, cached the way a scene's backdrop is.
///
/// Its own canvas under [`Strip`] rather than a draw in it, for the
/// reason `screens::scene::Scene::view` splits: a canvas layer draws
/// its meshes, then its images, so a composite in the strip's canvas
/// would sit over the dividers and the wire.
///
/// The frame-to-canvas scale is 1: the bar is laid out in design
/// pixels (`bar.svg` is the 1600x220 the harness captures at) and its
/// ground has to line up with the modules to the pixel, where a
/// `scene::Backdrop` fits the frame to whatever window it gets.
#[derive(Debug, Clone, Copy)]
struct Haze {
    style: Style,
    prims: &'static [Prim],
}

impl<Message> canvas::Program<Message> for Haze {
    type State = SoftCache;

    fn draw(
        &self,
        soft: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let size = (bounds.width.round() as u32, bounds.height.round() as u32);
        if size.0 > 0 && size.1 > 0 {
            let handle = soft.image(self.prims, &self.style.palette, size, 1.0);
            frame.draw_image(
                Rectangle { x: 0.0, y: 0.0, width: bounds.width, height: bounds.height },
                canvas::Image::new(handle).filter_method(image::FilterMethod::Linear),
            );
        }
        vec![frame.into_geometry()]
    }
}

/// Linear blend, for the one gradient the strip draws itself.
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

impl<Message> canvas::Program<Message> for Strip {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }

        // A `BarGround::Haze` is the `Haze` canvas under this one.

        if let BarGround::Band {
            left,
            right,
            rule: _,
            rule_width,
        } = self.ground
        {
            // Bands rather than a renderer gradient, for the reason
            // `widgets::ground` stacks discs: the falloff is smooth
            // enough at this size and it keeps the crate off
            // gradient support that varies by backend.
            let steps = 128;
            let step = w / steps as f32;
            for i in 0..steps {
                let t = (i as f32 + 0.5) / steps as f32;
                frame.fill(
                    &Path::rectangle(Point::new(i as f32 * step, 0.0), Size::new(step + 1.0, h)),
                    mix(left, right, t),
                );
            }
            frame.fill(
                &Path::rectangle(
                    Point::new(self.pad_left, h - rule_width),
                    Size::new((w - self.pad_left - self.pad_right).max(0.0), rule_width),
                ),
                self.chrome_ink,
            );
        }

        let stroke = Stroke::default()
            .with_color(self.chrome_ink)
            .with_width(self.stroke);

        if let BarChrome::Frame = self.chrome {
            let half = self.stroke / 2.0;
            let x0 = self.pad_left + half;
            let y0 = self.pad_y + half;
            let x1 = w - self.pad_right - half;
            let y1 = h - self.pad_y - half;
            frame.stroke(
                &Path::rectangle(Point::new(x0, y0), Size::new(x1 - x0, y1 - y0)),
                stroke,
            );

            let mut divider = |x: f32| {
                if x > self.pad_left && x < w - self.pad_right {
                    frame.stroke(
                        &Path::new(|b| {
                            b.move_to(Point::new(x, self.pad_y));
                            b.line_to(Point::new(x, h - self.pad_y));
                        }),
                        stroke,
                    );
                }
            };
            // One on the trailing edge of every module in the left run
            // -- which closes that run against the open centre segment
            // -- and one on the leading edge of every module in the
            // right, which does the same from the other side.
            // The runs are laid out from the frame's centreline, not
            // from the bar's padding -- see `frame_edge`.
            let mut x = self.pad_left + half;
            for &(gap, width) in &self.left {
                x += gap + width;
                divider(x);
            }
            let mut x = w - self.pad_right - half;
            for &(gap, width) in self.right.iter().rev() {
                x -= width;
                divider(x);
                x -= gap;
            }
        }

        match self.ornament {
            BarOrnament::None => {}
            BarOrnament::Bracket => self.bracket(&mut frame, w, h),
            BarOrnament::Wire => self.wire(&mut frame, w, h),
        }

        vec![frame.into_geometry()]
    }
}

impl Strip {
    /// Where the left run ends and where the right run begins.
    fn span(&self, w: f32) -> (f32, f32) {
        let left: f32 = self
            .left
            .iter()
            .map(|&(gap, width)| gap + width)
            .sum::<f32>()
            + self.pad_left;
        let right = w
            - self.pad_right
            - self
                .right
                .iter()
                .map(|&(gap, width)| gap + width)
                .sum::<f32>();
        (left, right)
    }

    /// Kitsch's bracket: mailbox-trace's list bracket scaled into the
    /// 3px the bar leaves under its cells. Down the left edge, an r8
    /// corner, then right along the foot at full strength to 60% of its
    /// run and fading to nothing at the end of it.
    fn bracket(&self, frame: &mut Frame, _w: f32, h: f32) {
        const X: f32 = 1.5;
        const RADIUS: f32 = 8.0;
        const REACH: f32 = 440.0;
        const SOLID: f32 = 0.6;

        let y = h - 1.5;
        let stroke = |color: Color| Stroke::default().with_color(color).with_width(self.stroke);
        frame.stroke(
            &Path::new(|b| {
                b.move_to(Point::new(X, 0.0));
                b.line_to(Point::new(X, y - RADIUS));
                b.quadratic_curve_to(Point::new(X, y), Point::new(X + RADIUS, y));
                b.line_to(Point::new(X + RADIUS + (REACH - X - RADIUS) * SOLID, y));
            }),
            stroke(self.ornament_ink),
        );

        // The fade, as steps: the ramp is 176px long and a canvas
        // stroke takes one colour.
        let start = X + RADIUS + (REACH - X - RADIUS) * SOLID;
        let steps = 24;
        for i in 0..steps {
            let t0 = i as f32 / steps as f32;
            let t1 = (i + 1) as f32 / steps as f32;
            let alpha = 1.0 - (t0 + t1) / 2.0;
            frame.stroke(
                &Path::new(|b| {
                    b.move_to(Point::new(start + (REACH - start) * t0, y));
                    b.line_to(Point::new(start + (REACH - start) * t1 + 0.5, y));
                }),
                stroke(Color {
                    a: alpha,
                    ..self.ornament_ink
                }),
            );
        }
    }

    /// Neokitsch's header wire band, bridging the empty centre: eight
    /// strands on a low run at each end, each rising through one S-bend
    /// onto a single bridge line, brightening downward.
    ///
    /// Dropped entirely when the gap cannot hold it, which is the
    /// design's own instruction -- a band squeezed to nothing would be
    /// a smear behind the window label.
    fn wire(&self, frame: &mut Frame, w: f32, _h: f32) {
        const INSET: f32 = 6.0;
        const RUN: f32 = 32.0;
        const BEND: f32 = 32.0;
        const STRANDS: usize = 8;
        const PITCH: f32 = 2.0;
        const TOP: f32 = 13.5;
        const BRIDGE: f32 = 5.0;
        const FOOT: f32 = 27.5;

        let (left, right) = self.span(w);
        let (x0, x1) = (left + INSET, right - INSET);
        if x1 - x0 < 2.0 * (RUN + BEND) + 120.0 {
            return;
        }

        for i in 0..STRANDS {
            let ys = TOP + PITCH * i as f32;
            let alpha = 0.35 + (1.0 - 0.35) * i as f32 / (STRANDS - 1) as f32;
            let path = Path::new(|b| {
                b.move_to(Point::new(x0, ys));
                b.line_to(Point::new(x0 + RUN, ys));
                b.bezier_curve_to(
                    Point::new(x0 + RUN + 17.0, ys),
                    Point::new(x0 + RUN + 15.0, BRIDGE),
                    Point::new(x0 + RUN + BEND, BRIDGE),
                );
                b.line_to(Point::new(x1 - RUN - BEND, BRIDGE));
                b.bezier_curve_to(
                    Point::new(x1 - RUN - 15.0, BRIDGE),
                    Point::new(x1 - RUN - 17.0, ys),
                    Point::new(x1 - RUN, ys),
                );
                b.line_to(Point::new(x1, ys));
            });
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(Color {
                        a: alpha,
                        ..self.ornament_ink
                    })
                    .with_width(1.0),
            );
        }

        // The end hooks overlap into one bright vertical edge, which
        // login-trace records and store-trace draws.
        frame.stroke(
            &Path::new(|b| {
                b.move_to(Point::new(x0, TOP));
                b.line_to(Point::new(x0, FOOT));
                b.move_to(Point::new(x1, TOP));
                b.line_to(Point::new(x1, FOOT));
            }),
            Stroke::default()
                .with_color(self.ornament_ink)
                .with_width(1.2),
        );
    }
}

/// Width a label needs, in pixels.
///
/// A [`Surface`] paints the box it is handed and its canvas fills
/// whatever space it is given, so in a shrink-width row the cells
/// collapse and clip their own text -- a five-character hostname came
/// out as three.
/// Sizing from the label is also the better behaviour for a bar: cells
/// stop reflowing every time CPU% ticks from 9 to 10.
///
/// `pad_x` is the leading air and `trail` the trailing, which are only
/// the same number in three of the four eras: a neokitsch cell reserves
/// its last 38px for the tab, and no label may sit on one.
///
/// So the reserve follows the *tab* rather than the era. A module that
/// carries none has nothing to keep its label off, which is why the
/// neokitsch CTA plate is `12 + text + 12` where the button beside it
/// is `12 + text + 38`.
fn width_for(style: &Style, label: &str, dress: &Dress) -> f32 {
    let b = &style.bar;
    let trail = if dress.tab { b.trail } else { b.pad_x };
    text_width(style, label) + b.pad_x + trail
}

/// A label's own measure, in pixels.
///
/// Spaces are counted at their own advance rather than at a letter's:
/// three of the four designs sized their cells by counting characters
/// flat and get `space_em == em` for it, and the fourth measured, so
/// this is one arithmetic with an era's answer in it rather than two.
fn text_width(style: &Style, label: &str) -> f32 {
    let b = &style.bar;
    let size = style.metrics.text_body as f32;
    let spaces = label.chars().filter(|c| *c == ' ').count() as f32;
    let letters = label.chars().count() as f32 - spaces;
    (letters * size * b.em + spaces * size * b.space_em).ceil()
}

/// A dressed module of the bar: the era's silhouette, its trim, and a
/// label or an icon inside it.
fn plate<'a, Message: 'static>(
    style: &Style,
    dress: &Dress,
    width: f32,
    lead: Option<f32>,
    ticks: bool,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let b = &style.bar;
    let trim = Trim {
        tab: dress
            .tab
            .then_some(b.tab)
            .flatten()
            .map(|tab| (tab, ink_of(style, tab.fill))),
        ticks: ticks.then(|| ink_of(style, dress.ink)),
        grain: Trim::grain_of(style, dress),
        stroke: b.stroke,
    };

    let body: Element<'a, Message> = if let Some(lead) = lead {
        container(content)
            .padding(Padding::ZERO.left(lead))
            .center_y(Length::Fill)
            .width(Length::Fill)
            .into()
    } else {
        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    };

    // Under `BarChrome::Frame` the strip has already drawn a divider
    // centred on each module boundary and the frame centred on the
    // row's edges, and the module is drawn on top of it. A face flush
    // to its cell buries the inner half of every line around it: a
    // filled cell showed 1px dividers where its neighbours showed 2,
    // and the frame's top and bottom edges vanished under it. The
    // design (entropism `bar.svg`, "fills are inset 1px inside the
    // chrome") keeps the fill inside the stroke: the row is laid out
    // from the chrome's centrelines (`frame_edge`) and each face is
    // inset half a stroke from its cell, so the lines run through
    // unbroken.
    let face: Element<'a, Message> = match b.chrome {
        BarChrome::Frame => container(face_canvas(style, dress, b.stroke))
            .padding(frame_edge(b))
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        BarChrome::Loose => face_canvas(style, dress, b.stroke),
    };
    let mut layers = stack![face];
    if !trim.is_empty() {
        layers = layers.push(canvas(trim).width(Length::Fill).height(Length::Fill));
    }
    container(layers.push(body))
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .into()
}

/// One module of the bar, with the width the strip has to know about.
struct Slot<'a, Message> {
    gap: f32,
    width: f32,
    element: Element<'a, Message>,
}

/// A run of modules, as a row and as the measurements [`Strip`] draws
/// its chrome from.
fn run<'a, Message: 'static>(
    slots: Vec<Slot<'a, Message>>,
) -> (Element<'a, Message>, Vec<(f32, f32)>) {
    let metrics: Vec<(f32, f32)> = slots.iter().map(|slot| (slot.gap, slot.width)).collect();
    let mut row = row![].height(Length::Fill).align_y(iced::Alignment::Center);
    for slot in slots {
        if slot.gap > 0.0 {
            row = row.push(
                Space::new()
                    .width(Length::Fixed(slot.gap))
                    .height(Length::Fill),
            );
        }
        row = row.push(slot.element);
    }
    (row.into(), metrics)
}

/// How large a menu row's icon is drawn, in pixels.
///
/// Tied to the body text rather than to [`icon_size`]: a tray cell's
/// icon is sized by the height of the bar and a menu row's by the line
/// it sits on, and an era declaring a taller bar would otherwise get
/// menu rows with a 24px icon beside 14px text.
///
/// Public for the same reason [`icon_size`] is -- whoever decodes the
/// icons decodes them at the size they will be drawn.
pub fn menu_icon_size(style: &Style) -> f32 {
    (style.metrics.text_body as f32 * 1.15).round()
}

/// The height of a row's content: one line of body text, or the icon
/// when the icon is taller.
///
/// 1.4 rather than iced's own 1.3 line height: the extra is what keeps
/// a descender off the edge of a row this pins the height of.
fn menu_line(style: &Style) -> f32 {
    (style.metrics.text_body as f32 * 1.4)
        .ceil()
        .max(menu_icon_size(style))
}

/// How tall one row is drawn.
///
/// Pinned rather than measured, which is what makes submenu placement
/// arithmetic instead of a guess: iced lays a panel out after this
/// function has already decided where the panel hanging off row *n*
/// goes, so the only way the two agree is for the row to be told its
/// height rather than asked for it.
fn row_height(style: &Style, entry: &MenuEntry) -> f32 {
    match entry.kind {
        MenuKind::Separator => match style.bar.menu.rule {
            // Entropism has no floating rule anywhere: a break between
            // groups is an empty cell between two of the row dividers
            // the panel already draws.
            MenuRule::Empty { height } => height,
            _ => style.bar.stroke + MENU_RULE_AIR * 2.0,
        },
        _ => menu_line(style) + style.bar.menu.row_air * 2.0,
    }
}

/// The same, plus the divider that follows it where the era draws one.
/// This is the figure submenu placement is arithmetic on.
fn row_pitch(style: &Style, entry: &MenuEntry) -> f32 {
    row_height(style, entry)
        + if style.bar.menu.row_divider {
            style.bar.stroke
        } else {
            0.0
        }
}

/// Whether a panel reserves the icon column.
///
/// Per panel rather than per row: one row with an icon indents every
/// label in the panel, so that a menu of six commands and one icon
/// reads as a column rather than as a step.
fn has_icons(entries: &[MenuEntry]) -> bool {
    entries.iter().any(|entry| entry.icon.is_some())
}

/// How wide one panel is drawn, in pixels.
///
/// Measured from the labels rather than filled to the content, because
/// a menu that resizes itself per era's `Corner` is one that lands
/// somewhere different per era.
fn level_width(style: &Style, entries: &[MenuEntry]) -> f32 {
    let m = &style.bar.menu;
    let per_char = style.metrics.text_body as f32 * style.bar.em;
    let widest = entries
        .iter()
        .map(|entry| entry.label.chars().count().min(MENU_LABEL_CHARS))
        .max()
        .unwrap_or(0);

    // Both columns are reserved by the whole panel as soon as one row
    // wants them, for the reason in `has_icons`.
    let gutter = if has_icons(entries) {
        m.icon_col + m.icon_gap
    } else {
        0.0
    };
    let marker = if entries
        .iter()
        .any(|entry| matches!(entry.kind, MenuKind::Submenu))
    {
        per_char + m.icon_gap
    } else {
        0.0
    };

    // The floor is what keeps a menu of `OK` from being a stamp; the
    // ceiling is `MENU_LABEL_CHARS` worth of the largest era's body
    // text, so the clip above is what bounds this and not the clamp.
    ((widest as f32 * per_char).ceil() + gutter + marker + m.level_pad).clamp(140.0, 460.0)
        + m.ring_inset()
}

/// One panel of the open chain: its rows, how far its top edge sits
/// below the root panel's, and which of its rows holds the next panel
/// open.
struct Level<'a> {
    entries: &'a [MenuEntry],
    top: f32,
    open: Option<usize>,
}

/// Walk `open` as far as it actually goes.
///
/// A path is the panel's state and the tree under it is another
/// application's data, so the two can disagree: a path indexing past
/// the end of a level, or naming a row whose submenu turned out to be
/// empty, is a chain that stops there rather than an error. Drawing
/// never has to wait for the host to notice and truncate.
fn levels<'a>(style: &Style, menu: &'a TrayMenu, open: &[usize]) -> Vec<Level<'a>> {
    let mut levels = Vec::with_capacity(open.len() + 1);
    let mut entries: &'a [MenuEntry] = &menu.entries;
    let mut top = 0.0;
    let mut depth = 0;

    loop {
        let next = open.get(depth).copied().filter(|&index| {
            entries
                .get(index)
                .is_some_and(|entry| !entry.children.is_empty())
        });
        levels.push(Level {
            entries,
            top,
            open: next,
        });
        let Some(index) = next else { return levels };

        // The child's first row lines up with the row that opened it,
        // and both panels carry the same top padding -- so the padding
        // cancels and this is a sum of row pitches and nothing else.
        top += entries[..index]
            .iter()
            .map(|entry| row_pitch(style, entry))
            .sum::<f32>();
        entries = &entries[index].children;
        depth += 1;
    }
}

/// How wide the whole open chain is, in pixels.
///
/// Public for the same reason [`icon_size`] is: whoever places the
/// panel has to know how wide it will be, and the alternative is two
/// constants that agree until one of them is edited. The chain's
/// *right* edge is what goes under the pointer, so this width is the
/// offset that puts it there.
///
/// The panels' width, not the element's: the element runs
/// [`menu_overshoot`] further right, for the highlight plate that
/// neokitsch draws past its panel. That strip belongs *past* the
/// pointer, so a host placing the chain by this width gets it there
/// for free -- and one sizing a surface to the chain needs the sum.
pub fn menu_chain_width(style: &Style, menu: &TrayMenu, open: &[usize]) -> f32 {
    let m = &style.bar.menu;
    let levels = levels(style, menu, open);
    let panels: f32 = levels
        .iter()
        .map(|level| level_width(style, level.entries))
        .sum();
    panels + m.level_gap * (levels.len() as f32 - 1.0).max(0.0)
}

/// How far the chain's element runs past [`menu_chain_width`], to the
/// right: the root panel's highlight plate, where the era has one run
/// past its outline. Zero in three eras, 8 in neokitsch.
pub fn menu_overshoot(style: &Style) -> f32 {
    style.bar.menu.row_overshoot
}

/// A corner's cut as a pair, for the paths that walk one by hand.
fn cut_xy(cut: Cut) -> (f32, f32) {
    match cut {
        Cut::Square => (0.0, 0.0),
        Cut::Chamfer { x, y } => (x, y),
        Cut::Round { radius } => (radius, radius),
        // No panel wears a peak; drawn as its chamfer, as `Surface` does
        // off the top-left.
        Cut::Peak { x, y, .. } => (x, y),
    }
}

/// A menu panel's silhouette, and whatever the era draws inside it.
///
/// Not a [`Surface`]: neomil's root panel is not a corner-treated box
/// at all -- its right edge carries a filled bar on slanted ends and
/// then steps 8px inward for the rest of its height -- and kitsch's
/// closes on a solid curl. Both live inside the panel's own width, so
/// they cost the chain arithmetic nothing.
#[derive(Debug, Clone, Copy)]
struct Panel {
    corners: Corners,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_width: f32,
    echo: PanelEcho,
    echo_ink: Color,
    accent: Color,
    root: bool,
    /// Width at the right the panel leaves undrawn: the strip a
    /// highlighted row's plate runs into (`BarMenu::row_overshoot`).
    overshoot: f32,
}

impl Panel {
    /// The outline, including neomil's stepped right edge.
    fn path(&self, w: f32, h: f32) -> Path {
        let PanelEcho::EdgeBar { step, top, len } = self.echo else {
            return outline(self.corners, Ticket::default(), w, h);
        };
        if !self.root {
            return outline(self.corners, Ticket::default(), w, h);
        }
        let tr = cut_xy(self.corners.top_right);
        let bl = cut_xy(self.corners.bottom_left);
        Path::new(|b| {
            b.move_to(Point::new(0.0, 0.0));
            b.line_to(Point::new(w - tr.0, 0.0));
            b.line_to(Point::new(w, tr.1));
            b.line_to(Point::new(w, top + len - step));
            b.line_to(Point::new(w - step, top + len));
            b.line_to(Point::new(w - step, h));
            b.line_to(Point::new(bl.0, h));
            b.line_to(Point::new(0.0, h - bl.1));
            b.close();
        })
    }

    /// One onion ring, `d` inside the outline: open, from the shared
    /// left edge round the top, down the right edge and along the
    /// bottom until it meets the shared bottom-left cut. A chamfer on
    /// the top-right keeps its angle and its start x, so its foot stays
    /// where the outline's is -- the cards' rings do exactly that
    /// (dashboard-trace `#nring1..6`).
    fn ring(&self, w: f32, h: f32, d: f32) -> Path {
        let (ax, ay) = cut_xy(self.corners.top_left);
        let (cx, cy) = cut_xy(self.corners.top_right);
        let (rx, ry) = cut_xy(self.corners.bottom_right);
        let (bx, by) = cut_xy(self.corners.bottom_left);
        let round = |cut: Cut| matches!(cut, Cut::Round { .. });
        Path::new(|b| {
            b.move_to(Point::new(0.0, d + ay));
            if round(self.corners.top_left) {
                b.quadratic_curve_to(Point::new(0.0, d), Point::new(ax, d));
            } else {
                b.line_to(Point::new(ax, d));
            }
            b.line_to(Point::new(w - cx, d));
            match self.corners.top_right {
                Cut::Round { .. } => {
                    b.quadratic_curve_to(Point::new(w - d, d), Point::new(w - d, d + cy))
                }
                Cut::Chamfer { .. } | Cut::Peak { .. } => {
                    b.line_to(Point::new(w - d, d + (cx - d) * cy / cx))
                }
                Cut::Square => b.line_to(Point::new(w - d, d)),
            }
            b.line_to(Point::new(w - d, h - d - ry));
            match self.corners.bottom_right {
                Cut::Round { .. } => {
                    b.quadratic_curve_to(Point::new(w - d, h - d), Point::new(w - d - rx, h - d))
                }
                Cut::Chamfer { .. } | Cut::Peak { .. } => b.line_to(Point::new(w - d - rx, h - d)),
                Cut::Square => b.line_to(Point::new(w - d, h - d)),
            }
            let foot = if by > 0.0 { bx * (1.0 - d / by) } else { 0.0 };
            b.line_to(Point::new(foot.max(0.0), h - d));
        })
    }

    /// The curl kitsch closes its one container with, as `(fill,
    /// crest)`: mailbox-trace's wave scaled into the panel's foot.
    fn wave(h: f32) -> (Path, Path) {
        let crest = |b: &mut canvas::path::Builder| {
            b.move_to(Point::new(0.0, h - 28.0));
            b.quadratic_curve_to(Point::new(0.0, h - 20.0), Point::new(8.0, h - 20.0));
            b.line_to(Point::new(40.5, h - 20.0));
            b.quadratic_curve_to(Point::new(47.5, h - 20.0), Point::new(51.5, h - 14.5));
            b.line_to(Point::new(62.0, h - 1.5));
            b.quadratic_curve_to(Point::new(63.2, h), Point::new(65.0, h));
        };
        (
            Path::new(|b| {
                crest(b);
                b.line_to(Point::new(8.0, h));
                b.quadratic_curve_to(Point::new(0.0, h), Point::new(0.0, h - 8.0));
                b.close();
            }),
            Path::new(crest),
        )
    }
}

impl<Message> canvas::Program<Message> for Panel {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let inset = if self.stroke.is_some() {
            self.stroke_width / 2.0
        } else {
            0.0
        };
        let (w, h) = (
            bounds.width - inset * 2.0 - self.overshoot,
            bounds.height - inset * 2.0,
        );
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        frame.translate(iced::Vector::new(inset, inset));

        let path = self.path(w, h);
        if let Some(fill) = self.fill {
            frame.fill(&path, fill);
        }

        // The curl goes under the outline, which is drawn over it
        // unchanged -- mailbox-trace's wave sits inside the container
        // it closes.
        if let (PanelEcho::Wave, true) = (self.echo, self.root) {
            let (body, crest) = Panel::wave(h);
            frame.fill(&body, self.echo_ink);
            frame.stroke(
                &crest,
                Stroke::default()
                    .with_color(self.accent)
                    .with_width(self.stroke_width),
            );
        }

        if let Some(color) = self.stroke {
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(color)
                    .with_width(self.stroke_width),
            );
        }

        // Neokitsch's onion rings, nested inside the outline: over the
        // fill, under the rows (which the padding keeps clear of them).
        if let PanelEcho::Rings { count, pitch } = self.echo {
            // The detail panel's measured ratios (dashboard-trace
            // `#npring1..4`); the fifth and later would be invisible.
            const FADE: [f32; 4] = [0.7, 0.7, 0.55, 0.25];
            for ring in 1..=count {
                let d = ring as f32 * pitch;
                frame.stroke(
                    &self.ring(w, h, d),
                    Stroke::default()
                        .with_color(Color {
                            a: FADE[(ring - 1).min(FADE.len() - 1)],
                            ..self.accent
                        })
                        // The design draws the rings at 1; rsvg renders
                        // that as one solid pixel and an antialiased
                        // canvas as two half-lit ones, which reads as a
                        // fainter ring than the trace has. 1.5 puts a
                        // whole pixel on the line.
                        .with_width(1.5),
                );
            }
        }

        // The bright bar riding the right edge, and the two glitch
        // echoes trailing it to the panel's foot.
        if let (PanelEcho::EdgeBar { step, top, len }, true) = (self.echo, self.root) {
            let bar = Path::new(|b| {
                b.move_to(Point::new(w, top));
                b.line_to(Point::new(w, top + len - step));
                b.line_to(Point::new(w - step, top + len));
                b.line_to(Point::new(w - step, top + step));
                b.close();
            });
            frame.fill(&bar, self.accent);
            for dx in [5.0f32, 3.0] {
                frame.fill(
                    &Path::rectangle(
                        Point::new(w - dx, top + len),
                        Size::new(self.stroke_width, (h - top - len).max(0.0)),
                    ),
                    self.echo_ink,
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

/// A menu row's own trim: the spine neomil runs down a highlighted row,
/// and the separate icon cell kitsch splits one into.
#[derive(Debug, Clone, Copy)]
struct RowTrim {
    /// `(x, width, length, ink)`.
    spine: Option<(f32, f32, f32, Color)>,
    /// `(x, width, corners, ink)`.
    split: Option<(f32, f32, Corners, Color)>,
}

impl RowTrim {
    fn is_empty(&self) -> bool {
        self.spine.is_none() && self.split.is_none()
    }
}

impl<Message> canvas::Program<Message> for RowTrim {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        if let Some((x, width, len, ink)) = self.spine {
            frame.fill(
                &Path::rectangle(Point::new(x, 0.0), Size::new(width, len)),
                ink,
            );
        }
        if let Some((x, width, corners, ink)) = self.split {
            let path = outline(corners, Ticket::default(), width, bounds.height);
            frame.translate(iced::Vector::new(x, 0.0));
            frame.fill(&path, ink);
        }
        vec![frame.into_geometry()]
    }
}

/// A separator, in whichever shape the era says a break between groups
/// is -- and the row divider, which entropism draws on every boundary.
#[derive(Debug, Clone, Copy)]
struct Rule {
    inset: (f32, f32),
    width: f32,
    ink: Color,
    /// Neokitsch stands a tab on its list rules.
    tab: Option<(Tab, Color)>,
}

impl<Message> canvas::Program<Message> for Rule {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        let y = ((h - self.width) / 2.0).max(0.0);
        let x0 = self.inset.0;
        let x1 = (w - self.inset.1).max(x0);
        frame.fill(
            &Path::rectangle(Point::new(x0, y), Size::new(x1 - x0, self.width)),
            self.ink,
        );
        if let Some((tab, ink)) = self.tab {
            // Standing on the rule rather than hanging off a box, so
            // the rule's own top edge is the tab's baseline.
            frame.fill(&tab_path(tab, x1 - 12.0, y, 0.0), ink);
        }
        vec![frame.into_geometry()]
    }
}

/// The era's left arrow, for the submenu marker of an era that has one.
#[derive(Debug, Clone, Copy)]
struct Arrow {
    ink: Color,
}

impl<Message> canvas::Program<Message> for Arrow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        let mid = h / 2.0;
        let head = h * 0.8;
        frame.fill(
            &Path::new(|b| {
                b.move_to(Point::new(0.0, mid));
                b.line_to(Point::new(head, 0.0));
                b.line_to(Point::new(head, h));
                b.close();
            }),
            self.ink,
        );
        frame.fill(
            &Path::rectangle(
                Point::new(head * 0.75, mid - 1.0),
                Size::new((w - head * 0.75).max(0.0), 2.0),
            ),
            self.ink,
        );
        vec![frame.into_geometry()]
    }
}

/// One menu row.
fn menu_row<'a, Message: Clone + 'static>(
    style: &Style,
    entry: &MenuEntry,
    path: MenuPath,
    gutter: bool,
    open: bool,
    edge: f32,
    on_entry: fn(i32) -> Message,
    on_submenu: fn(MenuPath) -> Message,
) -> Element<'a, Message> {
    let m = &style.bar.menu;

    if let MenuKind::Separator = entry.kind {
        return separator_row(style, entry, edge);
    }

    let label = clip(&entry.label, MENU_LABEL_CHARS);
    // The parent of an open submenu wears the era's own mark for "the
    // one you are looking at", which is not always its selection:
    // three eras fill it and neokitsch outlines it, because a material
    // means "chosen" there and an outline means "current".
    //
    // `edge` is how far the row's box runs past where its content
    // stops -- the rings' depth, plus the root panel's overshoot. The
    // open face and the content stay inside it; the highlight is the
    // one thing that runs out to the box's edge, which is the point of
    // widening the box (see `menu_panel`).
    let dressed = if open {
        Some((m.open, (m.open_inset.0, m.open_inset.1 + edge)))
    } else if matches!(entry.kind, MenuKind::Toggle(true)) {
        Some((m.row, m.row_inset))
    } else {
        None
    };

    let ink = match (dressed, entry.enabled) {
        (Some((dress, _)), _) => dress.ink,
        (None, false) => m.disabled,
        (None, true) => Ink::Fg,
    };

    let size = menu_icon_size(style);
    let leading: Option<Element<'a, Message>> = gutter.then(|| {
        let slot: Element<'a, Message> = match &entry.icon {
            Some(handle) => image(handle.clone())
                .width(Length::Fixed(size))
                .height(Length::Fixed(size))
                // Same reasoning as a tray cell's: an item's icon
                // arrives at whatever size it had, and nearest would
                // alias it.
                .filter_method(image::FilterMethod::Linear)
                .into(),
            None => Space::new()
                .width(Length::Fixed(size))
                .height(Length::Shrink)
                .into(),
        };
        container(slot)
            .center_x(Length::Fixed(m.icon_col))
            .width(Length::Fixed(m.icon_col + m.icon_gap))
            .into()
    });

    let trailing: Element<'a, Message> = match (entry.kind, m.marker) {
        (MenuKind::Submenu, MenuMarker::Arrow { w, h }) => canvas(Arrow {
            ink: ink_of(style, ink),
        })
        .width(Length::Fixed(w))
        .height(Length::Fixed(h))
        .into(),
        (MenuKind::Submenu, MenuMarker::Text) => ink_text(style, ink, false, "<").into(),
        _ => Space::new()
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into(),
    };

    // An open face that carries a tab has it standing on its inside
    // bottom edge, under where the marker would sit; the marker moves
    // left of it (neokitsch bar.svg: "the \"<\" glyph right-aligned at
    // x 1420.2 so it clears the tab", 41 in from a box ending at
    // 1461.2 -- the tab's base and inset plus one icon gap).
    let clear_tab = match dressed {
        Some((dress, _)) if dress.tab => style
            .bar
            .tab
            .map_or(0.0, |tab| tab.base + tab.inset + m.icon_gap),
        _ => 0.0,
    };

    let inner = row![]
        .extend(leading)
        .push(ink_text(style, ink, false, label))
        .push(Space::new().width(Length::Fill).height(Length::Shrink))
        .push(trailing)
        .push(Space::new().width(Length::Fixed(clear_tab)).height(Length::Shrink))
        .align_y(iced::Alignment::Center)
        .width(Length::Fill)
        // See `row_height`: this is the half of that agreement the
        // renderer gets told about.
        .height(Length::Fixed(menu_line(style)));

    let padded = container(inner)
        .padding(Padding {
            top: m.row_air,
            right: m.row_side + edge,
            bottom: m.row_air,
            left: m.row_side,
        })
        .width(Length::Fill);

    let face: Element<'a, Message> = match dressed {
        None => padded.into(),
        Some((dress, inset)) => {
            // Kitsch splits a selected row in two -- an icon cell, a
            // 2px gap, then the chamfered body -- but only where the
            // row has an icon to put in one.
            let split = m
                .row_split
                .filter(|_| entry.icon.is_some())
                .map(|(width, corners)| (inset.0, width, corners, ink_of(style, dress.fill)));
            let body_left = inset.0 + split.map(|(_, width, _, _)| width + 2.0).unwrap_or(0.0);

            let trim = RowTrim {
                spine: (m.spine > 0.0).then(|| {
                    (
                        (inset.0 - 5.0).max(0.0),
                        m.spine,
                        // Down to the knee of the row's own chamfer,
                        // which is where store-trace stops the nav
                        // row's spine.
                        row_height(style, entry) - cut_xy(dress.corners.bottom_left).1,
                        ink_of(style, dress.fill),
                    )
                }),
                split,
            };

            let mut plate = stack![face_canvas(style, &dress, style.bar.stroke)];
            let row_trim = Trim {
                tab: dress
                    .tab
                    .then_some(style.bar.tab)
                    .flatten()
                    .map(|tab| (tab, ink_of(style, tab.fill))),
                ticks: None,
                grain: Trim::grain_of(style, &dress),
                stroke: style.bar.stroke,
            };
            if !row_trim.is_empty() {
                plate = plate.push(
                    canvas(row_trim).width(Length::Fill).height(Length::Fill),
                );
            }

            let mut background = stack![container(plate)
                .padding(Padding {
                    top: 0.0,
                    right: inset.1,
                    bottom: 0.0,
                    left: body_left,
                })
                .width(Length::Fill)
                .height(Length::Fill)];
            if !trim.is_empty() {
                background =
                    background.push(canvas(trim).width(Length::Fill).height(Length::Fill));
            }

            layered(background, padded)
        }
    };

    // Sending `clicked` to a row an application has greyed is asking
    // it to do something it just said it would not do.
    if !entry.enabled {
        return face;
    }

    let message = match entry.kind {
        // A submenu with nothing in it *yet* answers too. dbusmenu lets
        // an application leave a submenu empty until `AboutToShow` is
        // called on that row's own id, and the host sends that when the
        // row is clicked -- so refusing the click here would be the one
        // thing that makes such a menu permanently unopenable. Nothing
        // is drawn until the children turn up: `levels` walks into a
        // row only when it has some, so a row the application really
        // has nothing for stays marked and looks unmoved.
        MenuKind::Submenu => on_submenu(path),
        _ => on_entry(entry.id),
    };

    mouse_area(face)
        .on_press(message)
        .interaction(mouse::Interaction::Pointer)
        .into()
}

/// A break between groups, in the era's own shape.
fn separator_row<'a, Message: 'static>(
    style: &Style,
    entry: &MenuEntry,
    edge: f32,
) -> Element<'a, Message> {
    let m = &style.bar.menu;
    let height = row_height(style, entry);
    if let MenuRule::Empty { .. } = m.rule {
        return Space::new()
            .width(Length::Fill)
            .height(Length::Fixed(height))
            .into();
    }
    let inset = match m.rule {
        MenuRule::Inset => m.row_inset,
        MenuRule::Tabbed => (6.0, 6.0),
        _ => (0.0, 0.0),
    };
    canvas(Rule {
        inset: (inset.0, inset.1 + edge),
        width: style.bar.stroke,
        ink: ink_of(style, m.rule_ink),
        tab: match m.rule {
            MenuRule::Tabbed => style.bar.tab.map(|tab| (tab, ink_of(style, tab.fill))),
            _ => None,
        },
    })
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .into()
}

/// The rule entropism puts on every row boundary inside a panel.
fn row_divider<'a, Message: 'static>(style: &Style) -> Element<'a, Message> {
    canvas(Rule {
        inset: (0.0, 0.0),
        width: style.bar.stroke,
        ink: ink_of(style, style.bar.menu.panel.stroke),
        tab: None,
    })
    .width(Length::Fill)
    .height(Length::Fixed(style.bar.stroke))
    .into()
}

/// One panel of the chain, filled and stroked.
fn menu_panel<'a, Message: Clone + 'static>(
    style: &Style,
    level: &Level<'_>,
    prefix: &[usize],
    root: bool,
    on_entry: fn(i32) -> Message,
    on_submenu: fn(MenuPath) -> Message,
) -> Element<'a, Message> {
    let m = &style.bar.menu;
    let gutter = has_icons(level.entries);
    // A highlighted row's plate can run past the panel's right outline
    // (neokitsch's does, by 8), and a canvas draws only inside its own
    // bounds -- so the rows' boxes have to reach that far. The root
    // container is widened by the overshoot, the rows' column gives up
    // its ring inset on the right, and each row pads its content back
    // by both so that only the highlight moves.
    let overshoot = if root { m.row_overshoot } else { 0.0 };
    let edge = m.ring_inset() + overshoot;
    let mut rows = iced::widget::column![].width(Length::Fill);
    for (index, entry) in level.entries.iter().enumerate() {
        let mut path = prefix.to_vec();
        path.push(index);
        rows = rows.push(menu_row(
            style,
            entry,
            path,
            gutter,
            level.open == Some(index),
            edge,
            on_entry,
            on_submenu,
        ));
        if m.row_divider && index + 1 < level.entries.len() {
            rows = rows.push(row_divider(style));
        }
    }

    let panel = Panel {
        corners: m.panel.corners,
        fill: style.ink(m.panel.fill),
        stroke: style.ink(m.panel.stroke),
        stroke_width: style.bar.stroke,
        echo: m.echo,
        // The two ornaments that live inside a panel take their ink
        // from the era rather than from the panel's stroke: neomil's
        // glitch echoes are the deep red under the fill red, and
        // kitsch's curl is the era's one solid decoration colour.
        echo_ink: match m.echo {
            PanelEcho::Wave => style.ornament(),
            _ => style.palette.border,
        },
        accent: ink_of(style, m.panel.stroke),
        root,
        overshoot,
    };

    container(layered(
        canvas(panel).width(Length::Fill).height(Length::Fill),
        // The rows sit inside the innermost ring, in the era that has
        // them; the left edge is the rings' own and takes no more air.
        // The right is the rows' own business (`edge`, above).
        container(rows).padding(Padding {
            top: m.air + m.ring_inset(),
            right: m.side,
            bottom: m.air + m.ring_inset() + if root { m.foot } else { 0.0 },
            left: m.side,
        }),
    ))
    .width(Length::Fixed(level_width(style, level.entries) + overshoot))
    .height(Length::Shrink)
    .into()
}

/// An item's context menu, in the era's own dress: the root panel and
/// whatever chain of submenus `open` names, as one element.
///
/// Filled rather than outlined, unlike every other surface on the bar:
/// this is the one thing the bar draws that floats over other
/// applications, and an unfilled panel would show a terminal through
/// its own rows.
///
/// ## Why a submenu is drawn here rather than on a surface of its own
///
/// The host already went to some trouble over the *first* surface: an
/// `xdg_popup` and a menu-sized layer surface both dismiss only by
/// being clicked, because `layershellev` 0.13.7 never calls
/// `xdg_popup.grab()` and the bar takes no keyboard focus to lose. The
/// answer was an output-sized `Overlay` layer surface that hears every
/// click on the screen (see `examples/cp-eras-ui-bar.rs`).
///
/// A submenu inherits that problem and adds one. Stacking a second
/// output-sized overlay would put a surface over the parent panel, so
/// the parent's other rows would stop answering -- the child would
/// have to be drawn with a hole in it the shape of the menu underneath
/// to get them back. And a *menu-sized* second layer surface is the
/// option that was already rejected once, for the grab.
///
/// The overlay is output-sized and this function is drawing on it, so
/// the child panel needs no surface at all: it is a second column in a
/// row, placed by arithmetic in coordinates this file owns. Dismissal
/// then stays exactly as coherent as it was, because there is nothing
/// new to dismiss -- one click outside destroys the one surface and
/// the whole chain with it, rather than unwinding a stack of them.
///
/// The chain grows **leftwards**, which is why the submenu marker
/// points that way. The tray is the last group on the right-hand side
/// of the bar, so the root panel is already hard against the right
/// edge of the screen; a submenu opening rightwards would be a submenu
/// off the edge of it. Going left needs no knowledge of how wide the
/// output is, which is the same property the surface choice was made
/// for.
pub fn tray_menu<'a, Message: Clone + 'static>(
    style: &Style,
    menu: &TrayMenu,
    open: &[usize],
    on_entry: fn(i32) -> Message,
    on_submenu: fn(MenuPath) -> Message,
) -> Element<'a, Message> {
    let m = &style.bar.menu;
    let levels = levels(style, menu, open);

    // Deepest first, so that the root panel ends up rightmost and the
    // chain grows away from the edge of the screen.
    let mut chain = row![].align_y(iced::Alignment::Start);
    for (depth, level) in levels.iter().enumerate().rev() {
        if depth + 1 < levels.len() && m.level_gap > 0.0 {
            chain = chain.push(
                Space::new()
                    .width(Length::Fixed(m.level_gap))
                    .height(Length::Fixed(1.0)),
            );
        }
        // `depth` never runs past the walk, so this slice is the
        // prefix that actually got followed.
        let panel = menu_panel(style, level, &open[..depth], depth == 0, on_entry, on_submenu);
        chain = chain.push(
            iced::widget::column![
                Space::new()
                    .width(Length::Shrink)
                    .height(Length::Fixed(level.top)),
                panel
            ]
            .height(Length::Shrink),
        );
    }

    // Every ornament an era draws lives inside its panel's own box
    // (neokitsch's rings did not until 2026-09-04, and the chain then
    // carried a padded canvas of them), so the chain is the panels and
    // nothing else.
    container(chain).height(Length::Shrink).into()
}

/// How the bar reports a pointer event on a tray cell to its host.
///
/// `usize` indexes [`Readings::tray`]. `None` at the call site means
/// the bar draws tray cells but does not listen -- which is what
/// `cp-eras-ui-bar-window` wants, since a golden of a still life
/// should not carry hit-testing it never exercises.
///
/// A bare function pointer rather than a closure: the caller is a
/// message constructor and has nothing to capture, and a `&dyn Fn`
/// would have to outlive the `Element`, which in an iced `view` means
/// finding somewhere `'static` to keep a closure that does nothing.
pub type OnTray<Message> = fn(usize, TrayAction) -> Message;

/// Everything the bar draws, already gathered.
#[derive(Debug, Clone, Default)]
pub struct Readings {
    pub host: String,
    pub workspaces: Vec<Workspace>,
    pub window: String,
    /// Whole percent, so the bar never reflows on a decimal.
    pub cpu: u8,
    pub memory: u8,
    /// `None` when no sound server answered. A machine without one is a
    /// normal state rather than a failure, so the module leaves the row
    /// instead of showing a zero it cannot vouch for.
    pub audio: Option<Audio>,
    /// Empty when there is no tray, no session bus, or nothing has
    /// registered an icon. All three are ordinary states and all three
    /// draw nothing, so `Vec` rather than `Option<Vec>`: there is no
    /// fourth case for the bar to tell apart.
    pub tray: Vec<TrayItem>,
    pub network: Network,
    pub clock: String,
    pub date: String,
}

/// Which of the era's dresses a module wears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wear {
    Idle,
    Selected,
    /// The reading is a warning: the sink is muted, there is no route
    /// out, or an item is asking to be looked at.
    Alert,
}

fn dress_for(style: &Style, wear: Wear) -> Dress {
    match wear {
        Wear::Idle => style.bar.idle,
        Wear::Selected => style.bar.selected,
        Wear::Alert => style.bar.alert,
    }
}

/// The label an era puts on a module in `wear`.
///
/// Entropism is the reason this is not just the reading: the only
/// urgency mark anywhere in its material is a literal " (!)" suffix in
/// the *same* ink as its neighbours, so the era says alarm in words and
/// `palette.alert` goes unused on this surface.
fn worn_label(style: &Style, label: String, wear: Wear) -> String {
    match (wear, style.bar.alert_suffix) {
        (Wear::Alert, Some(suffix)) => format!("{label}{suffix}"),
        _ => label,
    }
}

/// A bar module carrying a label.
fn text_cell<'a, Message: 'static>(
    style: &Style,
    wear: Wear,
    label: impl Into<String>,
    bold: bool,
    width: Option<f32>,
) -> Slot<'a, Message> {
    worn_cell(style, dress_for(style, wear), wear, label, bold, width)
}

/// The same, in a dress the caller has already adjusted -- which is
/// only ever the workspace run, and only in the era whose workspaces
/// are a different shape from its readouts.
fn worn_cell<'a, Message: 'static>(
    style: &Style,
    dress: Dress,
    wear: Wear,
    label: impl Into<String>,
    bold: bool,
    width: Option<f32>,
) -> Slot<'a, Message> {
    let label = worn_label(style, label.into(), wear);
    let track = if wear == Wear::Alert {
        label.chars().count() as f32 * style.bar.alert_track
    } else {
        0.0
    };
    let width = width.unwrap_or_else(|| width_for(style, &label, &dress) + track);
    // The label is set against the leading edge because the trailing
    // end belongs to the tab; a plate with no tab centres it, the way
    // ENTER / LOGIN does.
    let lead = (style.bar.label_left && dress.tab).then_some(style.bar.pad_x);
    let content = ink_text(style, dress.ink, bold, label).into();
    Slot {
        gap: style.bar.gap,
        width,
        element: plate(style, &dress, width, lead, false, content),
    }
}

/// Clip to `max` characters, counting characters rather than bytes so a
/// non-ASCII SSID does not get cut mid-codepoint.
fn clip(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let head: String = text.chars().take(max.saturating_sub(1)).collect();
    format!("{head}…")
}

/// Volume and mute in one module.
///
/// `VOL` and `MUT` rather than a speaker glyph: the icon set is still a
/// to-do, and three characters either way means the cell keeps its
/// width when the sink is muted, so nothing to its left reflows.
fn audio_cell<'a, Message: 'static>(style: &Style, audio: &Audio) -> Slot<'a, Message> {
    // Three digits covers PulseAudio's amplification range without the
    // cell growing; past that the number is not the interesting fact.
    let volume = audio.volume.min(999);

    if audio.muted {
        text_cell(style, Wear::Alert, format!("MUT {volume:>3}%"), false, None)
    } else {
        text_cell(style, Wear::Idle, format!("VOL {volume:>3}%"), false, None)
    }
}

/// How large a tray icon is drawn, in pixels.
///
/// Derived from the bar's own height rather than fixed, so an era that
/// declares a taller bar gets larger icons instead of a small one
/// floating in a big cell. The margin is the cell's stroke, its 2px of
/// vertical padding and the bar's own 3px, doubled.
///
/// Public because whoever gathers the readings has to decode icons at
/// the size they will be drawn, and the alternative is two constants
/// that agree until one of them is edited.
pub fn icon_size(style: &Style) -> f32 {
    (style.bar.height as f32 - 12.0).clamp(10.0, 24.0)
}

/// One tray item drawn as its icon.
///
/// The icon is handed over already decoded and composited, so all this
/// decides is the box around it -- which is the whole argument for
/// having built this bar: a tray icon is a module, so it wears the
/// era's silhouette, tab and trim for free.
fn icon_cell<'a, Message: 'static>(
    style: &Style,
    icon: &image::Handle,
    attention: bool,
) -> Slot<'a, Message> {
    let size = icon_size(style);
    let wear = if attention { Wear::Alert } else { Wear::Idle };
    let dress = dress_for(style, wear);

    // Same reserve as a label's, for the same reason.
    let mut width = size
        + if dress.tab {
            style.bar.icon_pad
        } else {
            style.bar.pad_x * 2.0
        };
    let pixmap: Element<'a, Message> = image(icon.clone())
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        // Nearest would alias a 22px item icon scaled to 14;
        // these are photographs as far as the bar is concerned.
        .filter_method(image::FilterMethod::Linear)
        .into();

    // An era that spells alarm as a word says it here too, beside the
    // pixmap, rather than moving an ink the era never moves.
    let content: Element<'a, Message> = match (attention, style.bar.alert_suffix) {
        (true, Some(suffix)) => {
            let suffix = suffix.trim().to_string();
            width += width_for(style, &suffix, &dress) - style.bar.pad_x;
            row![
                pixmap,
                Space::new()
                    .width(Length::Fixed(style.bar.menu.icon_gap))
                    .height(Length::Shrink),
                ink_text(style, dress.ink, false, suffix)
            ]
            .align_y(iced::Alignment::Center)
            .into()
        }
        _ => pixmap,
    };

    Slot {
        gap: style.bar.gap,
        width,
        element: plate(style, &dress, width, None, false, content),
    }
}

/// One tray item, and the pointer events it answers to.
fn tray_cell<'a, Message: Clone + 'static>(
    style: &Style,
    item: &TrayItem,
    index: usize,
    on_tray: Option<OnTray<Message>>,
) -> Slot<'a, Message> {
    let wear = if item.attention {
        Wear::Alert
    } else {
        Wear::Idle
    };
    let slot = match &item.icon {
        Some(icon) => icon_cell(style, icon, item.attention),
        // Clipped for the same reason the SSID is: a label is whatever
        // the application chose to call itself, and one long one must
        // not push the clock off the screen.
        None => text_cell(style, wear, clip(&item.label, 6), false, None),
    };

    let Some(on_tray) = on_tray else {
        return slot;
    };

    Slot {
        element: mouse_area(slot.element)
            .on_press(on_tray(index, TrayAction::Activate))
            .on_middle_press(on_tray(index, TrayAction::Secondary))
            .on_right_press(on_tray(index, TrayAction::Context))
            .on_scroll(move |delta| on_tray(index, TrayAction::Scroll(detents(delta))))
            // The one cell on the bar that does anything, so it is the
            // one cell that should look like it does.
            .interaction(mouse::Interaction::Pointer)
            .into(),
        ..slot
    }
}

/// A wheel movement as whole detents, keeping the delta's own sign.
///
/// Both of iced's deltas are folded into one number because the
/// protocol's `Scroll` takes an integer and an axis, not a pixel
/// count. A pixel delta comes from a touchpad, where 15px is the
/// conventional detent; rounding away from zero means a small flick
/// still counts as one, which is what the item is waiting to be told.
/// Zero stays zero, and the host is expected to drop it rather than
/// tell an application that nothing happened.
fn detents(delta: mouse::ScrollDelta) -> i32 {
    let lines = match delta {
        mouse::ScrollDelta::Lines { y, .. } => y,
        mouse::ScrollDelta::Pixels { y, .. } => y / 15.0,
    };
    if lines == 0.0 {
        0
    } else {
        lines.abs().ceil().copysign(lines) as i32
    }
}

/// The route out, or nothing at all while it is still unknown.
fn network_cell<'a, Message: 'static>(
    style: &Style,
    network: &Network,
) -> Option<Slot<'a, Message>> {
    match network {
        Network::Unknown => None,
        Network::Offline => Some(text_cell(style, Wear::Alert, "NET --", false, None)),
        Network::Wired { interface } => Some(text_cell(
            style,
            Wear::Idle,
            format!("NET {}", clip(interface, 12)),
            false,
            None,
        )),
        Network::Wireless { interface, ssid } => {
            let name = if ssid.is_empty() { interface } else { ssid };
            Some(text_cell(
                style,
                Wear::Idle,
                format!("WIFI {}", clip(name, 16)),
                false,
                None,
            ))
        }
    }
}

/// The hostname tape at the far left.
///
/// Not a [`Wear`]: three eras give the tape a silhouette no other
/// module has -- neomil's blunt-pointed code tape with its barcode
/// ticks, kitsch's stepped USER box, neokitsch's veneer plate with a
/// chamfered top-right -- and its label is set against the leading edge
/// rather than centred, because that is where the ticks leave room.
fn host_tape<'a, Message: 'static>(style: &Style, host: &str) -> Slot<'a, Message> {
    let b = &style.bar;
    let per_char = style.metrics.text_body as f32 * b.em;
    let width = (host.chars().count() as f32 * per_char).ceil() + b.pad_x * 2.0 + b.tape_extra;
    // The ticks own the first 26px of the plate, so the name starts
    // after them rather than at the ordinary inset.
    let lead = if b.tape_ticks { 32.0 } else { b.pad_x };
    let content = ink_text(style, b.tape.ink, false, host.to_string()).into();
    Slot {
        gap: 0.0,
        width,
        element: plate(style, &b.tape, width, Some(lead), b.tape_ticks, content),
    }
}

/// The focused window's title, in whatever the era makes of it.
fn window_label<'a, Message: 'static>(style: &Style, window: &str) -> Element<'a, Message> {
    let w = style.bar.window;
    let mut text = ink_text(style, w.ink, false, window.to_string());
    if let Some(face) = w.face {
        text = text.font(font_of(face));
    }
    match w.dress {
        // Bare text: entropism's long open centre string, and
        // neokitsch's annotation hanging under the wire bridge.
        None => container(text)
            .padding(Padding::ZERO.left(if w.leading { w.pad_x } else { 0.0 }))
            .center_y(Length::Fill)
            .into(),
        // A box: neomil's tab box and kitsch's DESCRIPTION box, both a
        // plainer shape than the modules either side of them.
        //
        // Sized by the text rather than by `width_for`'s estimate,
        // which is the one place on the bar where that is right: a
        // module keeps a fixed width so the row does not reflow when a
        // reading ticks, and a window title is not a reading. It is
        // also what the designs measured -- neomil's box is 239 wide
        // for a label the per-character estimate calls 276.
        Some(dress) => {
            let height = style.bar.height as f32 - style.bar.pad_y * 2.0;
            container(layered(
                face_canvas(style, &dress, w.stroke.unwrap_or(style.bar.stroke)),
                container(text)
                    .padding(Padding::from([0.0, w.pad_x]))
                    .center_y(Length::Fixed(height)),
            ))
            .center_y(Length::Fill)
            .into()
        }
    }
}

/// The whole bar. `height` should match `Style::bar.height`, which is
/// also what the layer surface reserves as its exclusive zone.
///
/// `on_tray` is how a clicked tray cell gets back to whoever can talk
/// to the item; `None` draws the same bar with no hit-testing at all.
/// It is a parameter rather than a field on [`Readings`] because it is
/// the one thing here that is not a *reading* -- the readings are what
/// the machine is doing, and this is what the bar should do about it.
pub fn bar<'a, Message: Clone + 'static>(
    style: &Style,
    r: &'a Readings,
    on_tray: Option<OnTray<Message>>,
) -> Element<'a, Message> {
    let b = &style.bar;

    let mut left: Vec<Slot<'a, Message>> = Vec::new();
    let tape = b.host_tape && !r.host.is_empty();
    if tape {
        left.push(host_tape(style, &r.host));
    }
    for (index, ws) in r.workspaces.iter().enumerate() {
        let wear = if ws.active {
            Wear::Selected
        } else {
            Wear::Idle
        };
        let mut dress = dress_for(style, wear);
        if let Some(corners) = b.ws_corners {
            dress.corners = corners;
        }
        let mut slot = worn_cell(
            style,
            dress,
            wear,
            ws.id.to_string(),
            b.bold_tiers,
            Some(b.ws_width),
        );
        slot.gap = if index == 0 && tape {
            b.ws_lead
        } else {
            b.ws_gap
        };
        left.push(slot);
    }

    // Built by pushing rather than as a literal, because the audio and
    // network modules are absent -- not blank -- when their subsystem
    // has nothing to say.
    let mut right: Vec<Slot<'a, Message>> = Vec::new();
    // Tray first, so the modules that are always present keep a fixed
    // distance from the right edge; an application starting up should
    // not move the clock.
    for (index, item) in r.tray.iter().enumerate() {
        right.push(tray_cell(style, item, index, on_tray));
    }
    if let Some(network) = network_cell(style, &r.network) {
        right.push(network);
    }
    if let Some(audio) = &r.audio {
        right.push(audio_cell(style, audio));
    }
    right.push(text_cell(
        style,
        Wear::Idle,
        format!("CPU {:>2}%", r.cpu),
        false,
        None,
    ));
    right.push(text_cell(
        style,
        Wear::Idle,
        format!("MEM {:>2}%", r.memory),
        false,
        None,
    ));
    right.push(text_cell(style, Wear::Idle, r.date.as_str(), false, None));
    right.push(match b.clock_plain {
        // Neokitsch's clock is the login screen's: larger, lighter and
        // in no box at all, the only clock anywhere in the run.
        Some((size, face)) => {
            let label = r.clock.clone();
            let width = text_width(style, &label) * f32::from(size)
                / style.metrics.text_body as f32;
            Slot {
                gap: b.gap,
                width,
                // Pinned to the same estimate the run is measured
                // with. A shrink-width clock would let the face's own
                // advance decide where the whole tray starts, and the
                // run is right-anchored, so every module left of it
                // would move with the hour. The digits sit against the
                // right of that reservation, since the design right-
                // aligns them to the bar's edge and the estimate runs
                // a little wide of the face's advance.
                element: container(
                    ink_text(style, b.idle.ink, false, label)
                        .size(f32::from(size))
                        .font(font_of(face)),
                )
                .width(Length::Fixed(width))
                .align_x(iced::alignment::Horizontal::Right)
                .center_y(Length::Fill)
                .into(),
            }
        }
        None => text_cell(
            style,
            Wear::Selected,
            r.clock.as_str(),
            b.bold_tiers,
            None,
        ),
    });

    if let Some(first) = left.first_mut() {
        first.gap = 0.0;
    }
    if let Some(first) = right.first_mut() {
        first.gap = 0.0;
    }

    let (left_row, left_metrics) = run(left);
    let (right_row, right_metrics) = run(right);

    let centre: Element<'a, Message> = if r.window.is_empty() {
        Space::new()
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into()
    } else {
        window_label(style, &r.window)
    };

    let modules = if b.window.leading {
        row![
            left_row,
            centre,
            Space::new().width(Length::Fill).height(Length::Shrink),
            right_row,
        ]
    } else {
        row![
            left_row,
            Space::new().width(Length::Fill).height(Length::Shrink),
            centre,
            Space::new().width(Length::Fill).height(Length::Shrink),
            right_row,
        ]
    };

    // Under `BarChrome::Frame` the modules are segments of the frame and
    // the design measures them from its centreline: entropism's frame
    // is drawn at x 6..8 and its first segment starts at 7, its last
    // ends at 1593 inside a right pad of 6. So the module row sits
    // inside the centreline rectangle, half a stroke in from the
    // padding, and `Strip` lays its dividers out from the same origin.
    let edge = frame_edge(b);

    let strip = Strip {
        ground: b.ground,
        chrome: b.chrome,
        ornament: b.ornament,
        chrome_ink: match b.ground {
            BarGround::Band { rule, .. } => ink_of(style, rule),
            _ => ink_of(style, b.menu.panel.stroke),
        },
        ornament_ink: match b.ornament {
            BarOrnament::Bracket => ink_of(style, Ink::Emphasis),
            _ => ink_of(style, Ink::Banner),
        },
        stroke: b.stroke,
        pad_left: b.pad_left,
        pad_right: b.pad_right,
        pad_y: b.pad_y,
        left: left_metrics,
        right: right_metrics,
    };

    let mut layers = stack![];
    if let BarGround::Haze { prims } = b.ground {
        layers = layers.push(
            canvas(Haze { style: *style, prims })
                .width(Length::Fill)
                .height(Length::Fill),
        );
    }
    layers = layers.push(canvas(strip).width(Length::Fill).height(Length::Fill));
    layers = layers.push(
        container(modules.align_y(iced::Alignment::Center).height(Length::Fill))
            .padding(Padding {
                top: b.pad_y + edge,
                right: b.pad_right + edge,
                bottom: b.pad_y + edge,
                left: b.pad_left + edge,
            })
            .width(Length::Fill)
            .height(Length::Fill),
    );

    container(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Era;

    fn style() -> Style {
        crate::eras::style(Era::Neomil)
    }

    fn command(id: i32, label: &str) -> MenuEntry {
        MenuEntry {
            id,
            label: label.to_string(),
            enabled: true,
            kind: MenuKind::Command,
            icon: None,
            children: Vec::new(),
        }
    }

    fn separator(id: i32) -> MenuEntry {
        MenuEntry {
            kind: MenuKind::Separator,
            ..command(id, "")
        }
    }

    fn submenu(id: i32, label: &str, children: Vec<MenuEntry>) -> MenuEntry {
        MenuEntry {
            kind: MenuKind::Submenu,
            children,
            ..command(id, label)
        }
    }

    /// A menu shaped like pasystray's: a couple of commands, a rule,
    /// and a submenu with something in it.
    fn menu() -> TrayMenu {
        TrayMenu {
            entries: vec![
                command(1, "Default Server"),
                separator(2),
                submenu(
                    3,
                    "Default Sink",
                    vec![command(4, "Dummy Output"), command(5, "Headphones")],
                ),
                command(6, "Quit"),
            ],
        }
    }

    #[test]
    fn a_separator_is_shorter_than_a_row() {
        let style = style();
        assert!(row_height(&style, &separator(1)) < row_height(&style, &command(1, "Quit")));
    }

    #[test]
    fn a_closed_menu_is_one_panel_wide() {
        let style = style();
        let menu = menu();
        assert_eq!(levels(&style, &menu, &[]).len(), 1);
        assert_eq!(
            menu_chain_width(&style, &menu, &[]),
            level_width(&style, &menu.entries)
        );
    }

    #[test]
    fn an_open_submenu_adds_its_own_panel_and_nothing_else() {
        let style = style();
        let menu = menu();
        let open = [2];

        let levels = levels(&style, &menu, &open);
        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0].open, Some(2));
        assert_eq!(levels[1].entries.len(), 2);

        assert_eq!(
            menu_chain_width(&style, &menu, &open),
            level_width(&style, &menu.entries)
                + level_width(&style, &menu.entries[2].children)
                + style.bar.menu.level_gap
        );
    }

    #[test]
    fn a_child_panel_starts_level_with_the_row_that_opened_it() {
        let style = style();
        let menu = menu();
        let levels = levels(&style, &menu, &[2]);

        // The two rows above the submenu, and nothing else: both
        // panels carry the same top padding, so it cancels.
        let expected = row_pitch(&style, &menu.entries[0]) + row_pitch(&style, &menu.entries[1]);
        assert_eq!(levels[0].top, 0.0);
        assert_eq!(levels[1].top, expected);
    }

    #[test]
    fn a_path_that_outruns_the_tree_stops_where_the_tree_does() {
        let style = style();
        let menu = menu();

        // Past the end of the root panel.
        assert_eq!(levels(&style, &menu, &[99]).len(), 1);
        // A row with no children of its own -- which is what an
        // application offering an empty submenu produces.
        assert_eq!(levels(&style, &menu, &[0]).len(), 1);
        // One level deeper than the tree goes.
        assert_eq!(levels(&style, &menu, &[2, 0]).len(), 2);
    }

    #[test]
    fn one_row_with_an_icon_widens_the_whole_panel() {
        let style = style();
        // Long enough that neither width lands on the clamp, which is
        // what makes the difference the gutter and not the floor.
        let plain = vec![command(1, "Recording Streams and Modules")];
        let mut iconned = plain.clone();
        iconned[0].icon = Some(image::Handle::from_rgba(1, 1, vec![0, 0, 0, 255]));

        assert_eq!(
            level_width(&style, &iconned) - level_width(&style, &plain),
            style.bar.menu.icon_col + style.bar.menu.icon_gap
        );
    }

    /// Every era's bar is the same drawing wearing a different table,
    /// so the table is the thing worth asserting on: four eras, four
    /// silhouettes, and not one `if era ==` in this file.
    #[test]
    fn the_four_eras_dress_a_module_differently() {
        use crate::widgets::surface::Cut;

        let cell = |era: Era| crate::eras::style(era).bar.idle.corners;
        assert_eq!(cell(Era::Entropism), Corners::square());
        assert_eq!(
            cell(Era::Neomil).bottom_left,
            Cut::Chamfer { x: 6.0, y: 6.0 }
        );
        assert_eq!(cell(Era::Kitsch), Corners::all(Cut::Round { radius: 8.0 }));
        assert_eq!(
            cell(Era::Neokitsch).bottom_left,
            Cut::Chamfer { x: 10.0, y: 7.0 }
        );
        assert_eq!(cell(Era::Neokitsch).top_left, Cut::Round { radius: 3.0 });
    }
}
