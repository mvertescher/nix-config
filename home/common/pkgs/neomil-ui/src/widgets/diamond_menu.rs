use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme, Size};

const DIAMOND_DIAGONAL: f32 = 120.0;
const GAP: f32 = 10.0;
const LABEL_GAP: f32 = 15.0;

#[derive(Debug, Clone)]
pub struct DiamondMenuItem<Message> {
    pub label: String,
    pub subtext: String,
    pub on_press: Message,
}

pub struct DiamondMenuProgram<Message> {
    items: Vec<DiamondMenuItem<Message>>,
    color_accent: Color,
    color_bg: Color,
}

pub struct MenuState {
    hovered_index: Option<usize>,
    cache: canvas::Cache,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            hovered_index: None,
            cache: canvas::Cache::new(),
        }
    }
}

fn hash_str(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for c in s.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
    }
    hash
}

fn get_centers(size: Size) -> [Point; 6] {
    let d = DIAMOND_DIAGONAL;
    let gap = GAP;
    let dx = d + gap;
    let dy = d / 2.0 + gap * (2.0f32.sqrt() - 0.5);

    let x_center = size.width / 2.0;
    let y_center = size.height / 2.0;

    let x0 = x_center - 1.25 * dx;
    let y0 = y_center - dy / 2.0;

    [
        // Row 1
        Point::new(x0, y0),
        Point::new(x0 + dx, y0),
        Point::new(x0 + 2.0 * dx, y0),
        // Row 2
        Point::new(x0 + dx / 2.0, y0 + dy),
        Point::new(x0 + 3.0 * dx / 2.0, y0 + dy),
        Point::new(x0 + 5.0 * dx / 2.0, y0 + dy),
    ]
}

impl<Message: Clone> canvas::Program<Message> for DiamondMenuProgram<Message> {
    type State = MenuState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = state.cache.draw(renderer, bounds.size(), |frame| {
            let centers = get_centers(bounds.size());
            let d = DIAMOND_DIAGONAL;

            for (i, item) in self.items.iter().enumerate() {
                let center = centers[i];
                let is_hovered = state.hovered_index == Some(i);

                // Adjust colors for hover
                let bg_alpha = if is_hovered { 0.3 } else { 0.15 };
                let bg_color = Color { a: bg_alpha, ..self.color_bg };
                let border_color = self.color_accent;
                let border_width = if is_hovered { 2.5 } else { 1.5 };

                // Draw outer diamond
                let path = canvas::Path::new(|builder| {
                    builder.move_to(Point::new(center.x, center.y - d / 2.0));
                    builder.line_to(Point::new(center.x + d / 2.0, center.y));
                    builder.line_to(Point::new(center.x, center.y + d / 2.0));
                    builder.line_to(Point::new(center.x - d / 2.0, center.y));
                    builder.close();
                });

                frame.fill(&path, bg_color);
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(border_color)
                        .with_width(border_width),
                );

                // Draw inner diamond
                let d_inner = d * 0.8;
                let inner_path = canvas::Path::new(|builder| {
                    builder.move_to(Point::new(center.x, center.y - d_inner / 2.0));
                    builder.line_to(Point::new(center.x + d_inner / 2.0, center.y));
                    builder.line_to(Point::new(center.x, center.y + d_inner / 2.0));
                    builder.line_to(Point::new(center.x - d_inner / 2.0, center.y));
                    builder.close();
                });

                frame.stroke(
                    &inner_path,
                    canvas::Stroke::default()
                        .with_color(border_color)
                        .with_width(1.0),
                );

                // Draw procedural glyph (5x5 symmetric grid)
                let cell_size = 4.0;
                let cell_gap = 1.0;
                let grid_width = 5.0 * cell_size + 4.0 * cell_gap;
                let grid_left = center.x - grid_width / 2.0;
                let grid_top = center.y - grid_width / 2.0 - 10.0; // Shifted up

                let hash = hash_str(&item.label);
                for r in 0..5 {
                    for c in 0..5 {
                        let bit_idx = match c {
                            0 | 4 => r,
                            1 | 3 => 5 + r,
                            2 => 10 + r,
                            _ => unreachable!(),
                        };
                        let is_filled = (hash & (1 << bit_idx)) != 0;
                        if is_filled {
                            let px = grid_left + c as f32 * (cell_size + cell_gap);
                            let py = grid_top + r as f32 * (cell_size + cell_gap);
                            frame.fill_rectangle(
                                Point::new(px, py),
                                iced::Size::new(cell_size, cell_size),
                                border_color,
                            );
                        }
                    }
                }

                // Draw subtext
                let text = canvas::Text {
                    content: item.subtext.clone(),
                    position: Point::new(center.x, center.y + 15.0),
                    color: border_color,
                    size: 10.0.into(),
                    font: iced::Font::default(),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    ..Default::default()
                };
                frame.fill_text(text);

                // Draw label
                let is_top = i < 3;
                let label_pos = if is_top {
                    Point::new(center.x, center.y - d / 2.0 - LABEL_GAP)
                } else {
                    Point::new(center.x, center.y + d / 2.0 + LABEL_GAP)
                };
                let label_text = canvas::Text {
                    content: item.label.clone(),
                    position: label_pos,
                    color: border_color,
                    size: 12.0.into(),
                    font: iced::Font::default(),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: if is_top {
                        iced::alignment::Vertical::Bottom
                    } else {
                        iced::alignment::Vertical::Top
                    },
                    ..Default::default()
                };
                frame.fill_text(label_text);
            }
        });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let mut new_hovered_index = None;
        if let Some(local_pos) = cursor.position_in(bounds) {
            let centers = get_centers(bounds.size());
            for (i, center) in centers.iter().enumerate() {
                let dx = (local_pos.x - center.x).abs();
                let dy = (local_pos.y - center.y).abs();
                if dx + dy <= DIAMOND_DIAGONAL / 2.0 {
                    new_hovered_index = Some(i);
                    break;
                }
            }
        }

        if new_hovered_index != state.hovered_index {
            state.hovered_index = new_hovered_index;
            state.cache.clear();
        }

        let mut message = None;
        if let canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(idx) = state.hovered_index {
                message = Some(self.items[idx].on_press.clone());
            }
        }

        let status = if message.is_some() {
            canvas::event::Status::Captured
        } else {
            canvas::event::Status::Ignored
        };

        (status, message)
    }
}

pub fn diamond_menu<'a, Message: 'static + Clone>(
    items: Vec<DiamondMenuItem<Message>>,
    color_accent: Color,
    color_bg: Color,
) -> Element<'a, Message> {
    assert_eq!(items.len(), 6, "DiamondMenu requires exactly 6 items");

    canvas(DiamondMenuProgram {
        items,
        color_accent,
        color_bg,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
