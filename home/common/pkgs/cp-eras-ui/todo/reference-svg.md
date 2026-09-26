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
current source documentation. All 252 Rust tests and all 19 repository checks
pass, including 27 visual cases at 100.000%; details are recorded below.

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
  and per-action width residuals up to 2.92px remain. Native Iced also has
  a roughly 0.83–1.61px upward baseline offset under the shared mailbox
  line-height model; calibrate that explicitly. Words/breaks are preserved.
- [x] **E3 — Store fourth-card crop:** primary source drawing stops near
  x1564.6, while the SVG continues to x1600. Measure a persistent viewport
  and retain the open cut; classify faint residue beyond it separately.
- [ ] **E4 — Store artwork and lettering:** replace the block rifle and
  angular 4ST substitute with measured contours/detail. Refit stats,
  sockets and compliance text. Preserve inverse ink on selected card1.
- [ ] **E5 — Four-screen materials and remaining hub printing:** fit broad
  light fields before fine texture. Current radial ground is too olive
  at the sides and misses the upper illumination. Check each source before
  sharing a model; retain the already-close login footer fill. Hub geometry
  and 7+3 body lines are supported; refine caption/body ink locally.
- [ ] **E6 — Derived sheet, bar palette and provenance:** update inherited
  specimens; reconcile bar border/selection `#5d7752`/`#9cb795` with the
  accepted `#8fba97`/`#a6d3a7` roles. Correct obsolete stroke/halo claims
  and label historical implementation deltas. Bar remains an original.

## Kitsch

- [x] **K1 — Dashboard content and panel:** replace nine placeholder bars
  with the source's eight lines; reconstruct the connected warning ribbon,
  tab and rounded panel corners. Keep the source BRAINDANCE content.
- [ ] **K2 — Dashboard fan printing/material:** idle labels need source
  mint ink instead of dark `#123c38`. Keep selected EVENTS dark on yellow.
  Label polarity is corrected. Face fields and stroke/material fitting
  remain open; supported fan/ghost geometry is preserved.
- [ ] **K3 — Four-screen grounds:** fit the rose and grey-green fields
  against clear patches/holdouts. Test common dashboard/mail/store data;
  login differs. This includes broad color error, beyond fine grain.
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
- [ ] **K7 — Store fourth-card fade:** source product content fades out
  around x1523..1550; SVG persists through x1600. Measure the foreground
  cutoff and colored residue rather than assume only a hard clip. This
  makes the previously recorded vignette follow-up concrete.
- [ ] **K8 — Store artwork and feet:** finish rifle internals, source
  socket scatter and certification marks; restore rounded lower corners
  on normal/selected cards. Preserve selected card2's measured gun offset.
- [ ] **K9 — Source citations:** fix stale login bracket/barcode bounds,
  bar USER-box extent and blanket ink/stroke claims. Component definitions
  currently agree with their traces; synchronize each corrected specimen.

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
- [ ] **NK-04 — Login logo/tape:** replace plain ARASAKA type with the
  source stencil contours and fit annotation text inside its cells.
- [ ] **NK-05 — Scoped four-screen typography:** fit CTA/boxed letters,
  header/module labels, mailbox FROM/heading/body weight, store stat/socket
  labels and 4ST letterforms. Preserve correct mailbox words/line breaks.
  Promote previously deferred small DPS/coarse type observations to work.
- [x] **NK-06 — Dashboard reference copy:** restore the source's 6+2 body
  lines. Current SVG intentionally substitutes 5+3 mailbox lines; keep
  application content choices separate from the source reference.
- [ ] **NK-07 — Veneer pattern:** correct dominant direction, seam and
  coverage per source surface. Store selected body is mostly vertical in
  the source but horizontal in SVG; dashboard panel and mailbox bar also
  have different convergence patterns. This follows earlier grain-presence
  work. Dominant geometry is actionable without the exact authoring asset.
- [ ] **NK-08 — Store weapons:** replace coarse block silhouettes with
  source contour and internal line artwork on all four cards; preserve
  raised selected-card placement and source-supported detail only.
- [ ] **NK-09 — Store tabs/shoulders:** product and nav tabs taper in the
  opposite direction to source; fit narrow tops/wide bases and curved
  shoulder joins. Preserve the separately inverted mailbox selection tab.
- [x] **NK-10 — Closed mailbox envelopes:** add the omitted lower fold
  diagonals. Preserve open rows1/3/7 and closed selected row2/rows4/5/6.
- [ ] **NK-11 — Shared ground:** fit clear-source regions and reconcile
  dashboard's older haze with the other three approximations. Exclude the
  login's lower wire when sampling ground; fine noise remains separate.
- [ ] **NK-12 — Components/provenance:** replace stale flat EMAIL, caption
  bars and placeholder panel/tape specimens with current trace excerpts.
  Refresh obsolete implementation deltas and bar haze/CTA citations while
  preserving the bar's explicit original-design status.

- [ ] **NK-13 — Mailbox row placement:** native review finds the existing
  60.2px pitch is shorter than source (~60.8px); drift grows from ~0.4px at
  row2 to ~4.4px at row7. Measure text, rules, glyphs and selected-row bounds
  together, preserving open/closed states and selection/feedback behavior.

## Neomil

Keep the existing screen records as the owners of these tasks:

- [Login](neomil-login.md): avatar/portraits, mask/card/header printing,
  screen-specific materials, palette/geometry drift and fine echoes.
- [Dashboard](neomil-dashboard.md): add **NM1**, footer primary typography
  and printing copies, alongside maker microtext and remaining small ink.
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
