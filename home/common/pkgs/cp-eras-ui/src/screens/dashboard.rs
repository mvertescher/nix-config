//! The module-hub dashboard, in any era.
//!
//! All four eras' references show the same screen in different dress:
//! a menu of six modules with one selected -- neomil's staggered
//! diamonds, entropism's 3x2 tiles, kitsch's fan blades, neokitsch's
//! cascade cards -- a detail panel describing the selection, and a
//! security-badge row with the second badge filled. That took until
//! 2026-09-01 to establish, because two of the four sources had been
//! described without being opened (`docs/sources.md`), and the screen
//! this file used to hold drew three layout arms to the misreading:
//! a widget-built hub shell, an "ops-charts" screen and a "tile row",
//! scoring 0% against the traces on G2i. Folded 2026-09-03.
//!
//! What is here now is the same shape as `screens::store`: the era
//! table carries the drawing and this file carries nothing but the
//! screen. [`crate::style::Style::dashboard`] is the scene, a
//! `&'static [Prim]` transcribed from `docs/<era>/dashboard-trace.svg`
//! at the trace's own 1600x900 and kept in the `// --- dashboard ---`
//! block of `src/eras/<era>.rs`; [`crate::style::Style::dashboard_selection`]
//! is the module the trace shows selected. The shared
//! [`crate::screens::scene`] renderer paints it, and each menu unit is
//! a [`crate::style::Prim::Plate`] in [`crate::style::Group::Module`]
//! whose `on` drawing is painted when its index is the selection, so a
//! click on any unit re-dresses the screen without this file knowing
//! what a menu unit looks like in any era.
//!
//! The traces draw their own grounds (hazes as `Ramp` and `Lobe`
//! prims), so like the store this screen stacks its canvas over
//! [`crate::widgets::ground()`] and nothing else.
//!
//! Run it with `cp-eras-ui-dashboard --era <name>`; with no flag it
//! follows the desktop theme.

use crate::screens::scene::{Picked, Scene};
use crate::style::Style;
use crate::widgets::ground;
use iced::widget::stack;
use iced::Element;

pub struct Dashboard {
    pub style: Style,
    /// The selected module, an index into the era's six
    /// [`crate::style::Group::Module`] plates. Seeded from
    /// [`Style::dashboard_selection`], which is what makes the opening
    /// state match each era's own material.
    pub selected: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// A module plate was clicked: make it the selection.
    Select { index: usize },
}

impl Dashboard {
    pub fn new(style: Style) -> Self {
        Dashboard {
            style,
            selected: style.dashboard_selection,
        }
    }

    pub fn title(&self) -> String {
        format!("DASHBOARD — {}", self.style.era.name())
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Select { index } => self.selected = index,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            ground(&self.style),
            Scene {
                style: self.style,
                prims: self.style.dashboard,
                picked: Picked {
                    module: self.selected,
                    ..Picked::default()
                },
                // The dashboard has one chooser, so the group is not
                // worth carrying: any plate in this scene is a module.
                on_select: |_group, index| Message::Select { index },
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
    use crate::style::{Era, Group};

    /// Every era offers the same six modules, however differently it
    /// draws them. An era table that forgot to wrap its menu in plates
    /// would render fine and be dead to the mouse, which is exactly the
    /// failure this catches -- and until an era's `// --- dashboard ---`
    /// block is filled in, its empty scene is skipped rather than failed,
    /// so the fold and the four transcriptions can land separately.
    #[test]
    fn every_era_offers_six_modules() {
        for era in Era::ALL {
            let scene = era.style().dashboard;
            if scene.is_empty() {
                continue;
            }
            let mut found = Vec::new();
            plates(scene, 0.0, 0.0, &mut found);
            let modules: Vec<_> = found
                .iter()
                .filter(|(g, ..)| *g == Group::Module)
                .map(|(_, i, _)| *i)
                .collect();
            assert_eq!(modules, vec![0, 1, 2, 3, 4, 5], "{} modules", era.name());
            assert!(
                found.iter().all(|(g, ..)| *g == Group::Module),
                "{} dashboard carries a non-module plate",
                era.name()
            );
        }
    }

    /// Hit-testing walks the scene the same way painting does, so a
    /// click at a plate's own centre has to come back as that plate.
    #[test]
    fn a_click_at_a_plates_centre_selects_that_plate() {
        for era in Era::ALL {
            let scene = era.style().dashboard;
            let mut found = Vec::new();
            plates(scene, 0.0, 0.0, &mut found);
            for (group, index, centre) in found {
                assert_eq!(
                    hit(scene, 1.0, centre),
                    Some((group, index)),
                    "{} {:?} {}",
                    era.name(),
                    group,
                    index
                );
            }
        }
    }

    /// The opening selection is era data -- each trace fills a
    /// different module -- and it has to name one of the six.
    #[test]
    fn the_screen_opens_on_the_selection_its_era_was_traced_with() {
        for era in Era::ALL {
            let dash = Dashboard::new(era.style());
            assert_eq!(dash.selected, era.style().dashboard_selection);
            assert!(dash.selected < 6, "{}", era.name());
        }
    }

    #[test]
    fn selecting_moves_the_selection() {
        let mut dash = Dashboard::new(Era::Kitsch.style());
        dash.update(Message::Select { index: 4 });
        assert_eq!(dash.selected, 4);
    }
}
