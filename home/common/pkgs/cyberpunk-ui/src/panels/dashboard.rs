//! The ops dashboard, in any era.
//!
//! Neomil's design target is the odd one out: where the other three
//! eras' `target-app.svg` is the 4ST store, neomil's is "NEOMIL OPS" --
//! a module hub. The predecessor crate had the same screen in
//! entropism (`EMAILS / MATRIX / STORE / CHAT / PRIVATE / DEVICES`, one
//! selected, its description beside it, security levels down the side),
//! drawn as tiles rather than diamonds. Two eras reached the same
//! screen independently, which is the same evidence that put `store`,
//! `login` and `mailbox` in the shared vocabulary.
//!
//! So it is written once here: six modules, one selected, the selected
//! one's description, and the era's chrome. Nothing in this file asks
//! which era it is -- and that is the standing test, not a comment.
//!
//! What deliberately is *not* here is the diamond menu. That is
//! neomil's interaction model, kitsch's is an extruded fan and
//! entropism's is flat tiles, and a screen cannot choose between them
//! without an era branch. The choice belongs in `style.rs` beside
//! `Chrome` and `Footnotes`; until it is there the hub draws its
//! modules from the shared `Surface` vocabulary, which already carries
//! each era's corner treatment and selection idiom.
//!
//! Run it with `cyberpunk-ui-dashboard --era <name>`; with no flag it
//! follows the desktop theme.

use crate::style::Style;
use crate::widgets::surface::{layered, surface, Surface};
use crate::widgets::{badge, footer, ground, marker, text, top_bar};
use iced::widget::{canvas, column, container, row, stack, Space};
use iced::{Element, Length, Padding};

/// The modules the hub offers: label, the catalogue code the references
/// print under it, and the one-line blurb the entropism set puts inside
/// each block.
const MODULES: [(&str, &str, &str); 6] = [
    (
        "VEHICLES",
        "161-9A",
        "Registered chassis, plates and transit permits.",
    ),
    (
        "LOCATIONS",
        "161-9B",
        "Districts, checkpoints and mapped access routes.",
    ),
    (
        "FACTIONS",
        "161-9C",
        "Standing, known contacts and open contracts.",
    ),
    (
        "WEAPONS",
        "161-9D",
        "Licensed hardware and surplus combat inventory.",
    ),
    (
        "PRODUCTS",
        "161-9E",
        "Catalogue, stock levels and delivery windows.",
    ),
    (
        "CORPORATIONS",
        "161-9F",
        "Charters, subsidiaries and trade agreements.",
    ),
];
const SELECTED_MODULE: usize = 3;

/// Fixed rather than `Length::Fill`, and the difference is visible.
/// Three `Fill` rows divide the column into thirds that are not whole
/// pixels, and a 1px outline whose bottom edge lands on a fractional
/// boundary renders dim on the second row and not at all on the third
/// -- the box loses its floor. Confirmed by rendering it both ways.
const TILE_HEIGHT: f32 = 220.0;

const LEVELS: [&str; 4] = ["T1", "T2", "T3", "T4"];
const LEVEL_SELECTED: usize = 1;

const DETAIL: [&str; 2] = [
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
     eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    "Quis ipsum suspendisse ultrices gravida. Risus commodo viverra \
     maecenas accumsan lacus vel facilisis.",
];

const SPEC: [(&str, &str); 4] = [
    ("RECORDS", "1 842"),
    ("CLEARANCE REQUIRED", "T2"),
    ("LAST SYNC", "10/05/2077 04:12"),
    ("SOURCE", "NEXUS NETWORK V10.8"),
];

const VENDORS: [&str; 2] = ["PETROCHEM", "BETTERLIFE TEC"];

const NOTE: [&str; 2] = [
    "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
    "SERVING CUSTOMERS SINCE 2006.",
];

pub struct Dashboard {
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Dashboard {
    pub fn new(style: Style) -> Self {
        Dashboard { style }
    }

    pub fn title(&self) -> String {
        format!("OPS DASHBOARD — {}", self.style.era.name())
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        let s = &self.style;
        stack![ground(s), container(self.screen()).padding(40)].into()
    }

    fn screen(&self) -> Element<'_, Message> {
        let s = &self.style;

        column![
            top_bar(
                s,
                [
                    "COMPUTER SYSTEMS SOFTWAREV2",
                    "OPS DASHBOARD",
                    "FLAIR TRS 5MMP",
                ],
            ),
            row![
                container(self.sidebar()).width(Length::Fixed(260.0)),
                Space::new(s.metrics.gap * 2.0, 0.0),
                container(self.modules()).width(Length::FillPortion(6)),
                Space::new(s.metrics.gap * 2.0, 0.0),
                container(self.detail()).width(Length::FillPortion(5)),
            ]
            .height(Length::Fill),
            footer(
                s,
                "INTERFACE LOADED",
                "PROVIDED BY NEXUS NETWORK V10.8",
                "BUILD 6.47.48441.R15",
            ),
        ]
        .spacing(s.metrics.gap)
        .into()
    }

    /// Logotype, operator meta, clearance grid, footnote.
    fn sidebar(&self) -> Element<'_, Message> {
        let s = &self.style;

        let meta = |k: &'static str, v: &'static str| {
            row![
                text::label(s, k),
                Space::new(Length::Fill, Length::Shrink),
                text::body(s, v),
            ]
        };

        let mut grid = column![].spacing(s.metrics.gap * 0.5);
        let mut pair = row![].spacing(s.metrics.gap * 0.5);
        for (i, level) in LEVELS.iter().enumerate() {
            pair = pair.push(badge(s, level, i == LEVEL_SELECTED, 56.0));
            if i % 2 == 1 {
                grid = grid.push(pair);
                pair = row![].spacing(s.metrics.gap * 0.5);
            }
        }

        column![
            text::title(s, "next").size(52),
            text::label(s, "T E C H N O L O G Y"),
            Space::new(0.0, s.metrics.gap),
            meta("CUSTOMER", "#NC488402"),
            meta("CLEARANCE", "T2"),
            meta("LAST UPDATE", "10/05/2077"),
            Space::new(0.0, s.metrics.gap * 1.5),
            self.heading("A", "SECURITY LEVEL"),
            Space::new(0.0, s.metrics.gap),
            grid,
            Space::new(0.0, s.metrics.gap * 2.0),
            marker(s, "B", &NOTE),
        ]
        .spacing(4)
        .into()
    }

    /// The module hub: six tiles, one of them selected.
    fn modules(&self) -> Element<'_, Message> {
        let s = &self.style;

        let mut grid = column![].spacing(s.metrics.gap * 0.5);
        let mut pair = row![].spacing(s.metrics.gap * 0.5);
        for (i, (name, code, blurb)) in MODULES.iter().enumerate() {
            pair = pair.push(self.tile(name, code, blurb, i == SELECTED_MODULE));
            if i % 2 == 1 {
                grid = grid.push(pair);
                pair = row![].spacing(s.metrics.gap * 0.5);
            }
        }

        column![
            self.heading("C", "COMPUTER SYSTEMS"),
            Space::new(0.0, s.metrics.gap),
            grid,
        ]
        .into()
    }

    /// One module. `Surface::selected` is what carries the era here:
    /// solid fill in three of them, veneer in neokitsch, and the corner
    /// treatment comes along with it.
    fn tile<'a>(
        &'a self,
        name: &'a str,
        code: &'a str,
        blurb: &'a str,
        selected: bool,
    ) -> Element<'a, Message> {
        let s = &self.style;

        let bg = if selected {
            Surface::selected(s)
        } else {
            Surface::outlined(s)
        };

        let (label, sub, note) = if selected {
            (
                text::on_select(s, name),
                text::on_select(s, code).size(s.metrics.text_caption),
                text::on_select(s, blurb).size(s.metrics.text_caption + 2),
            )
        } else {
            (
                text::body(s, name),
                text::caption(s, code),
                text::caption(s, blurb).size(s.metrics.text_caption + 2),
            )
        };

        container(surface(
            bg,
            Padding::from([10, 12]),
            column![label, sub, Space::new(0.0, 8.0), note].spacing(1),
        ))
        .width(Length::Fill)
        .height(Length::Fixed(TILE_HEIGHT))
        .into()
    }

    /// The selected module's description, and the vendors it is
    /// licensed from.
    fn detail(&self) -> Element<'_, Message> {
        let s = &self.style;
        let (name, code, _) = MODULES[SELECTED_MODULE];

        let mut body = column![
            text::title(s, name).size(s.metrics.text_title - 3),
            text::caption(s, code),
            Space::new(0.0, s.metrics.gap),
        ]
        .spacing(2);

        for para in DETAIL {
            body = body.push(text::body(s, para));
            body = body.push(Space::new(0.0, 10.0));
        }

        // Key-value rows under the prose, as every era's detail panel
        // has: the panel is the tall element on this screen and prose
        // alone leaves it reading as an empty box.
        body = body.push(Space::new(0.0, s.metrics.gap));
        for (k, v) in SPEC {
            body = body.push(
                row![
                    text::label(s, k),
                    Space::new(Length::Fill, Length::Shrink),
                    text::body(s, v),
                ]
                .width(Length::Fill),
            );
            body = body.push(Space::new(0.0, 6.0));
        }

        // `layered` rather than `backdrop`: the latter runs its content
        // at `Length::Fill`, which is right for a panel and wrong for a
        // chip -- two vendor tags would take half the column each.
        let mut vendors = row![].spacing(s.metrics.gap * 0.5);
        for vendor in VENDORS {
            vendors = vendors.push(layered(
                canvas(Surface::outlined(s))
                    .width(Length::Fill)
                    .height(Length::Fill),
                container(text::caption(s, vendor)).padding(Padding::from([4, 10])),
            ));
        }

        column![
            self.heading("D", "DESCRIPTION"),
            Space::new(0.0, s.metrics.gap),
            container(surface(Surface::outlined(s), s.metrics.pad, body)).height(Length::Fill),
            Space::new(0.0, s.metrics.gap),
            vendors,
        ]
        .spacing(0)
        .into()
    }

    /// A boxed letter and a caption: the section header every era uses.
    fn heading<'a>(&'a self, letter: &'a str, label: &'a str) -> Element<'a, Message> {
        row![marker(&self.style, letter, &[]), text::body(&self.style, label)]
            .spacing(8)
            .align_y(iced::Alignment::Center)
            .into()
    }
}
