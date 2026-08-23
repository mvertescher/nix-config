//! The one primitive every era's screens are built from.
//!
//! A surface is a filled and/or stroked shape whose corner treatment
//! comes from [`Corner`] and whose fill comes from [`Fill`]. Panels,
//! cards, nav pills, badges and list rows are all this widget with
//! different sizes and fills, which is what lets the store screen have a
//! single implementation across four eras.

use crate::style::{Corner, Selection, Style};
use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{layout, overlay, renderer, Clipboard, Layout, Shell, Widget};
use iced::widget::{canvas, container, stack};
use iced::{event, mouse, Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

/// Which corners a chamfer applies to. Neo-militarism cuts different
/// corners on different widgets, so the shape is a parameter rather
/// than a constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corners {
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_right: bool,
    pub bottom_left: bool,
}

impl Corners {
    pub const ALL: Corners = Corners {
        top_left: true,
        top_right: true,
        bottom_right: true,
        bottom_left: true,
    };
    pub const NONE: Corners = Corners {
        top_left: false,
        top_right: false,
        bottom_right: false,
        bottom_left: false,
    };
    /// The neo-militarism default: cut the bottom-right only.
    pub const BOTTOM_RIGHT: Corners = Corners {
        bottom_right: true,
        ..Corners::NONE
    };
    pub const TOP_RIGHT: Corners = Corners {
        top_right: true,
        ..Corners::NONE
    };
    /// Diagonally opposite cuts, as on the neomil info panel.
    pub const OPPOSED: Corners = Corners {
        top_right: true,
        bottom_left: true,
        ..Corners::NONE
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
    None,
    Solid(Color),
    /// Neokitsch's selection material. Synthesised rather than shipped:
    /// a base tone, banded warp, and grain lines clipped to the shape.
    Veneer {
        base: Color,
        light: Color,
        dark: Color,
        grain: Color,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub corner: Corner,
    pub corners: Corners,
    pub fill: Fill,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
}

impl Surface {
    /// An outlined surface in the era's border colour.
    pub fn outlined(style: &Style) -> Self {
        Surface {
            corner: style.corner,
            corners: default_corners(style.corner),
            fill: Fill::None,
            stroke: Some(style.palette.border),
            stroke_width: style.metrics.stroke,
        }
    }

    /// A filled surface in an arbitrary colour, unstroked.
    pub fn filled(style: &Style, color: Color) -> Self {
        Surface {
            corner: style.corner,
            corners: default_corners(style.corner),
            fill: Fill::Solid(color),
            stroke: None,
            stroke_width: style.metrics.stroke,
        }
    }

    /// The selected state, in whatever way this era expresses selection.
    /// This is the call that keeps `Selection::Veneer` from leaking into
    /// every widget as an era check.
    pub fn selected(style: &Style) -> Self {
        let fill = match style.selection {
            Selection::Solid => Fill::Solid(style.palette.select),
            Selection::Veneer => Fill::Veneer {
                base: crate::eras::neokitsch::VENEER,
                light: crate::eras::neokitsch::VENEER_LIGHT,
                dark: crate::eras::neokitsch::VENEER_DARK,
                grain: crate::eras::neokitsch::GRAIN,
            },
        };
        Surface {
            corner: style.corner,
            corners: default_corners(style.corner),
            fill,
            stroke: None,
            stroke_width: style.metrics.stroke,
        }
    }

    pub fn corners(mut self, corners: Corners) -> Self {
        self.corners = corners;
        self
    }

    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = Some(color);
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke = None;
        self
    }
}

/// Which corners an era treats by default. Only neo-militarism varies
/// them per widget; the rest apply their treatment uniformly.
pub fn default_corners(corner: Corner) -> Corners {
    match corner {
        Corner::Square => Corners::NONE,
        Corner::Chamfer { .. } => Corners::BOTTOM_RIGHT,
        Corner::Round { .. } => Corners::ALL,
        Corner::ClipTopRight { .. } => Corners::TOP_RIGHT,
    }
}

/// The outline of a surface, as a canvas path.
pub fn outline(corner: Corner, corners: Corners, w: f32, h: f32) -> canvas::Path {
    let amount = corner.inset().min(w / 2.0).min(h / 2.0);

    canvas::Path::new(|b| {
        match corner {
            Corner::Round { .. } => {
                let r = amount;
                // Quadratic corners rather than arcs: visually identical
                // at this radius and one less API surface to depend on.
                b.move_to(Point::new(r, 0.0));
                b.line_to(Point::new(w - r, 0.0));
                b.quadratic_curve_to(Point::new(w, 0.0), Point::new(w, r));
                b.line_to(Point::new(w, h - r));
                b.quadratic_curve_to(Point::new(w, h), Point::new(w - r, h));
                b.line_to(Point::new(r, h));
                b.quadratic_curve_to(Point::new(0.0, h), Point::new(0.0, h - r));
                b.line_to(Point::new(0.0, r));
                b.quadratic_curve_to(Point::new(0.0, 0.0), Point::new(r, 0.0));
            }
            _ => {
                // Square, chamfer and clip-top-right are all the same
                // walk with per-corner cuts; square just cuts nothing.
                let c = if matches!(corner, Corner::Square) {
                    0.0
                } else {
                    amount
                };
                let tl = if corners.top_left { c } else { 0.0 };
                let tr = if corners.top_right { c } else { 0.0 };
                let br = if corners.bottom_right { c } else { 0.0 };
                let bl = if corners.bottom_left { c } else { 0.0 };

                b.move_to(Point::new(tl, 0.0));
                b.line_to(Point::new(w - tr, 0.0));
                if tr > 0.0 {
                    b.line_to(Point::new(w, tr));
                }
                b.line_to(Point::new(w, h - br));
                if br > 0.0 {
                    b.line_to(Point::new(w - br, h));
                }
                b.line_to(Point::new(bl, h));
                if bl > 0.0 {
                    b.line_to(Point::new(0.0, h - bl));
                }
                b.line_to(Point::new(0.0, tl));
                if tl > 0.0 {
                    b.line_to(Point::new(tl, 0.0));
                }
            }
        }
        b.close();
    })
}

/// The shape's horizontal extent at height `y`, used to clip grain lines
/// to a surface without needing path clipping in the renderer.
///
/// All four corner treatments are convex and axis-aligned apart from a
/// single corner, so this is exact rather than an approximation.
pub fn span_at(corner: Corner, corners: Corners, w: f32, h: f32, y: f32) -> (f32, f32) {
    let amount = corner.inset().min(w / 2.0).min(h / 2.0);
    if amount <= 0.0 || y < 0.0 || y > h {
        return (0.0, w);
    }

    let (mut x0, mut x1) = (0.0f32, w);

    // How far in the edge is drawn at `y`, for a corner of the given
    // treatment sitting `d` away from the shape's end.
    let inward = |d: f32| -> f32 {
        if d >= amount {
            return 0.0;
        }
        match corner {
            Corner::Round { .. } => {
                let dy = amount - d;
                amount - (amount * amount - dy * dy).max(0.0).sqrt()
            }
            _ => amount - d,
        }
    };

    if corners.top_left {
        x0 = x0.max(inward(y));
    }
    if corners.bottom_left {
        x0 = x0.max(inward(h - y));
    }
    if corners.top_right {
        x1 = x1.min(w - inward(y));
    }
    if corners.bottom_right {
        x1 = x1.min(w - inward(h - y));
    }
    (x0, x1)
}

impl<Message> canvas::Program<Message> for Surface {
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

        // A stroke straddles its path, so a shape built flush to the
        // canvas bounds loses the outer half of its outline to
        // clipping -- visible as a box missing its right and bottom
        // edges. Inset the path by half the stroke so the whole line
        // lands inside.
        let inset = match self.stroke {
            Some(_) => self.stroke_width / 2.0,
            None => 0.0,
        };
        let (pw, ph) = (w - inset * 2.0, h - inset * 2.0);
        if pw <= 0.0 || ph <= 0.0 {
            return vec![frame.into_geometry()];
        }
        frame.translate(iced::Vector::new(inset, inset));

        let path = outline(self.corner, self.corners, pw, ph);

        match self.fill {
            Fill::None => {}
            Fill::Solid(color) => frame.fill(&path, color),
            Fill::Veneer {
                base,
                light,
                dark,
                grain,
            } => {
                frame.fill(&path, base);

                // Warp: broad bands across the grain direction, alpha-
                // blended so the plank reads as figured rather than flat.
                let bands = 7;
                for i in 0..bands {
                    let t = i as f32 / bands as f32;
                    let y0 = t * ph;
                    let band_h = ph / bands as f32;
                    let tone = if i % 2 == 0 { light } else { dark };
                    let (x0, x1) = span_at(self.corner, self.corners, pw, ph, y0 + band_h / 2.0);
                    if x1 <= x0 {
                        continue;
                    }
                    let band = canvas::Path::rectangle(
                        Point::new(x0, y0),
                        iced::Size::new(x1 - x0, band_h),
                    );
                    frame.fill(
                        &band,
                        Color {
                            a: 0.16,
                            ..tone
                        },
                    );
                }

                // Grain: fine lines along the plank, clipped to the
                // shape by span rather than by renderer clipping.
                let mut y = 3.0;
                let mut n = 0;
                while y < ph {
                    let (x0, x1) = span_at(self.corner, self.corners, pw, ph, y);
                    if x1 > x0 {
                        let line = canvas::Path::new(|b| {
                            b.move_to(Point::new(x0 + 1.0, y));
                            b.line_to(Point::new(x1 - 1.0, y));
                        });
                        frame.stroke(
                            &line,
                            canvas::Stroke::default()
                                .with_color(Color {
                                    a: if n % 3 == 0 { 0.30 } else { 0.16 },
                                    ..grain
                                })
                                .with_width(if n % 3 == 0 { 0.9 } else { 0.6 }),
                        );
                    }
                    y += 5.0;
                    n += 1;
                }
            }
        }

        if let Some(color) = self.stroke {
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_color(color)
                    .with_width(self.stroke_width),
            );
        }

        vec![frame.into_geometry()]
    }
}

/// Wrap `content` in a surface. The canvas sits behind rather than
/// clipping, so text keeps its own layout and the shape stays cheap.
pub fn surface<'a, Message: 'static>(
    surface: Surface,
    padding: impl Into<iced::Padding>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    stack![
        canvas(surface).width(Length::Fill).height(Length::Fill),
        container(content.into()).padding(padding).width(Length::Fill)
    ]
    .into()
}

/// Wrap `content` in a surface that takes its size from the content.
///
/// [`surface`] paints the box it is handed, which is what a caller who
/// knows the box wants -- a bar cell, a nav pill, a list row. It is the
/// wrong way round when the content is what should decide. The canvas
/// is the first layer of a `stack` and a `stack` takes its size from its
/// first layer, so a surface whose caller does not pin a height grows to
/// whatever space it is offered: the same failure that made bar cells
/// clip their own labels, seen from the other side.
///
/// A [`Backdrop`] inverts that. The content lays out first, against the
/// limits the parent gave, and the surface is then laid out to exactly
/// the size that came back.
pub fn backdrop<'a, Message: 'static>(
    surface: Surface,
    padding: impl Into<iced::Padding>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Backdrop {
        children: vec![
            canvas(surface).width(Length::Fill).height(Length::Fill).into(),
            container(content.into())
                .padding(padding)
                .width(Length::Fill)
                .into(),
        ],
    }
    .into()
}

/// Two layers, sized by the second.
///
/// Deliberately not a `stack` with the arguments swapped: the layer that
/// decides the size has to be drawn *last*, or the fill covers the text.
/// `stack` ties those two together -- its first child both sizes the
/// widget and sits at the bottom -- and this is the one place that needs
/// them apart.
struct Backdrop<'a, Message> {
    /// `[background, content]`, in draw order. `content` is the sizer.
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Backdrop<'a, Message> {
    const CONTENT: usize = 1;
}

impl<Message> Widget<Message, Theme, Renderer> for Backdrop<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        self.children[Self::CONTENT].as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self.children[Self::CONTENT].as_widget().layout(
            &mut tree.children[Self::CONTENT],
            renderer,
            limits,
        );
        let size = content.size();

        // Min and max are the same, so the background's `Length::Fill`
        // resolves to the content's size rather than to the parent's.
        let background = self.children[0].as_widget().layout(
            &mut tree.children[0],
            renderer,
            &layout::Limits::new(size, size),
        );

        layout::Node::with_children(size, vec![background, content])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            for ((child, state), layout) in self
                .children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child.as_widget().operate(state, layout, renderer, operation);
            }
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // Topmost layer first, as `stack` does: the content is in front.
        self.children
            .iter_mut()
            .rev()
            .zip(tree.children.iter_mut().rev())
            .zip(layout.children().rev())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .find(|&status| status == event::Status::Captured)
            .unwrap_or(event::Status::Ignored)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .rev()
            .zip(tree.children.iter().rev())
            .zip(layout.children().rev())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .find(|&interaction| interaction != mouse::Interaction::None)
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer, translation)
    }
}

impl<'a, Message: 'a> From<Backdrop<'a, Message>> for Element<'a, Message> {
    fn from(backdrop: Backdrop<'a, Message>) -> Self {
        Element::new(backdrop)
    }
}
