use iced::widget::{canvas, column, container, row, text, Space, scrollable, button};
use iced::{Alignment, Color, Element, Length};
use crate::colors;
use crate::background::background;
use crate::widgets::{floppy_icon, message_card, text_box, VerticalText};
use crate::top_bar::top_bar;
use crate::fonts::{
    FONT_ORBITRON_REGULAR, FONT_ORBITRON_MEDIUM, FONT_ORBITRON_BOLD,
};

const FONT_MEDIUM: iced::Font = FONT_ORBITRON_MEDIUM;
const FONT_BOLD: iced::Font = FONT_ORBITRON_BOLD;

/// Data structure representing an email/message.
#[derive(Debug, Clone)]
pub struct Email {
    pub id: usize,
    pub title: String,
    pub sender: String,
    pub body: String,
    pub is_new: bool,
}

/// The mail panel view.
/// Assembles the message list on the left and the selected message detail on the right.
pub fn mail_panel<'a, Message: 'static + Clone>(
    emails: &'a [Email],
    selected_id: Option<usize>,
    on_select: impl Fn(usize) -> Message + Clone + 'static,
    on_delete: impl Fn(usize) -> Message + Clone + 'static,
    scrollable_id: iced::widget::scrollable::Id,
    color_accent: Color,
) -> Element<'a, Message> {
    
    // --- 1. LEFT COLUMN: Message List ---
    let left_header = row![
        container(
            text("MESSAGES")
                .size(12)
                .font(FONT_MEDIUM)
                .style(move |_| text::Style { color: Some(color_accent) })
        )
        .padding([5, 15])
        .style(move |_| container::Style {
            border: iced::Border {
                color: color_accent,
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
            floppy_icon(color_accent, is_selected, 1.0),
            Space::with_width(10),
            message_card(
                &email.title,
                &email.sender,
                email.is_new,
                is_selected,
                (on_select.clone())(email.id),
                color_accent,
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
            .id(scrollable_id)
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
                        background: Some(Color { a: 0.02, ..color_accent }.into()),
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        scroller: Scroller {
                            color: Color { a: 0.3, ..color_accent },
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
        let delete_btn = button(
            text("DELETE")
                .font(FONT_BOLD)
                .size(12)
                .style(|_| text::Style { color: Some(colors::COLOR_BG) })
        )
        .padding([10, 20])
        .style(move |_, _| button::Style {
            background: Some(iced::Background::Color(color_accent)),
            border: iced::Border {
                color: color_accent,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .on_press((on_delete.clone())(email.id));

        let outline_btn = |label: &'static str| {
            button(
                text(label)
                    .font(FONT_BOLD)
                    .size(12)
                    .style(move |_| text::Style { color: Some(color_accent) })
            )
            .padding([10, 20])
            .style(move |_, _| button::Style {
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                border: iced::Border {
                    color: color_accent,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
        };

        let footer_buttons = row![
            delete_btn,
            outline_btn("REPLY"),
            outline_btn("ARCHIVE"),
            outline_btn("CLOSE"),
        ]
        .spacing(10)
        .width(Length::Fill);

        column![
            container(
                text("CONTENT")
                    .size(12)
                    .font(FONT_MEDIUM)
                    .style(move |_| text::Style { color: Some(color_accent) })
            )
            .padding([5, 15])
            .style(move |_| container::Style {
                border: iced::Border {
                    color: color_accent,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
            Space::with_height(20),
            text_box(
                &email.title,
                &email.body,
                &["PETROCHEM", "BETTERLIFE TEC"],
                "", // No logo
                "",
                "",
                None, // No footer inside the box
                color_accent,
            ),
            Space::with_height(15),
            footer_buttons, // Buttons below the box
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
                    .style(move |_| text::Style { color: Some(color_accent) })
            )
            .padding([5, 15])
            .style(move |_| container::Style {
                border: iced::Border {
                    color: color_accent,
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
                    .style(move |_| text::Style { color: Some(Color { a: 0.3, ..color_accent }) })
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| container::Style {
                border: iced::Border {
                    color: Color { a: 0.1, ..color_accent },
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
