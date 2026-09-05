//! The working mail client, in any era.
//!
//!     cp-eras-ui-mail                # follow the desktop theme
//!     cp-eras-ui-mail --era kitsch   # force one
//!
//! `cp-eras-ui-mailbox` is the display-only design target for the
//! same screen; this one has selection, focus, scrolling and deletion
//! wired up. hjkl move and switch panes, `a` adds a message, `d`
//! deletes the selected one, ctrl-f/ctrl-b page the thread.
//!
//! See examples/cp-eras-ui-store.rs for the reasoning behind the
//! --era handling; it is the same here.

use cp_eras_ui::fonts;
use cp_eras_ui::panels::{
    mail::{mail_list_viewport_height, message_row_pitch},
    mail_panel, Email, MailFocus, ThreadMessage,
};
use cp_eras_ui::{Era, Style};
use cp_eras_ui::Element;
use iced::{keyboard, Event, Subscription, Task};

/// Launch size. `scroll_to_selected` derives the message-list viewport
/// from the height, so both must share this one number.
const WINDOW_SIZE: (f32, f32) = (1600.0, 900.0);

pub fn main() -> iced::Result {
    let style = match era_from_args() {
        Some(era) => {
            let mut style = era.style();
            let theme = cp_eras_ui::theme::Theme::load();
            if Era::parse(&theme.era) == Some(era) {
                style.palette = style.palette.with_theme(&theme);
            }
            style
        }
        None => Style::from_desktop(),
    };

    iced::application(move || App::new(style), App::update, App::view)
        .title(App::title)
        .theme(|app: &App| app.style)
        .font(fonts::ORBITRON_REGULAR)
        .font(fonts::ORBITRON_MEDIUM)
        .font(fonts::ORBITRON_SEMIBOLD)
        .font(fonts::ORBITRON_BOLD)
        .font(fonts::RAJDHANI_LIGHT)
        .font(fonts::RAJDHANI_REGULAR)
        .font(fonts::RAJDHANI_MEDIUM)
        .font(fonts::RAJDHANI_SEMIBOLD)
        .font(fonts::RAJDHANI_BOLD)
        .default_font(fonts::FONT_RAJDHANI_REGULAR)
        .window_size(WINDOW_SIZE)
        .antialiasing(true)
        .subscription(App::subscription)
        .run()
}

fn era_from_args() -> Option<Era> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if let Some(name) = arg.strip_prefix("--era=") {
            return Era::parse(name);
        }
        if arg == "--era" {
            return args.next().as_deref().and_then(Era::parse);
        }
    }
    None
}

struct App {
    style: Style,
    emails: Vec<Email>,
    selected_id: Option<usize>,
    list_scrollable_id: iced::widget::Id,
    content_scrollable_id: iced::widget::Id,
    focus: MailFocus,
}

impl App {
    fn new(style: Style) -> Self {
        App {
            style,
            ..App::default()
        }
    }

    fn title(&self) -> String {
        format!("MAIL SYSTEM — {}", self.style.era.name())
    }
}

impl Default for App {
    fn default() -> Self {
        let emails = vec![
            Email {
                id: 1,
                title: "Lorem Ipsum Dolor".to_string(),
                sender: "Marcus Aurelius".to_string(),
                body: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\nSystem status report:\n\n| Subsystem | Status | Load |\n|---|---|---|\n| Core ICE | NOMINAL | 42% |\n| Buffer | ACTIVE | 18% |\n| Uplink | STABLE | 88% |\n\nPlease verify credentials.".to_string(),
                is_new: true,
                timestamp: "22:00".to_string(),
                thread: vec![
                    ThreadMessage {
                        sender: "Julius Caesar".to_string(),
                        body: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\nExcepteur sint occaecat cupidatat non proident:\n- Subnet breach detected\n- ICE deployed successfully\n- Packet loss at 12%\n\nStatus: MONITORING.".to_string(),
                        timestamp: "22:02".to_string(),
                    },
                    ThreadMessage {
                        sender: "Cicero".to_string(),
                        body: "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.".to_string(),
                        timestamp: "22:03".to_string(),
                    },
                    ThreadMessage {
                        sender: "Seneca".to_string(),
                        body: "Lorem ipsum dolor sit amet, consectetur adipiscing elit:\n\n* Premium coverage active\n* High-threat zone deployment\n* Life-saving speed guaranteed\n* 24/7 neural monitoring".to_string(),
                        timestamp: "22:05".to_string(),
                    },
                    ThreadMessage {
                        sender: "Pliny".to_string(),
                        body: "Target coordinates and payout:\n\n| Target | Sector | Eddies |\n|---|---|---|\n| T-Bug | Kabuki | 5000 |\n| Vik | Watson | 2000 |".to_string(),
                        timestamp: "22:08".to_string(),
                    },
                    ThreadMessage {
                        sender: "Tacitus".to_string(),
                        body: "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".to_string(),
                        timestamp: "22:09".to_string(),
                    },
                    ThreadMessage {
                        sender: "Lucretius".to_string(),
                        body: "De rerum natura. Analysis of the grid:\n\n- Node 1: STABLE\n- Node 2: UNSTABLE\n- Node 3: DISCONNECTED".to_string(),
                        timestamp: "22:12".to_string(),
                    },
                    ThreadMessage {
                        sender: "Virgil".to_string(),
                        body: "Arma virumque cano. Battle statistics:\n\n| Unit | HP | Armor | Status |\n|---|---|---|---|\n| V | 100 | 150 | OK |\n| Jackie | 0 | 0 | KIA |".to_string(),
                        timestamp: "22:15".to_string(),
                    },
                    ThreadMessage {
                        sender: "Horace".to_string(),
                        body: "Carpe diem. Minimalist list:\n* Live today\n* Trust tomorrow\n* Drink wine".to_string(),
                        timestamp: "22:18".to_string(),
                    },
                    ThreadMessage {
                        sender: "Ovid".to_string(),
                        body: "Metamorphoses. Changing states:\n\n| Before | After | Duration |\n|---|---|---|\n| Human | Cyberpsycho | 12s |\n| Flesh | Metal | Permanent |".to_string(),
                        timestamp: "22:20".to_string(),
                    },
                    ThreadMessage {
                        sender: "Marcus Aurelius".to_string(),
                        body: "Meditations. Conclusion reached: All is opinion. The patient is stable, the netrunners are traced. Job done.".to_string(),
                        timestamp: "22:25".to_string(),
                    }
                ],
            },
            Email {
                id: 2,
                title: "Consectetur Adipiscing".to_string(),
                sender: "Seneca".to_string(),
                body: "Epistulae Morales ad Lucilium. Short advice on cyberware:\n\n* Less is more\n* Protect your wetware\n* Avoid cheap rippers".to_string(),
                is_new: true,
                timestamp: "18:30".to_string(),
                thread: vec![
                    ThreadMessage {
                        sender: "Lucilius".to_string(),
                        body: "But what about the new Sandevistan?\n\n| Model | Speed | Cost |\n|---|---|---|\n| Apogee | +500% | 100k |\n| Falcon | +300% | 50k |".to_string(),
                        timestamp: "18:32".to_string(),
                    },
                    ThreadMessage {
                        sender: "Seneca".to_string(),
                        body: "It is a golden shackle. Useful, but dangerous. Use with caution.".to_string(),
                        timestamp: "18:35".to_string(),
                    }
                ],
            },
            Email {
                id: 3,
                title: "Tempor Incididunt".to_string(),
                sender: "Trauma Team".to_string(),
                body: "Your premium membership details:\n\n| Benefit | Status |\n|---|---|\n| 3m Response | ACTIVE |\n| Aero-dyne | ACTIVE |\n| Platinum Care | ACTIVE |".to_string(),
                is_new: false,
                timestamp: "15:45".to_string(),
                thread: vec![],
            },
            Email {
                id: 4,
                title: "Labore Et Dolore".to_string(),
                sender: "Militech".to_string(),
                body: "Militech logistics update. Delayed items:\n- 10x Heavy Combat Mechs\n- 50x Smart Rifles\n- 100x Grenades\n\nSecurity clearance required.".to_string(),
                is_new: false,
                timestamp: "12:10".to_string(),
                thread: vec![],
            },
            Email {
                id: 5,
                title: "Ut Enim Ad Minim".to_string(),
                sender: "Cicero".to_string(),
                body: "In Catilinam. Quo usque tandem abutere, Catilina, patientia nostra?:\n\n1. Catilina is plotting\n2. The Senate knows\n3. O tempora, o mores!".to_string(),
                is_new: true,
                timestamp: "10:30".to_string(),
                thread: vec![],
            },
            Email {
                id: 6,
                title: "Quis Nostrud".to_string(),
                sender: "Julius Caesar".to_string(),
                body: "De Bello Gallico. Gallia est omnis divisa in partes tres:\n* Belgae\n* Aquitani\n* Celtae\n\nWe conquered them all.".to_string(),
                is_new: false,
                timestamp: "09:15".to_string(),
                thread: vec![],
            },
            Email {
                id: 7,
                title: "Ullamco Laboris".to_string(),
                sender: "Pliny".to_string(),
                body: "Naturalis Historia. Observations on Mount Vesuvius eruption. It was big. Lots of ash. Stay away.".to_string(),
                is_new: false,
                timestamp: "Yesterday".to_string(),
                thread: vec![],
            },
            Email {
                id: 8,
                title: "Aliquip Ex Ea".to_string(),
                sender: "Tacitus".to_string(),
                body: "Annales. The fire of Rome. Nero was playing the lyre. Suspicious:\n\n| suspect | Motive | Alibi |\n|---|---|---|\n| Nero | Rebuilding | Playing lyre |\n| Christians | Scapegoat | None |".to_string(),
                is_new: true,
                timestamp: "Yesterday".to_string(),
                thread: vec![
                    ThreadMessage {
                        sender: "Pliny".to_string(),
                        body: "I agree, Nero is guilty. We should write a letter about it.".to_string(),
                        timestamp: "Yesterday".to_string(),
                    }
                ],
            },
            Email {
                id: 9,
                title: "Duis Aute Irure".to_string(),
                sender: "Lucretius".to_string(),
                body: "De rerum natura. Atoms and void. That's all there is. No gods, no cyberpsychosis, just physics.".to_string(),
                is_new: false,
                timestamp: "2 days ago".to_string(),
                thread: vec![],
            },
            Email {
                id: 10,
                title: "Dolore Eu Fugiat".to_string(),
                sender: "Virgil".to_string(),
                body: "Aeneid. The wooden horse. Timeo Danaos et dona ferentes:\n- It's a trap\n- Don't bring it in\n- Laocoon was right".to_string(),
                is_new: false,
                timestamp: "3 days ago".to_string(),
                thread: vec![],
            },
            Email {
                id: 11,
                title: "Excepteur Sint".to_string(),
                sender: "Horace".to_string(),
                body: "Ars Poetica. How to write good code:\n1. Keep it simple\n2. Don't panic\n3. Use Ref Cell only when needed".to_string(),
                is_new: true,
                timestamp: "3 days ago".to_string(),
                thread: vec![],
            },
            Email {
                id: 12,
                title: "Occaecat Cupidatat".to_string(),
                sender: "Ovid".to_string(),
                body: "Ars Amatoria. How to hack hearts:\n* Be mysterious\n* Send encrypted shards\n* Never reply instantly".to_string(),
                is_new: false,
                timestamp: "4 days ago".to_string(),
                thread: vec![],
            },
            Email {
                id: 13,
                title: "Non Proident".to_string(),
                sender: "Tacitus".to_string(),
                body: "Germania. The tribes of the north. They are tough. They don't use cyberware, just axes. Dangerous.".to_string(),
                is_new: false,
                timestamp: "5 days ago".to_string(),
                thread: vec![],
            },
            Email {
                id: 14,
                title: "Sunt In Culpa".to_string(),
                sender: "Cicero".to_string(),
                body: "De Officiis. On duties. A citizen must:\n- Pay taxes\n- Hack Arasaka\n- Protect the family".to_string(),
                is_new: true,
                timestamp: "1 week ago".to_string(),
                thread: vec![],
            },
            Email {
                id: 15,
                title: "Qui Officia".to_string(),
                sender: "Seneca".to_string(),
                body: "De Brevitate Vitae. Life is short, but cyberware makes it feel longer. Or shorter, if you get shot. Choose wisely.".to_string(),
                is_new: false,
                timestamp: "1 week ago".to_string(),
                thread: vec![],
            },
        ];

        App {
            style: Style::from_desktop(),
            emails,
            selected_id: Some(1),
            list_scrollable_id: iced::widget::Id::unique(),
            content_scrollable_id: iced::widget::Id::unique(),
            focus: MailFocus::List,
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
                // Derived from the panel's own geometry (panels/mail.rs):
                // row pitch and list viewport, not estimates.
                let item_height = message_row_pitch(&self.style.metrics);
                let viewport_height = mail_list_viewport_height(&self.style, WINDOW_SIZE.1);
                let total_items = self.emails.len();

                let target_y = (index as f32) * item_height;
                let total_height = (total_items as f32) * item_height;
                let max_scroll = (total_height - viewport_height).max(0.0);

                let center_offset = target_y - (viewport_height / 2.0) + (item_height / 2.0);
                let final_y = center_offset.clamp(0.0, max_scroll);

                iced::widget::operation::scroll_to(
                    self.list_scrollable_id.clone(),
                    iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: final_y },
                )
            } else {
                Task::none()
            }
        } else {
            Task::none()
        }
    }

    fn delete_email(&mut self, id: usize) -> Task<Message> {
        self.emails.retain(|e| e.id != id);
        if self.selected_id == Some(id) {
            self.selected_id = self.emails.first().map(|e| e.id);
        }
        self.scroll_to_selected()
    }

    fn add_random_email(&mut self) {
        const SENDERS: &[&str] = &[
            "Marcus Aurelius",
            "Julius Caesar",
            "Cicero",
            "Seneca",
            "Pliny",
            "Tacitus",
            "Lucretius",
            "Virgil",
            "Horace",
            "Ovid",
        ];
        const TITLES: &[&str] = &[
            "Lorem Ipsum Dolor",
            "Consectetur Adipiscing",
            "Tempor Incididunt",
            "Labore Et Dolore",
            "Ut Enim Ad Minim",
            "Quis Nostrud",
            "Ullamco Laboris",
            "Aliquip Ex Ea",
            "Duis Aute Irure",
            "Dolore Eu Fugiat",
        ];
        const BODIES: &[&str] = &[
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit:\n\n* Premium coverage active\n* High-threat zone deployment\n* Life-saving speed guaranteed\n* 24/7 neural monitoring",
            "Tempor incididunt ut labore et dolore magna aliqua. System status report:\n\n| Subsystem | Status | Load |\n|---|---|---|\n| Core ICE | NOMINAL | 42% |\n| Buffer | ACTIVE | 18% |\n| Uplink | STABLE | 88% |\n\nPlease verify credentials.",
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\nExcepteur sint occaecat cupidatat non proident:\n- Subnet breach detected\n- ICE deployed successfully\n- Packet loss at 12%\n\nStatus: MONITORING.",
            "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium.\n\nTarget coordinates and payout:\n\n| Target | Sector | Eddies |\n|---|---|---|\n| T-Bug | Kabuki | 5000 |\n| Vik | Watson | 2000 |",
        ];

        let next_id = self.emails.iter().map(|e| e.id).max().unwrap_or(0) + 1;
        let index = next_id;

        // Pseudo-random length between 5 and 20
        let thread_len = 5 + (index * 7 + 3) % 16; // 5 + [0..15] = 5..20

        let mut thread = Vec::new();
        for i in 0..thread_len {
            let sender_index = (index + i * 3 + 1) % SENDERS.len();
            let body_index = (index + i * 7 + 2) % BODIES.len();
            let timestamp = format!("{}m ago", (thread_len - i) * 2);

            thread.push(ThreadMessage {
                sender: SENDERS[sender_index].to_string(),
                body: BODIES[body_index].to_string(),
                timestamp,
            });
        }

        let root_timestamp = format!("{}m ago", (thread_len + 1) * 2);

        let new_email = Email {
            id: next_id,
            title: TITLES[index % TITLES.len()].to_string(),
            sender: SENDERS[index % SENDERS.len()].to_string(),
            body: BODIES[index % BODIES.len()].to_string(),
            is_new: true,
            timestamp: root_timestamp,
            thread,
        };

        self.emails.push(new_email);
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectEmail(id) => {
                self.focus = MailFocus::List;
                return self.select_email(id);
            }
            Message::DeleteEmail(id) => {
                return self.delete_email(id);
            }
            Message::Event(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                    match key {
                        keyboard::Key::Character(c) => {
                            let ctrl = modifiers.control();
                            match c.as_str() {
                                "f" if ctrl => {
                                    return iced::widget::operation::scroll_by(
                                        self.content_scrollable_id.clone(),
                                        iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: 400.0 },
                                    );
                                }
                                "b" if ctrl => {
                                    return iced::widget::operation::scroll_by(
                                        self.content_scrollable_id.clone(),
                                        iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: -400.0 },
                                    );
                                }
                                "a" => {
                                    self.add_random_email();
                                }
                                "d" => {
                                    if let Some(id) = self.selected_id {
                                        return self.delete_email(id);
                                    }
                                }
                                "h" => {
                                    self.focus = MailFocus::List;
                                }
                                "l" => {
                                    if self.selected_id.is_some() {
                                        self.focus = MailFocus::Content;
                                    } else if let Some(first) = self.emails.first() {
                                        self.selected_id = Some(first.id);
                                        self.focus = MailFocus::Content;
                                        return self.scroll_to_selected();
                                    }
                                }
                                "j" => {
                                    match self.focus {
                                        MailFocus::List => {
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
                                        MailFocus::Content => {
                                            return iced::widget::operation::scroll_by(
                                                self.content_scrollable_id.clone(),
                                                iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: 30.0 },
                                            );
                                        }
                                    }
                                }
                                "k" => {
                                    match self.focus {
                                        MailFocus::List => {
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
                                        MailFocus::Content => {
                                            return iced::widget::operation::scroll_by(
                                                self.content_scrollable_id.clone(),
                                                iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: -30.0 },
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::Event)
    }

    fn view(&self) -> Element<'_, Message> {
        mail_panel(
            &self.style,
            &self.emails,
            self.selected_id,
            Message::SelectEmail,
            Message::DeleteEmail,
            self.list_scrollable_id.clone(),
            self.content_scrollable_id.clone(),
            self.focus,
        )
    }
}
