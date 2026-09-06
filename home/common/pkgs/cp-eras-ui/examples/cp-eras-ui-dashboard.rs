//! The module-hub dashboard, in any era.
//!
//!     cp-eras-ui-dashboard                # follow the desktop theme
//!     cp-eras-ui-dashboard --era kitsch   # force one
//!
//! `shell` decides the era and loads the faces; see there for the
//! `--era` reasoning.

use cp_eras_ui::screens::dashboard::{Dashboard, Message};
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    shell::application(move || Dashboard::new(style), Dashboard::update, Dashboard::view)
        .title(Dashboard::title)
        .subscription(Dashboard::subscription)
        .run()
}

#[allow(dead_code)]
fn _assert_message(_: Message) {}
