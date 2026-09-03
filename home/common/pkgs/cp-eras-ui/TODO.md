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
- [x] ~~**Rework `docs/kitsch/dashboard.svg` and
  `docs/neokitsch/dashboard.svg`**~~ — **deleted instead, 2026-09-03**,
  with the other two `dashboard.svg`. They were "original composites"
  drawn app-first while their real source went unread (0.03–0.07 on
  the ink gate) and reworking them would have meant redrawing the
  trace under another name. The trace is the design; the *screen* is
  what has to follow it, which is the `Layout` item below.
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
- [ ] Neomil: store card 4's title is ghosted/duplicated in the photo
  (glitch treatment); mailbox has small QR-like glyphs beside the
  panel the trace simplifies; dashboard glitch echoes are two hairline
  pairs where the photo has a few more fragments.
- [x] Entropism mailbox T1–T4: bold 27 + scale(1.42..1.46, 1).
- [ ] Entropism `bar.svg` menu icons overlap the row text (icon to
  1177, text at 1175) — pre-existing, the golden has a gap; the third
  pass did not touch `bar.svg`.
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
- [ ] **Entropism OUTLINE ink — decision needed.** The real outline ink
  measures ~#8fba97, i.e. the same sage as the text (#94bb94); the
  earlier ~#709174 was rescale dilution and the palette's OUTLINE
  #5d7752 is far from both. Options: repoint OUTLINE, or add a role.
  Affects every entropism screen and the bar; until decided the login
  conversion carries its inks as `Ink::Fixed` (#75967b / #8aac8c /
  #20281c / #799d81) and the READMEs say "under measurement".

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
  the extractor's cells; documented in `bar.rs`). Neokitsch labels are
  `Face::Medium` (Regular was visibly thinner than the design's
  dark-on-gold; decided by eye, `src/eras/neokitsch.rs`). Neokitsch
  stroke is 2.0 not the delta's 1.6 (AA). The tray diamonds render
  purple/orange because the example feeds sample SNI raster icons; the
  SVG's diamonds are stand-ins, not a fault. **The layer-shell bar is
  unverified until a switch**; only `bar-window` is captured.
  - [ ] Not followed, per era: entropism selection inset 1px; neomil
    window box 1.0 stroke (`Bar` has one stroke width); kitsch chevron
    4.9px shoulder step; neokitsch highlight row running 8px past the
    panel edge (iced clips), the haze's blue annulus, window label at
    weight 400. Each needs a widget, not a value — see "widget gaps"
    under the conversion wave.
- [ ] Stale claims — the four era `README.md`s were rewritten
  2026-09-03 (dashboard composites no longer described as real;
  numbers now point at the trace headers). Still stale, all pointing
  at the same root: `src/style.rs` `Layout::OpsCharts` / `TileRow`
  docs describe invented compositions; `src/eras/entropism.rs` :4 "1px
  strokes", :5 "doc #24-32", the `TileRow` block "single row of four
  tiles … #42" (42 is the store; the trace is a hub);
  `src/screens/dashboard.rs` :49-64 / :225-234 / :251-259 and
  `src/widgets/charts.rs` :7,17 "straight off the traces";
  `src/eras/neomil.rs` :9 and `src/theme.rs` :7,10,189 cite
  `src/colors.rs`, which is gone; `src/eras/kitsch.rs` :92-107 overhang
  12/18 where `store-trace.svg` measures 27 on 35; crate `README.md`
  :54-58, :66, :94 (neokitsch double-gold frame — an invention per
  `sources.md:183`), :171-173, :294-312; `docs/sources.md` :26-29 now
  behind the READMEs. All of it collapses into the `Layout` decision
  above (fold OpsCharts/TileRow back to ModuleHub) — fix it there, once.
  Unverified README claims the docs pass left in place: kitsch bezel
  #f08c1e and #fcbb15, neokitsch #c78948, the neomil `dashboard.svg`
  wordmark vs `dashboard.rs:225-234` "wordmark" (moot: the SVG is
  deleted), the entropism
  desktop-theme paragraph, the `target-components.svg` inventories.

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
  merged by hand). Dashboard is still blocked on `Layout`. What
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
  100/84/88/82. Two accepted FAILs, do not chase:
  - **entropism login 28%** — the warm-lift ground shape (72% of the
    design's area) loses its k-means centre to the render's edge ramp:
    iced built without `web-colors` blends AA in linear space and 4×
    MSAA cannot make 0.625 coverage, so hairline pixels split across
    two bins. Backdrops are pixel-identical and the side-by-side
    matches. The fixes (`web-colors`, sRGB AA) are crate-wide and
    move every golden.
  - **neokitsch bar 52%** — extractor fragmentation, see the bar item.
  - kitsch mailbox 67% (unselected chevrons are two cells in the
    design, one in the render) and kitsch login 72% (`Wash::RoseBloom`
    kept over Plain's 98% because layout IoU is 0.963 vs 0.568) are
    PASSes with a known reason.
  - Dashboard, not in the wave, gated after the merge: entropism and
    neokitsch PASS; neomil 0% (the item at the top of this file) and
    kitsch 19% — the kitsch number is identical against the
    `a0a9274` trace, so pre-existing, not a regression. **Superseded
    2026-09-03:** those four numbers were against the app-shaped
    `dashboard.svg` composites, since deleted; against the traces all
    four dashboards score 0% (see the housekeeping item).

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
    are not drawn.
  - **Widget gaps the agents reported** (each "use widgets, don't edit
    them" collided with): absolute placement; `Cut` has no Step cut;
    `Surface` has no tab, ticks or dense grain; no custom panel
    silhouette; no blur; `fonts.rs` has no semibold (`Face::SemiBold`
    maps to Medium); `Ground` caps at ~6% alpha (neokitsch haze wants
    #4f4262). This is the input to the canvas-vs-widgets decision
    below.
  - **Three haze implementations coexist**: login `wash_image` (RGBA
    buffer as a cached canvas image), mailbox `Lobe` annuli/wedges (96
    bands / 72 wedges), store `Prim::Lobe` annuli. Unify — the image
    one is the smoothest and the only one that survived a side-by-side
    without banding.
  - **Extractor cluster budget:** with k=8, a haze takes 5 clusters,
    so line art drawn dimmer than measured merges ink families (the
    neokitsch mailbox wire went to `Ink::Tape`, RIFLES to `Ink::Fg`,
    per a re-cut trace). Backgrounds are not optional for the
    extractor (kitsch bloom = 5/8 clusters). Noted in `PIPELINE.md`.
  - `widgets::row::mail_row` / `row::Mail` have no caller now.
  - **Live inconsistency:** `src/eras/kitsch.rs` has `border: TEAL`
    (#7ddec8) where `home/themes/kitsch/scheme.nix` says border
    #2e5f57; the store samples #5fd6c2. Outlives this crate — decide
    which is right.
  - Bar/entropism inks and the login `Ink::Fixed`s wait on the
    OUTLINE decision under "Trace improvements".
- [ ] **Canvas vs widgets — decide.** Four screens are now display
  lists over era tables and the widget layer (`widgets/`, `Layout`,
  `Cut`, `Surface`, `Ground`) serves only the dashboard and the bar's
  window. Either the widget layer grows the gaps above and the screens
  fold back onto it, or it is retired to what the bar needs and the
  dashboard converts the same way. Decide before the dashboard, and
  together with the `Layout` fold-back — same decision.

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

> Both neomil sheets were deleted 2026-09-03 (see Housekeeping); the
> acceptance test no longer exists as a file. The items below stand on
> their own as widget work, but "NEOMIL OPS" is not a screen the
> material has — the neomil traces are the targets now.

- [ ] **Theme/Catalog first**: replace loose color consts at call
  sites with a semantic iced Theme + widget catalogs (surface/
  primary/dim/danger...) so every later widget styles against tokens.
  Everything below is written twice if this comes second.
- [x] **Migrate to iced 0.14** — done 2026-09-02, before the build-out;
  record under "SVG→iced pre-work" above (the `web-colors` opt-out is
  the part to know about).
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
