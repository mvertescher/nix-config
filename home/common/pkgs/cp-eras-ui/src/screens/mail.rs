//! The mailbox, in any era.
//!
//! List on the left, message on the right, security levels and action
//! buttons where the era puts them. Present in all four sets of design
//! targets, which is why it earns a place beside the store.

use crate::style::Style;
use crate::widgets::{badge, footer, ground, marker, row::Mail, surface, text, top_bar};
use crate::widgets::row::{mail_row, rule};
use crate::widgets::surface::Surface;
use iced::widget::{column, container, row, stack, Space};
use iced::{Element, Length, Padding};

const SELECTED: usize = 1;

fn inbox() -> [Mail<'static>; 6] {
    [
        Mail { subject: "You'll regret that", from: "FROM: JACKIE", unread: false },
        Mail { subject: "Urgent information (!)", from: "FROM: MOM", unread: true },
        Mail { subject: "Heist data sent to you", from: "FROM: 805000451", unread: false },
        Mail { subject: "I'm worried man", from: "FROM: RACHEL ROSS", unread: true },
        Mail { subject: "Special offer to you!", from: "FROM: JINX JINX STORE", unread: true },
        Mail { subject: "I'm worried man", from: "FROM: RACHEL ROSS", unread: false },
    ]
}

const BODY: [&str; 3] = [
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do \
     eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
     nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
     reprehenderit in voluptate velit esse cillum dolore eu fugiat.",
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem \
     accusantium doloremque laudantium, totam rem aperiam.",
];

const ACTIONS: [&str; 4] = ["REPLY", "FORWARD", "DELETE", "REPORT SPAM"];
const LEVELS: [&str; 4] = ["T1", "T2", "T3", "T4"];
const LEVEL_SELECTED: usize = 1;

pub struct MailBox {
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl MailBox {
    pub fn new(style: Style) -> Self {
        MailBox { style }
    }

    pub fn title(&self) -> String {
        format!("MAIL BOX — {}", self.style.era.name())
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
                    "PERSONAL LINK SOFTWAREV2",
                    "MAIL BOX",
                    "FLAIR TRS 5MMP",
                ],
            ),
            row![
                container(self.list()).width(Length::FillPortion(4)),
                Space::new(s.metrics.gap * 2.0, 0.0),
                container(self.message()).width(Length::FillPortion(6)),
                Space::new(s.metrics.gap * 2.0, 0.0),
                container(self.levels()).width(Length::Fixed(180.0)),
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

    fn list(&self) -> Element<'_, Message> {
        let s = &self.style;
        let mut list = column![self.heading("A", "MAIL BOX"), Space::new(0.0, s.metrics.gap)]
            .spacing(0);

        for (i, mail) in inbox().iter().enumerate() {
            let selected = i == SELECTED;
            list = list.push(mail_row(s, mail, selected));
            if !selected {
                list = list.push(rule(s));
            }
        }

        list = list.push(Space::new(0.0, s.metrics.gap * 2.0));
        list = list.push(marker(
            s,
            "C",
            &[
                "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
                "SERVING CUSTOMERS SINCE 2006.",
            ],
        ));
        list.into()
    }

    fn message(&self) -> Element<'_, Message> {
        let s = &self.style;
        let mut body = column![
            text::title(s, "URGENT INFORMATION (!)").size(s.metrics.text_title - 3),
            text::caption(s, "FROM: MOM"),
            Space::new(0.0, s.metrics.gap),
        ]
        .spacing(2);

        for para in BODY {
            body = body.push(text::body(s, para));
            body = body.push(Space::new(0.0, 10.0));
        }

        let mut actions = row![].spacing(s.metrics.gap * 0.5);
        for (i, action) in ACTIONS.iter().enumerate() {
            // The last action is destructive and is the only filled one
            // in the references -- the era's alert colour, not its
            // selection colour, which is the distinction `alert` exists
            // for everywhere except kitsch.
            let last = i == ACTIONS.len() - 1;
            let bg = if last {
                Surface::filled(s, s.palette.alert).no_stroke()
            } else {
                Surface::outlined(s)
            };
            let label = if last {
                text::on_select(s, *action).size(s.metrics.text_caption + 3)
            } else {
                text::body(s, *action).size(s.metrics.text_caption + 3)
            };
            actions = actions.push(
                container(surface(bg, Padding::from([5, 8]), label))
                    .width(Length::Fill)
                    .height(Length::Fixed(30.0)),
            );
        }

        column![
            self.heading("B", "MESSAGE"),
            Space::new(0.0, s.metrics.gap),
            container(surface(Surface::outlined(s), s.metrics.pad, body))
                .height(Length::Fill),
            Space::new(0.0, s.metrics.gap),
            actions,
        ]
        .spacing(0)
        .into()
    }

    fn levels(&self) -> Element<'_, Message> {
        let s = &self.style;
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
            self.heading("D", "ENCRYPTION LEVEL"),
            Space::new(0.0, s.metrics.gap),
            grid,
        ]
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
