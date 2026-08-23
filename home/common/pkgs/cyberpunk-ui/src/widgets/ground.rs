//! Page background.
//!
//! Two of the four eras vignette every screen with a coloured bloom, and
//! it is not decoration that can be skipped: the kitsch and neokitsch
//! references have no flat-background screens at all. The other two are
//! flat, so this collapses to a single filled rectangle for them.

use crate::style::{Ground, Style};
use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub struct Background {
    pub base: Color,
    pub ground: Ground,
    pub bloom: Color,
}

impl Background {
    pub fn new(style: &Style) -> Self {
        Background {
            base: style.palette.bg,
            ground: style.ground,
            bloom: style.palette.bloom,
        }
    }
}

impl<Message> canvas::Program<Message> for Background {
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

        frame.fill(
            &canvas::Path::rectangle(Point::ORIGIN, bounds.size()),
            self.base,
        );

        if let Ground::Bloom { x, y, radius } = self.ground {
            // Stacked translucent discs rather than a radial gradient:
            // the falloff is smooth enough at these radii and it keeps
            // the crate off renderer-specific gradient support.
            let cx = x * w;
            let cy = y * h;
            let r_max = radius * w.max(h);
            let steps = 26;
            for i in (0..steps).rev() {
                let t = (i + 1) as f32 / steps as f32;
                let r = r_max * t;
                // Quadratic falloff, so the core stays hot and the tail
                // reaches well into the page like the references.
                let a = 0.055 * (1.0 - t) * (1.0 - t) + 0.006;
                frame.fill(
                    &canvas::Path::circle(Point::new(cx, cy), r),
                    Color { a, ..self.bloom },
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

/// A full-bleed background for the current era.
pub fn ground<'a, Message: 'static>(style: &Style) -> Element<'a, Message> {
    canvas(Background::new(style))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
