# Reference SVG correctness

Initial audit 2026-09-21; correction status updated through AC, 2026-09-29. [All workstreams](../TODO.md) ·
[Evidence and complete inventory](../docs/reference-svg-audit.md).

All 24 references were rendered and reviewed: 16 source-backed screen
traces, four derived component sheets and four original bars. All 16 G1i
checks pass, but they do not validate text, detailed art or local material.
The source images are available locally. Completed entries record accepted
corrections; open entries distinguish bounded local fits from unresolved
source-material research. Exact unknown texture recipes and unsourced
interaction design remain separate. The audit itself changed documentation only; the first
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
- [ ] **E2 — Remaining mailbox glyph/printing fidelity:** body widths,
  paragraph spacing, badge geometry, native baselines and all four action
  widths are fitted. N source/native action endpoints agree within one
  pixel. Original typeface contours and photographed edge modulation remain
  separate from those completed layout corrections; further changes need
  source-supported local comparisons, not another global scale adjustment.
  Preserve words and breaks. See [baseline measurements](../docs/entropism/mailbox-baseline.md)
  and [paragraph/badge correction](../docs/entropism/mailbox-layout.md).
- [x] **E3 — Store fourth-card crop:** primary source drawing stops near
  x1564.6, while the SVG continues to x1600. Measure a persistent viewport
  and retain the open cut; classify faint residue beyond it separately.
- [ ] **E4 — Remaining store fine lettering/printing:** primary rifles,
  inverse seams, 4ST contours, socket scatter and local stat/socket fit are
  implemented and native-reviewed. W fits the value glyph size/weight and
  X calibrates the stretched native baselines; ordinary and selected
  holdouts improve while the unchanged `5` runs preserve their fit.
  See [value measurements](../docs/entropism/store-typography.md).
  Do not replace that artwork again.
  Remaining exact glyph forms and fine weapon/printing echoes need a newly
  identified source crop before implementation. See [art measurements](../docs/entropism/store-art.md).
- [x] **E5 — Broad four-screen materials and hub printing:** independent
  login/shared grounds replace the olive field; native/SVG patch medians
  agree within one RGB level. Caption stroke contamination, tile/caption/
  body inks, complete ten-line copy and native body baselines are corrected.
  Reference login footer fill also matches source while custom themes keep
  semantic selection. H/K1/L/N review and the N full check support these
  scoped corrections. Fine grain, local exposure and exact letter edges
  remain source-material limits. See [ground fit](../docs/entropism/ground-fit.md)
  and [hub printing](../docs/entropism/dashboard-printing.md).
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
- [ ] **K2 — Remaining dashboard fan overlap/softness:** mint idle labels,
  dark selected EVENTS, broad face illumination and label placement are
  corrected. W also fits ghost width, count and pitch; all seven measured
  edge profiles improve, and independent opposite-edge positions stay
  within one native pixel across fan depth. Fractional held/selection and
  opening checks preserve the added far silhouettes. Translucent overlap
  visibility and photographed softness remain approximate; do not repeat
  the rejected uniform ghost-opacity reduction. See the
  [material measurements](../docs/kitsch/dashboard-material.md).
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
- [ ] **K5 — Remaining store compliance/metadata fit:** login controls and
  clock, store titles/stats/values/sockets/footer tracking, and mailbox
  senders/notices are fitted. Selected sender weight follows selection;
  native notice primary areas now agree with source within 0.5%. P corrects
  the first compliance line's width; three native endpoints are within one
  source pixel. T fits the footer brand tracking and passes native/state,
  gate and full-check review. Exact glyph contours, softness and fine
  metadata printing remain approximate. See [printing fits](../docs/kitsch/typography-fit.md).
- [x] **K5 follow-up — Port shelf-brand width/weight to native drawing.**
  AB review separates a runtime transcription gap from exact glyph limits:
  source and SVG PETROCHEM span native x1558–1683 on card 1, while Iced
  ends at x1644; selected card 2 and ordinary card 3 repeat the 39-pixel
  deficit. BETTERLIFE TEC is also too narrow and uses Regular rather than
  the source/SVG's bold weight. Fit these two native runs independently,
  preserve their boxes, band geometry and semantic inks, and validate
  ordinary/selected/fractional feedback plus first/middle/last segments.
  The already fitted SVG and unrelated compliance/footer runs stay fixed.
  AB source/native/state review and both gates pass. The full check passes
  all 22 checks and 27 exact visual cases; 286 Rust tests pass. See the
  [AB checkpoint](../docs/reference-svg-round13.md) for the local metrics
  and retained fine-printing limits.

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
- [ ] **K8 — Remaining tiny certification printing:** source rifle and
  inverse contours, 25-cell socket scatter, lower card corners and the
  primary certification/warning silhouettes are implemented and reviewed.
  N corrects the RG5 frame, SC disc, hollow warning and inset three-bar mark;
  tiny RG5/SC glyphs and warning microprint remain approximate. Further work
  must use local source comparisons and retain selected contrast/fourth-card
  clipping. Exact photographic halo remains separate. See
  [art measurements](../docs/kitsch/store-art.md).
- [x] **K8 follow-up — Restore four SC badge corner apertures.** A local
  source recheck finds four small light cutouts inside the dark square,
  outside its circular disc, repeated on all four photographed cards.
  SVG and Iced omit them. Fit compact knockout contours against card 1
  and check the selected and remaining ordinary copies independently.
  Preserve the square, disc, SC letters, band colors and fourth-card clip;
  verify source/SVG/native pixels and feedback before closing. This is
  a bounded topology correction, separate from illegible warning text and
  exact photographic halo.
  AB source/native/state review and both gates pass. The full check passes
  all 22 checks and 27 exact visual cases; 286 Rust tests pass. See the
  [AB checkpoint](../docs/reference-svg-round13.md) for the local metrics
  and retained fine-printing limits.

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
  while exact glyph contours and photographic softness remain. Through T,
  the store 4ST/stat/socket fits, seven mailbox title/sender rows and
  dashboard CUSTOMER header pass source/native/state and integrated checks.
  V also improves all seven dashboard module-label center errors and
  aligns the left LEVEL bounding box, with fractional/custom/opening
  review. Exact glyph contours, local header ink and photographic softness
  remain; rejected uniform ink/left-T1 changes stay unapplied. CTA geometry
  is already close to source. See the
  [ink measurements](../docs/neokitsch/mailbox-inks.md) and
  [header fits](../docs/neokitsch/dashboard-header-typography.md).
- [x] **NK-05 follow-up — Port mailbox action-label typography.** AB
  source/SVG/native review finds all four RIFLES runs still use 15px Regular
  in Iced, versus the SVG's 16px Medium with tracking. Native widths are
  92–93 pixels, source 109 and SVG 111; native caps are also five pixels
  short. Fit native size/weight/width and label origins independently of
  the button frames, preserving action content and semantic ink. These
  action drawings have no pointer behavior in the current mailbox; do not
  invent interaction states for this typography correction. Check all four
  source copies, fractional row-selection states and custom ink.
  Photographic halo and exact font contours remain separate.
  AB source/native/state review and both gates pass. The full check passes
  all 22 checks and 27 exact visual cases; 286 Rust tests pass. See the
  [AB checkpoint](../docs/reference-svg-round13.md) for the local metrics
  and retained fine-printing limits.

- [x] **NK-06 — Dashboard reference copy:** restore the source's 6+2 body
  lines. Current SVG intentionally substitutes 5+3 mailbox lines; keep
  application content choices separate from the source reference.
- [ ] **NK-07 — Remaining veneer curvature and convergence:** the selected
  store body already has 125 longitudinal strands and its cross-grain seam
  in SVG/Iced; the old horizontal-body premise is superseded. Its remaining
  local differences are straighter/evenly spaced strands and a regular
  socket-band stripe in place of the photographed fan. Y corrects the
  right seam bend; broader convergence remains unresolved. A local cubic
  family improves angles but worsens ridge density and tight RGB holdouts,
  so it is rejected.
  Dashboard lower convergence and mailbox bar corner/central branching
  remain open. Rejected fits and their held-out failures are recorded in
  [veneer measurements](../docs/neokitsch/veneer-fit.md).
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
  the photo. The inner upper turns are corrected below; fit remaining stroke
  materials on both backgrounds, retaining
  NK-09's corrected common shoulder foot, tab taper and contour count.
  Native source profiles at x1320/1360/1400 resolve all six strokes;
  at x1360 the faint innermost line peaks near RGB76/58/37. Avoid a bright
  mask that drops it. The ninth batch corrects the three inner upper
  turns: native ridge positions agree with source within about 1–2 pixels
  on both backgrounds, and full stems/lower joins retain continuity.
  W corrects the lower outer extents; X restores the missing fifth lower
  echo and fits the side turns. Native ordinary bottom ridges are within
  one source pixel; selected inner ridges retain 1–3px errors. A selected
  spacing trial improves the straight span but worsens independent bends
  and is rejected. The innermost line remains too dim, with no supported
  uniform opacity correction. The implementation gate's classifier-boundary
  failure is corrected through strict component-mask identity, preserving
  missing/moved/overlap failures and the existing thresholds. See the
  [frame measurements](../docs/neokitsch/store-frame.md) and
  [gate diagnosis](../docs/reference-svg-round11.md).

- [x] **NK-15 — Runtime socket scatter:** native K review found the
  selected store socket still renders a schematic block QR, while the
  source and SVG have scattered cells. Port the measured SVG occupancy,
  pitch and origins to ordinary/selected runtime cards and inspect the
  BASKET variant. Preserve source orientation and feedback recoloring;
  this is artwork, not an encoded QR payload. Completed with 25 discrete
  cells, measured pitch/origins and the BASKET variant. Native/SVG socket
  mask IoU is .973; source/native improves .463→.644. Held fourth-card
  clipping and recoloring pass native review and the N repository check.
  Source glow/material remains separate.

## Neomil

Keep the existing screen records as the owners of these tasks:

- [Login](neomil-login.md): avatar/portraits, mask/card/header printing,
  screen-specific materials, palette/geometry drift and fine echoes.
- [Dashboard](neomil-dashboard.md): **NM1** footer primary typography and
  broad printing copies are improved; runtime cap-height residuals and fine
  striations remain. Maker microtext, tiny tape ink and chip-2's bounded
  ink correction have passed local native review.
- [Mailbox](neomil-mailbox.md): **NM2 is fixed**, including the inward
  side step, square lower corner and measured primary stroke. Upper-edge
  fit and normal-cartridge count/cadence/angle are corrected; fine
  cartridge/text contrast and echoes remain.
- [Store](neomil-store.md): **NM3 is fixed**, restoring the 25th socket
  cell and selected origin. The second batch fits cell spacing and all four
  origins. Primary artwork/materials and selected frame geometry are
  corrected; repeated/margin printing and scan echoes remain.

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
