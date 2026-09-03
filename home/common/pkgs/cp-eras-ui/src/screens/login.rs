//! The access screen, in any era.
//!
//! The simplest of the shared screens and so the sharpest test of the
//! vocabulary: a label, a field, one button, and the era's chrome. If
//! four eras cannot dress *this* from the same code, nothing else was
//! going to work either.
//!
//! Entropism swaps its thin build-string footer for one solid sage
//! band on this screen. That is `Chrome::Segmented`'s business, not
//! this file's -- see `widgets::chrome`.

use crate::style::Style;
use crate::widgets::{footer, ground, input, marker, text, top_bar};
use iced::widget::{column, container, row, stack, Space};
use iced::{Element, Length};

pub struct Login {
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Login {
    pub fn new(style: Style) -> Self {
        Login { style }
    }

    pub fn title(&self) -> String {
        format!("ACCESS — {}", self.style.era.name())
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
                    "RIPPERDOC SURGICAL SOFTWAREV2",
                    "STORE ACCESS SCREEN",
                    "FLAIR TRS 5MMP",
                ],
            ),
            // The references centre this in a great deal of empty space;
            // the emptiness is the design, not a gap to be filled.
            container(input::labelled_field(s, "USERNAME:", "**********", "NEXT"))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            row![
                marker(
                    s,
                    "A",
                    &[
                        "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
                        "SERVING CUSTOMERS SINCE 2006.",
                    ],
                ),
                Space::new().width(Length::Fill).height(Length::Shrink),
                text::caption(s, "GUEST ACCESS IS LOGGED"),
            ],
            Space::new().height(s.metrics.gap),
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
}
