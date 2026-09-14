//! Native controls with component-sheet material backdrops.
//!
//! The native widget is the sole child throughout rest, hover and focus;
//! its cursor, selection, IME, IDs, operations and messages remain native.
//! Constructors install the supplied native callback; None disables the
//! control. The wrapper never publishes activation messages.
use std::{cell::Cell, rc::Rc};
use crate::{catalog, Element, Style};
use crate::eras::control_materials::{material, Material, Shape};
use super::surface::{Fill, SurfaceFace};
use iced::advanced::{layout, overlay, renderer, widget::{self, Operation, Tree}, Clipboard, Layout, Shell, Widget};
use iced::{mouse, Event, Length, Rectangle, Renderer, Size};
use iced::widget::{button as native_button, text_input, canvas};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind { Primary, Ghost, Field }
/// `Pressed` means focused for a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State { Active, Hovered, Pressed, Disabled }

pub fn button<'a, Message: Clone + 'a>(style: Style, kind: Kind,
    native: iced::widget::Button<'a, Message, Style>, on_press: Option<Message>) -> Control<'a, Message> {
    assert!(kind != Kind::Field);
    let enabled = on_press.is_some();
    let native = native.on_press_maybe(on_press);
    let visual = Rc::new(Cell::new(State::Active));
    let state = visual.clone();
    let native = native.style(move |_, _status| {
        let s = state.get();
        let status = match s { State::Active => native_button::Status::Active, State::Hovered => native_button::Status::Hovered,
            State::Pressed => native_button::Status::Pressed, State::Disabled => native_button::Status::Disabled };
        let mut coat = match kind { Kind::Primary => catalog::button::primary(&style, status),
            _ => catalog::button::ghost(&style, status) };
        if let Some(material) = material(&style, kind, s) {
            coat.background = None; coat.border = iced::Border::default(); coat.text_color = material.ink;
        }
        coat
    });
    Control { child: native.into(), style, kind, enabled, visual, preview: None }
}

pub fn field<'a, Message: Clone + 'a>(style: Style,
    native: iced::widget::TextInput<'a, Message, Style>, on_input: Option<impl Fn(String) -> Message + 'a>) -> Control<'a, Message> {
    let enabled = on_input.is_some();
    let native = native.on_input_maybe(on_input);
    let visual = Rc::new(Cell::new(State::Active));
    let state = visual.clone();
    let native = native.style(move |_, _status| {
        let s = state.get();
        let status = match s { State::Active => text_input::Status::Active, State::Hovered => text_input::Status::Hovered,
            State::Pressed => text_input::Status::Focused { is_hovered: false },
            State::Disabled => text_input::Status::Disabled };
        let mut coat = catalog::field(&style, status);
        if material(&style, Kind::Field, s).is_some() {
            coat.background = iced::Color::TRANSPARENT.into(); coat.border = iced::Border::default();
        }
        coat
    });
    Control { child: native.into(), style, kind: Kind::Field, enabled, visual, preview: None }
}

pub struct Control<'a, Message> {
    child: Element<'a, Message>, style: Style, kind: Kind, enabled: bool,
    visual: Rc<Cell<State>>, preview: Option<State>,
}
impl<Message> Control<'_, Message> {
    /// Force only the visual material for a developer state comparison.
    /// This does not fabricate native text focus or a blinking caret.
    pub fn preview(mut self, state: State) -> Self { self.preview = Some(state); self }
}
#[derive(Default)]
struct Gesture { held: bool, enabled: bool }
impl Gesture {
    fn update(&mut self, event: &Event, inside: bool, enabled: bool) {
        if !enabled { self.held = false; return; }
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerPressed { .. }) => self.held = inside,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerLifted { .. })
            | Event::Touch(iced::touch::Event::FingerLost { .. })
            | Event::Window(iced::window::Event::Unfocused)
            | Event::Mouse(mouse::Event::CursorLeft) => self.held = false,
            _ => {}
        }
    }
}
impl<Message: 'static> Widget<Message, Style, Renderer> for Control<'_, Message> {
    fn tag(&self) -> widget::tree::Tag { widget::tree::Tag::of::<Gesture>() }
    fn state(&self) -> widget::tree::State { widget::tree::State::new(Gesture { held: false, enabled: self.enabled }) }
    fn children(&self) -> Vec<Tree> { vec![Tree::new(&self.child)] }
    fn diff(&self, tree: &mut Tree) {
        let gesture = tree.state.downcast_mut::<Gesture>();
        // Native button's pressed flag is private: a disabled transition
        // invalidates that gesture, but text input state is never replaced.
        if self.kind != Kind::Field && gesture.enabled != self.enabled {
            tree.children[0] = Tree::new(&self.child); gesture.held = false;
        }
        gesture.enabled = self.enabled;
        tree.diff_children(std::slice::from_ref(&self.child));
    }
    fn size(&self) -> Size<Length> { self.child.as_widget().size() }
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let child = self.child.as_widget_mut().layout(&mut tree.children[0], renderer, limits);
        layout::Node::with_children(child.size(), vec![child])
    }
    fn draw(&self, tree: &Tree, renderer: &mut Renderer, theme: &Style,
        style: &renderer::Style, layout: Layout<'_>, cursor: mouse::Cursor, viewport: &Rectangle) {
        let child_layout = layout.children().next().unwrap();
        let focused = self.kind == Kind::Field && tree.children[0].state
            .downcast_ref::<text_input::State<<Renderer as iced::advanced::text::Renderer>::Paragraph>>().is_focused();
        let state = self.preview.unwrap_or_else(|| {
            if !self.enabled { State::Disabled }
            else if focused { State::Pressed }
            else if cursor.is_over(child_layout.bounds()) {
                if tree.state.downcast_ref::<Gesture>().held && self.kind != Kind::Field { State::Pressed }
                else { State::Hovered }
            } else { State::Active }
        });
        self.visual.set(state);
        if let Some(material) = material(&self.style, self.kind, state) {
            let [left, top, right, bottom] = material.outset();
            let bounds = child_layout.bounds();
            let size = Size::new(bounds.width + left + right, bounds.height + top + bottom);
            let mut background: Element<'_, Message> = canvas(MaterialPaint(material))
                .width(size.width).height(size.height).into();
            let mut state = Tree::new(&background);
            let node = background.as_widget_mut().layout(&mut state, renderer, &layout::Limits::new(size, size));
            background.as_widget().draw(&state, renderer, &self.style, style,
                Layout::with_offset(iced::Vector::new(bounds.x - left, bounds.y - top), &node), cursor, viewport);
        }
        self.child.as_widget().draw(&tree.children[0], renderer, theme, style, child_layout, cursor, viewport);
    }
    fn update(&mut self, tree: &mut Tree, event: &Event, layout: Layout<'_>, cursor: mouse::Cursor,
        renderer: &Renderer, clipboard: &mut dyn Clipboard, shell: &mut Shell<'_, Message>, viewport: &Rectangle) {
        let child_layout = layout.children().next().unwrap();
        let gesture = tree.state.downcast_mut::<Gesture>();
        let before = gesture.held;
        gesture.update(event, cursor.is_over(child_layout.bounds()), self.enabled);
        if before != gesture.held { shell.request_redraw(); }
        if self.kind != Kind::Field && matches!(event, Event::Window(iced::window::Event::Unfocused) | Event::Mouse(mouse::Event::CursorLeft)) {
            // Native button already defines cancellation for FingerLost;
            // forward it to prevent a later release from activating a click
            // that left the window or lost focus.
            self.child.as_widget_mut().update(&mut tree.children[0],
                &Event::Touch(iced::touch::Event::FingerLost { id: iced::touch::Finger(0), position: iced::Point::ORIGIN }),
                child_layout, cursor, renderer, clipboard, shell, viewport);
        }
        self.child.as_widget_mut().update(&mut tree.children[0], event, child_layout, cursor, renderer, clipboard, shell, viewport);
    }
    fn operate(&mut self, tree: &mut Tree, layout: Layout<'_>, renderer: &Renderer, operation: &mut dyn Operation) {
        self.child.as_widget_mut().operate(&mut tree.children[0], layout.children().next().unwrap(), renderer, operation);
    }
    fn mouse_interaction(&self, tree: &Tree, layout: Layout<'_>, cursor: mouse::Cursor, viewport: &Rectangle, renderer: &Renderer) -> mouse::Interaction {
        self.child.as_widget().mouse_interaction(&tree.children[0], layout.children().next().unwrap(), cursor, viewport, renderer)
    }
    fn overlay<'b>(&'b mut self, tree: &'b mut Tree, layout: Layout<'b>, renderer: &Renderer, viewport: &Rectangle,
        translation: iced::Vector) -> Option<overlay::Element<'b, Message, Style, Renderer>> {
        self.child.as_widget_mut().overlay(&mut tree.children[0], layout.children().next().unwrap(), renderer, viewport, translation)
    }
}
impl<'a, Message: 'static> From<Control<'a, Message>> for Element<'a, Message> {
    fn from(control: Control<'a, Message>) -> Self { Element::new(control) }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Real native event delivery, using only the software renderer: no window,
    // display server, GPU device, or live desktop interaction.
    fn software_renderer() -> Renderer {
        futures_lite::future::block_on(<Renderer as renderer::Headless>::new(
            iced::Font::default(), iced::Pixels(16.0), Some("tiny-skia"),
        )).expect("software renderer")
    }
    fn deliver<Message: 'static>(control: &mut Control<'_, Message>, tree: &mut Tree,
        renderer: &Renderer, event: Event, inside: bool) -> Vec<Message> {
        let node = control.layout(tree, renderer,
            &layout::Limits::new(Size::ZERO, Size::new(300.0, 100.0)));
        let bounds = node.bounds();
        let cursor = mouse::Cursor::Available(if inside { bounds.center() }
            else { iced::Point::new(bounds.width + 20.0, bounds.height + 20.0) });
        let mut messages = Vec::new();
        control.update(tree, &event, Layout::new(&node), cursor, renderer,
            &mut iced::advanced::clipboard::Null, &mut Shell::new(&mut messages),
            &Rectangle::with_size(Size::new(500.0, 200.0)));
        messages
    }
    #[test]
    fn native_button_cancellation_and_disable_discard_pending_activation() {
        let renderer = software_renderer();
        let make = |enabled: bool| button(crate::Era::Kitsch.style(), Kind::Primary,
            native_button("Go"), enabled.then_some(()));
        let down = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        let up = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
        for cancel in [Event::Mouse(mouse::Event::CursorLeft),
            Event::Window(iced::window::Event::Unfocused)] {
            let mut control = make(true);
            let mut tree = Tree::new(&control as &dyn Widget<(), Style, Renderer>);
            assert!(deliver(&mut control, &mut tree, &renderer, down.clone(), true).is_empty());
            assert!(deliver(&mut control, &mut tree, &renderer, cancel, true).is_empty());
            assert!(deliver(&mut control, &mut tree, &renderer, up.clone(), true).is_empty());
            deliver(&mut control, &mut tree, &renderer, down.clone(), true);
            assert_eq!(deliver(&mut control, &mut tree, &renderer, up.clone(), true), vec![()]);
            assert!(deliver(&mut control, &mut tree, &renderer, up.clone(), true).is_empty());
        }
        let mut control = make(true);
        let mut tree = Tree::new(&control as &dyn Widget<(), Style, Renderer>);
        deliver(&mut control, &mut tree, &renderer, down.clone(), true);
        control = make(false);
        control.diff(&mut tree);
        control = make(true);
        control.diff(&mut tree);
        assert!(deliver(&mut control, &mut tree, &renderer, up.clone(), true).is_empty());
        deliver(&mut control, &mut tree, &renderer, down, true);
        assert_eq!(deliver(&mut control, &mut tree, &renderer, up, true), vec![()]);
    }
    #[test]
    fn native_touch_release_and_loss_match_button_activation() {
        let renderer = software_renderer();
        let mut control = button(crate::Era::Neokitsch.style(), Kind::Ghost,
            native_button("Go"), Some(()));
        let mut tree = Tree::new(&control as &dyn Widget<(), Style, Renderer>);
        let id = iced::touch::Finger(7);
        let position = iced::Point::ORIGIN;
        let down = Event::Touch(iced::touch::Event::FingerPressed { id, position });
        let up = Event::Touch(iced::touch::Event::FingerLifted { id, position });
        assert!(deliver(&mut control, &mut tree, &renderer, down.clone(), true).is_empty());
        assert_eq!(deliver(&mut control, &mut tree, &renderer, up.clone(), true), vec![()]);
        deliver(&mut control, &mut tree, &renderer, down.clone(), true);
        assert!(deliver(&mut control, &mut tree, &renderer, up.clone(), false).is_empty());
        assert!(deliver(&mut control, &mut tree, &renderer, up.clone(), true).is_empty());
        deliver(&mut control, &mut tree, &renderer, down, true);
        deliver(&mut control, &mut tree, &renderer,
            Event::Touch(iced::touch::Event::FingerLost { id, position }), true);
        assert!(deliver(&mut control, &mut tree, &renderer, up, true).is_empty());
    }
    #[test]
    fn native_input_rebuild_preserves_selection_for_committed_text() {
        let renderer = software_renderer();
        let make = |enabled: bool| field(crate::Era::Neokitsch.style(),
            text_input("Hint", "value"), enabled.then_some(|value: String| value));
        let mut control = make(true);
        let mut tree = Tree::new(&control as &dyn Widget<String, Style, Renderer>);
        type InputState = text_input::State<<Renderer as iced::advanced::text::Renderer>::Paragraph>;
        tree.children[0].state.downcast_mut::<InputState>().focus();
        tree.children[0].state.downcast_mut::<InputState>().select_range(1, 4);
        let selection = tree.children[0].state.downcast_ref::<InputState>().cursor();
        let commit = Event::InputMethod(iced::advanced::input_method::Event::Commit("é".into()));
        control = make(false);
        control.diff(&mut tree);
        assert!(deliver(&mut control, &mut tree, &renderer, commit.clone(), true).is_empty());
        assert_eq!(tree.children[0].state.downcast_ref::<InputState>().cursor(), selection);
        control = make(true).preview(State::Hovered);
        control.diff(&mut tree);
        assert!(tree.children[0].state.downcast_ref::<InputState>().is_focused());
        assert_eq!(deliver(&mut control, &mut tree, &renderer, commit, true), vec!["vée"]);
        // Event delegation only: no OS IME connection or preedit UI verification.
    }
    #[test]
    fn native_input_focus_survives_visual_rebuild_and_disabled_state() {
        let make = |enabled: bool| field(crate::Era::Kitsch.style(), text_input("Hint", "value"), enabled.then_some(|_: String| ()));
        let initial: Element<'_, ()> = make(true).into();
        let mut tree = Tree::new(&initial);
        type InputState = text_input::State<<Renderer as iced::advanced::text::Renderer>::Paragraph>;
        tree.children[0].state.downcast_mut::<InputState>().focus();
        tree.children[0].state.downcast_mut::<InputState>().select_range(1, 4);
        let selection = tree.children[0].state.downcast_ref::<InputState>().cursor();
        for state in [State::Hovered, State::Pressed, State::Disabled, State::Active] {
            let rebuilt = make(state != State::Disabled).preview(state);
            rebuilt.diff(&mut tree);
            assert_eq!(tree.children.len(), 1);
            assert!(tree.children[0].state.downcast_ref::<InputState>().is_focused());
            assert_eq!(tree.children[0].state.downcast_ref::<InputState>().cursor(), selection);
        }
    }
    #[test]
    fn visual_gesture_cancels_on_disable_exit_and_focus_loss() {
        for cancel in [Event::Mouse(mouse::Event::CursorLeft), Event::Window(iced::window::Event::Unfocused),
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))] {
            let mut gesture = Gesture::default();
            gesture.update(&Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)), true, true);
            assert!(gesture.held);
            gesture.update(&cancel, false, true);
            assert!(!gesture.held);
        }
        let mut gesture = Gesture { held: true, enabled: true };
        gesture.update(&Event::Mouse(mouse::Event::CursorMoved { position: iced::Point::ORIGIN }), true, false);
        assert!(!gesture.held);
    }
}

/// Only the material data selects a silhouette; input behavior is shared.
struct MaterialPaint(Material);
impl<Message> canvas::Program<Message, Style> for MaterialPaint {
    type State = ();
    fn draw(&self, _: &(), renderer: &Renderer, theme: &Style, bounds: Rectangle,
        cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
        let material = self.0;
        let [left, top, right, bottom] = material.outset();
        let w = bounds.width - left - right;
        let h = bounds.height - top - bottom;
        if w <= 0.0 || h <= 0.0 { return vec![]; }
        let mut layers = match material.shape {
            Shape::Surface => <SurfaceFace as canvas::Program<Message, Style>>::draw(
                &material.face, &(), renderer, theme, Rectangle { height: bounds.height - (bottom - material.face.outset()[3]), ..bounds }, cursor),
            Shape::Step { shoulder, run, rise } => {
                let mut frame = canvas::Frame::new(renderer, bounds.size());
                let path = canvas::Path::new(|p| {
                    let rise = rise.min(h * 0.5);
                    let x = (w * shoulder).min(w - run.min(w * 0.5));
                    p.move_to(iced::Point::new(0.0, rise));
                    p.line_to(iced::Point::new(x, rise));
                    p.line_to(iced::Point::new((x + run).min(w), 0.0));
                    p.line_to(iced::Point::new(w, 0.0));
                    p.line_to(iced::Point::new(w, h));
                    p.line_to(iced::Point::new(0.0, h)); p.close();
                });
                frame.translate(iced::Vector::new(left, top));
                if let Some(echo) = material.face.echo {
                    frame.with_save(|frame| {
                        frame.translate(iced::Vector::new(echo.step.x, echo.step.y));
                        if let Some(ink) = echo.fill {
                            let mut ink = ink.of(&theme.palette); ink.a *= echo.fill_alpha;
                            frame.fill(&path, ink);
                        }
                        let mut ink = echo.ink.of(&theme.palette); ink.a *= echo.alpha;
                        frame.stroke(&path, canvas::Stroke::default().with_color(ink).with_width(echo.width));
                    });
                }
                if let Fill::Solid(ink) = material.face.surface.fill { frame.fill(&path, ink); }
                if let Some(ink) = material.face.surface.stroke {
                    frame.stroke(&path, canvas::Stroke::default().with_color(ink)
                        .with_width(material.face.surface.stroke_width));
                }
                vec![frame.into_geometry()]
            }
        };
        if let Some((ink, held)) = material.tab {
            let mut frame = canvas::Frame::new(renderer, bounds.size());
            frame.translate(iced::Vector::new(left, top));
            // The RIFLES tab occupies x131..168 of its 184px face.
            // Anchor it proportionally as native content chooses the width.
            let path = canvas::Path::new(|p| {
                if held {
                    p.move_to(iced::Point::new(w * 131.0 / 184.0, h - 2.4));
                    p.line_to(iced::Point::new(w * 168.0 / 184.0, h - 2.4));
                    p.line_to(iced::Point::new(w * 165.0 / 184.0, h + 1.0));
                    p.line_to(iced::Point::new(w * 134.0 / 184.0, h + 1.0));
                } else {
                    p.move_to(iced::Point::new(w * 131.0 / 184.0, h));
                    p.line_to(iced::Point::new(w * 135.0 / 184.0, h - 7.0f32.min(h * 0.5)));
                    p.line_to(iced::Point::new(w * 164.0 / 184.0, h - 7.0f32.min(h * 0.5)));
                    p.line_to(iced::Point::new(w * 168.0 / 184.0, h));
                }
                p.close();
            });
            frame.fill(&path, ink); layers.push(frame.into_geometry());
        }
        layers
    }
}
