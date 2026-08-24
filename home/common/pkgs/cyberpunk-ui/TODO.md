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
  - [x] ~~Implement `DiamondMenu` widget~~ — built, then deleted 2026-08-24
    when `widgets::table` landed: no neomil sheet draws a diamond, and
    the sheet puts a services table where the dashboard puts its menu.
  - [ ] Update demo app layout, colors, and text to match image

## Toolkit infrastructure

- [x] **Visual regression, landed 2026-08-22** as `tests.visual`
  (`nix build -f . tests.visual`): weston headless + pixman inside the
  build sandbox, weston-screenshooter capture, diffed against a
  committed golden by scripts/check_similarity.py. Two independent runs
  are byte-identical, so the threshold is strict. Original note kept
  below for the reasoning.

- [x] ~~Visual regression as a nix checkPhase~~: headless-compositor
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
  (the Cache-invalidation pattern the deleted diamond_menu used is the
  plumbing; see git history).

## Headless check: feasibility settled (2026-08-22)

Both unknowns blocking "visual regression as a nix checkPhase" were
probed inside an actual nix build sandbox, not just on a headless box.

- [x] **Weston runs in the build sandbox.** `--backend=headless
  --renderer=pixman --shell=kiosk --no-config` starts and enables its
  output, with `XDG_RUNTIME_DIR` pointed at a mode-700 dir under
  `$TMPDIR` and `HOME` set. No seat, no /dev/dri needed.
- [x] **The app renders, but the recorded recipe was incomplete.** A
  software Vulkan ICD alone is not sufficient. Without forcing the
  backend, wgpu picks GLES and panics in
  `wgpu-hal-0.19.5/src/gles/egl.rs:789` — `unwrap()` on `None`, i.e. no
  EGL display in the sandbox. The fix is **`WGPU_BACKEND=vulkan`**
  alongside
  `VK_ICD_FILENAMES=<mesa>/share/vulkan/icd.d/lvp_icd.x86_64.json`
  (mesa 26.1.2 ships it at that path). With both set the app runs
  clean.

Remaining for the checkPhase itself:

- [x] Capture with `weston-screenshooter` and compare (done). Note the
  comparison target has to change: `images/` is **gitignored**, so the
  downloaded Behance references are not in the repo and cannot be in a
  hermetic build. Diff against the tracked `docs/target-*.svg`
  rasterised instead — which is also the cleaner answer, since those
  are our own design targets rather than someone else's copyrighted
  artwork.
- [x] Wired as `passthru.tests` rather than a gating `checkPhase`: a GPU-less compositor is exactly the kind of thing that fails
  for environmental reasons, and it should not block every build of the
  toolkit until it has proven stable.

## Black window on the nvidia/Hyprland session (2026-08-22)

- [x] **Fixed 2026-08-22: wgpu was choosing a non-presenting adapter.**
  This machine exposes three Vulkan adapters (discrete nvidia, the
  Ryzen's integrated RADV, llvmpipe) and wgpu picked one that cannot
  present, so the app really was rendering - somewhere the compositor
  never shows. `WGPU_POWER_PREF=high` alone fixes it (1 unique colour
  in the window without, 815 with); set via --set-default in the
  wrapper of both Iced crates. Original symptom description below.

- [x] ~~The app renders headless but not on the real desktop.~~ Under
  weston headless with the llvmpipe Vulkan ICD it draws correctly (that
  is what tests.visual captures). Launched on a live Hyprland session
  with the nvidia driver it starts, stays alive, logs *nothing* to
  stderr, and presents a solid black window. Forcing
  `WGPU_BACKEND=vulkan` does not change it, and `nvidia_icd.json` is
  present in /run/opengl-driver/share/vulkan/icd.d, so the ICD is
  findable.

  Worth noting the visual test cannot catch this: it exercises the
  software path only. A green build does not mean the app works on the
  machine you use.

  Things not yet tried: `WGPU_BACKEND=gl`, running under
  `nixGL`/`nixglhost`, checking whether the wrapper's LD_LIBRARY_PATH
  shadows the driver's libvulkan, and whether the compositor reporting
  `explicit sync: no` matters for wgpu presentation on nvidia.

## Goldens and git history (watch item, 2026-08-24)

- Every verification round that re-renders the 21-case matrix writes
  ~1.5 MB of PNGs into permanent git history. Fine today (repo `.git` is
  ~6.6 MB, and the byte-identical policy means a golden only changes
  when pixels genuinely do), but the escape hatches — git-lfs, or
  splitting this crate into its own repo — both fight the nix fetchers
  and the wrapper's pin, so the cheap moment to act is early. Notice
  this at 100 MB, not at 1 GB. The wrapper's TODO carries the design
  context under *Design review (2026-08-24)*.
