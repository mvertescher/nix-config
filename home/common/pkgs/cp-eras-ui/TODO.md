- [x] create scripts/download_images.py to download all the neomil related images from:
  - https://www.behance.net/gallery/118663901/Cyberpunk-2077User-Interface-(Part-1)
  - https://www.behance.net/gallery/133185623/Cyberpunk-2077User-Interface-(Part-2)
- [x] define main colors.rs:
  - primary red #FF3B45
  - primary black #DEDE17
  - also need to set opacities properly
- [x] create iced advanced container. "chip type 1"
- [ ] Reproduce dashboard image (`img-07-dashboard.png`) in demo app:

  > **Everything under this heading was built against a trace that was
  > invented, and most of it is wrong.** Corrected 2026-09-01 after
  > actually opening `img-07-dashboard.png`. The photo holds a
  > **six-diamond staggered menu** (half-diagonal 104, centres
  > (334,460) (530,460) (725,460) / (431,593) (628,592) (822,592),
  > labelled VEHICLES / LOCATIONS / FACTIONS above and WEAPONS /
  > PRODUCTS / CORPORATIONS below) and a **chamfered GO HOME info
  > panel** at x 1128..1358, y 313..756. It holds no chart cards at
  > all. `docs/neomil/dashboard-trace.svg` has been rewritten from
  > measured geometry and now passes `fidelity_check.sh --inventory
  > neomil dashboard` at 92% of source shape area; the *implementation*
  > has not been touched and still scores 0%.

  - [x] ~~Implement custom background (gradient/glow)~~ — landed
    2026-08-31 as stacked strips forming a "cold-blue top band" with
    crest blocks. **The premise is wrong**: the source has no band and
    no edge. It is a broad blue glow over near-black, at full strength
    to y~250 and gone by y~420, with a warm near-black vignette down
    the left margin. Measured stops are in `dashboard-trace.svg`'s
    `glowh`/`glowv`. Redo against those.
  - [ ] Implement `InfoPanel` widget (chamfered corners) — **reopened.**
    The old note said "the ops-charts material shows no such panel";
    it does. The GO HOME panel is the right-hand third of the source:
    230x443, chamfered top-left (14) and bottom-left (42), 1px bright
    border over a dark-red translucent fill, heading + two body
    paragraphs, a scrollbar rail on its right edge and a maker's mark
    at its foot. Note the chamfers are top-left/bottom-left, not the
    top-right/bottom-left this item used to claim.
  - [ ] Restore `DiamondMenu` — **reopened.** It was built, then deleted
    2026-08-24 on the reasoning "no neomil sheet draws a diamond, and
    the sheet puts a services table where the dashboard puts its menu".
    The first half is false: `img-07-dashboard.png` is a six-diamond
    menu, and it is the era's dashboard material. The deletion was
    argued from `docs/neomil/target-app.svg` (the *console* screen,
    `img-08-main.png`) while the dashboard source went unread. Geometry
    to build against is in `dashboard-trace.svg`: pure 45° diamonds, no
    chamfer, x pitch 195.5, row pitch 132, row 2 offset +96.5, a 16px
    ground gap on the midpoint of each same-row pair (rows only
    interlock — L1 distance 195.5 against a diameter of 208 — so no
    cross-row separator is needed), an inset outline at half-diagonal
    68 and a glyph at each centre.
  - [ ] Update demo app layout, colors, and text to match image —
    **reopened.** `Layout::OpsCharts` and `screens::dashboard::ops_charts`
    render three `widgets::charts` cards, a right rail and a corner
    block, none of which are in the material:
    `fidelity_check.sh --inventory` scores the golden at **0% of source
    shape area, with 12 diamonds and the rule absent**. The sampled
    `BAND_TOP`/`BAND_BOTTOM`/`CARD_DARK` consts on `eras/neomil.rs` were
    sampled off the same misreading. Decide first whether neomil's
    dashboard should follow its material (diamond menu + info panel) or
    stay a shared cross-era layout — see the "Layout" entry below — and
    only then rebuild. Do not touch `tests/golden/` until that is settled.

## Design pipeline: the traces were invented (2026-09-01)

> **Standing rule (2026-09-01): SVG edits require a vision-capable
> model; SVG → iced conversion does not.** The traces exist so that
> coding models can build the Rust iced screens from a text spec
> (measured coordinates + sampled palette) without ever needing to see
> the source photos. Dispatch accordingly, and see `docs/PIPELINE.md`
> § Division of labour.

Two of the two `dashboard-trace.svg` files that have ever been checked
against their source material turned out to have been written without
anyone opening the image. Both passed the G1 grid gate. Both had their
invented descriptions copied into `docs/sources.md` as observation, and
from there into `src/style.rs`, `src/screens/dashboard.rs` and the
`Layout` enum's justification.

- [x] **`scripts/extract_spec.py`** — measures an image into a shape
  inventory: palette by deterministic k-means, ground/ink split by which
  clusters reach the canvas border, per-cluster connected components
  (hole-filled, so an outlined widget is one shape and not a ring plus a
  core), overlapping convex blobs split by nearest-peak on the distance
  transform, each component fitted against rect / diamond / chamfered-rect
  / rule templates by an occlusion-aware IoU. No RNG; same bytes in, same
  JSON out. `--crops DIR` writes a zoom per shape for inspection.
- [x] **`scripts/spec_diff.py`** + `fidelity_check.sh --inventory` — the
  gate G1 could not be. Matches two inventories shape by shape and fails
  when a whole class is absent or under 60% of source shape area is
  matched. On the old neomil trace it reports, in one line, `diamond 12
  source / 0 candidate — ABSENT`. Needs numpy + scipy; the script's nix
  fallback builds them.
- [x] **`docs/neomil/dashboard-trace.svg` rewritten** from measured
  geometry. PASS at 92% of source shape area, centre error median 1.4px,
  and the six diamonds now extract at half-diagonal 103 against the
  source's 103.
- [x] **`docs/entropism/dashboard-trace.svg` rewritten** (2026-09-01)
  from the real hub material (`entropism-store.png` — the swapped name
  stands; `fidelity_check.sh`'s G1i source table points at the right
  file with a comment). PASS at 92% of source shape area, centre error
  median 1.5px. The file-name swap itself is still unfixed and still
  documented in `docs/sources.md`; `src/style.rs` and
  `src/screens/dashboard.rs` still name the files as they stand.
- [x] **Kitsch and neokitsch runs opened** (2026-09-01): the "no
  dashboard material" claims were **false in both cases**. Kitsch #49
  (`e6ea35…`) is the hub — two 3-blade fans, EVENTS selected yellow,
  USER box, 01–04 badges, BRAINDANCE panel. Neokitsch #69 (`17a5c4…`)
  is the hub — six staircase cascade cards, EMAIL selected gold, detail
  panel, T1–T4 badges. Full-res sources downloaded
  (`images/{kitsch,neokitsch}-dashboard.png`), traces written from
  measured geometry (`docs/{kitsch,neokitsch}/dashboard-trace.svg`),
  both gated. The gate grew an **ink-placement mode** for these two:
  their rotated/translucent geometry (fan blades, onion cascades)
  fragments unstably under the axis-aligned shape templates, so the
  verdict rides per-colour-family occupancy IoU (faithful ~0.5 vs
  0.03–0.07 for the old composites; `spec_diff.py --gate inks`).
- [ ] **Rework `docs/kitsch/dashboard.svg` and
  `docs/neokitsch/dashboard.svg`** — both are "original composites"
  drawn app-first while their real source went unread; both score
  0.03–0.07 against the material on the ink gate. They should follow
  their era's `dashboard-trace.svg`, not the other way around
  (`docs/PIPELINE.md`, direction of change).
- [ ] **Decide whether `Layout` should exist — the material now answers
  the factual half.** With all four sources finally opened (2026-09-01),
  **all four eras are the same module-hub screen in different dress**:
  a menu of six modules with one selected (neomil diamonds, entropism
  tiles, kitsch fan blades, neokitsch cascade cards), a detail panel
  describing the selection, and a security-badge row with the second
  badge filled. That is exactly the `ModuleHub` + per-era `Menu` model
  the crate already has — the material never disagreed about what a
  dashboard is; the traces and unread claims did. So `OpsCharts` and
  `TileRow` model a misreading, and `widgets::charts` has no referent
  in any source. What remains is the design call: fold all four back
  onto `ModuleHub` (the menu variants differ per era, which `Menu`
  already carries) and delete the two layouts, or keep them as
  deliberate original compositions. Settle this before rebuilding any
  dashboard arm, and do not touch `tests/golden/` until it is settled.

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
