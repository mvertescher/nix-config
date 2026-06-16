use iced::widget::{column, container, Space};
use iced::{event, Background, Element, Length, Subscription, Task};
use entropism_ui::panels::store::{self, StoreScreen};
use entropism_ui::layout;

use entropism_ui::fonts;

pub fn main() -> iced::Result {
    iced::application("CYBR ENTR-09 // STORE SCREEN DEBUGGER", App::update, App::view)
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
    store_screen: StoreScreen,
    window_size: Option<iced::Size>,
}

#[derive(Debug, Clone)]
enum Message {
    StoreMsg(store::Message),
    Event(iced::Event),
    WindowResized(iced::Size),
}

impl Default for App {
    fn default() -> Self {
        Self {
            store_screen: StoreScreen::new(get_demo_store_categories(), get_demo_store_items()),
            window_size: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StoreMsg(msg) => {
                let _event = self.store_screen.update(msg);
            }
            Message::Event(iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. })) => {
                if let Some(msg) = self.store_screen.handle_key(&key) {
                    return self.update(Message::StoreMsg(msg));
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
            iced::time::every(std::time::Duration::from_millis(16)).map(|instant| {
                Message::StoreMsg(store::Message::Tick(instant))
            }),
        ])
    }

    fn view(&self) -> Element<'_, Message> {
        use entropism_ui::colors;
        let color_bg = colors::COLOR_BG;
        let color_green_accent = colors::COLOR_GREEN_ACCENT;

        let header = layout::top_bar(color_green_accent, self.window_size);

        let body = self.store_screen.view(color_bg, color_green_accent, self.window_size)
            .map(Message::StoreMsg);

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

fn get_demo_store_categories() -> Vec<String> {
    vec![
        "HASTA".to_string(),
        "GLADIUS".to_string(),
        "SAGITTA".to_string(),
        "SCUTUM".to_string(),
        "PUGIO".to_string(),
    ]
}

fn get_demo_store_items() -> Vec<Vec<entropism_ui::panels::store::StoreItem>> {
    vec![
        vec![
            entropism_ui::panels::store::StoreItem { name: "AURUM".to_string(), sub: "MODUS OPERANDI".to_string(), dps: 120, pnt: 45, acc: 8, rof: 6, desc: "Gravitas: XII\nCeleritas: V\nSpatium: XLV\n\nBonus:\nFortuna +V% celeris".to_string() },
            entropism_ui::panels::store::StoreItem { name: "ARGENTUM".to_string(), sub: "BELLUM ACER".to_string(), dps: 98, pnt: 32, acc: 6, rof: 8, desc: "Gravitas: XV\nCeleritas: VIII\nSpatium: XXXVIII\n\nBonus:\nImpeditus +X% armor".to_string() },
        ],
        vec![
            entropism_ui::panels::store::StoreItem { name: "VERTIGO".to_string(), sub: "TEMPUS FUGIT".to_string(), dps: 86, pnt: 30, acc: 5, rof: 5, desc: "Gravitas: XX\nCeleritas: XXII\nSpatium: XII\n\nBonus:\nReflexio +IX virtutis\nNexus modularis +II".to_string() },
            entropism_ui::panels::store::StoreItem { name: "IGNIS".to_string(), sub: "CARPE DIEM".to_string(), dps: 95, pnt: 25, acc: 4, rof: 10, desc: "Gravitas: XVIII\nCeleritas: XXV\nSpatium: XV\n\nBonus:\nReflexio +V virtutis".to_string() },
            entropism_ui::panels::store::StoreItem { name: "AQUA".to_string(), sub: "MEMENTO MORI".to_string(), dps: 78, pnt: 28, acc: 9, rof: 4, desc: "Gravitas: VIII\nCeleritas: VI\nSpatium: XXV\n\nBonus:\nIntellectus modularis".to_string() },
        ],
        vec![
            entropism_ui::panels::store::StoreItem { name: "SOLIS".to_string(), sub: "AMOR FATI".to_string(), dps: 220, pnt: 85, acc: 9, rof: 1, desc: "Gravitas: XLV\nCeleritas: II\nSpatium: CXX\n\nBonus:\nAdfero potentia xII.V".to_string() },
        ],
        vec![
            entropism_ui::panels::store::StoreItem { name: "TERRA".to_string(), sub: "AD ASTRA".to_string(), dps: 180, pnt: 15, acc: 2, rof: 1, desc: "Gravitas: LXXX\nCeleritas: LXV\nSpatium: VIII\n\nBonus:\nImpactus +XL% celeris".to_string() },
        ],
        vec![
            entropism_ui::panels::store::StoreItem { name: "VENTUS".to_string(), sub: "IN MEDIAS RES".to_string(), dps: 110, pnt: 40, acc: 7, rof: 2, desc: "Gravitas: XXXV\nCeleritas: XII\nSpatium: XXX\n\nBonus:\nLetalis letum xII.O".to_string() },
        ],
    ]
}
