# Live desktop check, 2026-11-13 (Neomil)

The first run of these apps on the real desktop: terra, Hyprland, one
3840x2160 output at scale 1, RTX 4070, the installed package
(`cp-eras-ui-0.0.0`, crate at `f733e1f`), theme `neomil/reference`. Every
earlier check was headless. Matt authorised launching the apps, sending
test input and taking screenshots; no real password was typed.

Method: windows opened with `hyprctl dispatch exec`, keys from `wtype`,
pointer from `hyprctl dispatch movecursor` and `wlrctl pointer click`,
frames from `grim`. Timings marked "small capture" poll a 600x600 region
(78 ms a sample); full-frame captures cost about 3 s each and are too
coarse to time anything.

## What works

- **The greeter.** `cp-eras-ui-login --greet` was the login screen on real
  hardware and greetd opened Matt's session from it (journal, 09:01:14).
- **The bar** draws at the top of the output (layer 3840x31): workspace
  chips, NET, VOL, CPU, MEM, date, clock, updating.
- **Dashboard hub**: draws at 3822x2111. Right, Down, Enter from the
  starting selection opens the store ("4ST STORE"); Escape returns.
- **Store**: draws correctly at 4K. Floating at 1600x900 and re-tiling
  both redraw.
- **Login (windowed)**: typed text appears masked; Backspace edits; Enter
  keeps the window (mock submit).
- **Mailbox**: Down moves the selected row and the message pane follows.
- **Warm route changes** (small capture): first change 200-370 ms after
  the key, settled in 0.7-1.0 s. Idle CPU 0%.

## Findings

1. **The bar dies when the output is re-created, and stays dead.** 22 s
   after login the compositor removed and re-added the output (hypridle
   logged the `wl_output` going and coming back at 09:01:40); the bar's
   layer surface closed, the bar exited 0, and `Restart=on-failure` did
   not bring it back. The session had no bar. Mitigated in
   `home/themes/lib/era.nix` (`Restart=always`), then fixed the same day in
   the bar (`StartMode::AllScreens`); verified live, see `todo/bar.md`.
2. **Cold starts are slow at 4K.** Window mapped after 3.6 s (dashboard),
   2.8 s (login), 2.4 s (mailbox). The first store open took between 3.6
   and 6.8 s (full-frame captures, so no finer). The dashboard process
   averaged 280% CPU over its first 24 s.
3. **Memory**: 1.1 GB resident for the dashboard hub, 0.98 GB for the
   mailbox; 439 MiB of GPU memory for the hub.
4. **Keyboard selection on the Neomil hub is invisible.** Arrow keys move
   the selection (Enter then opens a different module) but not one pixel
   of the window changes -- measured with no tolerance; the only changing
   pixels were the bar's CPU and MEM readouts. The Neomil dashboard table
   has no selected-module drawing (no `Prim::Pick` in its files) and
   `dashboard_cursor: false`. A data gap, not a redraw bug.
5. **Five of six hub tiles go nowhere.** `dashboard_destinations` has only
   module 4 (PRODUCTS) -> Store. Clicking VEHICLES and WEAPONS did
   nothing, as written. The mailbox is not reachable from the Neomil hub.
6. **Hover feedback on hub tiles was not confirmed.** The capture
   includes the pointer, so "the region changed" proves nothing; by eye
   the hovered tile looked the same.
7. **Login: Escape after typing did not return to the rest picture**
   (frame differs from the pre-typing frame). Not yet looked at; it may be
   the caret's blink phase rather than leftover state.

## Not covered

- Other eras (Entropism, Kitsch, Neo-kitsch): Neomil is the live theme.
- IME input. Fractional scale. A second monitor.
- Store keyboard navigation and card hover: that batch ran while the hub
  was showing (the selection had moved, so Enter opened nothing) and its
  results were discarded.
- Mailbox Enter/Escape: the title did not change, and what they are meant
  to do there was not checked against the code.
- Frame pacing during the boot-in animation.
