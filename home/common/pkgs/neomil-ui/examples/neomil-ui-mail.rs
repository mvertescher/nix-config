use iced::{Element, Subscription, Task, Event, keyboard};
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
        .subscription(App::subscription)
        .run()
}

struct App {
    emails: Vec<Email>,
    selected_id: Option<usize>,
    scrollable_id: iced::widget::scrollable::Id,
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
            Email {
                id: 5,
                title: "Aenean auctor wisi".to_string(),
                sender: "Marcus Aurelius".to_string(),
                body: "Aenean auctor wisi et urna. Aliquam erat volutpat. Duis ac turpis. Integer rutrum ante eu lacus. Vestibulum libero nisl, porta vel, scelerisque eget, malesuada at, neque. Vivamus eget nibh. Etiam cursus leo vel metus.".to_string(),
                is_new: true,
            },
            Email {
                id: 6,
                title: "Vestibulum ante ipsum".to_string(),
                sender: "Julius Caesar".to_string(),
                body: "Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Morbi lacinia molestie dui. Praesent blandit dolor. Sed non quam. In vel mi sit amet augue congue elementum.".to_string(),
                is_new: false,
            },
            Email {
                id: 7,
                title: "Mauris ipsum nulla".to_string(),
                sender: "Cicero".to_string(),
                body: "Mauris ipsum nulla, metus accumsan a, ultricies sit amet, lacinia eget, lectus. Mauris imperdiet, sem ac laoreet interdum, magna tellus gravida elit, ac dignissim magna sed pede. Aliquam erat volutpat.".to_string(),
                is_new: false,
            },
            Email {
                id: 8,
                title: "Donec quis dui".to_string(),
                sender: "Seneca".to_string(),
                body: "Donec quis dui at dolor tempor interdum. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Mauris viverra et, ac laoreet interdum, magna tellus gravida elit.".to_string(),
                is_new: true,
            },
            Email {
                id: 9,
                title: "Phasellus neque".to_string(),
                sender: "Pliny the Elder".to_string(),
                body: "Phasellus neque. Cras ac dui. In hac habitasse platea dictumst. Vivamus convallis eleifend nisl. Nullam eget leo leo. Aliquam vulputate, pede vel vehicula accumsan, mi neque rutrum erat.".to_string(),
                is_new: false,
            },
            Email {
                id: 10,
                title: "Integer in mauris".to_string(),
                sender: "Tacitus".to_string(),
                body: "Integer in mauris eu nibh euismod gravida. Duis ac tellus. Donec quis dui at dolor tempor interdum. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae.".to_string(),
                is_new: false,
            },
            Email {
                id: 11,
                title: "Vivamus eget nibh".to_string(),
                sender: "Suetonius".to_string(),
                body: "Vivamus eget nibh. Etiam cursus leo vel metus. Nulla facilisi. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam. Sed nisi. Nulla quis sem at nibh elementum imperdiet.".to_string(),
                is_new: true,
            },
            Email {
                id: 12,
                title: "Etiam cursus leo".to_string(),
                sender: "Plutarch".to_string(),
                body: "Etiam cursus leo vel metus. Nulla facilisi. Integer nec odio. Praesent libero. Sed cursus ante dapibus diam. Sed nisi. Nulla quis sem at nibh elementum imperdiet. Duis sagittis ipsum.".to_string(),
                is_new: false,
            },
            Email {
                id: 13,
                title: "Cras ac dui".to_string(),
                sender: "Livy".to_string(),
                body: "Cras ac dui. In hac habitasse platea dictumst. Vivamus convallis eleifend nisl. Nullam eget leo leo. Aliquam vulputate, pede vel vehicula accumsan, mi neque rutrum erat, eu congue orci.".to_string(),
                is_new: false,
            },
            Email {
                id: 14,
                title: "Duis ac turpis".to_string(),
                sender: "Ovid".to_string(),
                body: "Duis ac turpis. Integer rutrum ante eu lacus. Vestibulum libero nisl, porta vel, scelerisque eget, malesuada at, neque. Vivamus eget nibh. Etiam cursus leo vel metus. Nulla facilisi.".to_string(),
                is_new: true,
            },
            Email {
                id: 15,
                title: "Nullam eget leo".to_string(),
                sender: "Virgil".to_string(),
                body: "Nullam eget leo leo. Aliquam vulputate, pede vel vehicula accumsan, mi neque rutrum erat, eu congue orci lorem eget lorem. Vestibulum ante ipsum primis in faucibus orci luctus.".to_string(),
                is_new: false,
            },
        ];

        App {
            emails,
            selected_id: Some(1),
            scrollable_id: iced::widget::scrollable::Id::unique(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SelectEmail(usize),
    DeleteEmail(usize),
    Event(Event),
}

impl App {
    fn select_email(&mut self, id: usize) -> Task<Message> {
        self.selected_id = Some(id);
        self.scroll_to_selected()
    }

    fn scroll_to_selected(&self) -> Task<Message> {
        if let Some(selected_id) = self.selected_id {
            if let Some(index) = self.emails.iter().position(|e| e.id == selected_id) {
                let item_height = 70.0; // 60px card + 10px spacing
                let viewport_height = 600.0; // Estimated viewport height
                let total_items = self.emails.len();
                
                let target_y = (index as f32) * item_height;
                let total_height = (total_items as f32) * item_height;
                let max_scroll = (total_height - viewport_height).max(0.0);

                let center_offset = target_y - (viewport_height / 2.0) + (item_height / 2.0);
                let final_y = center_offset.clamp(0.0, max_scroll);

                iced::widget::scrollable::scroll_to(
                    self.scrollable_id.clone(),
                    iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: final_y },
                )
            } else {
                Task::none()
            }
        } else {
            Task::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectEmail(id) => {
                return self.select_email(id);
            }
            Message::DeleteEmail(id) => {
                self.emails.retain(|e| e.id != id);
                if self.selected_id == Some(id) {
                    self.selected_id = self.emails.first().map(|e| e.id);
                }
                return self.scroll_to_selected();
            }
            Message::Event(Event::Keyboard(keyboard::Event::KeyPressed { key, .. })) => {
                match key {
                    keyboard::Key::Character(c) => {
                        match c.as_str() {
                            "j" => {
                                let next_id = if let Some(selected_id) = self.selected_id {
                                    if let Some(index) = self.emails.iter().position(|e| e.id == selected_id) {
                                        if index < self.emails.len() - 1 {
                                            Some(self.emails[index + 1].id)
                                        } else {
                                            Some(selected_id)
                                        }
                                    } else {
                                        self.emails.first().map(|e| e.id)
                                    }
                                } else {
                                    self.emails.first().map(|e| e.id)
                                };
                                if let Some(id) = next_id {
                                    return self.select_email(id);
                                }
                            }
                            "k" => {
                                let prev_id = if let Some(selected_id) = self.selected_id {
                                    if let Some(index) = self.emails.iter().position(|e| e.id == selected_id) {
                                        if index > 0 {
                                            Some(self.emails[index - 1].id)
                                        } else {
                                            Some(selected_id)
                                        }
                                    } else {
                                        self.emails.first().map(|e| e.id)
                                    }
                                } else {
                                    self.emails.first().map(|e| e.id)
                                };
                                if let Some(id) = prev_id {
                                    return self.select_email(id);
                                }
                            }
                            "h" => {
                                self.selected_id = None;
                            }
                            "l" => {
                                if self.selected_id.is_none() {
                                    if let Some(first) = self.emails.first() {
                                        return self.select_email(first.id);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::Event)
    }

    fn view(&self) -> Element<'_, Message> {
        mail_panel(
            &self.emails,
            self.selected_id,
            Message::SelectEmail,
            Message::DeleteEmail,
            self.scrollable_id.clone(),
            colors::COLOR_PRIMARY_RED,
        )
    }
}
