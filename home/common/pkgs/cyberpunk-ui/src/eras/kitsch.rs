//! Kitsch -- "style over substance".
//!
//! Teal line-work and yellow selection over a rose bloom on warm black.
//! Everything rounded, no chamfers anywhere. Sampled from Behance Part
//! 1, doc #34-42.
//!
//! Note the inversion the era forces on the role vocabulary: yellow is
//! *selection*, not alarm. Failure states are essentially absent from
//! the reference, so `alert` and `select` are the same colour here --
//! the only era where that is true.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, Chrome, Compliance, Corner, Era, Footnotes, Ground, Metrics, Nameplate,
    Menu, Selection, Style, Ticket,
};

pub const BG: iced::Color = rgb(0x0b0b07);
pub const BLOOM: iced::Color = rgb(0xa63355);
pub const TEAL: iced::Color = rgb(0x7ddec8);
pub const TEAL_SOLID: iced::Color = rgb(0x1cb39b);
pub const MINT: iced::Color = rgb(0x87f4d9);
pub const YELLOW: iced::Color = rgb(0xfcc428);
pub const ON_YELLOW: iced::Color = rgb(0x37220f);
pub const BEZEL: iced::Color = rgb(0xf08c1e);
pub const TEAL_DIM: iced::Color = rgb(0x4d9484);
/// Ink for figures sitting on the mint stat band.
pub const ON_MINT: iced::Color = rgb(0x0b3b31);
/// Lit face of an extruded fan-menu slab, and the darker teal its
/// stacked outlines recede in. Sampled off the braindance screens; see
/// `docs/kitsch/target-components.svg`, "EXTRUDED FAN MENU".
pub const SLAB: iced::Color = rgb(0x2bc4ac);
pub const SLAB_SHADE: iced::Color = rgb(0x177a6b);
/// The yellow one stop down: the shelf band on the *selected* card
/// (`docs/kitsch/target-app.svg`, `M830 308 ... fill="#f0a80a"` against
/// `#fcc428` on the other three) and the folded corner of the callout
/// panel. Not reachable by mixing `YELLOW` towards `ON_YELLOW` -- it is
/// darker *and* more saturated, and its blue channel sits below both
/// endpoints.
pub const YELLOW_SHADE: iced::Color = rgb(0xf0a80a);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: BLOOM,
        border: TEAL,
        dim: TEAL_DIM,
        fg: TEAL,
        alert: YELLOW,
        tape: BEZEL,
        select: YELLOW,
        on_select: ON_YELLOW,
        emphasis: Some((MINT, ON_MINT)),
        // On the selected card the band darkens and keeps its ink: the
        // era shades the band rather than inverting it, because the
        // card underneath is already yellow.
        banner_selected: Some((YELLOW_SHADE, ON_YELLOW)),
        ornaments: Ornaments {
            // The shelf band on every product card: yellow, poking past
            // the card's left edge, its glyphs and brand tag in the
            // dark ink. Same fill as `select` here and a different one
            // in neokitsch, which is why it is not an alias for it.
            banner: Some((YELLOW, ON_YELLOW)),
            relief: Some((SLAB, SLAB_SHADE)),
            // The page-curl at the foot of the nav container -- one per
            // screen -- plus the chip squares and PROTECTED bars.
            ornament: Some(TEAL_SOLID),
            // No wells in the era: kitsch cards are unfilled outlines.
            inset: None,
        },
        cta: YELLOW,
        bloom: BLOOM,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Kitsch,
        palette: palette(),
        corner: Corner::Round { radius: 16.0 },
        selection: Selection::Solid,
        // Out of the top-right, heavily vignetted.
        ground: Ground::Bloom {
            x: 0.82,
            y: 0.0,
            radius: 0.75,
        },
        chrome: Chrome::Caption,
        nameplate: Nameplate::Header,
        bar: Bar::default(),
        // The shelf band hangs 12px past the card and steps its
        // trailing corner down 8; measured off target-app.svg.
        banner: Banner {
            overhang: 12.0,
            notch: 8.0,
        },
        // A halfway down the column under the page-curl, C under the
        // right of the shelf.
        footnotes: Footnotes::MidColumn,
        // Below the card rather than inside it, and on the selected
        // one too.
        compliance: Compliance::Below,
        // The nav pill's top-right juts out 18 and drops 15, on a body
        // that is otherwise the era's `radius: 16` pill. Sampled off
        // `M172 340 h158 l18 15 ...` in target-app.svg.
        ticket: Ticket {
            reach: 18.0,
            drop: 15.0,
        },
        // The extruded fan: slabs radiating from a pivot, each with
        // stacked outline copies receding up-right.
        menu: Menu::Fan,
        // The dotted matrix, hollow square and hollow triangle that
        // head every shelf band and lead every socket row.
        glyphs: true,
        metrics: Metrics {
            stroke: 1.5,
            gap: 20.0,
            pad: 18.0,
            ..Metrics::default()
        },
    }
}
