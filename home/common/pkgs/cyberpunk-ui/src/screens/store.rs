//! The 4ST store: the toolkit's acceptance test.
//!
//! Three of the four eras' design targets render this same screen
//! (`docs/<era>/target-app.svg`; neomil's is an ops dashboard instead),
//! which makes it the honest measure of whether the shared vocabulary is
//! real. There is one implementation here. If a future era cannot wear
//! it without adding an `if era ==` to this file, the abstraction is
//! wrong and that is worth knowing.
//!
//! Run it with `cyberpunk-ui-store --era <name>`; with no flag it
//! follows the desktop theme.

use crate::style::{Chrome, Footnotes, Style};
use crate::widgets::{
    bracket_panel, footer, ground, marker, page_curl, pill, product_card, text, top_bar, Product,
};
use iced::widget::{column, container, row, stack, Space};
use iced::{Element, Length, Padding};

const CATEGORIES: [&str; 5] = ["RIFLES", "SMG", "SNIPER", "SHOTGUN", "PISTOL"];
const SELECTED_CATEGORY: usize = 1;
const SELECTED_CARD: usize = 1;
const CARDS: usize = 4;

const NOTE_A: [&str; 2] = [
    "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
    "SERVING CUSTOMERS SINCE 2006.",
];
const NOTE_B: [&str; 2] = [
    "MAPS ARE PROVIDED BY SEOCHO. SATELLITE",
    "SERVICES SINCE 2006.",
];

pub struct Store {
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Store {
    pub fn new(style: Style) -> Self {
        Store { style }
    }

    pub fn title(&self) -> String {
        format!("4ST STORE — {}", self.style.era.name())
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        let s = &self.style;

        stack![ground(s), container(self.screen()).padding(40)].into()
    }

    fn screen(&self) -> Element<'_, Message> {
        let s = &self.style;

        let mut page = column![].spacing(s.metrics.gap);

        page = page.push(top_bar(
            s,
            [
                "DIGITAL DISTRIBUTION SOFTWAREV2",
                "STORE ACCESS SCREEN",
                "FLAIR TRS 5MMP",
            ],
        ));

        // Neokitsch runs its footnotes along the strata rail above the
        // content rather than down the nav column.
        if s.footnotes == Footnotes::TopRail {
            page = page.push(
                container(
                    row![
                        marker(s, "A", &NOTE_A),
                        Space::new(Length::Fill, 0.0),
                        marker(s, "C", &NOTE_B),
                    ]
                    .align_y(iced::Alignment::Center),
                )
                .padding(Padding {
                    top: 0.0,
                    right: 200.0,
                    bottom: 0.0,
                    left: 300.0,
                }),
            );
        }

        page = page.push(
            row![
                container(self.sidebar()).width(Length::Fixed(260.0)),
                Space::new(s.metrics.gap * 2.0, 0.0),
                self.shelf(),
            ]
            .height(Length::Fill),
        );

        page = page.push(footer(
            s,
            "INTERFACE LOADED",
            "PROVIDED BY NEXUS NETWORK V10.8",
            "BUILD 6.47.48441.R15",
        ));

        page.into()
    }

    /// Logotype, customer meta, category nav, footnote markers.
    fn sidebar(&self) -> Element<'_, Message> {
        let s = &self.style;

        let mut nav = column![].spacing(s.metrics.gap * 0.5);
        for (i, name) in CATEGORIES.iter().enumerate() {
            nav = nav.push(pill(s, name, i == SELECTED_CATEGORY));
        }

        let meta = |k: &'static str, v: &'static str| {
            row![
                text::label(s, k),
                Space::new(Length::Fill, Length::Shrink),
                text::body(s, v),
            ]
        };

        // Kitsch is the era with no top bar to hang the meta block
        // under, and it encloses the head of it instead.
        let customer: Element<'_, Message> = if s.chrome == Chrome::Caption {
            bracket_panel(
                s,
                meta("customer", "#NC488402").into(),
                column![
                    meta("loyalty discount", "10%"),
                    meta("last update", "10/05/2077"),
                ]
                .into(),
            )
        } else {
            column![
                meta("CUSTOMER", "#NC488402"),
                meta("LOYALTY DISCOUNT", "10%"),
                meta("LAST UPDATE", "10/05/2077"),
            ]
            .into()
        };

        let mut side = column![
            text::title(s, "4ST").size(52),
            text::label(s, "S T O R E"),
            Space::new(0.0, s.metrics.gap),
            customer,
            Space::new(0.0, s.metrics.gap * 1.5),
            nav,
        ]
        .spacing(4);

        // Where the markers go is era-owned: an earlier pass had one
        // rule for all four and matched none of them.
        match s.footnotes {
            Footnotes::UnderNav => {
                side = side.push(Space::new(0.0, s.metrics.gap * 2.0));
                side = side.push(marker(s, "A", &NOTE_A));
                side = side.push(Space::new(0.0, 10.0));
                side = side.push(marker(s, "B", &NOTE_B));
            }
            Footnotes::MidColumn => {
                side = side.push(Space::new(0.0, s.metrics.gap));
                side = side.push(page_curl(s, 76.0));
                side = side.push(Space::new(0.0, s.metrics.gap));
                side = side.push(marker(s, "A", &NOTE_A));
            }
            // The markers are on the rail; the column ends at the nav.
            Footnotes::TopRail => {}
        }

        side.into()
    }

    /// The row of product cards, one of them selected and grown.
    fn shelf(&self) -> Element<'_, Message> {
        let s = &self.style;
        let product = Product::magnum();

        // `align_y(Top)` on the row rather than on each card: the row is
        // what would otherwise stretch its children to the tallest of
        // them, which would put the unselected cards' dead space back
        // the moment the selected one grew its detail block.
        let mut shelf = row![]
            .spacing(s.metrics.gap)
            .width(Length::Fill)
            .align_y(iced::alignment::Vertical::Top);
        for i in 0..CARDS {
            let selected = i == SELECTED_CARD;
            // The selected card is taller in every reference: it grows
            // the detail block rather than overlaying a popover.
            shelf = shelf.push(
                container(product_card(s, &product, selected)).width(Length::FillPortion(1)),
            );
        }

        match s.footnotes {
            Footnotes::UnderNav => shelf.into(),
            // Kitsch's second marker sits under the right of the shelf,
            // opposite the one under the page-curl.
            Footnotes::MidColumn => column![
                shelf,
                Space::new(0.0, s.metrics.gap),
                container(marker(s, "C", &NOTE_B))
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Right),
            ]
            .into(),
            Footnotes::TopRail => column![
                shelf,
                Space::new(0.0, s.metrics.gap),
                marker(s, "B", &NOTE_A),
            ]
            .into(),
        }
    }
}
