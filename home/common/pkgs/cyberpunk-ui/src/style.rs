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
