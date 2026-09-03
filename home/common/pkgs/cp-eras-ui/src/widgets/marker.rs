//! Boxed footnote markers.
//!
//! [A], [B], [C] beside two lines of tiny print. Present in every era's
//! references, always in the same shape, which makes it a good check
//! that the shared vocabulary is really shared: nothing here branches.

use super::surface::Surface;
use super::text;
use crate::style::Style;
use iced::widget::{canvas, column, container, row, stack, Space};
use iced::{Element, Length};

pub fn marker<'a, Message: 'static>(
    style: &Style,
    letter: &'a str,
    lines: &[&'a str],
) -> Element<'a, Message> {
    // Square in every era's references -- `rect x=380 y=796 width=26
    // height=26` even in kitsch, which rounds its containers -- and in
    // the marker's own ink rather than the border's. It was its own
    // canvas program until [`Surface::square`] made that sayable.
    let boxed = container(stack![
        canvas(
            Surface::outlined(style)
                .square()
                .stroke(style.palette.fg)
        )
        .width(Length::Fill)
        .height(Length::Fill),
        container(text::body(style, letter))
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ])
    .width(Length::Fixed(24.0))
    .height(Length::Fixed(24.0));

    let mut notes = column![].spacing(2);
    for line in lines {
        notes = notes.push(text::caption(style, *line));
    }

    row![boxed, Space::new().width(10.0), notes]
        .align_y(iced::Alignment::Center)
        .into()
}
