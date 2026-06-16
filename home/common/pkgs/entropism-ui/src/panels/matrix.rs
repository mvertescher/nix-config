use crate::glow::{get_radial_offsets, glowing_border_container, glowing_text};
use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow};

const VISIBLE_COLS: usize = 12;
const VISIBLE_ROWS: usize = 22;

#[derive(Debug, Clone)]
pub enum Message {
    GoBack,
    ScrollLeft,
    ScrollRight,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenEvent {
    GoToDashboard,
}

pub struct MatrixScreen {
    cols: Vec<String>,
    rows: Vec<String>,
    data: Vec<Vec<String>>,
    scroll_row: usize,
    scroll_col: usize,
}

impl MatrixScreen {
    pub fn new() -> Self {
        let cols = (0..25).map(|i| format!("SECURE_NODE_{:02}", i)).collect();
        let rows = (0..60)
            .map(|i| format!("PORT_{:04}", 8000 + i * 11))
            .collect();

        // Generate deterministic mock data
        let mut data = Vec::new();
        for r in 0..60 {
            let mut row_data = Vec::new();
            for c in 0..25 {
                let val = (r * 7 + c * 13) % 100;
                let status = if val % 5 == 0 {
                    "OFFLINE".to_string()
                } else if val % 3 == 0 {
                    format!("ERR_0x{:02X}", val)
                } else {
                    format!("OK_{:02}%", val)
                };
                row_data.push(status);
            }
            data.push(row_data);
        }

        Self {
            cols,
            rows,
            data,
            scroll_row: 0,
            scroll_col: 0,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<ScreenEvent> {
        match message {
            Message::GoBack => Some(ScreenEvent::GoToDashboard),
            Message::ScrollLeft => {
                self.scroll_col = self.scroll_col.saturating_sub(1);
                None
            }
            Message::ScrollRight => {
                self.scroll_col =
                    (self.scroll_col + 1).min(self.cols.len().saturating_sub(VISIBLE_COLS));
                None
            }
            Message::ScrollUp => {
                self.scroll_row = self.scroll_row.saturating_sub(1);
                None
            }
            Message::ScrollDown => {
                self.scroll_row =
                    (self.scroll_row + 1).min(self.rows.len().saturating_sub(VISIBLE_ROWS));
                None
            }
        }
    }

    pub fn handle_key(&mut self, key: &iced::keyboard::Key) -> Option<Message> {
        match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => Some(Message::GoBack),
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => Some(Message::GoBack),
            iced::keyboard::Key::Character(c) => match c.as_str() {
                "h" => Some(Message::ScrollLeft),
                "l" => Some(Message::ScrollRight),
                "k" => Some(Message::ScrollUp),
                "j" => Some(Message::ScrollDown),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn view(
        &self,
        _color_bg: Color,
        color_green_accent: Color,
        window_size: Option<iced::Size>,
    ) -> Element<'_, Message> {
        let (w, h) = match window_size {
            Some(size) => (size.width, size.height),
            None => (1920.0, 1080.0),
        };

        let (off_x, off_y) = get_radial_offsets(w * 0.5, h * 0.5, w, h);

        // Header and back button
        let back_btn =
            iced::widget::button(text("RETURN TO DASHBOARD").size(12).style(move |_| {
                text::Style {
                    color: Some(color_green_accent),
                }
            }))
            .padding(8)
            .on_press(Message::GoBack)
            .style(move |_, _| iced::widget::button::Style {
                background: None,
                border: Border {
                    color: color_green_accent,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                text_color: color_green_accent,
            });

        // Grid offset indicator HUD
        let coords_hud = container(
            text(format!(
                "GRID COORDS: X: {}..{} | Y: {}..{}",
                self.scroll_col,
                self.scroll_col + VISIBLE_COLS - 1,
                self.scroll_row,
                self.scroll_row + VISIBLE_ROWS - 1
            ))
            .size(12)
            .style(move |_| text::Style {
                color: Some(color_green_accent),
            }),
        )
        .padding(8)
        .style(move |_| container::Style {
            border: Border {
                color: Color::from_rgba(0.57, 0.72, 0.62, 0.3),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        let header_row = row![
            glowing_text(
                "MATRIX DATA ROUTING GRID",
                20,
                color_green_accent,
                off_x,
                off_y
            ),
            Space::new(Length::Fill, 0.0),
            coords_hud,
            Space::new(20.0, 0.0),
            back_btn
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        // Build the 2D spreadsheet layout
        let cell_width = 130.0;
        let cell_height = 36.0;

        // Slice columns and rows
        let visible_cols = &self.cols[self.scroll_col..self.scroll_col + VISIBLE_COLS];
        let visible_rows = &self.rows[self.scroll_row..self.scroll_row + VISIBLE_ROWS];

        // Render header row cells: [ "PORT / NODE", cols[0], cols[1], ... ]
        let mut col_headers =
            vec![
                container(text("PORT // NODE").size(11).style(move |_| text::Style {
                    color: Some(color_green_accent),
                }))
                .width(Length::Fixed(cell_width))
                .height(Length::Fixed(cell_height))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(move |_| container::Style {
                    border: Border {
                        color: color_green_accent,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    background: Some(Background::Color(Color::from_rgba(0.57, 0.72, 0.62, 0.08))),
                    ..Default::default()
                })
                .into(),
            ];

        for col_name in visible_cols {
            col_headers.push(
                container(text(col_name).size(10).style(move |_| text::Style {
                    color: Some(color_green_accent),
                }))
                .width(Length::Fixed(cell_width))
                .height(Length::Fixed(cell_height))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(move |_| container::Style {
                    border: Border {
                        color: color_green_accent,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    background: Some(Background::Color(Color::from_rgba(0.57, 0.72, 0.62, 0.05))),
                    ..Default::default()
                })
                .into(),
            );
        }

        let first_row = row(col_headers);

        // Render all visible data rows
        let mut grid_rows = vec![first_row.into()];

        for (r_relative, row_name) in visible_rows.iter().enumerate() {
            let r_actual = self.scroll_row + r_relative;

            let mut row_cells =
                vec![
                    container(text(row_name).size(11).style(move |_| text::Style {
                        color: Some(color_green_accent),
                    }))
                    .width(Length::Fixed(cell_width))
                    .height(Length::Fixed(cell_height))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(move |_| container::Style {
                        border: Border {
                            color: color_green_accent,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        background: Some(Background::Color(Color::from_rgba(
                            0.57, 0.72, 0.62, 0.05,
                        ))),
                        ..Default::default()
                    })
                    .into(),
                ];

            for c_relative in 0..VISIBLE_COLS {
                let c_actual = self.scroll_col + c_relative;
                let cell_val = &self.data[r_actual][c_actual];

                // Style cell based on status
                let cell_color = if cell_val.starts_with("OK") {
                    color_green_accent
                } else if cell_val.starts_with("ERR") {
                    Color::from_rgb(0.9, 0.3, 0.3) // soft red
                } else {
                    Color::from_rgb(0.5, 0.5, 0.5) // gray
                };

                row_cells.push(
                    container(text(cell_val).size(10).style(move |_| text::Style {
                        color: Some(cell_color),
                    }))
                    .width(Length::Fixed(cell_width))
                    .height(Length::Fixed(cell_height))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(move |_| container::Style {
                        border: Border {
                            color: Color::from_rgba(0.57, 0.72, 0.62, 0.3),
                            width: 0.5,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    })
                    .into(),
                );
            }

            grid_rows.push(row(row_cells).into());
        }

        // Draw a static viewport of rows/columns
        let total_grid_width = cell_width * (VISIBLE_COLS + 1) as f32;
        let inner_grid = column(grid_rows).width(Length::Fixed(total_grid_width));

        let grid_container = glowing_border_container(
            container(inner_grid)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            1.0,
            color_green_accent,
            off_x,
            off_y,
        );

        let layout_content = column![header_row, Space::new(0.0, 15.0), grid_container]
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }
}
