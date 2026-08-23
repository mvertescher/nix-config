//! The product's line-art, as every store card carries it.
//!
//! All three store targets draw the same pistol outline at the same
//! proportions and differ only in scale and stroke weight -- entropism
//! `M560 424 h130 l16 10 h26 v16 h-32 l-10 12 h-76 l-9 -10 h-45 Z`,
//! kitsch and neokitsch the same walk with their own numbers. So this
//! is one path in its own coordinate space, fitted to whatever box the
//! card hands it, rather than three sampled outlines.
//!
//! It is line-work in the card's ink, which means it needs no role of
//! its own: `fg` on an outlined card, `on_select` on a filled one, the
//! same rule every other mark on the card follows.

use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme, Vector};

/// The art's own coordinate box, taken from the entropism target: the
/// body is 172x38 and the grip hangs to 56.
const ART_W: f32 = 172.0;
const ART_H: f32 = 56.0;

#[derive(Debug, Clone, Copy)]
pub struct Silhouette {
    pub color: Color,
    pub stroke: f32,
}

impl<Message> canvas::Program<Message> for Silhouette {
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
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }

        // Fit rather than stretch: a card narrower than the art scales
        // it down whole, and a wider one leaves it centred at full
        // height. A stretched gun reads immediately as wrong.
        let pad = self.stroke;
        let scale = ((w - pad * 2.0) / ART_W).min((h - pad * 2.0) / ART_H);
        if scale <= 0.0 {
            return vec![frame.into_geometry()];
        }
        frame.translate(Vector::new(
            (w - ART_W * scale) / 2.0,
            (h - ART_H * scale) / 2.0,
        ));
        frame.scale(scale);

        let body = canvas::Path::new(|b| {
            b.move_to(Point::new(0.0, 0.0));
            b.line_to(Point::new(130.0, 0.0));
            b.line_to(Point::new(146.0, 10.0));
            b.line_to(Point::new(172.0, 10.0));
            b.line_to(Point::new(172.0, 26.0));
            b.line_to(Point::new(140.0, 26.0));
            b.line_to(Point::new(130.0, 38.0));
            b.line_to(Point::new(54.0, 38.0));
            b.line_to(Point::new(45.0, 28.0));
            b.line_to(Point::new(0.0, 28.0));
            b.close();
        });
        let grip = canvas::Path::new(|b| {
            b.move_to(Point::new(36.0, 38.0));
            b.line_to(Point::new(36.0, 56.0));
            b.line_to(Point::new(52.0, 56.0));
            b.line_to(Point::new(52.0, 38.0));
        });

        // The stroke is in art units so it scales with the drawing;
        // dividing it out keeps the line the weight the caller asked
        // for whatever the card's width turned out to be.
        let stroke = canvas::Stroke::default()
            .with_color(self.color)
            .with_width(self.stroke / scale);
        frame.stroke(&body, stroke);
        frame.stroke(&grip, stroke);

        vec![frame.into_geometry()]
    }
}

/// The product art, `height` tall and as wide as it is offered.
pub fn silhouette<'a, Message: 'static>(
    color: Color,
    stroke: f32,
    height: f32,
) -> Element<'a, Message> {
    canvas(Silhouette { color, stroke })
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .into()
}
