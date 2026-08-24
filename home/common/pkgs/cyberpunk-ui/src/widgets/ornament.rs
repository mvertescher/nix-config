//! Kitsch's page-curl, the one solid flourish the era allows itself.
//!
//! `docs/kitsch/target-components.svg` states the rule outright -- "one
//! solid page-curl per screen" -- and the store spends it at the foot
//! of the nav column, where the container's outline stops being an
//! outline and becomes a filled swoosh with a rule running off its
//! shoulder. The fill is [`crate::style::Style::ornament`], which is
//! exactly the role that colour was added for.

use crate::style::Style;
use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

/// The curl's own coordinate box, from
/// `M142 736 q2 -88 88 -88 h28 q-56 12 -64 62 q-7 44 -52 50 Z` with its
/// origin moved to the top-left of the shape's bounds.
const CURL_W: f32 = 116.0;
const CURL_H: f32 = 112.0;
/// Where the rule leaves the curl's shoulder, in the same units.
const SHOULDER: f32 = 88.0;

#[derive(Debug, Clone, Copy)]
struct PageCurl {
    pub fill: Color,
    pub rule: Color,
    pub stroke: f32,
}

impl<Message> canvas::Program<Message> for PageCurl {
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
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }

        let scale = h / CURL_H;
        let curl = canvas::Path::new(|b| {
            let p = |x: f32, y: f32| Point::new(x * scale, y * scale);
            b.move_to(p(0.0, SHOULDER));
            b.quadratic_curve_to(p(2.0, 0.0), p(88.0, 0.0));
            b.line_to(p(116.0, 0.0));
            b.quadratic_curve_to(p(60.0, 12.0), p(52.0, 62.0));
            b.quadratic_curve_to(p(45.0, 106.0), p(0.0, 112.0));
            b.close();
        });
        frame.fill(&curl, self.fill);

        // The rule carries on from the curl's shoulder to the far edge
        // of the column, which is what makes the swoosh read as the end
        // of a container rather than a sticker.
        let x0 = CURL_W * scale;
        if w > x0 {
            let y = SHOULDER * scale;
            let rule = canvas::Path::new(|b| {
                b.move_to(Point::new(x0, y));
                b.line_to(Point::new(w, y));
            });
            frame.stroke(
                &rule,
                canvas::Stroke::default()
                    .with_color(self.rule)
                    .with_width(self.stroke),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// The rule down the leading edge of a column, which the page-curl at
/// its foot then becomes.
///
/// In `docs/kitsch/target-app.svg` the curl is not a sticker sitting
/// under the nav; it is the end of the nav container's outline. The two
/// are one gesture drawn as two paths -- `M140 306 v396 q0 34 34 34
/// h120` for the container and `M142 736 q2 -88 88 -88 h28 ...` for the
/// solid -- and drawing only the second is what left the curl floating.
/// This is the first: a hairline at `x = 140` from just under the
/// customer block to where the curl takes over.
#[derive(Debug, Clone, Copy)]
struct ColumnRule {
    pub color: Color,
    pub stroke: f32,
}

impl<Message> canvas::Program<Message> for ColumnRule {
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
        let h = super::surface::visible(bounds.y, bounds.height);
        if h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        let x = self.stroke / 2.0;
        let rule = canvas::Path::new(|b| {
            b.move_to(Point::new(x, 0.0));
            b.line_to(Point::new(x, h));
        });
        frame.stroke(
            &rule,
            canvas::Stroke::default()
                .with_color(self.color)
                .with_width(self.stroke),
        );
        vec![frame.into_geometry()]
    }
}

/// The column rule, filling whatever height it is handed.
pub fn column_rule<'a, Message: 'static>(style: &Style) -> Element<'a, Message> {
    canvas(ColumnRule {
        color: style.palette.border,
        stroke: style.metrics.stroke,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// The page-curl, `height` tall, with its rule running to the right
/// edge of whatever width it is offered.
pub fn page_curl<'a, Message: 'static>(
    style: &Style,
    height: f32,
) -> Element<'a, Message> {
    canvas(PageCurl {
        fill: style.ornament(),
        rule: style.palette.border,
        stroke: style.metrics.stroke,
    })
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .into()
}
