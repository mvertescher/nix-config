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

/// Status-bar shape. Mirrors the `barHeight` and `hostTape` knobs the
/// nix era builder already takes, so the bar and the generated waybar
/// config cannot disagree about how tall the bar is -- which matters,
/// because the height is also the exclusive zone the compositor
/// reserves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bar {
    pub height: u32,
    /// Show the hostname as a tape-coloured label at the far left.
    pub host_tape: bool,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            height: 26,
            host_tape: true,
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
/// `x=352 w=176`, in `docs/neokitsch/target-components.svg`, and
/// `x=506 w=244` against a card at `x=520 w=230` in its `target-app`.
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
/// about it: `docs/kitsch/target-app.svg` draws every category as
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

/// How an era lays out a *menu* -- the one thing four dressed
/// rectangles could not express.
///
/// Every other knob here dresses a shape the four eras agree on. This
/// one does not: they reach for four different objects when they offer
/// a choice of modules, and no amount of corner radius turns one into
/// another. That is what makes it a variant beside [`Chrome`] and
/// [`Footnotes`] rather than a widget-side era test -- a screen says
/// "put a menu here", and the era says what a menu is.
///
/// The evidence, in the same document order as the rest of the table:
///
/// * **Entropism** -- `target-components.svg` "MENU TILES": a grid of
///   `120x120` squares three to a row, the selected one a solid sage
///   fill, each with a hairline and a tiny caption strip under it
///   ("REPORT ERROR · V2.11 · CERTIFIED"). Square, flat, repeated: the
///   era doing the least it can.
/// * **Kitsch** -- `target-components.svg` "EXTRUDED FAN MENU": slabs
///   fanned about a pivot at `-15`, `+7` and `+27` degrees, each with
///   two stacked outline copies receding up-right by `(6,-8)` at half
///   opacity, labels rotated to the slab. The active slab is yellow,
///   the rest the era's lit teal.
/// * **Neomil** -- the cut-diamond hub, and the one arm that cites no
///   file. Both of that era's sheets were re-read to settle this and
///   neither draws a diamond anywhere. What they *do* draw, twice, is
///   neomil choosing between things at two other scales:
///   `target-app.svg` has a ~60px vertical nav rail of five 16px
///   glyphs with one filled, and `target-components.svg` has a "TAB
///   BAR" -- `SYS | NET | GEAR | LOG`, four chamfered pills in a row,
///   the active one solid red on dark ink and the rest outlined --
///   plus a vertical context menu with one row highlighted.
///
///   Neither is this. A 16px global rail and a four-tab in-panel
///   switcher are not a six-module hub filling a screen's centre
///   column with a code and a glyph apiece, and at the slot
///   `screens::dashboard` puts a menu in, `target-app.svg` puts a
///   *services table*. So the sampled answer for that slot is the data
///   table this crate has not grown yet, and a six-tall rail of tabs
///   would be no more sampled than the diamonds are -- it would just
///   be unsampled in a duller way, at the cost of the only widget here
///   that hit-tests.
///
///   The hub therefore stays, as a stand-in for the table rather than
///   as this era's interaction model. It is inherited from the
///   pre-generalisation `neomil-ui`, it is the one row in this table
///   that would not survive a re-derivation from the references, and
///   it is the row to revisit the day `docs/neomil`'s "table with
///   selection + scrollbar" becomes a widget.
/// * **Neokitsch** -- `target-components.svg` "CARD CASCADE (device
///   software)": tall clipped-corner cards `68x134` at an `88` pitch,
///   staggered vertically (`0, -30, -34, -30`), the active one filled
///   with veneer, each carrying its name at its own foot.
///
/// Deliberately not carrying the fine geometry. [`Chrome`],
/// [`Footnotes`], [`Compliance`] and [`Nameplate`] are all bare choices
/// whose figures live in the widget that draws them, and a menu is the
/// same kind of thing: the numbers above are constants of one drawing,
/// not values a host or a variant would retune. The one exception is
/// `Tiles::columns`, which is a *layout* fact -- a caller sizing the
/// column the menu sits in needs it, and the reference sheet and the
/// dashboard disagree about it (three against two).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    /// Entropism: a grid of square tiles under caption strips.
    Tiles { columns: usize },
    /// Kitsch: extruded slabs fanned about a pivot.
    Fan,
    /// Neomil: hex-packed cut diamonds.
    Diamonds,
    /// Neokitsch: staggered clipped-corner cards.
    Cascade,
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
/// * entropism (`docs/entropism/target-app.svg`) sets it *inside* the
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
    /// What this era means by a menu.
    pub menu: Menu,
    /// Whether the era stamps compliance glyphs -- dotted matrix,
    /// hollow square, hollow triangle -- on its bands and rows.
    ///
    /// A parameter rather than a widget-side era test because it is a
    /// fact about the era and not about any one widget: the same three
    /// marks head kitsch's shelf band and lead its EMPTY SOCKET row,
    /// and no other era's references carry them anywhere.
    pub glyphs: bool,
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

    /// The highlight band behind key figures, as `(fill, ink)`.
    pub fn emphasis(&self) -> (Color, Color) {
        self.palette.emphasis_band()
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

    /// Fill for a recessed well: input fields, sockets.
    pub fn inset(&self) -> Color {
        self.palette.inset()
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
