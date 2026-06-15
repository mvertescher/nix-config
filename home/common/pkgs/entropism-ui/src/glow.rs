use iced::widget::{container, stack, text, Space};
use iced::{Background, Border, Color, Element, Length};
use std::borrow::Cow;

pub fn get_radial_offsets(x: f32, y: f32, window_width: f32, window_height: f32) -> (f32, f32) {
    let center_x = window_width / 2.0;
    let center_y = window_height / 2.0;

    // Returns normalized vector components in range [-1.0, 1.0]
    ((x - center_x) / center_x, (y - center_y) / center_y)
}

pub fn radiate_element<'a, Message: 'static>(
    color: Color,
    offset_x: f32,
    offset_y: f32,
    make_element: impl Fn(Color) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let ghost_color_1 = Color { a: 0.08, ..color };
    let ghost_color_2 = Color { a: 0.02, ..color };

    // Max offset is 8px, so we use a base padding of 8px.
    let get_padding = |shift_x: f32, shift_y: f32| iced::Padding {
        left: 8.0 + shift_x,
        right: 8.0 - shift_x,
        top: 8.0 + shift_y,
        bottom: 8.0 - shift_y,
    };

    stack![
        // Sharp Foreground (no shift, full opacity)
        container(make_element(color)).padding(get_padding(0.0, 0.0)),
        // Ghost 1 (medium shift, medium opacity)
        container(make_element(ghost_color_1)).padding(get_padding(offset_x * 4.0, offset_y * 4.0)),
        // Ghost 2 (outermost shift, lowest opacity)
        container(make_element(ghost_color_2)).padding(get_padding(offset_x * 8.0, offset_y * 8.0)),
    ]
    .into()
}

pub fn glowing_text<'a, Message: 'static>(
    content: impl Into<Cow<'a, str>>,
    size: u16,
    color: Color,
    offset_x: f32,
    offset_y: f32,
) -> Element<'a, Message> {
    let content_cow = content.into();
    radiate_element(color, offset_x, offset_y, move |c| {
        text(content_cow.clone())
            .size(size)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .style(move |_| text::Style { color: Some(c) })
            .into()
    })
}

pub fn glowing_border_container<'a, Message: 'static>(
    content: impl Into<Element<'a, Message>>,
    border_width: f32,
    color: Color,
    offset_x: f32,
    offset_y: f32,
) -> Element<'a, Message> {
    let border_glow = radiate_element(color, offset_x, offset_y, move |c| {
        container(Space::new(Length::Fill, Length::Fill))
            .style(move |_| container::Style {
                border: Border {
                    color: c,
                    width: border_width,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    });

    stack![
        border_glow,
        container(content.into())
            .padding(8.0) // Shift padding matching base radiate offsets
            .width(Length::Fill)
            .height(Length::Fill)
    ]
    .into()
}

pub fn glowing_bar<'a, Message: 'static>(
    content: impl Into<Element<'a, Message>>,
    bar_y: f32,
    bar_height: f32,
    color: Color,
    bg_color: Option<Color>,
    window_size: Option<iced::Size>,
) -> Element<'a, Message> {
    let (w, h) = match window_size {
        Some(size) => (size.width, size.height),
        None => (1920.0, 1080.0),
    };

    // Calculate dynamic offset from the true center of the window
    let (bar_off_x, bar_off_y) = get_radial_offsets(w / 2.0, bar_y, w, h);

    let make_border = move |c| {
        container(Space::new(Length::Fill, Length::Fixed(bar_height)))
            .style(move |_| container::Style {
                border: Border {
                    color: c,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    };

    let border_glow = radiate_element(color, bar_off_x, bar_off_y, make_border);

    stack![
        // 1. Solid Background (drawn underneath)
        container(
            container(Space::new(Length::Fill, Length::Fill)).style(move |_| container::Style {
                background: bg_color.map(Background::Color),
                ..Default::default()
            })
        )
        .padding(8.0)
        .width(Length::Fill)
        .height(Length::Fixed(bar_height + 16.0)),
        // 2. Border Glow (drawn on top of background)
        border_glow,
        // 3. Content (drawn on top of border glow)
        container(content.into())
            .padding(8.0)
            .width(Length::Fill)
            .height(Length::Fixed(bar_height + 16.0))
    ]
    .into()
}
