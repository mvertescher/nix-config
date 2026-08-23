//! Neo-militarism -- "substance over style".
//!
//! Three reds on near-black with a cold blue ambient glow, chamfered
//! surfaces, stencil labelling. Sampled from Behance Part 1, doc #44-52
//! and the shipped game HUD it became (the reference side panel notes
//! the HUD "evolved from and is based on this style").
//!
//! Unlike the other three eras, this one already had an implementation
//! before the sampling pass, in `src/colors.rs`. The figures agree; the
//! values here are the same reds, restated in the era table so all four
//! read alike.

use crate::palette::{rgb, Palette};
use crate::style::{Bar, Chrome, Corner, Era, Ground, Metrics, Nameplate, Selection, Style};

pub const BG: iced::Color = rgb(0x050304);
pub const GLOW: iced::Color = rgb(0x001a33);
pub const RED_DEEP: iced::Color = rgb(0x5e1112);
pub const RED_MID: iced::Color = rgb(0xa32226);
pub const RED_FILL: iced::Color = rgb(0xde2e2e);
pub const RED_HOT: iced::Color = rgb(0xff3b45);
pub const OFF_WHITE: iced::Color = rgb(0xdedede);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: GLOW,
        border: RED_DEEP,
        dim: RED_MID,
        fg: RED_FILL,
        alert: RED_HOT,
        tape: OFF_WHITE,
        // Selected surfaces take the fill red and put off-white on top;
        // the references never invert to white-on-red text.
        select: RED_FILL,
        on_select: rgb(0x1a0405),
        emphasis: None,
        cta: RED_FILL,
        bloom: GLOW,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Neomil,
        palette: palette(),
        corner: Corner::Chamfer { cut: 15.0 },
        selection: Selection::Solid,
        ground: Ground::Flat,
        chrome: Chrome::Tape,
        nameplate: Nameplate::Header,
        bar: Bar::default(),
        metrics: Metrics {
            stroke: 1.5,
            gap: 16.0,
            pad: 16.0,
            ..Metrics::default()
        },
    }
}
