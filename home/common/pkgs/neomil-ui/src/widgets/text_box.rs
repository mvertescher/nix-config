use iced::widget::{canvas, column, container, row, text, Space, stack};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse, Vector};
use crate::fonts::{FONT_ORBITRON_BOLD, FONT_RAJDHANI_REGULAR};

const MARGIN: f32 = 12.0; // Safety margin for external decorations
const TAB_DEPTH: f32 = 18.0; // How much the right tab steps out

#[derive(Debug, Clone)]
pub struct TextBoxBackground {
    pub bg_color: Color,
    pub border_color: Color,
    pub cut_small: f32,
    pub cut_large: f32,
    pub border_width: f32,
    pub vertical_texts: Vec<String>,
}

impl<Message> canvas::Program<Message> for TextBoxBackground {
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
        let cut_s = self.cut_small;
        let cut_l = self.cut_large;
        let inset = self.border_width / 2.0;

        let x_left = MARGIN + inset;
        // Shift x_right left to make room for the tab within the canvas
        let x_right = w - MARGIN - inset - TAB_DEPTH;
        let y_top = MARGIN + inset;
        let y_bottom = h - MARGIN - inset;

        let y_tab_start = y_top + cut_s + 20.0;
        let y_tab_height = 120.0; // Height of the text part of the tab
        let y_ext_height = 80.0;  // Height of the filled extension
        let y_tab_end = y_tab_start + y_tab_height;
        let y_ext_end = y_tab_end + y_ext_height;
        
        let transition = TAB_DEPTH; // Diagonal transition for the tab (45 degrees)

        // Ensure we have enough space to draw the cuts and tab
        if x_right > x_left + cut_s + cut_l && y_bottom > y_ext_end + cut_l {
            // Path: Top-Left sharp, Top-Right cut (small), Right border with step-out tab + extension, Bottom-Right sharp, Bottom-Left cut (large)
            let path = canvas::Path::new(|builder| {
                // Top-left sharp
                builder.move_to(Point::new(x_left, y_top));
                
                // Top-right cut (small)
                builder.line_to(Point::new(x_right - cut_s, y_top));
                builder.line_to(Point::new(x_right, y_top + cut_s));
                
                // Right border with tab and extension
                builder.line_to(Point::new(x_right, y_tab_start));
                builder.line_to(Point::new(x_right + TAB_DEPTH, y_tab_start + transition));
                builder.line_to(Point::new(x_right + TAB_DEPTH, y_ext_end - transition));
                builder.line_to(Point::new(x_right, y_ext_end));
                
                // Bottom-right sharp
                builder.line_to(Point::new(x_right, y_bottom));
                
                // Bottom-left cut (large)
                builder.line_to(Point::new(x_left + cut_l, y_bottom));
                builder.line_to(Point::new(x_left, y_bottom - cut_l));
                builder.close();
            });

            // 1. Fill main background (5% opacity)
            frame.fill(&path, self.bg_color);

            // 2. Fill the tab extension with solid accent color (100% opacity)
            let ext_path = canvas::Path::new(|builder| {
                builder.move_to(Point::new(x_right, y_tab_end));
                builder.line_to(Point::new(x_right + TAB_DEPTH, y_tab_end - transition));
                builder.line_to(Point::new(x_right + TAB_DEPTH, y_ext_end - transition));
                builder.line_to(Point::new(x_right, y_ext_end));
                builder.close();
            });
            frame.fill(&ext_path, self.border_color);

            // 3. Stroke border (100% opacity) with round joins
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(self.border_color)
                    .with_width(self.border_width)
                    .with_line_join(canvas::LineJoin::Round),
            );

            // --- DECORATIONS ---
            // Vertical texts inside the tab (centered in the text part of the tab)
            let y_center = y_tab_start + y_tab_height / 2.0;
            let n = self.vertical_texts.len();
            for (i, txt) in self.vertical_texts.iter().enumerate() {
                let x_offset = if n == 1 {
                    TAB_DEPTH / 2.0
                } else {
                    TAB_DEPTH * 0.25 + (TAB_DEPTH * 0.5) * (i as f32 / (n - 1) as f32)
                };
                let x_pos = x_right + x_offset;

                frame.with_save(|frame| {
                    frame.translate(Vector::new(x_pos, y_center));
                    frame.rotate(-std::f32::consts::FRAC_PI_2);

                    let canvas_text = canvas::Text {
                        content: txt.clone(),
                        position: Point::ORIGIN,
                        color: self.border_color,
                        size: 8.0.into(),
                        font: FONT_ORBITRON_BOLD,
                        horizontal_alignment: iced::alignment::Horizontal::Center,
                        vertical_alignment: iced::alignment::Vertical::Center,
                        ..Default::default()
                    };
                    frame.fill_text(canvas_text);
                });
            }
        }

        vec![frame.into_geometry()]
    }
}

/// A specialized text box widget with custom corner cuts, a title, body text,
/// and Cyberpunk-themed decorative elements (step-out tab on the right with vertical text and filled extension, and bottom logo).
pub fn text_box<'a, Message: 'static>(
    title: &'a str,
    body: &'a str,
    vertical_texts: &[&'a str],
    logo_char: &'a str,
    logo_sub1: &'a str,
    logo_sub2: &'a str,
    color_accent: Color,
) -> Element<'a, Message> {
    // Background is accent color at 5% opacity
    let bg_color = Color { a: 0.05, ..color_accent };

    let bg_program = TextBoxBackground {
        bg_color,
        border_color: color_accent,
        cut_small: 12.0,
        cut_large: 30.0,
        border_width: 2.0,
        vertical_texts: vertical_texts.iter().map(|s| s.to_string()).collect(),
    };

    // --- Content Layout (Title, Body, Logo) ---
    let mut left_content = column![
        text(title)
            .size(20)
            .font(FONT_ORBITRON_BOLD)
            .style(move |_| text::Style { color: Some(color_accent) }),
        Space::with_height(15),
        text(body)
            .size(12)
            .font(FONT_RAJDHANI_REGULAR)
            .style(move |_| text::Style { color: Some(color_accent) }),
        Space::with_height(Length::Fill),
    ]
    .spacing(0)
    .width(Length::Fill);

    // Add bottom logo if logo_char is not empty
    if !logo_char.is_empty() {
        let logo = row![
            text(logo_char)
                .size(54)
                .font(FONT_ORBITRON_BOLD)
                .style(move |_| text::Style { color: Some(color_accent) }),
            Space::with_width(10),
            column![
                text(logo_sub1)
                    .size(10)
                    .font(FONT_ORBITRON_BOLD)
                    .style(move |_| text::Style { color: Some(color_accent) }),
                text(logo_sub2)
                    .size(10)
                    .font(FONT_ORBITRON_BOLD)
                    .style(move |_| text::Style { color: Some(color_accent) }),
            ]
        ]
        .align_y(Alignment::Center);
        
        left_content = left_content.push(logo);
    }

    // Right padding is increased by TAB_DEPTH to keep content inside the main box area
    stack![
        canvas(bg_program)
            .width(Length::Fill)
            .height(Length::Fill),
        container(left_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(iced::Padding {
                top: 20.0 + MARGIN,
                bottom: 20.0 + MARGIN,
                left: 20.0 + MARGIN,
                right: 20.0 + MARGIN + TAB_DEPTH,
            })
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
