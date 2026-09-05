//! The login screen, in any era.
//!
//!     cp-eras-ui-login                # follow the desktop theme
//!     cp-eras-ui-login --era kitsch   # force one
//!
//! `shell` decides the era and loads the faces; see there for the
//! `--era` reasoning.
//!
//! The screen's *content* -- how many accounts are offered, what they
//! are called, which one is live, and every string on the page -- is
//! era table data (`Style::access`), transcribed from
//! `docs/<era>/login-trace.svg`. So this file only picks the era and
//! opens the frame the traces are measured in, which is the one
//! `scripts/fidelity_check.sh --implementation <era> login` captures.

use cp_eras_ui::screens::login::{Login, Message};
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    shell::application(move || Login::new(style), Login::update, Login::view)
        .title(Login::title)
        .run()
}

#[allow(dead_code)]
fn _assert_message(_: Message) {}
