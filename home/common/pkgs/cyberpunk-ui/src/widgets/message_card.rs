//! The mail-list card, from the neo-militarism app that predates the
//! era generalisation.
//!
//! It took its ink from `crate::colors`, the crate's original
//! neomil-only colour table, and was the last thing in the crate that
//! did. It takes a [`Style`] now and `colors.rs` is gone, so no widget
//! is pinned to one era's palette any more.
//!
//! Two caveats worth stating rather than leaving to be discovered:
//!
//! * **Nothing calls this.** [`crate::screens::mail`] and
//!   [`crate::panels::mail`] both draw their rows with
//!   [`crate::widgets::row::mail_row`], so no golden covers this file.
//!   The palette wiring below is therefore correct by construction and
//!   not by render.
//! * The *geometry* is still neomil's: a fixed 8px double chamfer, cut
//!   at the top right and bottom left. That is why this stays in the
//!   era-specific half of `widgets`. Generalising it is not a matter of
//!   reading `style.corner` here -- [`crate::widgets::surface`] already
//!   draws all four corner treatments, and the honest fix is for this
//!   card to become a `Surface` the way `mail_row` is, or to be deleted
//!   in its favour.

use iced::widget::{canvas, column, container, row, text, Space, stack, mouse_area};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse};
use crate::fonts::{FONT_RAJDHANI_REGULAR, FONT_RAJDHANI_BOLD, FONT_ORBITRON_BOLD};
use crate::style::Style;

#[derive(Debug, Clone, Copy)]
pub struct MessageCardBackground {
    pub color: Color,
    pub is_selected: bool,
    pub cut_size: f32,
}

impl<Message> canvas::Program<Message> for MessageCardBackground {
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
        let h = bounds.height;
        let cut = self.cut_size;

        let is_selected = self.is_selected;
        let offset = if is_selected { 0.0 } else { 1.0 };
        let w_eff = w - offset;
        let h_eff = h - offset;
        let x0 = offset;
        let y0 = offset;

        let path = canvas::Path::new(|builder| {
            builder.move_to(Point::new(x0, y0)); // Top-left sharp
            builder.line_to(Point::new(w_eff - cut, y0)); // Top-right cut start
            builder.line_to(Point::new(w_eff, y0 + cut)); // Top-right cut end
            builder.line_to(Point::new(w_eff, h_eff)); // Bottom-right sharp
            builder.line_to(Point::new(x0 + cut, h_eff)); // Bottom-left cut start
            builder.line_to(Point::new(x0, h_eff - cut)); // Bottom-left cut end
            builder.close();
        });

        if is_selected {
            frame.fill(&path, self.color);
        } else {
            // Translucent background (5% opacity)
            frame.fill(&path, Color { a: 0.05, ..self.color });
            // 1px border
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(self.color)
                    .with_width(1.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// A message card widget for the mail panel list.
/// Displays a title, sender, and an optional "NEW" tag.
/// It is interactive and triggers `on_press` when clicked.
///
/// The era supplies every colour. A selected card is filled with
/// `palette.select` and inked with `palette.on_select` -- the pair
/// [`crate::widgets::row::mail_row`] already uses -- so kitsch gets
/// dark ink on yellow and neokitsch dark ink on veneer, instead of the
/// page ground the neomil-only table handed back for all four.
pub fn message_card<'a, Message: 'static + Clone>(
    style: &Style,
    title: &'a str,
    sender: &'a str,
    is_new: bool,
    is_selected: bool,
    on_press: Message,
) -> Element<'a, Message> {
    // One accent drives the card: the era's selection fill when the row
    // is chosen, its body colour for the wash and hairline when it is
    // not. That is what the caller used to pass in by hand.
    let accent = if is_selected {
        style.palette.select
    } else {
        style.palette.fg
    };

    let bg_program = MessageCardBackground {
        color: accent,
        is_selected,
        cut_size: 8.0,
    };

    // Ink follows the fill it sits on.
    let text_color = if is_selected {
        style.palette.on_select
    } else {
        style.palette.fg
    };

    // "NEW" tag styling. Both the label and its outline take the same
    // ink as the rest of the card; the selected case used to be a flat
    // `Color::BLACK`, which is legible only for as long as every era
    // selects with a light fill. `on_select` is the role that actually
    // promises that.
    let new_tag = if is_new {
        let (tag_text_color, tag_border_color) = (text_color, text_color);

        Some(
            container(
                text("NEW")
                    .size(8)
                    .font(FONT_ORBITRON_BOLD)
                    .style(move |_| text::Style { color: Some(tag_text_color) })
            )
            .padding([2, 54])
            .style(move |_| container::Style {
                background: None,
                border: iced::Border {
                    color: tag_border_color,
                    width: 1.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            })
        )
    } else {
        None
    };

    // Content Layout
    let mut info_column = column![
        row![
            text(title)
                .size(14)
                .font(FONT_RAJDHANI_BOLD)
                .style(move |_| text::Style { color: Some(text_color) }),
            Space::with_width(Length::Fill),
            text(sender)
                .size(12)
                .font(FONT_RAJDHANI_REGULAR)
                .style(move |_| text::Style { color: Some(text_color) }),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
    ]
    .spacing(4)
    .width(Length::Fill);

    if let Some(tag) = new_tag {
        info_column = info_column.push(
            row![Space::with_width(Length::Fill), tag]
                .width(Length::Fill)
        );
    } else {
        // Maintain height consistency if no tag
        info_column = info_column.push(Space::with_height(12));
    }

    let card_content = stack![
        canvas(bg_program)
            .width(Length::Fill)
            .height(Length::Fill),
        container(info_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([10, 15])
            .align_y(Alignment::Center)
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Make the entire card clickable and wrap in a padded container to prevent clipping
    container(
        mouse_area(card_content)
            .on_press(on_press)
    )
    .width(Length::Fill)
    .height(Length::Fixed(60.0))
    .padding(iced::Padding {
        top: 1.0,
        right: 0.0,
        bottom: 1.0,
        left: 0.0,
    })
    .into()
}
