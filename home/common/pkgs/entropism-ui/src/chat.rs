use iced::widget::{column, container, text, Space, button};
use iced::{Alignment, Border, Color, Element, Length, Shadow};
use crate::glow::{glowing_text, get_radial_offsets, glowing_border_container};

#[derive(Debug, Clone)]
pub enum Message {
    GoBack,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScreenEvent {
    GoToDashboard,
}

pub struct ChatScreen {
    _focused_item: usize,
}

impl ChatScreen {
    pub fn new() -> Self {
        Self {
            _focused_item: 0,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<ScreenEvent> {
        match message {
            Message::GoBack => Some(ScreenEvent::GoToDashboard),
        }
    }

    pub fn handle_key(&mut self, key: &iced::keyboard::Key) -> Option<Message> {
        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) = key {
            return Some(Message::GoBack);
        }
        if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) = key {
            return Some(Message::GoBack);
        }
        None
    }

    pub fn view(
        &self,
        _color_bg: Color,
        color_green_accent: Color,
        window_size: Option<iced::Size>,
    ) -> Element<Message> {
        let (w, h) = match window_size {
            Some(size) => (size.width, size.height),
            None => (1920.0, 1080.0),
        };

        let (off_x, off_y) = get_radial_offsets(w * 0.5, h * 0.5, w, h);

        let content = column![
            glowing_text("CHAT MODULE // OFFLINE", 24, color_green_accent, off_x, off_y),
            Space::new(0.0, 20.0),
            glowing_text("PEER CONNECTIONS: 0", 16, color_green_accent, off_x, off_y),
            Space::new(0.0, 40.0),
            glowing_border_container(
                column![
                    text("CHAT CHANNELS ARE LOCKED.")
                        .size(16)
                        .style(move |_| text::Style { color: Some(color_green_accent) }),
                    Space::new(0.0, 10.0),
                    text("ESTABLISHING SECURE SSH TUNNEL... TIMEOUT.")
                        .size(14)
                        .style(move |_| text::Style { color: Some(color_green_accent) }),
                ]
                .padding(20)
                .width(Length::Fill),
                1.0,
                color_green_accent,
                off_x,
                off_y,
            ),
            Space::new(0.0, 40.0),
            button(
                text("RETURN TO DASHBOARD")
                    .size(14)
                    .style(move |_| text::Style { color: Some(color_green_accent) })
            )
            .padding(12)
            .on_press(Message::GoBack)
            .style(move |_, _| iced::widget::button::Style {
                background: None,
                border: Border {
                    color: color_green_accent,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                text_color: color_green_accent,
            })
        ]
        .align_x(Alignment::Center)
        .max_width(600.0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
