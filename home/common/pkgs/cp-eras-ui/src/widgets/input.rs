//! Fields and the affirmative action beside them.
//!
//! Every era's login screen is the same three things: a label, a masked
//! field, and one button that commits. They differ only in the fill that
//! button takes, which is why `Palette::cta` exists as its own slot --
//! see the note there.

use super::surface::{surface, Surface};
use super::text;
use crate::style::Style;
use iced::widget::{column, container, row, Space};
use iced::{Element, Length, Padding};

/// A value field. Display-only for now: the screens this serves are
/// design targets rather than working login forms, and a real
/// `text_input` would need styling per era before it earned its place.
pub fn field<'a, Message: 'static>(
    style: &Style,
    value: &'a str,
    width: f32,
) -> Element<'a, Message> {
    container(surface(
        Surface::outlined(style),
        Padding::from([6, 10]),
        text::body(style, value),
    ))
    .width(Length::Fixed(width))
    .height(Length::Fixed(32.0))
    .into()
}

/// The affirmative action: ENTER, NEXT, LOGIN. Filled, never outlined --
/// in all four references this is the one solid control on the screen.
pub fn cta<'a, Message: 'static>(
    style: &Style,
    label: &'a str,
    width: f32,
) -> Element<'a, Message> {
    container(surface(
        Surface::filled(style, style.palette.cta).no_stroke(),
        Padding::from([6, 10]),
        container(text::on_select(style, label)).center_x(Length::Fill),
    ))
    .width(Length::Fixed(width))
    .height(Length::Fixed(32.0))
    .into()
}

/// Label, field and action as the references arrange them.
pub fn labelled_field<'a, Message: 'static>(
    style: &Style,
    label: &'a str,
    value: &'a str,
    action: &'a str,
) -> Element<'a, Message> {
    column![
        text::body(style, label),
        Space::new(0.0, 6.0),
        row![
            field(style, value, 300.0),
            Space::new(style.metrics.gap * 0.5, 0.0),
            cta(style, action, 110.0),
        ],
    ]
    .into()
}
