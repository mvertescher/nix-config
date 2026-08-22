- [x] create scripts/download_images.py to download all the neomil related images from:
  - https://www.behance.net/gallery/118663901/Cyberpunk-2077User-Interface-(Part-1)
  - https://www.behance.net/gallery/133185623/Cyberpunk-2077User-Interface-(Part-2)
- [x] define main colors.rs:
  - primary red #FF3B45
  - primary black #DEDE17
  - also need to set opacities properly
- [x] create iced advanced container. "chip type 1"
- [ ] Reproduce dashboard image (`img-07-dashboard.png`) in demo app:
  - [ ] Implement custom background (gradient/glow)
  - [ ] Implement `InfoPanel` widget (chamfered top-right/bottom-left)
  - [ ] Implement `DiamondMenu` widget (interlocking interactive diamonds)
  - [ ] Update demo app layout, colors, and text to match image

## Toolkit infrastructure

- [ ] **Visual regression as a nix checkPhase**: headless-compositor
  screenshot + pixel diff against reference images is genuinely mature
  practice for a UI toolkit — most hobby toolkits have nothing. But
  it's currently desktop-coupled shell scripts (hyprctl resolution
  detection), and the grim capture doesn't work against current Weston
  (evaluation on 2026-08-21 had to use weston-screenshooter; the
  script's --debug flag suggests that was the original path too, so
  the grim step may have always been broken). The high-leverage move:
  fold it into the derivation as a nix checkPhase — weston headless +
  pixman renderer + weston-screenshooter + visual_diff.py runs fine on
  a headless box (proved on the server: needs a software Vulkan ICD
  for the app, e.g. VK_ICD_FILENAMES=<mesa>/share/vulkan/icd.d/
  lvp_icd.x86_64.json, since iced panics rather than falling back
  when wgpu finds no adapter) — which would make every build a visual
  regression test, CI-able the day this becomes its own repo.

## Palette correction (2026-08-21)

- [x] The original task's "primary black #DEDE17" was a double typo —
  pixel analysis of the reference images found ZERO yellow anywhere
  (0 px within 25% fuzz across img-06/07/08); #DEDE17 is almost
  certainly a mangled #DE2E2E, the fill red sampled from the reference
  diamonds. colors.rs now carries the sampled three-red system
  (bright #FF3B45 / fill #DE2E2E / deep #5E1112) + sparing #DEDEDE
  off-white; COLOR_PRIMARY_BLACK and COLOR_YELLOW are gone. If a
  warning accent is ever wanted, it is a deliberate extension, not
  reference canon.

## Full toolkit build-out (design targets in docs/, added 2026-08-22)

Implement the widget set mocked in `docs/target-components.svg`;
`docs/target-app.svg` ("NEOMIL OPS") is the acceptance test — done
when that screen assembles from library widgets. Priority order:

- [ ] **Theme/Catalog first**: replace loose color consts at call
  sites with a semantic iced Theme + widget catalogs (surface/
  primary/dim/danger...) so every later widget styles against tokens.
  Everything below is written twice if this comes second.
- [ ] **Migrate to iced 0.14** (0.13 pinned; 0.14 is stable now) —
  before the widget build-out, not after.
- [ ] **Form controls** (style iced built-ins, don't hand-canvas):
  button (primary/ghost/override-hatch/disabled/icon), text_input
  with focus treatment, checkbox/toggle/radio, pick_list + menu,
  slider with ticks.
- [ ] **App shell**: `neomil_ui::app(...)` bootstrap (fonts, theme,
  transparent window, background layer) replacing the per-example
  ritual.
- [ ] **Data display**: styled scrollable/scrollbar, table/list rows
  with selection, key-value spec rows, log view with severity colors.
- [ ] **Feedback**: segmented meter, progress bar (+indeterminate
  scan), toast/banner (warn = dim red, error = bright red), modal
  with scrim, tooltip, status bar.
- [ ] **Chrome**: tab bar (generalize the T-chips), context menu,
  parameterized top_bar (move the demo copy into examples/).
- [ ] **Icon set**: 16px-grid canvas path icons behind one
  `icon(Icon::..., color, size)` entry point; retire the pixel-blob
  placeholders.
- [ ] **Motion**: hover flicker + panel boot-in as canned animations
  (the diamond_menu Cache-invalidation pattern is the plumbing).
