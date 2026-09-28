# Reference SVG correctness

Rechecked 2026-09-21. [All workstreams](../TODO.md) ·
[Evidence and complete inventory](../docs/reference-svg-audit.md).

All 24 references were rendered and reviewed: 16 source-backed screen
traces, four derived component sheets and four original bars. All 16 G1i
checks pass, but they do not validate text, detailed art or local material.
The source images are available locally. These are actionable corrections;
exact unknown texture recipes and unsourced interaction design remain
separate. The audit itself changed documentation only; the first
[correction batch](../docs/reference-svg-fixes.md) now implements nine tasks
and parts of E2/K2 across ten screens. The [second batch](../docs/reference-svg-round2.md)
adds login/badge/envelope contours, mailbox text fitting, socket spacing and
current source documentation. That second batch passed all 252 Rust tests
and all 19 repository checks, including 27 visual cases at 100.000%.
The [third batch](../docs/reference-svg-round3.md) corrects K7, NK-13 and
the Neomil login's extra SVG-to-Iced drift. Validation is recorded there.
The [fourth batch](../docs/reference-svg-round4.md) adds E2 baseline
calibration, NK-09 store contours and Kitsch selected-card hit regions.
The [fifth batch](../docs/reference-svg-round5.md) corrects E2 paragraph and
badge geometry, K8 lower card corners, and NK-12 component/provenance drift.
The [sixth batch](../docs/reference-svg-round6.md) separates NK-05 selected
mailbox inks, improves NM1 footer typography/copies, and corrects K9 citations.

## Entropism

- [x] **E1 — Login masks and typography:** source has ten masks; SVG has
  eleven, with heavier glyphs and tighter pitch. Match the ten primary
  stars and underline, then synchronize component examples and source
  descriptions. Preserve runtime secret semantics.
- [ ] **E2 — Mailbox text fit:** first body line ends at x1233.3 in the
  source versus x1149.6 in the SVG. The remaining lines have similar
  width errors. `textLength`/`lengthAdjust` do not affect the pinned render;
  use supported geometry/typography, preserving actual words and breaks.
  Body fit and boxed A/B/C are corrected: mean body line-end error is
  2.46px, down from77.04px. The second pass fits 26 section/row/sender/panel/
  button/micro-print runs: mean edge error 2.76→0.20px. Source-font contours
  and per-action width residuals up to 2.92px remain. The fourth batch
  calibrates the shared mailbox baseline factor: native SVG-to-Iced
  vertical-edge error over 26 runs falls 1.210→0.056px, with widths
  unchanged. The fifth batch moves the third paragraph from y533.2 to
  y535 without changing earlier lines, and fits each badge's width and
  center in SVG, component sheet and Iced. Native runtime horizontal
  bounds match the source except one T4 edge by 0.417px. Font contours,
  small vertical raster differences and action-label width residuals
  remain open. Words/breaks are preserved. See the
  [baseline measurements](../docs/entropism/mailbox-baseline.md) and
  [paragraph/badge correction](../docs/entropism/mailbox-layout.md).
- [x] **E3 — Store fourth-card crop:** primary source drawing stops near
  x1564.6, while the SVG continues to x1600. Measure a persistent viewport
  and retain the open cut; classify faint residue beyond it separately.
- [ ] **E4 — Store artwork and lettering:** replace the block rifle and
  angular 4ST substitute with measured contours/detail. Refit stats,
  sockets and compliance text. Preserve inverse ink on selected card1.
- [ ] **E5 — Four-screen materials and remaining hub printing:** fit broad
  light fields before fine texture. Current radial ground is too olive
  at the sides and misses the upper illumination. Check each source before
  sharing a model; retain the already-close login footer fill in SVG. Native
  integration review found Iced still substitutes the brighter selection
  role there; the reference-only override is now verified for standalone
  and published palettes. Hub geometry
  and 7+3 body lines are supported; refine caption/body ink locally.
- [x] **E6 — Derived sheet, bar palette and provenance:** update inherited
  specimens; reconcile bar border/selection `#5d7752`/`#9cb795` with the
  accepted `#8fba97`/`#a6d3a7` roles. Correct obsolete stroke/halo claims
  and label historical implementation deltas. Bar remains an original.
  Parent excerpt equality, local render review and the bar implementation
  gate pass; the H checkpoint passes the full repository check. See
  [consistency record](../docs/entropism/component-sync.md).

## Kitsch

- [x] **K1 — Dashboard content and panel:** replace nine placeholder bars
  with the source's eight lines; reconstruct the connected warning ribbon,
  tab and rounded panel corners. Keep the source BRAINDANCE content.
- [ ] **K2 — Dashboard fan printing/material:** idle labels need source
  mint ink instead of dark `#123c38`. Keep selected EVENTS dark on yellow.
  Label polarity is corrected. Face fields and stroke/material fitting
  remain open; supported fan/ghost geometry is preserved.
- [x] **K3 — Four-screen grounds:** fit the rose and grey-green fields
  against clear patches/holdouts. Test common dashboard/mail/store data;
  login differs. This includes broad color error, beyond fine grain.
  All four held-out source comparisons improve; native SVG/Iced patch
  medians agree within 1–2 RGB levels. Both relevant gate sets and the full
  H repository check pass. Fine photographed grain is separate. See
  [ground measurements](../docs/kitsch/ground-fit.md).
- [x] **K4 — Login control contours:** restore the input's stepped lower
  edge and gap above ENTER, plus rounded step/control corners. Preserve
  the full-height bracket and the three-card arrangement.
- [ ] **K5 — Scoped typography:** fit login ENTER/PROTECTED/clock, store
  metadata/stats/socket/compliance labels and mailbox sender/notice text.
  PROTECTED spans about101px in the source versus78px in the SVG. Preserve
  correct mailbox body wording and line breaks; avoid global scaling.
  Login metrics are fitted within about 1px; store/mail and exact glyph
  forms remain open.
- [x] **K6 — Selected mailbox envelope:** source row1 has an open flap;
  the SVG reuses the closed symbol. Trace a distinct open glyph and retain
  closed icons for the remaining four rows. Correct the old prose claiming
  the source has no open-envelope variant. Follow-up also restores the
  four closed glyphs’ source lower folds and measured16.05×9.95 bounds.
- [x] **K7 — Store fourth-card fade:** primary foreground now stops near
  x1523, with five decreasing-opacity slices through x1550 in SVG/Iced.
  The left-projecting flag and strokes remain intact, and the same fade
  applies to selection/feedback. The source's broader ground vignette and
  exact chromatic echo remain separate material limits. See the
  [measurement record](../docs/kitsch/store-fade.md).
- [ ] **K8 — Store artwork and feet:** finish rifle internals, source
  socket scatter and certification marks. Rounded lower corners on
  normal/selected cards are corrected with a measured 10.5px quadratic
  corner span; normal/selected right-edge errors fall to 0.67/0.42px.
  Selected card2's gun offset, feedback and fourth-card fade are preserved.
  Source rifle contours and 25-cell socket scatter are now verified in K1;
  native primary-mask IoU is .911/.941 for ordinary/selected rifles.
  Certification/warning SVG details are ported, but the source comparison
  still shows schematic RG5/ring/E lettering and warning micro-lines;
  their primary contour/printing fit remains actionable. Exact halo
  fitting is separate. See [corner measurements](../docs/kitsch/store-corners.md).
- [x] **K9 — Source citations:** corrected login bracket/barcode bounds,
  bar USER-box extent and blanket ink/stroke claims. Descriptions distinguish
  path control points from endpoints and thresholded source ink from exact
  geometry. Current bar implementation and inferred interactions are labeled;
  login/bar drawings and component geometry are unchanged. See the
  [citation record](../docs/kitsch/source-citations.md).

## Neo-kitsch

- [x] **NK-01 — Login wire ends:** reverse the incorrectly curled outer
  ends to the source's descending, rounded feet. Preserve 22 strands and
  broad center plateau; fit both endpoints and synchronize the sheet.
- [x] **NK-02 — Dashboard/mail header band:** source left rise spans
  roughly x160..220; SVG uses x52..130 and crosses T1. Remeasure rise,
  outer curls and terminal strands independently on both screens.
- [x] **NK-03 — T2 badge:** restore the source shoulder and tightly swept
  ring fan; the SVG deliberately spreads the top strands too widely and
  makes the rise too deep/rounded. Preserve the outlined selected state.
- [x] **NK-04 — Login logo/tape:** replace plain ARASAKA type with the
  source stencil contours and fit annotation text inside its cells.
  Seven outer contours/four counters and native fitted captions are reviewed
  against the source and SVG. Both login gates and the H full check pass.
  See [branding measurements](../docs/neokitsch/login-branding.md).
- [ ] **NK-05 — Scoped four-screen typography:** fit CTA/boxed letters,
  header/module labels, mailbox FROM/heading/body weight, store stat/socket
  labels and 4ST letterforms. Preserve correct mailbox words/line breaks.
  Promote previously deferred small DPS/coarse type observations to work.
  Selected mailbox title, sender and envelope now use the distinct trace
  inks (#7b5438/#895f3b/#865c39) in the reference runtime, including held
  selection. Custom palettes retain semantic roles. Native captures confirm
  the change is confined to those marks; the photo remains darker/softer,
  and subject/sender font fitting remains open. See the
  [ink measurements](../docs/neokitsch/mailbox-inks.md).
- [x] **NK-06 — Dashboard reference copy:** restore the source's 6+2 body
  lines. Current SVG intentionally substitutes 5+3 mailbox lines; keep
  application content choices separate from the source reference.
- [ ] **NK-07 — Veneer pattern:** correct dominant direction, seam and
  coverage per source surface. Store selected body is mostly vertical in
  the source but horizontal in SVG; dashboard panel and mailbox bar also
  have different convergence patterns. This follows earlier grain-presence
  work. Dominant geometry is actionable without the exact authoring asset.
- [x] **NK-08 — Store weapons:** shared source-native MAGNUM contours
  replace the block silhouettes, fitted at 0.99 scale with measured local
  translation. Both plain and raised cards retain their placement and
  internal source detail. Native primary gold-mask IoU is .821; local
  source/SVG/Iced review, both gates and the K1 full check pass. Faint
  glow and material texture remain distinct from primary weapon art.
- [x] **NK-09 — Store tabs/shoulders:** product and nav tabs now have
  narrow tops/wide bases and curved joins. Five shoulder echoes fan from
  the source-supported common low tangent; native review recovered the
  faint fifth echo that a bright-pixel mask missed. Trace, component
  excerpts, Rust and inferred hover outlines are synchronized. The
  separate inverted mailbox tab is preserved. Remaining frame ink and
  right-corner fit are explicit in NK-14. See the
  [measurements](../docs/neokitsch/store-tabs.md).
- [x] **NK-10 — Closed mailbox envelopes:** add the omitted lower fold
  diagonals. Preserve open rows1/3/7 and closed selected row2/rows4/5/6.
- [x] **NK-11 — Shared ground:** fit clear-source regions and reconcile
  dashboard's older haze with the other three approximations. Exclude the
  login's lower wire when sampling ground; fine noise remains separate.
  Held-out source-to-native RMS is 2.06–2.23 RGB levels over four screens;
  source/SVG, SVG/Iced and the H full repository checks pass. Original
  bar and inferred hover preblend are preserved. See
  [ground measurements](../docs/neokitsch/ground-fit.md).
- [x] **NK-12 — Components/provenance:** component EMAIL/panel veneer,
  caption text/positions and tape now match current dashboard excerpts.
  The literal six-plus-two body copy was already current and is preserved.
  Obsolete implementation deltas and bar haze/CTA citations are corrected;
  bar drawing and interaction specimens are unchanged. The bar remains an
  original design. See [excerpt verification](../docs/neokitsch/component-sync.md).

- [x] **NK-13 — Mailbox envelope placement:** remeasurement corrected the
  premise: rules, text and selected bounds support the existing 60.2px
  pitch. Only the envelopes drift. Per-row glyph offsets now correct
  rows3–7 without moving text, rules or hit regions. Native source/SVG
  row7 displacement falls from4.58px to0px, with residuals at most one
  source pixel (0.42 design px). Open/closed states and feedback ink remain.
  See [measurements](../docs/neokitsch/mailbox-row-spacing.md).

- [ ] **NK-14 — Store frame ink and right-corner refinement:** the outer
  contour and five echo strokes still differ in brightness/softness from
  the photo; the innermost right turns are tighter. Fit stroke materials
  and corner extents on both plain and selected backgrounds, retaining
  NK-09's corrected common shoulder foot, tab taper and contour count.
  Native source profiles at x1320/1360/1400 resolve all six strokes;
  at x1360 the faint innermost line peaks near RGB76/58/37. Avoid a bright
  mask that drops it. See the [frame record](../docs/neokitsch/store-tabs.md).

- [ ] **NK-15 — Runtime socket scatter:** native K review finds the
  selected store socket still renders a schematic block QR, while the
  source and SVG have scattered cells. Port the measured SVG occupancy,
  pitch and origins to ordinary/selected runtime cards and inspect the
  BASKET variant. Preserve source orientation and feedback recoloring;
  this is artwork, not an encoded QR payload.

## Neomil

Keep the existing screen records as the owners of these tasks:

- [Login](neomil-login.md): avatar/portraits, mask/card/header printing,
  screen-specific materials, palette/geometry drift and fine echoes.
- [Dashboard](neomil-dashboard.md): **NM1** footer primary typography and
  broad printing copies are improved; runtime cap-height residuals and fine
  striations remain, alongside maker microtext and remaining small ink.
- [Mailbox](neomil-mailbox.md): **NM2 is fixed**, including the inward
  side step, square lower corner and measured primary stroke. Upper-edge
  fit and fine cartridge/text detail remain.
- [Store](neomil-store.md): **NM3 is fixed**, restoring the 25th socket
  cell and selected origin. The second batch fits cell spacing and all four
  origins; materials, printing and artwork remain.

- [x] **NM4 — Current source/excerpt descriptions:** correct stale
  component statements about deleted `Layout::OpsCharts` and different
  screen grounds; refresh README/source gate and geometry claims. Keep
  historic results dated and original bars clearly distinguished from
  photographed screens.

## First-batch validation

All 250 Rust tests, ten G1i and ten G2i gates pass. Full `./check` passes
all 19 checks, including all 27 visual cases at 100.000%. Ten reviewed goldens
were refreshed. Source/SVG/Iced crops, opening and feedback states, native
and fractional rendering are documented in the correction record. All
changes remain staged; live desktop verification is still separate.

## Second-batch validation

K4, NK-03, NK-10, NM4 and the Neomil socket-spacing follow-up are complete.
E2 and K5 have further measured typography improvements, with remaining
font/baseline/label-fit work kept explicit. NK-13 records the newly measured
mailbox row-spacing drift.

All 252 Rust tests, eight G1i and five G2i gates pass. All 24 SVGs have unique
IDs and resolved references. Five reviewed goldens are refreshed; full
`./check` passes all 19 checks and all 27 visual cases at 100.000%, without
retries. Native/fractional, long-input, clipped-card, feedback and opening
reviews are in the [second-batch record](../docs/reference-svg-round2.md).
All changes remain staged and uncommitted; live desktop checks remain open.

## Acceptance for each correction

- [ ] For each correction, inspect the original, freshly rendered SVG and
  native crops with repository fonts. Preserve source quirks listed in the
  audit; unreadable marks require supported vectors, not invented text.
- [ ] Update component excerpts and source descriptions with their parent
  trace. Keep inferred interaction states explicitly identified.
- [ ] Rerun the relevant G1i gate and measure the changed region; a broad
  PASS alone cannot close a task. Never hide primary art as `photo` or
  weaken thresholds to accommodate missing detail.
- [ ] When implementing corrected references, review SVG→Iced states and
  opening motion, run relevant regressions and the full repository checks,
  and refresh only reviewed intentional goldens. Live desktop checks remain
  separate. The correction batch records its implementation and golden changes.

## Seventh-batch checkpoint

The H snapshot passes 263 Rust tests and all 22 repository checks; all 27
golden cases match 100.000% on their first attempt. All 16 source/SVG gates
pass, as do all 13 affected SVG/Iced comparisons. The extractor now uses
observed contour support before accepting inferred corners/diamond tips or
splitting a textured panel. Nineteen controls preserve missing/moved-widget
and genuine-overlap failures; matching thresholds are unchanged. Native
source/SVG/Iced review supports the named completed primary-art/ground
items; it does not establish exact fine photographic fidelity. E4's
selected inverse artwork and Neomil login materials continue in the next
batch. See [the seventh-batch record](../docs/reference-svg-round7.md).
