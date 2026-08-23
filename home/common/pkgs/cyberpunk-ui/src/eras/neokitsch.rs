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

use crate::palette::{rgb, Palette};
use crate::style::{Chrome, Corner, Era, Ground, Metrics, Nameplate, Selection, Style};

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
        emphasis: None,
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
        metrics: Metrics {
            stroke: 1.6,
            gap: 18.0,
            pad: 18.0,
            // The footer nameplate wants a band the other eras spend
            // on their header.
            card: 330.0,
            card_selected: 470.0,
            ..Metrics::default()
        },
    }
}
