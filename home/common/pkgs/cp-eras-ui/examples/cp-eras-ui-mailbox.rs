//! The mailbox screen, in any era.
//!
//!     cp-eras-ui-mailbox                # follow the desktop theme
//!     cp-eras-ui-mailbox --era kitsch   # force one
//!
//! A click or `j`/`k` picks a row. `shell` decides the era and loads
//! the faces; see there for the `--era` reasoning.
//!
//! One capture-only environment variable, for headless renders of a
//! moved selection: `CP_ERAS_UI_MAIL_ROW=<n>` opens with row `n` picked,
//! as a click on it would. Not read by the library; unset, the binary
//! is unchanged.

use cp_eras_ui::screens::mail::{MailBox, Message};
use cp_eras_ui::screens::nav;
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    // The era's print material over the whole frame (`print`).
    cp_eras_ui::print::wear(style.era);
    let row = std::env::var("CP_ERAS_UI_MAIL_ROW").ok().and_then(|v| v.trim().parse::<usize>().ok());
    shell::application(move || {
        let mut mail = MailBox::new(style);
        if let Some(row) = row { mail.update(Message::Select(row)); }
        mail
    }, MailBox::update, MailBox::view)
        .title(MailBox::title)
        .subscription(|mail| {
            iced::Subscription::batch([
                nav::strokes().filter_map(MailBox::stroke),
                mail.subscription(),
            ])
        })
        .run()
}

#[allow(dead_code)]
fn _assert_message(_: Message) {}
