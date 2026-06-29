use iced::Element;
use neomil_ui::fonts;
use neomil_ui::colors;
use neomil_ui::panels::{mail_panel, Email};

pub fn main() -> iced::Result {
    iced::application("NEOMIL // MAIL SYSTEM", App::update, App::view)
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

struct App {
    emails: Vec<Email>,
    selected_id: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        let emails = vec![
            Email {
                id: 1,
                title: "Lorem ipsum dolor".to_string(),
                sender: "Aenean Vulputate".to_string(),
                body: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin sodales elit id neque porta, ac dictum magna elementum. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Aliquam sit amet luctus magna. Suspendisse potenti. Duis ac sem non ante eleifend dictum. Duis quis magna eget diam lobortis facilisis.".to_string(),
                is_new: true,
            },
            Email {
                id: 2,
                title: "Donec pretium posuere".to_string(),
                sender: "Morbi Tristique".to_string(),
                body: "Donec pretium posuere diam, at sodales magna elementum eget. Phasellus congue accumsan nisl, vel rhoncus est volutpat eu. Mauris convallis rhoncus erat, vel varius leo viverra id. Nullam placerat varius magna, a laoreet magna convallis a. Sed eget massa ac purus sodales imperdiet.".to_string(),
                is_new: true,
            },
            Email {
                id: 3,
                title: "Nulla facilisi".to_string(),
                sender: "Phasellus Porta".to_string(),
                body: "Nulla facilisi. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam. Sed nisi. Nulla quis sem at nibh elementum imperdiet. Duis sagittis ipsum. Praesent mauris. Fusce nec tellus sed augue semper porta. Mauris massa. Vestibulum lacinia arcu eget nulla.".to_string(),
                is_new: false,
            },
            Email {
                id: 4,
                title: "Curabitur sodales".to_string(),
                sender: "Vestibulum Lacinia".to_string(),
                body: "Curabitur sodales ligula in libero. Sed dignissim lacinia nunc. Curabitur tortor. Pellentesque nibh. Aenean quam. In scelerisque sem at dolor. Maecenas mattis. Sed convallis tristique sem. Proin ut ligula vel nunc egestas porttitor. Morbi lectus risus, iaculis vel, suscipit quis, luctus non, massa.".to_string(),
                is_new: false,
            },
        ];

        App {
            emails,
            selected_id: Some(1), // Select first email by default
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SelectEmail(usize),
    DeleteEmail(usize),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::SelectEmail(id) => {
                self.selected_id = Some(id);
                // Mark as read
                if let Some(email) = self.emails.iter_mut().find(|e| e.id == id) {
                    email.is_new = false;
                }
            }
            Message::DeleteEmail(id) => {
                self.emails.retain(|e| e.id != id);
                if self.selected_id == Some(id) {
                    // Select the first remaining email, or None
                    self.selected_id = self.emails.first().map(|e| e.id);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        mail_panel(
            &self.emails,
            self.selected_id,
            Message::SelectEmail,
            Message::DeleteEmail,
            colors::COLOR_PRIMARY_RED,
        )
    }
}
