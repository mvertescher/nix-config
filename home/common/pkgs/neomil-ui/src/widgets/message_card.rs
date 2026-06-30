use iced::widget::{canvas, column, container, row, text, Space, stack, mouse_area};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse};
use crate::fonts::{FONT_RAJDHANI_REGULAR, FONT_RAJDHANI_BOLD, FONT_ORBITRON_BOLD};
use crate::colors;

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
pub fn message_card<'a, Message: 'static + Clone>(
    title: &'a str,
    sender: &'a str,
    is_new: bool,
    is_selected: bool,
    on_press: Message,
    color_accent: Color,
) -> Element<'a, Message> {
    let bg_program = MessageCardBackground {
        color: color_accent,
        is_selected,
        cut_size: 8.0,
    };

    // Text colors based on selection state
    let text_color = if is_selected {
        colors::COLOR_BG // Dark text on solid accent background
    } else {
        color_accent
    };

    // "NEW" tag styling
    let new_tag = if is_new {
        let (tag_text_color, tag_border_color) = if is_selected {
            (Color::BLACK, Color::BLACK)
        } else {
            (color_accent, color_accent)
        };

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
