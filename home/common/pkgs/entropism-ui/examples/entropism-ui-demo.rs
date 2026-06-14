use iced::widget::{column, container, Space};
use iced::{event, Background, Color, Element, Length, Subscription, Task};
use entropism_ui::dashboard::{self, DashboardScreen};
use entropism_ui::mail::{self, MailScreen};
use entropism_ui::store::{self, StoreScreen};
use entropism_ui::chat::{self, ChatScreen};
use entropism_ui::matrix::{self, MatrixScreen};
use entropism_ui::layout;

use entropism_ui::fonts;

pub fn main() -> iced::Result {
    iced::application("CYBR ENTR-09 // TERMINAL DEMO", App::update, App::view)
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
    Dashboard(DashboardScreen),
    Mail(MailScreen),
    Store(StoreScreen),
    Chat(ChatScreen),
    Matrix(MatrixScreen),
}

struct App {
    screen: Screen,
    window_size: Option<iced::Size>,
}

#[derive(Debug, Clone)]
enum Message {
    DashboardMsg(dashboard::Message),
    MailMsg(mail::Message),
    StoreMsg(store::Message),
    ChatMsg(chat::Message),
    MatrixMsg(matrix::Message),
    Event(iced::Event),
    WindowResized(iced::Size),
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Dashboard(DashboardScreen::new()),
            window_size: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DashboardMsg(msg) => {
                if let Screen::Dashboard(ref mut dashboard_screen) = self.screen {
                    if let Some(event) = dashboard_screen.update(msg) {
                        match event {
                            dashboard::ScreenEvent::Disconnect => {
                                self.screen = Screen::Dashboard(DashboardScreen::new());
                            }
                            dashboard::ScreenEvent::GoToChat => {
                                self.screen = Screen::Chat(ChatScreen::new());
                            }
                            dashboard::ScreenEvent::GoToMail => {
                                self.screen = Screen::Mail(MailScreen::new(get_demo_emails()));
                            }
                            dashboard::ScreenEvent::GoToStore => {
                                self.screen = Screen::Store(StoreScreen::new(get_demo_store_categories(), get_demo_store_items()));
                            }
                            dashboard::ScreenEvent::GoToMatrix => {
                                self.screen = Screen::Matrix(MatrixScreen::new());
                            }
                        }
                    }
                }
            }
            Message::MailMsg(msg) => {
                if let Screen::Mail(ref mut mail_screen) = self.screen {
                    let (event, task) = mail_screen.update(msg);
                    if let Some(ev) = event {
                        match ev {
                            mail::ScreenEvent::GoToDashboard => {
                                self.screen = Screen::Dashboard(DashboardScreen::new());
                            }
                        }
                    }
                    return task.map(Message::MailMsg);
                }
            }
            Message::StoreMsg(msg) => {
                if let Screen::Store(ref mut store_screen) = self.screen {
                    if let Some(event) = store_screen.update(msg) {
                        match event {
                            store::ScreenEvent::GoToDashboard => {
                                self.screen = Screen::Dashboard(DashboardScreen::new());
                            }
                        }
                    }
                }
            }
            Message::ChatMsg(msg) => {
                if let Screen::Chat(ref mut chat_screen) = self.screen {
                    if let Some(event) = chat_screen.update(msg) {
                        match event {
                            chat::ScreenEvent::GoToDashboard => {
                                self.screen = Screen::Dashboard(DashboardScreen::new());
                            }
                        }
                    }
                }
            }
            Message::MatrixMsg(msg) => {
                if let Screen::Matrix(ref mut matrix_screen) = self.screen {
                    if let Some(event) = matrix_screen.update(msg) {
                        match event {
                            matrix::ScreenEvent::GoToDashboard => {
                                self.screen = Screen::Dashboard(DashboardScreen::new());
                            }
                        }
                    }
                }
            }
            Message::Event(iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })) => {
                match &mut self.screen {
                    Screen::Dashboard(ref mut dashboard_screen) => {
                        if let Some(msg) = dashboard_screen.handle_key(&key) {
                            return self.update(Message::DashboardMsg(msg));
                        }
                    }
                    Screen::Mail(ref mut mail_screen) => {
                        if let Some(msg) = mail_screen.handle_key(&key) {
                            return self.update(Message::MailMsg(msg));
                        }
                    }
                    Screen::Store(ref mut store_screen) => {
                        if let Some(msg) = store_screen.handle_key(&key) {
                            return self.update(Message::StoreMsg(msg));
                        }
                    }
                    Screen::Chat(ref mut chat_screen) => {
                        if let Some(msg) = chat_screen.handle_key(&key) {
                            return self.update(Message::ChatMsg(msg));
                        }
                    }
                    Screen::Matrix(ref mut matrix_screen) => {
                        if let Some(msg) = matrix_screen.handle_key(&key) {
                            return self.update(Message::MatrixMsg(msg));
                        }
                    }
                    _ => {}
                }
            }
            Message::WindowResized(size) => {
                println!("WINDOW_SIZE: {}x{}", size.width, size.height);
                self.window_size = Some(size);
            }
            _ => {}
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![
            event::listen().map(Message::Event),
            iced::window::resize_events().map(|(_, size)| Message::WindowResized(size)),
        ];
        if let Screen::Store(_) = &self.screen {
            subs.push(iced::time::every(std::time::Duration::from_millis(16)).map(|instant| {
                Message::StoreMsg(store::Message::Tick(instant))
            }));
        }
        Subscription::batch(subs)
    }

    fn view(&self) -> Element<Message> {
        use entropism_ui::colors;
        let color_bg = colors::COLOR_BG;
        let color_green_accent = colors::COLOR_GREEN_ACCENT;

        let header = layout::top_bar(color_green_accent, self.window_size);

        // Render central content based on screen state
        let central_content: Element<Message> = match &self.screen {
            Screen::Dashboard(dashboard_screen) => {
                dashboard_screen.view(color_bg, color_green_accent, self.window_size).map(Message::DashboardMsg)
            }
            Screen::Mail(mail_screen) => {
                mail_screen.view(color_bg, color_green_accent, self.window_size).map(Message::MailMsg)
            }
            Screen::Store(store_screen) => {
                store_screen.view(color_bg, color_green_accent, self.window_size).map(Message::StoreMsg)
            }
            Screen::Chat(chat_screen) => {
                chat_screen.view(color_bg, color_green_accent, self.window_size).map(Message::ChatMsg)
            }
            Screen::Matrix(matrix_screen) => {
                matrix_screen.view(color_bg, color_green_accent, self.window_size).map(Message::MatrixMsg)
            }
        };

        let central_area = container(central_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let is_dark_bottom_bar = match &self.screen {
            Screen::Dashboard(_) | Screen::Mail(_) | Screen::Store(_) | Screen::Chat(_) | Screen::Matrix(_) => true,
        };

        let footer_banner = layout::bottom_bar(color_bg, color_green_accent, self.window_size, is_dark_bottom_bar);

        // Combined Main Panel
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

fn get_demo_emails() -> Vec<entropism_ui::mail::Email> {
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
        // Generate random subject: PREFIX + NOUN + optional index
        let prefix = prefixes[rng.next_range(0, prefixes.len())];
        let noun = nouns[rng.next_range(0, nouns.len())];
        let subject = format!("{} {} // {:02}", prefix, noun, i);

        // Random sender
        let from = senders[rng.next_range(0, senders.len())].to_string();

        // Random body containing 3 to 6 randomized sentences
        let sentences_count = rng.next_range(3, 7);
        let mut body_parts = Vec::new();
        for _ in 0..sentences_count {
            body_parts.push(body_sentences[rng.next_range(0, body_sentences.len())]);
        }
        let body = body_parts.join("\n\n");

        emails.push(entropism_ui::mail::Email { subject, from, body });
    }

    emails
}

fn get_demo_store_categories() -> Vec<String> {
    vec![
        "HASTA".to_string(),
        "GLADIUS".to_string(),
        "SAGITTA".to_string(),
        "SCUTUM".to_string(),
        "PUGIO".to_string(),
    ]
}

fn get_demo_store_items() -> Vec<Vec<entropism_ui::store::StoreItem>> {
    vec![
        // Hasta (Rifles)
        vec![
            entropism_ui::store::StoreItem { name: "AURUM".to_string(), sub: "MODUS OPERANDI".to_string(), dps: 120, pnt: 45, acc: 8, rof: 6, desc: "Gravitas: XII\nCeleritas: V\nSpatium: XLV\n\nBonus:\nFortuna +V% celeris".to_string() },
            entropism_ui::store::StoreItem { name: "ARGENTUM".to_string(), sub: "BELLUM ACER".to_string(), dps: 98, pnt: 32, acc: 6, rof: 8, desc: "Gravitas: XV\nCeleritas: VIII\nSpatium: XXXVIII\n\nBonus:\nImpeditus +X% armor".to_string() },
        ],
        // Gladius (SMG)
        vec![
            entropism_ui::store::StoreItem { name: "VERTIGO".to_string(), sub: "TEMPUS FUGIT".to_string(), dps: 86, pnt: 30, acc: 5, rof: 5, desc: "Gravitas: XX\nCeleritas: XXII\nSpatium: XII\n\nBonus:\nReflexio +IX virtutis\nNexus modularis +II".to_string() },
            entropism_ui::store::StoreItem { name: "IGNIS".to_string(), sub: "CARPE DIEM".to_string(), dps: 95, pnt: 25, acc: 4, rof: 10, desc: "Gravitas: XVIII\nCeleritas: XXV\nSpatium: XV\n\nBonus:\nReflexio +V virtutis".to_string() },
            entropism_ui::store::StoreItem { name: "AQUA".to_string(), sub: "MEMENTO MORI".to_string(), dps: 78, pnt: 28, acc: 9, rof: 4, desc: "Gravitas: VIII\nCeleritas: VI\nSpatium: XXV\n\nBonus:\nIntellectus modularis".to_string() },
        ],
        // Sagitta (Sniper)
        vec![
            entropism_ui::store::StoreItem { name: "SOLIS".to_string(), sub: "AMOR FATI".to_string(), dps: 220, pnt: 85, acc: 9, rof: 1, desc: "Gravitas: XLV\nCeleritas: II\nSpatium: CXX\n\nBonus:\nAdfero potentia xII.V".to_string() },
        ],
        // Scutum (Shotgun)
        vec![
            entropism_ui::store::StoreItem { name: "TERRA".to_string(), sub: "AD ASTRA".to_string(), dps: 180, pnt: 15, acc: 2, rof: 1, desc: "Gravitas: LXXX\nCeleritas: LXV\nSpatium: VIII\n\nBonus:\nImpactus +XL% celeris".to_string() },
        ],
        // Pugio (Pistol)
        vec![
            entropism_ui::store::StoreItem { name: "VENTUS".to_string(), sub: "IN MEDIAS RES".to_string(), dps: 110, pnt: 40, acc: 7, rof: 2, desc: "Gravitas: XXXV\nCeleritas: XII\nSpatium: XXX\n\nBonus:\nLetalis letum xII.O".to_string() },
        ],
    ]
}
