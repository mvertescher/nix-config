//! Top and bottom furniture.
//!
//! All four eras frame every screen with something; they disagree about
//! what. Entropism runs a row of outlined boxes over a build-string
//! footer, neomil a thin rule with a hostname tape, kitsch a single
//! centred compliance caption, neokitsch the stepped double-stroke
//! device frame itself with a strata wedge at its foot.
//! [`crate::style::Chrome`] picks; the screens never ask.

use super::surface::{surface, Surface};
use super::text;
use crate::style::{Chrome, Style};
use crate::Element;
use iced::widget::{canvas, column, container, row, rule, Space};
use iced::{mouse, Color, Length, Padding, Point, Rectangle, Renderer};

/// Height of the stepped device-frame rail at the top and bottom of a
/// neokitsch screen.
const RAIL: f32 = 46.0;
/// Length of the flush corner section between the tab and the step.
const RAIL_CORNER: f32 = 220.0;
/// Horizontal run of the step diagonal.
const RAIL_STEP_W: f32 = 22.0;
/// Vertical rise of the step diagonal.
const RAIL_STEP_H: f32 = 16.0;
/// Corner radius of the outer stroke's tab.
const RAIL_RADIUS: f32 = 16.0;
/// Inset of the inner stroke from the outer one.
const RAIL_INSET: f32 = 6.0;
/// Width of the outer stroke.
const RAIL_OUTER: f32 = 2.4;

/// The layered fine lines neokitsch closes every screen with, bunching
/// into a wedge at one end.
#[derive(Debug, Clone, Copy)]
struct Strata {
    pub color: Color,
    pub lines: usize,
}

impl<Message> canvas::Program<Message, Style> for Strata {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Style,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        let step = if self.lines > 1 {
            h / self.lines as f32
        } else {
            h
        };

        for i in 0..self.lines {
            let t = i as f32 / self.lines.max(1) as f32;
            let y = i as f32 * step;
            // Each line starts further in and fades, so the stack reads
            // as a wedge rather than a ruled block.
            let x0 = w * 0.012 * i as f32;
            let path = canvas::Path::new(|b| {
                b.move_to(Point::new(x0, y));
                b.line_to(Point::new(w - x0 * 0.5, y));
            });
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(Color {
                        a: 1.0 - t * 0.8,
                        ..self.color
                    })
                    .with_width(1.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// Which edge of the screen a [`DeviceRail`] band dresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailSide {
    Top,
    Bottom,
}

/// One band of neokitsch's device frame: the double gold stroke with
/// the middle section stepped out and a tab at each corner.
///
/// The geometry is `docs/neokitsch/target-app.svg` (deleted 2026-09-03:
/// the full-screen frame was that sheet's invention -- the photo's only
/// frame is the wire band `store-trace.svg` draws. Still worn by the
/// widget-based dashboard and the bar's mail panel until the widget
/// decision in `TODO.md`):
/// `path d="M150 120 h300 l24 -20 h972 l24 20 h300 q30 0 30 30 v760
/// q0 30 -30 30 h-300 l-24 20 h-972 l-24 -20 h-300 ..."` -- the
/// corner sections sit below the top rail and above the bottom rail,
/// the steps are 24 wide by 20 tall, and the outer stroke is a gold
/// that reads lit at the top. This band is that path truncated to the
/// screen edge, so the tall vertical sides collapse into the corner
/// tabs; the "outer" stroke is [`crate::palette::Ornaments::relief`]'s
/// lit side on the top rail and the flat frame gold on the bottom one,
/// and the "inner" stroke is its shaded side (`FRAME_INNER`).
#[derive(Debug, Clone, Copy)]
struct DeviceRail {
    side: RailSide,
    /// The lit edge of the bevel: `FRAME_LIT`.
    lit: Color,
    /// The flat frame gold: `FRAME`.
    gold: Color,
    /// The shaded inner stroke: `FRAME_INNER`.
    shade: Color,
    /// The era's stroke weight, for the inner line.
    stroke: f32,
}

impl DeviceRail {
    fn new(style: &Style, side: RailSide) -> Self {
        let (lit, shade) = style.relief();
        DeviceRail {
            side,
            lit,
            gold: style.palette.border,
            shade,
            stroke: style.metrics.stroke,
        }
    }
}

/// The band's outer or inner stroke, as one open path: up the left
/// tab, over the rounded corner, along the corner section, up the step
/// into the raised middle, down the far step, along the far corner
/// section, over that corner and down the right tab.
///
/// `corner_y` is the flush corner-section line and `mid_y` the raised
/// middle line; the two are `RAIL_STEP_H` apart, 4px from the band's
/// own edge so the step never clips.
fn rail_path(
    w: f32,
    h: f32,
    corner_y: f32,
    mid_y: f32,
    radius: f32,
    corner: f32,
    hangs: bool,
) -> canvas::Path {
    let tab = 24.0;
    let x1 = tab + corner;
    let x2 = x1 + RAIL_STEP_W;
    let x3 = w - x2;
    let x4 = w - tab - corner;
    let l_tab = tab - radius;
    let r_tab = w - tab + radius;

    canvas::Path::new(|b| {
        if hangs {
            // The top rail: the tabs hang below the corner line, so
            // the path starts at a tab's foot in the band's margin.
            b.move_to(Point::new(l_tab, h));
            b.line_to(Point::new(l_tab, corner_y + radius));
            b.quadratic_curve_to(Point::new(l_tab, corner_y), Point::new(tab, corner_y));
            b.line_to(Point::new(x1, corner_y));
            b.line_to(Point::new(x2, mid_y));
            b.line_to(Point::new(x3, mid_y));
            b.line_to(Point::new(x4, corner_y));
            b.line_to(Point::new(w - tab, corner_y));
            b.quadratic_curve_to(Point::new(r_tab, corner_y), Point::new(r_tab, corner_y + radius));
            b.line_to(Point::new(r_tab, h));
        } else {
            // The bottom rail: the tabs stand above the corner line,
            // so the path starts at a tab's head at the band's top.
            b.move_to(Point::new(l_tab, 0.0));
            b.line_to(Point::new(l_tab, corner_y - radius));
            b.quadratic_curve_to(Point::new(l_tab, corner_y), Point::new(tab, corner_y));
            b.line_to(Point::new(x1, corner_y));
            b.line_to(Point::new(x2, mid_y));
            b.line_to(Point::new(x3, mid_y));
            b.line_to(Point::new(x4, corner_y));
            b.line_to(Point::new(w - tab, corner_y));
            b.quadratic_curve_to(Point::new(r_tab, corner_y), Point::new(r_tab, corner_y - radius));
            b.line_to(Point::new(r_tab, 0.0));
        }
    })
}

impl<Message> canvas::Program<Message, Style> for DeviceRail {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Style,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);

        let (outer, inner) = match self.side {
            // The top rail is the lit side of the bevel; the bottom one
            // is the flat frame gold, as the target's stroke gradient
            // runs lit at the top to darker at the foot.
            RailSide::Top => (self.lit, self.shade),
            RailSide::Bottom => (self.gold, self.shade),
        };

        let (corner_y, mid_y) = match self.side {
            // The middle section steps out of the band -- up at the top
            // of the screen, down at the foot -- leaving 4px of the
            // band as clearance on the raised side and a 10px tab
            // between the corner line and the band edge on the other.
            RailSide::Top => (20.0, 20.0 - RAIL_STEP_H),
            RailSide::Bottom => (h - 20.0, h - 20.0 + RAIL_STEP_H),
        };

        // The inner stroke is inset toward the frame's interior: below
        // the outer line at the top, above it at the foot.
        let (inner_corner_y, inner_mid_y) = match self.side {
            RailSide::Top => (corner_y + RAIL_INSET, mid_y + RAIL_INSET),
            RailSide::Bottom => (corner_y - RAIL_INSET, mid_y - RAIL_INSET),
        };

        frame.stroke(
            &rail_path(
                w,
                h,
                corner_y,
                mid_y,
                RAIL_RADIUS,
                RAIL_CORNER,
                self.side == RailSide::Top,
            ),
            canvas::Stroke::default()
                .with_color(outer)
                .with_width(RAIL_OUTER),
        );
        frame.stroke(
            &rail_path(
                w,
                h,
                inner_corner_y,
                inner_mid_y,
                RAIL_RADIUS - RAIL_INSET,
                RAIL_CORNER - RAIL_INSET * 2.0,
                self.side == RailSide::Top,
            ),
            canvas::Stroke::default()
                .with_color(inner)
                .with_width(self.stroke),
        );

        vec![frame.into_geometry()]
    }
}

/// The header for the current era. `segments` is used by the eras that
/// have a segmented bar and ignored by the ones that do not.
pub fn top_bar<'a, Message: 'static>(
    style: &Style,
    segments: [&'a str; 3],
) -> Element<'a, Message> {
    match style.chrome {
        Chrome::Segmented => {
            // `fill="#728f76"` in the entropism target -- the ink
            // between `dim` and `fg`, not `dim` itself. At 14pt on that
            // ground `dim` measures 2.1:1, which is what made the
            // segmented bar read as three empty boxes.
            let boxed = |label: &'a str, width: Length, center: bool| {
                let inner: Element<'a, Message> = if center {
                    container(text::mid(style, label))
                        .center_x(Length::Fill)
                        .into()
                } else {
                    text::mid(style, label).into()
                };
                container(surface(
                    Surface::outlined(style),
                    Padding::from([4, 12]),
                    inner,
                ))
                .width(width)
                .height(Length::Fixed(28.0))
            };
            row![
                boxed(segments[0], Length::FillPortion(3), false),
                boxed(segments[1], Length::FillPortion(7), true),
                boxed(segments[2], Length::FillPortion(2), false),
            ]
            .spacing(6)
            .into()
        }
        Chrome::Tape => row![
            container(surface(
                Surface::filled(style, style.palette.tape).no_stroke(),
                Padding::from([3, 10]),
                text::body(style, segments[0]).color(style.palette.bg),
            ))
            .width(Length::Fixed(320.0))
            .height(Length::Fixed(24.0)),
            Space::new().width(12.0),
            container(text::label(style, segments[1])).center_y(Length::Fixed(24.0)),
            Space::new().width(Length::Fill).height(Length::Shrink),
            container(text::label(style, segments[2])).center_y(Length::Fixed(24.0)),
        ]
        .into(),
        // Kitsch puts nothing at the top; neokitsch opens with the
        // device frame's top rail, the meta line beneath it.
        Chrome::Caption => row![
            text::caption(style, segments[0]),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text::caption(style, segments[2]),
        ]
        .into(),
        Chrome::DeviceFrame => column![
            canvas(DeviceRail::new(style, RailSide::Top))
                .width(Length::Fill)
                .height(Length::Fixed(RAIL)),
            row![
                text::caption(style, segments[0]),
                Space::new().width(Length::Fill).height(Length::Shrink),
                text::caption(style, segments[2]),
            ]
            .padding(Padding::from([2, 0])),
        ]
        .into(),
    }
}

/// The footer for the current era.
pub fn footer<'a, Message: 'static>(
    style: &Style,
    left: &'a str,
    middle: &'a str,
    right: &'a str,
) -> Element<'a, Message> {
    // The build-string footer is `fill="#728f76"` in the entropism
    // target, same as the segmented bar above it: small print, but not
    // the tertiary ink the footnote bodies are set in.
    let line = row![
        text::mid(style, left).size(f32::from(style.metrics.text_caption + 2)),
        Space::new().width(Length::Fill).height(Length::Shrink),
        text::mid(style, middle).size(f32::from(style.metrics.text_caption + 2)),
        Space::new().width(Length::Fill).height(Length::Shrink),
        text::mid(style, right).size(f32::from(style.metrics.text_caption + 2)),
    ];

    match style.chrome {
        Chrome::Segmented => column![
            // The era's divider, from `catalog::divider`.
            rule::horizontal(1),
            container(line).padding(Padding::from([8, 0])),
        ]
        .into(),
        Chrome::DeviceFrame => column![
            // The strata wedge sits inside the frame, above its foot
            // rail, as the (deleted) `target-app.svg` composite had it: a
            // five-line wedge at `y=856..884` against a frame foot at
            // `y=940`.
            canvas(Strata {
                color: style.ornament(),
                lines: 5,
            })
            .width(Length::Fill)
            .height(Length::Fixed(30.0)),
            container(line).padding(Padding::from([4, 0])),
            canvas(DeviceRail::new(style, RailSide::Bottom))
                .width(Length::Fill)
                .height(Length::Fixed(RAIL)),
        ]
        .into(),
        Chrome::Caption => container(
            row![
                text::caption(style, middle)
                    .size(f32::from(style.metrics.text_caption + 2))
                    .color(style.palette.fg),
            ]
            .spacing(6),
        )
        .center_x(Length::Fill)
        .into(),
        Chrome::Tape => container(line).padding(Padding::from([8, 0])).into(),
    }
}
