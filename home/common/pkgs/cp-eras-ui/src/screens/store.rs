//! The 4ST store: the toolkit's acceptance test.
//!
//! All four eras' references show this screen, and
//! `docs/<era>/store-trace.svg` measures each of them. This module is
//! the *screen* for those four traces: it walks
//! [`crate::style::Style::store`] -- the era's scene, as data -- and
//! paints it through the shared [`crate::screens::scene`] renderer.
//! Nothing here names an era, and there is no `if era ==`: the four
//! screens differ by their table entry, the way every other knob on
//! [`Style`] works, just with a richer value. `src/style.rs`'s store
//! section records why this screen carries geometry rather than a
//! composition; the short version is that the four traces do not
//! disagree about a *shape*, they disagree about the furniture around
//! it, and no corner radius turns entropism's segmented header strip
//! into neokitsch's eight-strand wire band.
//!
//! The scene is drawn on a single canvas at the trace's own 1600x900
//! coordinates, so a figure in an era table can be diffed against the
//! SVG line it came from and `scripts/fidelity_check.sh --implementation
//! <era> store` compares like with like.
//!
//! Run it with `cp-eras-ui-store --era <name>`; with no flag it
//! follows the desktop theme.

use crate::screens::scene::{Picked, Scene};
use crate::style::{Group, Style};
use crate::widgets::ground;
use iced::widget::stack;
use iced::Element;

pub struct Store {
    pub style: Style,
    /// The chosen category and card, as indices into the era's plates.
    /// Seeded from [`Style::store_selection`], which is what makes the
    /// opening state match each era's own material.
    pub category: usize,
    pub card: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// A plate was clicked: pick it for its group.
    Select { group: Group, index: usize },
}

impl Store {
    pub fn new(style: Style) -> Self {
        let (category, card) = style.store_selection;
        Store {
            style,
            category,
            card,
        }
    }

    pub fn title(&self) -> String {
        format!("4ST STORE — {}", self.style.era.name())
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Select {
                group: Group::Category,
                index,
            } => self.category = index,
            Message::Select {
                group: Group::Card,
                index,
            } => self.card = index,
            // No store scene carries a module plate; one arriving here
            // would be a table error, and ignoring it is the answer
            // that keeps this screen from knowing about the dashboard.
            Message::Select {
                group: Group::Module,
                ..
            } => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            ground(&self.style),
            Scene {
                style: self.style,
                prims: self.style.store,
                picked: Picked {
                    category: self.category,
                    card: self.card,
                    module: 0,
                },
                on_select: |group, index| Message::Select { group, index },
            }
            .view(),
        ]
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::scene::{hit, plates};
    use crate::style::Era;

    /// Every era offers the same two choices -- five categories and
    /// four cards -- however differently it draws them. An era table
    /// that forgot to wrap its shelf in plates would render fine and be
    /// dead to the mouse, which is exactly the failure this catches.
    #[test]
    fn every_era_offers_five_categories_and_four_cards() {
        for era in Era::ALL {
            let mut found = Vec::new();
            plates(era.style().store, 0.0, 0.0, &mut found);
            let cats: Vec<_> = found
                .iter()
                .filter(|(g, ..)| *g == Group::Category)
                .map(|(_, i, _)| *i)
                .collect();
            let cards: Vec<_> = found
                .iter()
                .filter(|(g, ..)| *g == Group::Card)
                .map(|(_, i, _)| *i)
                .collect();
            assert_eq!(cats, vec![0, 1, 2, 3, 4], "{} categories", era.name());
            assert_eq!(cards, vec![0, 1, 2, 3], "{} cards", era.name());
        }
    }

    /// Hit-testing walks the scene the same way painting does, so a
    /// click at a plate's own centre has to come back as that plate.
    #[test]
    fn a_click_at_a_plates_centre_selects_that_plate() {
        for era in Era::ALL {
            let store = era.style().store;
            let mut found = Vec::new();
            plates(store, 0.0, 0.0, &mut found);
            for (group, index, centre) in found {
                assert_eq!(
                    hit(store, 1.0, centre),
                    Some((group, index)),
                    "{} {:?} {}",
                    era.name(),
                    group,
                    index
                );
            }
        }
    }

    /// The opening selection is era data, and the traces disagree about
    /// it: entropism grows its first card, the other three their
    /// second. A screen that hardcoded either would match one trace and
    /// miss three.
    #[test]
    fn the_screen_opens_on_the_selection_its_era_was_traced_with() {
        assert_eq!(Store::new(Era::Entropism.style()).card, 0);
        for era in [Era::Kitsch, Era::Neomil, Era::Neokitsch] {
            assert_eq!(Store::new(era.style()).card, 1, "{}", era.name());
        }
        for era in Era::ALL {
            let store = Store::new(era.style());
            assert!(store.category < 5, "{}", era.name());
        }
    }

    /// Selecting moves only its own group.
    #[test]
    fn selecting_a_card_leaves_the_category_alone() {
        let mut store = Store::new(Era::Kitsch.style());
        let category = store.category;
        store.update(Message::Select {
            group: Group::Card,
            index: 3,
        });
        assert_eq!(store.card, 3);
        assert_eq!(store.category, category);
        store.update(Message::Select {
            group: Group::Category,
            index: 4,
        });
        assert_eq!(store.category, 4);
        assert_eq!(store.card, 3);
    }
}
