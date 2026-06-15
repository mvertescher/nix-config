use iced::widget::{button, column, row, text, text_input};
use iced::{Background, Border, Color, Element, Shadow};

pub struct LoginScreen {
    pub username: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UsernameChanged(String),
    Submit,
}

impl LoginScreen {
    pub fn new() -> Self {
        Self {
            username: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<String> {
        match message {
            Message::UsernameChanged(val) => {
                self.username = val;
                None
            }
            Message::Submit => {
                if !self.username.trim().is_empty() {
                    Some(self.username.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn view(&self, color_bg: Color, color_green_accent: Color) -> Element<Message> {
        column![
            text("USERNAME:").size(22).style(move |_| text::Style {
                color: Some(color_green_accent)
            }),
            row![
                text_input("**********", &self.username)
                    .on_input(Message::UsernameChanged)
                    .on_submit(Message::Submit)
                    .padding(12)
                    .size(18)
                    .width(360)
                    .style(move |_, _| text_input::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        border: Border {
                            color: color_green_accent,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        icon: color_green_accent,
                        value: color_green_accent,
                        placeholder: Color::from_rgba(0.57, 0.72, 0.62, 0.4),
                        selection: Color::from_rgba(0.57, 0.72, 0.62, 0.2),
                    }),
                button(text("NEXT").size(18).style(move |_| text::Style {
                    color: Some(color_bg)
                }))
                .on_press(Message::Submit)
                .padding(12)
                .style(move |_, _| button::Style {
                    background: Some(Background::Color(color_green_accent)),
                    border: Border {
                        color: color_green_accent,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    shadow: Shadow::default(),
                    text_color: color_bg,
                })
            ]
            .spacing(12)
        ]
        .spacing(12)
        .into()
    }
}
