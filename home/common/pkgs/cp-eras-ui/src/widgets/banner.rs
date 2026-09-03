//! The accent band a maximalist era runs across its card.
//!
//! Kitsch's shelf band and neokitsch's footer nameplate are the same
//! object seen from the two ends of a card: a filled band that hangs
//! past the card's leading edge, carrying small marks at one end and a
//! label at the other. The colours come from
//! [`crate::style::Style::banner_colors`] and the shape from
//! [`crate::style::Banner`], so there is no era test in here -- an era
//! that declares no banner simply never asks for one.
//!
//! The overhang is why the band is drawn by the card's own layout
//! rather than by [`super::surface`]: the band is wider than the shape
//! behind it. The card gives its background an inset of `overhang` and
//! lets this run the full width, which puts the tab outside the card
//! without any widget having to paint beyond its bounds.

use super::text;
use crate::style::Style;
use iced::widget::{canvas, container, row, stack, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Theme};

/// The band's fill and ink for the given card state.
///
/// This used to key off [`crate::style::Selection`] and *infer* the
/// selected pair -- pull kitsch's fill 15% towards its own ink, swap
/// neokitsch's outright -- on the reasoning that what a band does when
/// selected follows from what the card it sits on is made of. The
/// reasoning was sound and the arithmetic was not: a 15% mix lands on
/// `#deab24` where the target draws `#f0a80a`, and no mix towards the
/// ink can reach it at all.
///
/// So the pair is now sampled per era and asked for by name. Both the
/// values and the argument for keeping them era-owned rather than
/// publishing them as a role live on
/// [`crate::palette::Palette::banner_on_select`]; this is the two-line
/// consequence.
pub fn banner_colors(style: &Style, selected: bool) -> (Color, Color) {
    if selected {
        style.banner_on_select()
    } else {
        style.banner_colors()
    }
}

/// The band's shape: a filled rectangle whose leading `overhang` steps
/// down by `notch`.
#[derive(Debug, Clone, Copy)]
struct Band {
    pub fill: Color,
    pub height: f32,
    pub overhang: f32,
    pub notch: f32,
}

impl<Message> canvas::Program<Message> for Band {
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
        if w <= 0.0 || self.height <= 0.0 {
            return vec![frame.into_geometry()];
        }

        let overhang = self.overhang.min(w);
        let path = canvas::Path::new(|b| {
            b.move_to(Point::new(0.0, 0.0));
            b.line_to(Point::new(w, 0.0));
            b.line_to(Point::new(w, self.height));
            b.line_to(Point::new(overhang, self.height));
            b.line_to(Point::new(0.0, self.height + self.notch));
            b.close();
        });
        frame.fill(&path, self.fill);

        vec![frame.into_geometry()]
    }
}

/// A band `height` tall carrying `lead` at its head and `tail` at its
/// foot, laid out across whatever width it is offered.
pub fn banner<'a, Message: 'static>(
    style: &Style,
    selected: bool,
    height: f32,
    lead: Element<'a, Message>,
    tail: Element<'a, Message>,
) -> Element<'a, Message> {
    let (fill, _) = banner_colors(style, selected);
    let overhang = style.banner.overhang;
    let notch = style.banner.notch;

    stack![
        canvas(Band {
            fill,
            height,
            overhang,
            notch,
        })
        .width(Length::Fill)
        .height(Length::Fixed(height + notch)),
        container(
            row![lead, Space::new().width(Length::Fill), tail]
                .align_y(iced::Alignment::Center)
                .spacing(6)
        )
        .padding(Padding {
            top: 0.0,
            right: 6.0,
            bottom: 0.0,
            // Marks start just inside the tab, as the reference does.
            left: overhang.max(8.0),
        })
        .height(Length::Fixed(height))
        .center_y(Length::Fixed(height)),
    ]
    .into()
}

/// The band's natural height for text of the era's `size`.
///
/// Sampled: the kitsch shelf band is 20 high around a 9pt tag and the
/// neokitsch nameplate 22 around a 12pt name, so the band is its text
/// plus a fixed 11.
pub fn band_height(size: u16) -> f32 {
    size as f32 + 11.0
}

/// Nothing, in a shape the band's row will accept.
pub fn blank<'a, Message: 'static>() -> Element<'a, Message> {
    Space::new().into()
}

/// A label in the band's ink, at the band's own size.
pub fn tag<'a, Message: 'static>(
    style: &Style,
    selected: bool,
    content: &'a str,
    size: u16,
) -> Element<'a, Message> {
    let (_, ink) = banner_colors(style, selected);
    text::caption(style, content).size(f32::from(size)).color(ink).into()
}
