//! The module-hub dashboard, in any era -- and the screens behind it.
//!
//!     cp-eras-ui-dashboard                # follow the desktop theme
//!     cp-eras-ui-dashboard --era kitsch   # force one
//!
//! Opens on the dashboard. `h j k l` walk the menu, Enter or a click
//! opens the module's screen (the mailbox, the store), `h j k l` move
//! inside it and Esc comes back: `screens::hub`. `shell` decides the
//! era and loads the faces; see there for the `--era` reasoning.

use cp_eras_ui::screens::hub::{Hub, Message};
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    shell::application(move || Hub::new(style), Hub::update, Hub::view)
        .title(Hub::title)
        .subscription(Hub::subscription)
        .run()
}

#[allow(dead_code)]
fn _assert_message(_: Message) {}
