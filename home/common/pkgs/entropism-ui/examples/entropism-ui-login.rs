use iced::widget::{column, container, Space, text};
use iced::{event, Background, Color, Element, Length, Subscription, Alignment, Task};
use entropism_ui::login::{self, LoginScreen};
use entropism_ui::layout;

use entropism_ui::fonts;

pub fn main() -> iced::Result {
    iced::application("CYBR ENTR-09 // LOGIN DEMO", App::update, App::view)
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

enum Screen {
    Login(LoginScreen),
    Granted(String),
}

struct App {
    screen: Screen,
    window_size: Option<iced::Size>,
}

#[derive(Debug, Clone)]
enum Message {
    LoginMsg(login::Message),
    Event(iced::Event),
    WindowResized(iced::Size),
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Login(LoginScreen::new()),
            window_size: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoginMsg(msg) => {
                if let Screen::Login(ref mut login_screen) = self.screen {
                    if let Some(username) = login_screen.update(msg) {
                        self.screen = Screen::Granted(username);
                    }
                }
            }
            Message::Event(iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })) => {
                if let Screen::Granted(_) = &self.screen {
                    if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) = key {
                        self.screen = Screen::Login(LoginScreen::new());
                    }
                }
            }
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

    fn view(&self) -> Element<Message> {
        use entropism_ui::colors;
        let color_bg = colors::COLOR_BG;
        let color_green_accent = colors::COLOR_GREEN_ACCENT;

        let header = layout::top_bar(color_green_accent, self.window_size);

        let central_content: Element<Message> = match &self.screen {
            Screen::Login(login_screen) => {
                login_screen.view(color_bg, color_green_accent).map(Message::LoginMsg)
            }
            Screen::Granted(username) => {
                container(
                    column![
                        text("ACCESS GRANTED").size(32).style(move |_| text::Style { color: Some(color_green_accent) }),
                        Space::new(0.0, 10.0),
                        text(format!("WELCOME BACK, {}", username)).size(16).style(move |_| text::Style { color: Some(color_green_accent) }),
                        Space::new(0.0, 30.0),
                        text("PRESS ESC TO LOGOUT").size(12).style(move |_| text::Style { color: Some(Color { a: 0.5, ..color_green_accent }) }),
                    ]
                    .align_x(Alignment::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
            }
        };

        let central_area = container(central_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let is_dark_bottom_bar = match &self.screen {
            Screen::Login(_) => false,
            Screen::Granted(_) => true,
        };

        let footer_banner = layout::bottom_bar(color_bg, color_green_accent, self.window_size, is_dark_bottom_bar);

        let main_panel = column![
            header,
            central_area,
            footer_banner
        ]
        .spacing(20)
        .width(Length::Fill)
        .height(Length::Fill);

        container(main_panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(move |_| container::Style {
                background: Some(Background::Color(color_bg)),
                ..Default::default()
            })
            .into()
    }
}
