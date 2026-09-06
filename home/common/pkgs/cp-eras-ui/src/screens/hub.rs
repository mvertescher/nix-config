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
//! The clock is the dashboard's. Its boot-in runs once, when the hub
//! starts, and a return from a screen finds the panel where the
//! boot-in left it rather than replaying it -- the dashboard's `now`
//! carries on across the route change because the dashboard is never
//! rebuilt.
//!
//! `cp-eras-ui-dashboard` runs this; the goldens see its opening
//! frame, which is the dashboard's own.

use crate::screens::dashboard::{self, Dashboard};
use crate::screens::mail::{self, MailBox};
use crate::screens::nav::{self, Stroke};
use crate::screens::store::{self, Store};
use crate::style::{Destination, Style};
use crate::Element;
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
        }
    }

    /// Go where the dashboard's selection leads, if anywhere.
    fn open(&mut self) {
        self.route = match self.dashboard.destination() {
            Some(Destination::Mail) => Route::Mail,
            Some(Destination::Store) => Route::Store,
            None => return,
        };
    }

    /// The keyboard, whole, plus the dashboard's clock while its
    /// boot-in runs. The screens' own `stroke` maps are not used here:
    /// the hub reads every stroke itself, because Open and Back are
    /// route changes and the screens do not know they are in one.
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            nav::strokes().map(Message::Stroke),
            self.dashboard.subscription().map(Message::Dashboard),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screens::nav::Dir;
    use crate::style::Era;

    /// In every era both screens are behind some module, so a hub that
    /// only has a keyboard can reach all three screens.
    #[test]
    fn every_era_leads_to_both_screens() {
        for era in Era::ALL {
            let leads = era.style().dashboard_destinations;
            assert!(leads.contains(&Some(Destination::Mail)), "{} has no way to the mail", era.name());
            assert!(leads.contains(&Some(Destination::Store)), "{} has no way to the store", era.name());
        }
    }

    /// The labelled modules lead where their labels say.
    #[test]
    fn labelled_modules_lead_where_they_say() {
        let leads = |era: Era| era.style().dashboard_destinations;
        assert_eq!(leads(Era::Entropism)[0], Some(Destination::Mail), "entropism EMAILS");
        assert_eq!(leads(Era::Neokitsch)[0], Some(Destination::Mail), "neokitsch EMAIL");
        assert_eq!(leads(Era::Kitsch)[2], Some(Destination::Store), "kitsch PRODUCTS");
        assert_eq!(leads(Era::Kitsch)[3], Some(Destination::Store), "kitsch PRODUCTS");
        assert_eq!(leads(Era::Neomil)[4], Some(Destination::Store), "neomil PRODUCTS");
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
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 5 }));
        assert_eq!(hub.route, Route::Store);
        assert_eq!(hub.dashboard.selected, 5);
    }

    /// A module with nothing behind it selects and stays.
    #[test]
    fn an_empty_module_only_selects() {
        let mut hub = Hub::new(Era::Neomil.style());
        hub.update(Message::Dashboard(dashboard::Message::Select { index: 1 }));
        assert_eq!(hub.route, Route::Dashboard);
        assert_eq!(hub.dashboard.selected, 1);
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
