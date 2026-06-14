use iced::widget::{container, row, Space, canvas};
use iced::{Background, Border, Color, Element, Length, mouse};
use crate::glow::{glowing_text, get_radial_offsets};

pub fn top_bar<Message: 'static>(
    color_green_accent: Color,
    window_size: Option<iced::Size>,
) -> Element<'static, Message> {
    let (w, h) = match window_size {
        Some(size) => (size.width, size.height),
        None => (1920.0, 1080.0),
    };

    let divider = move || {
        container(Space::new(1.0, Length::Fill)).style(move |_| container::Style {
            background: Some(Background::Color(color_green_accent)),
            ..Default::default()
        })
    };

    let gap_divider = || {
        row![
            Space::new(12.0, 0.0),
            divider(),
            Space::new(12.0, 0.0),
        ]
    };

    // Calculate dynamic offsets
    let (off_x1, off_y1) = get_radial_offsets(w * 0.33, 15.0, w, h);
    let (off_x2, off_y2) = get_radial_offsets(w * 0.83, 15.0, w, h);
    let (off_x3, off_y3) = get_radial_offsets(w, 15.0, w, h);

    container(
        row![
            container(glowing_text("ENTROPISM SOFTWAREV1", 16, color_green_accent, off_x1, off_y1))
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
            container(glowing_text("STORE ACCESS SCREEN", 16, color_green_accent, off_x2, off_y2))
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
            container(glowing_text("FLAIR TRS 5MMP", 16, color_green_accent, off_x3, off_y3))
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
        .height(Length::Fixed(38.0))
    )
    .width(Length::Fill)
    .style(move |_| container::Style {
        border: Border {
            color: color_green_accent,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn bottom_bar<Message: 'static>(
    color_bg: Color,
    color_green_accent: Color,
    window_size: Option<iced::Size>,
    is_dark: bool,
) -> Element<'static, Message> {
    let size_suffix = match window_size {
        Some(size) => format!(" // {:.0}x{:.0}", size.width, size.height),
        None => "".to_string(),
    };
    let build_str = format!("BUILD 6.47.48441.R15{}", size_suffix);

    let (bg_color, text_color, has_border) = if is_dark {
        (Some(color_bg), color_green_accent, true)
    } else {
        (Some(color_green_accent), color_bg, false)
    };

    let divider = move || {
        container(Space::new(1.0, Length::Fill)).style(move |_| container::Style {
            background: Some(Background::Color(text_color)),
            ..Default::default()
        })
    };

    let gap_divider = || {
        row![
            Space::new(12.0, 0.0),
            divider(),
            Space::new(12.0, 0.0),
        ]
    };

    let (w, h) = match window_size {
        Some(size) => (size.width, size.height),
        None => (1920.0, 1080.0),
    };

    let (off_x1, off_y1) = get_radial_offsets(w * 0.33, h - 15.0, w, h);
    let (off_x2, off_y2) = get_radial_offsets(w * 0.83, h - 15.0, w, h);
    let (off_x3, off_y3) = get_radial_offsets(w, h - 15.0, w, h);

    container(
        row![
            container(glowing_text("INTERFACE LOADED", 16, text_color, off_x1, off_y1))
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
            container(glowing_text("PROVIDED BY NEXUS NETWORK V10.8", 16, text_color, off_x2, off_y2))
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
            container(glowing_text(build_str, 16, text_color, off_x3, off_y3))
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
        .height(Length::Fixed(38.0))
    )
    .width(Length::Fill)
    .style(move |_| container::Style {
        background: bg_color.map(Background::Color),
        border: if has_border {
            Border {
                color: color_green_accent,
                width: 1.0,
                radius: 0.0.into(),
            }
        } else {
            Border::default()
        },
        ..Default::default()
    })
    .into()
}

#[derive(Debug)]
pub struct DebugGrid;

impl<Message> canvas::Program<Message> for DebugGrid {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let grid_color = Color::from_rgba(0.2, 0.5, 0.2, 0.15); // faint green
        let major_color = Color::from_rgba(0.8, 0.2, 0.2, 0.25); // faint red

        // Draw horizontal grid lines every 10px
        for y in (0..bounds.height as u32).step_by(10) {
            let is_major = y % 50 == 0;
            let color = if is_major { major_color } else { grid_color };
            let width = if is_major { 1.0 } else { 0.5 };

            let path = canvas::Path::line(
                iced::Point::new(0.0, y as f32),
                iced::Point::new(bounds.width, y as f32),
            );
            frame.stroke(&path, canvas::Stroke::default().with_color(color).with_width(width));
        }

        // Draw vertical grid lines every 10px
        for x in (0..bounds.width as u32).step_by(10) {
            let is_major = x % 50 == 0;
            let color = if is_major { major_color } else { grid_color };
            let width = if is_major { 1.0 } else { 0.5 };

            let path = canvas::Path::line(
                iced::Point::new(x as f32, 0.0),
                iced::Point::new(x as f32, bounds.height),
            );
            frame.stroke(&path, canvas::Stroke::default().with_color(color).with_width(width));
        }

        vec![frame.into_geometry()]
    }
}

pub fn debug_grid<'a, Message: 'static>() -> Element<'a, Message> {
    canvas(DebugGrid)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
