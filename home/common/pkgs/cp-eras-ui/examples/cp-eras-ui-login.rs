//! The login screen, in any era -- and the greeter.
//!
//!     cp-eras-ui-login                # follow the desktop theme
//!     cp-eras-ui-login --era kitsch   # force one
//!     cp-eras-ui-login --greet --era neomil --user mverte \
//!         --cmd 'uwsm start hyprland-uwsm.desktop'
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
//!
//! With `--greet` the screen is a greetd greeter: Enter sends the
//! secret to `$GREETD_SOCK` for `--user`, and when greetd accepts it
//! the process asks for `--cmd` as the session and exits 0, so a
//! `cage -s -- cp-eras-ui-login --greet ...` session hands the seat
//! over. The greeter account has no desktop theme to follow, so
//! `--era` is how it is dressed. Without `--greet` it is the demo:
//! typing shows, Enter clears.

use cp_eras_ui::screens::login::{Greeter, Login};
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let style = shell::style();
    let greeter = if shell::switch(&args, "--greet") {
        let user = shell::flag(&args, "--user").unwrap_or_else(|| usage("--greet needs --user <name>"));
        let cmd = shell::flag(&args, "--cmd")
            .unwrap_or_else(|| usage("--greet needs --cmd <session command>"));
        Some(Greeter {
            user,
            cmd: vec![cmd],
        })
    } else {
        None
    };
    shell::application(
        move || Login::greeting(style, greeter.clone()),
        Login::update,
        Login::view,
    )
    .subscription(Login::subscription)
    .title(Login::title)
    .run()
}

fn usage(why: &str) -> ! {
    eprintln!("cp-eras-ui-login: {why}");
    eprintln!("usage: cp-eras-ui-login [--era <era>] [--greet --user <name> --cmd <command>]");
    std::process::exit(2)
}
