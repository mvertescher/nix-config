//! Entropism -- "necessity over style".
//!
//! One hue. Sage green on a warm dark olive-brown ground, square
//! everything, 1px strokes, no glow and no gradients. Selection is a
//! solid sage fill. Sampled from Behance Part 1, doc #24-32.
//!
//! The predecessor crate (`entropism-ui`) carried twelve colours --
//! including cybr's red, cyan, mint, violet, orange and gold -- and a
//! radial glow module. None of that is in the reference; see
//! `docs/entropism/README.md`. This is the one-hue system.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, Chrome, Compliance, Corner, Era, Footnotes, Ground, Metrics, Nameplate,
    Selection, Style,
};

pub const BG: iced::Color = rgb(0x110c07);
pub const SAGE_SOLID: iced::Color = rgb(0x9cb795);
pub const SAGE_TEXT: iced::Color = rgb(0x94bb94);
pub const MID: iced::Color = rgb(0x728f76);
pub const OUTLINE: iced::Color = rgb(0x5d7752);
pub const DIM: iced::Color = rgb(0x3d4d38);
pub const ON_SOLID: iced::Color = rgb(0x1f2a1c);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        // No lifted panel in the reference: surfaces are outlines on the
        // page ground, so panel and bg are the same colour on purpose.
        panel: BG,
        border: OUTLINE,
        dim: DIM,
        fg: SAGE_TEXT,
        // The one place the reference departs from monochrome is the
        // "(!)" urgency marker, and even that is the same sage. Alert
        // reads as mid rather than inventing a second hue.
        alert: MID,
        tape: MID,
        select: SAGE_SOLID,
        on_select: ON_SOLID,
        emphasis: None,
        // A minimalist era declares no ornament: the vocabulary is
        // additive and nothing here wants it.
        ornaments: Ornaments::default(),
        cta: SAGE_SOLID,
        bloom: BG,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Entropism,
        palette: palette(),
        corner: Corner::Square,
        selection: Selection::Solid,
        ground: Ground::Flat,
        chrome: Chrome::Segmented,
        nameplate: Nameplate::Header,
        bar: Bar::default(),
        banner: Banner::default(),
        // A and B under the nav, and a dead lower third the reference
        // is content with.
        footnotes: Footnotes::UnderNav,
        // The reference sets it inside the outline, under the sockets
        // of every unselected card.
        compliance: Compliance::Inside,
        glyphs: false,
        metrics: Metrics {
            stroke: 1.0,
            gap: 14.0,
            pad: 14.0,
            ..Metrics::default()
        },
    }
}
