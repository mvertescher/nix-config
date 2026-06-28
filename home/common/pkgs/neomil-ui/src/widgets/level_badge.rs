use iced::widget::{canvas, container, stack};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub struct LevelBadgeBackground {
    pub bg_color: Color,
    pub border_color: Color,
    pub cut_size: f32,
    pub border_width: f32,
}

impl<Message> canvas::Program<Message> for LevelBadgeBackground {
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
            let inset = if self.border_width > 0.0 {
                self.border_width / 2.0 + 1.0
            } else {
                0.0
            };
            let path = canvas::Path::new(|builder| {
                builder.move_to(Point::new(inset, inset));
                builder.line_to(Point::new(w - inset, inset));
                builder.line_to(Point::new(w - inset, h - inset));
                builder.line_to(Point::new(cut + inset, h - inset));
                builder.line_to(Point::new(inset, h - cut - inset));
                builder.close();
            });

            // Fill background
            frame.fill(&path, self.bg_color);

            // Stroke border (only if width > 0)
            if self.border_width > 0.0 {
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(self.border_color)
                        .with_width(self.border_width),
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelBadgeStyle {
    Solid,
    Outline,
    Translucent,
}

/// A badge with a chamfered bottom-left corner, used for level indicators.
pub fn level_badge<'a, Message: 'static>(
    content: impl Into<Element<'a, Message>>,
    color_accent: Color,
    style: LevelBadgeStyle,
) -> Element<'a, Message> {
    let (bg_color, border_width) = match style {
        LevelBadgeStyle::Solid => (color_accent, 0.0),
        LevelBadgeStyle::Outline => (Color { a: 0.15, ..color_accent }, 1.0),
        LevelBadgeStyle::Translucent => (Color { a: 0.15, ..color_accent }, 0.0),
    };

    let bg_program = LevelBadgeBackground {
        bg_color,
        border_color: color_accent,
        cut_size: 10.0,
        border_width,
    };

    stack![
        canvas(bg_program)
            .width(Length::Fill)
            .height(Length::Fill),
        container(content.into())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    ]
    .width(Length::Fixed(60.0))
    .height(Length::Fixed(60.0))
    .into()
}
