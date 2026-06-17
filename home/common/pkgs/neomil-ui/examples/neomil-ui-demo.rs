use iced::widget::{column, container, text};
use iced::{Color, Element, Length};
use neomil_ui::colors;
use neomil_ui::fonts;

pub fn main() -> iced::Result {
    iced::application("NEOMIL // UI TOOLKIT DEMO", App::update, App::view)
        .font(fonts::ORBITRON_REGULAR)
        .font(fonts::ORBITRON_MEDIUM)
        .font(fonts::ORBITRON_SEMIBOLD)
        .font(fonts::ORBITRON_BOLD)
        .default_font(iced::Font {
            family: iced::font::Family::Name("Orbitron"),
            weight: iced::font::Weight::Medium,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .run()
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let content = column![
            text("Neomil UI Toolkit")
                .size(40)
                .font(iced::Font {
                    family: iced::font::Family::Name("Orbitron"),
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }),
            text("A minimal Rust / Iced / Crane based UI toolkit.")
                .size(20),
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(colors::COLOR_BG)),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into()
    }
}
