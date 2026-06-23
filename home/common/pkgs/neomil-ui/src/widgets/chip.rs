use iced::widget::{canvas, container, stack};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub struct ChipBackground {
    pub bg_color: Color,
    pub border_color: Color,
    pub cut_size: f32,
    pub border_width: f32,
}

impl<Message> canvas::Program<Message> for ChipBackground {
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
        let cut = self.cut_size;

        if w > cut && h > cut {
            // Draw a polygon with a cut bottom-right corner:
            // (0,0) -> (w,0) -> (w, h-cut) -> (w-cut, h) -> (0, h) -> (0,0)
            let path = canvas::Path::new(|builder| {
                builder.move_to(Point::new(0.0, 0.0));
                builder.line_to(Point::new(w, 0.0));
                builder.line_to(Point::new(w, h - cut));
                builder.line_to(Point::new(w - cut, h));
                builder.line_to(Point::new(0.0, h));
                builder.close();
            });

            // Fill background
            frame.fill(&path, self.bg_color);

            // Stroke border
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(self.border_color)
                    .with_width(self.border_width),
            );

            // Draw a small decorative accent line on the top border (e.g. from 10 to 40)
            let accent_path = canvas::Path::new(|builder| {
                builder.move_to(Point::new(10.0, 1.0));
                builder.line_to(Point::new(40.0, 1.0));
            });
            frame.stroke(
                &accent_path,
                canvas::Stroke::default()
                    .with_color(self.border_color)
                    .with_width(3.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// A container with a custom Cyberpunk-themed cut corner ("chip type 1").
pub fn chip_type_1<'a, Message: 'static>(
    content: impl Into<Element<'a, Message>>,
    color_accent: Color,
    color_bg: Color,
) -> Element<'a, Message> {
    let bg_program = ChipBackground {
        bg_color: Color { a: 0.15, ..color_bg },
        border_color: color_accent,
        cut_size: 15.0,
        border_width: 1.5,
    };

    stack![
        canvas(bg_program)
            .width(Length::Fill)
            .height(Length::Fill),
        container(content.into())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(iced::Padding {
                top: 15.0,
                bottom: 15.0,
                left: 15.0,
                right: 25.0,
            })
    ]
    .into()
}
