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

use crate::screens::nav::{self, Dir, Stroke};
use crate::screens::scene::{plates, Picked, Scene};
use crate::style::{Group, Style};
use crate::widgets::ground;
use crate::Element;
use iced::widget::stack;

pub struct Store {
    pub style: Style,
    /// The chosen category and card, as indices into the era's plates.
    /// Seeded from [`Style::store_selection`], which is what makes the
    /// opening state match each era's own material.
    pub category: usize,
    pub card: usize,
    /// Where the keyboard is: the plate a move sets out from. A click
    /// puts it on the clicked plate, and a move lands it on the nearest
    /// plate that way in either group and selects that plate for its
    /// group, so walking the shelf and choosing from it are one motion.
    /// Opens on the card, the choice the trace grows.
    focus: (Group, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// A plate was clicked: pick it for its group.
    Select { group: Group, index: usize },
    /// A key moved the focus to the nearest plate that way.
    Move(Dir),
}

impl crate::shell::Wears for Store {
    fn wears(&self) -> Style {
        self.style
    }
}

impl Store {
    pub fn new(style: Style) -> Self {
        let (category, card) = style.store_selection;
        Store {
            style,
            category,
            card,
            focus: (Group::Card, card),
        }
    }

    pub fn title(&self) -> String {
        format!("4ST STORE — {}", self.style.era.name())
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Move(dir) => {
                if let Some(landing) = self.neighbour(dir) {
                    self.update(Message::Select { group: landing.0, index: landing.1 });
                }
            }
            Message::Select {
                group: Group::Category,
                index,
            } => {
                self.category = index;
                self.focus = (Group::Category, index);
            }
            Message::Select {
                group: Group::Card,
                index,
            } => {
                self.card = index;
                self.focus = (Group::Card, index);
            }
            // No store scene carries a module plate; one arriving here
            // would be a table error, and ignoring it is the answer
            // that keeps this screen from knowing about the dashboard.
            Message::Select {
                group: Group::Module,
                ..
            } => {}
        }
    }

    /// The plate nearest the focus in `dir`, in either group, from the
    /// plates' centres (`nav::step`); `None` at the shelf's edge.
    fn neighbour(&self, dir: Dir) -> Option<(Group, usize)> {
        let mut found = Vec::new();
        plates(self.style.store, 0.0, 0.0, &mut found);
        let from = found.iter().find(|&&(g, i, _)| (g, i) == self.focus)?.2;
        nav::step(found.iter().map(|&(g, i, c)| ((g, i), c)), from, dir)
    }

    /// The keyboard's part in this screen: moves. Enter and Esc are the
    /// hub's, so on its own the store drops them.
    pub fn stroke(stroke: Stroke) -> Option<Message> {
        match stroke {
            Stroke::Move(dir) => Some(Message::Move(dir)),
            Stroke::Open | Stroke::Back => None,
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
                // The store has no clock yet: nothing in any era's store
                // table moves. Frame 0 rather than rest so that the first
                // `Prim::Motion` added to one is seen stuck at its `from`
                // -- and by the goldens, at rest -- until the screen
                // ticks the way the dashboard does.
                at: std::time::Duration::ZERO,
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

    /// The keyboard reaches both groups in every era: `h` from the shelf
    /// lands on the nav, `j` walks the nav down, `l` from the nav lands
    /// back on the shelf. The eras hang the two differently, so this is
    /// the one thing `nav::step`'s scoring is held to on real tables.
    #[test]
    fn the_keys_walk_between_nav_and_shelf_in_every_era() {
        for era in Era::ALL {
            let mut store = Store::new(era.style());
            assert_eq!(store.focus.0, Group::Card, "{}", era.name());
            for _ in 0..4 {
                store.update(Message::Move(Dir::Left));
                if store.focus.0 == Group::Category {
                    break;
                }
            }
            assert_eq!(store.focus.0, Group::Category, "{}: h never reaches the nav", era.name());
            let top = store.category;
            store.update(Message::Move(Dir::Down));
            assert_ne!(store.category, top, "{}: j does not walk the nav", era.name());
            for _ in 0..4 {
                store.update(Message::Move(Dir::Right));
                if store.focus.0 == Group::Card {
                    break;
                }
            }
            assert_eq!(store.focus.0, Group::Card, "{}: l never reaches the shelf", era.name());
        }
    }
}
