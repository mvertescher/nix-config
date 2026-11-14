//! The 4ST store, in any era.
//!
//!     cp-eras-ui-store                # follow the desktop theme
//!     cp-eras-ui-store --era kitsch   # force one
//!
//! Rendering the same screen in each era side by side is the quickest
//! way to see whether the toolkit's claim holds -- and to compare
//! against `docs/<era>/store-trace.svg`, which is what
//! `scripts/fidelity_check.sh --implementation <era> store` holds this
//! binary to. `shell` decides the era and loads the faces; see there
//! for the `--era` reasoning.
//!
//! The screen is live: clicking a category or a card selects it, `h j
//! k l` walk to the nearest one, and the era's own table says which
//! drawing each wears. It opens on the
//! selection its trace shows -- entropism's first card, everyone
//! else's second -- so a capture of it is comparable with the trace.
//!
//! Two capture-only environment variables, for headless renders of a
//! moved selection (`scripts/render.sh` passes its environment through;
//! a headless compositor has no pointer to click with):
//! `CP_ERAS_UI_CATEGORY=<0..4>` and `CP_ERAS_UI_CARD=<0..3>` open on that
//! category / card instead of the era's `store_selection`. Neither is
//! read by the library; with neither set the binary is unchanged.

use cp_eras_ui::screens::nav;
use cp_eras_ui::screens::store::Store;
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    // The era's print material over the whole frame (`print`).
    cp_eras_ui::print::wear(style.era);
    let pick = |name: &str, below: usize| std::env::var(name).ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|&i| i < below);
    let (category, card) = (pick("CP_ERAS_UI_CATEGORY", 5), pick("CP_ERAS_UI_CARD", 4));
    shell::application(move || {
        let mut store = Store::new(style);
        if let Some(i) = category { store.category = i; }
        if let Some(i) = card { store.card = i; }
        store
    }, Store::update, Store::view)
        .title(Store::title)
        .subscription(|store| iced::Subscription::batch([
            nav::strokes().filter_map(Store::stroke),
            store.subscription(),
        ]))
        .run()
}
