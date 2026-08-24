//! Neokitsch -- "substance and style".
//!
//! Gold line-work on true black under a violet haze, with the device
//! frame itself part of the UI. Sampled from Behance Part 1, doc #54-62.
//!
//! Its defining rule is that selection is a *material*, not a colour:
//! the chosen tab, pill, card or mail row fills with wood veneer. That
//! is the one place the four eras genuinely stress the abstraction --
//! see [`crate::style::Selection::Veneer`] and `widgets::surface`, which
//! synthesises the grain rather than shipping a raster asset.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, Chrome, Compliance, Corner, Era, Footnotes, Ground, Metrics, Nameplate,
    Menu, Selection, Style, Ticket,
};

pub const BG: iced::Color = rgb(0x0a0a0a);
pub const BLOOM: iced::Color = rgb(0x34344c);
pub const FRAME: iced::Color = rgb(0x916424);
pub const FRAME_INNER: iced::Color = rgb(0x5e3414);
pub const GOLD_TEXT: iced::Color = rgb(0xe7c686);
pub const CHAMPAGNE: iced::Color = rgb(0xd3b279);
pub const VENEER: iced::Color = rgb(0xe3af5f);
pub const VENEER_LIGHT: iced::Color = rgb(0xf4c474);
pub const VENEER_DARK: iced::Color = rgb(0xd8a558);
pub const GRAIN: iced::Color = rgb(0xb98a44);
pub const AMBER: iced::Color = rgb(0xfcc474);
pub const FIELD: iced::Color = rgb(0x2c1c14);
pub const ON_VENEER: iced::Color = rgb(0x3a2410);
pub const DIM: iced::Color = rgb(0x8a7048);
/// Top stop of the device frame's outer stroke -- the lit side of the
/// bevel, against `FRAME_INNER` as its shaded one.
pub const FRAME_LIT: iced::Color = rgb(0xc69a55);
/// The fine lines of a strata divider, bunching into a wedge.
pub const STRATA: iced::Color = rgb(0x634427);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: BLOOM,
        border: FRAME,
        dim: DIM,
        fg: GOLD_TEXT,
        // The only strong call-to-action colour in the reference; used
        // for ENTER / LOGIN bars and nothing else.
        alert: AMBER,
        tape: VENEER,
        select: VENEER,
        on_select: ON_VENEER,
        // No highlight band anywhere in the references; that is a
        // kitsch device and this era does hierarchy by brightness.
        emphasis: None,
        // You cannot shade a *material*: a slightly darker champagne
        // band on a grained plank reads as a knot, so the era inverts
        // instead. `target-components.svg` fills the selected card's
        // footer nameplate `#3a2410` and prints the name on it in
        // `#e7c686` -- the era's `fg`, a stop brighter than the
        // champagne the unselected band is filled with.
        banner_selected: Some((ON_VENEER, GOLD_TEXT)),
        ornaments: Ornaments {
            // The card's footer nameplate, and the BASKET panel at the
            // top right of the store. Champagne, not veneer: selection
            // is a material here and the nameplate is not selected.
            banner: Some((CHAMPAGNE, ON_VENEER)),
            // The device frame is a double stroke, lit outside and
            // shaded in.
            relief: Some((FRAME_LIT, FRAME_INNER)),
            ornament: Some(STRATA),
            // The login field, and the socket wells on a card.
            inset: Some(FIELD),
        },
        cta: AMBER,
        bloom: BLOOM,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Neokitsch,
        palette: palette(),
        corner: Corner::ClipTopRight { cut: 30.0 },
        selection: Selection::Veneer,
        // Top-centre, softer and wider than the kitsch bloom.
        ground: Ground::Bloom {
            x: 0.5,
            y: 0.0,
            radius: 0.75,
        },
        chrome: Chrome::DeviceFrame,
        nameplate: Nameplate::Footer,
        bar: Bar::default(),
        // The footer nameplate hangs past the card by the same 12 as
        // kitsch, but does not step: `rect x=340 w=188` against a card
        // at `x=352 w=176` in target-components.svg.
        banner: Banner {
            overhang: 12.0,
            notch: 0.0,
        },
        // A and C along the top strata rail, B under the cards.
        footnotes: Footnotes::TopRail,
        // The footer nameplate is the card's last edge; the target
        // prints no notice under it.
        compliance: Compliance::None,
        // The era has a step-notch shape but spends it on the mailbox
        // footer; its nav pills are plain `rx="4"` rects.
        ticket: Ticket::default(),
        // "CARD CASCADE (device software)": tall clipped-corner cards,
        // staggered, the active one filled with veneer.
        menu: Menu::Cascade,
        glyphs: false,
        metrics: Metrics {
            stroke: 1.6,
            gap: 18.0,
            pad: 18.0,
            ..Metrics::default()
        },
    }
}
