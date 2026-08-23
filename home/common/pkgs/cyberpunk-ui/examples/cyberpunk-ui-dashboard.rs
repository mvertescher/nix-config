use iced::Element;
use cyberpunk_ui::fonts;
use cyberpunk_ui::panels::dashboard;

pub fn main() -> iced::Result {
    iced::application("NEOMIL // UI TOOLKIT DEMO", App::update, App::view)
        .font(fonts::ORBITRON_REGULAR)
        .font(fonts::ORBITRON_MEDIUM)
        .font(fonts::ORBITRON_SEMIBOLD)
        .font(fonts::ORBITRON_BOLD)
        .font(fonts::RAJDHANI_LIGHT)
        .font(fonts::RAJDHANI_REGULAR)
        .font(fonts::RAJDHANI_MEDIUM)
        .font(fonts::RAJDHANI_SEMIBOLD)
        .font(fonts::RAJDHANI_BOLD)
        .default_font(iced::Font {
            family: iced::font::Family::Name("Rajdhani"),
            weight: iced::font::Weight::Medium,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .window(iced::window::Settings {
            transparent: true,
            ..Default::default()
        })
        .style(|_state, _theme| iced::application::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: iced::Color::WHITE,
        })
        .run()
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {
    MenuSelected(usize),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::MenuSelected(idx) => {
                println!("Selected menu item: {}", idx);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        dashboard(Message::MenuSelected)
    }
}
