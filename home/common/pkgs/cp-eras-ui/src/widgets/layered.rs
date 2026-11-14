//! A canvas whose geometries are drawn in separate layers.
//!
//! iced's [`Canvas`](iced::widget::Canvas) draws every geometry its
//! program returns into the current layer, and a wgpu layer batches all
//! of its meshes, then all of its images, then all of its text. So an
//! image -- the neo-kitsch veneer, [`crate::veneer`] -- lands above every
//! mesh of its canvas, however late in the paint order those come: the
//! socket rules, envelopes, QR cells and outlines that sit *on* a veneer
//! would vanish under it.
//!
//! This is the same widget with one change: geometry `0` is drawn into
//! the current layer, as the stock canvas would, and each geometry after
//! it opens a layer of its own over the previous ones. A program that
//! returns one geometry -- every scene without a veneer -- is drawn
//! exactly as by the stock canvas. `screens/scene.rs`'s `Pass` and the
//! mailbox's `Sheet` decide what goes in which.
//!
//! Everything else (state, events, redraw requests, mouse interaction)
//! follows `iced_widget` 0.14's `canvas.rs` line for line.

use iced::advanced::graphics::geometry;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Shell, Widget};
use iced::widget::canvas::Program;
use iced::{event, mouse, window, Element, Event, Length, Rectangle, Size, Vector};
use std::marker::PhantomData;

/// A [`Canvas`](iced::widget::Canvas) that layers its geometries.
pub struct Layered<P, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
    P: Program<Message, Theme, Renderer>,
{
    width: Length,
    height: Length,
    program: P,
    last_mouse_interaction: Option<mouse::Interaction>,
    _marker: PhantomData<(Message, Theme, Renderer)>,
}

/// A layered canvas of `program`, 100x100 until sized, as `canvas()`.
pub fn canvas<P, Message, Theme, Renderer>(program: P) -> Layered<P, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
    P: Program<Message, Theme, Renderer>,
{
    Layered {
        width: Length::Fixed(100.0),
        height: Length::Fixed(100.0),
        program,
        last_mouse_interaction: None,
        _marker: PhantomData,
    }
}

impl<P, Message, Theme, Renderer> Layered<P, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
    P: Program<Message, Theme, Renderer>,
{
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<P, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Layered<P, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
    P: Program<Message, Theme, Renderer>,
{
    fn tag(&self) -> tree::Tag {
        struct Tag<T>(T);
        tree::Tag::of::<Tag<P::State>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(P::State::default())
    }

    fn size(&self) -> Size<Length> {
        Size { width: self.width, height: self.height }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<P::State>();
        let is_redraw_request = matches!(event, Event::Window(window::Event::RedrawRequested(_now)));

        if let Some(action) = self.program.update(state, event, bounds, cursor) {
            let (message, redraw_request, event_status) = action.into_inner();
            shell.request_redraw_at(redraw_request);
            if let Some(message) = message {
                shell.publish(message);
            }
            if event_status == event::Status::Captured {
                shell.capture_event();
            }
        }

        if shell.redraw_request() != window::RedrawRequest::NextFrame {
            let mouse_interaction = self.mouse_interaction(tree, layout, cursor, viewport, renderer);
            if is_redraw_request {
                self.last_mouse_interaction = Some(mouse_interaction);
            } else if self.last_mouse_interaction.is_some_and(|last| last != mouse_interaction) {
                shell.request_redraw();
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<P::State>();
        self.program.mouse_interaction(state, layout.bounds(), cursor)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }
        let state = tree.state.downcast_ref::<P::State>();
        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            let local = Rectangle::with_size(bounds.size());
            for (i, geometry) in self.program.draw(state, renderer, theme, bounds, cursor).into_iter().enumerate() {
                if i == 0 {
                    renderer.draw_geometry(geometry);
                } else {
                    renderer.with_layer(local, |renderer| renderer.draw_geometry(geometry));
                }
            }
        });
    }
}

impl<'a, P, Message, Theme, Renderer> From<Layered<P, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + geometry::Renderer,
    P: 'a + Program<Message, Theme, Renderer>,
{
    fn from(canvas: Layered<P, Message, Theme, Renderer>) -> Self {
        Element::new(canvas)
    }
}
