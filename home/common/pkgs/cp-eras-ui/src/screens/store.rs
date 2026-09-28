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

use crate::motion;
use crate::screens::nav::{self, Dir, Stroke};
use crate::screens::scene::{plates_selected, Picked, Scene};
use crate::style::{Group, Style};
use crate::widgets::ground;
use crate::Element;
use iced::widget::stack;
use iced::Subscription;
use std::time::{Duration, Instant};

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
    /// The screen's t = 0: the process origin, or the moment the hub
    /// opened it (`motion::onset`, [`Store::enter`]).
    origin: Instant,
    /// The moment the scene is painted at, for its `Prim::Motion`s.
    /// Advanced by [`Message::Tick`] while the boot-in runs, then left
    /// where it is.
    now: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// A plate was clicked: pick it for its group.
    Select { group: Group, index: usize },
    /// A key moved the focus to the nearest plate that way.
    Move(Dir),
    /// The clock, while the boot-in runs.
    Tick(Instant),
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
            origin: motion::origin(),
            now: motion::now(),
        }
    }

    pub fn title(&self) -> String {
        format!("4ST STORE — {}", self.style.era.name())
    }

    /// The screen is coming up: start its clock here, so its boot-in
    /// plays from now (`motion`, the module note). Under a pinned
    /// clock this changes nothing.
    pub fn enter(&mut self) {
        self.origin = motion::onset();
        self.now = motion::now();
    }

    /// Where the scene's clock is, counted from the screen's origin.
    pub(crate) fn at(&self) -> Duration {
        self.now.saturating_duration_since(self.origin)
    }

    /// A redraw every frame until the scene is at rest, and none when
    /// the clock is pinned: as `Dashboard::subscription`.
    pub fn subscription(&self) -> Subscription<Message> {
        if motion::frozen() || self.at() >= motion::REST {
            return Subscription::none();
        }
        iced::time::every(Duration::from_millis(16)).map(Message::Tick)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick(at) => self.now = at,
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
        plates_selected(self.style.store, self.picked(), 0.0, 0.0, &mut found);
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

    fn picked(&self) -> Picked {
        Picked { category: self.category, card: self.card, module: 0 }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let style = self.style.store_style();
        let (prims, backdrop) = self.style.store_layers(self.category, self.card);
        stack![
            ground(&style),
            Scene {
                style,
                prims,
                cursor_group: self.style.store_cursor,
                states: self.style.store_states,
                picked: self.picked(),
                on_select: |group, index| Message::Select { group, index },
                at: self.at(),
            }
            .view_with_backdrop(backdrop),
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
    fn reference_surfaces_follow_selection_without_changing_navigation_or_custom_palettes() {
        use crate::palette::rgb;
        use crate::screens::scene::hit_selected;
        let style = Era::Neomil.style();
        let reference = style.store_reference.expect("source store material");
        assert_eq!(reference.backdrops.len(), 20);
        for category in 0..5 {
            for card in 0..4 {
                let picked = Picked { category, card, module: 0 };
                let (scene, backdrop) = style.store_layers(category, card);
                assert!(std::ptr::eq(scene, reference.scene));
                assert_eq!(reference.backdrops.iter().filter(|b| b.category == category && b.card == card).count(), 1);
                let original_motion = style.store.iter().find_map(|p| match p {
                    crate::style::Prim::Motion { motion, .. } => Some(motion), _ => None,
                }).unwrap();
                assert!(backdrop.iter().any(|p| matches!(p, crate::style::Prim::Motion { motion, .. } if motion == original_motion)));
                let mut original = Vec::new();
                let mut actual = Vec::new();
                plates_selected(style.store, picked, 0.0, 0.0, &mut original);
                plates_selected(scene, picked, 0.0, 0.0, &mut actual);
                assert_eq!(actual, original);
                // Exercise grown-card bottoms and the permanent fourth-card
                // cut, including points just outside each drawing.
                for x in [147.0, 153.0, 360.0, 362.0, 436.0, 450.0, 769.0, 1039.0, 1097.0, 1426.0, 1556.0, 1558.0] {
                    for y in [150.0, 180.0, 300.0, 580.0, 612.0, 650.0, 779.0, 797.0, 799.0] {
                        let point = iced::Point::new(x, y);
                        assert_eq!(hit_selected(scene, picked, 1.0, point), hit_selected(style.store, picked, 1.0, point));
                    }
                }
            }
        }
        assert_eq!(style.store_layers(5, 4), (style.store, style.store));
        let mut custom = style;
        custom.palette.panel = rgb(0x183638);
        assert_eq!(custom.store_layers(0, 1), (style.store, style.store));
        for era in [Era::Entropism, Era::Kitsch, Era::Neokitsch] {
            let style = era.style();
            assert_eq!(style.store_layers(0, 1), (style.store, style.store));
        }
    }

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

    #[test]
    fn selected_lower_details_and_cropped_margin_have_correct_hits_at_all_scales() {
        use crate::screens::scene::hit_selected;
        use iced::Point;
        let mut store = Store::new(Era::Neomil.style());
        let point = |x, y, k| Point::new(x * k, y * k);
        for k in [0.5, 1.0, 1.25, 1.6, 2.4] {
            assert_eq!(hit_selected(store.style.store, store.picked(), k, point(900.0, 700.0, k)), Some((Group::Card, 1)));
            assert_eq!(hit_selected(store.style.store, store.picked(), k, point(1580.0, 350.0, k)), None);
            for card in 0..4 {
                store.update(Message::Select { group: Group::Card, index: card });
                let x = [437.0, 769.0, 1096.0, 1425.0][card] + 66.0;
                assert_eq!(hit_selected(store.style.store, store.picked(), k, point(x, 700.0, k)), Some((Group::Card, card)));
                assert_eq!(hit_selected(store.style.store, store.picked(), k, point(1557.25, 350.0, k)), None);
                assert_eq!(hit_selected(store.style.store, store.picked(), k, point(1580.0, 700.0, k)), None);
                let idle_x = [437.0, 769.0, 1096.0, 1425.0][(card + 1) % 4] + 66.0;
                assert_eq!(hit_selected(store.style.store, store.picked(), k, point(idle_x, 700.0, k)), None);
            }
            store.update(Message::Select { group: Group::Card, index: 1 });
        }
    }

    #[test]
    fn kitsch_selected_lower_body_and_footer_follow_each_card_at_all_scales() {
        use crate::screens::scene::hit_selected;
        use iced::Point;

        let mut store = Store::new(Era::Kitsch.style());
        let inside = [585.0, 905.0, 1224.0, 1485.0];
        for scale in [0.75, 1.0, 1.25, 2.4] {
            let hit = |store: &Store, x: f32, y: f32| {
                hit_selected(store.style.store, store.picked(), scale,
                    Point::new(x * scale, y * scale))
            };
            for selected in 0..4 {
                store.update(Message::Select { group: Group::Card, index: selected });
                for (card, x) in inside.into_iter().enumerate() {
                    assert_eq!(hit(&store, x, 650.0),
                        (card == selected).then_some((Group::Card, card)),
                        "scale {scale}, selected {selected}, card {card} lower body");
                    assert_eq!(hit(&store, x, 705.0),
                        (card == selected).then_some((Group::Card, card)),
                        "scale {scale}, selected {selected}, card {card} footer");
                }
                assert_eq!(hit(&store, 1485.0, 719.0), None,
                    "scale {scale}, selected {selected}: below the grown face");
                assert_eq!(hit(&store, 1549.0, 650.0),
                    (selected == 3).then_some((Group::Card, 3)),
                    "scale {scale}, selected {selected}: last visible pixel of card4");
                assert_eq!(hit(&store, 1550.25, 650.0), None,
                    "scale {scale}, selected {selected}: cropped margin");
            }
        }
    }

    #[test]
    fn every_selected_navigation_centre_remains_visible_and_interactive() {
        use crate::screens::scene::hit_selected;
        for era in Era::ALL {
            let mut store = Store::new(era.style());
            for card in 0..4 {
                store.update(Message::Select { group: Group::Card, index: card });
                let mut found = Vec::new();
                plates_selected(store.style.store, store.picked(), 0.0, 0.0, &mut found);
                assert_eq!(found.len(), 9, "{era:?}, card{card}");
                for (group, index, centre) in found {
                    assert_eq!(hit_selected(store.style.store, store.picked(), 1.0, centre), Some((group, index)), "{era:?}, card{card}, {centre:?}");
                    if era == Era::Neomil && group == Group::Card && index == 3 {
                        assert_eq!(centre.x, 1491.0, "fourth-card centre uses its visible132px width");
                    }
                    if era == Era::Kitsch && group == Group::Card && index == 3 && card == 3 {
                        assert_eq!(centre, iced::Point::new(1496.5, 468.0),
                            "fourth-card centre uses its visible107px width and grown height");
                    }
                }
            }
        }
    }

}
