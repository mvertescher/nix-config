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
use crate::style::{Selection, Style};
use iced::widget::{canvas, container, row, stack, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Theme};

/// The band's fill and ink for the given card state.
///
/// A band in the selection fill, on a card in the selection fill, is
/// invisible -- so both maximalist eras move it, and they move it in
/// opposite directions. Which one an era wants follows from what its
/// selected card is *made of*, which is already a parameter:
///
/// * [`Selection::Solid`] -- kitsch. The band on the yellow card is the
///   bottom stop of the same gradient the card is filled with, a shade
///   of itself (`fill="#f0a80a"` against `#fcc428`). Rendering selection
///   flat, the equivalent move is to pull the fill towards its own ink.
/// * [`Selection::Veneer`] -- neokitsch. You cannot shade a *material*:
///   a slightly darker champagne band on a grained plank reads as a
///   knot, so the target inverts instead -- `rect fill="#3a2410"` with
///   `#e7c686` text where the unselected cards have `#d3b279` with
///   `#3a2410`. That is a straight swap of the pair.
///
/// This is a derivation, not a role, and it is the weaker of the two:
/// a published `banner`-on-selected pair would say what each era wants
/// rather than inferring it. It is here because the vocabulary in
/// `home/themes/lib/roles.nix` has no slot for it yet.
pub fn banner_colors(style: &Style, selected: bool) -> (Color, Color) {
    let (fill, ink) = style.banner_colors();
    if !selected {
        return (fill, ink);
    }
    match style.selection {
        Selection::Veneer => (ink, fill),
        Selection::Solid => {
            let mix = |a: f32, b: f32| a * 0.85 + b * 0.15;
            (
                Color {
                    r: mix(fill.r, ink.r),
                    g: mix(fill.g, ink.g),
                    b: mix(fill.b, ink.b),
                    a: fill.a,
                },
                ink,
            )
        }
    }
}

/// The band's shape: a filled rectangle whose leading `overhang` steps
/// down by `notch`.
#[derive(Debug, Clone, Copy)]
pub struct Band {
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
            row![lead, Space::new(Length::Fill, 0.0), tail]
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
    Space::new(0.0, 0.0).into()
}

/// A label in the band's ink, at the band's own size.
pub fn tag<'a, Message: 'static>(
    style: &Style,
    selected: bool,
    content: &'a str,
    size: u16,
) -> Element<'a, Message> {
    let (_, ink) = banner_colors(style, selected);
    text::caption(style, content).size(size).color(ink).into()
}
