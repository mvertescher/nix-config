use crate::glow::{get_radial_offsets, glowing_border_container, glowing_text, radiate_element};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow};

pub struct DashboardScreen {
    selected_block: usize,
    security_level: usize,
    focused_item: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    BlockSelected(usize),
    BlockEntered(usize),
    SecurityLevelSelected(usize),
    Disconnect,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenEvent {
    Disconnect,
    GoToChat,
    GoToMail,
    GoToStore,
    GoToMatrix,
}

const BLOCKS: &[(&str, &str)] = &[
    ("EMAILS", "Access terminal archive and mail client modules."),
    (
        "MATRIX",
        "Manage data routing grids and sub-net architectures.",
    ),
    (
        "STORE",
        "Browse internal inventory database and purchase combat hardware surplus.",
    ),
    (
        "CHAT",
        "Access real-time local networks and peer chat channels.",
    ),
    ("PRIVATE", "Encrypted vault modules for credential storage."),
    (
        "DEVICES",
        "Control connected neural links and local access bridges.",
    ),
];

impl DashboardScreen {
    pub fn new() -> Self {
        Self {
            selected_block: 2, // Default to BRAINDANCE (index 2)
            security_level: 1, // Default to T2 (index 1)
            focused_item: 2,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<ScreenEvent> {
        match message {
            Message::BlockSelected(idx) => {
                self.selected_block = idx;
                self.focused_item = idx;
                None
            }
            Message::BlockEntered(idx) => {
                self.selected_block = idx;
                self.focused_item = idx;
                if idx == 0 {
                    return Some(ScreenEvent::GoToMail);
                }
                if idx == 1 {
                    return Some(ScreenEvent::GoToMatrix);
                }
                if idx == 2 {
                    return Some(ScreenEvent::GoToStore);
                }
                if idx == 3 {
                    return Some(ScreenEvent::GoToChat);
                }
                None
            }
            Message::SecurityLevelSelected(level) => {
                self.security_level = level;
                self.focused_item = level + 6;
                None
            }
            Message::Disconnect => Some(ScreenEvent::Disconnect),
        }
    }

    pub fn handle_key(&mut self, key: &iced::keyboard::Key) -> Option<Message> {
        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) = key {
            return Some(Message::Disconnect);
        }
        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) = key {
            if self.focused_item <= 5 {
                return Some(Message::BlockEntered(self.focused_item));
            }
            if self.focused_item >= 6 && self.focused_item <= 9 {
                return Some(Message::SecurityLevelSelected(self.focused_item - 6));
            }
        }
        if let iced::keyboard::Key::Character(c) = key {
            match c.as_str() {
                "h" => match self.focused_item {
                    1 | 2 | 4 | 5 => {
                        self.focused_item -= 1;
                        self.selected_block = self.focused_item;
                        return Some(Message::BlockSelected(self.focused_item));
                    }
                    6 | 7 => {
                        self.focused_item = 2;
                        self.selected_block = 2;
                        return Some(Message::BlockSelected(2));
                    }
                    8 | 9 => {
                        self.focused_item = 5;
                        self.selected_block = 5;
                        return Some(Message::BlockSelected(5));
                    }
                    _ => {}
                },
                "l" => match self.focused_item {
                    0 | 1 | 3 | 4 => {
                        self.focused_item += 1;
                        self.selected_block = self.focused_item;
                        return Some(Message::BlockSelected(self.focused_item));
                    }
                    2 => {
                        self.focused_item = 7;
                    }
                    5 => {
                        self.focused_item = 9;
                    }
                    _ => {}
                },
                "k" => match self.focused_item {
                    3 | 4 | 5 => {
                        self.focused_item -= 3;
                        self.selected_block = self.focused_item;
                        return Some(Message::BlockSelected(self.focused_item));
                    }
                    7 | 8 | 9 => {
                        self.focused_item -= 1;
                    }
                    _ => {}
                },
                "j" => match self.focused_item {
                    0 | 1 | 2 => {
                        self.focused_item += 3;
                        self.selected_block = self.focused_item;
                        return Some(Message::BlockSelected(self.focused_item));
                    }
                    6 | 7 | 8 => {
                        self.focused_item += 1;
                    }
                    _ => {}
                },
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
        let make_block = |idx: usize| -> Element<Message> {
            let name = BLOCKS[idx].0;
            let is_selected = self.focused_item == idx;

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

            let metadata_divider =
                container(Space::new(1.0, Length::Fill)).style(move |_| container::Style {
                    background: Some(Background::Color(text_color)),
                    ..Default::default()
                });

            // Estimated layout center coordinates for each tile
            let (est_x, est_y) = match idx {
                0 => (w * 0.1, h * 0.35), // Emails
                1 => (w * 0.3, h * 0.35), // Matrix
                2 => (w * 0.5, h * 0.35), // Braindance
                3 => (w * 0.1, h * 0.65), // Blank
                4 => (w * 0.3, h * 0.65), // Private
                5 => (w * 0.5, h * 0.65), // Devices
                _ => (0.0, 0.0),
            };

            let (off_x, off_y) = get_radial_offsets(est_x, est_y, w, h);

            let inner_content = if is_selected {
                column![
                    Space::new(0.0, Length::Fill),
                    text(name).size(16).style(move |_| text::Style {
                        color: Some(text_color)
                    }),
                    Space::new(0.0, Length::Fill),
                    container(Space::new(Length::Fill, 1.0)).style(move |_| container::Style {
                        background: Some(Background::Color(text_color)),
                        ..Default::default()
                    }),
                    row![
                        Space::new(3.0, 0.0),
                        text("BLOCK DETAIL LEFT")
                            .size(7)
                            .style(move |_| text::Style {
                                color: Some(text_color)
                            }),
                        Space::new(3.0, 0.0),
                        metadata_divider,
                        Space::new(3.0, 0.0),
                        text("BLOCK DETAIL RIGHT")
                            .size(7)
                            .style(move |_| text::Style {
                                color: Some(text_color)
                            }),
                        Space::new(3.0, 0.0),
                    ]
                    .align_y(Alignment::Center)
                    .height(Length::Fixed(24.0))
                ]
                .align_x(Alignment::Center)
            } else {
                column![
                    Space::new(0.0, Length::Fill),
                    glowing_text(name, 16, text_color, off_x, off_y),
                    Space::new(0.0, Length::Fill),
                    container(Space::new(Length::Fill, 1.0)).style(move |_| container::Style {
                        background: Some(Background::Color(text_color)),
                        ..Default::default()
                    }),
                    row![
                        Space::new(3.0, 0.0),
                        glowing_text("BLOCK DETAIL LEFT", 7, text_color, off_x, off_y),
                        Space::new(3.0, 0.0),
                        metadata_divider,
                        Space::new(3.0, 0.0),
                        glowing_text("BLOCK DETAIL RIGHT", 7, text_color, off_x, off_y),
                        Space::new(3.0, 0.0),
                    ]
                    .align_y(Alignment::Center)
                    .height(Length::Fixed(24.0))
                ]
                .align_x(Alignment::Center)
            };

            if is_selected {
                let block_btn = button(inner_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .on_press(Message::BlockEntered(idx))
                    .style(move |_, _| button::Style {
                        background: box_bg,
                        border: Border {
                            color: color_green_accent,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        shadow: Shadow::default(),
                        text_color,
                    });

                container(block_btn)
                    .padding(8.0)
                    .width(Length::Fixed(170.0))
                    .height(Length::Fixed(195.0))
                    .into()
            } else {
                let block_btn = button(inner_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(0)
                    .on_press(Message::BlockEntered(idx))
                    .style(move |_, _| button::Style {
                        background: None,
                        border: Border::default(),
                        shadow: Shadow::default(),
                        text_color,
                    });

                let glowing_container =
                    glowing_border_container(block_btn, 1.0, color_green_accent, off_x, off_y);

                container(glowing_container)
                    .width(Length::Fixed(170.0))
                    .height(Length::Fixed(195.0))
                    .into()
            }
        };

        let grid_row1 = row![make_block(0), make_block(1), make_block(2)].spacing(45);
        let grid_row2 = row![make_block(3), make_block(4), make_block(5)].spacing(45);

        let col_a = column![
            section_header("A", "MAIL BOX", w * 0.33, 50.0),
            container(
                column![grid_row1, grid_row2]
                    .spacing(45)
                    .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        ]
        .width(Length::FillPortion(4))
        .height(Length::Fill);

        // --- Column B: MESSAGE ---
        let selected_block_data = BLOCKS[self.selected_block];
        let (col_b_off_x, col_b_off_y) = get_radial_offsets(w * 0.78, h * 0.5, w, h);

        let col_b = column![
            section_header("B", "MESSAGE", w * 0.78, 50.0),
            Space::new(0.0, 15.0),
            glowing_border_container(
                column![
                    radiate_element(color_green_accent, col_b_off_x, col_b_off_y, move |c| {
                        container(
                            text(selected_block_data.0)
                                .size(16)
                                .style(move |_| text::Style { color: Some(c) }),
                        )
                        .padding(12)
                        .width(Length::Fill)
                        .style(move |_| container::Style {
                            border: Border {
                                color: c,
                                width: 1.0,
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        })
                        .into()
                    }),
                    Space::new(0.0, 15.0),
                    glowing_text(
                        selected_block_data.1,
                        14,
                        color_green_accent,
                        col_b_off_x,
                        col_b_off_y
                    ),
                ]
                .padding(15),
                1.0,
                color_green_accent,
                col_b_off_x,
                col_b_off_y
            )
        ]
        .width(Length::FillPortion(2));

        // --- Column C: SECURITY LEVEL ---
        let make_level = |level: usize| {
            let label = match level {
                0 => "T1",
                1 => "T2",
                2 => "T3",
                3 => "T4",
                _ => "",
            };

            let is_selected = self.security_level == level;
            let is_focused = self.focused_item == level + 6;

            let btn_text = if is_focused {
                format!("> {}", label)
            } else {
                label.to_string()
            };

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

            // Dynamic offsets for levels (Column C is far right)
            let est_level_y = match level {
                0 => h * 0.2,
                1 => h * 0.35,
                2 => h * 0.5,
                3 => h * 0.65,
                _ => 0.0,
            };
            let (col_c_off_x, col_c_off_y) = get_radial_offsets(w * 0.95, est_level_y, w, h);

            let btn_content = if is_selected {
                text(btn_text)
                    .size(16)
                    .style(move |_| text::Style {
                        color: Some(text_color),
                    })
                    .into()
            } else {
                glowing_text(btn_text, 16, text_color, col_c_off_x, col_c_off_y)
            };

            button(btn_content)
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(50.0))
                .on_press(Message::SecurityLevelSelected(level))
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

        let col_c = column![
            section_header("C", "SECURITY LEVEL", w * 0.95, 50.0),
            Space::new(0.0, 15.0),
            column![make_level(0), make_level(1), make_level(2), make_level(3),]
                .spacing(15)
                .height(Length::Fill)
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
