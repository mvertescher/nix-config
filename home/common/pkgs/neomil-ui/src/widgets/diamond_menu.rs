use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme, Size, Vector};
use crate::fonts::FONT_ORBITRON_BOLD;

const DIAMOND_DIAGONAL: f32 = 220.0;
const GAP: f32 = 20.0;
const LABEL_GAP: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct DiamondMenuItem<Message> {
    pub label: String,
    pub subtext: String,
    pub on_press: Message,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabEdge {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabAnchor {
    None,
    Start, // Anchor at p1
    End,   // Anchor at p2
}

struct ButtonGeometry {
    cut_top: bool,
    cut_bottom: bool,
    cut_left: bool,
    cut_right: bool,
    tab_edge: TabEdge,
}

fn get_geometry(index: usize) -> ButtonGeometry {
    match index {
        // Row 1 (Top)
        0 => ButtonGeometry {
            cut_top: true,
            cut_bottom: false,
            cut_left: false,
            cut_right: true,
            tab_edge: TabEdge::BottomRight,
        },
        1 => ButtonGeometry {
            cut_top: true,
            cut_bottom: false,
            cut_left: true,
            cut_right: true,
            tab_edge: TabEdge::BottomRight,
        },
        2 => ButtonGeometry {
            cut_top: true,
            cut_bottom: false,
            cut_left: true,
            cut_right: false,
            tab_edge: TabEdge::BottomRight,
        },
        // Row 2 (Bottom)
        3 => ButtonGeometry {
            cut_top: false,
            cut_bottom: true,
            cut_left: false,
            cut_right: true,
            tab_edge: TabEdge::TopRight,
        },
        4 => ButtonGeometry {
            cut_top: false,
            cut_bottom: true,
            cut_left: true,
            cut_right: true,
            tab_edge: TabEdge::TopRight,
        },
        5 => ButtonGeometry {
            cut_top: false,
            cut_bottom: true,
            cut_left: true,
            cut_right: false,
            tab_edge: TabEdge::TopRight,
        },
        _ => panic!("Invalid button index"),
    }
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
    // Vertical offset for hexagonal packing
    let dy = d / 2.0 + gap * (2.0f32.sqrt() - 0.5);

    let x_center = size.width / 2.0;
    let y_center = size.height / 2.0;

    let x0 = x_center - 1.25 * dx;
    let y0 = y_center - dy / 2.0;

    [
        // Row 1 (Top)
        Point::new(x0, y0),
        Point::new(x0 + dx, y0),
        Point::new(x0 + 2.0 * dx, y0),
        // Row 2 (Bottom)
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
                
                self.draw_button(frame, center, d, item, is_hovered, i);
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
            let d = DIAMOND_DIAGONAL;
            let cut_val = d * 0.10;

            for (i, center) in centers.iter().enumerate() {
                let geom = get_geometry(i);
                let dx = (local_pos.x - center.x).abs();
                let dy = (local_pos.y - center.y).abs();

                // 1. Must be inside the basic diamond
                if dx + dy <= d / 2.0 {
                    // 2. Check horizontal limits (left/right cuts)
                    let limit_left = if geom.cut_left { d / 2.0 - cut_val } else { d / 2.0 };
                    let limit_right = if geom.cut_right { d / 2.0 - cut_val } else { d / 2.0 };
                    let x_ok = if local_pos.x < center.x {
                        dx <= limit_left
                    } else {
                        dx <= limit_right
                    };

                    // 3. Check vertical limits (top/bottom cuts)
                    let limit_top = if geom.cut_top { d / 2.0 - cut_val } else { d / 2.0 };
                    let limit_bottom = if geom.cut_bottom { d / 2.0 - cut_val } else { d / 2.0 };
                    let y_ok = if local_pos.y < center.y {
                        dy <= limit_top
                    } else {
                        dy <= limit_bottom
                    };

                    if x_ok && y_ok {
                        new_hovered_index = Some(i);
                        break;
                    }
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

impl<Message> DiamondMenuProgram<Message> {
    fn draw_button(
        &self,
        frame: &mut canvas::Frame,
        center: Point,
        d: f32,
        item: &DiamondMenuItem<Message>,
        is_hovered: bool,
        index: usize,
    ) {
        let geom = get_geometry(index);
        let is_top_row = index < 3;
        
        // Inverted Style: Solid red background, dark text/glyphs inside
        let bg_alpha = if is_hovered { 1.0 } else { 0.90 };
        let bg_color = Color { a: bg_alpha, ..self.color_accent };
        let border_color = self.color_accent;
        let border_width = if is_hovered { 2.5 } else { 1.5 };
        
        let inside_color = self.color_bg;
        let cut_val = d * 0.10; // 10% of diagonal (22px for 220px)

        // Calculate outer vertices based on custom cuts
        let p_tl_top = Point::new(center.x - cut_val, center.y - d / 2.0 + cut_val);
        let p_tr_top = Point::new(center.x + cut_val, center.y - d / 2.0 + cut_val);
        let p_tr_right = Point::new(center.x + d / 2.0 - cut_val, center.y - cut_val);
        let p_br_right = Point::new(center.x + d / 2.0 - cut_val, center.y + cut_val);
        let p_br_bottom = Point::new(center.x + cut_val, center.y + d / 2.0 - cut_val);
        let p_bl_bottom = Point::new(center.x - cut_val, center.y + d / 2.0 - cut_val);
        let p_bl_left = Point::new(center.x - d / 2.0 + cut_val, center.y + cut_val);
        let p_tl_left = Point::new(center.x - d / 2.0 + cut_val, center.y - cut_val);

        // Draw outer path with custom cuts and single tab
        let path = canvas::Path::new(|builder| {
            // Start at top-left of top corner if cut, else start at top point
            let start_pt = if geom.cut_top { p_tl_top } else { Point::new(center.x, center.y - d / 2.0) };
            builder.move_to(start_pt);
            
            // 1. Top edge (horizontal)
            if geom.cut_top {
                builder.line_to(p_tr_top);
            }
            
            // 2. TR edge (slanted) - Bottom row has TR tab anchored at Top corner (Start)
            let tr_start = if geom.cut_top { p_tr_top } else { Point::new(center.x, center.y - d / 2.0) };
            let tr_end = if geom.cut_right { p_tr_right } else { Point::new(center.x + d / 2.0, center.y) };
            let tr_anchor = if geom.tab_edge == TabEdge::TopRight { TabAnchor::Start } else { TabAnchor::None };
            self.draw_edge(builder, tr_start, tr_end, tr_anchor);
            
            // 3. Right edge (vertical)
            if geom.cut_right {
                builder.line_to(p_br_right);
            }
            
            // 4. BR edge (slanted) - Top row has BR tab anchored at Bottom corner (End)
            let br_start = if geom.cut_right { p_br_right } else { Point::new(center.x + d / 2.0, center.y) };
            let br_end = if geom.cut_bottom { p_br_bottom } else { Point::new(center.x, center.y + d / 2.0) };
            let br_anchor = if geom.tab_edge == TabEdge::BottomRight { TabAnchor::End } else { TabAnchor::None };
            self.draw_edge(builder, br_start, br_end, br_anchor);
            
            // 5. Bottom edge (horizontal)
            if geom.cut_bottom {
                builder.line_to(p_bl_bottom);
            }
            
            // 6. BL edge (slanted) - never has tab
            let bl_start = if geom.cut_bottom { p_bl_bottom } else { Point::new(center.x, center.y + d / 2.0) };
            let bl_end = if geom.cut_left { p_bl_left } else { Point::new(center.x - d / 2.0, center.y) };
            self.draw_edge(builder, bl_start, bl_end, TabAnchor::None);
            
            // 7. Left edge (vertical)
            if geom.cut_left {
                builder.line_to(p_tl_left);
            }
            
            // 8. TL edge (slanted) - never has tab
            let tl_start = if geom.cut_left { p_tl_left } else { Point::new(center.x - d / 2.0, center.y) };
            let tl_end = if geom.cut_top { p_tl_top } else { Point::new(center.x, center.y - d / 2.0) };
            self.draw_edge(builder, tl_start, tl_end, TabAnchor::None);
            
            builder.close();
        });

        frame.fill(&path, bg_color);
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(border_color)
                .with_width(border_width)
                .with_line_join(canvas::LineJoin::Round),
        );

        // Draw inner path (always clean, matches outer cuts but scaled)
        let d_inner = d * 0.85;
        let cut_val_inner = cut_val * 0.85;
        
        let pi_tl_top = Point::new(center.x - cut_val_inner, center.y - d_inner / 2.0 + cut_val_inner);
        let pi_tr_top = Point::new(center.x + cut_val_inner, center.y - d_inner / 2.0 + cut_val_inner);
        let pi_tr_right = Point::new(center.x + d_inner / 2.0 - cut_val_inner, center.y - cut_val_inner);
        let pi_br_right = Point::new(center.x + d_inner / 2.0 - cut_val_inner, center.y + cut_val_inner);
        let pi_br_bottom = Point::new(center.x + cut_val_inner, center.y + d_inner / 2.0 - cut_val_inner);
        let pi_bl_bottom = Point::new(center.x - cut_val_inner, center.y + d_inner / 2.0 - cut_val_inner);
        let pi_bl_left = Point::new(center.x - d_inner / 2.0 + cut_val_inner, center.y + cut_val_inner);
        let pi_tl_left = Point::new(center.x - d_inner / 2.0 + cut_val_inner, center.y - cut_val_inner);

        let inner_path = canvas::Path::new(|builder| {
            let start_pt = if geom.cut_top { pi_tl_top } else { Point::new(center.x, center.y - d_inner / 2.0) };
            builder.move_to(start_pt);
            
            if geom.cut_top {
                builder.line_to(pi_tr_top);
            }
            
            let tr_end = if geom.cut_right { pi_tr_right } else { Point::new(center.x + d_inner / 2.0, center.y) };
            builder.line_to(tr_end);
            
            if geom.cut_right {
                builder.line_to(pi_br_right);
            }
            
            let br_end = if geom.cut_bottom { pi_br_bottom } else { Point::new(center.x, center.y + d_inner / 2.0) };
            builder.line_to(br_end);
            
            if geom.cut_bottom {
                builder.line_to(pi_bl_bottom);
            }
            
            let bl_end = if geom.cut_left { pi_bl_left } else { Point::new(center.x - d_inner / 2.0, center.y) };
            builder.line_to(bl_end);
            
            if geom.cut_left {
                builder.line_to(pi_tl_left);
            }
            
            let tl_end = if geom.cut_top { pi_tl_top } else { Point::new(center.x, center.y - d_inner / 2.0) };
            builder.line_to(tl_end);
            
            builder.close();
        });

        frame.stroke(
            &inner_path,
            canvas::Stroke::default()
                .with_color(inside_color)
                .with_width(1.0),
        );

        // Draw procedural glyph (5x5 symmetric grid)
        let cell_size = 6.0;
        let cell_gap = 1.0;
        let grid_width = 5.0 * cell_size + 4.0 * cell_gap;
        let grid_left = center.x - grid_width / 2.0;
        let grid_top = center.y - grid_width / 2.0 - 16.0;

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
                        inside_color,
                    );
                }
            }
        }

        // Draw subtext (inside, below glyph)
        let subtext = canvas::Text {
            content: item.subtext.clone(),
            position: Point::new(center.x, center.y + 24.0),
            color: inside_color,
            size: 13.0.into(),
            font: FONT_ORBITRON_BOLD,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            ..Default::default()
        };
        frame.fill_text(subtext);

        // Draw label (outside)
        let label_pos = if is_top_row {
            Point::new(center.x, center.y - d / 2.0 - LABEL_GAP)
        } else {
            Point::new(center.x, center.y + d / 2.0 + LABEL_GAP)
        };
        let label_text = canvas::Text {
            content: item.label.clone(),
            position: label_pos,
            color: border_color,
            size: 16.0.into(),
            font: FONT_ORBITRON_BOLD,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: if is_top_row {
                iced::alignment::Vertical::Bottom
            } else {
                iced::alignment::Vertical::Top
            },
            ..Default::default()
        };
        frame.fill_text(label_text);
    }

    fn draw_edge(
        &self,
        builder: &mut canvas::path::Builder,
        p1: Point,
        p2: Point,
        anchor: TabAnchor,
    ) {
        let depth = 10.0;
        let transition = 10.0;
        let tab_width: f32 = 95.0; // Constant physical width in pixels

        if anchor == TabAnchor::None {
            builder.line_to(p2);
        } else {
            let v = Vector::new(p2.x - p1.x, p2.y - p1.y);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            let u = Vector::new(v.x / len, v.y / len);
            let n = Vector::new(u.y, -u.x);

            // Ensure tab_width doesn't exceed edge length
            let w = tab_width.min(len);

            let (pt_start, pt_end) = match anchor {
                TabAnchor::Start => {
                    // Start at p1, extend along edge by w
                    (p1, p1 + u * w)
                }
                TabAnchor::End => {
                    // Start at p2 - w, extend to p2
                    (p2 - u * w, p2)
                }
                _ => unreachable!(),
            };

            let pt_step1 = pt_start + u * transition + n * depth;
            let pt_step2 = pt_end - u * transition + n * depth;

            builder.line_to(pt_start);
            builder.line_to(pt_step1);
            builder.line_to(pt_step2);
            builder.line_to(pt_end);
            builder.line_to(p2);
        }
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
