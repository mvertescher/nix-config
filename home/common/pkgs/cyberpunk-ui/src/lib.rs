//! A UI toolkit for the four Cyberpunk 2077 interface eras.
//!
//! The four eras -- entropism, kitsch, neo-militarism, neokitsch -- were
//! sampled from the published reference material rather than described
//! from memory; see `docs/<era>/README.md` for provenance, palettes and
//! observed rules, and `docs/<era>/target-app.svg` for the design target
//! each is measured against.
//!
//! The shape of the crate follows from one observation: all four
//! references dress the *same screens*. So an era is data --
//! [`style::Style`], a table of palette, corner treatment, selection
//! idiom, ground and chrome -- and screens are written once against it.
//! [`screens::Store`] is the acceptance test for that claim.
//!
//! Apps should start from [`style::Style::from_desktop`], which follows
//! whatever era the nix theme layer has published, so `switch` re-dresses
//! them without a rebuild.

pub mod eras;
pub mod palette;
pub mod screens;
pub mod style;
pub mod theme;
pub mod widgets;

// Neo-militarism-era modules from before the generalisation. Still in
// use by the dashboard and mail examples; they will move under a
// per-era namespace as those screens are rewritten against `screens`.
pub mod background;
pub mod colors;
pub mod fonts;
pub mod panels;
pub mod top_bar;

pub use style::{Era, Style};
