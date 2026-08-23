//! The 4ST store: the toolkit's acceptance test.
//!
//! Every era's design target renders this same screen (`docs/<era>/
//! target-app.svg`), which makes it the honest measure of whether the
//! shared vocabulary is real. There is one implementation here. If a
//! future era cannot wear it without adding an `if era ==` to this file,
//! the abstraction is wrong and that is worth knowing.
//!
//! Run it with `cyberpunk-ui-store --era <name>`; with no flag it
//! follows the desktop theme.

use crate::style::{Chrome, Style};
use crate::widgets::{footer, ground, marker, pill, product_card, text, top_bar, Product};
use iced::widget::{column, container, row, stack, Space};
use iced::{Element, Length, Padding};

const CATEGORIES: [&str; 5] = ["RIFLES", "SMG", "SNIPER", "SHOTGUN", "PISTOL"];
const SELECTED_CATEGORY: usize = 1;
const SELECTED_CARD: usize = 1;
const CARDS: usize = 4;

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

        column![
            text::title(s, "4ST").size(52),
            text::label(s, "S T O R E"),
            Space::new(0.0, s.metrics.gap),
            meta("CUSTOMER", "#NC488402"),
            meta("LOYALTY DISCOUNT", "10%"),
            meta("LAST UPDATE", "10/05/2077"),
            Space::new(0.0, s.metrics.gap * 1.5),
            nav,
            Space::new(0.0, s.metrics.gap * 2.0),
            marker(
                s,
                "A",
                &[
                    "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
                    "SERVING CUSTOMERS SINCE 2006.",
                ],
            ),
            Space::new(0.0, 10.0),
            marker(
                s,
                "B",
                &[
                    "MAPS ARE PROVIDED BY SEOCHO. SATELLITE",
                    "SERVICES SINCE 2006.",
                ],
            ),
        ]
        .spacing(4)
        .into()
    }

    /// The row of product cards, one of them selected and grown.
    fn shelf(&self) -> Element<'_, Message> {
        let s = &self.style;
        let product = Product::magnum();

        let mut shelf = row![].spacing(s.metrics.gap).width(Length::Fill);
        for i in 0..CARDS {
            let selected = i == SELECTED_CARD;
            // The selected card is taller in every reference: it grows
            // the detail block rather than overlaying a popover.
            let card = container(product_card(s, &product, selected))
                .width(Length::FillPortion(1))
                .align_y(iced::alignment::Vertical::Top);
            shelf = shelf.push(card);
        }

        // Neokitsch alone footnotes the shelf rather than the sidebar.
        if s.chrome == Chrome::DeviceFrame {
            column![
                shelf,
                Space::new(0.0, s.metrics.gap),
                container(marker(
                    s,
                    "C",
                    &[
                        "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
                        "SERVING CUSTOMERS SINCE 2006.",
                    ],
                ))
                .padding(Padding::from([0, 0])),
            ]
            .into()
        } else {
            shelf.into()
        }
    }
}
