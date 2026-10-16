# Neomil store correctness

Audit: 2026-09-21. Reference `images/img-09-store.png`, 3840×2160;
trace `docs/neomil/store-trace.svg`, 1600×900 design units. Catalog
content is intentionally static; no network/payment backend is specified.

## Runtime and geometry

- [x] **Restore standalone animation subscription.** The launcher previously only
  subscribed to navigation, so the shelf-opening clip never advanced.
  An unfrozen five-second capture has no product cards. A temporary
  subscription-only correction makes live output pixel-identical to the
  frozen reference. Batch navigation with `Store::subscription()` and
  add an unfrozen startup matrix case. The hub already forwards ticks.
- [x] **Match hit regions to selected and cropped card geometry.** Previously all
  plates used284×462 hit boxes despite the selected extension to y800.
  `(900,700)` is visible selected detail with no target; `(1580,350)` is
  hidden margin but targets the fourth card. Use current selection and
  persistent viewport bounds for pointer hits and keyboard centers;
  preserve release cancellation and all other era behavior. Temporary
  opening wipes must not become permanent navigation bounds.
- [x] **Retain the fourth-card crop through selection and feedback.**
  Selecting it previously substituted full GROWN and painted through the x1557 cut
  into the margin. Apply persistent viewport drawing and hit constraints
  to rest/hover/held/selected states, including fractional scales. The
  original only specifies selected card2; other selection states must
  preserve shelf consistency without claiming source-still evidence.
- [x] **Correct selected wash and open fourth-card edge.** The upper
  wash was an unmasked282×365 rectangle painted over the frame, whereas
  the SVG follows its chamfer, side step and diagonal lower-right edge.
  Keep material inside the contour and the frame visible. CARD4_EDGE
  is deliberately open but `shut_path` added an incorrect vertical edge
  at x1557; preserve its open shape in all feedback states.
- [x] **Restore selected lower-right source chamfer.** The source frame
  runs approximately(1038.5,779.5)→(1021,797.1), an18px cut. The SVG
  and Iced end at square(1039,800). Update trace and implementation
  together, retaining the tall selected card and all details inside it.

- [x] **Correct ordinary primary frame geometry before fitting echoes.**
  FH separates ordinary card contours from their calibrated contents and
  corrects top/bottom, per-card sides and fan joins, including the previous
  card-3 SVG/native discrepancy. SVG/component/native fill boundaries agree;
  selected geometry and the fourth cut stay fixed. Source comparisons improve
  coherently at all three sizes; the fourth-left 1600 L1 loss remains explicit.
  Sixteen state pairs, both Store gates, 305 packaged tests and full 22-check/
  28-case verification pass. Fresh package pixels match at all three sizes.
  See [round sixty-nine](../docs/reference-svg-round69.md).
- [x] **Refine remaining ordinary primary frame coverage.** Geometry,
  ordinary tops and left edges are verified, including inferred ordinary
  second-card geometry. Continuous outlines preserve semantic inks, material
  origins, selected art, held suppression and the fourth card's distinct
  bright held border and cut. Round seventy-two passes all ninety three-size
  source comparisons in each renderer, seven state pairs, three custom cases,
  both Store gates, 305 packaged tests, full checks and fresh package parity.
  Round seventy-nine completes bottom coverage below. Earlier combined
  bottom thinning is rejected for repeated smaller-size losses. The isolated
  lower-vertical thinning trial also failed
  because the frame contributes to the visible bar-side edge. The later
  coupled stem/bar correction, accepted with the frame impressions in
  [round seventy-four](../docs/reference-svg-round74.md), resolves that fan
  regression with supporting-line geometry; four tiny junction-row losses
  remain explicit. Do not fit an arbitrary cutoff, cap or taper.
  Source blur, color space and ground separation prevent unique physical
  width inference. IM rejects a Dashboard bottom donor; IN validates two
  predeclared Login exterior patches. IO's two fixed 4K profiles show repeated
  overbright primary shoulders and mixed echo residuals. IP re-evaluates the
  frozen 0.6 bottom with accepted impressions: whole bottoms improve at all
  three sizes, but C1/C3 upper-shoulder losses repeat at 1600 and 4K, with a
  small 1600 cut-exterior loss. The candidate is rejected; no native port or
  new width/phase sweep follows. KI verifies four clear exterior donor strips
  and nearby unchanged face-tint predictions, retaining the contaminated
  fourth-card control and localized printing residual. The hidden shoulder
  backing remains a model assumption. A further change needs supported
  coverage, registration or paint evidence with explicit uncertainty and
  separate controls. KL supplies a conditional three-channel coverage model
  consistent across four clean strips. One C1-derived SVG box approximation
  then passes all 106 KM regions at three sizes, retaining twelve isolated
  corner/cap pixel losses. KN's native transfer improves every changed
  pixel at all three sizes without region losses. [Round seventy-nine](../docs/reference-svg-round79.md)
  integrates it after sixteen state cases, component review, three-size
  package parity, both Store gates, 311 packaged tests and all 22 checks/28
  first-attempt matrix cases. It adds no primitive, text work or effect pass.
  GO's historical rejection remains valid; this correction uses the new
  conditional coverage evidence rather than another width sweep.
  [Round seventy-three](../docs/reference-svg-round73.md)
  corrects the fourth permanent cut from x1557 to x1558.3 after independent
  top/bottom source controls. The coordinated face/content/material/hit-cut
  transfer passes native state/custom controls, both gates, 305 packaged
  tests, full checks and three-size package parity. Whole-card comparisons
  improve at every size; fractional bottom-endpoint L1 +1.50% / RMS +1.25%
  remains an explicit local residual. Remaining fine printing stays separate. See
  [round seventy-two](../docs/reference-svg-round72.md) and the
  [source plan](../docs/neomil/store-ordinary-frame.md).

## Source fidelity

- [x] **Restore the shared ground.** Clear source patches are identical
  to dashboard. Reusing the existing source-fit model improves native
  clear-ground RGB RMS7.285→1.736 across2,262,312 pixels, corroborated
  by two spatial holdouts. SVG and Iced use identical controls; see
  [measurements](../docs/neomil/login-store-material.md).
- [x] **Restore broad card/nav materials.** Translucent ordinary cards
  preserve the source blue ground; selected upper/lower and navigation
  fields use measured two-dimensional sRGB ramps.135 held-out native
  patches have source RGB RMSE.52–.81, with SVG/native channel differences
  at most one. All 20 category/card combinations preserve hit geometry,
  opening and permanent card4 clipping; custom themes retain semantic
  fields. Native state review, both gates and the K1 full check pass.
  Fine scan modulation remains in its own task below. See
  [material measurements](../docs/neomil/store-material.md).
- [x] **Fit primary store typography and footer.** P–T independently fit
  ordinary/selected titles, subtitles, stats and values, with native baseline
  calibration. Navigation and boxed/unboxed footer fits preserve custom
  palette semantics; reference bright ink uses #fb3535. Source/SVG/native
  and fractional held/custom/cropped review, both gates and the T full
  check pass. Exact glyph contours and fine printing remain in their scoped
  tasks below. See
  [typography measurements](../docs/neomil/store-typography.md).
- [x] **Restore omitted SVG printing and weapon details in Iced.** Rotated
  BETTERLIFE TEC/PETROCHEM, margin runs, MASURAO contours, rifle rail/grid/
  screws and other primary details are now present. Label tracking and
  socket weight are restored. Native/state review and the N full check
  pass; remaining source-shape/material refinements are scoped below.
- [x] **Correct the small margin/footer code to `0B`.** The original store
  margin and unboxed footer, and the login margin, say `0B CP`; all three
  inherited `08 CP` in the trace and runtime. Source inspection confirms
  the B's straight stem and two bowls. Rust/reference/component literals
  are corrected. The locally fitted native store margin improves mask
  overlap .176→.398, with bounds within one source pixel. Store/login
  native review, relevant gates and the full T check pass.
- [x] **Integrate the reviewed selected-frame/side-printing correction.**
  U/V correct the selected top/chamfer, right edge/fan and PETROCHEM box
  independently while preserving the aligned left edge and icons. The
  native top now has one bright row, matching the source; three independent
  top-strip errors fall from about 61.3 to 20.0–21.1 red levels. Native
  rest, ordinary/selected/fourth-card press, custom-palette and opening
  review preserve clipping and feedback. Local gates and the full Z
  check pass: 275 Rust tests, 22 repository checks and 27 exact visual cases. See the
  [measurements](../docs/neomil/store-selected-frame.md). Fine edge echoes
  remain separate from this bounded geometry correction.
- [x] **Replace the misplaced small MASURAO margin word and long rule.**
  AB replaces the overlapping font with six compact source-derived stencil
  glyphs and a short slash, preserving the accepted CJK. Native source RGB
  error improves on each letter; CJK overlap improves .485→.950 because
  the old overprinting is gone. All 3,054 changed 4K pixels lie in the
  margin. Fractional selection/held/opening and custom inks pass review,
  both gates pass, and the full check passes all 22 checks/27 exact cases.
  This closes primary placement/contours, not the O/plaque material below.
- [x] **Trim the margin plaque's unsupported rectangular extension.** AC
  restores its source-supported slanted leading edge. All 126 changed 4K
  pixels lie on that edge; leading RGB error falls 20.20→14.52 and bright
  intrusions into dark source pixels fall 88→4. Ten native captures preserve
  selection, feedback, opening, custom ink and clipping. Both gates, 286
  Rust tests and all 22 repository checks pass, including 27 exact visual
  cases. See [round fourteen](../docs/reference-svg-round14.md).
- [ ] **Refine margin O/plaque ink and the second-A junction.** AB's
  primary-word correction left approximate hatch material and a partly
  merged A2/plaque junction. AC fixes only the leading boundary: native
  plaque RGB error is now 23.09 and dark-gap intrusions 6, while the O
  counter and A2 are unchanged. A wider clip and uniform letter dimming
  fail local controls and remain unapplied. Preserve the accepted primary
  glyphs, slash and CJK; further changes need source-supported local fits.
  AD's accepted central hatch-width correction improves native plaque RGB
  error 23.09→22.00 and source-core matches 275→312. A2 is unchanged;
  one additional dark-gap intrusion (6→7) remains explicit. Ten native
  captures, both gates, 286 Rust tests and the full 22-check/27-exact-case
  run pass. See [round fifteen](../docs/reference-svg-round15.md).
  This bounded ink improvement does not close the full residual.
  AE adds a narrow semantic-background counter gap: five native 4K
  pixels change, all in a source-dark column, reducing A2 intrusions
  15→11 while preserving all 65 bright-core matches. The broad junction,
  O ring and plaque remain unchanged. See
  [round sixteen](../docs/reference-svg-round16.md) for integration.
  AK widens a short inner-O opening: three native 4K pixels improve,
  the 1600 golden stays identical, and five fractional states alter only
  one O pixel while preserving custom ink and feedback. Both gates and
  full checks pass. This has limited source evidence and no independently
  changed native holdout; the broader O/plaque/A2 task remains open.
  Uniform shifts/ink and upper gradients are rejected after dark-core
  losses. See [round twenty-two](../docs/reference-svg-round22.md).
  CJ adds the source-supported lower A2 split, preserving the bright
  crossing. Every changed native pixel improves at three sizes and six
  fractional state pairs pass. SVG/component geometry is synchronized;
  small SVG soft-edge tradeoffs remain explicit. Production/package, both
  gates, 295 Rust tests and full repository checks pass; the broader ink
  task stays open. See
  [round forty-six](../docs/reference-svg-round46.md).

  ED measures a coherent upper-plaque source grating with unequal local
  pitches 1.288/1.762 instead of the current 1.5/1.5. Six adjacent blank
  Login/Store rectangles are exact. One source-phase, original-width SVG
  trial improves broad plaque error at 4K/direct1600/fractional size, but
  loses leading-edge and A2 dark detail; it remains unapplied. Preserve
  the measured grating and distinguish its upper phase from junction
  coverage before another complete hatch. Earlier swept controls are not
  independent holdouts. See
  [round fifty-seven](../docs/reference-svg-round57.md#neomil-margin-plaque-grating).
  EE attributes eight lost dark matches to the lower junction; four are
  clip-edge pixels and none lies inside opaque A2. The upper fitted phase
  is weak in the lower source windows, without proving a physical cutoff.
  Opaque A2 brightening is SVG-specific: its A2 and hatch colors differ,
  while native uses `Ink::Fg` for both. This does not rescue the rejected
  candidate or establish native loss. A source-supported lower boundary
  or primary/material layer is needed before porting the grating. See
  [round fifty-eight](../docs/reference-svg-round58.md#neomil-plaque-junction).
  EF's source-only row/window probe confirms that the frozen upper phase
  varies across the lower junction, where letters and printing overlap.
  It does not identify a physical cutoff. Do not repeat an arbitrary
  lower crop or dark-pixel knockout; obtain a continuous visible stencil
  or separate layer/exposure constraint before porting the grating. See
  [round fifty-nine](../docs/reference-svg-round59.md#neomil-plaque-lower-boundary).

- [ ] **Refine repeated/margin printing.** Primary MASURAO, KIROSHI,
  certification marks and rifles are source-supported vectors. Remaining
  dim repeated-mark fields and faint copies need local fitting. The
  ordinary CC35 compliance lines also retain a primary typography mismatch:
  fresh source/SVG/package contacts show shorter, dimmer rendered runs
  with narrower glyphs and repeated placement differences on cards 1/3/4.
  Both lines are implemented. JV verifies the two source lines are about
  6–7 physical pixels lower at 4K; complete source widths are 608/442
  versus native 527/386 on cards 1/3. JW's +2.5-design-y SVG correction
  improves all 54 complete-line all/bright/dark comparisons at three
  sizes; 67 local band/gap exceptions remain recorded. Alpha/exterior
  pixels are exact. JX's same native coordinates improve all 54 complete-line
  comparisons, retaining 93 local exceptions. [Round seventy-eight](../docs/reference-svg-round78.md)
  integrates this vertical correction after sixteen state cases, three-size
  package parity, both Store gates, 311 packaged tests and all 22 checks/28
  first-attempt matrix cases. Fonts, ink, horizontal layout, drawing routes
  and primitive counts stay fixed. Width, contours and horizontal placement
  remain open; no new text effect is added. KB's exact-string probe verifies
  24 cases/1,008 glyphs with correct bundled bytes; Auto/Basic agree and
  Advanced differs below 0.0001 pixel. Matched word/O controls expose
  contour differences, so the aggregate width ratio does not justify a
  blanket stretch. KC's isolated third-card x11→9.1 SVG trial preserves
  fonts/ink, but fails three complete-line bright/dark controls at smaller
  sizes despite broad gains. Astra verifies all 972 records and retains
  79 local exceptions. Reject the standalone shift; no native port or
  retuning follows, and the full caption font/layout discrepancy stays open.
  KD–KF verify the pinned generic FreeSans Regular's metrics and actual
  glyph selection, but its unfitted face/cap swap fails 20/54 full-line
  controls and first/third O counters. Reject that swap. A shared source
  word-origin/tracking model reduces held-out edge RMS to 2.024 pixels
  at 4K, retaining up to 4.066 pixels of error. KG's complete SVG trial
  improves 44/54 full-line controls but fails ten dark subsets and the first
  O hole. Reject the transfer; all source/context losses remain recorded.
  Explain the remaining word/contour and printing differences before
  another fit. Any later tracked native transfer also needs shaping and
  cost evidence. Font identity remains unknown; the cached text route stays intact.
  See the
  [compliance study](../docs/neomil/store-typography.md#ordinary-compliance-lines--jv).
  The margin O/plaque residual is the separate correction above. T fits PETROCHEM tracking; a shared
  BETTERLIFE shift/shear is not source-supported. V corrects selected side
  placement as recorded above. Preserve the accepted primary paths
  and their feedback colors; do not reconstruct the main artwork again.
  IW's fixed source/SVG/current-package crops expose a separate third-card
  side-label registration question: the first/third content origins differ
  by 659 design units, while their corrected frame origins differ by 657.1.
  IX/IY independently measure the source box edges and boundary-clean
  BETTERLIFE at the same 1577-pixel repeat. IX's initial word window included
  neighboring frame ink and is superseded. JA's frozen −1.9 design-unit
  whole-label trial improves BETTERLIFE at three sizes but worsens the box
  and PETROCHEM at 4K; do not port it. JB/JC correct only BETTERLIFE after
  three-size source checks and ten native state/custom controls. Whole-word
  and all eight bands containing ink improve; the ninth stays exact.
  Box/PETROCHEM, first/selected printing, fonts, advances and frame geometry
  remain unchanged. Local pixel losses and overlapping control boundaries
  are retained in [round seventy-six](../docs/reference-svg-round76.md).
  Packaged source/state parity, both Store gates, 310 Rust tests and all
  22 checks/28 matrix cases pass. Broader photographic printing remains open.
  See the
  [side-label plan](../docs/neomil/store-printing.md#third-card-side-label-registration--iw).
  IZ's independent ordinary-box `Ink::Dim`→`Ink::Fg` trial at unchanged
  geometry worsens all 24 edge/cap regions across three sizes, despite
  brighter source cores. Retain Dim; physical ink/coverage remains separate
  from registration. V's accepted selected-box ink stays unchanged.
  JF/JG/JH isolate complete source/rendered PETROCHEM support and confirm
  an absolute placement discrepancy; the earlier truncated windows do not
  establish a source blocker. JI's single rail-midpoint box/word correction
  improves both whole contexts and all full bands at three SVG sizes.
  Its fractional card-3 lower glyph aperture loses 211 RGB L1, and the 4K
  left-rail dark subset loses 2,746; retain these and smaller subset losses
  in the identical native trial. JK reproduces current packaged baselines
  and passes 265 tests, but both 4K glyph apertures lose (card 1 +12,763 and
  card 3 +3,635 RGB L1), despite whole-context gains. The direct transfer
  is rejected. JM removes boxes and
  BETTERLIFE and confirms that isolated native card 1 still loses 18,616
  RGB L1; this is not solely box overlap. JN confirms actual Iced paragraph
  and outline baseline 0.892 × size with byte-exact bundled Rajdhani under
  all three shaping modes. The scene's 0.84 subtraction puts this rotated
  word 0.936 pixel left at 4K. JO's local +0.39-design-x compensation
  relative to JI reverses both 4K glyph losses and improves both glyph
  apertures at all three sizes. [Round seventy-seven](../docs/reference-svg-round77.md)
  integrates the corrected boxes/text and a permanent baseline regression.
  Thirteen targeted states, three-size packaged parity, both Store gates,
  311 packaged tests and all 22 checks/28 matrix cases pass. Local subset
  losses remain recorded. Shared baseline, accepted BETTERLIFE, fonts, ink
  and cached outlines stay intact; broader photographic printing remains open.
  JT rejects Dim→Fg at the accepted registration as well: 4K whole L1
  gains conceal RMS and dark rail/cap losses, while both cards' whole
  contexts and all rails/caps lose at 1600 and fractional sizes. All 306
  declared subsets and six source contacts are reviewed; 196 nonempty L1
  or RMS exceptions remain recorded. Alpha/exterior pixels are exact and
  266 library tests pass. Retain Dim; no state or production change follows.
  This does not alter the accepted font-baseline correction.
  See [aperture evidence](../docs/neomil/store-printing.md#ordinary-petrochem-aperture).
  AL confirms missing repeated `MAGNUM 650` printing on ordinary cards
  1/3/4. A full shifted copy harms counters; a three-row upper-band copy
  adds unsupported ink in dark gaps. Both are rejected. Fit the repeated
  glyph contours and interrupted rows, freeze before testing other cards,
  and retain counter/gap controls alongside RGB error. See the
  [title-printing audit](../docs/neomil/store-printing.md#al-missing-ordinary-title-printing).
  AM's coherent two-impression model preserves all solid title pixels and
  recovers much of the missing printing. Remaining losses are antialiased
  fringes, a few faint counter pixels and early-row gaps. AN integrates its
  native calibration; retain the full impression and these controls,
  without a counter knockout that would erase real source echo ink. See the
  [AM study](../docs/neomil/store-printing.md#am-interrupted-title-impressions).
  AN's fixed 4K native title errors improve 13.876→12.124,
  12.074→10.076 and 12.932→10.135 on cards 1/3/4. All 11,353 changed
  pixels stay inside ordinary titles, with solid primary and selected
  titles unchanged. Sixteen paired drawing states, both gates and the
  integrated matrix pass. The 43/47/185 worsened true-gap pixels and
  smaller fringe/counter losses remain explicit. Exact phase, softness,
  lower repetitions and other margin printing keep this item open. See
  [round twenty-five](../docs/reference-svg-round25.md), including the
  clipped-mesh correction and separate glyph-reuse performance task.
- [x] **Make margin CJK deterministic.** The source `益荒男` run is now a
  measured vector rather than a fallback-font-dependent text run. The N
  frozen and live store matrix cases both match their reviewed baseline
  exactly, replacing K1's 170 differing fallback-glyph pixels.
- [x] **Fit primary certification and KIROSHI symbols.** Source-supported
  two-tone certification paths and KIROSHI contours retain feedback ink;
  selected native mask IoU is .984/.990. Ordinary/repeated footer printing
  fields and faint copies remain part of the open artwork/echo tasks.
- [ ] **Fit remaining scan modulation and printing echoes.** Keep this
  separate from primary geometry/art corrections. Use actual source
  holdouts; do not add generic glow/noise or relax fidelity thresholds.
  GZ/HA identify omitted frame copies after the accepted primary correction.
  Two fixed affine maps survive separate side/corner continuity checks;
  uniform 1%/2% screen-centered copies fail. HC rejects bright-primary echo
  ink on green-channel direction; HD's darker shared-ink inference depends
  on uncertain background endpoints, including contaminated controls.
  HE validates the same-coordinate original Login donor on first/fourth
  clear top controls; the third-card donor fails. HF yields feasible darker
  effective top ink and improves both withheld columns, but retains a
  repeated waveform error. HJ's fixed source-centroid substitution improves
  ten full top profiles, with small local losses retained. Round seventy-three
  completes the independently supported fourth primary-cut correction.
  HP's fixed complete-contour SVG trial improves all nine whole-frame
  comparisons, but worsens smaller-size first-card lower-stem and third-card
  left regions. HQ distinguishes peak-calibrated maps from vector axes;
  HR's single composed-x trial is mixed and does not remove those losses.
  Neither echo trial is accepted. HS/HV isolate combined primary/echo
  coverage; HW's supporting-line bar union repairs the large fan regression
  with local junction tradeoffs retained. HY validates 513 exterior ground
  pixels against Dashboard. HZ finds narrower literal SVG profiles and a
  conditional constant-ink contradiction across rows. IA confirms actual
  small-size filter collapse; IB's fixed nested profile improves transparent
  alpha, but IC loses intensity on opaque artwork and is rejected. ID verifies
  single-mask color transfer within one RGB count over 105 fixed cases.
  IE's complete candidate improves or preserves all 150 regional checks
  against production, with smaller-size losses against HW retained. IF's
  native rest transfer also passes all 150 regional checks against its own
  packaged baseline, retaining four small raw-row losses. IG verifies stable
  cache reuse and bytes but roughly doubles 4K cold preparation. IH's verified
  byte-equivalent bounded walk reduces that prototype cost 21% at 4K.
  [Round seventy-four](../docs/reference-svg-round74.md) integrates the
  accepted masks and coupled lower-stem/bar geometry after fourteen
  state/custom/control cases and four warm-draw processes pass. All 307
  packaged tests, 22 checks, 28 matrix cases, both Store gates and three-size
  actual-package parity pass. Warm drawing is similar; the added artwork
  retains a cold-preparation cost. This completes the bounded ordinary-frame
  impressions, while remaining scan and glyph printing below stays open.
  Geometry alone does not identify the original ink/softness recipe. See the
  [frame-echo plan](../docs/neomil/store-frame-echo.md).
  DS–DV confirm a missing ordinary `HAND GUN` impression, distinct from
  the accepted title. Common-shift and upper-only trials fail transfer
  or truncate the contour. A complete per-card copy improves averages
  but retains material local dark/bright losses in SVG and actual Iced
  at all three sizes. All opaque primary and selected pixels remain exact.
  A card-1-trained five-phase opacity fit worsens card-3 dark controls;
  none is accepted. The next source-supported task must explain local
  registration/contour and interrupted ink, preserving true gaps rather
  than masking individual failed pixels. See
  [round fifty-four](../docs/reference-svg-round54.md).
  EB rejects removing the two apparently blank card-3 echo rows: their
  source red contrast is low because of the blue photographic ground,
  while removing ink worsens RGB and gap controls at both sizes. The clip
  wrapper itself is RGBA-exact. A next model must validate its local
  ground estimate outside the glyphs before using it to infer missing ink.
  See [round fifty-five](../docs/reference-svg-round55.md#neomil-subtitle-rows-and-photographic-ground).
  EC validates Login-photo ground plus one per-card RGB offset against
  six independent clean rectangles. It supports broader faint ink, not
  a row knockout. Both an additive halo and a softened complete replacement
  improve broad scores but fail local or direct-1600 blank controls;
  neither is accepted. Preserve the donor method and require a new
  source-supported explanation of those local losses before another
  blur/opacity fit. This does not recover the original background asset.
  See [round fifty-six](../docs/reference-svg-round56.md#neomil-subtitle-ground-and-impression).
- [x] **Restore directional socket printing echoes.** AA adds leftward
  card-1, rightward card-3/4 and downward selected-card trails while
  preserving all 25 bright primary cells. Native source RGB error on fixed
  footprints falls 19.76→11.92, 26.46→11.95, 20.09→12.51 and 23.34→14.47.
  Sixteen native captures cover opening, fractional feedback, every selected
  slot, custom palettes and fourth-card clipping. All changed pixels stay
  inside the clusters; source-only states share backdrop eligibility.
  No idle image layer is added. Both gates, 285 Rust tests and the full AA
  check pass: 22 checks, 27 exact visual cases. Exact irregular scan grain
  remains in the separate task above. See
  [round twelve](../docs/reference-svg-round12.md).
- [x] **Correct socket-scatter occupancy and selected placement.** The
  all-reference recheck finds 25 source cells, not the 24 claimed by the
  trace: lattice column 2, row 6 is missing, local `(10.5,24.5)` in `#qr`.
  The selected source top-left cell center is `(782.71,712.71)`; the SVG
  puts it at `(786.5,714.5)`, about 3.8px right and 1.8px down. Preserve
  primary orientation: both source rows use the same pattern, while their
  echoes run in different directions. Update repeated symbols, selected
  position and component/Rust copies together. Do not invent a QR payload.
  The first batch restored25 cells and selected origin; its
  `(782.7083,712.7083)` estimate is refined below using an explicit
  pixel-center convention. See the [first correction record](../docs/reference-svg-fixes.md#neomil).
- [x] **Refine socket cell spacing.** Implemented with measured 3.6667px
  pitch and per-card origins across trace/components/Iced; center RMS is
  approximately 0.10–0.13px for all four symbols. Removed an inherited SVG
  stroke that enlarged cells beyond Iced's 3px fill. See [measurements](../docs/neomil/socket-spacing.md).
  Both gates and all 19 repository checks pass, including 27 visual cases
  at 100.000%. Directional printing echoes remain separate.

## Acceptance and preserved findings

- Inspect source→SVG→Iced at rest, early opening, all card selections,
  lower-detail pointer states, fourth-card clipping and fractional scale.
  Keep component excerpts/source docs consistent. Add meaningful runtime
  regression coverage and a live startup case; refresh only reviewed
  intentional goldens. Run both gates, Rust tests and full repo checks.
- [ ] Live desktop navigation/feedback verification remains separate;
  no GUI launch is authorized solely by headless testing requirements.

Baseline frozen output equals its golden. G1i18/43 shapes,71% source
area; G2i14/36 shapes,79% trace area. All pass despite these defects;
extracted-shape area is not a complete visual correctness score.

Preserve VIDEO/AUDIO/GAMEPLAY/CYBERWARE/CONTROLLER, `SPERAD`,
`+2 MODULES SLOTS`, MAGNUM650/HAND GUN, initially selected card2,
its solid gun shifted14px left, and the corrected fourth-card cut at x1558.3.
The socket glyph is scattered cells, not a QR payload. Shared exact
background texture/authoring remains a separate source-material follow-up.

## First implementation batch

The standalone launcher now batches navigation and animation. Its actual
five-second live capture is pixel-identical to frozen rest; the permanent
`storeLive.neomil` matrix case uses the same golden with an unfrozen clock.

Generic selection branches choose grown/resting plate bounds; permanent
viewports constrain drawing, pointer hits and keyboard centers together.
Temporary motion wipes intersect drawing bounds without hiding navigation
targets. The selected card now responds at(900,700); hidden(1580,350)
does not respond. Fourth-card center isx1491, inside its visible area.
Plate hits retain their existing rectangular semantics, not polygon hits.

The upper wash follows its chamfer/step/diagonal with one-pixel polygon
bands and a single final frame stroke. The fourth edge stays open; the
selected lower corner matches the corrected SVG. The old0.13 painted
ground-cover workaround is removed:0.14 clipping preserves these meshes.
Actual rest/selected/held and fractional captures confirm this. The right
margin has zero changed pixels between resting, fourth-selected and held
states, including 1537×947. Zero-time/mid-opening captures are reviewed.

The batch passes all 249 Rust tests, including selected-bound/navigation,
release cancellation, nested clipping and geometry regressions. Both store
fidelity gates pass: G1i matches 12/43 shapes (61% source area), and G2i
matches 19/29 (93% trace area). The source-area change from baseline is
explained in the material measurements; it does not close the artwork tasks.
All 19 repository checks pass, including all 27 visual cases and the new
unfrozen store startup case. Only the login/store goldens changed in this
batch. All changes remain staged; repeat the acceptance checks for
subsequent visual corrections.

## Socket-scatter follow-up, 2026-09-21

The reference recheck's NM3 correction restores all 25 source cells and
the selected origin in SVG, components and Iced. Native-source/rest
comparisons and both fidelity gates pass. Finer spacing and printing
echoes remain open.
The [correction record](../docs/reference-svg-fixes.md) contains current
measurements and validation: 250 Rust tests and all 19 repository checks,
including 27 visual cases. Changes remain staged and uncommitted.
