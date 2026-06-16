use iced::widget::{column, container, Space};
use iced::{event, Background, Element, Length, Subscription, Task};
use entropism_ui::layout;

use entropism_ui::fonts;

pub fn main() -> iced::Result {
    iced::application("CYBR ENTR-09 // DEFAULT LAYOUT DEBUGGER", App::update, App::view)
        .font(fonts::RAJDHANI_REGULAR)
        .font(fonts::RAJDHANI_MEDIUM)
        .font(fonts::RAJDHANI_SEMIBOLD)
        .font(fonts::RAJDHANI_BOLD)
        .font(fonts::RAJDHANI_LIGHT)
        .default_font(iced::Font {
            family: iced::font::Family::Name("Rajdhani"),
            weight: iced::font::Weight::Medium,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .subscription(App::subscription)
        .run()
}

struct App {
    window_size: Option<iced::Size>,
}

#[derive(Debug, Clone)]
enum Message {
    #[allow(dead_code)]
    Event(iced::Event),
    WindowResized(iced::Size),
}

impl Default for App {
    fn default() -> Self {
        Self {
            window_size: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowResized(size) => {
                self.window_size = Some(size);
            }
            _ => {}
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen().map(Message::Event),
            iced::window::resize_events().map(|(_, size)| Message::WindowResized(size)),
        ])
    }

    fn view(&self) -> Element<'_, Message> {
        use entropism_ui::colors;
        let color_bg = colors::COLOR_BG;
        let color_green_accent = colors::COLOR_GREEN_ACCENT;

        let header = layout::top_bar(color_green_accent, self.window_size);

        let central_area = container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill);

        let footer_banner = layout::bottom_bar(color_bg, color_green_accent, self.window_size, true);

        let main_panel = column![
            header,
            central_area,
            footer_banner
        ]
        .spacing(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20);

        let background = container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(color_bg)),
                ..Default::default()
            });

        iced::widget::stack![
            background,
            layout::debug_grid(),
            main_panel
        ].into()
    }
}
