use crate::glow::{get_radial_offsets, glowing_bar, glowing_text};
use iced::widget::{container, row, text, Space};
use iced::{Background, Color, Element, Length};
use std::borrow::Cow;

pub fn bar<'a, Message: 'static>(
    left_text: impl Into<Cow<'a, str>>,
    middle_text: impl Into<Cow<'a, str>>,
    right_text: impl Into<Cow<'a, str>>,
    bar_y: f32,
    bar_height: f32,
    color_green_accent: Color,
    color_bg: Color,
    invert_colors: bool,
    window_size: Option<iced::Size>,
) -> Element<'a, Message> {
    let (w, h) = match window_size {
        Some(size) => (size.width, size.height),
        None => (1920.0, 1080.0),
    };

    let left_cow = left_text.into();
    let middle_cow = middle_text.into();
    let right_cow = right_text.into();

    let (bg_color, text_color, has_border) = if invert_colors {
        (Some(color_green_accent), color_bg, false)
    } else {
        (Some(color_bg), color_green_accent, true)
    };

    let divider = move || {
        container(Space::new(1.0, Length::Fill)).style(move |_| container::Style {
            background: Some(Background::Color(text_color)),
            ..Default::default()
        })
    };

    let gap_divider = || row![Space::new(12.0, 0.0), divider(), Space::new(12.0, 0.0),];

    // Calculate dynamic offsets for the text segments
    let (off_x1, off_y1) = get_radial_offsets(w * 0.33, bar_y, w, h);
    let (off_x2, off_y2) = get_radial_offsets(w * 0.83, bar_y, w, h);
    let (off_x3, off_y3) = get_radial_offsets(w, bar_y, w, h);

    // Build segment widgets (using glow only in dark/non-inverted mode)
    let left_widget: Element<'a, Message> = if invert_colors {
        text(left_cow).size(16).color(text_color).into()
    } else {
        glowing_text(left_cow, 16, text_color, off_x1, off_y1)
    };

    let middle_widget: Element<'a, Message> = if invert_colors {
        text(middle_cow).size(16).color(text_color).into()
    } else {
        glowing_text(middle_cow, 16, text_color, off_x2, off_y2)
    };

    let right_widget: Element<'a, Message> = if invert_colors {
        text(right_cow).size(16).color(text_color).into()
    } else {
        glowing_text(right_cow, 16, text_color, off_x3, off_y3)
    };

    let row_content = row![
        container(left_widget)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_y(iced::Alignment::Center)
            .padding(iced::Padding {
                left: 15.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            }),
        gap_divider(),
        container(middle_widget)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .align_y(iced::Alignment::Center)
            .padding(iced::Padding {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            }),
        gap_divider(),
        container(right_widget)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_y(iced::Alignment::Center)
            .padding(iced::Padding {
                left: 0.0,
                right: 15.0,
                top: 0.0,
                bottom: 0.0,
            }),
    ]
    .align_y(iced::Alignment::Center)
    .height(Length::Fixed(bar_height));

    if has_border {
        glowing_bar(
            row_content,
            bar_y,
            bar_height,
            color_green_accent,
            bg_color,
            window_size,
        )
    } else {
        container(row_content)
            .width(Length::Fill)
            .height(Length::Fixed(bar_height))
            .style(move |_| container::Style {
                background: bg_color.map(Background::Color),
                ..Default::default()
            })
            .into()
    }
}
