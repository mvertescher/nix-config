//! The four eras, as sampled from the reference material.
//!
//! Each module is a table of values, not behaviour. Palette figures are
//! pixel reads off the 1400px Behance modules, recorded alongside the
//! design targets in `docs/<era>/README.md`; the observed rules that
//! justify each geometry choice are recorded there too.

pub mod entropism;
pub mod kitsch;
pub mod neokitsch;
pub mod neomil;

use crate::style::{Era, Style};

pub fn style(era: Era) -> Style {
    match era {
        Era::Entropism => entropism::style(),
        Era::Kitsch => kitsch::style(),
        Era::Neomil => neomil::style(),
        Era::Neokitsch => neokitsch::style(),
    }
}
