use iced::widget::{column, container, row, text, Space};
use iced::{Color, Element, Length};
use neomil_ui::colors;
use neomil_ui::fonts;
use neomil_ui::widgets::chip_type_1;

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
        let header = text("NEOMIL // CORE INTERFACE V1.0")
            .size(32)
            .font(iced::Font {
                family: iced::font::Family::Name("Orbitron"),
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .style(|_| text::Style {
                color: Some(colors::COLOR_PRIMARY_RED),
            });

        let subheader = text("SECURE MILITARY CONFIGURATION TERMINAL")
            .size(14)
            .style(|_| text::Style {
                color: Some(colors::COLOR_YELLOW),
            });

        // A status panel using the primary red accent chip
        let status_box = chip_type_1(
            column![
                text("SYSTEM STATUS: NOMINAL").size(16).style(|_| text::Style {
                    color: Some(Color::WHITE),
                }),
                Space::new(0.0, 10.0),
                text("HOST CONFIG: OK").size(14).style(|_| text::Style {
                    color: Some(colors::COLOR_YELLOW),
                }),
                text("NETWORK: CONNECTED").size(14).style(|_| text::Style {
                    color: Some(colors::COLOR_YELLOW),
                }),
                text("DEVICES: 4 DETECTED").size(14).style(|_| text::Style {
                    color: Some(colors::COLOR_YELLOW),
                }),
            ]
            .spacing(5),
            colors::COLOR_PRIMARY_RED,
            colors::COLOR_BG,
        );

        // A log panel using the yellow accent chip
        let action_box = chip_type_1(
            column![
                text("SECURITY LOG").size(16).style(|_| text::Style {
                    color: Some(Color::WHITE),
                }),
                Space::new(0.0, 10.0),
                text("LOG [09:21:43] AUTHENTICATING...").size(12).style(|_| text::Style {
                    color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                }),
                text("LOG [09:21:45] ACCESS GRANTED").size(12).style(|_| text::Style {
                    color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                }),
            ]
            .spacing(5),
            colors::COLOR_YELLOW,
            colors::COLOR_BG,
        );

        let panels = row![
            container(status_box).width(Length::FillPortion(1)),
            Space::new(20.0, 0.0),
            container(action_box).width(Length::FillPortion(1)),
        ]
        .width(Length::Fixed(800.0));

        let content = column![
            header,
            subheader,
            Space::new(0.0, 30.0),
            panels,
        ]
        .spacing(10)
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
