pub mod banner;
pub mod bracket;
pub mod card;
pub mod chrome;
pub mod glyph;
pub mod ground;
pub mod marker;
pub mod menu;
pub mod input;
pub mod ornament;
pub mod pill;
pub mod row;
pub mod silhouette;
pub mod surface;
pub mod text;

// The neo-militarism widget set, from before the toolkit was
// generalised. These are era-specific by nature -- an interaction model
// rather than a dressed rectangle -- and stay as their own modules
// rather than being forced into the shared vocabulary.
//
// One caveat, recorded in `menu.rs` and `style.rs` too: the diamond hub
// is a *stand-in*, not sampled. No neomil sheet draws a diamond; the
// sampled answer for that slot is a data table this crate has not grown.
//
// `diamond_menu` is no longer unreachable: it is the `Menu::Diamonds`
// arm of `menu`, which is how an era-specific interaction model gets a
// screen without a screen ever naming the era.
//
// `message_card` was the opposite case and is gone. It was a dressed
// rectangle, not an interaction model: a mail row with a hardcoded 8px
// double chamfer, unreachable since the entropism-ui retirement. Every
// part of it is already in the shared vocabulary -- `row::mail_row` for
// the posed row, `panels::mail`'s `message_row` for the clickable one,
// and `Surface` for the shape, which draws all four eras' corners and
// carries `Corners::OPPOSED` for the diagonal cut it was named after.
// Left in place it was a trap: the next caller would have got neomil's
// chamfer in all four eras.
pub mod chip;
pub mod diamond_menu;
pub mod floppy_icon;
pub mod floppy_vector;
pub mod level_badge;
pub mod text_box;
pub mod vertical_text;

pub use banner::banner;
pub use bracket::bracket_panel;
pub use card::{notice as card_notice, product_card, Product};
pub use chrome::{footer, top_bar};
pub use glyph::{glyph, Glyph};
pub use ground::ground;
pub use marker::marker;
pub use menu::{menu, MenuItem};
pub use ornament::{column_rule, page_curl};
pub use pill::{badge, pill};
pub use row::{mail_row, Mail};
pub use silhouette::silhouette;
pub use surface::{layered, surface, Corners, Fill, Surface};

pub use chip::{chip_type_1, info_panel};
pub use diamond_menu::{diamond_menu, DiamondMenuItem};
pub use floppy_icon::{floppy_icon, FloppyIcon};
pub use level_badge::{level_badge, LevelBadgeStyle};
pub use text_box::text_box;
pub use vertical_text::VerticalText;
