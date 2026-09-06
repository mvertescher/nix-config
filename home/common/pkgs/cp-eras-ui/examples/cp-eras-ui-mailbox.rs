//! The mailbox screen, in any era.
//!
//!     cp-eras-ui-mailbox                # follow the desktop theme
//!     cp-eras-ui-mailbox --era kitsch   # force one
//!
//! A click or `j`/`k` picks a row. `shell` decides the era and loads
//! the faces; see there for the `--era` reasoning.

use cp_eras_ui::screens::mail::{MailBox, Message};
use cp_eras_ui::screens::nav;
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    shell::application(move || MailBox::new(style), MailBox::update, MailBox::view)
        .title(MailBox::title)
        .subscription(|_| nav::strokes().filter_map(MailBox::stroke))
        .run()
}

#[allow(dead_code)]
fn _assert_message(_: Message) {}
