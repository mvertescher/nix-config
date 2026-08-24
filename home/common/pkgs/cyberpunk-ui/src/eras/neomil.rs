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

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, Chrome, Compliance, Corner, Era, Footnotes, Ground, Metrics, Nameplate,
    Menu, Selection, Style, Ticket,
};

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
        // No band anywhere, so nothing to restate for the selected
        // state either; `banner()` degrades to a tape label and
        // `banner_on_select()` swaps it.
        banner_selected: None,
        // A minimalist era declares no ornament: the vocabulary is
        // additive and nothing here wants it.
        ornaments: Ornaments::default(),
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
        banner: Banner::default(),
        // A and B under the nav, and a dead lower third the reference
        // is content with.
        footnotes: Footnotes::UnderNav,
        // No store target for this era, and the notice is a
        // maximalist-adjacent flourish; it stays unclaimed.
        compliance: Compliance::None,
        // Neomil chamfers its containers; it does not cut a wedge into
        // a nav pill.
        ticket: Ticket::default(),
        // The services table. This era's own target *is* an ops
        // screen, and where the dashboard puts a menu it puts
        // `UNIT | MEM | UPTIME | STATE` with one row picked out. The
        // cut-diamond hub that used to sit here was the one entry in
        // this table with no `docs/` citation; see
        // [`crate::style::Menu::Table`].
        menu: Menu::Table,
        glyphs: false,
        metrics: Metrics {
            stroke: 1.5,
            gap: 16.0,
            pad: 16.0,
            ..Metrics::default()
        },
    }
}
