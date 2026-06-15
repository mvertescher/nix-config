use iced::widget::{column, container, Space};
use iced::{event, Background, Color, Element, Length, Subscription, Task};
use entropism_ui::panels::mail::{self, MailScreen};
use entropism_ui::layout;

use entropism_ui::fonts;

pub fn main() -> iced::Result {
    iced::application("CYBR ENTR-09 // MAIL SCREEN DEBUGGER", App::update, App::view)
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
    mail_screen: MailScreen,
    window_size: Option<iced::Size>,
}

#[derive(Debug, Clone)]
enum Message {
    MailMsg(mail::Message),
    Event(iced::Event),
    WindowResized(iced::Size),
}

impl Default for App {
    fn default() -> Self {
        Self {
            mail_screen: MailScreen::new(get_demo_emails()),
            window_size: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MailMsg(msg) => {
                let (_event, task) = self.mail_screen.update(msg);
                return task.map(Message::MailMsg);
            }
            Message::Event(iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })) => {
                if let Some(msg) = self.mail_screen.handle_key(&key) {
                    return self.update(Message::MailMsg(msg));
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

        let body = self.mail_screen.view(color_bg, color_green_accent, self.window_size)
            .map(Message::MailMsg);

        let layout_content = column![
            header,
            Space::new(0.0, 15.0),
            body
        ]
        .padding(20);

        container(layout_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(color_bg)),
                ..Default::default()
            })
            .into()
    }
}

fn get_demo_emails() -> Vec<entropism_ui::panels::mail::Email> {
    struct Lcg {
        state: u32,
    }

    impl Lcg {
        fn new(seed: u32) -> Self {
            Self { state: seed }
        }

        fn next(&mut self) -> u32 {
            self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
            self.state
        }

        fn next_range(&mut self, min: usize, max: usize) -> usize {
            let val = self.next() as usize;
            min + (val % (max - min))
        }
    }

    let mut rng = Lcg::new(12345);

    let prefixes = vec!["AD", "DE", "IN", "OB", "PROPTER", "SINE", "SUB"];
    let nouns = vec![
        "AMICITIA", "CUPIDITAS", "DOLOR", "EPISTULA", "FIDES", "FORTUNA", "GAUDIUM",
        "HONOR", "IMPERIUM", "JUSTITIA", "LIBERTAS", "MEMORIA", "NUNTIUS", "OFFICIUM",
        "PAX", "RATIO", "SALUS", "TEMPUS", "VERITAS", "VIRTUS"
    ];
    let senders = vec![
        "MARCUS", "JULIA", "LUCIUS", "AEMILIA", "GAIUS", "TULLIA", "PUBLIUS", "CORNELIA",
        "DECIMUS", "LIVIA", "QUINTUS", "CLAUDIA", "SEXTUS", "FABIA", "TITUS", "OCTAVIA"
    ];
    let body_sentences = vec![
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
        "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
        "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        "Curabitur pretium tincidunt lacus. Nulla gravida orci a odio.",
        "Nullam varius, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris.",
        "Integer in mauris eu nibh euismod gravida. Duis ac tellus.",
        "Donec ac tempus orci, sit amet pretium nisi. Nullam eget leo leo.",
        "Aliquam vulputate, pede vel vehicula accumsan, mi neque rutrum erat, eu congue orci lorem eget lorem.",
        "Morbi tristique senectus et netus et malesuada fames ac turpis egestas.",
        "Aenean sit amet justo vel justo sodales accumsan."
    ];

    let mut emails = Vec::new();
    for i in 1..=30 {
        let prefix = prefixes[rng.next_range(0, prefixes.len())];
        let noun = nouns[rng.next_range(0, nouns.len())];
        let subject = format!("{} {} // {:02}", prefix, noun, i);

        let from = senders[rng.next_range(0, senders.len())].to_string();

        let sentences_count = rng.next_range(3, 7);
        let mut body_parts = Vec::new();
        for _ in 0..sentences_count {
            body_parts.push(body_sentences[rng.next_range(0, body_sentences.len())]);
        }
        let body = body_parts.join("\n\n");

        emails.push(entropism_ui::panels::mail::Email { subject, from, body });
    }

    emails
}
