use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy)]
pub struct FloppyIcon {
    pub color: Color,
}

impl<Message> canvas::Program<Message> for FloppyIcon {
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

        let center_x = bounds.width / 2.0;
        let center_y = bounds.height / 2.0;

        // 1:2 Isometric Projection
        let rx = 1.2;
        let ry = 0.6;
        let project = |x: f32, y: f32, z: f32| {
            Point::new(
                center_x + (x - y) * rx,
                center_y + (x + y) * ry - z,
            )
        };

        let half = 9.0;
        let thick = 4.0;
        let cut = 3.5;

        // Monochrome Red Palette (using the accent color at different opacities)
        // This creates a glowing, holographic HUD look.
        let base_color = self.color;
        
        // Body faces (very translucent to let the background show through, with slight shading)
        let color_top_body = Color { a: 0.15, ..base_color };
        let color_left_body = Color { a: 0.10, ..base_color };
        let color_right_body = Color { a: 0.05, ..base_color };
        let color_cut_body = Color { a: 0.08, ..base_color };

        // Details (more opaque to stand out)
        let color_shutter = Color { a: 0.45, ..base_color };
        let color_label = Color { a: 0.70, ..base_color };
        let color_hub = Color { a: 0.50, ..base_color };
        
        // Holes (black to represent emptiness/shadow, blending with the dark UI background)
        let color_hole = Color::BLACK;

        // Stroke (wireframe outline) - slightly translucent to look glowing/soft
        let stroke_color = Color { a: 0.85, ..base_color };
        let stroke_width = 1.2; // Slightly thicker to help with anti-aliasing visibility

        // --- 1. DRAW 3D SIDES (Thickness) ---
        
        // Bottom-Left Side Face
        let left_side = canvas::Path::new(|builder| {
            builder.move_to(project(-half, half, 0.0));
            builder.line_to(project(half, half, 0.0));
            builder.line_to(project(half, half, thick));
            builder.line_to(project(-half, half, thick));
            builder.close();
        });
        frame.fill(&left_side, color_left_body);
        frame.stroke(&left_side, canvas::Stroke::default().with_color(stroke_color).with_width(stroke_width));

        // Bottom-Right Side Face
        let right_side = canvas::Path::new(|builder| {
            builder.move_to(project(half, half, 0.0));
            builder.line_to(project(half, -half + cut, 0.0));
            builder.line_to(project(half, -half + cut, thick));
            builder.line_to(project(half, half, thick));
            builder.close();
        });
        frame.fill(&right_side, color_right_body);
        frame.stroke(&right_side, canvas::Stroke::default().with_color(stroke_color).with_width(stroke_width));

        // Cut Side Face
        let cut_side = canvas::Path::new(|builder| {
            builder.move_to(project(half, -half + cut, 0.0));
            builder.line_to(project(half - cut, -half, 0.0));
            builder.line_to(project(half - cut, -half, thick));
            builder.line_to(project(half, -half + cut, thick));
            builder.close();
        });
        frame.fill(&cut_side, color_cut_body);
        frame.stroke(&cut_side, canvas::Stroke::default().with_color(stroke_color).with_width(stroke_width));


        // --- 2. DRAW TOP FACE ---
        let top_face = canvas::Path::new(|builder| {
            builder.move_to(project(-half, half, 0.0));
            builder.line_to(project(-half, -half, 0.0));
            builder.line_to(project(half - cut, -half, 0.0));
            builder.line_to(project(half, -half + cut, 0.0));
            builder.line_to(project(half, half, 0.0));
            builder.close();
        });
        frame.fill(&top_face, color_top_body);
        frame.stroke(&top_face, canvas::Stroke::default().with_color(stroke_color).with_width(stroke_width));


        // --- 3. DRAW TOP FACE DETAILS (All at z = 0) ---

        // 3a. Metal Shutter (Top-left-ish)
        let shutter = canvas::Path::new(|builder| {
            builder.move_to(project(-half + 4.0, -half, 0.0));
            builder.line_to(project(half - 6.0, -half, 0.0));
            builder.line_to(project(half - 6.0, -half + 7.0, 0.0));
            builder.line_to(project(-half + 4.0, -half + 7.0, 0.0));
            builder.close();
        });
        frame.fill(&shutter, color_shutter);
        frame.stroke(&shutter, canvas::Stroke::default().with_color(stroke_color).with_width(0.8));

        // 3b. Red Label (Center-bottom)
        let label = canvas::Path::new(|builder| {
            builder.move_to(project(-half + 1.5, -half + 9.0, 0.0));
            builder.line_to(project(half - 1.5, -half + 9.0, 0.0));
            builder.line_to(project(half - 1.5, half - 1.0, 0.0));
            builder.line_to(project(-half + 1.5, half - 1.0, 0.0));
            builder.close();
        });
        frame.fill(&label, color_label);
        frame.stroke(&label, canvas::Stroke::default().with_color(stroke_color).with_width(0.5));

        // 3c. Write-protect hole
        let hole = canvas::Path::new(|builder| {
            let x = -half + 1.5;
            let y = half - 4.0;
            let w = 2.5;
            let h = 2.5;
            builder.move_to(project(x, y, 0.0));
            builder.line_to(project(x + w, y, 0.0));
            builder.line_to(project(x + w, y + h, 0.0));
            builder.line_to(project(x, y + h, 0.0));
            builder.close();
        });
        frame.fill(&hole, color_hole);
        frame.stroke(&hole, canvas::Stroke::default().with_color(stroke_color).with_width(0.5));

        // 3d. Center Hub (sheared ellipse)
        let hub_cx = 0.0;
        let hub_cy = 1.0;
        let hub_r = 3.0;
        
        let hub = canvas::Path::new(|builder| {
            let segments = 16;
            for i in 0..segments {
                let theta = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
                let x = hub_cx + hub_r * theta.cos();
                let y = hub_cy + hub_r * theta.sin();
                let p = project(x, y, 0.0);
                if i == 0 {
                    builder.move_to(p);
                } else {
                    builder.line_to(p);
                }
            }
            builder.close();
        });
        frame.fill(&hub, color_hub);
        frame.stroke(&hub, canvas::Stroke::default().with_color(stroke_color).with_width(0.5));

        // Hub center hole
        let hub_hole = canvas::Path::new(|builder| {
            let segments = 8;
            let r = 1.0;
            for i in 0..segments {
                let theta = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
                let x = hub_cx + r * theta.cos();
                let y = hub_cy + r * theta.sin();
                let p = project(x, y, 0.0);
                if i == 0 {
                    builder.move_to(p);
                } else {
                    builder.line_to(p);
                }
            }
            builder.close();
        });
        frame.fill(&hub_hole, color_hole);

        vec![frame.into_geometry()]
    }
}

pub fn floppy_icon<'a, Message: 'static>(color: Color) -> Element<'a, Message> {
    canvas(FloppyIcon { color })
        .width(Length::Fixed(50.0))
        .height(Length::Fixed(50.0))
        .into()
}
