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

use cp_eras_ui::screens::nav;
use cp_eras_ui::screens::store::Store;
use cp_eras_ui::shell;

fn main() -> iced::Result {
    let style = shell::style();
    shell::application(move || Store::new(style), Store::update, Store::view)
        .title(Store::title)
        .subscription(|_| nav::strokes().filter_map(Store::stroke))
        .run()
}
