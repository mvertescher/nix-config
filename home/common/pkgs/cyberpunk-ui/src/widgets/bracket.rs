//! The bracketed panel kitsch puts its customer block in.
//!
//! Three eras set the customer/loyalty/last-update block as bare text.
//! Kitsch encloses its first row in a rounded outline and hangs two
//! curved tails off the right of it -- `M430 220 q30 0 30 30` and
//! `M390 226 q46 0 52 46` against a panel at `x=140 y=212 w=250 h=30`
//! -- which is the same gesture as the nav container's outline running
//! into the page-curl, played at a smaller size.
//!
//! It is a [`crate::style::Chrome`] decision rather than a role: the
//! era that draws it is the era that has no top bar to hang the meta
//! block under, and the outline follows the era's own [`Corner`].

use super::surface::{default_corners, layered, outline, Surface};
use crate::style::{Corner, Style};
use iced::widget::{canvas, column, container, row, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Theme};

/// The reference panel's coordinate box: the outline is 250 wide of the
/// 320 the tails reach to, and 30 high.
const PANEL_W: f32 = 250.0;
const REACH: f32 = 320.0;

#[derive(Debug, Clone, Copy)]
struct Bracket {
    pub color: Color,
    pub stroke: f32,
    pub corner: Corner,
    /// Height of the enclosed head row.
    pub head: f32,
}

impl<Message> canvas::Program<Message> for Bracket {
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

        let stroke = canvas::Stroke::default()
            .with_color(self.color)
            .with_width(self.stroke);
        let s = w / REACH;
        let panel_w = PANEL_W * s;
        let i = self.stroke / 2.0;

        // The outline takes the era's corner treatment, so this is the
        // same rounded box a nav pill is and not a second idea of what
        // a container looks like.
        frame.translate(iced::Vector::new(i, i));
        let box_ = outline(
            self.corner,
            default_corners(self.corner),
            crate::style::Ticket::default(),
            (panel_w - self.stroke).max(0.0),
            (self.head - self.stroke).max(0.0),
        );
        frame.stroke(&box_, stroke);
        frame.translate(iced::Vector::new(-i, -i));

        // Two tails, scaled off the panel rather than the head row: they
        // are a fixed gesture in the reference, not a function of how
        // tall the text turned out.
        let p = |x: f32, y: f32| Point::new(x * s, y * s);
        let tails = canvas::Path::new(|b| {
            b.move_to(p(290.0, 8.0));
            b.quadratic_curve_to(p(320.0, 8.0), p(320.0, 38.0));
            b.move_to(p(250.0, 14.0));
            b.quadratic_curve_to(p(296.0, 14.0), p(302.0, 60.0));
        });
        frame.stroke(&tails, stroke);

        vec![frame.into_geometry()]
    }
}

/// `head` inside a bracketed outline, with `rest` hanging below it.
///
/// The tails need room to the right of the outline, so the content is
/// held back from the trailing edge by the same proportion the
/// reference uses.
pub fn bracket_panel<'a, Message: 'static>(
    style: &Style,
    head: Element<'a, Message>,
    rest: Element<'a, Message>,
) -> Element<'a, Message> {
    let head_h = style.metrics.text_body as f32 + 12.0;
    let pad = style.corner.inset().clamp(8.0, 14.0);

    let content = row![
        column![
            container(head)
                .padding(Padding::from([0.0, pad]))
                .height(Length::Fixed(head_h))
                .center_y(Length::Fixed(head_h))
                .width(Length::Fill),
            container(rest)
                .padding(Padding {
                    top: 6.0,
                    right: pad,
                    bottom: 0.0,
                    left: pad,
                })
                .width(Length::Fill),
        ]
        .width(Length::FillPortion((PANEL_W as u16) / 10)),
        Space::new(
            Length::FillPortion(((REACH - PANEL_W) as u16) / 10),
            Length::Shrink
        ),
    ];

    layered(
        canvas(Bracket {
            color: Surface::outlined(style).stroke.unwrap_or(style.palette.border),
            stroke: style.metrics.stroke,
            corner: style.corner,
            head: head_h,
        })
        .width(Length::Fill)
        .height(Length::Fill),
        content,
    )
}
