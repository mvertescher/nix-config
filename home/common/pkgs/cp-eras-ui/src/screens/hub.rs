//! The screens as one application: the dashboard, with the mailbox
//! and the store behind its modules.
//!
//! Each screen stays what it was -- a `Style` reader with its own
//! state, messages and view, runnable on its own -- and this is the
//! router over them. The dashboard is the front: `h j k l` walk its
//! menu, and Enter (or a click) on a module opens the screen the era's
//! [`crate::style::Style::dashboard_destinations`] puts behind it.
//! Inside a screen the same keys move its own selection, and Esc comes
//! back to the dashboard. That is the whole grammar, and the eras own
//! nothing of it: they say which module leads where, and draw.
//!
//! The module labels are the photos' own, and no era labels both
//! screens: entropism and neokitsch have a mailbox module and no store,
//! kitsch and neomil the reverse. The screen no module names is a key
//! away instead -- `m` opens the mailbox and `s` the store, from the
//! dashboard whatever is selected -- so a module with nothing behind it
//! selects and stays, and the tables no longer stand the last module
//! in for the missing screen. The keys are the hub's ([`hotkey`]), not
//! [`nav::Stroke`]'s: a stroke is what every screen reads, and inside
//! a screen `m` and `s` mean nothing.
//!
//! Each screen keeps its own clock. The dashboard's boot-in runs once,
//! when the hub starts, and a return from a screen finds the panel
//! where the boot-in left it rather than replaying it -- Esc is a
//! close, not an open (`motion`, the module note, has the reasoning).
//! The store and the mailbox come up when Enter or a key opens them,
//! so [`Hub::go`] re-enters the screen it goes to and its boot-in
//! plays from that moment; under a pinned clock nothing re-bases and
//! every screen draws at the pinned moment.
//!
//! `cp-eras-ui-dashboard` runs this; the goldens see its opening
//! frame, which is the dashboard's own.

use crate::screens::dashboard::{self, Dashboard};
use crate::screens::mail::{self, MailBox};
use crate::screens::nav::{self, Stroke};
use crate::screens::store::{self, Store};
use crate::style::{Destination, Style};
use crate::Element;
use iced::keyboard::{self, Key};
use iced::Subscription;

/// Which screen is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Dashboard,
    Mail,
    Store,
}

pub struct Hub {
    pub dashboard: Dashboard,
    pub mail: MailBox,
    pub store: Store,
    pub route: Route,
}

#[derive(Debug, Clone)]
pub enum Message {
    Dashboard(dashboard::Message),
    Mail(mail::Message),
    Store(store::Message),
    Stroke(Stroke),
    /// `m` or `s` on the dashboard: straight to that screen.
    Go(Destination),
}

impl crate::shell::Wears for Hub {
    fn wears(&self) -> Style {
        self.dashboard.style
    }
}

impl Hub {
    pub fn new(style: Style) -> Self {
        Hub {
            dashboard: Dashboard::new(style),
            mail: MailBox::new(style),
            store: Store::new(style),
            route: Route::Dashboard,
        }
    }

    pub fn title(&self) -> String {
        match self.route {
            Route::Dashboard => self.dashboard.title(),
            Route::Mail => self.mail.title(),
            Route::Store => self.store.title(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            // A click on a module both selects and opens it: the
            // request was that the email shard *takes you* to the mail,
            // and the detail panel is only ever a caption of the
            // selection, which the keyboard still browses.
            Message::Dashboard(m @ dashboard::Message::Select { .. }) => {
                self.dashboard.update(m);
                self.open();
            }
            Message::Dashboard(m) => self.dashboard.update(m),
            Message::Mail(m) => self.mail.update(m),
            Message::Store(m) => self.store.update(m),
            Message::Stroke(Stroke::Move(dir)) => match self.route {
                Route::Dashboard => self.dashboard.update(dashboard::Message::Move(dir)),
                Route::Mail => self.mail.update(mail::Message::Move(dir)),
                Route::Store => self.store.update(store::Message::Move(dir)),
            },
            Message::Stroke(Stroke::Open) => {
                if self.route == Route::Dashboard {
                    self.open();
                }
            }
            Message::Stroke(Stroke::Back) => self.route = Route::Dashboard,
            Message::Go(to) => {
                if self.route == Route::Dashboard {
                    self.go(to);
                }
            }
        }
    }

    /// Go where the dashboard's selection leads, if anywhere.
    fn open(&mut self) {
        if let Some(to) = self.dashboard.destination() {
            self.go(to);
        }
    }

    /// Go to a screen and start its clock: it is coming up now. Enter,
    /// a click and the `m`/`s` keys all come through here, so a screen
    /// boots in the same way however it was reached.
    fn go(&mut self, to: Destination) {
        match to {
            Destination::Mail => {
                self.mail.enter();
                self.route = Route::Mail;
            }
            Destination::Store => {
                self.store.enter();
                self.route = Route::Store;
            }
        }
    }

    /// The keyboard, whole, plus the showing screen's clock while its
    /// boot-in runs. The screens' own `stroke` maps are not used here:
    /// the hub reads every stroke itself, because Open and Back are
    /// route changes and the screens do not know they are in one. The
    /// `m`/`s` keys ride alongside as [`hotkeys`], and `update` is
    /// what confines them to the dashboard.
    pub fn subscription(&self) -> Subscription<Message> {
        let clock = match self.route {
            Route::Dashboard => self.dashboard.subscription().map(Message::Dashboard),
            Route::Mail => self.mail.subscription().map(Message::Mail),
            Route::Store => self.store.subscription().map(Message::Store),
        };
        Subscription::batch([
            nav::strokes().map(Message::Stroke),
            hotkeys().map(Message::Go),
            clock,
        ])
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.route {
            Route::Dashboard => self.dashboard.view().map(Message::Dashboard),
            Route::Mail => self.mail.view().map(Message::Mail),
            Route::Store => self.store.view().map(Message::Store),
        }
    }
}

/// The screen a key goes to, if it is one of the two: `m` the mailbox,
/// `s` the store. Chords are not, as in [`nav::stroke`].
pub fn hotkey(key: &Key, modifiers: keyboard::Modifiers) -> Option<Destination> {
    if modifiers.control() || modifiers.alt() || modifiers.logo() {
        return None;
    }
    match key.as_ref() {
        Key::Character("m") => Some(Destination::Mail),
        Key::Character("s") => Some(Destination::Store),
        _ => None,
    }
}

/// Every key press on the window that is a [`hotkey`].
fn hotkeys() -> Subscription<Destination> {
    iced::event::listen_with(|event, _status, _window| match event {
        iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            hotkey(&key, modifiers)
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion;
    use crate::screens::nav::Dir;
    use crate::style::Era;
    use iced::keyboard::Modifiers;

    /// The modules that label a screen, per era: the photos' own
    /// lists (`docs/sources.md`). Everything else selects and stays.
    fn labelled(era: Era) -> Vec<(usize, Destination)> {
        match era {
            Era::Entropism => vec![(0, Destination::Mail)],
            Era::Neokitsch => vec![(0, Destination::Mail)],
            Era::Kitsch => vec![(2, Destination::Store), (3, Destination::Store)],
            Era::Neomil => vec![(4, Destination::Store)],
        }
    }

    /// The labelled modules lead where their labels say, and no other
    /// module leads anywhere: the stand-ins are gone.
    #[test]
    fn only_labelled_modules_lead_anywhere() {
        for era in Era::ALL {
            let labelled = labelled(era);
            for index in 0..6 {
                let mut hub = Hub::new(era.style());
                hub.update(Message::Dashboard(dashboard::Message::Select { index }));
                assert_eq!(hub.dashboard.selected, index);
                let want = match labelled.iter().find(|(i, _)| *i == index) {
                    Some((_, Destination::Mail)) => Route::Mail,
                    Some((_, Destination::Store)) => Route::Store,
                    None => Route::Dashboard,
                };
                assert_eq!(hub.route, want, "{} module {}", era.name(), index);
            }
        }
    }

    /// `m` and `s` are the keys and nothing else is; chords are left
    /// to the terminal.
    #[test]
    fn the_keys_decode() {
        let none = Modifiers::empty();
        assert_eq!(hotkey(&Key::Character("m".into()), none), Some(Destination::Mail));
        assert_eq!(hotkey(&Key::Character("s".into()), none), Some(Destination::Store));
        assert_eq!(hotkey(&Key::Character("h".into()), none), None);
        assert_eq!(hotkey(&Key::Character("m".into()), Modifiers::CTRL), None);
        assert_eq!(hotkey(&Key::Character("s".into()), Modifiers::ALT), None);
    }

    /// `m` opens the mailbox and `s` the store from every module of
    /// every era, labelled or not; Esc comes back and the selection
    /// survives. Inside a screen the keys mean nothing.
    #[test]
    fn the_keys_open_the_screens_from_any_selection() {
        for era in Era::ALL {
            for index in 0..6 {
                let mut hub = Hub::new(era.style());
                hub.dashboard.selected = index;
                for (to, route) in [(Destination::Mail, Route::Mail), (Destination::Store, Route::Store)] {
                    hub.update(Message::Go(to));
                    assert_eq!(hub.route, route, "{} module {}", era.name(), index);
                    hub.update(Message::Go(Destination::Mail));
                    hub.update(Message::Go(Destination::Store));
                    assert_eq!(hub.route, route, "a key inside a screen is not a route change");
                    hub.update(Message::Stroke(Stroke::Back));
                    assert_eq!(hub.route, Route::Dashboard);
                    assert_eq!(hub.dashboard.selected, index);
                }
            }
        }
    }

    /// Enter on the email module opens the mail, Esc comes back, and
    /// the dashboard's selection survives the trip.
    #[test]
    fn enter_opens_and_escape_returns() {
        let mut hub = Hub::new(Era::Neokitsch.style());
        assert_eq!(hub.route, Route::Dashboard);
        assert_eq!(hub.dashboard.selected, 0);
        hub.update(Message::Stroke(Stroke::Open));
        assert_eq!(hub.route, Route::Mail);
        hub.update(Message::Stroke(Stroke::Back));
        assert_eq!(hub.route, Route::Dashboard);
        assert_eq!(hub.dashboard.selected, 0);
    }

    /// A click on a module goes straight through.
    #[test]
    fn a_click_on_a_module_opens_it() {
        let mut hub = Hub::new(Era::Entropism.style());
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 0 }));
        assert_eq!(hub.route, Route::Mail);
        assert_eq!(hub.dashboard.selected, 0);
    }

    /// A module with nothing behind it selects and stays -- and that
    /// now includes the last module, which used to stand in for the
    /// screen the era does not label.
    #[test]
    fn an_empty_module_only_selects() {
        let mut hub = Hub::new(Era::Neomil.style());
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 1 }));
        assert_eq!(hub.route, Route::Dashboard);
        assert_eq!(hub.dashboard.selected, 1);
        hub.update(Message::Stroke(Stroke::Open));
        assert_eq!(hub.route, Route::Dashboard);
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 5 }));
        assert_eq!(hub.route, Route::Dashboard, "CORPORATIONS no longer stands in for the mailbox");
        hub.update(Message::Stroke(Stroke::Open));
        assert_eq!(hub.route, Route::Dashboard);
    }

    /// Moves go to the screen that is showing, and Enter inside a
    /// screen is not a route change.
    #[test]
    fn moves_go_to_the_showing_screen() {
        let mut hub = Hub::new(Era::Kitsch.style());
        let card = hub.store.card;
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 2 }));
        assert_eq!(hub.route, Route::Store);
        hub.update(Message::Stroke(Stroke::Move(Dir::Right)));
        assert_ne!(hub.store.card, card, "l on the shelf moves the card");
        hub.update(Message::Stroke(Stroke::Open));
        assert_eq!(hub.route, Route::Store);
        assert_eq!(hub.dashboard.selected, 2, "the store's keys never reach the dashboard");
    }

    /// Opening a screen starts its clock: a store that has been ticking
    /// for seconds comes up from its own t = 0 when Enter reaches it,
    /// and the dashboard's clock is not touched by the trip. Under a
    /// pinned clock (`CP_ERAS_UI_AT_MS`) nothing re-bases, and the
    /// assertion is that the pinned moment holds.
    #[test]
    fn opening_a_screen_starts_its_clock() {
        use std::time::{Duration, Instant};
        let mut hub = Hub::new(Era::Kitsch.style());
        hub.store.update(store::Message::Tick(Instant::now() + Duration::from_secs(5)));
        assert!(hub.store.at() >= Duration::from_secs(5) || motion::frozen());
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 2 }));
        assert_eq!(hub.route, Route::Store);
        if motion::frozen() {
            assert_eq!(hub.store.at(), motion::now() - motion::origin());
        } else {
            assert!(hub.store.at() < Duration::from_millis(100), "{:?}", hub.store.at());
        }
        hub.update(Message::Stroke(Stroke::Back));
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 2 }));
        if !motion::frozen() {
            assert!(hub.store.at() < Duration::from_millis(100), "a second open re-bases too");
        }
    }

    /// A key opens a screen by the same path as Enter, so the mailbox
    /// `m` reaches in kitsch -- which no module there leads to -- comes
    /// up from its own t = 0 just the same.
    #[test]
    fn opening_by_key_starts_the_clock_too() {
        use std::time::{Duration, Instant};
        let mut hub = Hub::new(Era::Kitsch.style());
        hub.mail.update(mail::Message::Tick(Instant::now() + Duration::from_secs(5)));
        assert!(hub.mail.at() >= Duration::from_secs(5) || motion::frozen());
        hub.update(Message::Go(Destination::Mail));
        assert_eq!(hub.route, Route::Mail);
        if motion::frozen() {
            assert_eq!(hub.mail.at(), motion::now() - motion::origin());
        } else {
            assert!(hub.mail.at() < Duration::from_millis(100), "{:?}", hub.mail.at());
        }
    }

    /// Every era's dashboard can be walked: from the opening selection
    /// some direction leads somewhere, and walking never leaves the six.
    #[test]
    fn the_menu_can_be_walked_in_every_era() {
        for era in Era::ALL {
            let mut hub = Hub::new(era.style());
            let mut seen = std::collections::BTreeSet::new();
            for dir in [Dir::Left, Dir::Down, Dir::Up, Dir::Right] {
                hub.update(Message::Stroke(Stroke::Move(dir)));
                assert!(hub.dashboard.selected < 6, "{}", era.name());
                seen.insert(hub.dashboard.selected);
            }
            assert!(seen.len() > 1, "{} menu is dead to the keys", era.name());
        }
    }
}
