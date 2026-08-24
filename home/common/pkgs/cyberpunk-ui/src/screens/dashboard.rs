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
//! The hub itself is [`crate::widgets::menu`], and this screen is why
//! that widget exists. The four eras do not merely dress a module
//! chooser differently, they reach for four different *objects* --
//! tiles, an extruded fan, a services table, a card cascade -- and this
//! file cannot pick between them without an era branch. So it names
//! none of them: it hands `menu` six [`MenuItem`]s and a selected
//! index, and [`crate::style::Menu`] on the era table says what a menu
//! is. Until that variant existed the hub drew a hardcoded two-column
//! grid of `Surface`s, which was the right stopgap and the wrong screen
//! for three eras out of four.
//!
//! Neomil's arm is a services table now rather than the cut-diamond hub
//! it inherited, which is the sampled answer for this slot and the
//! reason [`crate::widgets::table`] exists -- see
//! [`crate::style::Menu::Table`].
//!
//! Note the column count moved with it. The grid here was two wide;
//! entropism's sheet draws its tiles three to a row and the era table
//! says so, so wiring the menu up is also what stopped this screen
//! overriding the reference from the outside.
//!
//! Run it with `cyberpunk-ui-dashboard --era <name>`; with no flag it
//! follows the desktop theme.

use crate::style::Style;
use crate::widgets::surface::{layered, surface, Surface};
use crate::widgets::{badge, footer, ground, marker, menu, text, top_bar, MenuItem};
use iced::widget::{canvas, column, container, row, stack, Space};
use iced::{Element, Length, Padding};

/// The modules the hub offers: label, the catalogue code the references
/// print under it, and the one-line blurb the entropism set puts inside
/// each block.
///
/// `static` rather than `const` so `&MODULES` is one `'static` slice
/// the menu borrows, rather than a fresh temporary per call that only
/// lives long enough because of rvalue promotion.
///
/// All three fields are handed over even though only one era draws all
/// three, and that is deliberate: the fan has no room for a blurb and
/// only the table has room for all three, so each arm takes what its
/// object has room for instead of this screen deciding on its behalf.
static MODULES: [MenuItem<'static>; 6] = [
    MenuItem {
        label: "VEHICLES",
        code: "161-9A",
        blurb: "Registered chassis, plates and transit permits.",
    },
    MenuItem {
        label: "LOCATIONS",
        code: "161-9B",
        blurb: "Districts, checkpoints and mapped access routes.",
    },
    MenuItem {
        label: "FACTIONS",
        code: "161-9C",
        blurb: "Standing, known contacts and open contracts.",
    },
    MenuItem {
        label: "WEAPONS",
        code: "161-9D",
        blurb: "Licensed hardware and surplus combat inventory.",
    },
    MenuItem {
        label: "PRODUCTS",
        code: "161-9E",
        blurb: "Catalogue, stock levels and delivery windows.",
    },
    MenuItem {
        label: "CORPORATIONS",
        code: "161-9F",
        blurb: "Charters, subsidiaries and trade agreements.",
    },
];
const SELECTED_MODULE: usize = 3;

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
                container(self.modules())
                    .width(Length::FillPortion(6))
                    .height(Length::Fill),
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

    /// The module hub: six modules, one of them selected, in whatever
    /// the era means by a menu.
    ///
    /// The menu is given the whole remaining column rather than its
    /// natural height, and that is load-bearing for two of the four
    /// arms. `Menu::Fan` is a canvas that fits itself to the box it is
    /// handed; in a `Shrink` column it is handed nothing and draws
    /// nothing. The three layout arms are indifferent to it -- tiles,
    /// the cascade and the table keep their sampled heights and leave
    /// the slack below, which is what the sheets do under their own.
    fn modules(&self) -> Element<'_, Message> {
        let s = &self.style;

        column![
            self.heading("C", "COMPUTER SYSTEMS"),
            Space::new(0.0, s.metrics.gap),
            container(menu(s, &MODULES, SELECTED_MODULE)).height(Length::Fill),
        ]
        .height(Length::Fill)
        .into()
    }

    /// The selected module's description, and the vendors it is
    /// licensed from.
    fn detail(&self) -> Element<'_, Message> {
        let s = &self.style;
        let module = &MODULES[SELECTED_MODULE];

        let mut body = column![
            text::title(s, module.label).size(s.metrics.text_title - 3),
            text::caption(s, module.code),
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
