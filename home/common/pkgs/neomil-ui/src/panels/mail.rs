use iced::widget::{canvas, column, container, row, text, Space, scrollable, mouse_area, stack};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse};
use crate::colors;
use crate::background::background;
use crate::widgets::{floppy_icon, message_card, text_box, VerticalText};
use crate::top_bar::top_bar;
use crate::fonts::{
    FONT_ORBITRON_REGULAR, FONT_ORBITRON_MEDIUM, FONT_ORBITRON_BOLD,
    FONT_RAJDHANI_REGULAR,
};

const FONT_MEDIUM: iced::Font = FONT_ORBITRON_MEDIUM;
const FONT_BOLD: iced::Font = FONT_ORBITRON_BOLD;

/// Data structure representing a single message in a thread.
#[derive(Debug, Clone)]
pub struct ThreadMessage {
    pub sender: String,
    pub body: String,
    pub timestamp: String, // New!
}

/// Data structure representing an email/message.
#[derive(Debug, Clone)]
pub struct Email {
    pub id: usize,
    pub title: String,
    pub sender: String,
    pub body: String,
    pub is_new: bool,
    pub timestamp: String, // New!
    pub thread: Vec<ThreadMessage>, // Thread of replies
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MailFocus {
    List,
    Content,
}

/// The mail panel view.
/// Assembles the message list on the left and the selected message detail on the right.
pub fn mail_panel<'a, Message: 'static + Clone>(
    emails: &'a [Email],
    selected_id: Option<usize>,
    on_select: impl Fn(usize) -> Message + Clone + 'static,
    on_delete: impl Fn(usize) -> Message + Clone + 'static,
    list_scrollable_id: iced::widget::scrollable::Id,
    content_scrollable_id: iced::widget::scrollable::Id,
    focus: MailFocus,
    color_accent: Color,
) -> Element<'a, Message> {
    let list_color = if focus == MailFocus::List { color_accent } else { Color { a: 0.3, ..color_accent } };
    let content_color = if focus == MailFocus::Content { color_accent } else { Color { a: 0.3, ..color_accent } };
    
    // --- 1. LEFT COLUMN: Message List ---
    let left_header = row![
        container(
            text("MESSAGES")
                .size(12)
                .font(FONT_MEDIUM)
                .style(move |_| text::Style { color: Some(list_color) })
        )
        .padding([5, 15])
        .style(move |_| container::Style {
            border: iced::Border {
                color: list_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    // Build the list of message rows
    let mut list_column = column![]
        .spacing(10)
        .width(Length::Fill)
        .padding(iced::Padding {
            top: 0.0,
            right: 20.0,
            bottom: 0.0,
            left: 0.0,
        });
    for email in emails {
        let is_selected = Some(email.id) == selected_id;
        let row_item = row![
            floppy_icon(list_color, is_selected, 1.0),
            Space::with_width(10),
            message_card(
                &email.title,
                &email.sender,
                email.is_new,
                is_selected,
                (on_select.clone())(email.id),
                list_color,
            )
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);
        
        list_column = list_column.push(row_item);
    }

    let left_col = column![
        left_header,
        Space::with_height(20),
        scrollable(list_column)
            .id(list_scrollable_id)
            .direction(iced::widget::scrollable::Direction::Vertical(
                iced::widget::scrollable::Scrollbar::new()
                    .width(4.0)
                    .scroller_width(4.0)
                    .margin(5.0)
            ))
            .height(Length::Fill)
            .width(Length::Fill)
            .style(move |_, _| {
                use iced::widget::scrollable::{Style, Rail, Scroller};
                Style {
                    container: iced::widget::container::Style::default(),
                    vertical_rail: Rail {
                        background: Some(Color { a: 0.02, ..list_color }.into()),
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        scroller: Scroller {
                            color: Color { a: 0.3, ..list_color },
                            border: iced::Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: 4.0.into(),
                            },
                        },
                    },
                    horizontal_rail: Rail {
                        background: None,
                        border: iced::Border::default(),
                        scroller: Scroller {
                            color: Color::TRANSPARENT,
                            border: iced::Border::default(),
                        },
                    },
                    gap: None,
                }
            })
    ]
    .width(Length::FillPortion(3))
    .height(Length::Fill);

    // --- 2. RIGHT COLUMN: Message Detail ---
    let selected_email = selected_id
        .and_then(|id| emails.iter().find(|e| e.id == id));

    let right_col = if let Some(email) = selected_email {


        // Detail Buttons (Footer)
        let footer_buttons = row![
            cut_button("DELETE", Some((on_delete.clone())(email.id)), content_color, true, 30.0),
            cut_button("REPLY", None, content_color, false, 0.0),
            cut_button("ARCHIVE", None, content_color, false, 0.0),
            cut_button("CLOSE", None, content_color, false, 0.0),
        ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fixed(38.0));

        column![
            container(
                text("CONTENT")
                    .size(12)
                    .font(FONT_MEDIUM)
                    .style(move |_| text::Style { color: Some(content_color) })
            )
            .padding([5, 15])
            .style(move |_| container::Style {
                border: iced::Border {
                    color: content_color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
            Space::with_height(20),
            text_box(
                &email.title,
                Some(&email.timestamp),
                render_thread(email, color_accent),
                Some(content_scrollable_id.clone()),
                &["PETROCHEM", "BETTERLIFE TEC"],
                "", // No logo
                "",
                "",
                Some(footer_buttons.into()),
                content_color,
            ),
        ]
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .align_x(Alignment::Start)
    } else {
        // Empty state when no email is selected
        column![
            container(
                text("CONTENT")
                    .size(12)
                    .font(FONT_MEDIUM)
                    .style(move |_| text::Style { color: Some(content_color) })
            )
            .padding([5, 15])
            .style(move |_| container::Style {
                border: iced::Border {
                    color: content_color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
            Space::with_height(20),
            container(
                text("SELECT A MESSAGE TO VIEW CONTENT")
                    .font(FONT_MEDIUM)
                    .size(14)
                    .style(move |_| text::Style { color: Some(Color { a: 0.3, ..content_color }) })
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| container::Style {
                border: iced::Border {
                    color: Color { a: 0.1, ..content_color },
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
        ]
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .align_x(Alignment::Start)
    };

    // --- 3. ASSEMBLE MAIN AREA ---
    let main_area = row![
        left_col,
        Space::with_width(40),
        right_col,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(40);

    // --- 4. BOTTOM BAR ---
    let bottom_bar = row![
        Space::with_width(Length::Fill),
        container(
            row![
                text("68SD1D1100D15")
                    .size(10)
                    .font(FONT_BOLD)
                    .style(|_| text::Style { color: Some(colors::COLOR_BG) }),
                Space::with_width(15),
                text("COMBAT COLONIZATION\nDEFENCE PROGRAM")
                    .size(8)
                    .font(FONT_MEDIUM)
                    .style(|_| text::Style { color: Some(colors::COLOR_BG) }),
            ]
            .align_y(Alignment::Center)
        )
        .padding([8, 15])
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(color_accent)),
            ..Default::default()
        })
    ]
    .padding([10, 20]);

    // --- 5. ASSEMBLE LAYOUT WITH EDGE DECORATIONS ---
    let main_dashboard = column![
        top_bar(),
        main_area,
        bottom_bar,
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Far left vertical text
    let left_edge = canvas(VerticalText {
        text: "JHN 102 CKC 151 CC10 S111".to_string(),
        color: color_accent,
        size: 8.0,
        font: FONT_ORBITRON_REGULAR,
    })
    .width(Length::Fixed(20.0))
    .height(Length::Fill);

    // Far right vertical texts (stack)
    let right_edge = column![
        Space::with_height(Length::FillPortion(1)),
        container(
            canvas(VerticalText {
                text: "JHN 102 CKC 151 CC10 S111".to_string(),
                color: color_accent,
                size: 8.0,
                font: FONT_ORBITRON_REGULAR,
            })
            .width(Length::Fill)
            .height(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fixed(200.0)),
        Space::with_height(20),
        container(
            canvas(VerticalText {
                text: "KIROSHI".to_string(),
                color: color_accent,
                size: 10.0,
                font: FONT_BOLD,
            })
            .width(Length::Fill)
            .height(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fixed(100.0)),
        Space::with_height(Length::FillPortion(1)),
    ]
    .width(Length::Fixed(20.0))
    .height(Length::Fill);

    let screen_layout = row![
        left_edge,
        main_dashboard,
        right_edge,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([0, 10]);

    background(
        screen_layout,
        colors::COLOR_BG_VERY_TRANSPARENT,
        colors::COLOR_GLOW,
        true,
    )
}

/// Helper to render the email thread (main message + replies)
fn render_thread<'a, Message: 'static>(email: &'a Email, color_accent: Color) -> Element<'a, Message> {
    let mut col = column![].spacing(20).width(Length::Fill);

    // 1. Root Message (The original email)
    let root_msg = column![
        row![
            text("From: ")
                .font(FONT_BOLD)
                .size(11)
                .style(move |_| text::Style { color: Some(color_accent) }),
            text(&email.sender)
                .font(FONT_MEDIUM)
                .size(11)
                .style(move |_| text::Style { color: Some(color_accent) }),
            Space::with_width(Length::Fill),
            text(&email.timestamp)
                .font(FONT_RAJDHANI_REGULAR)
                .size(10)
                .style(move |_| text::Style { color: Some(Color { a: 0.4, ..color_accent }) }),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill),
        Space::with_height(8),
        parse_markdown(&email.body, color_accent),
    ]
    .width(Length::Fill);
    
    col = col.push(root_msg);

    // 2. Thread Replies (Nested cards)
    for reply in &email.thread {
        let reply_content = column![
            row![
                text("From: ")
                    .font(FONT_BOLD)
                    .size(11)
                    .style(move |_| text::Style { color: Some(color_accent) }),
                text(&reply.sender)
                    .font(FONT_MEDIUM)
                    .size(11)
                    .style(move |_| text::Style { color: Some(color_accent) }),
                Space::with_width(Length::Fill),
                text(&reply.timestamp)
                    .font(FONT_RAJDHANI_REGULAR)
                    .size(10)
                    .style(move |_| text::Style { color: Some(Color { a: 0.4, ..color_accent }) }),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill),
            Space::with_height(8),
            parse_markdown(&reply.body, color_accent),
        ]
        .width(Length::Fill);

        let reply_card = container(reply_content)
            .padding(12)
            .style(move |_| container::Style {
                background: Some(Color { a: 0.02, ..color_accent }.into()),
                border: iced::Border {
                    color: Color { a: 0.1, ..color_accent },
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill);

        let quote_block = row![
            Space::with_width(15),
            reply_card
        ]
        .width(Length::Fill);

        col = col.push(quote_block);
    }

    col.into()
}

// --- Simple Markdown Parser for Cyberpunk Terminal ---

fn parse_markdown<'a, Message: 'static>(markdown_text: &'a str, color_accent: Color) -> Element<'a, Message> {
    let mut col = column![].spacing(12).width(Length::Fill);

    // Split by double newline to get blocks
    let blocks = markdown_text.split("\n\n");

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if is_table(block) {
            col = col.push(render_table(block, color_accent));
        } else if is_list(block) {
            col = col.push(render_list(block, color_accent));
        } else {
            col = col.push(
                text(block)
                    .size(12)
                    .font(FONT_RAJDHANI_REGULAR)
                    .style(move |_| text::Style { color: Some(color_accent) })
            );
        }
    }

    col.into()
}

fn is_table(block: &str) -> bool {
    let lines: Vec<&str> = block.lines().map(|l| l.trim()).collect();
    if lines.len() < 2 {
        return false;
    }
    let second_line = lines[1];
    let has_separator = second_line.contains('|') && 
        second_line.chars().all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace() || c == '+');
    let has_pipes = lines[0].contains('|');
    has_pipes && has_separator
}

fn is_list(block: &str) -> bool {
    block.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with("* ") || t.starts_with("- ") || t.starts_with("• ")
    })
}

fn render_table<'a, Message: 'static>(block: &'a str, color_accent: Color) -> Element<'a, Message> {
    let mut table_col = column![].spacing(0).width(Length::Fill);
    let lines: Vec<&str> = block.lines().map(|l| l.trim()).collect();
    
    if lines.is_empty() {
        return table_col.into();
    }

    // Parse headers (ignore empty outer elements from splitting |col1|col2|)
    let headers: Vec<&str> = lines[0]
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
        
    let col_count = headers.len();
    if col_count == 0 {
        return table_col.into();
    }

    // Render Header Row
    let mut header_row = row![].width(Length::Fill);
    for header in headers {
        header_row = header_row.push(
            container(
                text(header)
                    .size(11)
                    .font(FONT_BOLD)
                    .style(move |_| text::Style { color: Some(color_accent) })
            )
            .padding(6)
            .style(move |_| container::Style {
                background: Some(Color { a: 0.1, ..color_accent }.into()),
                border: iced::Border {
                    color: Color { a: 0.2, ..color_accent },
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .width(Length::FillPortion(1))
        );
    }
    table_col = table_col.push(header_row);

    // Render Data Rows
    for line in lines.iter().skip(2) {
        let cells: Vec<&str> = line
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
            
        if cells.is_empty() {
            continue;
        }

        let mut data_row = row![].width(Length::Fill);
        for i in 0..col_count {
            let cell_text = cells.get(i).cloned().unwrap_or("");
            data_row = data_row.push(
                container(
                    text(cell_text)
                        .size(11)
                        .font(FONT_RAJDHANI_REGULAR)
                        .style(move |_| text::Style { color: Some(color_accent) })
                )
                .padding(6)
                .style(move |_| container::Style {
                    background: Some(Color { a: 0.02, ..color_accent }.into()),
                    border: iced::Border {
                        color: Color { a: 0.1, ..color_accent },
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .width(Length::FillPortion(1))
            );
        }
        table_col = table_col.push(data_row);
    }

    table_col.into()
}

fn render_list<'a, Message: 'static>(block: &'a str, color_accent: Color) -> Element<'a, Message> {
    let mut list_col = column![].spacing(6).width(Length::Fill);
    
    for line in block.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let content = if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
            &trimmed[2..]
        } else if trimmed.starts_with("• ") {
            &trimmed[3..]
        } else {
            trimmed
        };

        list_col = list_col.push(
            row![
                text("•")
                    .size(12)
                    .font(FONT_BOLD)
                    .style(move |_| text::Style { color: Some(color_accent) }),
                Space::with_width(8),
                text(content)
                    .size(12)
                    .font(FONT_RAJDHANI_REGULAR)
                    .style(move |_| text::Style { color: Some(color_accent) }),
            ]
            .align_y(Alignment::Center)
        );
    }

    list_col.into()
}

// --- Custom Cut Button Widget ---

#[derive(Debug, Clone, Copy)]
struct CutButtonBackground {
    bg_color: Color,
    border_color: Color,
    border_width: f32,
    cut_bottom_left: f32,
}

impl<Message> canvas::Program<Message> for CutButtonBackground {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let w = bounds.width;
        let h = bounds.height;
        let cut = self.cut_bottom_left;
        let inset = self.border_width / 2.0;

        let path = canvas::Path::new(|builder| {
            builder.move_to(Point::new(inset, inset));
            builder.line_to(Point::new(w - inset, inset));
            builder.line_to(Point::new(w - inset, h - inset));
            if cut > 0.0 {
                builder.line_to(Point::new(cut + inset, h - inset));
                builder.line_to(Point::new(inset, h - cut - inset));
            } else {
                builder.line_to(Point::new(inset, h - inset));
            }
            builder.close();
        });

        frame.fill(&path, self.bg_color);
        if self.border_width > 0.0 {
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(self.border_color)
                    .with_width(self.border_width),
            );
        }

        vec![frame.into_geometry()]
    }
}

fn cut_button<'a, Message: 'static + Clone>(
    label: &'static str,
    on_press: Option<Message>,
    color_accent: Color,
    is_solid: bool,
    cut_bottom_left: f32,
) -> Element<'a, Message> {
    let bg_color = if is_solid {
        color_accent
    } else {
        Color { a: 0.05, ..color_accent }
    };
    
    let text_color = if is_solid {
        colors::COLOR_BG
    } else {
        color_accent
    };

    let bg_program = CutButtonBackground {
        bg_color,
        border_color: color_accent,
        border_width: 1.0,
        cut_bottom_left,
    };

    let content = stack![
        canvas(bg_program)
            .width(Length::Fill)
            .height(Length::Fill),
        container(
            text(label)
                .font(FONT_BOLD)
                .size(12)
                .style(move |_| text::Style { color: Some(text_color) })
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    if let Some(msg) = on_press {
        mouse_area(content)
            .on_press(msg)
            .into()
    } else {
        content.into()
    }
}
