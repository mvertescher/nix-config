[All workstreams and current status](../TODO.md). File paths in these
records are relative to the crate root. Dated notes retain their original
reasoning; later completion entries supersede earlier open-item lists.

## Neomil dashboard fidelity correction plan (2026-09-14)

Source: `images/img-07-dashboard.png` (3840×2160, Behance asset
`3fc4ef118663901.60e5fa6a7f2f7.png`, screen #60). Compare against
`docs/neomil/dashboard-trace.svg` at the same resolution with repository
fonts. The audit confirms the broad layout, main tile red #ef3333, labels,
panel outline and footer framing are substantially correct. Preserve them.

Fresh baseline: G1i matches 19/30 counted source shapes, 94% of source
bounding-box area, median matched-center error 1.1 canvas px. This is not
94% visual fidelity: bbox IoU only needs 0.30, extracted text is not
compared, and unmatched candidate shapes do not fail the default gate.
Grid layout/edge correlations are 0.960/0.716; weighted ink IoU is 0.66.
Use source crops and measured contours, not gate score alone, for review.

- [x] **Pass 1: replace the menu glyph placeholders.** The two reusable
  cells previously drew identical 43×36 rectangles. Measure all six source
  glyph groups, including their rotated code artwork, numbers and circular
  marks; preserve any actual variation. Draw vector geometry, update the
  dashboard's component examples and transcribe it into the era table.
  Hover/press must retain the corrected geometry while changing only the
  existing inferred inks. Do not invent a QR payload or substitute an icon.
- [x] **Pass 1: restore GO HOME content and panel material.** Replace
  eight bars with the source's five-plus-four actual lines, matching line
  ends, type size and placement; restore the vertical brand lettering.
  Measure the large fill/scanlines instead of keeping the uniform purple
  wash: the empty-body sample at (1150,610)..(1300,640) has median RGB
  (33,7,7), versus the previous trace's (59,15,20). Keep the measured frame,
  edge bar, maker's mark and boot clip unless the source proves a fault.
- [x] **Pass 2: correct tile contour details.** Measure the vertically
  clipped side tips on inset outlines (VEHICLES right; other cells left),
  their stroke widths, and visible offset red fragments around the tiles.
  Separate actual layered geometry from photographic echoes; neither
  implies an additional hover state. Preserve centres and outer scale.
- [x] **Pass 3: restore header and small chrome.** Match the thin wide
  `next` logotype, TECHNOLOGY subtitle, code-tape text, LEVEL/T1 proportions
  and badge fills. Replace the wide empty margin boxes with measured 1/2
  chips and adjacent fragments; check DESCRIPTION's invented left chamfer.
  Replace vertical margin bars with their actual text and symbols.
- [x] **Native footer caption cap heights:** AD matches the source's
  vertical boxes using native sizes/baselines, retaining nominal widths,
  foreground roles and echo offsets. Whole-line RGB/core overlap improves;
  the final COLONIZATION section retains a small core-overlap regression.
  Code, divider and frame are unchanged. Native states, both gates, 286
  Rust tests and the full 22-check/27-exact-case run pass. See
  [round fifteen](../docs/reference-svg-round15.md). Exact glyphs and
  photographic printing remain in pass 4 below.
- [ ] **Pass 4: finish surface/typography fidelity.** Measure remaining
  scanlines, noise, glow and offset echoes. Explicitly distinguish design
  material from presentation residue before marking anything `photo`.
  Refine text widths/weights and localized fills against source crops;
  preserve the correctly sampled main red and broad layout.
- **Standing validation for each pass.** Review fresh source→SVG crops,
  update the matching `components.svg` excerpts and source documentation,
  transcribe into `src/eras/neomil.rs`, and inspect SVG→iced captures,
  including hover/held states and the panel's boot clip where affected.
  Update only intentionally changed dashboard goldens after visual review;
  run relevant Rust tests and `./check`. No gate-threshold changes to
  compensate for an unfinished trace. Live desktop verification remains
  separate from the headless checks.

Pass 1 implemented and visually reviewed at rest, in upper/lower-row hover
and held previews, and at 0.15 s of the panel-open clip. All six source
matrix/code/symbol groups are vectors; Rust uses compound paths so rotated
square modules survive the canvas renderer. The GO HOME panel now has all
nine lines, both vertical brands, and measured local material. Rust's 202
tests pass. G1i remains 19/30 shapes / 94% area; ink placement improves from
0.66 to 0.74. G2i passes at 36/41 shapes / 99% area. Numeric gates still need
visual review: G2i also passed an intermediate capture with missing matrix
modules, which was caught and corrected before accepting the goldens.
Only the Neomil and fallback dashboard goldens change; their pixel changes
are confined to the six glyph groups and GO HOME panel. Full `./check`
passes all 19 checks, including all 25 golden cases; the rebuilt Neomil and
fallback dashboards match their reviewed goldens exactly. That completed
pass 1; the following batch supersedes its remaining-work status.
The earlier statement that no concrete feature work remained was incomplete:
these are source-fidelity corrections with local material.

Next batch integrated (2026-09-14) after three isolated agent proposals:
pass 2's six fitted insets/1px outlines/solid tabs, pass 3's header and margin
chrome, and pass 4's measured labels and dashboard-only composite ground.
Shared other-screen tables, menu bounds, state behavior and main tile red
are preserved. [Detailed measurements](../docs/neomil/dashboard-fidelity.md)
record source evidence and limits. All 203 Rust tests pass. G1i passes at
18/30 shapes / 89% area with ink IoU 0.79; G2i passes at 18/24 shapes /
83% area. The inventory drops were reviewed: corrected thin insets and
palette segmentation change how visible geometry is grouped, without
omitting drawing. The detailed report records that evidence. Headless
rest, upper/lower hover/held and 0.15s panel-open captures are reviewed.
Scaled previews exposed antialiasing seams between background strips;
full-canvas masked rows remove them, with a fractional-scale opacity
regression and fresh reviewed previews. The full `./check` rerun after
that correction passes all 19 checks, including all 25 golden cases.
Only the Neomil and fallback dashboard goldens change. Pass 4 and live
desktop verification remain open.

The next pass-4 batch is integrated (2026-09-14):

- [x] **Secondary lettering:** source-derived paths for TECHNOLOGY,
  code tape, both margin codes, the KIROSHI wordmark and chip digits.
  The O has a curved inner stroke; the earlier "slashed O" reading was
  too strong. The tiny leading tape wordmark remains source vector art
  because its letters are not confidently readable.
- [x] **Local panel and badge fields:** all five badges have measured
  two-dimensional color variation and scan modulation; the GO HOME field
  now also varies horizontally. Main tile red and flat interiors stay
  as measured in the source.
- [x] **Connected panel echoes:** replace the four solid edge rectangles
  with two softened, scan-modulated contour and side-bar copies. Their
  measured local profiles improve without changing the primary frame.
- [x] **Six menu-label echoes:** two dim, scan-modulated copies now sit
  beneath each unchanged primary label. A shared fit from four strong
  labels also improves the two held-out labels in native source crops.
- [x] **Tape/chip local faces and validated ink:** five independently
  sampled bright faces/fragments use #fb3535. Tape-code and chip-1 ink
  gain measured periodic RGB variation inside their existing contours.
- [x] **Header printing echoes:** two softened copies for the logo,
  three captions, five frames, five LEVEL/tier groups and horizontal rule.
  All sixteen local held-out masks improve; weak T1-frame and selected-T2
  printing gains remain documented rather than treated as exact recovery.
- [x] **Tape/chip exterior echoes:** five bright silhouettes now have two
  softened, scan-modulated copies beneath their unchanged primary faces.
  All five actual SVG held-out comparisons improve; remaining tiny-fragment
  and green/blue residuals are documented in the fidelity report.
- [x] **GO HOME body and maker-shape echoes:** nine body-line copies pass
  both whole-line holdout folds; independently translated M/dot copies pass
  both spatial holdouts. Both use the existing panel opening clip.
- [x] **Matrix artwork ink and overlapping copies:** native crops of all
  six matrix/code groups show localized variation, strongest in VEHICLES,
  LOCATIONS, WEAPONS and PRODUCTS. A uniform 1.984934px row cycle explains
  little of the red-channel residual in eroded glyph cores. Separate local
  coverage/overlapping copies from periodic ink before selecting a model;
  preserve the measured module occupancy, code paths and flat tile fills.
  Completed 2026-09-21 with silhouette-clipped overlapping coverage and
  a smaller periodic ink term. Both spatial/cycle holdouts and separate
  code/circle comparisons improve for all six; exterior SVG pixels remain
  unchanged. The source does not identify a unique authoring recipe.
- [x] **Margin printing echoes:** fit the source trails around both JHN
  code groups and the KIROSHI wordmark, retaining the accepted primary
  contours. Source crops: (41,456)..(54,588), (1540,551)..(1558,671),
  and (1540,722)..(1559,770), respectively, in 1600×900 coordinates.
  Completed 2026-09-21: two independently fitted outward copies per
  family, retaining primary paths; all actual SVG holdouts improve.
- [x] **GO HOME heading primary fit, then echoes:** the current bright
  envelope begins about 4.17px too far right and is 2.08px narrower / 0.83px
  shorter than the source. Fit origin, size and weight locally; the body
  echo model worsens the heading comparison and must not be reused blindly.
  Completed 2026-09-21: fitted Rajdhani Bold origin/size/stretch and
  separate copies pass both GO↔HOME held-out comparisons.
- [x] **Maker primary contour, ink and microtext:** preserve broad placement
  while fitting the source's rounded shoulders/notches/corners and interior
  scan modulation. Both microtext lines need lighter strokes; PRECISION
  LIQUID also needs more width, whereas POLYMER MUSCLE is already close in
  width. Fit each run before its echoes. Regenerate the new M/dot echo
  union after correcting the primary shape. Completed 2026-09-21 with
  rounded M/dot contours, independent interior ink and regenerated echoes.
  The microtext premise needed correction: measured geometric monospace
  strokes, including barred I, replace Rajdhani. Remaining detail is below.
- [x] **Vertical-brand echoes:** both small rotated runs have independent
  source fits, completed 2026-09-21. Their finite softened copies remain
  before the primary frame under the existing panel opening clip.
- [x] **Maker microtext trailing copies and edge softness:** each geometric
  run now has its own softened trailing copies under the existing panel
  clip. Native source RGB RMS improves 21.19→12.18 and 23.02→12.82; all
  changed pixels are inside these two runs. Fractional and opening review,
  fidelity gates and the N full check pass. Exact horizontal scan
  striations remain a fine-printing limit; the unknown original recipe is
  not claimed. See [measurements](../docs/neomil/maker-microtext.md).
- [x] **Fit the tiny leading tape-mark ink.** Eight existing paths receive
  a local ink correction. Exactly 99 native pixels change inside their
  footprint; source RGB MAE falls 20.49→10.07. Independent holdouts,
  fractional/opening review, both gates and the T full check pass. Exact
  contours and edge softness remain approximate.
- [x] **Integrate the reviewed chip-2 ink correction.** V's local lateral
  gradient and echo-phase model passes all three native regions: aggregate
  core error falls 15.73→11.95 RGB levels, and edge error 20.05→17.43.
  Fractional rest/opening review, both gates and the full Z check pass.
  Earlier modulation and constant-color trials failed
  independent holdouts and remain rejected. Exact photographed printing
  softness is not claimed; see the
  [eleventh checkpoint](../docs/reference-svg-round11.md).
- [ ] **NM1 — Finish footer glyph and faint-print fitting.** The sixth
  reference batch fits each primary run independently: native SVG cap
  heights now match the source's 6.25px code and 5.83px captions, preserving
  their narrow widths. Z corrects Iced's code height/baseline independently
  of the already-fitted SVG: the first `6` now matches source bounds, and
  all 13 glyph comparisons improve. Production parity, fractional states
  and both gates pass. Exact contours and caption differences remain.
  Two translucent frame/text copies replace
  the single sharp offset frame, improving eight local crop errors in SVG
  and Iced. Fine horizontal striations, edge softness and exact repeated
  lettering remain open; do not treat the broad copies as an exact fit.
  The frame/divider/code are preserved; copies follow the foreground role
  for custom palettes and add no software-rendered cache layer. See the
  [footer measurements](../docs/neomil/dashboard-footer.md).
  AY rejects additive and replacement horizontal profiles: 4K gains do
  not transfer to the 1600px bottom/text controls. Removing secondary
  ink also exposes primary-edge misregistration; isolate that geometry
  before another secondary-ink fit. No trial artwork is integrated. See
  [the audit](../docs/neomil/dashboard-footer.md#ay-secondary-frame-audit-2026-09-30).
  AZ adds source-supported primary bottom coverage, improving all changed
  source/native pixels at three sizes while keeping top/text controls exact.
  Nine paired state/fallback checks pass. The old lower bright overshoot,
  stronger upper core, top width and fine printing remain unresolved;
  clipping alternatives fail fractional controls. See
  [the bounded correction](../docs/neomil/dashboard-footer.md#az-bottom-edge-coverage-2026-09-30)
  and [round thirty-seven](../docs/reference-svg-round37.md). Three-size
  production/fallback and packaged parity, both gates, 292 Rust tests and
  the full 22-check/27-case run pass. A subsequent height-only SVG trial
  also fails all fractional bottom controls; no native trial is accepted.
  NM1 stays open.

  BA's source-derived one-row top band and inner-rim join improve the SVG at 4K,
  1600 and fractional size, but the guarded native candidate is rejected. At
  1600 it paints row865 to R≈180 where the source is ≈154, creating false R>170
  core coverage; all three straight-edge train/holdout F1 controls fall
  1.000→.667 despite lower RGB MAE. No top band, component caption, golden or
  production geometry changes. BB's read-only side audit finds a left-specific
  4K core phase difference (source x2902–2904, native x2901–2903) and aligned
  right core; widening both sides is unsupported. AZ's accepted bottom remains,
  including its unresolved lower-row overshoot and strict upper-core excess.
  Fine secondary striations and live desktop behavior remain open. See [the
  footer
  record](../docs/neomil/dashboard-footer.md#ba-top-edge-and-side-audits).


  BF resolves BA's native top-band threshold failure with a separately
  calibrated .80 foreground-role band. All fixed three-size RGB/F1 controls
  improve or tie; one fractional corner-pixel loss remains. The source SVG
  and component use their measured full-alpha band. Nine paired state/fallback
  cases, production/packaged parity, both gates, 293 Rust tests and all
  22 checks/27 first-attempt visual cases pass. Only the two dashboard
  goldens change, by 142 pixels each. The old lower overshoot, side phase,
  exact lettering and secondary striations remain open. See
  [the bounded top correction](../docs/neomil/dashboard-footer.md#bf-calibrated-top-edge-coverage-2026-09-30)
  and [round forty-one](../docs/reference-svg-round41.md).

  BT integrates the missing inner-left coverage as a separately calibrated
  .88 foreground-role strip. All fixed three-size regional RGB/F1 controls
  improve or tie; the full-opacity trial is rejected for its 1600 false core.
  Eight state/fallback pairs preserve locality; selection/held fixtures
  have no distinct visible feedback. Exact outer-left/core/striations remain
  open. Production/package parity, both gates, 294 Rust tests and all
  22 checks / 27 first-attempt visual cases pass. The two dashboard
  goldens change by 23 pixels each. Combined verification is recorded in
  [round forty-three](../docs/reference-svg-round43.md).

  CF narrows only the straight native outer-left primary rim, improving
  38 4K pixels while both smaller frames remain exact. Three-size source
  controls and six actual-Dashboard pairs pass. Production/package parity,
  both Dashboard gates, 294 local/Nix tests and full 22-check/27-case
  verification pass. Preserved joins, remaining shoulder error and faint
  striations keep NM1 open. See
  [round forty-five](../docs/reference-svg-round45.md).

The [printing follow-up audit](../docs/neomil/dashboard-printing.md) records
the 2026-09-21 proposals, masks, actual-render measurements and limitations.
Pass 4 stays open for the two local tasks above and the background material
below; end-to-end and live checks are recorded separately.
- [x] **App primary-ink mapping:** the unmodified reference dashboard
  projects its foreground from shared #de2e2e to source/SVG #ef3333.
  The full palette and explicit variant determine eligibility; custom
  roles, variants, other screens and fixed interaction inks are preserved.
  Six new regression tests cover default/forced loading, no-config fallback,
  variant provenance, customized palettes and screen isolation. Combined
  capture and repository validation for this batch are recorded below.
- [x] **Characterize fine background noise:** residuals vary by channel,
  are absent from opaque tile interiors, and two clear patches totaling
  48,096 native pixels are identical across all four source screens,
  including their noise. This supports a shared fixed background asset.
- [ ] **Recover the fine background material:** the shared source pixels
  do not reveal the original asset or its generating process, or whether
  it belongs to the UI or its presentation. Those authoring inputs are
  needed for a faithful reconstruction; the other local screen exports
  are not independent noise samples. No arbitrary grain or `photo` tags.

All three agents' proposals and their integrated SVG/Rust counterparts
were reviewed. Detailed source masks, local error measurements and limits
are recorded in [the fidelity report](../docs/neomil/dashboard-fidelity.md).
All 203 Rust tests pass. Final G1i passes at 14/30 shapes / 81% area;
G2i passes at 12/18 / 90% using the release app. The report explains the
palette-grouping changes and retained unmatched details. Reviewed native
and fractional rest/hover/held captures and 0.15s boot frames retain the
corrected geometry. The release capture matches the refreshed Neomil
golden byte-for-byte; only Neomil and fallback dashboard goldens change.
The full repository check passes all 19 checks, including all 25 golden
cases. Pass 4 and live desktop verification remain open.

The next printing batch is integrated: all three agents' accepted
tape/chip, menu-label and header proposals now match in both SVG sheets
and Rust. All 203 Rust tests pass. G1i passes at 16/30 shapes / 82% area,
ink IoU 0.84; G2i passes at 17/22 / 98%, ink IoU 0.65. Native and
fractional SVG/app rest/hover/held and 0.15s boot captures are reviewed.
Exactly 27,280 app pixels change from the preceding batch, all within
the intended header/rule, tape/chip and label regions. Neomil and fallback
release captures agree; only their two reviewed dashboard goldens change.
Concurrent debug captures at 15s were blank, while a longer debug capture
matches the release capture at the normal 4s settle byte-for-byte.
The full repository check passes all 19 checks, including all 25 golden
cases. Fine background material still needs its original asset or authoring
recipe; pass 4 and live verification remain open.

The primary-ink/detail batch is integrated after three isolated agent
proposals: the dashboard reference foreground mapping, five tape/chip
exterior echo families, and nine body-line plus maker-shape echo groups.
All 209 Rust tests pass. G1i passes at 17/30 shapes / 82% area, ink IoU
0.86; G2i passes at 17/23 / 98%, ink IoU 0.70. Six app cap patches now
match source/SVG #ef3333 exactly. Native/fractional rest, hover, held and
0.15s opening captures are reviewed, and both SVG sheets are updated.
Reference and fallback release captures agree at the normal 4s settle;
only their two dashboard goldens change. The full repository check passes all 19 checks, including all 25 golden
cases. Matrix/margin printing and heading/maker primary measurements
above make the next local tasks concrete; no global effect or fine noise
recipe is claimed. Pass 4 and live verification remain open.

The recovered printing batch is integrated and staged (2026-09-21): six
bounded matrix/code ink fits, three margin and two vertical-brand echo
families, corrected heading, rounded M/dot geometry with independent ink,
regenerated maker copies and geometric microtext. All 209 Rust tests pass.
Dashboard G1i passes at 18/30 shapes / 83% area / ink IoU 0.87; G2i at
19/30 / 95% / 0.71. Native/fractional state captures preserve target-only
feedback and the 0.15s opening clip. Close-up review moved the microtext
strokes to the panel software surface, reducing SVG→app crop RMS from
17.79 to 6.46. Reference/fallback captures agree; all six caps remain
#ef3333. Two dashboard goldens are refreshed, plus the missing golden for
the previously committed Login-slot fix (a 4×20-pixel bounding box).
Login G2i passes at 26/31 / 96%. The full repository check passes all 19
checks and all 25 golden cases. The two next local tasks are maker
microtext trails/edge softness and chip-2/tiny tape-mark ink; original
background material and live verification remain open. Measurements and
limits are in [the printing audit](../docs/neomil/dashboard-printing.md).

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

  **Closed 2026-09-03 by the `Layout` fold** (see the [pipeline record](design-pipeline.md)):
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
    stay a shared cross-era layout — see the "Layout" entry in the [pipeline record](design-pipeline.md) — and
    only then rebuild. Do not touch `tests/golden/` until that is settled.
