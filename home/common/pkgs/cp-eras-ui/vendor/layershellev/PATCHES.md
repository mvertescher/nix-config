# layershellev 0.19.1 local patch

This is the crates.io `layershellev` 0.19.1 source (the Cargo registry
crate, `waycrate/exwlshelleventloop`), MIT per its `Cargo.toml`; the
published crate ships no LICENSE file. `Cargo.toml`, `README.md`, `src/`
and `examples/` are as published (`Cargo.toml.orig`, `Cargo.lock` and
`.cargo-ok` dropped).

## The change: an output's surfaces are closed, not dropped

`src/lib.rs`, three places, each marked `cp-eras-ui patch`.

Upstream, `OutputHandler::output_destroyed` *extracts* every unit bound to
the removed `wl_output` from `units` and queues its id in `closed_ids`.
The event loop then dispatches `DispatchMessage::Closed` for each queued
id -- but the unit is already gone, so iced_layershell's
`handle_closed_event` (`multi_window.rs`), which finds the iced window
through `ev.get_unit_with_id`, returns early. The iced window (renderer,
wgpu surface and swapchain, user interface) is never removed, the app is
never sent `window::Event::Closed`, and nobody sends
`zwlr_layer_surface_v1.destroy` / `wl_surface.destroy` either.

Confirmed with `WAYLAND_DEBUG=1` on headless sway 1.12: when an output is
unplugged the compositor sends `zwlr_layer_surface_v1.closed()` and
`wl_registry.global_remove` in one batch, so the extraction always wins
and even the `closed` path finds no unit. Measured with
`cp-eras-ui-bar` in `StartMode::AllScreens` on a 3840x2160 headless output:
about 24 MB resident retained per output re-creation (16 MB on terra's
Hyprland, `todo/bar.md`).

Patched, three places: `output_destroyed` only queues the ids and leaves the units in
place. The queued ids are dispatched as `Closed` while the unit still
exists, then removed with `remove_shell` (which destroys the layer
surface and wl_surface). An id
the `request_close` path already handled in the same iteration is
skipped. And `remove_shell` no longer requires `becreated` (a unit made
by a `NewLayerShell` request): upstream left an `AllScreens` unit the
compositor closed in `units` with its close flag up, to be closed again
on every pass of the loop, its layer surface never destroyed.

The matching iced_layershell patch (`vendor/iced_layershell/PATCHES.md`)
is what actually frees the window: iced_layershell looked the window up
through the unit's binding, which `AllScreens` units never carry, so
this change alone moved nothing (measured: still ~24 MB per cycle). With
both, 10 cycles retain about 10 MB in all, flattening to ~0.3 MB a cycle,
and the thread count holds.

Remove this vendor copy and the `[patch.crates-io]` entry when an upstream
`layershellev` release dispatches `Closed` for an output's units before
dropping them (or iced_layershell resolves a closed id through its own
window aliases).
