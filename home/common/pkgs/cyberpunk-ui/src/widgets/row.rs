//! List rows.
//!
//! The mail list is the second screen all four eras draw, and like the
//! product card it is one widget: two lines of text, a glyph, and the
//! era's selection idiom for the chosen row. Entropism squares it,
//! kitsch rounds it, neokitsch fills it with veneer -- all of which is
//! [`Surface`]'s business, not this function's.

use super::surface::{surface, Surface};
use super::text;
use crate::style::Style;
use iced::widget::{canvas, column, container, row, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Theme};

/// The envelope beside each row, drawn rather than set.
///
/// It was a text glyph first (U+2709 / U+25AD), which rendered as tofu:
/// Rajdhani has neither, and neither does any era's UI face. A shape
/// costs twenty lines and cannot be missing from a font.
#[derive(Debug, Clone, Copy)]
struct Envelope {
    color: Color,
    sealed: bool,
}

impl<Message> canvas::Program<Message> for Envelope {
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
        // Centre a 16x11 envelope in whatever box we are handed.
        let (ew, eh) = (16.0_f32.min(w), 11.0_f32.min(h));
        let (x, y) = ((w - ew) / 2.0, (h - eh) / 2.0);

        let stroke = canvas::Stroke::default()
            .with_color(self.color)
            .with_width(1.2);

        let body = canvas::Path::rectangle(Point::new(x, y), iced::Size::new(ew, eh));
        frame.stroke(&body, stroke.clone());

        // Sealed: the flap still folded down as a V. Opened: a flat
        // line, the way the references distinguish read from unread.
        let flap = canvas::Path::new(|b| {
            b.move_to(Point::new(x, y));
            if self.sealed {
                b.line_to(Point::new(x + ew / 2.0, y + eh * 0.62));
                b.line_to(Point::new(x + ew, y));
            } else {
                b.line_to(Point::new(x + ew, y));
            }
        });
        frame.stroke(&flap, stroke);

        vec![frame.into_geometry()]
    }
}

pub struct Mail<'a> {
    pub subject: &'a str,
    pub from: &'a str,
    pub unread: bool,
}

/// A single mail row. `selected` swaps in the era's selection fill and
/// flips the ink, exactly as the product card does.
pub fn mail_row<'a, Message: 'static>(
    style: &Style,
    mail: &Mail<'a>,
    selected: bool,
) -> Element<'a, Message> {
    let bg = if selected {
        Surface::selected(style)
    } else {
        // Unselected rows are ruled rather than boxed in the references:
        // an outline per row would turn the list into a grid.
        Surface::outlined(style).no_stroke()
    };

    let subject = if selected {
        text::on_select(style, mail.subject)
    } else {
        text::body(style, mail.subject)
    };
    let from = if selected {
        text::on_select(style, mail.from).size(style.metrics.text_caption)
    } else {
        text::caption(style, mail.from)
    };

    let glyph = container(
        canvas(Envelope {
            color: if selected {
                style.palette.on_select
            } else {
                style.palette.dim
            },
            sealed: mail.unread,
        })
        .width(Length::Fixed(20.0))
        .height(Length::Fixed(16.0)),
    );

    container(surface(
        bg,
        Padding::from([6, 10]),
        row![
            column![subject, from].spacing(1),
            Space::new(Length::Fill, Length::Shrink),
            glyph,
        ]
        .align_y(iced::Alignment::Center),
    ))
    .width(Length::Fill)
    .height(Length::Fixed(46.0))
    .into()
}

/// The rule drawn under an unselected row.
pub fn rule<'a, Message: 'static>(style: &Style) -> Element<'a, Message> {
    let color = style.palette.border;
    container(Space::new(Length::Fill, 1.0))
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(color)),
            ..Default::default()
        })
        .into()
}
