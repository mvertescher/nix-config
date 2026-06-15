use crate::widgets::bar::bar;
use iced::widget::canvas;
use iced::{mouse, Color, Element, Length};

pub fn top_bar<Message: 'static>(
    color_green_accent: Color,
    window_size: Option<iced::Size>,
) -> Element<'static, Message> {
    bar(
        "ENTROPISM SOFTWAREV1",
        "STORE ACCESS SCREEN",
        "FLAIR TRS 5MMP",
        19.0,
        38.0,
        color_green_accent,
        crate::colors::COLOR_BG,
        false,
        window_size,
    )
}

pub fn bottom_bar<Message: 'static>(
    color_bg: Color,
    color_green_accent: Color,
    window_size: Option<iced::Size>,
    is_dark: bool,
) -> Element<'static, Message> {
    let (_w, h) = match window_size {
        Some(size) => (size.width, size.height),
        None => (1920.0, 1080.0),
    };
    let size_suffix = match window_size {
        Some(size) => format!(" // {:.0}x{:.0}", size.width, size.height),
        None => "".to_string(),
    };
    let build_str = format!("BUILD 6.47.48441.R15{}", size_suffix);

    bar(
        "INTERFACE LOADED",
        "PROVIDED BY NEXUS NETWORK V10.8",
        build_str,
        h - 19.0,
        38.0,
        color_green_accent,
        color_bg,
        !is_dark,
        window_size,
    )
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
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(width),
            );
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
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(width),
            );
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
