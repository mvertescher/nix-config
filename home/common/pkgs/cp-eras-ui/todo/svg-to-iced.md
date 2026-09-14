[All workstreams and current status](../TODO.md). File paths in these
records are relative to the crate root. Dated notes retain their original
reasoning; later completion entries supersede earlier open-item lists.

### SVG→iced pre-work (2026-09-02)

What has to exist before coding agents convert the sixteen traces and
four bars, done in this order so the Rust is written once.

- [x] **iced 0.13 → 0.14.0, iced_layershell 0.13.7 → 0.19.1.** One
  chosen deviation from upstream defaults and it matters: 0.14 turned
  `web-colors` (sRGB blending) on by default, which drains the kitsch
  and neokitsch blooms and thins every glyph — 95.7% on the kitsch
  screens. `Cargo.toml` restates 0.14's default set minus that feature;
  keep the list in step with upstream's `default` on the next bump.
  Everything else is forced and recorded in the commit: `application(boot,
  update, view)`, `Widget::update` replacing `on_event`, zero-sized
  `Space` children now *dropped* by `Row`/`Column` (`is_void`, ~56 call
  sites — caught only by the goldens), `Pixels: From<u16>` gone, canvas
  text `align_x/align_y`, `scrollable::Id` → `widget::Id` +
  mandatory `auto_scroll`, `Subscription::run_with` needing a `Hash`
  handle (the bar's `MenuStream` newtype), and layershell losing
  `remove_id` in favour of `window::Event::Closed`. tiny-skia/png still
  single-copy (0.11.4 / 0.17.16), so resvg 0.46 stays.
  Goldens: 20 of 21 within 99.9 on the 0.14 build; the residue is
  `Rectangle::snap` now rounding (canvas hairlines lose their leading
  antialias row; `surface::visible()` documents the two fixes tried and
  measured worse) plus the glyphon → cryoglyph rasteriser.
  `dashboard/kitsch` is at 99.739 — the `Menu::Fan` blade edges, all
  rotated; geometry and fills byte-identical. Goldens are re-taken
  after the corner refactor below, which resolves it.
  **Unverified until a switch:** `cp-eras-ui-bar` as a layer surface
  (weston headless has no `zwlr_layer_shell_v1`): exclusive zone, the
  tray-menu overlay, `Message::Closed`, the `MenuStream` subscription,
  and the now-reachable middle click (`TrayAction::Secondary`).
  Also new and untried: 0.19.1's `NewPopUp` grabs with the last button
  serial (`multi_window.rs:832`), so the tray menu no longer *has* to be
  an output-sized overlay to get click-outside dismissal — the agent's
  README edit claimed the opposite and was corrected; README §bar has
  the citation.
- [x] **`scripts/render.sh`** — the golden matrix's recipe outside the
  sandbox, ~7s a capture, settle 3s (byte-identical to 15s, also under
  six concurrent renders; 0–2s matched too, so 3 is headroom). Era
  palette published into a scratch HOME via `nix eval` of
  `themes/<era>/scheme.nix`. Runs binaries through a `buildEnv` of
  weston+mesa+the crate's runtime libs because `nix shell nixpkgs#weston`
  sets PATH only and iced dlopens libvulkan/libxkbcommon/wayland.
  Untracked until committed: `git add -N` it first.
- [x] **G2i** — `fidelity_check.sh --implementation <era> [screen]
  [--bin-dir DIR]`: design SVG render vs `render.sh` capture, as shape
  inventories, pass/fail. Shapes gate for every era (the `inks`
  fallback is about photos; both sides here are clean renders — and it
  names the missing cells where inks gives one number). `--match-iou
  0.65`, not spec_diff's 0.30: entropism/bar's menu panel matched at 0.50
  while 140px left and 67px wider than the design, and the two converged
  pairs (entropism/dashboard 100%, neokitsch/dashboard 87% — against
  the since-deleted `dashboard.svg` composites, i.e. the screen against
  a drawing of itself; that is what made them a calibration pair) hold
  to 0.90.
  Move it only with that kind of evidence. `extract_spec.py`'s 80x45 ink
  grid used a `reshape` needing the canvas to divide evenly — 220 does
  not — replaced by index binning, bit-identical at 1600x900. Starting
  line on the 0.13 binaries: bars FAIL at 11/5/35/7% matched area
  (neomil/entropism/kitsch/neokitsch), logins at 0% — the app's login is
  a different composition from every trace, not a gate fault.
- [x] **Per-corner cuts** — `Corners` becomes four `Cut`s
  (`Square | Chamfer { x, y } | Round { radius }`); the era-level
  `Corner` stays and supplies `default_corners`. Decided over a
  `Bar`-only corner field because the neokitsch cell (r3 ×3 + 10x7 BL
  chamfer) is mixed treatments no single field expresses, and neomil
  cuts a different corner per widget on every screen. Pure refactor,
  proved: all 21 captures byte-identical before/after, 7 unit tests in
  `widgets::surface::tests` build the six bar shapes and read them back
  through `span_at`. `Cut::extent` scales x and y by one factor rather
  than clamping each — independent clamping would have widened neomil's
  15px chamfer on a 25x35 cell from 12.5x12.5 to 15x12.5. `Surface::corner`
  is gone; `Surface::corners` is a public field with no builder, so a
  redressed bar cell sets it after construction (add a builder if
  bar.rs ends up doing that at all eight sites). `src/bar.rs:5` still
  says the bar "cannot express a chamfered or clipped corner" — false
  now, rewrite with the redress.
- [x] **Re-take all 21 goldens** on the 0.14 build after the corner
  refactor, per the `tests/bar.nix` procedure (threshold 0, matrix,
  copy, threshold back). This is the baseline the conversion wave diffs
  against; it retires the `dashboard/kitsch` 99.739. The pre-0.14
  goldens are one commit back if the edge-pixel residue ever needs
  re-examining.
- [x] The conversion wave — bar, login, mailbox and store landed
  2026-09-03 (bar on main; the three screens on worktree branches,
  merged by hand). Dashboard followed on 2026-09-03 late (the `Layout` item). What
  landed: every converted screen is **one `canvas::Program` walking an
  era table** — `Style::access` (login), `Style::mailbox`,
  `Style::store: &[Prim]` + `store_selection` — with hit-testing
  `Message::Select` on mailbox and store (both were `Message {}`
  before). `Ink`/`Face`/`Seg` are one type each in `style.rs` (the
  branches each grew their own; consolidated at merge —
  `Ink::of(&palette)`, `Style::ink_in`). Era files carry the values in
  `// --- login ---` / `mailbox` / `store` blocks.

  G2i, final, all from main's script and traces (`/tmp/wave-shots/
  gates.txt` for the run), as entropism/neomil/kitsch/neokitsch: bar
  100/86/83/**52**, login **28**/96/72/89, mailbox 100/95/67/86, store
  100/84/88/82. **Re-run 2026-09-03 evening against the follow-up traces
  (`45c31b4`): identical except kitsch login 97 and neokitsch login 67**
  — attributed at the time to trace edits; wrong. Neither trace nor
  login code changed, and 2026-09-04 with `render.sh` settling 8s the
  two read 72/89 again: the 3s settle raced the login wash, and the
  evening run happened to capture before it painted (see the ticked
  "gate artefacts" item in the [pipeline record](design-pipeline.md)). 72/89 are the numbers; both PASS.
  Neomil login held 96 with the trace chamfer now 51 (the era table
  drew 46 until 2026-09-04, when the residue item in the [pipeline record](design-pipeline.md) moved it to 51
  alongside the card-1 block and chip fixes). Two accepted FAILs, do not chase (one since
  resolved):
  - **entropism login 28%** — **resolved 2026-09-04, 100% PASS.** The
    explanation that stood here (linear-space AA splitting hairline
    pixels across two bins; crate-wide fix) was wrong: the capture had
    no wash at all (3s settle, see the "gate artefacts" item in the [pipeline record](design-pipeline.md)) and, once
    it did, the extractor split rsvg's radial into two ground bins.
    Neither needed `web-colors`.
  - **neokitsch bar 52%** — extractor fragmentation, see the bar item.
  - kitsch mailbox 67% (unselected chevrons are two cells in the
    design, one in the render) is a PASS with a known reason; kitsch
    login's `Wash::RoseBloom` was kept over Plain's 98% because layout
    IoU is 0.963 vs 0.568 (it now scores 97 anyway, see above).
  - Dashboard, not in the wave, gated after the merge: entropism and
    neokitsch PASS; neomil 0% (the historical correction in the [Neomil record](neomil-dashboard.md)) and
    kitsch 19% — the kitsch number is identical against the
    `a0a9274` trace, so pre-existing, not a regression. **Superseded
    2026-09-03:** those four numbers were against the app-shaped
    `dashboard.svg` composites, since deleted; against the traces all
    four dashboards scored 0%; the fold the same night took them to 98/96/31/92 (the `Layout` item).

  Findings that outlive the wave, each verified by the orchestrator:
  - **Gate change:** `fidelity_check.sh --implementation` hides
    `class="photo"` elements (halos, glows) from the design render
    before comparing; `docs/PIPELINE.md` has the paragraph. XML
    comments must not contain `--`. Follow-on: `--source` mode only
    ran `dashboard.svg` (login/mailbox/store SKIP) — script gap, closed
    2026-09-03 when the script moved to one design per screen.
  - **iced_wgpu canvas buckets meshes < images < text per canvas**
    regardless of draw order — a covering strip cannot hide a caption
    on the same canvas; use layers. `Frame::draft`/clip keeps the
    region only as a scissor with an identity transform, so a clipped
    sub-frame cannot be re-based (`Prim::Clip` was built on it and
    retired). Strokes are ~15% heavier than rsvg (coverage .87 vs .75
    per px), which flips k-means bins — not fudged. No radial
    gradient: hazes are concentric ellipse annuli or a 1:1 RGBA image.
  - **`canvas::Text` has no letter-spacing, x-scale or rotation**, so
    fitted tracking and glyph x-scales from the traces are dropped
    (login has `Legend::stretch`/`tracking` as a prefix-measured
    workaround), and neomil's rotated maker's marks / margin strings
    are not drawn. (Since then: `rotate` on `Prim::Text`, and
    2026-09-04 `Prim::Tracked` for letter-spacing; x-scale is still
    login's `Wide` only.)
  - **Widget gaps the agents reported** (each "use widgets, don't edit
    them" collided with): absolute placement; `Cut` has no Step cut;
    `Surface` has no tab, ticks or dense grain; no custom panel
    silhouette; no blur; `fonts.rs` had no semibold (`Face::SemiBold`
    mapped to Medium — fixed 2026-09-04, `FONT_RAJDHANI_SEMIBOLD`);
    `Ground` caps at ~6% alpha (neokitsch haze wants
    #4f4262). This is the input to the canvas-vs-widgets decision
    below.
  - [x] **Haze unification — landed 2026-09-05.** The item said three
    implementations; the count was five, and the mailbox one had
    already gone: login `wash_image` (closures per `Wash` variant,
    sampled into an RGBA image), `scene.rs`'s `Prim::Lobe` arm (96
    even-odd annuli, the last user was the entropism dashboard's
    lift), `bar.rs` `BarGround::Haze` (64 discs, neokitsch strip),
    `bar.rs` `BarGround::Band` (128 strips, neomil) and
    `widgets::ground` `Ground::Bloom` (26 discs). Everything radial now
    goes through `screens/soft.rs` as a `Prim::Soft` group drawn by
    `scene::Backdrop` — a canvas of its own under the art, because a
    canvas layer draws its meshes before its images:
    - Login: `Access.wash: Wash` → `Access.backdrop: &[Prim]`, each
      era's login ground transcribed from its trace with the existing
      constructs (`Lobe`, `Ramp`, `Turn`, `Masked`). `Backdrop.stretch`
      added because the login and the mailbox map the frame axis by
      axis where a scene letterboxes. The closures were approximations
      — neomil's vignette alpha was squared where the trace's is two
      linear stops, neokitsch's haze and blue lacked the trace's 1.3°
      and 2° turns — and the renders now sit within a level of the
      rsvg-rendered traces where the goldens were up to 7 off. G2i
      logins 100/96/72/89, unchanged.
    - `scene.rs` `Prim::Lobe` outside a `Soft` group draws nothing
      now (`soft_only_prims_stay_soft` panics on one); the entropism
      dashboard opens with `Soft { HUB_GROUND }`, the kitsch store's
      `At { BACKDROP }` became `Soft { BACKDROP }`.
    - Bar: `BarGround::Haze { prims }` is the era's dashboard ground
      (`HUB_GROUND`, which `bar.svg` copies its `#haze`/`#hazeblue`
      from) composited at the strip's own pixels by a `Haze` canvas
      under `Strip`, k = 1. That brought the blue annulus with it (the
      item under "Bar restyle"): x 1599 lands on the trace's #202d47
      exactly, x 1520 #272b42 against the trace's #292e41.
    - **Kept, on purpose:** neomil's `Band` — a 2-stop horizontal ramp
      across 1600px steps a level at most every 12px, nothing to gain
      and the neomil bar golden would move for it; `Ground::Bloom` —
      every screen ground but the entropism mailbox's is a composited
      group opening with a full-frame fill, so at 1600x900 its discs
      show in no golden; they paint the letterbox margins of a
      non-16:9 window and the ground of `panels::mail`, the working
      client (`cp-eras-ui-mail`, no golden). Flattening it to `bg` is
      a follow-up that moves no golden and changes that client;
      `ERAS-DELTA.md` point 2 has the reader inventory.
    - Goldens re-taken: login-{neomil,neokitsch}, dashboard-entropism,
      store-kitsch, bar-neokitsch.
  - **Extractor cluster budget:** with k=8, a haze takes 5 clusters,
    so line art drawn dimmer than measured merges ink families (the
    neokitsch mailbox wire went to `Ink::Tape`, RIFLES to `Ink::Fg`,
    per a re-cut trace). Backgrounds are not optional for the
    extractor (kitsch bloom = 5/8 clusters). Noted in `PIPELINE.md`.
  - ~~`widgets::row::mail_row` / `row::Mail` have no caller now.~~
    Deleted 2026-09-05 with the rest of the callerless set (below).
  - **Live inconsistency, kitsch — settled 2026-09-05:** `src/eras/kitsch.rs`
    had `border: TEAL` (#7ddec8) where `home/themes/kitsch/palettes.nix`
    said #2e5f57 and the store and mailbox traces sample #5fd6c2. User
    chose the sampled value: nix `reference.border` is #5fd6c2, the
    crate's `border` is the new `TEAL_OUTLINE` (same value), and the
    store's `Ink::Fixed(OUTLINE)` became `Ink::Border` since the role
    now says what the const said. Moves the kitsch bar's chip, window
    and menu outlines from the dim teal to the outline teal (bar.svg
    draws them in #7ddec8, a stop brighter still). `bleach`/`ash`
    untouched.
  - **Live inconsistency, entropism — settled 2026-09-05:**
    `home/themes/entropism/palettes.nix` nexus published `tape =
    "#9cb795"`, the selection sage, where `src/eras/entropism.rs` and
    `bar.svg` have MID `#728f76` so the host tape does not read as a
    second selected cell beside workspace 3. User chose the crate side;
    nexus `tape` is #728f76 now. `tape` also feeds base16 `base0A` and
    the starship/tmux/waybar host labels in `lib/era.nix`, so those dim
    with it on entropism hosts — intended, it is the same label.
  - `scripts/render.sh` keyed its theme cache on `scheme.nix` +
    `roles.nix` and not the `palettes.nix` the scheme imports, so a
    retinted role rendered stale. Fixed 2026-09-05 (key includes
    `themes/<era>/palettes.nix`).
  - Bar/entropism inks and the login `Ink::Fixed`s waited on the
    OUTLINE decision under "Trace improvements" — taken 2026-09-05
    (`border` → #8fba97), and the login, mailbox and hub inks were
    folded onto the roles the same day — recorded there, with the
    `select` → #a6d3a7 decision that came out of it.
- [x] **Canvas vs widgets — decided 2026-09-05: retire.** ~~Four screens are now display
  lists over era tables and the widget layer (`widgets/`, `Layout`,
  `Cut`, `Surface`, `Ground`) serves only the dashboard and the bar's
  window. Either the widget layer grows the gaps above and the screens
  fold back onto it, or it is retired to what the bar needs and the
  dashboard converts the same way. Decide before the dashboard, and
  together with the `Layout` fold-back — same decision.~~ Overtaken on
  one side: the dashboard converted to a scene and `Layout` folded on
  2026-09-03, so "fold the screens back onto widgets" is no longer a
  live option — all four screens are `Prim` tables. What is left to
  decide is how much of `widgets/` to keep for the bar and the panels.
  Measured 2026-09-04 (callers outside `widgets/` itself, doc comments
  excluded): **live** — `ground` (every screen), `surface::{outline,
  layered, span_at, backdrop, surface, Surface, Corners, Cut, Fill}`
  (`bar.rs`, `screens::mail`, `panels::mail`, `style.rs` for
  `Corners`), `chrome::{top_bar, footer}` and `text` (`panels::mail`),
  `floppy_icon` (its example); **no caller** — `banner`, `bracket`,
  `card`, `glyph`, `input`, `ornament`, `row`, `silhouette`. The
  2026-09-04 count also listed `floppy_vector` as "named only in a
  neomil comment"; wrong — `floppy_icon.rs` calls it as
  `super::floppy_vector::draw_*`, which a `widgets::floppy_vector`
  grep does not see. Eight, not nine. Deleted 2026-09-05 with their
  `mod.rs` re-exports; `widgets/` is now `surface`, `ground`, `chrome`,
  `text`, `floppy_icon`, `floppy_vector`. Build, tests and matrix
  unchanged (nothing drew them). The [toolkit build-out list](toolkit.md) is
  therefore rebuild-from-traces work when a caller appears, not a
  revival of these files; git has them at `b1e6cb1` if a shape is
  wanted back.
