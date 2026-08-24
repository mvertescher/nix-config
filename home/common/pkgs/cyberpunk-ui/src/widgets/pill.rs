//! Nav items.
//!
//! Every era's store screen has the same vertical list of weapon
//! categories with exactly one selected, and each dresses it
//! differently: entropism outlines a box, kitsch cuts a ticket notch,
//! neomil chamfers, neokitsch rounds slightly and clips. The difference
//! is entirely [`Surface`]'s, so this is one function.
//!
//! The ticket notch is the one of those four that is *not* a corner
//! treatment, and it is why [`crate::style::Ticket`] exists as its own
//! knob: kitsch's cards are plain `rx="16"` and only its nav pills jut
//! out. So the era declares it, the pill asks for it, and `badge` --
//! the same shape at socket and clearance size -- deliberately does
//! not, because no reference draws one on those.

use super::surface::{surface, Surface};
use super::text;
use crate::style::Style;
use iced::widget::container;
use iced::{Element, Length, Padding};

pub fn pill<'a, Message: 'static>(
    style: &Style,
    label: &'a str,
    selected: bool,
) -> Element<'a, Message> {
    let bg = if selected {
        Surface::selected(style)
    } else {
        Surface::outlined(style)
    }
    .ticket(style.ticket);

    let content = if selected {
        text::on_select(style, label)
    } else {
        text::body(style, label)
    };

    container(surface(
        bg,
        Padding {
            top: 6.0,
            bottom: 6.0,
            left: 14.0,
            // The wedge grows into the box rather than out of it, so
            // the label needs to clear the body's right edge and not
            // the widget's. Costs nothing in the three eras that
            // declare no ticket.
            right: 14.0 + style.ticket.reach,
        },
        content,
    ))
    .width(Length::Fill)
    .height(Length::Fixed(34.0))
    .into()
}

/// A small square-ish state chip: security levels, T1..T4, socket slots.
pub fn badge<'a, Message: 'static>(
    style: &Style,
    label: &'a str,
    selected: bool,
    width: f32,
) -> Element<'a, Message> {
    let bg = if selected {
        Surface::selected(style)
    } else {
        Surface::outlined(style)
    };

    let content = if selected {
        text::on_select(style, label)
    } else {
        text::body(style, label)
    };

    container(surface(
        bg,
        Padding {
            top: 4.0,
            bottom: 4.0,
            left: 8.0,
            right: 8.0,
        },
        content,
    ))
    .width(Length::Fixed(width))
    .height(Length::Fixed(30.0))
    .into()
}
