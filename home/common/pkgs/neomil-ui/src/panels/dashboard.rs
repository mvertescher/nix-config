use iced::widget::{canvas, column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use crate::colors;
use crate::background::background;
use crate::widgets::{diamond_menu, text_box, DiamondMenuItem, VerticalText};
use crate::top_bar::top_bar;
use crate::fonts::{
    FONT_ORBITRON_REGULAR, FONT_ORBITRON_MEDIUM, FONT_ORBITRON_BOLD,
};

// Aliases

const FONT_MEDIUM: iced::Font = FONT_ORBITRON_MEDIUM;   // Title font
const FONT_BOLD: iced::Font = FONT_ORBITRON_BOLD;       // Title font

/// The main dashboard panel.
pub fn dashboard<'a, Message: 'static + Clone>(
    on_menu_select: impl Fn(usize) -> Message + Clone + 'static,
) -> Element<'a, Message> {


    // --- 4. MAIN CONTENT AREA ---
    // Left Column: Diamond Menu
    let left_col = column![
        container(
            text("COMPUTER SYSTEMS")
                .size(12)
                .font(FONT_MEDIUM)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) })
        )
        .padding([5, 15])
        .style(|_| container::Style {
            border: iced::Border {
                color: colors::COLOR_PRIMARY_RED,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }),
        Space::with_height(20),
        container(
            diamond_menu(
                vec![
                    DiamondMenuItem {
                        label: "VEHICLES".to_string(),
                        subtext: "161-9A".to_string(),
                        on_press: (on_menu_select.clone())(0),
                    },
                    DiamondMenuItem {
                        label: "LOCATIONS".to_string(),
                        subtext: "161-9A".to_string(),
                        on_press: (on_menu_select.clone())(1),
                    },
                    DiamondMenuItem {
                        label: "FACTIONS".to_string(),
                        subtext: "161-9A".to_string(),
                        on_press: (on_menu_select.clone())(2),
                    },
                    DiamondMenuItem {
                        label: "WEAPONS".to_string(),
                        subtext: "161-9A".to_string(),
                        on_press: (on_menu_select.clone())(3),
                    },
                    DiamondMenuItem {
                        label: "PRODUCTS".to_string(),
                        subtext: "161-9A".to_string(),
                        on_press: (on_menu_select.clone())(4),
                    },
                    DiamondMenuItem {
                        label: "CORPORATIONS".to_string(),
                        subtext: "161-9A".to_string(),
                        on_press: (on_menu_select.clone())(5),
                    },
                ],
                colors::COLOR_PRIMARY_RED,
                colors::COLOR_BG,
            )
        )
        .width(Length::Fill)
        .height(Length::Fill)
    ]
    .width(Length::FillPortion(3))
    .align_x(Alignment::Center);

    // Right Column: Info Panel (using the new text_box widget)
    let right_col = column![
        container(
            text("DESCRIPTION")
                .size(12)
                .font(FONT_MEDIUM)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) })
        )
        .padding([5, 15])
        .style(|_| container::Style {
            border: iced::Border {
                color: colors::COLOR_PRIMARY_RED,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }),
        Space::with_height(20),
        text_box(
            "GO HOME",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\nQuis ipsum suspendisse ultrices gravida. Risus commodo viverra maecenas accumsan lacus vel facilisis.",
            &["PETROCHEM", "BETTERLIFE TEC"],
            "M",
            "PRECISION LIQUID",
            "POLYMER MUSCLE",
            colors::COLOR_PRIMARY_RED,
        )
    ]
    .width(Length::FillPortion(2))
    .height(Length::Fill)
    .align_x(Alignment::Center);

    let main_area = row![
        left_col,
        Space::with_width(40),
        right_col,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(40);

    // --- 5. BOTTOM BAR ---
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
        .style(|_| container::Style {
            background: Some(iced::Background::Color(colors::COLOR_PRIMARY_RED)),
            ..Default::default()
        })
    ]
    .padding([10, 20]);

    // --- 6. ASSEMBLE LAYOUT WITH EDGE DECORATIONS ---
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
        color: colors::COLOR_PRIMARY_RED,
        size: 8.0,
        font: FONT_ORBITRON_REGULAR,
    })
    .width(Length::Fixed(20.0))
    .height(Length::Fill);

    // Far right vertical texts (stacked)
    let right_edge = column![
        Space::with_height(Length::FillPortion(1)),
        container(
            canvas(VerticalText {
                text: "JHN 102 CKC 151 CC10 S111".to_string(),
                color: colors::COLOR_PRIMARY_RED,
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
                color: colors::COLOR_PRIMARY_RED,
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

// --- HELPERS ---




