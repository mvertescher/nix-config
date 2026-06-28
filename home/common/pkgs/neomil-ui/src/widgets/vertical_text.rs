use iced::widget::canvas;
use iced::{Color, Point, Rectangle, Renderer, Theme, Vector, mouse};

#[derive(Debug, Clone)]
pub struct VerticalText {
    pub text: String,
    pub color: Color,
    pub size: f32,
    pub font: iced::Font,
}

impl<Message> canvas::Program<Message> for VerticalText {
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

        frame.with_save(|frame| {
            // Translate to center of canvas
            frame.translate(Vector::new(bounds.width / 2.0, bounds.height / 2.0));
            // Rotate by -90 degrees (counter-clockwise)
            frame.rotate(-std::f32::consts::FRAC_PI_2);

            let txt = canvas::Text {
                content: self.text.clone(),
                position: Point::ORIGIN,
                color: self.color,
                size: self.size.into(),
                font: self.font,
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            };
            frame.fill_text(txt);
        });

        vec![frame.into_geometry()]
    }
}
