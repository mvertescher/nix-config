[All workstreams and current status](../TODO.md). File paths in these
records are relative to the crate root. Dated notes retain their original
reasoning; later completion entries supersede earlier open-item lists.

## Toolkit infrastructure

- [x] **Visual regression, landed 2026-08-22** as `tests.visual`
  (`scripts/run_test_matrix.sh visual`; the `nix build -f .
  tests.visual` it landed with never worked once `default.nix` took
  `callPackage` arguments): weston headless + pixman inside the
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

> Both neomil sheets were deleted 2026-09-03 (see Housekeeping); the
> acceptance test no longer exists as a file. The items below stand on
> their own as widget work, but "NEOMIL OPS" is not a screen the
> material has — the neomil traces are the targets now, and the widget
> set to implement is the one on `docs/<era>/components.svg` (rebuilt
> from the traces the same day, see "Component sheets"), not the old
> by-eye mock. The callerless widget files (`banner`, `card`, `input`,
> `bracket`, `glyph`, `ornament`, `row`, `silhouette`) were deleted
> 2026-09-05 under "Canvas vs widgets"; none of the items below start
> from them.

- [x] **Theme/Catalog first** — done 2026-09-05. `Style` *is* the
  iced theme: `catalog.rs` gives it `theme::Base` (ground = the
  palette's `bg`/`fg`, `palette()` maps cta/select/tape/alert onto
  primary/success/warning/danger) and `Catalog` impls for
  `container`, `text`, `scrollable`, `button`, `text_input`,
  `checkbox`, `toggler`, `radio`, `slider`, `pick_list` + its menu,
  `rule`, `progress_bar`, all `Class = StyleFn` so `.style(closure)`
  still works. `crate::Element<'a, M>` is the alias to use; the old
  `iced::Element<'_, M>` (with `iced::Theme`) no longer type-checks
  against anything in the crate, and canvas programs are
  `Program<M, Style>`. Every example sets `.theme(|app| app.style)`
  (the layershell bar: `|app, _window|`). The per-site closures
  went: `panels::mail`'s `rail()` is `catalog::faded_rail(alpha)`,
  `bar-window`'s bg/fg closure is the theme base, `chrome::footer`'s
  1px border container is `rule::horizontal(1)` under
  `catalog::divider` (pixel-identical on the mail goldens), and the
  bar daemon's bg fill is `catalog::ground`. The two `.style` closures
  left are deliberate: the daemon's transparent application surface
  and the floppy bench's one-era dressing. Goldens did not move
  (21/21 after the switch).
  - Original text of the item, kept because its premise was half
    wrong: "replace loose color consts at call sites with a semantic
    iced Theme + widget catalogs ... so every later widget styles
    against tokens." Re-read 2026-09-05 before starting: the token
    layer already existed -- every canvas call site styles through
    `Ink` roles resolved by `Palette` (`bar.rs` `ink_of`, `scene.rs`
    `Scene::ink`, `login.rs`), the era tables name roles rather than
    values wherever the trace has a role, and the nix theme overlays
    the roles (`Palette::with_roles`). There were no loose colour
    consts at call sites left to replace (the `rgb(0x..)` outside
    `src/eras/` are `soft.rs`/`scene.rs` tests and the floppy icon).
    What was missing was only the iced side, and that is what landed.
- [x] **Migrate to iced 0.14** — done 2026-09-02, before the build-out;
  record under [SVG→iced pre-work](svg-to-iced.md) (the `web-colors` opt-out is
  the part to know about).
- [ ] **Form controls** (style iced built-ins, don't hand-canvas):
  button (primary/ghost/override-hatch/disabled/icon), text_input
  with focus treatment, checkbox/toggle/radio, pick_list + menu,
  slider with ticks.
  - Coats landed 2026-09-05: `Style::controls` (`Controls` of
    `Coat`s -- `primary`, `ghost`, `disabled`, `field`, plus
    `placeholder` and the era `radius`) read off each
    `components.svg` as text: entropism's stroke-2 button strip and
    stroke-1.25 field, kitsch's ENTER bar / PROTECTED / well, neokitsch's
    outlined r5 button and unlined `#3c1c11` field, neomil's
    filled/outlined pair and `#430e0f` field. `catalog::button::
    {primary, ghost, bare}` and `catalog::field` apply them (default
    button class is `ghost`; `bare` is for a `widgets::surface` face
    -- `panels::mail`'s DELETE is now a real `button` that way).
    Tests pin the cta fill per era and kitsch's PROTECTED triple.
  - **Remaining limits.** *Silhouettes*: the simple catalog sets only
    fill/edge/ink/radius. Kitsch's stepped bar and Neokitsch's tabbed
    bl-chamfer now use the native-control wrappers documented in
    `docs/control-materials.md`; working mail faces use `Surface`.
    Neomil's generic native controls retain the simple catalog coat.
    *Hover/press*: the reading is on the sheets since 2026-09-07
    (see "Motion"). Neomil and entropism built-in button/field coats
    are wired as of 2026-09-13. Kitsch/Neokitsch custom native-control
    wrappers and working mail faces are wired as of 2026-09-14 (below);
    Kitsch hub hover still needs a specific interpretation.
    *override-hatch*: no era sheet has a
    hatched button; iced has no pattern fill either. *Icon buttons*:
    nothing to style beyond `bare`; blocked on "Icon set". *Slider
    ticks*: `slider::Style` has no ticks; would be a widget, not a
    style. *checkbox/toggle/radio/pick_list/menu/slider*: styled and
    documented as **derived** from the coats (box = field coat, mark
    = cta fill; menu = panel/border/select pair; rail = the scroll
    rail's inks laid flat) -- no trace has any of them, so the
    derivations are the best available and are not verified against
    material. The item stays open for whichever of those a trace
    later shows.
- [x] **App shell** — done 2026-09-05 as `shell.rs`, not the
  `neomil_ui::app(...)` it was filed as: `shell::style()` (the `--era`
  flag, else the desktop; `era_from` is testable over any argument
  list), `shell::faces()` / `settings()` (every face the crate ships,
  Rajdhani regular, antialiasing), `shell::FRAME` (1600x900), and
  `shell::application(boot, update, view)`, which is
  `iced::application` with the theme read off a `Wears` state and the
  settings and frame applied. The four screens and the two example
  states implement `Wears`. Five examples went from ~40 lines of boot
  to four; `examples/bar/style.rs` is gone, both bar binaries resolve
  through `shell::style()` (the layershell daemon keeps its own
  `Settings` and takes `shell::faces()`). Every binary now loads all
  ten faces where they had loaded five to nine -- `fonts.rs` says a
  weight a binary never loaded is shaped in the wrong face. The matrix
  did not move (21/21), so no screen had been naming a face its binary
  lacked; the hazard is gone rather than a fault fixed.
  - Not done: the "transparent window, background layer" half. It was
    the old neomil mock's idea; every traced screen paints its own
    ground edge to edge and the theme base fills the rest in the
    palette's `bg`, so there is nothing for a transparent window to
    show. The floppy bench keeps its own boot (Orbitron default,
    neomil pinned, `tape` for text); it is a one-era ornament and not
    a screen.
- [ ] **Data display**: styled scrollable/scrollbar, table/list rows
  with selection, key-value spec rows, log view with severity colors.
  - The scrollable half is done under "Theme/Catalog first":
    `catalog::rail` is the default `scrollable` class (neomil's 6px
    `#3a0f12`/`#a8282b` reading, tested; the other eras derive from
    their border/dim inks since no other trace shows a rail) and
    `catalog::faded_rail(alpha)` is the unfocused-pane variant.
  - `tests.mail.<era>` added 2026-09-05: the working mail client now
    has four goldens of its own, so the list rows with selection, the
    focus fade, the rail and the `catalog::button::bare` DELETE are
    held still. The matrix is 25 cases. Seen in the takes and left as
    is: entropism's DELETE plate (`alert` fill under `on_select` ink)
    is low-contrast (a `panels::mail` choice, not a catalog one; the
    trace-backed reference for the screen is the display-only
    `mailbox` golden). The kitsch and neokitsch top bars carrying no
    MAIL BOX segment is `widgets::chrome::top_bar`'s Caption and
    DeviceFrame arms doing what their table says, not a gap.
  - The rest, re-read 2026-09-05: *list rows with selection* is
    `panels::mail::message_row` (Surface-dressed, four eras, now
    golden-held); *table* is the same panel's pipe-table renderer
    (`panels::mail::table`, in the `mail` goldens too). *Key-value
    spec rows* have material -- the store card specs (neomil RANGE /
    RECOIL / REFLEXES, neokitsch STORE META LINES) -- as `Prim`s in
    each era's store block, and no widget caller. *Log view with
    severity colours* has no material on any sheet. Neither gets
    built ahead of a caller (`widgets/mod.rs`).
- [ ] **Feedback**: segmented meter, progress bar (+indeterminate
  scan), toast/banner (warn = dim red, error = bright red), modal
  with scrim, tooltip, status bar.
  - Re-read 2026-09-05 against `widgets/mod.rs`'s rule (no widget
    without a caller) and the sheets. *Status bar* is `bar.rs`, four
    eras, golden-held; done. *Progress bar* has a derived
    `catalog::progress` style and no caller. *Segmented meter*: the
    material is the SECURITY LEVEL badge row every store trace draws
    (neomil `eras/neomil.rs` ~L103, "four security badges of which
    the second is lit") -- a `Prim` list per era today, and it would
    be the widget's source when a widget-built screen needs one. The
    "warn = dim red, error = bright red" toast rule is the deleted
    neomil mock's; no sheet draws a toast, a modal, a scrim or a
    tooltip, and kitsch's URGENT INFORMATION block is the closest
    thing to a banner. None of these gets built ahead of a caller.
- [ ] **Chrome**: tab bar (generalize the T-chips), context menu,
  parameterized top_bar (move the demo copy into examples/).
  - Re-read 2026-09-05. *Context menu* is `bar::tray_menu`, per-era
    tables, golden-held in `bar-window`; done. *top_bar* already takes
    its three segments and `footer` its three strings; the "demo copy"
    is the mailbox chrome strings hardcoded in `panels::mail::
    mail_panel`, which are the mailbox trace's own and are what the
    `mail` goldens hold -- moving them to the example would be a
    parameter nobody sets differently. *Tab bar*: the store's category
    row is the material (neomil "TAB ROW END CHIPS" / "LETTER CHIP" on
    the sheet, the other eras' category strips), drawn as `Prim`s in
    each era's `// --- store ---` block; a widget waits for a
    widget-built screen with tabs.
- [ ] **Icon set**: 16px-grid canvas path icons behind one
  `icon(Icon::..., color, size)` entry point; retire the pixel-blob
  placeholders.
  - Re-read 2026-09-05: there are no pixel-blob placeholders left to
    retire -- they went with the neomil widget set. The bar's tray
    icons are the SNI host's pixmaps (real, and `bar-window`
    synthesises one for the golden). The icon material is on the
    sheets (entropism `env`/`rifle`/`qr`, kitsch `k-env`/`k-gun`,
    neomil `icons`/`gunbody`, neokitsch `riflebox`/`basketclip`) and
    is drawn as per-era `Prim` paths inside the screens that use them.
    An `icon(..)` entry point is worth building the day two screens
    want the same glyph; today none do.
- [x] **The screens as one app (2026-09-06)**: `screens::hub` runs the
  dashboard with the mailbox and the store behind its modules, and
  `cp-eras-ui-dashboard` runs the hub. `h j k l` (and the arrows) walk
  every trace-driven screen *spatially* -- `screens::nav::step` picks
  the nearest plate ahead by centre, along + 2 x across, because none
  of the menus is a grid (neomil's stagger, kitsch's blades, the
  neokitsch cascade, the store's nav-beside-shelf) -- Enter or a
  click on a module opens its screen, Esc comes back; the mailbox
  walks rows with `j`/`k`. Which module leads where is era data,
  `Style::dashboard_destinations`, from the module labels: only
  entropism (EMAILS) and neokitsch (EMAIL) label a mailbox and only
  kitsch and neomil (PRODUCTS) a store, so the other screen in each
  era was at first **stood in** by the last module (DEVICES,
  LOCATIONS, CORPORATIONS). The lists themselves are not open: they
  are the photos' labels (`docs/sources.md`), a personal hub in
  entropism/neokitsch and a catalogue hub in kitsch/neomil, and no
  source shows both screens. **Decided at the desk 2026-09-07: keep
  the photo labels, route by key.** `m` opens the mailbox and `s`
  the store from the dashboard whatever is selected (`Hub::hotkey`,
  hub-local rather than a `nav::Stroke` because every screen matches
  `Stroke` exhaustively and inside a screen the keys mean nothing);
  the four stand-in entries are `None`, and a module with `None`
  behind it selects and stays. Enter, a click and a key all go
  through `Hub::go`, so a screen boots in the same way however it was
  reached. Content is the tables' own mock inbox and
  shelf, nothing live. The store's keyboard has a `focus` the mouse
  also sets, and a move selects what it lands on, so there is no
  separate cursor to draw -- which is also why nothing here needed a
  new golden: the opening frame is the dashboard's own. Only
  exercised headless and by unit test; the keys on a real window are
  unproven until `cp-eras-ui-dashboard --era neokitsch` is run on ws
  10.
- [ ] **Motion**: hover flicker + panel boot-in as canned animations
  (the Cache-invalidation pattern the deleted diamond_menu used is the
  plumbing; see git history).
  - [x] **Phase 1 (2026-09-06): the traces are no longer stills.**
    Motion is written on the trace as SMIL (`docs/PIPELINE.md`
    "Motion"): `<animate id=..>` on the element it moves, the element
    drawn at its *rest* value so rsvg and the goldens never see it.
    `scripts/frame.sh --at T` seeks a trace in headless Firefox;
    `src/motion.rs` is the crate's clock (`--at-ms` / `CP_ERAS_UI_AT_MS`
    freezes it; `render.sh --at`, `triptych.sh --at`; `tests/visual.nix`
    pins `motion::REST`, 2.4 s, as do the scripts with no `--at`). One animation end to end: the login caret blink
    (`#caret-blink`, 1.2s discrete, neomil's `__` tail and the
    kitsch/entropism caret plate; neokitsch shows none).
  - [x] **Hover and press, the reading (2026-09-07).** The traces
    show no such state, and that is also why `catalog` gives buttons
    and fields no hover/press treatment (see "Form controls"). The
    reading was taken from the run footage (`images/run-<era>/` and
    the full-res stills) by one vision agent per era and annotated
    on every `components.svg` as rest / hover / press sibling groups
    for the button, the field, a list row and a nav cell, each
    flagged SOURCED (still, position, pixel box) or INFERRED (from the
    era's own emphasis rungs), with a band comment listing what was
    looked at; each README has a "Hover and press" section. What the
    run settled: **entropism's sage fill is a cursor, not a
    selection** -- the mail photo fills Jackie's row, the panel
    carries Mom's message and row 2 has the open envelope -- so
    hover is sourced there as the cursor fill and press inferred as
    the fill dropping while held. Kitsch shows no pointer, lit
    sibling or held cell in any still; hover = lift (one ghost step
    at +20,-20 from #45's fan-up and #49's extrusion), press = the
    selection fill landing flat, inferred. Neomil: only the field's
    focus is sourced; hover a wash, press the spine, inferred from
    the flat/outlined pair. Neokitsch: no pointer, held cell or
    second ink on any sibling in nine stills and four masters (the
    four RIFLES buttons measure the same to a tenth of a level), and
    it does *not* have entropism's cursor tell -- mail row 2's veneer
    is the message the panel shows, and the open envelopes sit on
    rows 1, 3, 7. The reading is off the era's one ladder (plain
    type; outline with tab; the outline echoed as fading hairlines,
    T2 the only cell marked by echo alone; the veneer): hover = ECHO
    on T2's recipe (seven rings `#a97c48` 0.7, 0.85 to 0.55, fanning
    up and right), press = VENEER (the cell's sourced selected look
    spliced verbatim), the login plate takes a caret instead. Every
    hover inferred, every rest and press sourced. The one sourced
    interactive state in every era is the field's focus (the caret;
    neokitsch photographs its field at rest only, so even that is
    inferred there). **Partially wired 2026-09-13:** `ControlStates`
    carries optional hover/pressed coats per button class and field.
    `catalog` now applies neomil's wash/held-red pair and entropism's
    reverse-video/outline pair. Field focus wins over hover and iced
    owns the caret; disabled controls keep their disabled coat, and
    `bare` never paints over its custom face. Entropism's field colours
    are derived through the existing desktop roles, rather than the
    login's fixed sampled shades. Rest coats are unchanged.
    `cargo run --example control-states -- --era neomil` shows all
    four eras' state coats and live controls. As of 2026-09-14,
    Kitsch/Neokitsch use custom native-control backdrops for ghosts,
    echoes and veneer; their material is not a color-only catalog
    approximation. Working mail faces are also wired (below). Kitsch
    hub hover remains undefined; its pressed destination is implemented.
    These are instantaneous states; animated transitions remain open.
    Desktop interaction verification remains open.
    - **Hub cursor, 2026-09-14:** the shared scene's
      dashboard/store clicks now commit on release over the original
      plate. Dragging to another plate or outside cancels activation;
      leaving the window, losing focus, a key press or changing scenes
      cancels the held gesture. Entropism's dashboard opts into
      `dashboard_cursor`: the single fill moves on hover and switches
      off while held, using the existing on/off tile drawings. Only
      the render pick changes; hovering EMAILS does not open mail or
      replace the detail panel. Keyboard input restores the keyboard
      selection's fill. Other eras keep their existing drawings but
      share the release/cancel behavior. Store hover and mailbox
      hover/press feedback are still open, as are the other eras'
      dashboard visuals.
      No trace or golden was edited. Verification is headless; the desk
      pass remains open.
    - **Neomil dashboard states, 2026-09-14:** all six
      diamonds now lift to `#f63333` on hover and use the held
      `#a52223` / `#420f10` pair while pressed. This is the component
      sheet's **inferred filled-control rule** applied to the hub,
      not a newly sourced observation: every diamond is already filled,
      so hover uses the brighter rung rather than an outlined control's
      wash. `PlateStates` carries each module's transient drawings;
      the shared scene paints them inside the plate's existing
      translation/rotation/clip. Upper and lower diamonds reuse their
      exact rest paths, inset outline and glyph geometry. The keyboard's
      selection and surrounding labels/detail panel are unchanged;
      release/cancellation uses the preceding pass's gesture state.
      Tests cover every diamond's geometry/colours, targeting only the
      hovered plate, clearing feedback and preserving selection.
      Kitsch dashboard visuals, store hover, mailbox feedback
      and animation between interaction states remain open. Headless
      verification only; desktop interaction verification remains open.
    - **Neokitsch dashboard states, 2026-09-14 (working tree):** all
      six cascade cards lift their existing rings `#bd8951` to
      `#e8ab66` and their front outline to `#f2b463` on hover, keeping
      the six opacities and every path/tab/label unchanged, per
      `components.svg`'s inferred `nk-card-hover`. Press reuses the
      selected veneer drawing with its grain and inset tab. Selected
      cards retain their veneer under the pointer, including after
      a successful release; cancelled drags leave selection untouched.
      Tests cover card origins, unchanged hover geometry, exact reuse
      of the selected drawing, cancellation and release-to-selection.
      No trace or golden was edited. Desktop verification, animation,
      kitsch dashboard feedback and the other custom faces remain open.
    - **Mailbox cursor and gestures, 2026-09-14 (working tree):**
      all four mailbox lists now share the scene's release-to-select
      gesture state, parameterized by row index. Dragging to another
      row or outside, leaving the window, losing focus, pressing a key
      or changing the row table cancels activation. Entropism opts
      into `mailbox_cursor`: the existing reverse-video fill follows
      hover and disappears while held. Only the list's render pick
      changes; the message panel and read/unread flags stay put.
      Keyboard input restores the keyboard selection's fill.
      Tests exercise every row in every era at uniform and stretched
      sizes, cancellation, and independent cursor/content state.
      No trace or golden was edited. Other eras' mailbox feedback,
      store feedback, animated transitions and desktop verification
      remain open.
    - **Entropism store categories, 2026-09-14 (working tree):**
      `store_cursor = Some(Category)` applies the shared scene cursor
      to all five category cells. Hover moves the one reverse-video
      fill; holding the hovered category drops it. The chosen category
      and product card stay unchanged until release, and keyboard
      input restores the keyboard selection's fill. Product-card
      growth remains selection, not hover. Tests traverse every store
      plate across all four eras, verifying the cursor affects only
      Entropism categories and never changes the selected card's size.
      No trace or golden was edited. Store product-card feedback,
      other eras' store feedback and desktop verification remain open.
    - **Other store categories, 2026-09-14 (working tree):**
      Neomil, Kitsch and Neokitsch now supply `store_states` for all
      five categories; the shared scene applies their sheet's inferred
      hover/press drawings. Neomil uses the wash and held-red coats,
      with separate selected-state drawings to preserve its 67px
      selected row versus 62px rest row. Kitsch adds one ghost at
      (+20,-20) behind the teal face, then takes the flat selected
      yellow while held. Its foreground ghost uses the existing
      approximate canvas alpha blend, not the hub's software composite.
      Neokitsch adds seven outward echo rings and reuses its selected
      veneer while held. Kitsch/Neokitsch retain selected material on
      hover. Product feedback is recorded below. Tests cover the
      category tables, geometry/material preservation, selected-state
      precedence and gesture cancellation. No trace or golden changed.
      Remaining interaction work: Kitsch dashboard ghost layers,
      remaining custom mailbox/control faces,
      and animated transitions. Desktop verification remains open.
    - **Tray submenu hover, 2026-09-14 (working tree):** enabled
      rows now report enter/exit separately from clicks. Hover opens
      submenus idempotently, including lazy `AboutToShow` expansion;
      re-entering ancestors retains descendants, and sibling leaf
      hover closes deeper branches. A click-closed submenu stays
      closed through layout-generated re-entry until its row is left.
      Disabled rows, separators and invalid paths do nothing. Keyboard
      focus policy is unchanged; keyboard navigation and live tray-app
      verification remain open.
    - **Product-card states, 2026-09-14 (working tree):** Neomil,
      Kitsch and Neokitsch now provide all four `Group::Card` entries
      alongside category feedback. Hover/hold keeps the current
      compact or expanded geometry and content, using separate
      selected-state drawings. Neomil applies the documented 22% red
      wash and held-red/dark-ink pair, preserving the cut card's page
      restoration ramps. Kitsch adds the measured offset ghost and
      teal slab, then flat amber at compact size; selected states keep
      expanded geometry. Neokitsch echoes the current silhouette and
      compresses the selected veneer material into the compact values
      band while held; expanded details remain selection-only.
      These are inferred material adaptations, not newly observed
      frames. Foreground alpha approximations and existing shelf clips
      remain as documented. Tests compare geometry/content, preserve
      category selection, and cover cancellation and selected variants.
      Validation: 178 Rust tests and all 25 golden cases pass; headless
      compact/selected previews reviewed, including Neomil's cut card.
      Kitsch's compact QR keeps its geometry with dark printing on the
      filled hover/held faces. No trace or golden changed. The next batch
      below completes Entropism product feedback and two mailbox materials.
      Kitsch dashboard/row feedback, other custom controls, animated
      transitions and desktop verification remain open.
    - **Next interaction batch, 2026-09-14 (working tree):** Entropism
      product headers now move the reverse-video cursor independently
      from selected-card growth. Holding clears the header highlight;
      moving to another card removes the old highlight but retains its
      expanded details, sockets and brand inks. `PlateStates.selected_away`
      supplies that drawing without changing selection or category feedback.
      Neomil mailbox rows use wash/bright-filled hover and held-red/dark
      printing; Neokitsch rows use an inferred silhouette echo and the
      traced veneer while held, preserving selected veneer on hover.
      Row content, unread glyphs and the shown message remain independent
      of these transient materials. All 182 Rust tests pass; headless
      rest/hover/held previews reviewed for both mailbox materials and
      Entropism cards, including selected targets. All 25 golden cases
      pass with unchanged goldens. Desktop verification remains open.
    - **Kitsch dashboard audit, 2026-09-14:** the old claim that ghosts
      remain in `HUB_BACK` with the ground is stale. `FAN_LEFT` and
      `FAN_RIGHT` already hold separate software-composited ghost groups
      under their original motion clips. Held feedback is unblocked:
      remove the target blade's trail and show its existing yellow face.
      This needs coordinated foreground/backdrop pointer state, stable
      hit-test identity, and target-only static backdrop variants. Hover
      needs a hub-specific rule: its five-to-seven resting ghosts do not
      map directly to the component sheet's one-ghost lift. Keep that
      choice open; do not invent an additional layer or reduce the stack.
    - **Kitsch row and dashboard follow-up, 2026-09-14 (working
      tree):** trace mailbox rows now draw filled ghosts behind their
      two-piece teal hover faces, then flat yellow while held. The
      separate sender line keeps its own ink; selected hover preserves
      the traced yellow material. Dashboard held variants remove only
      the target blade's ghost trail and use its existing yellow face.
      Shared foreground/backdrop feedback preserves scene identity and
      clears together with activation or cancellation. Existing fan clips
      and the software compositor remain intact. Hub hover is unchanged
      pending the interpretation above. Combined validation recorded below.
    - **Native controls and working mail faces, 2026-09-14 (working
      tree):** `widgets::controls::{button, field}` retain one native
      child and draw Kitsch stepped/ghost/amber and Neokitsch tab/echo/
      veneer materials from era data. Optional callbacks disable the
      native control and its material together. Native input focus,
      selection, caret, operations and overlays remain delegated. The
      control-states example shows static materials and live controls
      for all eras; dimension/grain adaptations are documented in
      `docs/control-materials.md`. The simple catalog remains available.
      Working mail rows and DELETE now use custom surface feedback;
      Entropism shares one list cursor and extinguishes it while held.
      Passive action labels stay passive. Touch ownership, release/cancel,
      changed row identities and stationary-pointer scrolling are covered;
      redraw cannot resurrect keyboard/focus-cancelled hover. DELETE's
      destructive material adaptation is explicit, preserving alert at rest.
      All 199 Rust tests and five extractor tests pass. Headless control,
      Kitsch row/blade and working-row previews reviewed; all 25 goldens
      passed unchanged and all 19 `./check` checks passed. Live desktop/IME
      verification and animated transitions remain
      open. No golden or trace was edited.
    - [x] **Reader follow-ups verified 2026-09-14.** Entropism's
      filled first row differs from the open second-row message;
      trace comments and sheet captions now distinguish the adopted
      cursor reading from other reverse-video fills. The drawing and
      animation IDs are unchanged. Neomil `5707b7` is visibly the title
      card at #53; three-card Login is #59. The alleged Login `__`
      correction conflated two marks: the password caret was already
      correct, while the separate Login-button slot was solid in the
      trace and hidden behind the action in Rust. Trace, sheet and app
      now use the measured hollow U, drawn above the action independently
      of blinking. Complementary held-out SVG crop RMS falls from about
      23.23 to 3.09/2.95; app caret-on/off captures preserve the slot.
      Its state semantics remain unknown. Kitsch's open envelope remains
      read/unread, not a state. See [the source audit](../docs/source-followups.md)
      for measurements, evidence and limitations.
  - [x] **Phase 2 (2026-09-06): one boot-in.** Eased transitions as
    scene data: `Prim::Motion { motion: Motion { id, begin, dur, ease,
    change }, prims }` with `Change::Clip` (the trace's `<clipPath>`
    with an animated `<rect>`), painted by `scene.rs` through
    `Frame::with_clip` at `motion::progress` for the scene's `at`;
    `Easing` is lilt's, straight off the PIPELINE.md lookup. Carried
    end to end for neomil's GO HOME panel (`#panel-open`, 0.36 s
    EaseOutCubic top-down wipe from 0): the dashboard ticks at 16 ms
    until `REST` and then stops. Convention change that came with it:
    the rest frame is the trace, not frame 0, so a boot-in is written
    backwards from the drawing (the traced element is the `to`).
    Verified on the headless pipeline only (`triptych.sh --at 0.15`
    matches Firefox's frame); the wipe on terra's display is unseen
    until a switch. Other eras' dashboards and the store/mailbox have
    no boot-in annotated yet -- that is trace work per era, and each
    should read as *that* era comes up, not neomil's wipe copied
    (neokitsch has one since phase 3, below).
    `Change` had one variant then; opacity (`<animate attributeName=
    "opacity">`) is the obvious second and needs a group-alpha path
    the canvas does not have (a `with_clip` draft has no alpha), so
    it was a `Soft`-style composite or a per-prim ink blend -- the
    blend, in phase 3.
  - [x] **Phase 3 (2026-09-06): the neokitsch boot-in, and opacity.**
    The live era's dashboard comes up its own way: `#cards-open` wipes
    the cascade (cards, labels, captions) on from the left over 0.5 s
    EaseOutCubic, so the staircase steps up in reading order, and
    `#panel-fade` fades the detail panel in over 0.3 s from 0.4 s,
    EaseOut. `Change::Opacity` is the per-prim ink blend: `scene.rs`
    threads an `alpha` through `paint` beside `k`, and `ink` fades the
    source alpha before the linear rebase, so a solid ink at .4 is
    rebased exactly as the trace's `.4` (`a_faded_ink_is_the_
    translucent_one`). The stack limit is real and visible on this
    very panel: the dark paragraph bars over the gold body sit lighter
    mid-fade than the trace's group opacity draws them (the diff row
    of `triptych.sh --at 0.55 neokitsch dashboard` lights them solid),
    for 0.3 s at boot. iced's canvas has no group alpha (only `image`
    and `svg` widgets carry an `opacity`), so the exact route is the
    `Soft`-style composite -- rasterise the group and draw it as an
    image at alpha -- which needs `soft.rs` to rasterise text; not
    worth it for a few frames unless a longer fade wants it. Two
    conventions learned: a delayed `begin` needs a `<set>` holding the
    `from` in the trace (SMIL shows the base value before begin, and
    the base is the rest value; `Motion::begin` already holds on the
    iced side), and a clip must clear the halo, not just the ink --
    the first rect at x 180..920 nicked the DEVICES card's glow at 919
    (54 px in the rest frame), so the rule is a rest-frame pixel diff
    against the unannotated trace before anything else. The phase-2
    commit had also put two vocabulary-table rows at the top of
    `PIPELINE.md`, fused to the title; put back. `every_motion_rests_
    before_rest` checks every era's boot-ins end by `motion::REST`.
    Goldens untouched (rest frame). Wipe verified against Firefox at
    0.15 s (0.2% of pixels), fade at 0.55 s; on terra's display unseen
    until a switch. Entropism and kitsch dashboards and every store and
    mailbox still have no boot-in.
  - [x] **Phase 4, iced half, neomil and neokitsch (2026-09-07): the
    stores wipe in; the mailboxes are transcribed but not wired.** The
    two store tables carry their traces' `#shelf-open`: neomil's
    `STORE` wraps the four cards (now the `SHELF` table, one
    `Prim::At` per column) in a top-down clip x 430 y 144 w 1170, h 0
    -> 664 over 0.5 s EaseOutCubic; neokitsch's `CONTENT` wraps its
    `SHELF` in a left-to-right clip x 340 y 205 h 530, w 0 -> 1240,
    same curve, and `GROWN` holds the grown card's body (`GROWN_BODY`:
    the dark plate, its grain, the 620 and the socket rows) under
    `#body-fade` + `#body-fade-text` as ONE `Change::Opacity` 0 -> 1
    over 0.3 s from 0.4 s EaseOut. Two deliberate departures from the
    trace's letter, both invisible in frames: neomil's rect runs to x
    1600 rather than the trace's 1570 because the iced card 4 is cut
    by `CARD4`'s covering strip out to the frame edge (the trace cuts
    it with its own `#c4clip`), and a rect ending at 1570 shaved 30 px
    of that strip (43 px, <= 7 levels, the golden no longer
    byte-identical); and neokitsch's body fade sits inside the
    selected card's `Plate::on` rather than after the shelf, where the
    trace paints it, because the body must follow the selection and a
    second Plate per card is forbidden (`every_era_offers_five_
    categories_and_four_cards`) -- the wipe has cleared x 929 before
    the fade begins, so the stacking is the same in every frame. The
    mailboxes are layouts, not display lists, so their five `<animate>`s
    are `MAILBOX_MOTIONS` tables (`MailMotion` over `MailPart`s, the
    phase-4 mechanism `screens/mail.rs` paints) at the foot of each
    era's mailbox section: neomil `#list-open` (List; x 125 y 305 w
    400, h 0 -> 578, 0.44 s) and `#message-open` (Panel + Buttons; x
    720 y 304 w 745, h 0 -> 464, 0.36 s from 0.15 s); neokitsch
    `#list-open` (List; x 15 y 240 h 460, w 0 -> 520, 0.5 s),
    `#bar-fade` (Fills, which in this era is the selection bar alone;
    0 -> 1 over 0.3 s from 0.4 s EaseOut) and `#buttons-open`
    (Buttons; x 715 y 660 h 90, w 0 -> 810, 0.4 s from 0.3 s). Not
    live on that agent's pass: `Style::mailbox_motions` (style.rs)
    answered `&[]` for both eras, left so because the era files and
    the shared files were two agents' on the day (wired later that
    day; see the sub-item below). Verified with the arms flipped in a scratch
    copy: neokitsch's mailbox is byte-identical to its golden at REST
    and its frames are within 0.2 of the screen's own 2.1% baseline
    (0.15 s 1.8%, 0.35 s 1.9%, 0.55 s 2.3%); neomil's frames 0.5% /
    0.8% / 0.8% against a 0.8% baseline, but its rest frame loses the
    "Urgent Information (!)" heading (179x17 px at 743,272) because
    `Sheet::panel` draws `panel.title` under the Panel cover and the
    trace keeps that label above the clip. Two `mail.rs` details
    before the arms flip: draw the panel heading outside the Panel
    cover (its own `MailPart`, or unclipped when it sits outside
    `panel.frame`), and let an era say its selected row's printing --
    `sel_notch`, envelope, title, FROM: -- rides the Fills cover, as
    neokitsch's fades in with the bar (:404-418, :538-550); today it
    rides the List wipe and stands dark on the bare ground for 0.4 s
    (`triptych.sh --at 0.15 neokitsch mailbox` shows it). Entropism
    must keep the opposite (its scan draws the text, `#select-lit`
    lights the plate after), so it is a table knob, not a rule.
    Stores verified against Firefox: neomil 0.5% at 0.15 s and 0.35 s
    (rest 0.5%), neokitsch 0.7% / 0.8% / 1.0% at 0.15 / 0.35 / 0.55 s
    (rest 1.1%); goldens byte-identical at REST. Matrix 25/25 once
    `store.rs` ticked (it pinned its scene at `Duration::ZERO` until
    this day, which held every store clip at `from` and failed both
    store cases at 16.5% / 9.3%). Unseen on terra's display until a
    switch.
    - [x] **Wired (2026-09-07, later the same day).** `Style::mailbox_
      motions` and its era `match` are gone: `motions: &'static
      [MailMotion]` is a `Mailbox` field, and every era's literal names
      its own `MAILBOX_MOTIONS` (entropism's and kitsch's are the
      phase-4 tables from the same day; `every_motion_rests_before_
      rest` walks `style.mailbox.motions`). The two `mail.rs` details
      became two `MailPart`s, so each era's table says which cover
      its heading and its selected row's printing ride rather than
      the sheet assuming: `MailPart::Title` is the panel heading and
      sender, drawn by `Sheet::title` as its own step after the Panel
      -- entropism's `body-scan` and kitsch's `message-extrude` name
      it (their traces put the heading inside the clip), neomil's
      `#message-open` does not (the trace keeps "Urgent Information
      (!)" at y 287 above the clip's y 304), so the rest frame keeps
      it; `MailPart::Printing` is the selected row's `sel_notch`,
      envelope, title, FROM and NEW pill, drawn under `Sheet::also`
      inside the List cover -- neokitsch's `#bar-fade` names it beside
      `Fills` (trace :340-550: bar, grain, inverted tab, envelope,
      title, FROM all fade together), entropism / neomil / kitsch do
      not (`#select-lit` lights only the fill under ink the scan drew;
      `#row1` is inside the list wipe; kitsch's row is static). Numbers
      after the wiring: matrix 25/25, every cell 100.000%; 93 lib + 45
      integration tests; Firefox triptychs neomil 0.5% / 0.8% / 0.8%
      at 0.15 / 0.35 / 0.55 s (rest 0.8%), neokitsch 1.8% / 1.9% /
      2.3% (rest 2.1%) -- the same as the scratch flip, now with the
      heading at rest and no dark printing on bare ground at 0.15 s.
      Headless; unseen on terra's display until a switch.
    - Three things the sheet's mechanism learned that are not in its
      module doc's one paragraph:
      - **Draft ordering.** iced 0.14's `Frame::with_clip` pastes the
        draft's meshes into the parent *ahead of* everything the
        parent drew directly (a frame's own buffer is batched only at
        `into_geometry`), so a clipped region lands under chrome drawn
        before it -- the entropism REPORT SPAM ring showed through its
        own 2 px inner edge, 638 px off the golden. `Sheet::under`
        therefore drafts every region, clipped or not (a full-frame
        clip when unclipped), so paint order stays draw order; a draft
        nested in a draft (`Sheet::also` with a clip) is pasted under
        its parent's own drawing the same way. The same hazard holds
        for `scene.rs`: a clipped `Prim::Motion` paints under any
        sibling drawn directly before it, so a display list must not
        rely on a clipped Motion covering an earlier unclipped prim
        (the kitsch fans are disjoint in pixel coverage for this
        reason; `the_kitsch_fans_share_no_pixel` pins it).
      - **`Fills` granularity.** `MailPart::Fills` is every
        reverse-video fill on the sheet as one part -- the selection
        bar, the panel `head`, the filled chevron / button, the badge's
        selected fill -- because entropism's `#select-lit` lights all
        of them at once. An era that ever lights one and not another
        needs the part split, not a second alpha.
      - **`Store::at` is `pub(crate)`** for the hub test
        (`opening_a_screen_starts_its_clock`), which reads a screen's
        clock after `Hub::open` re-bases it via `motion::onset()`;
        `login.rs` was touched for one line (the Backdrop `at:` field)
        and nothing else.
  - [x] **Phase 4 at the desk (2026-09-07): the delay and the flash.**
    The first look on terra: "a strange delay when going from
    dashboard to email or back. Same for store. On initial dashboard
    app load, I do see some incomplete lines drawn for a second
    before the dashboard app shows." Not motion's fault -- the soft
    ground's. `SoftCache` composited the whole 4K surface on one
    thread (~600 ms), keyed on the tree's *shape* so a route change
    threw the ground away and recomposited, and handed iced one ~66
    MB image that `iced_wgpu` uploads asynchronously above
    `MAX_SYNC_SIZE` (2 MiB, `iced_wgpu/src/image/cache.rs`) -- so the
    first frame to hold it drew the lines and not the haze. Now
    `soft.rs` composites in horizontal bands under 2 MiB each on a
    thread per band (`std::thread::scope`; `composite_bands`,
    `composite_over_bands`; `bands_are_the_one_buffer_composite`
    pins that the bands are the single-buffer result cut up), and
    `SoftCache::shared()` is one process-wide store holding every
    screen's bands (`scene::Bands`; the bar's haze draws the same
    bands). First composite 102 ms, hits in microseconds, every band
    a synchronous upload. ~66 MB RGBA resident at 4K, on purpose.
    Measured in the sandbox only: the desk has not yet confirmed the
    delay gone or the haze in the first frame, and Hyprland's
    initial resize may cost one more ~110 ms composite.
  - [x] **Neokitsch dashboard at the desk (2026-09-07): veneer and
    words.** "The email shard gold selector should be textured iirc"
    and "the email folder on the right should probably display some
    preview context" -- both the photo's own, both flattened by the
    first pass. Grain at the photo's pitches (2.1 card, 2.7 panel)
    on the EMAIL card and the panel body, drawn as the store's and
    mailbox's is (`Prim::Turn { angle: -90 }` over `Prim::Grain`,
    which is horizontal-only and rectangular; the photo's contours
    wander and the prim cannot, left as its limit); the panel reads
    the inbox's selected message re-wrapped to its measured line
    ends (`PANEL_COPY`, `panel_copy_is_the_inbox_message`), the
    two-line tape under it, and each cascade card its "ONLY CC35
    CERTIFIED" caption as micro text at the `CASCADE` origins. Trace
    gains the same; gate PASS, inks 0.64. Golden re-blessed.
    - [x] **Neokitsch striped-body extraction, 2026-09-14 (working
      tree).** The existing 5x5 close already recovered the primary
      veneer body; sparse secondary grain still fragmented. The repair
      requires repeated fine strands and an independently measured solid
      body in a similar ink, preserving background gaps and unrelated
      shapes. Dashboard inventory rises 34% to 97%, store 89% to 94%,
      mailbox 84% to 98%, with no render, golden or threshold changes.
      Five regression tests pass. Audits of all 25 goldens and 56 cached
      fixtures change only Neokitsch veneer: 22 and 48 respectively have
      identical segmentation. Do not compensate for extractor faults in
      the UI or trace.
