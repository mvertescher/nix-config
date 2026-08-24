//! Top and bottom furniture.
//!
//! All four eras frame every screen with something; they disagree about
//! what. Entropism runs a row of outlined boxes over a build-string
//! footer, neomil a thin rule with a hostname tape, kitsch a single
//! centred compliance caption, neokitsch a strata wedge of fine lines.
//! [`crate::style::Chrome`] picks; the screens never ask.

use super::surface::{surface, Surface};
use super::text;
use crate::style::{Chrome, Style};
use iced::widget::{canvas, column, container, row, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Theme};

/// The layered fine lines neokitsch closes every screen with, bunching
/// into a wedge at one end.
#[derive(Debug, Clone, Copy)]
struct Strata {
    pub color: Color,
    pub lines: usize,
}

impl<Message> canvas::Program<Message> for Strata {
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
        let step = if self.lines > 1 {
            h / self.lines as f32
        } else {
            h
        };

        for i in 0..self.lines {
            let t = i as f32 / self.lines.max(1) as f32;
            let y = i as f32 * step;
            // Each line starts further in and fades, so the stack reads
            // as a wedge rather than a ruled block.
            let x0 = w * 0.012 * i as f32;
            let path = canvas::Path::new(|b| {
                b.move_to(Point::new(x0, y));
                b.line_to(Point::new(w - x0 * 0.5, y));
            });
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(Color {
                        a: 1.0 - t * 0.8,
                        ..self.color
                    })
                    .with_width(1.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// The header for the current era. `segments` is used by the eras that
/// have a segmented bar and ignored by the ones that do not.
pub fn top_bar<'a, Message: 'static>(
    style: &Style,
    segments: [&'a str; 3],
) -> Element<'a, Message> {
    match style.chrome {
        Chrome::Segmented => {
            // `fill="#728f76"` in the entropism target -- the ink
            // between `dim` and `fg`, not `dim` itself. At 14pt on that
            // ground `dim` measures 2.1:1, which is what made the
            // segmented bar read as three empty boxes.
            let boxed = |label: &'a str, width: Length, center: bool| {
                let inner: Element<'a, Message> = if center {
                    container(text::mid(style, label))
                        .center_x(Length::Fill)
                        .into()
                } else {
                    text::mid(style, label).into()
                };
                container(surface(
                    Surface::outlined(style),
                    Padding::from([4, 12]),
                    inner,
                ))
                .width(width)
                .height(Length::Fixed(28.0))
            };
            row![
                boxed(segments[0], Length::FillPortion(3), false),
                boxed(segments[1], Length::FillPortion(7), true),
                boxed(segments[2], Length::FillPortion(2), false),
            ]
            .spacing(6)
            .into()
        }
        Chrome::Tape => row![
            container(surface(
                Surface::filled(style, style.palette.tape).no_stroke(),
                Padding::from([3, 10]),
                text::body(style, segments[0]).color(style.palette.bg),
            ))
            .width(Length::Fixed(320.0))
            .height(Length::Fixed(24.0)),
            Space::new(12.0, 0.0),
            container(text::label(style, segments[1])).center_y(Length::Fixed(24.0)),
            Space::new(Length::Fill, Length::Shrink),
            container(text::label(style, segments[2])).center_y(Length::Fixed(24.0)),
        ]
        .into(),
        // Kitsch puts nothing at the top; neokitsch's frame is drawn by
        // the screen itself, so the bar is just the meta line.
        Chrome::Caption | Chrome::DeviceFrame => row![
            text::caption(style, segments[0]),
            Space::new(Length::Fill, Length::Shrink),
            text::caption(style, segments[2]),
        ]
        .into(),
    }
}

/// The footer for the current era.
pub fn footer<'a, Message: 'static>(
    style: &Style,
    left: &'a str,
    middle: &'a str,
    right: &'a str,
) -> Element<'a, Message> {
    // Copied out so the container-style closure does not borrow `style`.
    let border = style.palette.border;

    // The build-string footer is `fill="#728f76"` in the entropism
    // target, same as the segmented bar above it: small print, but not
    // the tertiary ink the footnote bodies are set in.
    let line = row![
        text::mid(style, left).size(style.metrics.text_caption + 2),
        Space::new(Length::Fill, Length::Shrink),
        text::mid(style, middle).size(style.metrics.text_caption + 2),
        Space::new(Length::Fill, Length::Shrink),
        text::mid(style, right).size(style.metrics.text_caption + 2),
    ];

    match style.chrome {
        Chrome::Segmented => column![
            container(Space::new(Length::Fill, 1.0)).style(move |_: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(border)),
                    ..Default::default()
                }
            }),
            container(line).padding(Padding::from([8, 0])),
        ]
        .into(),
        Chrome::DeviceFrame => column![
            canvas(Strata {
                color: style.palette.border,
                lines: 5,
            })
            .width(Length::Fill)
            .height(Length::Fixed(30.0)),
            container(line).padding(Padding::from([4, 0])),
        ]
        .into(),
        Chrome::Caption => container(
            row![
                text::caption(style, middle)
                    .size(style.metrics.text_caption + 2)
                    .color(style.palette.fg),
            ]
            .spacing(6),
        )
        .center_x(Length::Fill)
        .into(),
        Chrome::Tape => container(line).padding(Padding::from([8, 0])).into(),
    }
}
