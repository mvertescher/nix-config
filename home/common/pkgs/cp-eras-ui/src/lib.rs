//! A UI toolkit for the four Cyberpunk 2077 interface eras.
//!
//! The four eras -- entropism, kitsch, neo-militarism, neokitsch -- were
//! sampled from the published reference material rather than described
//! from memory; see `docs/<era>/README.md` for provenance, palettes and
//! observed rules, and `docs/<era>/<screen>-trace.svg` (`bar.svg` for the
//! bar) for the design each screen is measured against.
//!
//! The shape of the crate follows from one observation: all four
//! references dress the *same screens*. So an era is data --
//! [`style::Style`], a table of palette, corner treatment, selection
//! idiom, ground and chrome -- and screens are written once against it.
//! [`screens::Store`] is the acceptance test for that claim.
//!
//! Apps boot through [`shell`]: [`shell::style`] follows whatever era the
//! nix theme layer has published (or a `--era` flag), so `switch`
//! re-dresses them without a rebuild, and [`shell::application`] hands
//! iced the faces, the theme and the trace frame.

pub mod catalog;
pub mod eras;
pub mod palette;
pub mod screens;
pub mod shell;
pub mod style;
pub mod theme;
pub mod widgets;

// The interactive half of the mail screen -- same era-agnostic
// contract, but wired for selection and focus rather than posed for a
// golden. `background` and `top_bar` used to live beside it and are
// gone: `widgets::ground` and `widgets::chrome::top_bar` do the same
// jobs for four eras rather than one.
pub mod panels;

pub mod bar;
pub mod fonts;

pub use style::{Era, Style};

/// An element dressed by an era: iced's, with [`Style`] as the theme.
/// See [`catalog`] for what that buys.
pub type Element<'a, Message> = iced::Element<'a, Message, Style, iced::Renderer>;
