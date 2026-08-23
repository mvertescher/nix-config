//! Boxed footnote markers.
//!
//! [A], [B], [C] beside two lines of tiny print. Present in every era's
//! references, always in the same shape, which makes it a good check
//! that the shared vocabulary is really shared: nothing here branches.

use super::surface::{outline, Corners};
use super::text;
use crate::style::{Corner, Style};
use iced::widget::{canvas, column, container, row, stack, Space};
use iced::{mouse, Color, Element, Length, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
struct Box {
    color: Color,
    width: f32,
}

impl<Message> canvas::Program<Message> for Box {
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
        let path = outline(
            Corner::Square,
            Corners::NONE,
            bounds.width,
            bounds.height,
        );
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(self.color)
                .with_width(self.width),
        );
        vec![frame.into_geometry()]
    }
}

pub fn marker<'a, Message: 'static>(
    style: &Style,
    letter: &'a str,
    lines: &[&'a str],
) -> Element<'a, Message> {
    let boxed = container(stack![
        canvas(Box {
            color: style.palette.fg,
            width: style.metrics.stroke,
        })
        .width(Length::Fill)
        .height(Length::Fill),
        container(text::body(style, letter))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ])
    .width(Length::Fixed(24.0))
    .height(Length::Fixed(24.0));

    let mut notes = column![].spacing(2);
    for line in lines {
        notes = notes.push(text::caption(style, *line));
    }

    row![boxed, Space::new(10.0, 0.0), notes]
        .align_y(iced::Alignment::Center)
        .into()
}
