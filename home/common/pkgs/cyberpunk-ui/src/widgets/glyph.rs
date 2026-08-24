//! The compliance glyphs a maximalist era stamps on its bands and rows.
//!
//! Three shapes, all measured off `docs/kitsch/target-components.svg`:
//! a dotted matrix (`<pattern id="dotmat">`, a 2px square on a 4px
//! pitch), a hollow square, and a hollow triangle. They appear twice on
//! the kitsch store card -- as a 10px strip at the head of the shelf
//! band, and as a 24px matrix block leading the EMPTY SOCKET row -- and
//! nowhere else, which is why an era opts in through
//! [`crate::style::Style::glyphs`] rather than the widget asking which
//! era it is in.

use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Glyph {
    /// A dotted matrix. Not a texture: at these sizes it is nine or
    /// forty-nine little squares, so it is cheaper to lay them out than
    /// to tile a pattern.
    Matrix,
    Square,
    Triangle,
}

#[derive(Debug, Clone, Copy)]
struct GlyphShape {
    pub glyph: Glyph,
    pub color: Color,
    pub stroke: f32,
}

/// Dot size and pitch of [`Glyph::Matrix`], from the reference pattern.
const DOT: f32 = 2.0;
const PITCH: f32 = 4.0;

impl<Message> canvas::Program<Message> for GlyphShape {
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

        match self.glyph {
            Glyph::Matrix => {
                let mut y = 0.0;
                while y + DOT <= h {
                    let mut x = 0.0;
                    while x + DOT <= w {
                        frame.fill_rectangle(
                            Point::new(x, y),
                            Size::new(DOT, DOT),
                            self.color,
                        );
                        x += PITCH;
                    }
                    y += PITCH;
                }
            }
            Glyph::Square => {
                let i = self.stroke / 2.0;
                frame.stroke_rectangle(
                    Point::new(i, i),
                    Size::new(w - i * 2.0, h - i * 2.0),
                    canvas::Stroke::default()
                        .with_color(self.color)
                        .with_width(self.stroke),
                );
            }
            Glyph::Triangle => {
                let i = self.stroke / 2.0;
                let path = canvas::Path::new(|b| {
                    b.move_to(Point::new(i, h - i));
                    b.line_to(Point::new(w / 2.0, i));
                    b.line_to(Point::new(w - i, h - i));
                    b.close();
                });
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(self.color)
                        .with_width(self.stroke),
                );
            }
        }

        vec![frame.into_geometry()]
    }
}

/// One glyph, `size` square.
pub fn glyph<'a, Message: 'static>(
    glyph: Glyph,
    color: Color,
    stroke: f32,
    size: f32,
) -> Element<'a, Message> {
    canvas(GlyphShape {
        glyph,
        color,
        stroke,
    })
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .into()
}
