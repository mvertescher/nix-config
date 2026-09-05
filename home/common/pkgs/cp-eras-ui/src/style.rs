//! What separates one era from another, as data.
//!
//! The four eras were sampled from the Behance references (see
//! `docs/<era>/README.md`). Laid side by side they differ in a handful
//! of parameters and two or three genuine decorations -- not in their
//! widget vocabulary. All four render the same 4ST store screen, which
//! is why [`crate::screens::store`] has one implementation and four
//! dresses.
//!
//! So this is a struct of values, not a trait to implement. An era is a
//! `Style` you can print, diff, and unit-test; adding a fifth is filling
//! in a table, and a widget that wants to branch on era has to justify
//! why the parameter it needs is not already here.

use crate::palette::Palette;
use crate::widgets::surface::Corners;
use iced::Color;

/// How a surface treats its corners. Sampled per era; this single
/// parameter carries most of the visual difference between them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Corner {
    /// Entropism: 1px boxes, right angles, no exceptions.
    Square,
    /// Neo-militarism: a diagonal cut. Which corners are cut varies by
    /// widget, so the amount lives here and the choice at the call site.
    Chamfer { cut: f32 },
    /// Kitsch: everything rounded, no chamfers anywhere.
    Round { radius: f32 },
    /// Neokitsch: square but for a single clipped top-right corner.
    ClipTopRight { cut: f32 },
}

impl Corner {
    /// The inset a corner treatment eats, for callers that need to keep
    /// content clear of it.
    pub fn inset(self) -> f32 {
        match self {
            Corner::Square => 0.0,
            Corner::Chamfer { cut } | Corner::ClipTopRight { cut } => cut,
            Corner::Round { radius } => radius,
        }
    }
}

/// How the selected element is filled. Three of the four eras are a flat
/// fill in `palette.select`; neokitsch fills with a material.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Selection {
    Solid,
    /// Neokitsch: wood veneer. The references use a photographic fill;
    /// this approximates it with a warp gradient plus grain lines, which
    /// is close enough at UI scale and keeps the crate asset-free.
    Veneer,
}

/// Page background treatment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ground {
    /// Entropism: flat, or near enough. The reference has a faint CRT
    /// vertical falloff and nothing else.
    Flat,
    /// Kitsch and neokitsch: a coloured radial bloom vignetting the
    /// screen from a corner or the top edge. `x`/`y` are in fractions of
    /// the window; `radius` likewise.
    Bloom { x: f32, y: f32, radius: f32 },
}

/// A colour the bar names by *role* rather than by value, so a
/// published theme still moves it.
///
/// The four bar designs between them reach for eleven inks, and all but
/// four of those are roles the palette already publishes. [`Ink::Fixed`]
/// is the escape hatch for the remainder -- neomil's two band stops and
/// its card dark, neokitsch's light veneer -- which are era constants
/// with no semantic slot to sit in and no theme that could retint them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ink {
    /// Nothing is drawn.
    None,
    Bg,
    Border,
    Dim,
    Fg,
    /// Between `dim` and `fg`, 60% of the way: what every trace's
    /// outline ink actually samples as. See `widgets::text::mid_ink`.
    Mid,
    Alert,
    Tape,
    /// The era's selection *material*: a flat fill in three eras and
    /// wood veneer in the fourth, resolved through
    /// [`crate::widgets::surface::Surface::selected`].
    Select,
    OnSelect,
    /// The era's solid decoration colour.
    Ornament,
    /// The lit edge of the era's relief pair.
    Relief,
    /// The era's accent-band fill.
    Banner,
    /// The era's emphasis fill, falling back to `fg`.
    Emphasis,
    /// The era's call-to-action fill.
    Cta,
    /// The era's recessed fill, or `panel` where it declares none.
    Inset,
    /// A colour no published role covers.
    Fixed(Color),
}

impl Ink {
    /// The ink as a colour under `palette`, for the canvas screens that
    /// never pass [`Ink::None`]; if one does, it gets the foreground.
    pub fn of(self, palette: &crate::palette::Palette) -> Color {
        Style::ink_in(palette, self).unwrap_or(palette.fg)
    }
}

/// The face an era sets its bar labels in.
///
/// A weight rather than a family: all four designs set Rajdhani and
/// disagree only about how heavy. Kitsch's note is the reason it is a
/// table entry and not a constant -- "a 1.25px line next to 400-weight
/// Rajdhani reads heavier than the text", so the era compensates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Regular,
    Medium,
    SemiBold,
    Bold,
}

/// How one class of bar module is dressed: its silhouette and its inks.
///
/// Four of these -- idle, selected, alert, tape -- plus the two the menu
/// carries are the whole of an era's bar vocabulary, which is why
/// `bar.rs` needs no era test anywhere: a module asks the table which
/// dress it wears and paints it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dress {
    pub corners: Corners,
    pub fill: Ink,
    pub stroke: Ink,
    /// Label and icon colour.
    pub ink: Ink,
    /// The era's [`Tab`] stands on this module's bottom edge.
    pub tab: bool,
    /// Kitsch's stepped box, as `(run, rise)`: the bottom edge sits
    /// `rise` higher everywhere past `run` from the module's leading
    /// edge, joined by a short diagonal.
    ///
    /// Not a [`Cut`](crate::widgets::surface::Cut) because a corner
    /// treatment eats a *corner*: the GUES 7702 box keeps its full
    /// height for its first 26px and drops the rest of the bottom
    /// edge, which is a notch out of the middle of an edge and not out
    /// of an end of it.
    pub step: Option<(f32, f32)>,
}

impl Default for Dress {
    fn default() -> Self {
        Dress {
            corners: Corners::square(),
            fill: Ink::None,
            stroke: Ink::None,
            ink: Ink::Fg,
            tab: false,
            step: None,
        }
    }
}

/// The filled trapezoid neokitsch stands on the bottom edge of every
/// outlined thing it draws.
///
/// Geometry rather than a bool on [`Dress`] because the two sizes are
/// one shape at two scales -- `base 22 / top 16` on a readout,
/// `base 14 / top 8` on a workspace or icon cell -- and the era table is
/// where that pair belongs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tab {
    pub base: f32,
    pub top: f32,
    pub height: f32,
    /// How far the base's right end sits inside the module's right edge.
    pub inset: f32,
    /// The narrow form, and the module width below which it is used.
    pub narrow_base: f32,
    pub narrow_top: f32,
    pub narrow_below: f32,
    pub fill: Ink,
}

/// The strip's own background.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarGround {
    /// The page ground shows through: the bar is its modules and
    /// nothing else.
    Plain,
    /// Neomil: the header glow, a left-to-right gradient closed by a
    /// rule along its foot.
    Band {
        left: Color,
        right: Color,
        rule: Ink,
        rule_width: f32,
    },
    /// Neokitsch: the violet haze the top of every screen in the run
    /// sits in, as the lobe `store-trace.svg` measures -- centre,
    /// radius, vertical squash and four stops -- clipped to the strip
    /// by the bar's own bounds.
    ///
    /// Not [`Ground::Bloom`], which is what the era declares for a
    /// *page*: at 31px the page bloom reads as a thin dark wash, where
    /// the traces are violet across the middle 1200px of the strip and
    /// near-black only at its ends. The stops are the measurement, so
    /// they travel with the era rather than with the drawing.
    Haze {
        cx: f32,
        cy: f32,
        r: f32,
        /// How far the lobe is flattened vertically.
        squash: f32,
        /// `(offset, colour)`, inner to outer.
        stops: [(f32, Color); 5],
    },
}

/// Chrome the bar draws around and between its modules.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarChrome {
    /// Modules stand alone with air between them.
    Loose,
    /// Entropism: the bar *is* the era's header strip -- one outlined
    /// frame with a divider on every module boundary inside a run, and
    /// no gaps at all.
    Frame,
}

/// A decoration the bar draws on its own canvas, under the modules.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarOrnament {
    None,
    /// Kitsch: a mint bracket down the left edge and along the foot,
    /// fading out to the right.
    Bracket,
    /// Neokitsch: the header wire band bridging the centre gap.
    Wire,
}

/// Where the focused-window label sits and what it wears.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowLabel {
    /// The box the label sits in; `None` draws bare text.
    pub dress: Option<Dress>,
    pub ink: Ink,
    /// Set against the left run rather than centred in the free span,
    /// the way entropism's header strings sit after their divider.
    pub leading: bool,
    pub pad_x: f32,
    /// The box's stroke width where it is not the bar's: neomil draws
    /// its tab box at 1.0 inside a bar whose every other line is 1.5
    /// (`bar.svg` §9, mailbox-trace x 241..451). `None` uses
    /// [`Bar::stroke`].
    pub stroke: Option<f32>,
    /// The face the label is set in where it is not the bar's: neokitsch
    /// sets its strip at 600 and the annotation under the wire bridge
    /// at 400 (`bar.svg` §6). `None` uses [`Bar::face`].
    pub face: Option<Face>,
}

/// What a dbusmenu separator is, in an era's own vocabulary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuRule {
    /// A rule inset from the panel's sides.
    Inset,
    /// Kitsch: a rule edge to edge.
    Full,
    /// Entropism: an empty cell of this height, between the two row
    /// dividers that already bound it -- not a floating rule.
    Empty { height: f32 },
    /// Neokitsch: a rule carrying a [`Tab`] standing on it.
    Tabbed,
}

/// The marker on a row that opens a submenu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuMarker {
    /// The `<` glyph.
    Text,
    /// Neomil: the era's own left arrow -- a shaft and a triangular
    /// head, `w` by `h`.
    Arrow { w: f32, h: f32 },
}

/// What the bar draws outside or inside a menu panel's own outline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelEcho {
    None,
    /// Neomil: a filled bar riding the right edge on slanted ends, the
    /// edge stepping `step` inward below it, and two hairline echoes of
    /// that edge trailing to the panel's foot.
    EdgeBar { step: f32, top: f32, len: f32 },
    /// Neokitsch: `count` echo outlines nested *inside* the panel at
    /// `pitch`, sharing its left edge and bottom-left corner and
    /// stepping in on the other three sides, fading inward. The rows
    /// sit inside the innermost one (see [`BarMenu::ring_inset`]).
    Rings { count: usize, pitch: f32 },
    /// Kitsch: the solid curl in the panel's foot.
    Wave,
}

/// How an era dresses the tray menu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarMenu {
    pub panel: Dress,
    /// Air inside a panel, around its column of rows.
    pub air: f32,
    pub side: f32,
    /// Air above and below one row's content, and either side of it.
    pub row_air: f32,
    pub row_side: f32,
    /// Width of the reserved icon column, and the gap after it.
    pub icon_col: f32,
    pub icon_gap: f32,
    /// Gap between adjacent panels of the open chain.
    pub level_gap: f32,
    /// Air a panel adds beyond its widest row: the two side insets
    /// plus whatever the era leaves the label clear of its own edge.
    pub level_pad: f32,
    /// Entropism: a rule after every row but the last, so the pitch is
    /// the row plus the stroke.
    pub row_divider: bool,
    pub rule: MenuRule,
    /// A highlighted row -- a set toggle, or the one under the pointer.
    pub row: Dress,
    /// The row whose submenu is open. The same dress as `row` in three
    /// eras; neokitsch outlines it instead of filling it, because its
    /// material says "chosen" and an outline says "current".
    pub open: Dress,
    /// How far the open-parent's face is inset, where it differs.
    pub open_inset: (f32, f32),
    /// Kitsch: the selected row is drawn in two pieces split by a 2px
    /// gap -- an icon cell this wide, in these corners, then the body.
    /// `None` in the three eras whose selection is one shape.
    pub row_split: Option<(f32, Corners)>,
    /// Ink of a disabled row's label.
    pub disabled: Ink,
    /// Ink of a separator rule.
    pub rule_ink: Ink,
    /// How far a highlighted row is inset from the panel's left and
    /// right inner edges.
    pub row_inset: (f32, f32),
    /// How far a highlighted row's plate runs *past* the root panel's
    /// right outline. Neokitsch 8: the mail bar runs past its own list
    /// rules (483 -> 512 in the trace), and the menu's highlight past
    /// the panel and its rings the same way. Zero elsewhere. Root panel
    /// only -- a submenu's would run under the panel it hangs off.
    pub row_overshoot: f32,
    /// A spine down the left of a highlighted row, and how far it runs.
    pub spine: f32,
    /// Extra room at the foot of the root panel, for [`PanelEcho`].
    pub foot: f32,
    pub marker: MenuMarker,
    pub echo: PanelEcho,
}

/// Status-bar shape. Mirrors the `barHeight` and `hostTape` knobs the
/// nix era builder already takes, so the bar and the generated waybar
/// config cannot disagree about how tall the bar is -- which matters,
/// because the height is also the exclusive zone the compositor
/// reserves.
///
/// Everything after those two is the era's *dress*, added 2026-09-03
/// when `bar.rs` followed the four `docs/<era>/bar.svg` designs. It is a
/// long table and that is the point: the four bars differ in a hundred
/// figures and not one line of drawing, so `bar.rs` stays a single
/// implementation and a fifth era is an entry here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bar {
    pub height: u32,
    /// Show the hostname as a tape-coloured label at the far left.
    pub host_tape: bool,

    /// Air the strip leaves at its left edge, its right edge, and above
    /// and below the module row.
    pub pad_left: f32,
    pub pad_right: f32,
    pub pad_y: f32,
    /// Gap between adjacent modules in a run.
    pub gap: f32,
    /// The same, between workspace cells, which every era pitches
    /// tighter than its readouts.
    pub ws_gap: f32,
    /// The same, between the host tape and the first workspace, which
    /// every era leaves wider than the run itself.
    pub ws_lead: f32,
    /// Width of one workspace cell. Fixed in all four eras: a run of
    /// numbered cells is a run of equal cells.
    pub ws_width: f32,
    /// The silhouette of a workspace cell, where it is not the
    /// silhouette of every other module. Kitsch alone: its workspaces
    /// are the store nav's chevrons and its readouts are the customer
    /// chip, and no single corner treatment is both.
    pub ws_corners: Option<Corners>,
    /// Horizontal padding inside a module.
    pub pad_x: f32,
    /// Extra width a module reserves past its label, beyond `pad_x` --
    /// neokitsch's tab zone, which no label may sit on.
    pub trail: f32,
    /// Per-character width of body text, in ems. The label's own
    /// measure, so cells stop reflowing when a reading ticks.
    pub em: f32,
    /// The same for a space, which in Rajdhani is a third of a digit's
    /// advance. Equal to `em` in the three eras whose designs sized
    /// their cells by counting characters -- their SVGs are literally
    /// `chars * 0.58em + 26` -- and a third of it in neokitsch, whose
    /// designer measured the labels instead: `VOL  62%` is 97 wide
    /// there against the 109 a uniform count gives, and the twelve
    /// pixels are its two spaces.
    pub space_em: f32,
    /// Tracking the era sets its CTA plate's label with, reserved in
    /// the plate's width.
    ///
    /// Reserved rather than drawn: iced 0.14 exposes no letter-spacing,
    /// so the label is set solid inside a plate sized for the spaced
    /// one. That is the faithful half of a thing we cannot do all of --
    /// the alternative is a plate 8px narrower than every one in the
    /// material.
    pub alert_track: f32,
    /// Stroke width of every line the bar draws.
    pub stroke: f32,
    /// Width a tray icon cell adds around its pixmap.
    pub icon_pad: f32,
    /// Labels are set against the module's leading edge rather than
    /// centred in it -- neokitsch, whose right end belongs to the tab.
    pub label_left: bool,
    /// The face every label is set in.
    pub face: Face,
    /// Extra width the host tape takes past an ordinary module's.
    pub tape_extra: f32,
    /// The tape carries the era's barcode ticks.
    pub tape_ticks: bool,

    pub ground: BarGround,
    pub chrome: BarChrome,
    pub ornament: BarOrnament,

    pub idle: Dress,
    pub selected: Dress,
    pub alert: Dress,
    pub tape: Dress,
    pub tab: Option<Tab>,
    pub window: WindowLabel,

    /// The era spells alarm as this suffix on the label, in the same
    /// ink as its neighbours, rather than by moving an ink.
    pub alert_suffix: Option<&'static str>,
    /// The workspace digits and the clock are set in the bold face.
    pub bold_tiers: bool,
    /// The clock is plain text rather than a module, at this size and
    /// in this face -- neokitsch's 18px at 500 (`bar.svg` §7) in a strip
    /// set at 600.
    pub clock_plain: Option<(u16, Face)>,

    pub menu: BarMenu,
}

impl BarMenu {
    /// How far the rows keep clear of the panel's top, right and
    /// bottom edges for [`PanelEcho::Rings`] -- the rings' whole depth.
    /// Zero in every other era. The left edge is the rings' own, so it
    /// is not padded.
    pub fn ring_inset(&self) -> f32 {
        match self.echo {
            PanelEcho::Rings { count, pitch } => count as f32 * pitch,
            _ => 0.0,
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            height: 31,
            host_tape: true,

            pad_left: 6.0,
            pad_right: 6.0,
            pad_y: 3.0,
            gap: 6.4,
            ws_gap: 6.4,
            ws_lead: 6.4,
            ws_width: 35.0,
            ws_corners: None,
            pad_x: 13.0,
            trail: 0.0,
            em: 0.58,
            space_em: 0.58,
            alert_track: 0.0,
            stroke: 1.0,
            icon_pad: 18.0,
            label_left: false,
            face: Face::Regular,
            tape_extra: 4.0,
            tape_ticks: false,

            ground: BarGround::Plain,
            chrome: BarChrome::Loose,
            ornament: BarOrnament::None,

            idle: Dress::default(),
            selected: Dress {
                fill: Ink::Select,
                ink: Ink::OnSelect,
                ..Dress::default()
            },
            alert: Dress {
                stroke: Ink::Alert,
                ink: Ink::Alert,
                ..Dress::default()
            },
            tape: Dress {
                fill: Ink::Tape,
                ink: Ink::OnSelect,
                ..Dress::default()
            },
            tab: None,
            window: WindowLabel {
                dress: None,
                ink: Ink::Dim,
                leading: false,
                pad_x: 8.0,
                stroke: None,
                face: None,
            },

            alert_suffix: None,
            bold_tiers: false,
            clock_plain: None,

            menu: BarMenu {
                panel: Dress {
                    fill: Ink::Bg,
                    stroke: Ink::Border,
                    ..Dress::default()
                },
                air: 6.0,
                side: 4.0,
                row_air: 3.0,
                row_side: 8.0,
                icon_col: 16.0,
                icon_gap: 6.0,
                level_gap: 0.0,
                level_pad: 24.0,
                row_divider: false,
                rule: MenuRule::Inset,
                row: Dress {
                    fill: Ink::Select,
                    ink: Ink::OnSelect,
                    ..Dress::default()
                },
                open: Dress {
                    fill: Ink::Select,
                    ink: Ink::OnSelect,
                    ..Dress::default()
                },
                open_inset: (0.0, 0.0),
                row_split: None,
                disabled: Ink::Dim,
                rule_ink: Ink::Border,
                row_inset: (0.0, 0.0),
                row_overshoot: 0.0,
                spine: 0.0,
                foot: 0.0,
                marker: MenuMarker::Text,
                echo: PanelEcho::None,
            },
        }
    }
}

/// Where a card carries its name. Structural rather than decorative:
/// three eras head their cards, neokitsch foots them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nameplate {
    Header,
    Footer,
}

/// The shape of an accent band, alongside the colours in
/// [`crate::palette::Ornaments::banner`].
///
/// Two numbers, both measured off the design targets rather than
/// guessed. Kitsch's shelf band is
/// `M360 228 h242 v20 h-230 l-12 8 Z` against a card whose left edge is
/// at 372: it hangs 12px past the surface and its trailing corner steps
/// down 8. Neokitsch's footer nameplate is a plain `rect` -- no step --
/// but it hangs by the same 12: `x=340 w=188` against a card at
/// `x=352 w=176`, in `docs/neokitsch/target-components.svg` (the by-eye
/// sheet replaced 2026-09-03 by `components.svg`, rebuilt from the
/// traces; every `target-components.svg` citation in this file is the
/// old sheet's, i.e. what the code was built to), and
/// `x=506 w=244` against a card at `x=520 w=230` in its `target-app`
/// (deleted 2026-09-03; `store-trace.svg` draws the nameplate as a tab
/// under the card's bottom edge).
/// (An earlier reading of this called it flush with the card. It is
/// not.) The minimalist eras have no banner at all and take the
/// default, which draws a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Banner {
    /// How far the band hangs past the leading edge of its surface.
    pub overhang: f32,
    /// Depth of the step cut into the trailing corner.
    pub notch: f32,
}

/// The outward wedge kitsch cuts into the top-right of a nav pill.
///
/// Pill-specific rather than era-wide, and the targets are explicit
/// about it: `docs/kitsch/target-app.svg` (deleted 2026-09-03 -- and
/// `store-trace.svg` disagrees: the categories are 216x39 peaked chevrons
/// on a 60px pitch, not pills; this widget is the composite's reading, see
/// `TODO.md`) draws every category as
/// `M172 340 h158 l18 15 v13 q0 12 -12 12 h-164 q-12 0 -12 -12 v-16
/// q0 -12 12 -12 Z` -- a `radius: 16` pill whose top-right corner is
/// not rounded at all but juts *out* by 18 and *down* by 15 -- while
/// the product cards beside it are plain `rx="16"` and the socket cells
/// plain `rect`s. So this is not [`Corner`]: a corner treatment applies
/// to the whole era's containers, and this applies to one widget and
/// *adds* width rather than eating it.
///
/// The other three eras leave it at the default. Neokitsch is the near
/// miss worth recording: it does own a step-notch shape
/// (`M60 658 h150 v20 l-8 8 h-142 Z`, `target-components.svg`), but its
/// nav pills are plain `rx="4"` rects and the notch belongs to the
/// mailbox footer, so wiring it here would put it on the wrong widget.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ticket {
    /// How far the wedge reaches past the pill's body, to the right.
    pub reach: f32,
    /// How far the wedge's outer point drops below the top edge.
    pub drop: f32,
}

impl Ticket {
    /// Whether the era cuts one at all. Zero draws nothing, so the
    /// shape code degrades to the plain corner walk.
    pub fn is_cut(self) -> bool {
        self.reach > 0.0 && self.drop > 0.0
    }
}

/// Where a screen puts its footnote markers.
///
/// Not decoration and not one rule with four dresses: the three store
/// targets disagree about it structurally, and an earlier pass that
/// sank the markers to the foot of the window with `Length::Fill`
/// matched none of them. Entropism stacks A and B directly under the
/// nav and lets the lower third of the column stay empty; kitsch sets a
/// single A halfway down, beneath the page-curl, and puts C under the
/// right of the shelf; neokitsch runs A and C along the top strata rail
/// and drops B under the cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Footnotes {
    /// Entropism, neomil: stacked under the nav.
    UnderNav,
    /// Kitsch: one marker mid-column under the ornament, one under the
    /// shelf.
    MidColumn,
    /// Neokitsch: a rail above the content, plus one under the shelf.
    TopRail,
}

/// Whether an era stamps the two-line compliance notice on a product
/// card, and where.
///
/// Three values rather than a bool because the two eras that draw it
/// disagree about which side of the card outline it belongs on, and
/// reading the targets settled a question an earlier note had backwards:
///
/// * entropism (`docs/entropism/target-app.svg`, deleted 2026-09-03;
///   `store-trace.svg` keeps the CC35 micro-caption inside each card)
///   sets it *inside* the
///   card -- `text x=538 y=642` against `rect x=520 y=320 w=270 h=360`,
///   so 38px above the card's own bottom edge -- and omits it from the
///   selected card, which spends that room on its detail block.
/// * kitsch sets it *outside*: `text x=520 y=608` against a card ending
///   at `y=586`, flush with the card's leading edge rather than its
///   padding, and on all four cards including the selected one.
/// * neokitsch's cards end in a footer nameplate and carry no notice at
///   all; neomil's target is an ops dashboard with no store screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compliance {
    None,
    /// Inside the outline, under the sockets, on unselected cards only.
    Inside,
    /// Below the card, aligned to its leading edge, on every card.
    Below,
}

/// Chrome conventions: what an era puts at the top and bottom of every
/// screen. All four have something; they disagree on what.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Chrome {
    /// Entropism: a row of outlined boxes across the top, a build-string
    /// footer across the bottom.
    Segmented,
    /// Neomil: a thin rule with a hostname tape.
    Tape,
    /// Kitsch: no top bar; a single centred compliance caption.
    Caption,
    /// Neokitsch: the device frame *is* the chrome -- double gold stroke,
    /// stepped corners, a strata wedge at the foot.
    DeviceFrame,
}

/// Numbers a screen needs that are not colours or shapes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metrics {
    pub stroke: f32,
    /// Gap between sibling cards and list rows.
    pub gap: f32,
    /// Interior padding of a surface.
    pub pad: f32,
    pub text_body: u16,
    pub text_caption: u16,
    pub text_title: u16,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics {
            stroke: 1.0,
            gap: 16.0,
            pad: 16.0,
            text_body: 14,
            text_caption: 9,
            text_title: 19,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub era: Era,
    pub palette: Palette,
    pub corner: Corner,
    pub selection: Selection,
    pub ground: Ground,
    pub chrome: Chrome,
    pub nameplate: Nameplate,
    pub bar: Bar,
    pub metrics: Metrics,
    /// Geometry for the accent band; its colours live on the palette.
    /// Default (a flush rectangle) for the two minimalist eras, which
    /// declare no banner colours either and so never draw one.
    pub banner: Banner,
    /// Where the footnote markers go.
    pub footnotes: Footnotes,
    /// Whether a product card carries the compliance notice, and on
    /// which side of its outline.
    pub compliance: Compliance,
    /// The wedge a nav pill cuts into its top-right corner. Zero in
    /// every era but kitsch.
    pub ticket: Ticket,
    /// Whether the era stamps compliance glyphs -- dotted matrix,
    /// hollow square, hollow triangle -- on its bands and rows.
    ///
    /// A parameter rather than a widget-side era test because it is a
    /// fact about the era and not about any one widget: the same three
    /// marks head kitsch's shelf band and lead its EMPTY SOCKET row,
    /// and no other era's references carry them anywhere.
    pub glyphs: bool,
    // --- login ---
    /// The era's access screen, measured off its login trace. See the
    /// `--- login ---` block at the foot of this file for why the
    /// geometry is table data rather than screen code.
    pub access: Access,
    // --- end login ---
    // --- mailbox ---
    /// What this era's mailbox *is*, straight off
    /// `docs/<era>/mailbox-trace.svg`: geometry in the trace's own
    /// 1600x900 frame, colours by role, line art as polylines. The four
    /// traces disagree structurally -- a framed list against boxed rows
    /// against bare rows hanging in a bracket against ruled ones -- and
    /// this is where that disagreement lives, so `screens::mail` stays
    /// one implementation. See [`Mailbox`].
    pub mailbox: Mailbox,
    // --- end mailbox ---

    // --- store ---
    /// The era's store screen, as the scene its
    /// `docs/<era>/store-trace.svg` measures. See the store section at
    /// the foot of this file for why this one screen is a display list
    /// and every other is a composition.
    pub store: &'static [Prim],
    /// Which category and which card the store opens on, as
    /// `(category, card)`. The traces disagree -- entropism grows its
    /// *first* card and the other three their second -- so the opening
    /// state is era data, not a constant of the screen.
    pub store_selection: (usize, usize),
    // --- end store ---

    // --- dashboard ---
    /// The era's module-hub dashboard, as the scene its
    /// `docs/<era>/dashboard-trace.svg` measures: the same `Prim`
    /// vocabulary as [`Style::store`], at the trace's own 1600x900. Each
    /// of the six menu units is a [`Prim::Plate`] in
    /// [`Group::Module`]. Lives in the `// --- dashboard ---` block of
    /// `src/eras/<era>.rs`.
    pub dashboard: &'static [Prim],
    /// Which module (0..6) the dashboard opens on: the one the era's
    /// trace shows filled. The traces disagree -- neomil fills a
    /// diamond, entropism BRAINDANCE, kitsch EVENTS, neokitsch EMAIL --
    /// so the opening state is era data, not a constant of the screen.
    pub dashboard_selection: usize,
    // --- end dashboard ---
}

/// The four UI eras of the reference material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Era {
    Entropism,
    Kitsch,
    Neomil,
    Neokitsch,
}

impl Era {
    pub const ALL: [Era; 4] = [Era::Entropism, Era::Kitsch, Era::Neomil, Era::Neokitsch];

    /// Match the `era = "..."` key the theme layer publishes. Unknown
    /// names are not an error here -- the caller decides whether to fall
    /// back or complain.
    pub fn parse(s: &str) -> Option<Era> {
        match s.trim().to_ascii_lowercase().as_str() {
            "entropism" => Some(Era::Entropism),
            "kitsch" => Some(Era::Kitsch),
            "neomil" | "neomilitarism" | "neo-militarism" => Some(Era::Neomil),
            "neokitsch" | "neo-kitsch" => Some(Era::Neokitsch),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Era::Entropism => "entropism",
            Era::Kitsch => "kitsch",
            Era::Neomil => "neomil",
            Era::Neokitsch => "neokitsch",
        }
    }

    /// The era's motto, as printed on its title card.
    pub fn motto(self) -> &'static str {
        match self {
            Era::Entropism => "NECESSITY OVER STYLE",
            Era::Kitsch => "STYLE OVER SUBSTANCE",
            Era::Neomil => "SUBSTANCE OVER STYLE",
            Era::Neokitsch => "SUBSTANCE AND STYLE",
        }
    }

    /// The reference-sampled style, with no theme file applied.
    pub fn style(self) -> Style {
        crate::eras::style(self)
    }
}

impl Style {
    /// Colours for an accent band, as `(fill, ink)`.
    ///
    /// Total, like every accessor in this group: an era that declares
    /// no ornament gets a documented degradation rather than an
    /// `Option`, so a banner widget is written once and worn by four
    /// eras the way every other widget here is. Pair it with
    /// [`Style::banner`] for the shape.
    pub fn banner_colors(&self) -> (Color, Color) {
        self.palette.banner()
    }

    /// The same band, on a *selected* element, as `(fill, ink)`.
    ///
    /// A band in the selection fill, on a card in the selection fill,
    /// is invisible -- so both maximalist eras move it, and they move
    /// it in opposite directions. See
    /// [`crate::palette::Palette::banner_on_select`] for why that is a
    /// sampled pair rather than a derivation or a published role.
    pub fn banner_on_select(&self) -> (Color, Color) {
        self.palette.banner_on_select()
    }

    /// Whether this era declares an accent band at all.
    ///
    /// [`Style::banner_colors`] is total so that a banner widget needs
    /// no era branch. Whether a card *wears* one is a different
    /// question, and it is exactly the information the absence carries:
    /// entropism and neomil head their cards with a hairline and a
    /// caption, and a tape-coloured band across them would be an
    /// invention rather than a degradation.
    pub fn banded(&self) -> bool {
        self.palette.ornaments.banner.is_some()
    }

    /// The lit and shaded edges of a raised surface, as
    /// `(bevel, shade)`. Equal in an era with no relief, which draws
    /// the flat box the minimalist eras already use.
    pub fn relief(&self) -> (Color, Color) {
        self.palette.relief()
    }

    /// Colour for non-structural decoration -- curls, strata, flags.
    pub fn ornament(&self) -> Color {
        self.palette.ornament()
    }

    /// Resolve one of the bar's named inks against this style.
    ///
    /// Total but for [`Ink::None`], which is the one answer that means
    /// "draw nothing" rather than "draw this": a `Dress` with no fill
    /// is an outline on the ground, and an `Option` is how the drawing
    /// code hears that without an era test.
    pub fn ink(&self, ink: Ink) -> Option<Color> {
        Style::ink_in(&self.palette, ink)
    }

    fn ink_in(p: &crate::palette::Palette, ink: Ink) -> Option<Color> {
        Some(match ink {
            Ink::None => return None,
            Ink::Bg => p.bg,
            Ink::Border => p.border,
            Ink::Dim => p.dim,
            Ink::Fg => p.fg,
            Ink::Mid => {
                let mix = |a: f32, b: f32| a * 0.4 + b * 0.6;
                Color {
                    r: mix(p.dim.r, p.fg.r),
                    g: mix(p.dim.g, p.fg.g),
                    b: mix(p.dim.b, p.fg.b),
                    a: p.fg.a,
                }
            }
            Ink::Alert => p.alert,
            Ink::Tape => p.tape,
            Ink::Select => p.select,
            Ink::OnSelect => p.on_select,
            Ink::Ornament => p.ornament(),
            Ink::Relief => p.relief().0,
            Ink::Banner => p.banner().0,
            Ink::Emphasis => p.emphasis.map(|(fill, _)| fill).unwrap_or(p.fg),
            Ink::Cta => p.cta,
            Ink::Inset => p.ornaments.inset.unwrap_or(p.panel),
            Ink::Fixed(color) => color,
        })
    }

    /// The style for whatever era the desktop is currently in, with the
    /// published roles overlaid. This is what an app should call: it
    /// follows `switch` without a rebuild.
    pub fn from_desktop() -> Style {
        let theme = crate::theme::Theme::load();
        let era = Era::parse(&theme.era).unwrap_or(Era::Neomil);
        let mut style = era.style();
        style.palette = style.palette.with_theme(&theme);
        style
    }
}

// --- login ---
//
// The access screen, as data. Everything between this marker and the
// closing one belongs to `screens::login`; it is kept in one block so
// the other screens' blocks merge beside it without touching each
// other.
//
// Why the geometry is *here* rather than in the screen: the four
// `docs/<era>/login-trace.svg` files are not one composition in four
// dresses. Entropism's access screen is a label, a field and a button
// alone in an empty frame; neomil's is three dossier cards under a
// badge header; kitsch's is three chip-headed guest rows inside a
// full-height bracket; neokitsch's is two identical entry groups over a
// wire band. They *do* share a grammar -- some number of account slots,
// one of them live, each with a name, a mark and a control -- and that
// grammar is [`Slot`]. What they do not share is where anything sits,
// so the measured coordinates come from the era table the same way
// [`Access`] and [`Style::store`] do, and `screens::login` names no era.
//
// Coordinates are in the traces' own 1600x900 frame and are transcribed
// from them verbatim. The screen scales them to whatever window it is
// given.

/// A box in the trace's 1600x900 frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plot {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Plot {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Plot {
        Plot { x, y, w, h }
    }
}

/// Per-corner chamfer depth, in design pixels; zero leaves the corner
/// square.
///
/// Deliberately *not* [`Corner`]: a corner treatment is an era-wide
/// property of containers, and the login traces cut individual corners
/// at amounts their era table does not carry -- neomil's active card
/// takes 46 off its top right where the era's own figure is 15, and
/// neokitsch's ENTER bar takes 16 off its bottom left where the era
/// clips the *top* right at 30. This is the same argument
/// [`crate::widgets::surface::Corners`] makes, restated in a form the
/// era table can hold.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bevel {
    pub tl: f32,
    pub tr: f32,
    pub br: f32,
    pub bl: f32,
}

impl Bevel {
    pub const NONE: Bevel = Bevel {
        tl: 0.0,
        tr: 0.0,
        br: 0.0,
        bl: 0.0,
    };

    pub const fn tr(cut: f32) -> Bevel {
        Bevel { tr: cut, ..Bevel::NONE }
    }

    pub const fn br(cut: f32) -> Bevel {
        Bevel { br: cut, ..Bevel::NONE }
    }

    pub const fn bl(cut: f32) -> Bevel {
        Bevel { bl: cut, ..Bevel::NONE }
    }
}

/// A shoulder in a plate's top edge.
///
/// Kitsch's ENTER bar is not a bevelled rectangle and cannot be made
/// into one: `M257,470 H418 L430,463 H591.5 V497.5 H257 Z` -- the right
/// two fifths of the bar stand 7px taller than the left, joined by a
/// short diagonal. The same shoulder is on both PROTECTED bars beside
/// it, so it is the era's bar shape rather than one widget's accident.
/// `drop` is how far the *left* portion sits below the plate's top
/// edge, and `run` the width of the diagonal that climbs out of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    pub x: f32,
    pub drop: f32,
    pub run: f32,
}

/// A filled and/or stroked plate: the one shape every part of an access
/// screen is drawn as.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plate {
    pub at: Plot,
    pub bevel: Bevel,
    /// A shoulder in the top edge; `None` is a level one.
    pub step: Option<Step>,
    pub fill: Option<Ink>,
    /// Bottom stop where the era grades its fill downward. Neomil's
    /// unselected cards are translucent red over a blue glow and so
    /// read lighter at the top than at the foot; every other plate in
    /// the four traces is flat, and leaves this `None`.
    pub foot: Option<Ink>,
    pub stroke: Option<Ink>,
    pub weight: f32,
}

impl Plate {
    pub const fn filled(at: Plot, fill: Ink) -> Plate {
        Plate {
            at,
            bevel: Bevel::NONE,
            step: None,
            fill: Some(fill),
            foot: None,
            stroke: None,
            weight: 0.0,
        }
    }

    pub const fn outlined(at: Plot, stroke: Ink, weight: f32) -> Plate {
        Plate {
            at,
            bevel: Bevel::NONE,
            step: None,
            fill: None,
            foot: None,
            stroke: Some(stroke),
            weight,
        }
    }

    pub const fn bevelled(mut self, bevel: Bevel) -> Plate {
        self.bevel = bevel;
        self
    }

    pub const fn stepped(mut self, x: f32, drop: f32, run: f32) -> Plate {
        self.step = Some(Step { x, drop, run });
        self
    }

    pub const fn over(mut self, fill: Ink) -> Plate {
        self.fill = Some(fill);
        self
    }

    pub const fn grading(mut self, foot: Ink) -> Plate {
        self.foot = Some(foot);
        self
    }

    pub const fn edged(mut self, stroke: Ink, weight: f32) -> Plate {
        self.stroke = Some(stroke);
        self.weight = weight;
        self
    }
}

/// One run of text, positioned the way the traces position text: by the
/// baseline, because that is what an SVG `<text y=...>` means and the
/// coordinates here are transcribed from those elements.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Legend {
    pub text: &'static str,
    pub x: f32,
    pub baseline: f32,
    pub size: f32,
    pub weight: iced::font::Weight,
    pub ink: Ink,
    /// `x` is the run's centre rather than its left edge -- the traces'
    /// `text-anchor="middle"`.
    pub centred: bool,
    /// Quarter-turn anticlockwise about `(x, baseline)`: the rotated
    /// micro-text neomil runs down both margins.
    pub turned: bool,
    /// Horizontal scale about `x`.
    ///
    /// The sources are set in a wider face than Rajdhani, and the
    /// polished traces stopped pretending otherwise: they stretch the
    /// runs whose width was measured -- neokitsch's ARASAKA logotype
    /// `scale(1.155 1)` onto its measured ink extent x 92..272,
    /// kitsch's guest notes `scale(1.32 1)` and the boxed letter
    /// `scale(1.7 1)`. `textLength` would have been the SVG way and
    /// librsvg ignores it, which is why the traces carry a transform
    /// and why this is a number rather than a target width.
    pub stretch: f32,
    /// Letter-spacing, in design pixels between glyphs.
    ///
    /// Carried as the traces carry it. `screens::login` has no glyph
    /// spacing to set -- the toolkit has no tracking -- so it fits the
    /// run's *extent* instead, by measuring the natural width and
    /// stretching by `(w + tracking * (glyphs - 1)) / w`. The ink ends
    /// where the trace says it ends, which is what was measured off the
    /// photo and what the extractor reads; the difference is inside the
    /// glyphs rather than between them.
    pub tracking: f32,
}

impl Legend {
    pub const fn new(text: &'static str, x: f32, baseline: f32, size: f32, ink: Ink) -> Legend {
        Legend {
            text,
            x,
            baseline,
            size,
            weight: iced::font::Weight::Normal,
            ink,
            centred: false,
            turned: false,
            stretch: 1.0,
            tracking: 0.0,
        }
    }

    pub const fn bold(mut self) -> Legend {
        self.weight = iced::font::Weight::Bold;
        self
    }

    pub const fn medium(mut self) -> Legend {
        self.weight = iced::font::Weight::Medium;
        self
    }

    pub const fn light(mut self) -> Legend {
        self.weight = iced::font::Weight::Light;
        self
    }

    pub const fn centred(mut self) -> Legend {
        self.centred = true;
        self
    }

    pub const fn turned(mut self) -> Legend {
        self.turned = true;
        self
    }

    pub const fn stretched(mut self, stretch: f32) -> Legend {
        self.stretch = stretch;
        self
    }

    pub const fn tracked(mut self, tracking: f32) -> Legend {
        self.tracking = tracking;
        self
    }
}

/// What an era draws inside a slot's identity box.
///
/// A variant rather than geometry because the three eras that draw one
/// draw genuinely different objects, and none of them is a dressed
/// rectangle: neomil's live card carries a wire hexagon over four
/// dashes and a cluster of compliance marks, its other two carry a
/// photographic portrait, and kitsch's is a printed chip -- a dark
/// hexagon split by a teal slash with a hatched wedge in one corner.
/// The figures are constants of one drawing, so they live in the
/// screen; which drawing an era uses is the era's business, so it lives
/// here. Same division as [`Chrome`] and [`Footnotes`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emblem {
    None,
    /// Neomil's active card: the wire hexagon and its satellites.
    Hexagon,
    /// Neomil's other cards: a portrait silhouette filling the box.
    Portrait,
    /// Kitsch: the printed chip.
    Chip,
}

/// One account slot.
///
/// The grammar all four traces share, and the reason this screen is one
/// implementation rather than four: an access screen offers some number
/// of accounts, each with a mark, a name, a footnote and a control, and
/// exactly one of them is the live one. Everything an era does not draw
/// is `None` -- entropism's single slot is a name, a field and a
/// button, and it is the same struct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slot {
    /// The card behind everything, where the era draws one.
    pub body: Option<Plate>,
    /// The darker lower section neomil's unselected cards carry.
    pub foot: Option<Plate>,
    /// The dark notch cut into a card's leading edge, and the rail
    /// beside it.
    pub notch: Option<Plot>,
    /// The identity box: an avatar, a chip.
    pub mark: Option<Plate>,
    pub emblem: Emblem,
    pub name: Option<Legend>,
    /// The label over the field, where the era sets one.
    pub prompt: Option<Legend>,
    pub field: Option<Plate>,
    pub value: Option<Legend>,
    /// The insertion mark: entropism's underline beneath the first pair
    /// of masked characters, kitsch's block in the field, neomil's on
    /// its Login button.
    pub caret: Option<Plate>,
    /// The control that commits, or the bar that says you may not.
    pub action: Option<Plate>,
    pub action_label: Option<Legend>,
    /// The boxed footnote letter and its micro-text.
    pub badge: Option<Plate>,
    pub badge_letter: Option<Legend>,
    pub notes: &'static [Legend],
}

impl Slot {
    pub const EMPTY: Slot = Slot {
        body: None,
        foot: None,
        notch: None,
        mark: None,
        emblem: Emblem::None,
        name: None,
        prompt: None,
        field: None,
        value: None,
        caret: None,
        action: None,
        action_label: None,
        badge: None,
        badge_letter: None,
        notes: &[],
    };
}

/// What an era puts across the top of its access screen.
///
/// A variant beside [`Chrome`] rather than a use of it: `Chrome` says
/// what an era's *screens* wear, and three of the four traces show this
/// screen wearing something else. Entropism's login does carry the
/// segmented strip, but neomil replaces the hub's tape with a full
/// dossier block, kitsch drops the top bar to a bare clock, and
/// neokitsch heads the screen with its logotype. So the login trace,
/// not the era's house rule, decides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Masthead {
    /// Entropism: one outlined strip divided into cells.
    Strip {
        plate: Plate,
        dividers: &'static [f32],
        labels: &'static [Legend],
    },
    /// Neomil: the dossier block -- a customer badge at the left, a
    /// protocol barcode and code tape in the middle, four security
    /// badges at the right, all over a full-width hairline rule. The
    /// badge outline and the barcode dashes are constants of the
    /// drawing and live in the screen; where the badges sit, and which
    /// of them is filled, is measured and lives here.
    Dossier {
        badges: &'static [Plate],
        rule: Plate,
        labels: &'static [Legend],
    },
    /// Kitsch: nothing but the clock.
    Clock { labels: &'static [Legend] },
    /// Neokitsch: the ARASAKA logotype over a two-cell caption box, and
    /// the clock at the far right.
    Logotype {
        cell: Plate,
        divider: f32,
        labels: &'static [Legend],
    },
}

/// The one large decoration an era carries on this screen.
///
/// Same rule as [`Masthead`]: the figures belong to one drawing and
/// live in the screen, the choice belongs to the era and lives here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fixture {
    /// Entropism. The emptiness is the design.
    None,
    /// Neomil: a numbered chip and rotated micro-text down each margin.
    /// The chip is a fixed cluster -- two dashes, a dot and a filled
    /// square -- so the screen draws it; where the squares sit is
    /// measured and lives here.
    Margins {
        chips: &'static [Plot],
        labels: &'static [Legend],
    },
    /// Kitsch: the full-height bracket, the filled lobe outside its
    /// diagonal, and the barcode standing in its foot.
    Bracket {
        left: f32,
        right: f32,
        knee: f32,
        foot: f32,
        barcode: Plot,
        labels: &'static [Legend],
    },
    /// Neokitsch: the stacked-hairline wire band across the foot.
    WireBand {
        /// Outer plateau, centre plateau, and the ends' vertical.
        outer: f32,
        inner: f32,
        end: f32,
        strands: usize,
    },
}

/// The foot of the access screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colophon {
    /// Neomil: nothing. The cards carry their own notices and the
    /// bottom fifth of the frame is ground.
    None,
    /// Entropism: a solid band -- the one large fill on the screen --
    /// carrying three dark strings along its bottom edge.
    Band {
        plate: Plate,
        labels: &'static [Legend],
    },
    /// Kitsch, neokitsch: one line of compliance micro-text.
    Notice { labels: &'static [Legend] },
}

/// What an era washes over the page on *this* screen, beyond its
/// declared [`Ground`].
///
/// [`Ground`] is the era's house background and is shared with every
/// other screen; neomil's login (and its hub -- the trace records the
/// two backdrops as pixel-identical at every sampled row) puts a broad
/// cold-blue glow over the top half of the frame that `Ground::Flat`
/// does not describe and that `Ground::Bloom` cannot: it is a
/// horizontal gradient under a vertical falloff, not a disc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wash {
    /// The era's [`Ground`] is the whole story.
    Plain,
    /// Entropism: the faint warm radial lift the login photo carries
    /// over its near-black ground.
    ///
    /// `Ground::Flat` is right about the era and wrong about this
    /// screen, and the shape gate is what proved it: the trace's lift
    /// is bright enough at its centre to be a palette cluster of its
    /// own that never reaches the frame edge, so the extractor bins it
    /// as *ink* and reads a 1044x586 shape in the middle of the
    /// screen -- 72% of the design's shape area, against a flat ground
    /// that offers nothing to match it.
    WarmLift,
    /// Neomil: the cold-blue glow, gone by y~420, over a warm near-black
    /// vignette down the left margin.
    ColdGlow,
    /// Kitsch: the rose bloom out of the top edge, and the grey-green
    /// cast down the left margin.
    ///
    /// Same argument as [`Wash::VioletHaze`], and the same measurement:
    /// the era's declared bloom is a disc out of the top *right*
    /// (`x: 0.82`), sampled off its store sheet, where the login photo
    /// blooms from the top *centre* and reaches `#a34e60` at y 10. A
    /// screen's backdrop is a fact about the screen here, not only
    /// about the era.
    RoseBloom,
    /// Neokitsch: the violet-over-black haze the whole run wears, with
    /// the cold-blue lobe inside it.
    ///
    /// `Ground::Bloom`'s stacked translucent discs cap out around 6%
    /// alpha and reach a tenth of this: measured on the two renders,
    /// the trace's haze is `#4f4262` at the top centre where the bloom
    /// puts `#1f1f33`. That is not a shade -- it is the difference
    /// between the backdrop holding two of the frame's palette
    /// clusters and holding none, which is what the shape gate saw.
    VioletHaze,
}

/// The era's access screen, measured off `docs/<era>/login-trace.svg`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Access {
    pub wash: Wash,
    pub masthead: Masthead,
    /// Left to right, as the trace lays them out.
    pub slots: &'static [Slot],
    pub fixture: Fixture,
    pub colophon: Colophon,
}

// --- end login ---

// --- mailbox ---
//
// The mailbox is the one screen with a *photo-shaped* trace per era
// (`docs/<era>/mailbox-trace.svg`), and the four traces disagree about
// more than dress: entropism frames its list, neomil boxes every row
// beside a column of cartridge icons, kitsch hangs five bare rows inside
// a teal bracket, neokitsch rules them and puts the envelope on the
// right. `screens::mail` still has one implementation, because
// everything the four disagree about is in the table below -- geometry
// in the trace's own 1600x900 frame, palette by role, and the era's
// incidental line art as polylines. A fifth era is a table entry, and
// nothing in `screens/` asks which era it is.

/// A rectangle in a trace's own 1600x900 frame.
///
/// Deliberately *not* fractions: every trace measures in that frame, so
/// the table reads the way the SVG does and the screen scales once.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Frame {
    pub const ZERO: Frame = Frame::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Frame {
        Frame { x, y, w, h }
    }

    /// The same box, moved. Rows are one frame plus a pitch.
    pub fn shifted(self, dx: f32, dy: f32) -> Frame {
        Frame {
            x: self.x + dx,
            y: self.y + dy,
            ..self
        }
    }
}

/// Corner bits, in `extract_spec.py`'s own order so a table entry and a
/// gate report name the same corner.
pub const TL: u8 = 1;
pub const TR: u8 = 2;
pub const BR: u8 = 4;
pub const BL: u8 = 8;

/// Which corners a mailbox shape cuts, how deep, and whether the cut is
/// a diagonal or an arc.
///
/// Not [`Corner`]: that is the era's *default* treatment for its
/// containers, and the mailbox traces cut per widget -- neomil's rows
/// cut bottom-left where the era's default cuts bottom-right, kitsch's
/// selection body cuts a *diagonal* where the era rounds everything
/// else. The era table still supplies the amounts; this says where they
/// land.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trim {
    pub corners: u8,
    pub cut: f32,
    pub round: bool,
}

impl Trim {
    pub const NONE: Trim = Trim {
        corners: 0,
        cut: 0.0,
        round: false,
    };

    pub const fn chamfer(corners: u8, cut: f32) -> Trim {
        Trim {
            corners,
            cut,
            round: false,
        }
    }

    pub const fn round(corners: u8, cut: f32) -> Trim {
        Trim {
            corners,
            cut,
            round: true,
        }
    }
}

/// Where a run of text sits, at what size, in which role.
///
/// Used both for the era's own fixed strings ([`Note`]) and for slots
/// the *screen* fills with content it owns -- the subject line, the
/// sender, a button's label.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Run {
    pub x: f32,
    /// The text's baseline, as the SVG writes it.
    pub y: f32,
    pub size: f32,
    pub ink: Ink,
    pub bold: bool,
    /// Rajdhani 500. The re-cut traces set most chrome at weight 500 or
    /// 600, which neither `bold` nor the regular face carries.
    pub medium: bool,
    /// Rajdhani 600; wins over `medium`.
    pub semibold: bool,
    /// Centre on `x` rather than starting at it.
    pub center: bool,
    /// End at `x` rather than starting at it.
    pub right: bool,
}

impl Run {
    pub const fn new(x: f32, y: f32, size: f32, ink: Ink) -> Run {
        Run {
            x,
            y,
            size,
            ink,
            bold: false,
            medium: false,
            semibold: false,
            center: false,
            right: false,
        }
    }

    pub const fn bold(mut self) -> Run {
        self.bold = true;
        self
    }

    pub const fn medium(mut self) -> Run {
        self.medium = true;
        self
    }

    pub const fn semibold(mut self) -> Run {
        self.semibold = true;
        self
    }

    pub const fn centered(mut self) -> Run {
        self.center = true;
        self
    }

    pub const fn right(mut self) -> Run {
        self.right = true;
        self
    }
}

/// One era-owned string: the in-fiction chrome the traces are covered
/// in. Content the *screen* owns -- subjects, senders, body copy --
/// never appears here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub at: Run,
    pub text: &'static str,
}

/// One drawn element of an era's mailbox chrome.
///
/// Three kinds cover all four traces. `Box` is every framed or filled
/// rectangle; `Poly` is the era's line art -- kitsch's bracket and its
/// wave, neokitsch's wire band and folder badge, neomil's code tape --
/// as a polyline in design coordinates, which is the one escape hatch
/// that keeps a bespoke ornament out of `screens/`; `Label` is a fixed
/// string.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Box {
        at: Frame,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
        trim: Trim,
    },
    Poly {
        points: &'static [(f32, f32)],
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
        close: bool,
    },
    Curve {
        start: (f32, f32),
        steps: &'static [Seg],
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
        close: bool,
    },
    Label(Note),
}

/// What an era puts behind an *unselected* list row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowDecor {
    /// Entropism: one outlined frame around the whole list, hairline
    /// dividers between rows.
    Framed,
    /// Neomil: a filled, stroked box per row with a spine at its left.
    Boxed,
    /// Kitsch: nothing at all -- the rows hang inside the bracket.
    Bare,
    /// Neokitsch: a hairline under each row carrying a small filled tab.
    Ruled,
}

/// Where a row's sender sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromAt {
    /// Three traces: a second line under the subject.
    Beneath,
    /// Neomil: right-aligned on the subject's own line.
    Trailing,
}

/// The wood veneer neokitsch fills its selection bar with.
///
/// [`crate::widgets::surface::Fill::Veneer`] synthesises a plank for a
/// laid-out widget -- banded warp plus grain, alpha-blended so it reads
/// as figured. The mailbox bar is *measured* instead, and the contrast
/// matters beyond looks: the trace draws 32 hairlines at a 1.7 pitch in
/// #cf975c on a #f8c678 base, and `extract_spec.py` bins those two
/// tones as separate ink families. A bar whose grain is blended away
/// reads as one shape where the design reads as sixteen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Veneer {
    pub base: Color,
    pub grain: Color,
    /// Line-to-line pitch and stroke width, both measured.
    pub pitch: f32,
    pub width: f32,
    /// The grain's zigzag along the plank, and it is not decoration:
    /// the trace's lines turn every 26px through a 2.4px sway, and that
    /// 52px period is what makes `extract_spec.py` split the bar into
    /// sixteen cells instead of one. A plank drawn with straight grain
    /// reads as a single shape against a design that reads as sixteen.
    /// `turn` is the half-period, `sway` the peak-to-peak amplitude and
    /// `phase` the x of the first vertex.
    pub turn: f32,
    pub sway: f32,
    pub phase: f32,
}

/// The column of isometric cartridge icons neomil sets beside its list,
/// and no other era draws. `None` everywhere else, which is how the
/// screen knows not to draw one without asking which era it is in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Icons {
    /// The top vertex of the first cartridge.
    pub x: f32,
    pub y: f32,
    pub pitch: f32,
}

/// One row of an era's mailbox, transcribed from its trace.
///
/// The four traces do *not* show the same inbox. Three of them list
/// variations on "You'll regret that / Urgent information (!) / Heist
/// data sent to you / ..." with different senders, different lengths and
/// different envelopes open; neomil's is "List of messages / I'm worried
/// man / Heist data sent to you / ..." with every row from Jackie. So
/// the rows live in the era table, and the screen draws whatever the
/// table says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mail {
    /// Set in the case the era's `title_upper` / `from_upper` flags
    /// produce: an era that shouts its subjects stores them in sentence
    /// case here and the screen uppercases them, so the same text reads
    /// the same way in the list and over the message.
    pub subject: &'static str,
    pub from: &'static str,
    /// An unread message shows an open-flap envelope in every trace
    /// that draws envelopes, and carries the NEW pill in the one era
    /// that draws pills instead.
    pub unread: bool,
}

/// Region A of every trace: the message list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MailList {
    /// An outlined frame around the whole list (entropism only).
    pub frame: Option<Frame>,
    pub frame_ink: Ink,
    pub frame_width: f32,
    /// The first row's box; every other row is this plus `pitch`.
    pub row: Frame,
    pub pitch: f32,
    /// The rows, top to bottom, exactly as many as the trace shows.
    pub rows: &'static [Mail],
    pub selected: usize,
    pub decor: RowDecor,
    pub row_fill: Option<Ink>,
    pub row_stroke: Option<Ink>,
    pub row_width: f32,
    pub row_trim: Trim,
    /// Neomil's 3px spine, relative to the row's top-left.
    pub spine: Option<Frame>,
    /// The divider a `Framed` or `Ruled` era draws at a row's foot,
    /// relative to the row's top-left.
    pub rule: Option<Frame>,
    pub rule_ink: Ink,
    /// The filled trapezoid neokitsch sets on each divider, relative to
    /// the row's top-left: `(x, y, w, h)`, drawn as a trapezoid whose
    /// top edge is inset by `h * 0.6`.
    pub tab: Option<Frame>,
    pub tab_ink: Ink,
    /// The selection fill, in absolute coordinates on its own row.
    pub sel: Frame,
    pub sel_trim: Trim,
    /// What it is filled with: `Ink::Select` everywhere but kitsch, whose
    /// trace draws the row a stop deeper than the panel tab (`#e8c21f`
    /// against `#fbd42c`, mailbox-trace.svg:224/256) -- the photo's
    /// selected row samples `#ccad19`, the tab `#fed82e`. The icon cell
    /// takes the same fill.
    pub sel_fill: Ink,
    /// Kitsch splits its selection in two: an icon cell, a 2px gap, and
    /// the body above. Absolute, like `sel`.
    pub sel_icon: Option<Frame>,
    pub sel_icon_trim: Trim,
    /// The dark outlined notch neokitsch cuts into the selection bar's
    /// bottom edge -- its tab motif, inverted. Absolute, like `sel`.
    pub sel_notch: Option<Frame>,
    /// The material the selection is filled with, where the era fills
    /// it with one rather than with `palette.select`.
    pub veneer: Option<Veneer>,
    /// The envelope glyph: left edge, offset down the row, width.
    pub glyph_x: f32,
    pub glyph_dy: f32,
    pub glyph_w: f32,
    pub text_x: f32,
    pub title_dy: f32,
    pub title_size: f32,
    pub title_bold: bool,
    pub from_dy: f32,
    pub from_size: f32,
    pub from_at: FromAt,
    /// What leads the sender line: "FROM: ", "from: ", or nothing at
    /// all where neomil sets the name on the subject's own line.
    pub from_prefix: &'static str,
    /// Entropism and kitsch shout their subjects, neokitsch shouts only
    /// its senders, neomil shouts neither. Two flags rather than one
    /// because no era sets them the same way.
    pub title_upper: bool,
    pub from_upper: bool,
    /// The outlined NEW pill neomil sets in a row's lower right, on
    /// every row whose [`Mail::unread`] is set. `None` everywhere else.
    pub new_pill: Option<Frame>,
    pub icons: Option<Icons>,
}

/// Region B of every trace: the message itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MailPanel {
    /// The panel outline. `None` for neokitsch, whose message is plain
    /// text on the ground.
    pub frame: Option<Frame>,
    pub frame_fill: Option<Ink>,
    pub frame_stroke: Option<Ink>,
    pub frame_width: f32,
    pub frame_trim: Trim,
    /// The filled title bar (entropism) or tab (kitsch) over the body.
    pub head: Option<Frame>,
    pub head_ink: Ink,
    pub head_trim: Trim,
    /// Which row the panel is reading at rest. Not always the selected
    /// row: entropism reads row 1 out of a list whose row 0 is
    /// selected, and the trace is unambiguous about both. A click moves
    /// the panel onto the clicked row.
    pub message: usize,
    pub title: Run,
    pub title_upper: bool,
    /// Neomil prints no sender beside its panel title.
    pub from: Option<Run>,
    /// What the trace sets in `title` / `from` at rest, where that is
    /// *not* the shown row's own subject / sender under the list's
    /// prefix and casing rules. `None` derives from `rows[message]`.
    /// Two traces need it: entropism heads its message "from: Mom"
    /// above a list that says "FROM: MOM", and neomil's panel reads
    /// "Urgent Information (!)", which is no row of its list at all.
    /// Only the resting state is pinned; once a click moves the panel
    /// off `message`, both are derived.
    pub heading: Option<&'static str>,
    pub sender: Option<&'static str>,
    /// Where the first body line's baseline sits, and how it is set.
    pub body: Run,
    /// Baseline-to-baseline within a paragraph, and between them.
    pub line: f32,
    pub para: f32,
    /// The body copy: paragraphs of lines, each line exactly as the
    /// trace sets it, hyphenated breaks ("incidi-" / "dunt") included.
    /// The traces break the same lorem four different ways -- two of
    /// them without its third paragraph -- and set each line
    /// explicitly, so there is nothing to wrap: the screen draws one
    /// run per entry.
    pub paragraphs: &'static [&'static [&'static str]],
}

/// Region C: the action buttons, or whatever the era puts where they go
/// -- kitsch stacks four chevron tabs down the right instead, which is
/// the same object with `dy` rather than `dx`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MailButtons {
    pub first: Frame,
    pub dx: f32,
    pub dy: f32,
    pub count: usize,
    /// Which one is filled, if any, and with what: `Ink::Select` everywhere
    /// but kitsch, whose DETAILS chevron is `#e6c020` (mailbox-trace.svg:280)
    /// -- the row's deeper yellow, not the tab's.
    pub filled: Option<usize>,
    pub fill: Ink,
    /// The fill of the idle ones. Neomil's three unselected buttons are
    /// filled `#1a0607` under their outline (mailbox-trace.svg:353); the
    /// other eras leave theirs open.
    pub idle_fill: Option<Ink>,
    /// Entropism draws its four as one outlined strip with dividers.
    pub joined: bool,
    /// Kitsch's tab shape: a peak on the leading edge, a cut trailing
    /// corner.
    pub chevron: bool,
    pub trim: Trim,
    pub width: f32,
    pub stroke: Ink,
    /// Label position, relative to the button's top-left.
    pub label: Run,
    /// Neokitsch's filled trapezoid on the bottom edge, relative.
    pub tab: Option<Frame>,
    pub labels: &'static [&'static str],
}

/// Region D: the clearance badges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MailBadges {
    pub first: Frame,
    pub dx: f32,
    pub dy: f32,
    pub cols: usize,
    pub count: usize,
    pub selected: Option<usize>,
    pub trim: Trim,
    pub width: f32,
    pub fill: Option<Ink>,
    pub stroke: Ink,
    /// Label position, relative to the badge's top-left.
    pub label: Run,
    /// Neomil heads each badge with a LEVEL caption.
    pub caption: Option<Run>,
    pub caption_text: &'static str,
    pub labels: &'static [&'static str],
}

/// The whole of an era's mailbox, as data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mailbox {
    /// The ground this screen's own trace draws, as the leading
    /// [`Prim::Soft`] group(s) a store or dashboard table would open
    /// with, composited under the sheet by `scene::Backdrop`. Empty for
    /// an era whose mailbox is content with [`Ground`].
    ///
    /// Added 2026-09-04 when `triptych.sh --diff` lit the whole frame
    /// on the kitsch and neomil mailboxes: both had been taking the
    /// era's generic `Ground` while their traces open with their own
    /// gradient defs (`#bloom` + `#leftwash`, `#glowh` under
    /// `#glowmask` + `#wash` under `#washmask` + `#vignette`).
    pub backdrop: &'static [Prim],
    /// Header, footer, section letters, micro-print and line art: every
    /// era-owned element that is not one of the four regions below.
    pub chrome: &'static [Piece],
    /// Pieces drawn *over* the four regions: ornaments that ride a
    /// region's edge and would otherwise be buried under its fill --
    /// neomil's bright bar on the panel's right edge lost its inner
    /// half the day the panel gained its `#1c0608` fill (2026-09-04).
    pub overlay: &'static [Piece],
    pub list: MailList,
    pub panel: MailPanel,
    pub buttons: MailButtons,
    pub badges: MailBadges,
}
// --- end mailbox ---
// --- store ---------------------------------------------------------------
//
// The scene vocabulary. Everything from here to the closing marker was
// added for the SVG -> iced conversion of the four
// `docs/<era>/store-trace.svg` files and is purely additive: no field
// above changes meaning. The dashboard folded onto the same vocabulary
// on 2026-09-03 ([`Style::dashboard`]), which is why [`Group`] has a
// third member; nothing else here is dashboard-specific.
//
// ## Why the store is a display list
//
// Every other knob in this file dresses a shape the four eras agree on.
// The store traces do not agree on a shape. They agree on a *screen* --
// masthead, customer block, category nav, four weapon cards with one
// grown, footnote markers -- and then draw it with four unrelated
// pieces of furniture: entropism outlines a full-width segmented header
// strip and a footer strip; neomil has no header at all and hangs its
// nav off a 3px spine; kitsch replaces the whole left column with a
// single swept bracket that wraps the customer block and ends in a
// solid "wave"; neokitsch runs an eight-strand wire band across the top
// and puts a BASKET plate in the corner. There is no corner radius that
// turns one into another, and the geometry is not a constant of one
// drawing the way [`Chrome`]'s figures are -- it is the measured content
// of four different photographs.
//
// So the era carries the drawing and the screen carries the drawing
// *engine*. `screens::store` matches on nothing: it hands
// [`Style::store`] to `screens::scene` and that paints it. A fifth era
// is a fifth table entry, which is the same contract every other field
// here has, just with a richer value. The dashboard is the same story
// told again (a six-diamond menu, a 3x2 tile grid, two three-blade fans
// and a six-card cascade are not one shape in four dresses), so it is a
// display list too.
//
// Coordinates are the trace's own, in the 1600x900 frame the traces and
// the golden matrix share, so a figure here can be diffed against the
// SVG line it came from.

/// Which chooser a [`Prim::Plate`] belongs to.
///
/// The store screen has exactly two things a person can pick: a weapon
/// category down the nav, and a card along the shelf. The dashboard has
/// one: a module in its six-unit menu. All are in every era's material
/// and all are drawn differently in each, so the plate -- hit box plus
/// the two drawings it wears -- is scene data like everything else
/// here, and the screens hit-test it without knowing which era they are
/// in. A screen only reads the groups its scene uses; a plate in
/// another group is a table error, not a feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Category,
    Card,
    /// A dashboard menu unit, indexed 0..6.
    Module,
}

/// Horizontal anchoring of a run of scene text, matching SVG's
/// `text-anchor` so a trace's `text-anchor="end"` transcribes directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Start,
    Middle,
    End,
}

/// One segment of a scene path, in the subset of SVG's path grammar the
/// four traces actually use.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Seg {
    /// Start a new subpath (SVG `M`). Two subpaths in one [`Prim::Path`]
    /// are filled even-odd, which is how the traces cut the hole out of
    /// entropism's "4" and kitsch's gun.
    Move(f32, f32),
    /// Straight line (SVG `L`, `H`, `V`).
    Line(f32, f32),
    /// Quadratic curve (SVG `Q`).
    Quad { cx: f32, cy: f32, x: f32, y: f32 },
    /// Cubic curve (SVG `C`). Only neokitsch's header wire band needs
    /// one, and it needs it eight times: each strand leaves its low run
    /// horizontally and arrives on the bridge horizontally, which a
    /// quadratic cannot do.
    Cubic { c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32 },
}

/// One drawing operation of a scene (a store or dashboard display list).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prim {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
    },
    Path {
        /// The path's opening point (SVG `M`).
        x: f32,
        y: f32,
        segs: &'static [Seg],
        close: bool,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
    },
    /// A run of text on its SVG baseline.
    Text {
        x: f32,
        y: f32,
        size: f32,
        ink: Ink,
        face: Face,
        anchor: Anchor,
        content: &'static str,
    },
    /// Horizontally stretched text, as SVG's
    /// `textLength` + `lengthAdjust="spacingAndGlyphs"`.
    ///
    /// Two traces set the 4ST logotype in a heavy *extended* face and
    /// say so explicitly -- kitsch `scale(1.7 1)`, neokitsch
    /// `textLength="121"` over glyphs that measure 70 -- and it is not
    /// cosmetic: at natural width the two glyphs collide into one blob
    /// where the design has two, and the era loses a whole shape class.
    Wide {
        x: f32,
        y: f32,
        size: f32,
        /// Horizontal scale about `x`.
        stretch: f32,
        ink: Ink,
        face: Face,
        content: &'static str,
    },
    /// Letter-spaced text: glyph `i` sits at `x + i * pitch`. The
    /// traces set the logotype's `S T O R E` this way and iced's text
    /// has no tracking, so the scene spells it out.
    Spaced {
        x: f32,
        y: f32,
        size: f32,
        ink: Ink,
        face: Face,
        pitch: f32,
        content: &'static str,
    },
    /// Letter-spaced text, as SVG's `letter-spacing`: each glyph's own
    /// advance plus `tracking` between neighbours. Where [`Prim::Spaced`]
    /// puts glyphs on a fixed pitch, this keeps the face's fitting and
    /// opens it a little, which is how the traces set nearly every
    /// label (0.25 on the fine print, 2 on kitsch's blade labels). The
    /// scene draws it glyph by glyph off measured advances, since
    /// iced's text has no tracking; `anchor` applies to the whole
    /// tracked run.
    Tracked {
        x: f32,
        y: f32,
        size: f32,
        ink: Ink,
        face: Face,
        anchor: Anchor,
        tracking: f32,
        content: &'static str,
    },
    /// Wood-veneer grain over a fill: hairlines on a `pitch`, clipped
    /// to a rectangle.
    ///
    /// The neokitsch source fills its selection, its card-2 body and
    /// its BASKET band with veneer rather than flat gold, and the trace
    /// draws that as hundreds of clipped strokes in a mid gold. It is a
    /// texture, but it is not only a texture: the mid gold is one of
    /// the four ink families the source's palette holds, and the region
    /// it covers is the largest single shape in the whole design. The
    /// source's strands are wavy; these are straight, which is the one
    /// place this scene simplifies rather than transcribes.
    Grain {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        pitch: f32,
        width: f32,
        ink: Ink,
    },
    /// A dot matrix -- every era's socket-row "QR" glyph, which is not
    /// a QR in any of them. `rows` is one string per row, `#` for a
    /// cell that is inked.
    Dots {
        x: f32,
        y: f32,
        cell: f32,
        pitch: f32,
        ink: Ink,
        rows: &'static [&'static str],
    },
    /// A rectangle with all four corners rounded to `r`. Kitsch is the
    /// era that rounds everything and its customer chip, cards and
    /// socket cells all want one; the other three reach for it once or
    /// not at all.
    Round {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        r: f32,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
    },
    /// A radial haze, as the trace's own `radialGradient`: a centre,
    /// two radii and the stop table, expanded at paint time into
    /// concentric rings.
    ///
    /// Kitsch opens on a rose bloom and neokitsch on a violet one, and
    /// neither is skippable decoration: the extractor's palette split
    /// is a k-means over the whole image, and the sources spend four
    /// and five of their eight clusters on these hazes. Drawn flat, the
    /// spare clusters go to the golds instead and the BASKET plate
    /// splits down its own hairline.
    ///
    /// A stop table rather than the rings themselves because a
    /// hand-written band list is both long and *visibly stepped* -- the
    /// steps are what this replaces. The rings are interpolated in
    /// sRGB, which is what SVG does with these gradients.
    Lobe {
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        /// `(offset, colour)`, in gradient order.
        stops: &'static [(f32, Color)],
    },
    /// An ellipse. Two traces open with a radial haze -- kitsch's rose
    /// bloom, neokitsch's violet lobes -- and an SVG `radialGradient`
    /// in bounding-box units is an *ellipse*, 2.6:1 in kitsch's case.
    /// iced's canvas has only linear gradients, so a scene draws a
    /// radial as concentric bands of this; getting the aspect wrong
    /// puts the haze 300px down the page.
    Ellipse {
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
    },
    /// A circle. The one curve the traces draw that is not a corner:
    /// neomil's card icons hold one and kitsch's band glyphs a second.
    Circle {
        x: f32,
        y: f32,
        r: f32,
        fill: Option<Ink>,
        stroke: Option<Ink>,
        width: f32,
    },
    /// A selectable plate: a hit box and the two drawings it wears.
    ///
    /// `on` is painted when this plate's `index` is the current
    /// selection for its [`Group`], `off` otherwise. That is the whole
    /// of "the selected card is grown and the others are not": the
    /// screen holds two indices, the scene holds both drawings, and
    /// nothing branches on an era.
    Plate {
        group: Group,
        index: usize,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        on: &'static [Prim],
        off: &'static [Prim],
    },
    /// A sub-scene translated by `(x, y)`: how a card template written
    /// once gets placed four times along the shelf.
    At {
        x: f32,
        y: f32,
        prims: &'static [Prim],
    },
    /// A sub-scene translated to `(x, y)` then rotated `angle` degrees
    /// clockwise about that point: the trace's
    /// `<g transform="translate(cx cy) rotate(a)">`. Text inside turns
    /// with it, which is how a blade label lies along its blade.
    Turn {
        x: f32,
        y: f32,
        angle: f32,
        prims: &'static [Prim],
    },
    /// A sub-scene composited in software, in sRGB, and drawn as one
    /// image: for translucent fills that stack on each other or on a
    /// haze, which wgpu's linear-light blending cannot land where the
    /// trace's `fill-opacity` does. Kitsch's ghost cards over its rose
    /// bloom are the case that forced it; `screens/soft.rs` has the
    /// measurements and the scope (fills only -- no text, no plates).
    ///
    /// The group is rasterised over transparency at the frame's own
    /// size, so it should open with its opaque ground: whatever it
    /// leaves uncovered is blended linear like anything else.
    Soft {
        prims: &'static [Prim],
    },
    /// A rect filled with the trace's own `linearGradient`: the axis
    /// `from` -> `to` in bounding-box fractions (SVG's `x1 y1 x2 y2`)
    /// and the whole stop table, read in sRGB the way rsvg reads it.
    ///
    /// Not iced's gradient, which `mix`es in linear light and
    /// `smoothstep`s between its (at most eight) stops and so lands the
    /// trace only *at* a stop. Inside a [`Prim::Soft`] group it is
    /// composited exactly at any angle; on the canvas it is painted as
    /// flat strips along its axis, so there it must be horizontal or
    /// vertical -- `soft_only_prims_stay_soft` in `scene.rs` checks.
    Ramp {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        from: (f32, f32),
        to: (f32, f32),
        /// `(offset, colour)`, in gradient order.
        stops: &'static [(f32, Color)],
    },
    /// `prims` drawn through an SVG luminance mask: each pixel's alpha
    /// is multiplied by the luminance of what `mask` draws there,
    /// `0.2125 R + 0.7154 G + 0.0721 B` times its alpha on the encoded
    /// values, which is what rsvg does with `mask="url(#m)"` (measured:
    /// `#808080` passes 128, pure red 54, pure green 183). Nothing the
    /// mask leaves uncovered shows. Neomil's cold glow is a horizontal
    /// gradient under a vertical one; neokitsch's blue lobe fades out
    /// leftward; no un-masked prim says either.
    ///
    /// Composited only, like [`Prim::Ramp`].
    Masked {
        prims: &'static [Prim],
        mask: &'static [Prim],
    },
}

// Scene constructors. Const fns rather than a builder so an era table
// stays a table -- one line per figure, diffable against the trace.

pub const fn fill_rect(x: f32, y: f32, w: f32, h: f32, ink: Ink) -> Prim {
    Prim::Rect { x, y, w, h, fill: Some(ink), stroke: None, width: 0.0 }
}

pub const fn line_rect(x: f32, y: f32, w: f32, h: f32, ink: Ink, width: f32) -> Prim {
    Prim::Rect { x, y, w, h, fill: None, stroke: Some(ink), width }
}

/// A horizontal rule from `x` to `x2` centred on `y`, drawn as a filled
/// rect so its extent does not depend on the renderer's stroke joins.
pub const fn hline(x: f32, y: f32, x2: f32, ink: Ink, width: f32) -> Prim {
    fill_rect(x, y - width / 2.0, x2 - x, width, ink)
}

/// A vertical rule from `y` to `y2` centred on `x`.
pub const fn vline(x: f32, y: f32, y2: f32, ink: Ink, width: f32) -> Prim {
    fill_rect(x - width / 2.0, y, width, y2 - y, ink)
}

pub const fn fill_path(x: f32, y: f32, segs: &'static [Seg], ink: Ink) -> Prim {
    Prim::Path { x, y, segs, close: true, fill: Some(ink), stroke: None, width: 0.0 }
}

pub const fn line_path(x: f32, y: f32, segs: &'static [Seg], ink: Ink, width: f32) -> Prim {
    Prim::Path { x, y, segs, close: false, fill: None, stroke: Some(ink), width }
}

pub const fn shut_path(x: f32, y: f32, segs: &'static [Seg], ink: Ink, width: f32) -> Prim {
    Prim::Path { x, y, segs, close: true, fill: None, stroke: Some(ink), width }
}

pub const fn txt(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Regular, anchor: Anchor::Start, content }
}

pub const fn txt_mid(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Regular, anchor: Anchor::Middle, content }
}
/// [`txt_mid`] with `letter-spacing`.
pub const fn tracked_mid(x: f32, y: f32, size: f32, tracking: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Tracked { x, y, size, ink, face: Face::Regular, anchor: Anchor::Middle, tracking, content }
}

pub const fn txt_end(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Regular, anchor: Anchor::End, content }
}

pub const fn txt_bold(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Bold, anchor: Anchor::Start, content }
}

pub const fn txt_bold_mid(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Bold, anchor: Anchor::Middle, content }
}
// --- end store -----------------------------------------------------------
