//! The module-hub dashboard, in any era -- and the screens behind it.
//!
//!     cp-eras-ui-dashboard                # follow the desktop theme
//!     cp-eras-ui-dashboard --era kitsch   # force one
//!
//! Opens on the dashboard. `h j k l` walk the menu, Enter or a click
//! opens the module's screen (the mailbox, the store) and `m` / `s`
//! open those two from anywhere on the menu, `h j k l` move inside a
//! screen and Esc comes back: `screens::hub`. `shell` decides the
//! era and loads the faces; see there for the `--era` reasoning.
//!
//! Two capture-only environment variables, for headless renders
//! (`scripts/render.sh` passes its environment through), since a
//! headless compositor has no keyboard or pointer to drive:
//!
//! - `CP_ERAS_UI_MODULE=<0..5>` opens on that module instead of the
//!   era's `dashboard_selection`, as if the keys had moved there.
//! - `CP_ERAS_UI_MODULE_STATE=hover|pressed` draws the selected module
//!   in the drawing a pointer hovering / holding it gets
//!   (`PlateStates::selected_hover` / `selected_pressed`, each falling
//!   back to `hover` / `pressed` as `scene` does), by putting that
//!   drawing in the plate's `on` slot. The pointer and hit boxes are
//!   untouched; it is a picture of the state, not the state.
//!
//! Neither is read by the library, and with neither set the binary is
//! exactly what it was.

use cp_eras_ui::screens::hub::{Hub, Message};
use cp_eras_ui::shell;
use cp_eras_ui::style::{Group, Prim, Style};

fn main() -> iced::Result {
    let style = capture_state(shell::style());
    let module = std::env::var("CP_ERAS_UI_MODULE").ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|&m| m < 6);
    shell::application(move || {
        let mut hub = Hub::new(style);
        if let Some(m) = module { hub.dashboard.selected = m; }
        hub
    }, Hub::update, Hub::view)
        .title(Hub::title)
        .subscription(Hub::subscription)
        .run()
}

/// `CP_ERAS_UI_MODULE_STATE`: every module plate's `on` drawing replaced
/// by its selected hover or pressed drawing. Only the selected plate
/// paints `on`, so only it shows the state.
fn capture_state(mut style: Style) -> Style {
    let Ok(state) = std::env::var("CP_ERAS_UI_MODULE_STATE") else { return style };
    let pick = |s: &cp_eras_ui::style::PlateStates| match state.trim() {
        "hover" => Some(s.selected_hover.unwrap_or(s.hover)),
        "pressed" => Some(s.selected_pressed.unwrap_or(s.pressed)),
        _ => None,
    };
    let states: Vec<(usize, &'static [Prim])> = style.dashboard_states.iter()
        .filter(|s| s.group == Group::Module)
        .filter_map(|s| pick(s).map(|p| (s.index, p)))
        .collect();
    style.dashboard = swap_on(style.dashboard, &states);
    style
}

fn swap_on(prims: &'static [Prim], states: &[(usize, &'static [Prim])]) -> &'static [Prim] {
    let out: Vec<Prim> = prims.iter().map(|&p| match p {
        Prim::Plate { group: Group::Module, index, x, y, w, h, on, off } => {
            let on = states.iter().find(|(i, _)| *i == index).map_or(on, |&(_, s)| s);
            Prim::Plate { group: Group::Module, index, x, y, w, h, on, off }
        }
        Prim::At { x, y, prims } => Prim::At { x, y, prims: swap_on(prims, states) },
        other => other,
    }).collect();
    Box::leak(out.into_boxed_slice())
}

#[allow(dead_code)]
fn _assert_message(_: Message) {}
