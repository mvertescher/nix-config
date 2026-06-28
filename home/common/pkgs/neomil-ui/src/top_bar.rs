use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use crate::colors;
use crate::widgets::{level_badge, LevelBadgeStyle};
use crate::fonts::{
    FONT_ORBITRON_REGULAR, FONT_ORBITRON_MEDIUM, FONT_ORBITRON_BOLD,
};

// Aliases for local use
const FONT_MEDIUM: iced::Font = FONT_ORBITRON_MEDIUM;
const FONT_BOLD: iced::Font = FONT_ORBITRON_BOLD;

/// The top bar component, including the horizontal divider line.
pub fn top_bar<'a, Message: 'static + Clone>() -> Element<'a, Message> {
    // --- CUSTOMER LEVEL (Top-Left) ---
    let customer_level = column![
        text("CUSTOMER")
            .size(10)
            .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
        level_badge(
            column![
                text("LEVEL")
                    .size(8)
                    .font(FONT_ORBITRON_REGULAR)
                    .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
                text("T1")
                    .size(14)
                    .font(FONT_ORBITRON_BOLD)
                    .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) })
            ]
            .spacing(2)
            .align_x(Alignment::Center),
            colors::COLOR_PRIMARY_RED,
            LevelBadgeStyle::Outline,
        )
    ]
    .spacing(5);

    // --- LOGO (Middle) ---
    let logo = column![
        text("#NC488402")
            .size(10)
            .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
        text("next")
            .size(36)
            .font(FONT_BOLD)
            .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
        text("TECHNOLOGY")
            .size(12)
            .font(FONT_MEDIUM)
            .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
        container(
            text("JHN 102 CKC 151 CC10 S111")
                .size(8)
                .font(FONT_ORBITRON_REGULAR)
                .style(|_| text::Style { color: Some(colors::COLOR_BG) })
        )
        .padding([2, 8])
        .style(|_| container::Style {
            background: Some(iced::Background::Color(colors::COLOR_PRIMARY_RED)),
            ..Default::default()
        })
    ]
    .align_x(Alignment::Center);

    // --- SECURITY LEVELS (Top-Right) ---
    let security_levels = column![
        text("SECURITY LEVEL")
            .size(10)
            .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
        row![
            make_sec_level("T1", true),
            make_sec_level("T2", false),
            make_sec_level("T3", false),
            make_sec_level("T4", false),
        ]
        .spacing(5)
    ]
    .spacing(5);

    // --- ASSEMBLE TOP BAR ROW ---
    let top_bar_row = row![
        customer_level,
        Space::with_width(Length::Fill),
        logo,
        Space::with_width(Length::Fill),
        security_levels,
    ]
    .align_y(Alignment::Center)
    .padding([10, 20]);

    // --- RED DIVIDER LINE ---
    let red_line = container(Space::new(Length::Fill, 1.5))
        .style(|_| container::Style {
            background: Some(iced::Background::Color(colors::COLOR_PRIMARY_RED)),
            ..Default::default()
        });

    // --- COMBINE ROW AND LINE ---
    column![
        top_bar_row,
        red_line,
    ]
    .spacing(0)
    .into()
}

// --- HELPERS ---

fn make_sec_level<'a, Message: 'static>(level: &'a str, active: bool) -> Element<'a, Message> {
    let title = text("LEVEL")
        .size(8)
        .font(FONT_ORBITRON_REGULAR)
        .style(move |_| text::Style {
            color: Some(if active { colors::COLOR_BG } else { colors::COLOR_PRIMARY_RED }),
        });

    let val = text(level)
        .size(14)
        .font(FONT_ORBITRON_BOLD)
        .style(move |_| text::Style {
            color: Some(if active { colors::COLOR_BG } else { colors::COLOR_PRIMARY_RED }),
        });

    let content = column![title, val]
        .spacing(2)
        .align_x(Alignment::Center);

    level_badge(
        content,
        colors::COLOR_PRIMARY_RED,
        if active {
            LevelBadgeStyle::Solid
        } else {
            LevelBadgeStyle::Translucent
        },
    )
}
