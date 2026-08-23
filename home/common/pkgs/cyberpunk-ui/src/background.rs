use iced::widget::{canvas, container, stack};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

struct BackgroundProgram {
    bg_color: Color,
    glow_color: Color,
    draw_glows: bool,
}

impl<Message> canvas::Program<Message> for BackgroundProgram {
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

        // Fill background with solid/translucent BG color
        frame.fill(&canvas::Path::rectangle(Point::ORIGIN, bounds.size()), self.bg_color);

        if self.draw_glows {
            // Helper to draw a radial glow
            let draw_glow = |frame: &mut canvas::Frame, center: Point, max_radius: f32, color: Color| {
                let steps = 40;
                let base_alpha = color.a;
                for i in 0..steps {
                    let t = i as f32 / steps as f32;
                    let radius = max_radius * (1.0 - t);
                    // Fade from base_alpha at center (t=1.0) to 0.0 at edge (t=0.0)
                    let alpha = base_alpha * t.powf(2.0); // Quadratic falloff
                    let circle_color = Color { a: alpha, ..color };
                    let path = canvas::Path::circle(center, radius);
                    frame.fill(&path, circle_color);
                }
            };

            // Top-Left Glow (Aurora)
            let tl_center = Point::new(bounds.width * 0.1, bounds.height * 0.1);
            let tl_radius = bounds.width.max(bounds.height) * 0.5;
            draw_glow(&mut frame, tl_center, tl_radius, self.glow_color);

            // Top-Right Glow (Aurora)
            let tr_center = Point::new(bounds.width * 0.9, bounds.height * 0.1);
            let tr_radius = bounds.width.max(bounds.height) * 0.5;
            draw_glow(&mut frame, tr_center, tr_radius, self.glow_color);
        }

        vec![frame.into_geometry()]
    }
}

/// A background widget that fills the area with a color (supports transparency)
/// and optionally draws the cyber aurora glows in the top corners.
pub fn background<'a, Message: 'static>(
    content: impl Into<Element<'a, Message>>,
    bg_color: Color,
    glow_color: Color,
    draw_glows: bool,
) -> Element<'a, Message> {
    let bg_program = BackgroundProgram {
        bg_color,
        glow_color,
        draw_glows,
    };

    stack![
        canvas(bg_program)
            .width(Length::Fill)
            .height(Length::Fill),
        container(content.into())
            .width(Length::Fill)
            .height(Length::Fill)
    ]
    .into()
}
