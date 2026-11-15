# iced_layershell 0.19.1 local patch

This is the crates.io `iced_layershell` 0.19.1 source (the Cargo registry
crate, `waycrate/exwlshelleventloop`), MIT per its `Cargo.toml`; the
published crate ships no LICENSE file. `Cargo.toml`, `README.md`, `src/`
and `tests/` are as published (`Cargo.toml.orig`, `Cargo.lock` and
`.cargo-ok` dropped). `README.md` and `src/build_pattern/*.md` are
compiled in with `include_str!`, which is why `default.nix` lets them
through its source filter.

## The change: find a closed window by its layer id

`src/multi_window.rs`, `handle_closed_event`, marked `cp-eras-ui patch`.

Upstream finds the iced window of a closed layer surface through the
layershellev unit's *binding*. A window made in `handle_refresh_event` for
a unit that has none -- every surface `StartMode::AllScreens` puts on an
output -- takes a fresh `window::Id` and never writes it back, so the
lookup fails and the handler returns: the window (its renderer, wgpu
surface and swapchain, user interface) is never removed and the
application is never sent `window::Event::Closed`. On the bar that was
the leak of `todo/bar.md`: about 24 MB and one thread retained per output
re-creation on a 3840x2160 headless sway output (16 MB on terra's
Hyprland). Patched: when the unit has no binding, or is already gone,
the window is found through the window manager's own layer-id alias,
which `WindowManager::insert` records for every window.

Its companion is the `layershellev` patch (`vendor/layershellev/PATCHES.md`),
which keeps an output's units alive until this handler has run and then
destroys their layer surfaces. Either alone frees the window; the pair
also releases the protocol objects.

Remove this vendor copy and the `[patch.crates-io]` entry when an upstream
release resolves a closed window without the unit's binding (or records
the binding for the windows it makes in `handle_refresh_event`).
