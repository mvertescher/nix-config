use crate::glow::{get_radial_offsets, glowing_border_container, glowing_text};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow};

#[derive(Debug, Clone)]
pub struct StoreItem {
    pub name: String,
    pub sub: String,
    pub dps: u16,
    pub pnt: u16,
    pub acc: u16,
    pub rof: u16,
    pub desc: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    CategorySelected(usize),
    ItemSelected(usize),
    GoBack,
    Tick(std::time::Instant),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenEvent {
    GoToDashboard,
}

pub struct StoreScreen {
    selected_category: usize,
    selected_item: usize,
    focused_item: usize,
    card_heights: Vec<f32>,
    categories: Vec<String>,
    items: Vec<Vec<StoreItem>>,
}

impl StoreScreen {
    pub fn new(categories: Vec<String>, items: Vec<Vec<StoreItem>>) -> Self {
        let mut s = Self {
            selected_category: 1, // Default to SMG (index 1)
            selected_item: 0,
            focused_item: 1,
            card_heights: vec![],
            categories,
            items,
        };
        s.reset_heights();
        s
    }

    fn reset_heights(&mut self) {
        if self.selected_category < self.items.len() {
            let count = self.items[self.selected_category].len();
            self.card_heights = vec![105.0; count];
            if self.selected_item < count {
                self.card_heights[self.selected_item] = 320.0;
            }
        } else {
            self.card_heights = vec![];
        }
    }

    pub fn update(&mut self, message: Message) -> Option<ScreenEvent> {
        match message {
            Message::CategorySelected(idx) => {
                self.selected_category = idx;
                self.selected_item = 0;
                self.focused_item = idx;
                self.reset_heights();
                None
            }
            Message::ItemSelected(idx) => {
                self.selected_item = idx;
                self.focused_item = idx + self.categories.len();
                None
            }
            Message::GoBack => Some(ScreenEvent::GoToDashboard),
            Message::Tick(_instant) => {
                if self.selected_category < self.items.len() {
                    let count = self.items[self.selected_category].len();
                    for i in 0..count {
                        if i < self.card_heights.len() {
                            let target = if self.selected_item == i {
                                320.0
                            } else {
                                105.0
                            };
                            let diff = target - self.card_heights[i];
                            if diff.abs() > 0.5 {
                                self.card_heights[i] += diff * 0.25;
                            } else {
                                self.card_heights[i] = target;
                            }
                        }
                    }
                }
                None
            }
        }
    }

    pub fn handle_key(&mut self, key: &iced::keyboard::Key) -> Option<Message> {
        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) = key {
            return Some(Message::GoBack);
        }

        let cat_count = self.categories.len();
        let max_cat_idx = if cat_count > 0 { cat_count - 1 } else { 0 };

        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) = key {
            if self.focused_item < cat_count {
                return Some(Message::CategorySelected(self.focused_item));
            } else {
                return Some(Message::ItemSelected(self.focused_item - cat_count));
            }
        }

        if let iced::keyboard::Key::Character(c) = key {
            let items_count = if self.selected_category < self.items.len() {
                self.items[self.selected_category].len()
            } else {
                0
            };
            match c.as_str() {
                "h" => {
                    // Navigate Left
                    if self.focused_item >= cat_count {
                        if self.focused_item > cat_count {
                            self.focused_item -= 1;
                            self.selected_item = self.focused_item - cat_count;
                        } else {
                            self.focused_item = self.selected_category;
                        }
                    }
                }
                "l" => {
                    // Navigate Right
                    if self.focused_item < cat_count {
                        self.focused_item = cat_count; // Move to first card
                        self.selected_item = 0;
                    } else if self.focused_item - cat_count < items_count - 1 {
                        self.focused_item += 1;
                        self.selected_item = self.focused_item - cat_count;
                    }
                }
                "k" => {
                    // Navigate Up inside left category list
                    if self.focused_item < cat_count && self.focused_item > 0 {
                        self.focused_item -= 1;
                        self.selected_category = self.focused_item;
                        self.selected_item = 0;
                        return Some(Message::CategorySelected(self.focused_item));
                    }
                }
                "j" => {
                    // Navigate Down inside left category list
                    if self.focused_item < max_cat_idx {
                        self.focused_item += 1;
                        self.selected_category = self.focused_item;
                        self.selected_item = 0;
                        return Some(Message::CategorySelected(self.focused_item));
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
    ) -> Element<Message> {
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

        // --- Left Panel ---
        let brand_block = column![
            glowing_text("4ST", 42, color_green_accent, -0.8, -0.6),
            glowing_text("STORE", 12, color_green_accent, -0.8, -0.5),
        ]
        .align_x(Alignment::Center);

        let (col_a_meta_off_x, col_a_meta_off_y) = get_radial_offsets(w * 0.1, h * 0.45, w, h);

        let customer_metadata = glowing_border_container(
            column![
                row![
                    text("CUSTOMER").size(10).style(move |_| text::Style {
                        color: Some(color_green_accent)
                    }),
                    Space::new(Length::Fill, 0.0),
                    text("#NC488402").size(10).style(move |_| text::Style {
                        color: Some(color_green_accent)
                    }),
                ],
                container(Space::new(Length::Fill, 1.0)).style(move |_| container::Style {
                    background: Some(Background::Color(color_green_accent)),
                    ..Default::default()
                }),
                row![
                    text("LOYALTY DISCOUNT")
                        .size(8)
                        .style(move |_| text::Style {
                            color: Some(color_green_accent)
                        }),
                    Space::new(Length::Fill, 0.0),
                    text("10%").size(8).style(move |_| text::Style {
                        color: Some(color_green_accent)
                    }),
                ],
                row![
                    text("LAST UPDATE").size(8).style(move |_| text::Style {
                        color: Some(color_green_accent)
                    }),
                    Space::new(Length::Fill, 0.0),
                    text("10/05/2077").size(8).style(move |_| text::Style {
                        color: Some(color_green_accent)
                    }),
                ],
            ]
            .spacing(4),
            1.0,
            color_green_accent,
            col_a_meta_off_x,
            col_a_meta_off_y,
        );

        let make_category_button = |idx: usize| {
            let name = &self.categories[idx];
            let is_selected = self.selected_category == idx;
            let is_focused = self.focused_item == idx;

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

            let label = if is_focused && !is_selected {
                format!("> {}", name)
            } else {
                name.to_string()
            };

            let btn_content: Element<Message> = if is_selected {
                text(label)
                    .size(14)
                    .style(move |_| text::Style {
                        color: Some(text_color),
                    })
                    .into()
            } else {
                let (off_x, off_y) =
                    get_radial_offsets(w * 0.1, h * 0.3 + (idx as f32) * 50.0, w, h);
                glowing_text(label, 14, text_color, off_x, off_y)
            };

            button(btn_content)
                .width(Length::Fill)
                .height(Length::Fixed(45.0))
                .on_press(Message::CategorySelected(idx))
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

        let mut category_list = column![].spacing(5);
        for idx in 0..self.categories.len() {
            category_list = category_list.push(make_category_button(idx));
        }

        let left_panel = column![
            brand_block,
            Space::new(0.0, 10.0),
            customer_metadata,
            Space::new(0.0, 15.0),
            category_list,
            Space::new(0.0, Length::Fill),
            glowing_text(
                "SPARE TIME MANAGER WAS DEVELOPED BY\nSEOCHO. SERVING CUSTOMERS SINCE 2006.",
                8,
                color_green_accent,
                -0.8,
                0.8
            ),
        ]
        .width(Length::FillPortion(2))
        .height(Length::Fill);

        // --- Right Panel ---
        // Description on top
        let description_header = glowing_text(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            12,
            color_green_accent,
            0.2,
            -0.7,
        );

        // Horizontal card picker list
        const EMPTY_ITEMS: &[StoreItem] = &[];
        let current_category_items = self
            .items
            .get(self.selected_category)
            .map(|v| v.as_slice())
            .unwrap_or(EMPTY_ITEMS);
        let make_item_card = |idx: usize| {
            let item = &current_category_items[idx];
            let is_selected = self.selected_item == idx;
            let is_focused = self.focused_item == idx + self.categories.len();

            let text_color = if is_selected {
                color_bg
            } else {
                color_green_accent
            };
            let card_header_bg = if is_selected {
                Some(Background::Color(color_green_accent))
            } else {
                None
            };

            // Calculate dynamic radial offsets
            let est_x = w * 0.45 + (idx as f32) * 220.0;
            let est_y = h * 0.55;
            let (_off_x, _off_y) = get_radial_offsets(est_x, est_y, w, h);

            let bold_font = iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            };

            let card_header = container(
                column![
                    text(&item.name)
                        .size(14)
                        .font(bold_font)
                        .style(move |_| text::Style {
                            color: Some(text_color)
                        }),
                    text(&item.sub).size(9).style(move |_| text::Style {
                        color: Some(text_color)
                    }),
                ]
                .padding(8),
            )
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: card_header_bg,
                border: Border {
                    color: color_green_accent,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            });

            // Card statistics block
            let stats_block = row![
                column![
                    text("DPS").size(8),
                    text(item.dps.to_string()).size(14).font(bold_font)
                ]
                .align_x(Alignment::Center),
                Space::new(Length::Fill, 0.0),
                column![
                    text("PNT").size(8),
                    text(item.pnt.to_string()).size(14).font(bold_font)
                ]
                .align_x(Alignment::Center),
                Space::new(Length::Fill, 0.0),
                column![
                    text("ACC").size(8),
                    text(item.acc.to_string()).size(14).font(bold_font)
                ]
                .align_x(Alignment::Center),
                Space::new(Length::Fill, 0.0),
                column![
                    text("ROF").size(8),
                    text(item.rof.to_string()).size(14).font(bold_font)
                ]
                .align_x(Alignment::Center),
            ]
            .padding(6);

            let card_height = self.card_heights[idx];

            let inner_card_content = column![
                card_header,
                stats_block,
                container(Space::new(Length::Fill, 1.0)).style(move |_| container::Style {
                    background: Some(Background::Color(color_green_accent)),
                    ..Default::default()
                }),
                container(text(&item.desc).size(11).style(move |_| text::Style {
                    color: Some(color_green_accent)
                }))
                .padding(8)
                .height(Length::Fill),
            ];

            let border_color = if is_focused {
                color_green_accent
            } else {
                Color {
                    a: 0.4,
                    ..color_green_accent
                }
            };
            let border_width = if is_focused { 2.0 } else { 1.0 };

            button(inner_card_content)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(card_height))
                .padding(0)
                .on_press(Message::ItemSelected(idx))
                .style(move |_, _| button::Style {
                    background: Some(Background::Color(color_bg)),
                    border: Border {
                        color: border_color,
                        width: border_width,
                        radius: 0.0.into(),
                    },
                    shadow: Shadow::default(),
                    text_color: color_green_accent,
                })
        };

        let mut cards_row = row![].spacing(20);
        for idx in 0..current_category_items.len() {
            cards_row = cards_row.push(make_item_card(idx));
        }

        let right_panel = column![
            section_header("B", "STORE ITEMS PICKER", w * 0.6, 50.0),
            Space::new(0.0, 10.0),
            description_header,
            Space::new(0.0, 20.0),
            cards_row,
        ]
        .width(Length::FillPortion(6))
        .height(Length::Fill);

        container(
            row![left_panel, right_panel]
                .spacing(35)
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
