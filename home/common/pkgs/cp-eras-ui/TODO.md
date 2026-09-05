- [x] create scripts/download_images.py to download all the neomil related images from:
  - https://www.behance.net/gallery/118663901/Cyberpunk-2077User-Interface-(Part-1)
  - https://www.behance.net/gallery/133185623/Cyberpunk-2077User-Interface-(Part-2)
- [x] define main colors.rs:
  - primary red #FF3B45
  - primary black #DEDE17
  - also need to set opacities properly
- [x] create iced advanced container. "chip type 1"
- [x] Reproduce dashboard image (`img-07-dashboard.png`) in demo app:

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

  **Closed 2026-09-03 by the `Layout` fold** (item in the next section):
  the neomil dashboard is now the `DASHBOARD` Prim table in
  `src/eras/neomil.rs`, transcribed from the trace — six diamonds as
  `Plate`s from `#cell-up`/`#cell-down`, the GO HOME panel, the glow
  rasterised at compile time from the trace's own stop tables — and
  scores G2i **96%**. The three sub-items below are therefore done, but
  not as the widgets they name: there is no `InfoPanel` or
  `DiamondMenu` widget, and there will not be one.

  - [x] ~~Implement custom background (gradient/glow)~~ — landed
    2026-08-31 as stacked strips forming a "cold-blue top band" with
    crest blocks. **The premise is wrong**: the source has no band and
    no edge. It is a broad blue glow over near-black, at full strength
    to y~250 and gone by y~420, with a warm near-black vignette down
    the left margin. Measured stops are in `dashboard-trace.svg`'s
    `glowh`/`glowv`. Redo against those.
  - [x] Implement `InfoPanel` widget (chamfered corners) — **reopened**, then closed by the fold (see above).
    The old note said "the ops-charts material shows no such panel";
    it does. The GO HOME panel is the right-hand third of the source:
    230x443, chamfered top-left (14) and bottom-left (42), 1px bright
    border over a dark-red translucent fill, heading + two body
    paragraphs, a scrollbar rail on its right edge and a maker's mark
    at its foot. Note the chamfers are top-left/bottom-left, not the
    top-right/bottom-left this item used to claim.
  - [x] Restore `DiamondMenu` — **reopened**, then closed by the fold (see above). It was built, then deleted
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
  - [x] Update demo app layout, colors, and text to match image — closed by the fold (see above);
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
- [x] ~~**Rework `docs/kitsch/dashboard.svg` and
  `docs/neokitsch/dashboard.svg`**~~ — **deleted instead, 2026-09-03**,
  with the other two `dashboard.svg`. They were "original composites"
  drawn app-first while their real source went unread (0.03–0.07 on
  the ink gate) and reworking them would have meant redrawing the
  trace under another name. The trace is the design; the *screen* is
  what has to follow it, which is the `Layout` item below.
- [x] **Decide whether `Layout` should exist — the material now answers
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
  **Settled 2026-09-03: neither.** The user chose a third option — fold
  the dashboard to a trace-driven canvas the way login/mailbox/store
  were converted, and delete `Layout` outright. What landed:
  - `Layout`, `Menu`, `Style::{layout, menu}` and `widgets::{charts,
    menu, table, marker, pill}` deleted (each grep-proved dead outside
    the dashboard; the bar reads its own `BarMenu`). The `Prim`
    interpreter moved out of `screens/store.rs` into
    `screens/scene.rs` (`Scene<M>`, `Picked`, `hit`, `scale`) and both
    store and dashboard drive it; store's capture is byte-identical
    before/after (md5-checked, G2i unchanged). `Group::Module` is the
    dashboard's `Plate` group; `Style::dashboard: &[Prim]` +
    `dashboard_selection` per era in `// --- dashboard ---` blocks.
    `screens/dashboard.rs` is 781 → 167 lines.
  - Two trace fixes first (vision): neomil's diamonds are now two
    explicit truncated defs (`#cell-up`/`#cell-down`, flats at 89) and
    the footer tape is a two-cell outlined frame; neokitsch's detail
    panel is a stepped top edge with an S-curve and four rings nested
    *inside* (the old one was the cascade card mirrored, with 42
    chamfer and six rings outside — none of it in the photo). G1:
    neomil 94 held, neokitsch 0.67 → 0.68.
  - G2i dashboard, was 0% ×4: entropism **98**, neomil **96**,
    neokitsch **92**, kitsch **31 FAIL**. Kitsch is a layout match by
    eye (see the side-by-side); the number is two measured effects, not
    the table: (1) iced/wgpu blends alpha in linear space and rsvg in
    sRGB, so the translucent ghost stacks render far brighter (ghost 6
    at α .12: design G=33, impl G=61) and the dark-teal family shifts;
    replacing alphas with linear-equivalents lifted it to 45%/IoU 0.75
    but the equivalent is backdrop-dependent, so it was not kept — this
    is a `scene.rs` decision that also touches every haze/lobe (item
    below); (2) the extractor hole-fills the BRAINDANCE panel's closed
    outline into one blob (10% of source area), which rsvg's
    62%-coverage edge escapes. Weighted IoU 0.69 = what the trace itself
    scores against the photo.
  - All five dashboard goldens re-taken (`tests/bar.nix` procedure);
    21/21 matrix green; the 16 non-dashboard G2i cells unchanged.
- [x] **Mailbox content is wrapped, not transcribed — and the shared
  premise is false** — done 2026-09-04 (findings at the end). `screens/mail.rs:22-27` says the four traces
  "agree" on content (same messages, same senders, same three lorem
  paragraphs), so `INBOX` and `BODY` are screen constants and `wrap()`
  (`mail.rs:580`) greedy-breaks the body at a mean-advance guess
  (`ADVANCE 0.464`). The 2026-09-04 triptychs (`scripts/triptych.sh`)
  say otherwise: kitsch's and neokitsch's traces set *three* paragraphs
  with different splits and no "Nemo enim…" (iced draws a fourth in
  kitsch); neomil's trace list is "List of messages / I'm worried man /
  Heist data sent to you…", every row from Jackie, not the entropism
  inbox; and every era's line breaks differ from the trace because the
  traces set each line explicitly, hyphenated ("incidi-" / "dunt").
  Same failure class as the invented traces — a premise stated in code
  that nobody checked against the material. Fix: rows and body lines
  move into the era tables verbatim from each `mailbox-trace.svg`
  (`&'static [&'static str]` per paragraph), `wrap()` and `ADVANCE` go,
  and the `mail.rs` module doc is corrected rather than deleted. Largest
  visible mailbox diff in all four eras; kitsch mailbox G2i 67 is partly
  this.
  **Done:** `style::Mail { subject, from, unread }`; `MailList.rows:
  &[Mail]` replaces `count` + `new_rows` (the NEW pill follows `unread`
  per row); `MailPanel.paragraphs: &[&[&str]]` (lines as the trace sets
  them, hyphens included) replaces `wrap`, plus `heading`/`sender`
  options that pin the resting panel text where no row supplies it. Per
  era `static ROWS` / `static PARAGRAPHS` in the `--- mailbox ---` block
  cite trace line numbers. `mail.rs` lost `Mail`/`INBOX`/`BODY`/`wrap`/
  `ADVANCE`; the `FromAt::Trailing` inset went 8→7 (neomil anchors
  every sender end at x 504). All four renders now match their trace
  on rows, senders, envelope state, paragraph count and every
  hyphenated break (checked by eye on neokitsch). G2i mailbox
  100/67/95/**84** (entropism/kitsch/neomil/neokitsch): kitsch did not
  move — its 67 is the hole-fill, not content — and neokitsch's 86→84 is
  the untouched top-right wire band re-binning (agent rebuilt the
  pre-change sources in a worktree to attribute it; ink IoU 0.32→0.38).
  What the traces actually say, none of it in the old constants:
  - entropism: only Mom is unread; the panel writes `from: Mom` where
    the list has `FROM: MOM` (pinned via `sender`).
  - neomil: the resting heading "Urgent Information (!)" is no row of
    its own list (trace line 320); pinned via `heading`, `message: 1`
    is only where a click returns to.
  - neokitsch: rows 6 and 7 are both Rachel Ross; rows 1, 3, 7 are
    open and the *selected* row is closed. Its body is set at weight
    600 (line 462); until 2026-09-04 iced drew it Medium (see the
    renderer-limits item below), now `Run::semibold()`.
  - kitsch: no open-envelope glyph at all; all five rows closed.
  - `ADVANCE`'s doc-claimed second reader (right-aligning the sender)
    did not exist.
- [x] **Rotated text** — done 2026-09-04 as `Prim::Turn { x, y, angle,
  prims }`, a group transform beside `Prim::At` (the trace's
  `<g transform="translate(cx cy) rotate(a)">`) rather than a field on
  `Prim::Text`, which every era table constructs. `scene.rs` interprets
  it in `paint` (`with_save` + translate + rotate), `hit_at` (inverse
  rotation of the point) and the test-only `plates`. iced's
  `Frame::rotate` matrix is SVG's `rotate(a)` exactly, so trace degrees
  pass straight through (positive = clockwise, y down; unit test pins
  it). Kitsch's six blade labels now turn with their blades, PRODUCTS
  as one run at 90 (the `stack!` glyph column is gone). G2i kitsch
  dashboard is blind to it — 45%/0.75 before and after; the shape gate
  cannot see a 19px label's orientation — so this is a by-eye fix, and
  the fan crop shows the labels along the blades as in the design.
  Letter-spacing 2 is still dropped (renderer-limits item below).
  Originally: split out of the renderer-limits item below
  because it is the one gap visible at reading distance: kitsch's
  dashboard blade labels (±30° in the trace) are drawn upright and the
  two PRODUCTS labels one glyph per line. `Prim::Text` grows an
  `angle` (degrees, default 0) and `scene.rs` draws it under
  `frame.with_save` + `translate`/`rotate` about the anchor point;
  kitsch's `DASHBOARD` table then sets the six labels as the trace
  does. Letter-spacing, stroked text and Rajdhani 600 stay in the item
  below.
- [x] **Two G2i FAILs the triptychs say are gate artefacts, not screen
  faults** — investigated 2026-09-04, both were, in different ways:
  - Entropism login 28% → **100% PASS**, two causes stacked. (1)
    `render.sh`'s 3s settle predates the login washes: a 3s capture of
    *every* era's login lacks its wash (entropism comes out flat
    palette bg; the other three differ from their goldens over most of
    the frame), while 5/8/15s are byte-identical to the goldens.
    `DEFAULT_SETTLE` is now 8 with the measurement in its comment. This
    also means the 2026-09-04 triptych login rows and every earlier
    G2i login number were taken without the wash. (2) With the wash
    present the extractor still split rsvg's radial into a
    border-touching ground (#151209) and a 33%-of-frame inner band
    (#18160d) it fitted as a 1044x586 "chamfer"; the app's smoother
    wash quantised as one ground. `extract_spec.py` now folds a
    non-border cluster within 10 RGB units of a ground cluster into the
    ground (`GROUND_NEIGHBOUR_DIST`). Full G2i matrix re-run on the
    pre-wave binaries: only the three non-neomil logins moved.
    Entropism 28→100 is above; kitsch 97→**72** and neokitsch 67→**89**
    are both the settle alone (kitsch re-captured at 3s under the new
    rule still scores 97 — with 22 "invented" shapes against 4 now, so
    the 97 was a wash-less capture scoring against a design with a
    wash). The 72 is honest and its residual is one colour bin: the
    barcode digits are drawn in the bars' bright teal where the trace
    has them dim (#518b7e), so the extractor merges them into the bars
    (63px tall vs the design's 51). Rule is unchanged for every
    non-login cell. G1i under the new rule: all 16 PASS; shape-area %
    entropism 83/92/88/79, kitsch 56/59/20/80, neomil 79/94/86/71,
    neokitsch 78/54/79/56 (login/dashboard/mailbox/store; kitsch and
    neokitsch gate on ink IoU, not on these).
  - Neokitsch bar 52% (14% since the rings nested inside the panels,
    2026-09-04): by eye a near match (`/tmp/g2i-neokitsch-bar/
    side-by-side.png`); the number is the extractor fusing the popup
    panels, their onion rings and the tray cells into single components
    on one side and not the other (impl `blob-01` [1094,3,443,116]
    swallows both menus and the tray strip). The real deltas are small
    and belong to § "Bar restyle": tray diamonds are purple/orange in
    the app where `bar.svg` has gold/black. Left FAIL; not a gate rule
    to add.
- [x] **Login first paint takes 3-5s — it does not; the harness was
  presenting late.** Filed 2026-09-04 as fallout of the above: the wash
  (`login.rs` `wash_image`, a software-rendered 1600x900 radial) was
  absent from a 3s capture and present from 4s, so the item blamed the
  raster cost and proposed precomputing it off the render thread.
  Measured the same day and the premise is wrong. Release and debug
  captures are byte-identical at every settle (so not CPU cost), and
  `eprintln!` probes in `Backdrop::draw`/`Art::draw` show the app draws
  exactly two frames — at ~60ms and ~130ms after weston starts, the
  wash rasterised in the first in ~19ms — and never draws again. The
  3-5s was the second frame *reaching the compositor*: wgpu's default
  FIFO presentation blocks `present()` until headless weston (pixman,
  no vblank) returns a frame callback, which it does seconds late for
  an idle surface. With `ICED_PRESENT_MODE=mailbox` (or `immediate`,
  `iced_wgpu` `settings.rs`) a 2s capture of the same binary is
  byte-identical to the goldens on 19 of 20 cells (the 20th was the
  kanji item below, fixed 2026-09-04), and no cell moves between 2s
  and 8s under either mode. `render.sh` now exports mailbox and
  `DEFAULT_SETTLE` is back down to 4 (~8s per G2i cell instead of ~13); the sandbox keeps FIFO
  and 15s, unchanged. Still open and not chased: *why* frame 1 lacks
  the wash when `iced_wgpu` uploads an image in the frame that draws
  it — at most a one-vblank flash on a real compositor, which is the
  whole of what this item was about. Do not precompute the wash.
- [x] **store/neomil golden has tofu where the host draws kanji.**
  `tests/golden/store-neomil-1600x900.png` rendered `益荒男`
  (`eras/neomil.rs` at 192,104) as three boxes: the sandbox has no CJK
  font, and `render.sh` on this host found one, so it was the only
  cell whose capture was not byte-identical to its golden. Fixed
  2026-09-04 the way the item proposed: `home/common/pkgs/noto-cjk-subset`
  takes the JP face out of nixpkgs' `NotoSansCJK-VF.otf.ttc`,
  `pyftsubset`s it to the three glyphs and freezes `wght=700` with
  `varLib.instancer` (5KB, family name kept as "Noto Sans CJK JP" so
  cosmic-text's Han fallback asks for it by name); `default.nix` and
  `shell.nix` stage it into `fonts/` beside Orbitron and Rajdhani,
  `fonts::NOTO_SANS_CJK_JP_BOLD` embeds it, and the five scene
  examples load it. Sandbox render now equals the host capture at
  AE 0. Riding along: the logotype is `Prim::Tracked` at the trace's
  letter-spacing 5 (bbox 148 wide vs the design's 150; was 138 as
  plain `Text`). Not drawn: the trace's `skewX(-13)` — `Prim` has no
  shear. G2i 78 → 79, `--diff` 0.52%. **Setting any new CJK string in
  an era table means extending that package's `text` first**, or the
  sandbox is back to tofu; `fonts.rs` says so at the const.
- [x] **Gate-side blending — the gate was right; the app was blending
  wrong, and the fix is `Prim::Soft`** (2026-09-04). This item used to
  say the residual after the alpha fix was the extractor's hole-fill
  plus linear gradient *interpolation*, and proposed an `extract_spec.py`
  mode that composites the design in linear light. Both wrong. Measured
  on the kitsch dashboard (45% FAIL): at ghost centres G was exact but
  R sat 8-10 levels bright and B 3-4 dark (design `68 49 55`, painted
  `76 47 52`), and each card split into a too-bright and a too-dark
  half where the layer count under it changed. Paste experiments —
  overwriting only the pixels off by more than N levels with the
  design's — put the gate's cliff at N=6: >8 → 49%, >6 → 67% PASS,
  >4 → 100%. So the FAIL was colour, 4-6 levels of it, on the faint
  ghost tails. Why no alpha fix reaches it: rsvg applies `fill-opacity`
  on encoded sRGB values, wgpu in linear light, and a rebased layer adds
  a *fixed* amount of linear light where the trace's adds more over a
  brighter backdrop; per channel, teal over pink must darken R while it
  brightens G, which no one alpha does. Tried and rejected before the
  fix: a full under-model (sample the backdrop at each prim's centre
  through every earlier fill, rebase against that) — 45→49% only, for
  ~200 lines; per-channel ink solving on top — exact at one point,
  flattens the ghost against the haze beside it. And the gate-side
  "linear-light design" mode is impossible: rsvg has no such mode, and
  it would move the design away from the drift, not the implementation
  toward it. What landed: `Prim::Soft { prims }` — a sub-scene
  composited in software, in sRGB, by `screens/soft.rs` (scanline
  even-odd fills with 4 sub-rows and exact span fractions, distance-band
  strokes, analytic lobes and washes, `At`/`Turn`) and drawn as one
  opaque image in a `Backdrop` canvas *under* the scene's canvas
  (`Scene::view`) — under, because iced batches a canvas layer as all
  meshes, then all images, then all text, so an image in the scene's
  own canvas covered every fill after it (measured: only the text
  survived). Cached per canvas size and palette (`SoftCache`, the
  program state); 66ms release / 600ms debug for the kitsch dashboard,
  paid once. Kitsch dashboard's ground+bloom+ghosts and both neokitsch
  backdrops are wrapped; ghost pixels now land exactly (`68 49 55`) and
  the cell scores **45 → 68% PASS**; the other seven cells are unchanged
  and five of the eight goldens byte-identical (kitsch dashboard,
  neokitsch dashboard and store re-taken). Two tests keep the scope:
  `soft_groups_hold_only_fills` (no text, grain, dots or plates inside
  a group) and `soft_groups_lead_their_scene` (a group is a backdrop,
  so it leads the list and is never nested). `blend_over` stays for the
  lone translucent prim over flat ground. The hole-fill note was a red
  herring: with the colour right the extractor matches 28/53 shapes.
- [x] **neokitsch backdrops are transcribed short of the trace.** Now
  that both are composited exactly (`Prim::Soft`), the residual on the
  hazes is transcription, not blending: store (150,50) design `64 54 83`
  vs painted `35 42 65`, (500,50) `98 73 115` vs `79 65 98`; dashboard
  (1550,150) `23 29 44` vs `22 23 35`. The store trace draws three
  things `BACKDROP` does not: `#hazelobe` (a third radial, cx 430 cy
  -40 r 560 scaled 1x0.30, `#7a5288` .85→.55→0, clipped to y<300 — the
  top-left lift), `mask="url(#bluemask)"` (a luminance mask fading the
  blue lobe out leftward, 0 at x=0 to full at x=640) and the 1.3°/2°
  `rotate` on both gradients. The dashboard trace has the same mask
  and rotations. Since G2i spends ~5 of its 8 clusters on a haze, this
  is where the neokitsch store's 82% likely sits.
  **Done 2026-09-04.** Table work on `Prim::Ramp`/`Prim::Masked`, as
  predicted: each haze is a `Lobe` at the origin under a `Turn` about
  its trace centre (`Turn { 770,-120, 1.3 }`, `Turn { 850,-120, 2 }`;
  dashboard 825/900), the blue goes through `Masked { .., BLUE_MASK }`
  where `BLUE_MASK` is the `#hazebluefade` greys as a horizontal
  `Ramp`, and the store adds the `#hazelobe` line. The mailbox trace
  opens with the store's three defs line for line, so `mailbox()` now
  takes `Prim::Soft { BACKDROP }` too, and the widget-level
  `Mailbox.haze` / `style::Lobe` / `mail.rs::wash_at` ring-band
  mechanism (neokitsch was its only user) is deleted. Every probed
  ground pixel is now within ±1 of the design render on all three
  screens (store (150,50) `65 53 83`, (500,50) `98 73 115`; dashboard
  (1550,150) `23 29 44`; mailbox (300,20) `88 69 110`). G2i store
  **82→89**, dashboard 94 held (38→37 shapes), mailbox 84 held; the
  `--diff` share store 1.29→1.06%, dashboard 0.265→0.262%, mailbox
  2.07→2.04% — the diff rows are dark across the ground, what is left
  is text, the veneer grain (a different strand set) and stroke AA.
  Three goldens re-taken (store, dashboard, mailbox neokitsch), 21/21.
  Note `soft_only_prims_stay_soft` caught the first attempt, which set
  `backdrop: BACKDROP` without the `Prim::Soft` wrapper: the `Masked`
  would have vanished silently and the `Turn`ed lobes drawn by the
  canvas instead.
- [x] `scripts/triptych.sh --diff` — done 2026-09-04: an optional
  fourth row, trace vs iced, so the review view points at the
  difference rather than leaving it to the eye. Not `visual_diff.py`,
  which the item named: that script draws a raw difference after a
  121x121 brute-force alignment search (minutes per pair, and two
  same-size renders of one design need no aligning), and pillow is not
  something triptych.sh otherwise needs. The row is ImageMagick, which
  it already requires: largest-channel |trace − iced| with a 2-level
  floor, square-rooted so a few levels of drift still register, on a
  black → yellow → red ramp over a 22%-grey copy of the trace; the
  caption carries the share of pixels off by more than 8 levels, the
  cliff the sRGB item measured. The trace is diffed *without* its
  `class="photo"` elements (G2i's design), so the expected halo does
  not light up. Text always lights (two rasterisers, two AAs) and
  dominates the percentage, so read the picture, not the number.
  Full run with `--diff`: 16 cells in 3m41s, all of it render.sh.
  First run's catch is the grounds item below.
- [x] **Four grounds are drawn from memory, not from their trace.**
  Done 2026-09-04 — see the **Done** note at the foot of this item.
  `--diff` lit the whole frame on the kitsch mailbox (7.8% off by >8),
  neomil mailbox (5.4%), entropism store (1.6%) and neomil store, and
  the G2i captures say why (design vs implementation, sRGB):
  - kitsch mailbox: the trace is `#bloom`, a radial at (800,−155)
    r 0.9 of a 1600x620 rect — `b05064` → `933b53` → `5c2236` →
    `1e0f14` → page — plus `#leftwash`, `2a2e2a` fading right from
    x=0 over y 60..840. Column x=800: design `153 63 87` at y 0, `38 18
    25` at 300, page `14 13 12` from 450; implementation `71 21 36`,
    `67 20 34`, `59 18 30`, still `47 15 24` at y 650 — a flat maroon
    over the whole frame, and `11 11 7` where the left wash should be
    `34 37 33`. It is `Ground::Bloom`, the generic 26-disc stack out of
    the top-right (`widgets/ground.rs`), which the `--- mailbox ---`
    header in `eras/kitsch.rs` wrongly says handles "the rose bloom and
    the grey-green left wash". The store on the same era already has
    `BACKDROP` with the right `ROSE`/`MARGIN` lobes; the mailbox needs
    its own from the mailbox trace's numbers, and G2i's 67 there —
    "PASS with a known reason", the chevron cells — is at least partly
    this.
  - neomil mailbox: the trace stacks `#glowh` under `#glowmask`, a
    `#wash` under `#washmask` and a `#vignette`; the implementation is
    `Ground::Flat` — `5 3 4` everywhere the design reads `31 31 34`
    (50,150) and `28 6 8` (800,450) — and a red element at (300,700)
    is `94 17 18` against the trace's `33 8 9`.
  - entropism store: the trace's `#lift` radial (cx .45, cy .4, r .8)
    is not drawn; the implementation is flat `17 12 7` where the design
    runs `22 20 9` … `28 27 16`, 5–11 levels under it everywhere.
  - neomil store: `STORE` draws the top glow as a plain vertical
    `Prim::Wash` 0..540, but the trace's `#glowh` is horizontal *and*
    masked by `#glowmask`, so the implementation is blue-grey at the
    top-left, `27 37 70`, where the design is `26 22 24`.
  The mailbox screen (`screens/mail.rs`) has no per-era backdrop group
  at all — it stacks the generic `ground()` under the sheet — and the
  entropism/neomil store tables carry none or an approximation. Fix is
  the kitsch-store pattern: a leading `Prim::Soft` group per screen
  transcribed from the trace's gradient defs. Masked gradients
  (`#glowmask`, `#washmask`, and neokitsch's `#bluemask` in the item
  above) need `soft.rs` to learn a luminance mask first; do that once.
  Not chased under the `--diff` item: it is transcription work on four
  cells, and the dashboard/login cells are dark on the same row, so
  the row is telling the truth.
  **Done 2026-09-04.** Two prims, both composited-only (`paint` skips
  them; `soft_only_prims_stay_soft` keeps them inside `Soft` groups):
  `Prim::Ramp`, a rect under the trace's own multi-stop
  `linearGradient` (`from`/`to` in bbox fractions), and `Prim::Masked
  { prims, mask }`, SVG's luminance mask — `0.2125R + 0.7154G +
  0.0721B` times alpha on the *encoded* values, which is what rsvg
  does (measured on a test document: `#808080` passes 128, pure red
  54, pure green 183, half-alpha white 128; no linearisation), and
  `a_mask_passes_its_luminance_as_rsvg_measures_it` pins those
  numbers. `Mailbox` gained `backdrop: &[Prim]`, a leading `Soft`
  group `mail.rs` stacks under the sheet through `scene::Backdrop`.
  Then per cell, from the traces' defs:
  - kitsch mailbox: `MAIL_GROUND` — page `#0e0d0c`, `#bloom` as a lobe
    at (800,−155) rx 1440 ry 558 (r 0.9 of the 1600x620 rect; not the
    store's 0.95), `#leftwash` = the store's `MARGIN` lobe. Column
    x=800: `153 63 87` / `38 18 25` / `14 13 12` / `14 13 12` at y 0 /
    300 / 450 / 650, design and implementation alike (the item's
    numbers above were the before). 7.8% → **1.5%** off by >8; what is
    left lit is the selected row's fill and the four chevron fills,
    solid — a colour miss on those, not the ground.
  - neomil: `HUB_GLOW` = `Masked { Ramp #glowh (10 stops, horizontal),
    Ramp #glowv (9 stops, vertical) }` and `HUB_VIGNETTE`, shared by
    all three neomil screens since the three traces define them
    identically. The **dashboard** dropped its 640-strip compile-time
    rasteriser (`glow()`, three linear pieces for the nine mask stops)
    for the construct itself: G2i 96 → **99**, `--diff` 0.3%. The
    **mailbox** stacks page, glow, `#wash` (3 stops) under `#washmask`
    (black → white by 0.45), vignette; (50,150) `31 31 34` exact,
    5.4% → 2.0% — and since the panel and every row were lit solid,
    two content fixes rode along: the panel is filled `#1c0608` (:324;
    was `frame_fill: None`) and the rows `#280c0d` (:262; were
    `Ink::Border`, `94 17 18` against `33 8 9`) → **0.9%**. The
    **store**: page, glow, `#wash` lobe (288,450) rx 720 ry 405,
    `#blackv` ramp on the 540x520 rect at (1060,380), vignette;
    (50,150) `26 22 24` exact, → **0.6%**.
  - entropism store: `#lift` lobe at (720,360) rx 1280 ry 720 over its
    pad colour `#100b03`; (50,50) `21 18 7` exact, 1.6% → **1.3%**.
  Goldens moved: mailbox.kitsch, mailbox.neomil, store.entropism,
  store.neomil, dashboard.neomil. G2i: entropism store 100 → 100,
  neomil dashboard 96 → 99, and three went *down* — kitsch mailbox
  67 → 61, neomil mailbox 95 → 89, neomil store 84 → 80 — with the
  ground palette now matching the design cluster for cluster (kitsch:
  `#0f0d0d 55.5%` vs `#0f0d0d 55.6%`, all five within a level). The
  drops are shape-inventory churn: the extractor's segmentation
  thresholds moved with the ground and it now merges the kitsch
  badges at (1336,190)+(1395,190) into one 118-wide rect, and splits
  the neomil rows differently. The pixels say the grounds are right;
  the gate is measuring its own thresholds here. Follow-ups the diff
  rows pointed at, all three settled on 2026-09-04:
  - ~~neomil mailbox's four action buttons: the trace fills the idle ones
    `#1a0607` (:353) and `MailButtons` has no idle fill field — a
    struct change across four eras, so left.~~ Done 2026-09-04:
    `MailButtons.idle_fill: Option<Ink>` (`Some(Ink::Fixed(#1a0607))`
    for neomil, `None` elsewhere), drawn under the outline. (950,720)
    `1a0607` exact, was `080303`; G2i 89 held, `--diff` 0.9 → 0.87%.
    The idle strokes stay `Ink::Dim` (nix `#a32226` vs trace `#8a2024`,
    an ERAS-DELTA role) and the selected one `select` (8 levels off).
  - ~~kitsch mailbox's selected-row fill and chevron fills read solid in
    the diff: colour, not placement.~~ Done 2026-09-04: the trace has
    three yellows on this screen and the era drew `select` #fcc428 for
    all of them — row `#e8c21f` (:224), DETAILS chevron `#e6c020`
    (:280), panel tab `#fbd42c` (:256); the photo's own spot samples
    (`#ccad19` / `#dfb715` / `#fed82e`) agree that the row and chevron
    are a stop deeper than the tab. Added `MailList.sel_fill: Ink`
    (the icon cell takes it too) and `MailButtons.fill: Ink`, both
    `Ink::Select` in the other three eras; kitsch's are `Ink::Fixed`
    and its `head_ink` is `Ink::Fixed(#fbd42c)`. All three probe exact;
    G2i 61 held, `--diff` 1.5 → **1.4%**. What is left lit is text, and
    the wave at the bracket's foot: trace `#1db5a4` against `ornament`
    `#1cb39b` — 9 levels on one channel, a nix-overridden role that
    `bar.svg` uses five times, so an ERAS-DELTA delta, not chased.
  - ~~translucent fills over a composited ground are rebased against
    the flat `palette.bg`~~ — **the premise was wrong on both examples**
    (measured 2026-09-04). Entropism (720,360) is the grown card's
    *opaque* header fill, `Ink::Select` (#9cb795 then; #a6d3a7 since
    2026-09-05) against the trace's #a8d4a2: the era-role-vs-trace-hex
    choice `ERAS-DELTA.md` already
    classifies, not blending. Neomil's `c2upper` was a two-stop
    `Prim::Wash` drawn by iced's gradient, which `mix`es in linear
    light and `smoothstep`s between stops — 14 levels off a third of
    the way down with the *right* stops at both ends. Fixed the same
    day: `Prim::Ramp` is now painted on the canvas too, as flat
    design-pixel strips read in sRGB (axis-aligned only there;
    `soft_only_prims_stay_soft` checks), `c2upper` carries the
    trace's four stops and lands exact down x=900, and `Prim::Wash`
    is deleted. Card 4's cut strip, which was a flat `GROUND` rect
    under a wash and up to 25 levels off over the composited ground,
    is two column ramps sampled off the design (within ±6). neomil
    store G2i 80 held, `--diff` 0.6 → 0.5%, golden moved. No
    translucent-over-composited case is left: kitsch's ghosts are
    inside their `Soft` group.
- [x] **`scene.rs` alpha blending** — done 2026-09-04, alpha converted at
  paint time. Translucent `Ink::Fixed` alphas are composited in linear
  space by wgpu but the traces were designed in sRGB (rsvg), so every
  ghost stack, haze and lobe came out brighter in the app than in its
  trace. `Scene::ink` and the `Lobe` stops now go through `blend_over(c,
  palette.bg)`: the sRGB result the trace wants is `r = a*c + (1-a)*bg`,
  and the alpha that reproduces it under linear blending is
  `a' = Σw(lin(r)-lin(bg)) / Σw(lin(c)-lin(bg))`, luminance-weighted
  across channels; opaque and clear colours pass through. Chosen over
  pre-blending because the ghost stacks overlap each other, which a
  pre-mixed opaque would get wrong. Measured on the pre-wave bins:
  kitsch dashboard 31→**45** (still FAIL; placement IoU 0.69→0.75),
  neokitsch dashboard 92→94, the other six dashboard/store cells
  unchanged; ghost-6 pixel (1045,456) G: design 34, before 61, after 32.
  Residual on kitsch is the extractor hole-fill, not colour. Notes:
  - the backdrop is the flat `palette.bg`, so over a haze the
    conversion is approximate; neomil's `PANEL_FILL` R came out 62
    against the design's 59 (was 78).
  - the *interpolation* space of a gradient is unchanged (wgpu still
    lerps stops in linear light), so the neomil GLOW→GROUND band is
    ~14 levels brighter mid-ramp than rsvg's. Only the stops are
    corrected. If it ever matters, `soft::stop` is where extra
    intermediate stops would go (a Lobe inside a `Prim::Soft` group is
    evaluated per pixel and has no such problem).
  - iced's `web-colors` feature is the principled fix (blend in sRGB
    outright) and `Cargo.toml` turns it off deliberately — it thins
    every glyph (record under "SVG→iced pre-work"). This item is the
    scene-local alternative.
  - neokitsch's hand pre-blended `RING_25..85` consts are opaque, so
    they pass through untouched — not double-corrected, but no longer
    needed; they could become translucent stops when that table is
    next touched.
  - `kitsch.rs`'s dashboard-comment bullet "nothing is pre-mixed" is
    still true: only the alpha is rescaled, the colour is not. (Since
    2026-09-04 the ghost stacks do not go through this at all; they are
    a `Prim::Soft` group — see "Gate-side blending" above.)
- [x] Dashboard renderer limits surfaced by the transcription, all
  small and all shared with the earlier conversions. Landed 2026-09-04:
  - **Rajdhani 600.** The stated limit was "renders as Medium", and the
    mapping `Face::SemiBold => FONT_RAJDHANI_MEDIUM` was a *workaround*,
    not the cause: no binary loaded `Rajdhani-SemiBold.ttf`, and asking
    the shaper for weight 600 with only 400/500/700 loaded returns
    **Bold** (CSS matching climbs from 600). So the bar strip and every
    `Face::SemiBold` label were a stop off in one direction or the
    other. Now `fonts::FONT_RAJDHANI_SEMIBOLD`, the file loaded by all
    six binaries, `Face::SemiBold` and `bar::era_face` map to it, and
    `Run::semibold()` gives the mail body the same. neokitsch's mailbox
    body line widths now match the design; kitsch's dashboard header /
    fan labels and neomil's module labels sit at the trace's weight.
  - **Letter-spacing** is `Prim::Tracked` (`Prim::Text` plus
    `tracking`, drawn glyph by glyph on shaper-measured advances, the
    anchor applied to the tracked run; `scene::advances` is login's
    prefix-measure, moved so both use one). Applied where the
    transcription had recorded the value: kitsch's blade / PRODUCTS
    labels (`letter-spacing="2"`) and neomil's six module labels
    (1.2). The traces use `letter-spacing` ~180 times; the rest is
    per-era transcription work (neokitsch header 1.5 / LEVEL 2 /
    annotations 0.4, neomil header and tabs, every store), not a
    renderer limit any more.
  - Rotated text landed earlier (`Prim::Text` `rotate`); kitsch's blade
    labels are on the blades.
  - Goldens moved: bar.neokitsch, all four dashboards, mailbox.neokitsch
    (re-taken by eye against the G2i design renders; login untouched).
    G2i is blind to weight and tracking — entropism 98, kitsch 69 (was
    68), neomil 96, neokitsch 94, neokitsch mailbox 84, all unchanged —
    so these are by-eye verifications only.
  - Still open, small: no stroked text (neomil's outlined `next`
    logotype is drawn filled); `Wide` is start-anchored only (centred
    stretched glyphs are placed by hand at `cx - run/2`). Gradient
    masks are `Prim::Masked` since later the same day (the "four
    grounds" item above); neokitsch's `#bluemask` landed as `BLUE_MASK`
    under the neokitsch backdrops item, done the same day.
- [x] Follow-ups the era agents flagged and did not touch. The neomil
  maker's-mark one (`components.svg:1113` "an M of 89x39", trace path 46
  wide) was mis-diagnosed: re-measured on `img-07` at 2.4x on
  2026-09-04, the mark is not an M and the "89x8 bar" under it is not
  a bar. It is a stencil-cut M — both stems lean in from the top, the
  left one drops a shoulder to x 1220, the right one is notched
  diagonally around a detached ~11px square dot at (1273,715) — with
  bbox x 1220..1287, y 682..725, and beneath it two centred 8px lines
  of micro-text, PRECISION LIQUID / POLYMER MUSCLE, at y 737.5 / 746.
  Trace, `components.svg` (prose, translated copy and the caption) and
  `neomil.rs` `MAKER_MARK` + DASHBOARD prims now carry that geometry;
  G2i pixel diff 0.32%, golden dashboard-neomil re-taken -- but not
  `dashboard-fallback`, which is the same neomil dashboard under the
  crate's compiled palette: it sat at 99.951% (the old M in an 89x64
  box at (1208,682)) through every run until 2026-09-05, passing the
  99.9 gate. Re-taken then; the wrapper TODO's rule stands, a golden
  that passes at 99.9-something is drifting. The design
  inventory now splits the mark into two "chamfer" stems that the
  implementation's single blob does not pair with (44/53 matched,
  98% area, still PASS) — an artefact of the inventory's ink
  quantisation, not a drawing gap; the crops agree by eye. The other
  two were doc rot, settled
  2026-09-04: neokitsch `STRATA`'s comment now says its one reader is
  the bar's mail example panel through `Chrome::DeviceFrame`, not the
  dashboard; kitsch `style.ticket`'s comment (`kitsch.rs:343-355`)
  already recorded that nothing reads it and why `Ticket` cannot
  describe the trace's `#nav` chevron — nothing to change there, and
  the widget-vs-canvas item below carries the decision.

### Trace improvements (2026-09-02)

All of the above landed in `a0a9274` (2026-09-02); the third vision
pass below and the conversion wave landed together on 2026-09-03.

All 16 era × screen traces exist and every one was checked by eye
against its source (`images/compare/<era>-<screen>-{trace,overlay}.png`,
regenerate with rsvg-convert + `magick`). Gate results are in
`docs/sources.md` § G1. **Every item here needs a vision-capable
model** (standing rule above).

**Second pass, same day** — one vision agent per era, file-disjoint on
`docs/<era>/`, every claim re-gated and overlaid by the orchestrator
before ticking. All 16 now PASS. What the pass found is as useful as
what it fixed: **five of the twelve first-pass items below had a false
premise**, each one written from memory of the overlay
rather than from a measurement. Corrected in place rather than deleted,
so the next list-writer sees the pattern: measure before you file.

Gate fails — all cleared:

- [x] **`neomil/store-trace.svg` — FAIL 55% → PASS 71%.** Rifles
  redrawn as four layered symbols (body / hatched forend / highlights /
  detail lines) measured on a 10px grid; card 2's is solid and 14px
  further left as in the photo. Kanji is real Noto CJK text on the
  slanted band. Card 4 ends in a plain cut at x 1557 — the chamfer and
  right-edge bar the trace had there were invented. Premises corrected:
  there are four rifle drawings, not two; only card 4 ever had a
  clipPath, so there was nothing to check on the other three.
- [x] **`entropism/mailbox-trace.svg` — FAIL 60% → PASS 88%.** Every
  outlined frame in the material photographs as 2px stroke + 1px
  near-black undershoot + faint sage overshoot 2–4px out; drawn as
  three concentric strokes per section and labelled in the header as a
  photographic halo, not a designed glow, so the iced side keeps the
  README's "no glow" rule. Envelope glyphs rebuilt from 10x crops.
  Premise corrected: the extractor's "second fit of the frame" was a
  text-halo blob spanning rows 2..7, not an outer ring; the ring is
  still real, and the gate now pairs the two by bbox coincidence.
- [x] **`neokitsch/mailbox-trace.svg` — FAIL 0.61 → PASS 0.68, all
  four source families paired.** The source's gold is three-tiered
  (bright bar/tabs, mid text cores, dark outlines) and the trace was
  one flat bright gold — so k-means never spent a centroid on the dark
  tier. Selection bar is now wood veneer (32 fine strokes, seam at x
  273), RIFLES outlines dark gold, list titles mid gold.

Things a viewer notices — all done:

- [x] **`neomil/dashboard-trace.svg` detail panel** — premise
  corrected: the source chamfers only TR (8px) and BL (42px); TL and
  BR are square, and the old trace's 14px TL chamfer was the real
  error. Redrawn from row/column scans, bright right-edge bar and
  glitch hairlines added, invented scrollbar rail removed. 94% → 94%.
- [x] **`entropism/dashboard-trace.svg` footer** — premise corrected:
  only BUILD is right-anchored (at 1525); PROVIDED BY is left-anchored
  at 519 exactly as on the mailbox, and the trace had it 180px too far
  right. 92% → 92%.
- [x] **Kitsch ghost cards** — premise **false**: measured in each
  card's own frame, every ghost has the same 162px long axis as the
  solid card. The real errors were the card itself (drawn 190x60 r14,
  photo 162x50 r8), the ghost step (+20,−20 in screen space for every
  stack), the counts (6/7/6/6/5/6) and two hub centres. Also the
  footer had been drawn yellow; it is mint on all four kitsch screens.
  0.54 → 0.59. `store-trace.svg` has no ghost stacks at all.
- [x] **Neokitsch halo ported into `dashboard-trace.svg`** verbatim
  from login. 0.54 → 0.60. The glow family's centroid moved +221px
  (halo is now everywhere content is; the source's glow weights left)
  — reported, not tuned.
- [x] **Font pinning** — `fidelity_check.sh` now writes a private
  `fonts.conf` (system config + `fonts/`) and exports
  `FONTCONFIG_FILE`. Premise half-wrong: the neokitsch "4S T" logotype
  gap persists with fonts pinned, so it is glyph spacing in the SVG
  (see below), not fallback.

Texture — done where cheap:

- [x] Rifles: neomil store (line art, above); kitsch store (silhouette
  from the mint-mask column profiles with evenodd holes for port,
  magazine well and trigger guard; card 2's gun sits 14px higher).
- [x] ~~QR glyphs (neomil login, entropism login)~~ — premise false
  twice: **neither login has a QR**. The neomil store's socket glyph
  is a 24-cell noise scatter (was drawn as a finder-square code — the
  schematic was the fabrication); the entropism store's is a 9x9 dot
  matrix with the middle row and column empty. Both now traced as
  measured.
- [x] Kitsch store band glyphs redrawn as distinct marks (cert square,
  disc-in-square, C-in-C, warning triangle, micro-text rules) at
  measured positions — they were also 5px too high.
- [x] Neokitsch wood grain on SMG, card 2 and basket (~2.1px pitch,
  seams at SMG x 192.5 / basket x 1374, card 2's fan-in stripe at y
  531). Gate 0.63 → 0.60 because k-means re-partitioned the glow
  family; accepted — overlay is closer. Revert is
  `/tmp/store-trace.svg.bak` if the number is preferred.
- [x] Entropism envelope glyphs (above); kitsch login chip glyph
  (hexagon, slash, wedge, bottom marks from a 6x zoom). Kitsch login
  also got its barcode regenerated from the column profile (50 bars;
  the old list merged neighbours) and the bracket at the measured
  1.3px. 0.47 → 0.54. Kitsch mailbox: body text to the measured line
  extents, outline 1.25px on .5 coordinates. 0.46 → 0.62.
- [x] Bar tray icons, all four `bar.svg`: they were `#8800aa` /
  `#ff6600` everywhere. Now era ink for idle, era alert colour for
  attention (neomil #de2e2e/#ff3b45, entropism #94bb94/#728f76,
  kitsch #7ddec8/#fcc428, neokitsch #e7c686/#fcc474). G2 will report
  the drift against the goldens on exactly those diamonds — expected;
  the goldens follow when the Rust bar is restyled.

Third pass, 2026-09-03 — one vision agent each for entropism, kitsch
and neokitsch (neomil had no pass; its items below stay open). G1i
after: kitsch login .54→.63, dashboard .59→.69, mailbox .62→.67, store
.57→.73; neokitsch login .73→.77, mailbox .68→.73, store .60→.66,
dashboard byte-identical; entropism dashboard PASS re-run. Two tool
facts the pass established, both verified: **rsvg 2.62.3 ignores
`textLength`** (every such attribute in the traces was a no-op; all
removed) **and per-glyph `x` lists** (`x="a b c"`) — fit text with
`letter-spacing` or `scale(sx,1)`, one `<text>` per glyph where the
gap must be exact.

- [x] **Neokitsch "4S T" logotype gap** — 4/S/T are per-glyph scaled
  texts now, STORE five texts, "MAGNUM 650" fitted by
  letter-spacing/scale (kitsch likewise).
- [x] **Entropism `dashboard-trace.svg` header strip** — premise
  half-wrong: measured 17px weight 500 at natural tracking, x 61 / 518
  / 1382, baseline 60 (not 15px). Applied to all four entropism traces
  and their footers.
- [x] ~~**Entropism halo consistency**~~ — premise false: the
  "three-stroke edge profile" is a Lanczos undershoot from
  `extract_spec.py`'s rescale (line ~356), not ink. Not ported; the
  mailbox header note corrected. Line widths: the entropism 2px
  outlines were rescale dilution too — 1.25px measured core (#8fba97
  hub, #75967b login, #93bd95 store). Kitsch strokes 1.5/2 → 1.25 on
  .5 coordinates across all four, badges 1.0; footers bold 700 at x
  503/641, footnotes size 8 scale(1.3) bright #82f0d3 (were dim).
- [x] **Neokitsch mailbox halo** — split into `#lines`/`#text` with
  separate filters; and see the `class="photo"` gate change below,
  which takes the halo out of G2i entirely.
- [x] Neokitsch violet haze retuned (+`#hazelobe`); widths fitted by
  letter-spacing/scale; QR glyphs 9x9; header micro block re-anchored
  left at x 843.3; grain stroke #e6ae6b → #cf975c. Seen, not fixed:
  gun silhouettes coarse, DPS labels small, text weight lighter than
  600 throughout.
- [x] Neomil photo details, triaged 2026-09-04 against the photos:
  - Store card 4's ghosted/duplicated title and the dashboard's extra
    glitch fragments are the photograph's treatment (`docs/PIPELINE.md`);
    the store trace's own comment calls the ghost "not drawn". Left.
  - The mailbox "QR-like glyphs beside the panel" are the boxed
    PETROCHEM / BETTERLIFE TEC maker's marks in the panel's top-right
    corner, and the trace had them wrong: the 0.8 box sat *beside*
    the text (x 1424..1432) where the photo draws it *around* PETROCHEM
    (img-08-main.png x 3431..3447, y 812..922 → design 1429.6..1436.25,
    338.3..384.2; BETTERLIFE TEC at 1440..1444.2). Same construct on
    the store cards (box 36..55 photo px in from the card's right edge
    → local 261..269 x 182.5..231.7, glyphs 7.5 not 7). Fixed in
    `mailbox-trace.svg`, `store-trace.svg`, `components.svg`, the
    store `card!` macro and `CARD2`'s `line_rect`.
  - Found on the way: the mailbox panel's `#1c0608` fill (added
    2026-09-04) was drawn *after* CHROME and buried the inner half of
    the bright bar riding its right edge (`EDGE_BAR`, x 1440..1455;
    only 1450..1455 survived). New `Mailbox::overlay` layer drawn after
    the four regions; neomil's holds the bar and the maker's-mark box,
    the other three eras' are empty. Goldens mailbox/store-neomil
    re-taken.
  - Not changed: the photo's panel outline and marks are 1-2 px of
    bright (251,53,53) at 2.4x, which integrates to about the trace's
    dim `#a8282b` at 1.2 / 0.8 — the same ink trade the neokitsch
    strands make.
- [x] Entropism mailbox T1–T4: bold 27 + scale(1.42..1.46, 1).
- [x] Entropism `bar.svg` menu icons overlap the row text (icon to
  1177, text at 1175) — stale by 2026-09-04: the current `bar.svg`
  reserves a 16px icon column (glyph +12..+28, label at +36; Mount's
  diamond 1153..1169, text 1177) and its own prose (`:168-170`) calls
  that "the fix for the old icon/text overlap". The iced bar matches
  (G2i entropism bar 5/5 shapes; example icons are the synthetic
  magenta `sample_icon`, by design).
- [x] Kitsch store card 2's band: 1.1px #a4583a outline path with a
  45° chamfered right end; card 1's chamfer angle fixed; PETROCHEM box
  at (163, 72.8, 58x8), PETROCHEM/BETTERLIFE scale(1.3, 1). Left: card
  4's right-edge vignette and the fan-card strokes are unmeasured;
  weights are by eye.
- [ ] Kitsch mailbox yellow family sits at 0.46 IoU because the
  extractor hole-fills the photo's closed panel outline into a solid
  block while the render's antialiased rounded corners break the ring;
  not a trace fault, do not square the corners. Verified again in the
  third pass, do not fix.
- [x] **Entropism OUTLINE ink — decided 2026-09-05: repoint `border`
  to #8fba97.** The real outline ink measures ~#8fba97, i.e. the same
  sage as the text (#94bb94); the earlier ~#709174 was rescale dilution
  and the old OUTLINE #5d7752 was far from both. Decided by the user
  off the side-by-sides (store and bar rendered at `border` #5d7752 and
  #8fba97 against the photo): the photo's frames are unmistakably the
  bright sage, and #5d7752 was the only one of the three where the nav
  and card frames receded into the ground. Applied to nexus `border`
  in `home/themes/entropism/palettes.nix` and to the crate's `OUTLINE`
  const (the same value `HUB_STROKE` was measured to), the era README
  and the comments that cited #5d7752. Two `Ink::Border` readers had
  chosen it for its *dimness*, not as a frame, and went to `Ink::Mid`
  so the meaning survives: the bar menu's `disabled` ink (at #8fba97 a
  disabled row would read as enabled) and the store's 8.5px footnote
  caption (the mailbox already drew the same caption in MID). Frames,
  menu rules and the bar chrome brighten as intended — the bar is
  where it changes character most, and is worth a look on the real
  monitor once entropism is live; terra is on `cybr`, so nothing on
  the desktop moved. Goldens: only bar and store moved (99.25% and 99.55%
  against the old ones, both re-taken); dashboard, login and mailbox
  came out byte-identical, which is the follow-up below stated as a
  measurement -- none of the three reads the role. Matrix 21/21.
  The follow-up landed the same day: login's four `Ink::Fixed`s
  (#75967b / #8aac8c / #20281c / #799d81), the mailbox's nine `Ink::Mid`
  outlines and dividers, and the hub's `HUB_STROKE`/`HUB_SOLID` now
  read `Ink::Border` / `Ink::Fg` / `Ink::Select` / `Ink::Cta` /
  `Ink::OnSelect` — the mapping the login block's own comment gave;
  the probe values stay in the block as history so nobody re-measures.
  The dashboard came out byte-identical (`HUB_STROKE` already was the
  role's value). Doing it surfaced a regression the OUTLINE change had
  made unnoticed the day before: G2i store had gone 32/32 → 19/32 FAIL
  at `border` #8fba97, because the k=8 extractor merges inks within
  ~13 levels and the brightened frames fell into the #9cb795 selection
  fills, so the filled rects lost their edges (confirmed by flipping
  `border` back — 32/32 — and restoring it). Resolved by the user's
  call, off the side-by-sides, to take `select`/`cta` (`SAGE_SOLID`) to
  #a6d3a7, the fill the three traces agree on (store #a8d4a2, mailbox
  #a6d2a8, hub #a6d3a7; fills do not dilute in the rescale the way
  1.25px strokes do): store back to 32/32, mailbox 15/15 → 11/15 →
  14/15, and the active workspace on the bar again stands off the
  `nomad` tape. Goldens bar/login/mailbox/store-entropism re-taken
  (99.77 / 98.41 / 99.28 / 99.53 against the previous); matrix 21/21.
  `store-trace.svg` strokes its frames in #93bd95 — the trace and the
  role still differ by a hair; a vision-model question, not a code one.

Housekeeping (the two deletions below landed 2026-09-03; nothing left
here is blocked):

- [x] **Delete `docs/<era>/target-app.svg` ×4** — done 2026-09-03,
  together with `neomil/target-components.svg` (sampled nothing from its
  run), `neomil/comparison.html` and `neomil/golden/` (one-off agent
  artefacts). `docs/sources.md` keeps a struck-through row per deleted
  file recording what was wrong with it. The doc comments that cited
  them by coordinate were *not* moved to the traces — the numbers they
  cite are the numbers the code was built to, and the traces disagree
  with several of them (kitsch `Ticket`/`Banner`, neokitsch
  `DeviceFrame`, the neomil table). Each citation is annotated "deleted
  2026-09-03" with what the trace says instead, so the disagreement is
  visible at the call site rather than papered over.
- [x] **`docs/<era>/dashboard.svg` ×4** — deleted 2026-09-03 rather than
  waiting on the `Layout` decision: they were the app's own layout
  drawn back into SVG, so keeping them as "designs" let G2i report a
  PASS for entropism/neokitsch that only meant the screen matched a
  drawing of itself. `fidelity_check.sh` now resolves every screen to
  `<screen>-trace.svg` (`bar.svg` for the bar), G2i and G1i on the same
  file. **Honest dashboard G2i baseline against the traces, all four
  eras: 0% matched area, FAIL** (captures verified non-blank; the
  side-by-sides in `/tmp/g2i-<era>-dashboard/` show two different
  compositions). Nothing to fix here — this is the `Layout` item's
  number until the screen is rebuilt from the traces. Side effect:
  `--source` now runs four screens per era instead of one (the "script
  gap" noted under the wave findings is closed).

### Component sheets (2026-09-03) — `docs/<era>/components.svg`

The three surviving `target-components.svg` (drawn by eye before the
traces existed, inheriting `target-app.svg`'s errors) were renamed to
`components.svg` and rebuilt, and neomil got one too, so all four eras
have a widget sheet. One vision agent per era, file-disjoint, brief in
`/tmp/comp/BRIEF.md` at the time: every component is a **translate-only
copy of a trace element** (`bar.svg`'s citations count as pointers back
into the traces), with an XML comment naming file and coordinates and
an on-sheet caption of the numbers, so a coding model can read the
sheet as a text spec. Each sheet ends with a sampled palette,
typography, ground stops, observed era rules and an **IMPLEMENTATION
DELTA** box listing where `src/eras/<era>.rs` still disagrees with the
traces. Renders viewed at 1920 wide, no rsvg warnings, no `--` in
comments. The sheets are derived from the traces and are **not gated**;
`fidelity_check.sh` never reads them.

Findings the rebuild surfaced; the follow-up wave (same day, four
file-disjoint agents, every claim spot-checked against the files and
renders) resolved them as follows:

- [x] **Trace prose disagrees with trace path** — resolved against the
  photos, not against either text, all five gates re-run and PASS
  (neomil login 79% / dashboard 94% / mailbox 86% unchanged; neokitsch
  mailbox 0.73 unchanged, dashboard 0.60 → 0.67). The item as written
  understated two of the five: it framed the neomil login chamfer as
  46-vs-47 when the photo reads 50.8..52.1 — *both* prose and paths were
  ~5px short (now 51, notch `M 0,390 L 15,405 V 483 L 0,496`, ends cut
  15 top / 13 bottom); and the neokitsch `#ncard` was not "91 vs 93" but
  an 83 body plus a 10 tab well with a 32 chamfer where the photo has
  91.7x328, chamfer 42.5, one 45° diagonal across the whole foot and
  the six onion rings nested *inside* the card, not offset outward (new
  `#ncard`/`#ncardsel`/`#nring1..6`). The neokitsch mailbox notch exists
  in the photo and is now drawn (gold tab to y 372 under a dark
  trapezoid to y 366). Neomil dashboard 104 and mailbox "row 1 is 70
  tall, then 68 on a 70 pitch" were prose-only fixes; the paths were
  right. `components.svg` copies and captions updated for both eras;
  `sources.md` rows and G1 tables too. Renders/overlays were in
  `/tmp/followup/nm/` and `/tmp/nkfix/` (gone on reboot).
- [x] **Neomil dashboard diamonds are truncated at their outward tips**
  — stale when re-read on 2026-09-04: the trace fix had already landed
  in the 2026-09-03 conversion wave ("two trace fixes first" above).
  `dashboard-trace.svg:27-36` and `:107-113` now say EVERY OUTWARD TIP
  IS CUT FLAT at 89 (inner outline at 59) and the defs are two explicit
  variants, `#cell-up` / `#cell-down`; `components.svg:113-130` mirrors
  them and `neomil.rs` `CELL_UP` / `CELL_DOWN` (`:1660-1695`) transcribe
  the same points. Checked on the G2i render: D1's top reads a ~32px
  plateau at y 369..375, not a point. Original measurement kept for the
  record: photo plateaus 30px at y 370.8 and 28..29px at y 682.9,
  i.e. a 104 half-diagonal cut 15 short.
- [x] **Neokitsch dashboard detail panel geometry** — stale when
  re-read on 2026-09-04: every sub-claim landed in the 2026-09-03
  conversion wave. `dashboard-trace.svg:42-58` and the `#npanel` def
  (`:160-182`) place the panel at (1170.8,259.7), 230.4x465.3, with the
  stepped shoulder at local y 30.3, the S-curve to x 110 and r10/r7.5
  corners; the rings are FOUR nested inside (`#npring1..4`, `:183-200`);
  the card labels sit at their measured baselines (`:416-425`, MATRIX
  366.3); the front stroke is 1.2 `#e8ab66` (`:384-388`, with the
  reasoning for not going to 1.0). `neokitsch.rs` mirrors it: `NPANEL`
  (`:2115-2174`), `HUB_EDGE = rgb(0xe8ab66)` (`:1935`), MATRIX at
  `txt_end(338.0, 366.3, …)` (`:2402`). Original measurements kept for
  the record: photo top 259.8, left 1170.8, shoulder y 290, chamfer 30
  wide, ring top lines 623.5 / 630.5 / 638 at 3840, front stroke
  1.25 / (251,194,98).
- [x] **Small trace prose/path residue** — worked through 2026-09-04,
  each re-measured on its photo before touching the file; only the
  `bar.svg` note at the end stays open:
  - neomil `#chip`: the photo squares are 12.5x12.9 (login), 12.9
    (mailbox), 12.5x12.1 (store), never 14 — the *def* was wrong, and
    the login prose's right chip "1542..1556" too (photo 1540.8). Def
    is now a 12.5 square at (0,1) in all four files, login's right use
    at x 1541, mailbox's left at 61, store's at 62 (photo 61.25 /
    62.5). `neomil.rs` login `chips` are 12.5 at y 347 and the store
    `CHIP` prim matches; `screens/login.rs` `Fixture::Margins` now
    places ticks, dot and the right-margin arrow from the square, so
    the trace's absolute arrow (1548,410) still lands.
  - neomil dashboard footer tape "third cell ends 1354 vs photo
    1358.8": not a defect — `dashboard-trace.svg:263-269` already says
    the 1358 is the dim echo of the frame 3px right and down.
  - neomil mailbox scroll widget: the photo's two arrows are THIN
    (shafts 1.5, heads 4.7) and both span y 692.7..714.6, not 10-wide
    heads staggered 687..715. Trace, `components.svg` and
    `neomil.rs` `SCROLL_UP`/`SCROLL_DOWN` redrawn; the ring and the
    "R" the iced mailbox never drew are now `SCROLL_RING` (four
    cubics) + a `strong` "R" (`CHROME` 38 -> 42 pieces). Fourth button:
    photo left edges 728.75 / 908.3 / ~1088 / 1267.1 — the trace's
    1267 is the photo, "5 apart" is ±1; left.
  - neomil login card 1: photo half-maximum edges 315.4 / 570.2; the
    313..572 included the glow. Block now y 315..570 (trace,
    `components.svg`, `neomil.rs` `Plot::new(372,315,253,255)`), and
    the era table's `Bevel::tr(46)` went to the trace's 51 at the same
    time (the "fix with the login table, not alone" note under the
    gate-artefacts item).
  - entropism `store-trace.svg` 4ST: photo glyphs x 137.5..305,
    y 101..152.5 — the paths (138..308, 102..151) were right and the
    prose's "y 102..160" wrong; prose corrected.
  - neokitsch `mailbox-trace.svg` RIFLES: local comment updated to the
    header's #ecbe82 1.25; photo left edges 734.6 / 929.2 / 1119.6 /
    1310.0 (steps 194, 191, 190), second use moved +192 -> +194, the
    implementation's uniform `dx: 192` documented as within 2px.
  - neokitsch `login-trace.svg` last two strands: their r 8 curl ran
    past y 812 by 1.1 / 5px. Tighter curls (r 6.9 / r 3) in the trace
    and the same clamp (`curl = CURL.min(end - oy)`) in
    `screens/login.rs` `wire_band`; golden login-neokitsch re-taken
    (27 px differ).
  - **`neokitsch/bar.svg` §10 chose its menu-panel geometry against
    the old `#ncard` numbers** — revisited 2026-09-04. Of the three
    things §10 took from the old card, one survives and two did not:
    the 22 chamfer stands, but on the mailbox selection bar's evidence
    alone (§10 also credited the detail panel, which the 2026-09-03
    re-measure found has *no* chamfer); the rings went from four
    offset outward at 3.5 to four nested inside at the detail panel's
    3.2, sharing the left edge and the bottom-left chamfer, fading
    .7 .7 .55 .25, exactly as `#nring1..6` / `#npring1..4` nest. Rows
    sit inside the innermost ring (root panel 31..180.6, was ..155).
    `bar.rs`: rings drawn by the panel's own canvas (`Panel::ring`),
    `BarMenu::ring_inset()` pads the rows; `ChainEcho`, `echo_pad`
    and `menu_edge_pad` deleted — nothing is drawn outside a panel any
    more, so the chain is the panels and the harness margin is a plain
    120. G2i neokitsch bar (accepted FAIL) 52% → 14%: the extractor
    now fuses the tray strip and both menus into one `blob-01`
    [1094,3,443,116] on the impl side; by eye the crops match. Golden
    bar-neokitsch re-taken.
- [x] **`src/eras/*.rs` vs traces — triaged, not applied.** Every
  delta-box line is classified in **`ERAS-DELTA.md`** by consumer:
  (a) read by a gated screen, (b) dashboard/bar only, (c) dead, (d) doc
  only; counts a/b/c/d entropism 5/2/0/3, kitsch 1/4/3/3, neokitsch
  6/5/0/3, neomil 2/4/1/1. Applied: class (d) doc corrections and class
  (c) "unconsumed as of 2026-09-03; trace value would be X" annotations
  in all four era files — comments only, `cargo build` all bins and
  `cargo test --no-run` clean. No value changed. Two things the triage
  established that change how to read the boxes:
  - **Gated renders overlay `home/themes/<era>/palettes.nix` via
    `Palette::with_roles`** (`src/palette.rs:138`, `default.nix`,
    `tests/visual.nix`, the examples): `bg/panel/border/dim/fg/alert/
    tape` and the declared extras come from nix, and only `select`,
    `on_select`, `cta`, `bloom`, `banner_selected` survive from the era
    table. So entropism `OUTLINE`/`SAGE_TEXT`, neokitsch
    `FRAME`/`STRATA`/`FIELD`/`GOLD_TEXT`, neomil `RED_MID`/`RED_DEEP`/
    `OFF_WHITE`, kitsch `SLAB`/`BEZEL` are inert in every golden unless
    nix moves too. The OUTLINE decision above is therefore a *nix*
    change as much as a Rust one.
  - **Kitsch `Ticket`, `Banner`, `banner_selected` are dead**: their
    only readers (`widgets/pill.rs` `pill()`, `widgets/banner.rs:97`,
    `widgets/card.rs:246`) have no callers in `src/`, `examples/` or
    `tests/`. Candidates for deletion in the toolkit-infrastructure
    pass, not here.
  - Wrong in the boxes/this item as written: `YELLOW_SHADE`'s doc
    claimed `store-trace.svg` has `#f0a80a` — it is in no trace (the
    grown card fills `#ffc233`, the band `#fec32f`); the neomil box's
    "band #2a3a51 to #101f3d" is the code's own BAND consts, not a
    measurement (the traces' glow is the `glowh` gradient); the module
    docs' Behance run numbers were ten low in all four eras (`sources.md`
    canonical positions), not just kitsch/neokitsch — fixed in all four.
  - "Ready to apply once the Layout decision lands" (15 rows) and "Would
    change a passing golden" (14 rows, goldens named) are the two
    closing sections of `ERAS-DELTA.md`.
- [x] **`docs/sources.md` trace rows had the wrong numbers** — first
  the three neomil rows (dashboard panel, login card x-ranges/chamfer/
  notch, mailbox panel and button geometry — written from memory, not
  the file), then an audit of the other 13 trace rows and 4 bar rows:
  **9 of 13 wrong somewhere**, mostly gate numbers superseded by the
  traces' own 2026-09-03 pass notes (kitsch 0.63/0.69/0.67/0.73,
  neokitsch login 0.77 / store 0.66) and "still open" notes already
  closed, plus off-by-one-to-four coordinates (519→518, 403→402,
  164→165, 155→151, 350→354, 93..278→92..272) and two wrong
  descriptions (kitsch ghosts "−50°" vs −45°; neokitsch rings "stepping
  up-left" vs symmetric, and on unselected cards only). The four bar
  rows and four trace rows were right. Every fix is marked "corrected
  2026-09-03"; the G1 tables were brought to the same numbers. Same
  pattern as the five false-premise items under Trace improvements:
  measure before you file.

### Bar restyle (2026-09-02) — SVG done; `bar.rs` followed 2026-09-03

The four `docs/<era>/bar.svg` were one skeleton with a palette swap.
Each was redrawn by a vision agent from its era's four verified
traces — the bar has no photo source, so the traces' chrome is the
only legitimate vocabulary — with every decision cited by file and
coordinate in the header, and an **IMPLEMENTATION DELTA** block at the
foot written for a coding model (no images needed). Rendered and
checked at 1x and 2x: legible, no overlaps, palette-only. G2 drift is
the point, not a regression: neomil 0.989/0.890/0.966 →
0.741/0.729/0.694, entropism 0.986/0.904/0.942 → 0.843/0.667/0.942,
kitsch 0.973/0.893/0.956 → 0.934/0.860/0.923, neokitsch
0.981/0.904/0.937 → 0.554/0.782/0.747.

- [x] neomil: the SECURITY LEVEL badge row — bottom-**left** chamfer 6
  on 25px cells (no neomil cell cuts BR; the old bar did), RED_DEEP
  fill / RED_FILL 1.5 stroke, bold digits, BAND_TOP→BAND_BOTTOM glow
  band with the 1.5px RED_MID rule under it, barcode host tape, GO HOME
  panel's right-edge bar + echoes on the menu, arrow submenu marker.
- [x] entropism: the bar *is* the header strip — one 2px outlined frame
  x 6..1594 with 2px dividers, no cell gaps, one filled segment per run
  (workspace 3, clock), solid MID tape with ON_SOLID ink, alert = the
  source's `(!)` suffix with no ink change, disabled = OUTLINE ink.
- [x] kitsch: chevron workspaces (store nav, 40 wide on 46 pitch),
  stepped USER/DESCRIPTION boxes for tape and window label, r8 chips
  at 1.25px on .5 coordinates, weight 500, mint bracket fading along
  the bar foot (opacity 1 to x 264, 0 at 440), teal wave in the menu
  foot, chamfered two-piece selected rows.
- [x] neokitsch: violet haze ground clipped to y 0..31, r3 cells with a
  10x7 BL cut and a 22/16/4 VENEER_LIGHT tab, veneer + grain + seam for
  selection and tape, solid AMBER square alert plate, 8-strand wire
  band bridging the centre gap, unboxed 18px clock, chamfer-22 menu
  cards with four onion rings clipped at y 31.

Open:

- [x] **Decide the bar's corner before touching `bar.rs`.** Decided
  2026-09-02: per-corner cuts, see "SVG→iced pre-work". Three of
  four deltas want a corner the era's screens do not use (neomil BL
  chamfer vs `Corner::Chamfer` cutting one fixed corner; neokitsch r3+BL
  cut vs `ClipTopRight 30`; kitsch r8 unclamped vs pill). Either `Bar`
  in `style.rs` grows its own corner field, or `Corner` learns per-side
  cuts — the neomil agent notes the material cuts different corners per
  widget on every screen (badges BL, login cards TR 46 + BL 22, buttons
  BR 9, panels TR 8 + BL), so the second is the honest fix and affects
  every neomil screen, not just the bar.
- [ ] **Neomil idle label contrast**: RED_FILL on RED_DEEP is faithful
  to the badges but the dimmest strip of the four at 14px. Lift the
  idle ink (OFF_WHITE or RED_HOT) if it is hard to read on the real
  monitor; the SVG is the place to decide, then the delta.
- [x] **`bar.rs` follows the four deltas** — landed 2026-09-03:
  `bar.rs` rewritten against `Style::bar` tables (`Dress`, `BarGround`,
  `BarChrome`, `BarOrnament`, `WindowLabel`, `Tab`, `MenuRule`,
  `MenuMarker`, `PanelEcho`, `BarMenu`), goldens re-taken. G2i:
  entropism 100%, neomil 86%, kitsch 83%, **neokitsch 52% FAIL,
  accepted** — the extractor fragments the veneer plates and ring bands
  into cells the design does not have; measured ceiling 58.5% with the
  trace's own 52px-period zigzag grain drawn (the period is what seeds
  the extractor's cells; documented in `bar.rs`). Neokitsch labels were
  `Face::Medium` (Regular was visibly thinner than the design's
  dark-on-gold; decided by eye) until 2026-09-04, when the 600 file was
  embedded and they became the trace's `SemiBold`. Neokitsch
  stroke is 2.0 not the delta's 1.6 (AA). The tray diamonds render
  purple/orange because the example feeds sample SNI raster icons; the
  SVG's diamonds are stand-ins, not a fault. **The layer-shell bar is
  unverified until a switch**; only `bar-window` is captured.
  - [x] "Not followed, per era" — six deviations, filed 2026-09-03 as
    each needing "a widget, not a value". Spot-checked 2026-09-05: five
    of the six were values or a shape the existing widget could carry,
    and the sixth (the haze blue annulus) wanted the haze unification
    rather than a widget. All six closed by 2026-09-05; the entries
    stay for the reasoning.
    - Entropism selection inset 1px — *fixed*, and it was worse than an
      inset: the module faces were drawn flush to their cells over the
      strip's chrome, burying the inner half of every divider next to a
      filled cell and the frame's whole top and bottom edges under the
      tape, workspace 3 and the clock. Two changes in `bar.rs`: under
      `BarChrome::Frame` the module row is laid out from the frame's
      centreline (`frame_edge`, half a stroke inside the padding, which
      is also where the design measures its segments — tape 7..72, not
      6..71) and each face is inset half a stroke (`plate`). Pixel
      transitions now match the design across the left run.
    - Neomil window box 1.0 stroke — *fixed*: `WindowLabel::stroke:
      Option<f32>`, threaded through `face_canvas`.
    - Neokitsch window label at 400 — *fixed*: `WindowLabel::face:
      Option<Face>`; also the clock, which `bar.svg` §7 sets at 500 and
      the code set in the strip's 600: `clock_plain` is now
      `Option<(u16, Face)>` and the digits sit against the right of
      their reservation (the design right-aligns to 1594; the estimate
      ran ~10px wide of the face's advance).
    - Kitsch chevron shoulder — *fixed* with a new `Cut::Peak { x, y,
      brow }` in `widgets/surface.rs`: the rising edge to a peak, a
      drop onto a brow, and the top edge running *below* the box's top
      from there. `outline` and `span_at` both honour it (the top-right
      corner bites from the brow); top-left only, elsewhere it is the
      chamfer. Unit-tested against `#chev` scaled 25/46; the 13-of-25
      rise trips `extent`'s half-height clamp and is squeezed 12.5/13,
      under half a pixel.
    - Neokitsch highlight row 8px past the panel edge — *fixed*
      2026-09-05, and the premise recorded here was wrong. It did
      **not** need the panel canvas to draw the highlight, and no hover
      state had to reach `Panel`; that reading assumed the row's box
      was the panel's inner width and could not be anything else. The
      fix went the other way round: the rows' boxes were widened past
      the panel's outline and the panel was told to leave that strip
      undrawn. `BarMenu::row_overshoot` (neokitsch 8, zero elsewhere);
      `menu_panel` widens the root container by it, gives the rows'
      column back the `ring_inset()` it padded on the right, and hands
      each row the sum as `edge`; `menu_row` pads its content and
      insets the open face and the separator rule by `edge` again, so
      the highlight is the only thing that moves; `Panel::draw`
      subtracts `overshoot` from its width so its outline stays put.
      `bar::menu_overshoot` is the host's share — `bar-window`
      subtracts it from `MENU_MARGIN`, so the panel still ends on the
      design's x=1480 and the plate runs to 1488.
      Measured on the re-taken golden: the plate's last veneer pixel is
      1486 against a panel outline at 1478..1479 — the design's 8, off
      the outline the chain actually lands on. Only neokitsch sets
      `PanelEcho::Rings`, so `edge` is 0 in the other three and no
      other era's geometry moves; against the old golden the whole
      diff is one 121x26 box at (1366,111) — the row's right end plus
      the veneer seam and grain hairlines that re-space with it.
      A submenu's rows now take `edge = ring_inset()` and no
      overshoot, so a highlight there crosses the rings but stops at
      the panel outline. Nothing in the design says otherwise, and
      `bar-window` has no highlighted submenu row to show it.
      `cp-eras-ui-bar` needed no change: it places the chain by
      `menu_chain_width` inside a full-width surface, so the strip
      lands past the pointer rather than being clipped. The one edge
      case is a pointer within 8px of the output's right edge, where
      it would run off; the tray never sits that far over.
    - Neokitsch open-row (submenu parent) box — *found 2026-09-05
      while measuring the overshoot; resolved the same day, and the
      SVG path was the wrong half*. `bar.svg` drew the Devices outline
      at x 1287..1445.2 (line 335) while its own prose called that "6
      from the rings' inner edge", which is 1461.2 — the number the
      separator rule on line 341 uses, and what the code gives both
      (`open_inset.1` and the rule's own inset; the golden measured
      the rule 1460..1461 and the open row 1458.5). `git log -p
      515c2a0` settled it: 1445.2 is the pre-nesting 1458 shifted by
      the 12.8 the rings moved, from when the rows began at y 37
      inside the panel's 22px chamfer and the box had to stop at the
      chamfer's foot. The rows now start below the innermost ring's
      9.2px chamfer, which clears 1461.2 by 3px, so the constraint is
      gone. Box, tab and the `<` glyph moved in `bar.svg` (path and
      prose); `neokitsch.rs` unchanged. One thing the prose said that
      the code did not do: "the "<" glyph right-aligned … so it clears
      the tab" — the marker sat under the tab's footprint.
      `bar::menu_row` now pads the trailing marker by `tab.base +
      tab.inset + icon_gap` (38px) on an open face that carries a tab;
      only neokitsch's does, the other three bars are byte-identical.
      Golden bar-neokitsch re-taken (99.994% against the previous);
      G2i neokitsch bar still the accepted 14%.
    - ~~Neokitsch haze blue annulus — *still open*.~~ Landed
      2026-09-05 with the haze unification (the `[x]` item under
      "SVG→iced pre-work", findings that outlive the wave): the
      strip's ground is the dashboard's `HUB_GROUND` composited by
      `soft.rs`, blue and mask included, so the cast on the last
      ~150px is there (x 1520 #272b42 vs the trace's #292e41; x 1599
      exact). It had been a faint blue cast plus a mask-faded arc on
      the left that alpha-stop annuli with a horizontal fade could not
      draw without even-odd ring paths sliced in x.
    - Found on the way, **not fixed** (theme, not crate): entropism's
      published `tape` is `#9cb795` (`home/themes/entropism/palettes.nix`
      nexus), the *selection* sage, so on the live bar the host tape
      reads as a second selected cell. The crate's own palette and
      `bar.svg` want MID `#728f76` ("the dimmer of the two sage fills
      so it does not read as a selection next to workspace 3"). The
      goldens carry the theme's value. Filed under "Live inconsistency"
      below.
    - Goldens bar-{entropism,neomil,kitsch,neokitsch} re-taken (AE 0 vs
      host renders); matrix 21/21.
- [x] Stale claims — the four era `README.md`s were rewritten
  2026-09-03 (dashboard composites no longer described as real;
  numbers now point at the trace headers). Swept 2026-09-04 against
  the tree: the `Layout` fold had already taken most of the list with
  it (`OpsCharts`/`TileRow` are gone from `style.rs`, `charts.rs` is
  gone, entropism.rs :4-7 and kitsch.rs's overhang comment were
  corrected in place, the crate README's neokitsch double-gold frame
  is described as the invention it was). What was still wrong and is
  now fixed: `src/theme.rs` :7,10,189 and `src/eras/neomil.rs` :12
  cited `src/colors.rs`, deleted in `61db2ae` (now say so and point
  at `Theme::fallback`); the crate README said dashboard blocks could
  still be `&[]` (all four are transcribed) and listed a neokitsch
  BASKET panel "on the mailbox footer" as not yet done — no trace
  puts BASKET on the mailbox (it is the store's plate, drawn), and the
  mailbox notch it also named is `sel_notch`, drawn since 2026-09-03.
  `docs/sources.md` :26-29 is not stale: it says the READMEs' old run
  numbers are shifted by ten, which is still the fact.
  Unverified README claims the docs pass left in place: kitsch bezel
  #f08c1e and #fcbb15, neokitsch #c78948, the neomil `dashboard.svg`
  wordmark vs `dashboard.rs:225-234` "wordmark" (moot: the SVG is
  deleted), the entropism desktop-theme paragraph. The
  `target-components.svg` inventories are moot too: the sheets were
  rebuilt as `components.svg` 2026-09-03 and the README entries
  rewritten from the new sheets.

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
  "gate artefacts" item above). 72/89 are the numbers; both PASS.
  Neomil login held 96 with the trace chamfer now 51 (the era table
  drew 46 until 2026-09-04, when the residue item above moved it to 51
  alongside the card-1 block and chip fixes). Two accepted FAILs, do not chase (one since
  resolved):
  - **entropism login 28%** — **resolved 2026-09-04, 100% PASS.** The
    explanation that stood here (linear-space AA splitting hairline
    pixels across two bins; crate-wide fix) was wrong: the capture had
    no wash at all (3s settle, see the "gate artefacts" item) and, once
    it did, the extractor split rsvg's radial into two ground bins.
    Neither needed `web-colors`.
  - **neokitsch bar 52%** — extractor fragmentation, see the bar item.
  - kitsch mailbox 67% (unselected chevrons are two cells in the
    design, one in the render) is a PASS with a known reason; kitsch
    login's `Wash::RoseBloom` was kept over Plain's 98% because layout
    IoU is 0.963 vs 0.568 (it now scores 97 anyway, see above).
  - Dashboard, not in the wave, gated after the merge: entropism and
    neokitsch PASS; neomil 0% (the item at the top of this file) and
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
  unchanged (nothing drew them). The "Toolkit build-out" list below is
  therefore rebuild-from-traces work when a caller appears, not a
  revival of these files; git has them at `b1e6cb1` if a shape is
  wanted back.

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
  `bar-window`'s bg/fg closure is the theme base. Goldens did not
  move (21/21 after the switch).
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
  record under "SVG→iced pre-work" above (the `web-colors` opt-out is
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
  - **Not done, and why.** *Silhouettes*: a built-in `button` is a
    rounded rectangle, so neomil's br-chamfer, kitsch's stepped bar
    and neokitsch's tabbed bl-chamfer stay `widgets::surface` plates
    inside a `bare` button; the coat sets only fill/edge/ink/radius.
    *Hover/press*: the traces are stills with no such state; nothing
    to read, left for "Motion". *override-hatch*: no era sheet has a
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
