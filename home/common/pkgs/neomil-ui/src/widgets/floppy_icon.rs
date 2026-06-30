use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Rectangle, Renderer, Theme, Vector};

#[derive(Debug, Clone, Copy)]
pub struct FloppyIcon {
    pub color: Color,
    pub is_selected: bool,
    pub scale: f32,
}

impl<Message> canvas::Program<Message> for FloppyIcon {
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

        // The design size is 240x220.
        // We scale the frame uniformly based on the width.
        let scale = bounds.width / 240.0;

        frame.with_save(|frame| {
            // Apply the scale
            frame.scale(scale);

            if self.is_selected {
                super::floppy_vector::draw_selected(frame);
            } else {
                super::floppy_vector::draw_unselected(frame);
            }
        });

        vec![frame.into_geometry()]
    }
}

pub fn floppy_icon<'a, Message: 'static>(color: Color, is_selected: bool, scale: f32) -> Element<'a, Message> {
    canvas(FloppyIcon { color, is_selected, scale })
        .width(Length::Fixed(50.0 * scale))
        .height(Length::Fixed(50.0 * scale))
        .into()
}

