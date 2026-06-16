use crate::glow::{get_radial_offsets, glowing_border_container, glowing_text};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Task};

#[derive(Debug, Clone)]
pub struct Email {
    pub subject: String,
    pub from: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    EmailSelected(usize),
    EncryptionLevelSelected(usize),
    ActionButtonPressed(usize), // 0: Reply, 1: Forward, 2: Delete, 3: Report Spam
    GoBack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenEvent {
    GoToDashboard,
}

pub struct MailScreen {
    selected_email: usize,
    encryption_level: usize,
    focused_item: usize,
    emails: Vec<Email>,
    pub scroll_list: crate::scroll_list::ScrollList,
}

impl MailScreen {
    pub fn new(emails: Vec<Email>) -> Self {
        Self {
            selected_email: 0,
            encryption_level: 1, // Default to T2 (index 1)
            focused_item: 0,
            emails,
            scroll_list: crate::scroll_list::ScrollList::new(75.0, 525.0),
        }
    }

    pub fn update(&mut self, message: Message) -> (Option<ScreenEvent>, Task<Message>) {
        match message {
            Message::EmailSelected(idx) => {
                self.selected_email = idx;
                self.focused_item = idx;
                self.scroll_list.selected_index = idx;
                let task = self.scroll_list.scroll_to_selected(self.emails.len());
                (None, task)
            }
            Message::EncryptionLevelSelected(level) => {
                self.encryption_level = level;
                self.focused_item = level + self.emails.len() + 4;
                (None, Task::none())
            }
            Message::ActionButtonPressed(btn_idx) => {
                self.focused_item = btn_idx + self.emails.len();
                (None, Task::none())
            }
            Message::GoBack => (Some(ScreenEvent::GoToDashboard), Task::none()),
        }
    }

    pub fn handle_key(&mut self, key: &iced::keyboard::Key) -> Option<Message> {
        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) = key {
            return Some(Message::GoBack);
        }

        let emails_count = self.emails.len();
        let max_email_idx = if emails_count > 0 {
            emails_count - 1
        } else {
            0
        };
        let btn_start = emails_count;
        let btn_end = emails_count + 3;
        let enc_start = emails_count + 4;
        let enc_end = emails_count + 7;

        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) = key {
            if self.focused_item <= max_email_idx {
                return Some(Message::EmailSelected(self.focused_item));
            }
            if self.focused_item >= btn_start && self.focused_item <= btn_end {
                return Some(Message::ActionButtonPressed(self.focused_item - btn_start));
            }
            if self.focused_item >= enc_start && self.focused_item <= enc_end {
                return Some(Message::EncryptionLevelSelected(
                    self.focused_item - enc_start,
                ));
            }
        }

        if let iced::keyboard::Key::Character(c) = key {
            match c.as_str() {
                "h" => {
                    // Navigate Left
                    if self.focused_item >= btn_start && self.focused_item <= btn_end {
                        if self.focused_item > btn_start {
                            self.focused_item -= 1;
                        } else {
                            self.focused_item = self.selected_email;
                        }
                    } else if self.focused_item >= enc_start && self.focused_item <= enc_end {
                        // T1, T2, T3, T4 grid navigation
                        match self.focused_item - enc_start {
                            0 => {
                                self.focused_item = btn_start;
                            } // T1 -> Reply
                            1 => {
                                self.focused_item = btn_start + 1;
                            } // T2 -> Forward
                            2 => {
                                self.focused_item = enc_start;
                            } // T3 -> T1
                            3 => {
                                self.focused_item = enc_start + 1;
                            } // T4 -> T2
                            _ => {}
                        }
                    }
                }
                "l" => {
                    // Navigate Right
                    if self.focused_item <= max_email_idx {
                        self.focused_item = btn_start; // To Reply
                    } else if self.focused_item >= btn_start && self.focused_item <= btn_end {
                        if self.focused_item < btn_end {
                            self.focused_item += 1;
                        } else {
                            self.focused_item = enc_start + 1; // To T2
                        }
                    } else if self.focused_item >= enc_start && self.focused_item <= enc_end {
                        match self.focused_item - enc_start {
                            0 => {
                                self.focused_item = enc_start + 2;
                            } // T1 -> T3
                            1 => {
                                self.focused_item = enc_start + 3;
                            } // T2 -> T4
                            _ => {}
                        }
                    }
                }
                "k" => {
                    // Navigate Up
                    if self.focused_item <= max_email_idx {
                        if self.focused_item > 0 {
                            self.focused_item -= 1;
                            self.selected_email = self.focused_item;
                            return Some(Message::EmailSelected(self.focused_item));
                        }
                    } else if self.focused_item >= enc_start && self.focused_item <= enc_end {
                        match self.focused_item - enc_start {
                            1 => {
                                self.focused_item = enc_start;
                            } // T2 -> T1
                            3 => {
                                self.focused_item = enc_start + 2;
                            } // T4 -> T3
                            _ => {}
                        }
                    }
                }
                "j" => {
                    // Navigate Down
                    if self.focused_item <= max_email_idx {
                        if self.focused_item < max_email_idx {
                            self.focused_item += 1;
                            self.selected_email = self.focused_item;
                            return Some(Message::EmailSelected(self.focused_item));
                        }
                    } else if self.focused_item >= enc_start && self.focused_item <= enc_end {
                        match self.focused_item - enc_start {
                            0 => {
                                self.focused_item = enc_start + 1;
                            } // T1 -> T2
                            2 => {
                                self.focused_item = enc_start + 3;
                            } // T3 -> T4
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn view(
        &self,
        color_bg: Color,
        color_green_accent: Color,
        window_size: Option<iced::Size>,
    ) -> Element<'_, Message> {
        let (w, h) = match window_size {
            Some(size) => (size.width, size.height),
            None => (1920.0, 1080.0),
        };

        let section_header = |letter: &'static str, title: &'static str, est_x: f32, est_y: f32| {
            let (off_x, off_y) = get_radial_offsets(est_x, est_y, w, h);
            row![
                container(glowing_text(letter, 14, color_green_accent, off_x, off_y))
                    .padding(4)
                    .style(move |_| container::Style {
                        border: Border {
                            color: color_green_accent,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    }),
                Space::new(8.0, 0.0),
                glowing_text(title, 16, color_green_accent, off_x, off_y)
            ]
            .align_y(Alignment::Center)
        };

        // --- Column A: MAIL BOX ---
        let make_list_item = |idx: usize| {
            let item = &self.emails[idx];
            let is_selected = self.selected_email == idx;
            let is_focused = self.focused_item == idx;

            let text_color = if is_selected {
                color_bg
            } else {
                color_green_accent
            };
            let box_bg = if is_selected {
                Some(Background::Color(color_green_accent))
            } else {
                None
            };

            let est_item_y = h * 0.2 + (idx as f32) * 80.0;
            let (off_x, off_y) = get_radial_offsets(w * 0.15, est_item_y, w, h);

            let arrow_prefix = if is_focused && !is_selected { "> " } else { "" };
            let subject_txt = format!("{}{} {}", arrow_prefix, "[M]", item.subject);
            let from_txt = format!("AB: {}", item.from);

            let item_content = if is_selected {
                column![
                    text(subject_txt).size(14).style(move |_| text::Style {
                        color: Some(text_color)
                    }),
                    Space::new(0.0, 4.0),
                    text(from_txt).size(10).style(move |_| text::Style {
                        color: Some(text_color)
                    }),
                ]
            } else {
                column![
                    glowing_text(subject_txt, 14, text_color, off_x, off_y),
                    Space::new(0.0, 4.0),
                    glowing_text(from_txt, 10, text_color, off_x, off_y),
                ]
            };

            button(container(item_content).padding(iced::Padding {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 10.0,
            }))
            .width(Length::Fill)
            .height(Length::Fixed(70.0))
            .on_press(Message::EmailSelected(idx))
            .style(move |_, _| button::Style {
                background: box_bg,
                border: Border {
                    color: color_green_accent,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                text_color,
            })
        };

        let mut email_list = column![].spacing(5);
        for idx in 0..self.emails.len() {
            email_list = email_list.push(make_list_item(idx));
        }

        let (col_a_off_x, col_a_off_y) = get_radial_offsets(w * 0.15, h * 0.5, w, h);

        let col_a = column![
            section_header("A", "CAPSA NUNTIORUM", w * 0.33, 50.0),
            Space::new(0.0, 5.0),
            glowing_text(
                "SPARE TIME MANAGER WAS DEVELOPED BY\nSEOCHO. SERVING CUSTOMERS SINCE 2006.",
                9,
                color_green_accent,
                -0.5,
                -0.6
            ),
            Space::new(0.0, 15.0),
            glowing_border_container(
                scrollable(email_list)
                    .id(self.scroll_list.scrollable_id.clone())
                    .width(Length::Fill)
                    .height(Length::Fill),
                1.0,
                color_green_accent,
                col_a_off_x,
                col_a_off_y
            )
        ]
        .width(Length::FillPortion(4))
        .height(Length::Fill);

        // --- Column B: MESSAGE ---
        let (col_b_off_x, col_b_off_y) = get_radial_offsets(w * 0.78, h * 0.5, w, h);

        let col_b = if self.emails.is_empty() {
            column![
                section_header("B", "EPISTULA", w * 0.78, 50.0),
                Space::new(0.0, 15.0),
                glowing_border_container(
                    glowing_text(
                        "NULLUS NUNTIUS",
                        14,
                        color_green_accent,
                        col_b_off_x,
                        col_b_off_y
                    ),
                    1.0,
                    color_green_accent,
                    col_b_off_x,
                    col_b_off_y
                )
            ]
            .width(Length::FillPortion(5))
        } else {
            let current_email = &self.emails[self.selected_email];
            let make_action_button = |idx: usize, label: &'static str| {
                let item_idx = idx + self.emails.len();
                let is_focused = self.focused_item == item_idx;

                let text_color = if is_focused {
                    color_bg
                } else {
                    color_green_accent
                };
                let btn_bg = if is_focused {
                    Some(Background::Color(color_green_accent))
                } else {
                    None
                };

                let btn_label = if is_focused {
                    format!("> {}", label)
                } else {
                    label.to_string()
                };

                let btn_content: Element<Message> = if is_focused {
                    text(btn_label)
                        .size(12)
                        .style(move |_| text::Style {
                            color: Some(text_color),
                        })
                        .into()
                } else {
                    glowing_text(btn_label, 12, text_color, col_b_off_x, col_b_off_y)
                };

                button(btn_content)
                    .width(Length::Fill)
                    .height(Length::Fixed(40.0))
                    .on_press(Message::ActionButtonPressed(idx))
                    .style(move |_, _| button::Style {
                        background: btn_bg,
                        border: Border {
                            color: color_green_accent,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        shadow: Shadow::default(),
                        text_color,
                    })
            };

            column![
                section_header("B", "EPISTULA", w * 0.78, 50.0),
                Space::new(0.0, 15.0),
                glowing_border_container(
                    column![
                        // Email Header block
                        container(
                            column![
                                glowing_text(
                                    &current_email.subject,
                                    16,
                                    color_green_accent,
                                    col_b_off_x,
                                    col_b_off_y
                                ),
                                Space::new(0.0, 4.0),
                                glowing_text(
                                    format!("ab: {}", current_email.from),
                                    12,
                                    color_green_accent,
                                    col_b_off_x,
                                    col_b_off_y
                                ),
                            ]
                            .padding(10)
                        )
                        .width(Length::Fill)
                        .style(move |_| container::Style {
                            border: Border {
                                color: color_green_accent,
                                width: 1.0,
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        }),
                        Space::new(0.0, 15.0),
                        // Email Body
                        container(glowing_text(
                            &current_email.body,
                            12,
                            color_green_accent,
                            col_b_off_x,
                            col_b_off_y
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill),
                        Space::new(0.0, 15.0),
                        // Action Button Bar
                        row![
                            make_action_button(0, "RESPONDERE"),
                            make_action_button(1, "TRANSMITTERE"),
                            make_action_button(2, "DELETA"),
                            make_action_button(3, "NUNTIARE INUTILIS"),
                        ]
                        .spacing(5)
                        .width(Length::Fill)
                    ]
                    .padding(15),
                    1.0,
                    color_green_accent,
                    col_b_off_x,
                    col_b_off_y
                )
            ]
            .width(Length::FillPortion(5))
        };

        // --- Column C: ENCRIPTION LEVEL ---
        let make_encryption_button = |level: usize, label: &'static str| {
            let item_idx = level + self.emails.len() + 4;
            let is_selected = self.encryption_level == level;
            let is_focused = self.focused_item == item_idx;

            let text_color = if is_selected {
                color_bg
            } else {
                color_green_accent
            };
            let btn_bg = if is_selected {
                Some(Background::Color(color_green_accent))
            } else {
                None
            };

            let btn_text = if is_focused {
                format!("> {}", label)
            } else {
                label.to_string()
            };

            // Calculate grid layout position estimation:
            // T1 (col 0, row 0), T2 (col 0, row 1), T3 (col 1, row 0), T4 (col 1, row 1)
            let (grid_col, grid_row) = match level {
                0 => (0, 0), // T1
                1 => (0, 1), // T2
                2 => (1, 0), // T3
                3 => (1, 1), // T4
                _ => (0, 0),
            };

            let est_x = w * 0.9 + (grid_col as f32) * 50.0;
            let est_y = h * 0.2 + (grid_row as f32) * 60.0;
            let (off_x, off_y) = get_radial_offsets(est_x, est_y, w, h);

            let btn_content = if is_selected {
                text(btn_text)
                    .size(16)
                    .style(move |_| text::Style {
                        color: Some(text_color),
                    })
                    .into()
            } else {
                glowing_text(btn_text, 16, text_color, off_x, off_y)
            };

            button(btn_content)
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(50.0))
                .on_press(Message::EncryptionLevelSelected(level))
                .style(move |_, _| button::Style {
                    background: btn_bg,
                    border: Border {
                        color: color_green_accent,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    shadow: Shadow::default(),
                    text_color,
                })
        };

        let level_grid = column![
            row![
                make_encryption_button(0, "T1"),
                Space::new(10.0, 0.0),
                make_encryption_button(2, "T3"),
            ],
            Space::new(0.0, 10.0),
            row![
                make_encryption_button(1, "T2"),
                Space::new(10.0, 0.0),
                make_encryption_button(3, "T4"),
            ],
        ];

        let col_c = column![
            section_header("C", "GRADUS ENCRIPTIONIS", w * 0.95, 50.0),
            Space::new(0.0, 15.0),
            level_grid,
        ]
        .width(Length::Shrink);

        container(
            row![col_a, col_b, col_c]
                .spacing(25)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .padding(iced::Padding {
            left: 15.0,
            right: 15.0,
            top: 0.0,
            bottom: 0.0,
        })
        .into()
    }
}
