# Reference SVG correctness

Initial audit 2026-09-21; corrections through CF and CH component synchronization, 2026-09-30. [All workstreams](../TODO.md) ·
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
The [AD checkpoint](../docs/reference-svg-round15.md) accepts bounded
Neomil plaque ink and native footer heights. Kitsch SC-opacity and
Neo-kitsch EMAIL-grain trials fail local controls and are rejected; their
owning records retain the next investigations. All 286 Rust tests, 22
repository checks and 27 exact visual cases pass; 201 frozen hashes match
the Nix source. The [AE checkpoint](../docs/reference-svg-round16.md)
adds a bounded SC type fit and second-A counter gap. Twenty native captures,
286 Rust tests, four fidelity gates and all 22 repository checks pass.
All 27 visual cases match exactly on their first attempt; 215 frozen file
hashes match the tested Nix source. Exact ink/phase, the broader margin
junction and veneer endpoints remain open. The [AI checkpoint](../docs/reference-svg-round20.md)
corrects locked-login notice ink and narrow-window notice containment.
Both affected gates, 288 Rust tests, 22 repository checks and 27 exact
visual cases pass; all 220 frozen hashes match the tested Nix source.

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
  AJ scratch review rejects global Medium weight and brighter fill:
  Medium loses 6–9 of 30 local spans at each threshold, and every trial
  increases RGB error. Body line endings are already close; avoid repeating
  those global trials. See [the review](../docs/reference-svg-round20.md#next-round-scratch-review).
- [x] **E2 follow-up — Correct the clearance-badge T contours and T1 gap.**
  AR replaces the overly heavy T stems and the digit 1 that closed the
  source-width T1 gap. Digits 2/3/4, badge frames and selection inks remain
  intact. The first native candidate is rejected; the revised source fit
  passes local 4K/small-size review with explicit edge/printing limits,
  thirteen paired states, both fidelity gates, 290 Rust tests and all 22
  repository checks. All 27 visual cases pass first attempt; fresh
  production and packaged frames match the reviewed candidate.
  Two T3 edge losses and fractional T1 loose/T2 strict-core losses remain
  in broader E2 alongside other digit contours and photographic printing.
  See [badge lettering](../docs/entropism/mailbox-badges.md) and
  [round twenty-nine](../docs/reference-svg-round29.md).
- [x] **E2 follow-up — Correct the T3 numeral silhouette.** AT replaces
  the font 3's extra left bars with the source-core contour, preserving
  the accepted T and other digits. Three-size source/native review,
  nine paired states, both gates, 292 Rust tests, 22 repository checks
  and 27 first-attempt visual cases pass. Production and packaged frames
  match the reviewed candidate. Only the mailbox golden changes by
  129 pixels. Other contour and photographic-edge residuals remain in E2.
  See [round thirty-one](../docs/reference-svg-round31.md).
- [x] **E2 follow-up — Correct the T4 numeral silhouette.** AU replaces
  the font digit with one source-supported contour and counter, preserving
  the accepted T. Source/native review at three sizes, eleven paired
  states, both gates, 292 Rust tests, all 22 repository checks and 27
  first-attempt visual cases pass. Production and packaged frames match
  the reviewed candidate. Only the mailbox golden changes by 161 pixels.
  The original digit 2 candidate failed fractional foot coverage; BT’s
  subsequent bounded correction is recorded below. Broader printing stays
  in E2. See
  [round thirty-two](../docs/reference-svg-round32.md).
- [x] **E2 follow-up — Correct the T2 numeral silhouette.** BT ports BP’s
  corrected foot after three-size source/SVG/native review and nine
  actual-MailBox pairs. Production and packaged parity, both gates,
  294 Rust tests and the full 22-check / 27-case matrix pass. The mailbox
  golden changes by 146 pixels. The fractional foot threshold loss remains
  explicit within broader E2; see
  [round forty-three](../docs/reference-svg-round43.md).
- [x] **E3 — Store fourth-card crop:** primary source drawing stops near
  x1564.6, while the SVG continues to x1600. Measure a persistent viewport
  and retain the open cut; classify faint residue beyond it separately.
- [ ] **E4 — Remaining store fine lettering/printing:** primary rifles,
  inverse seams, 4ST contours, socket scatter and local stat/socket fit are
  implemented and native-reviewed. W fits the value glyph size/weight and
  X calibrates the stretched native baselines; ordinary and selected
  holdouts improve while the unchanged `5` runs preserve their fit.
  AF corrects the two native product-heading baselines and the SVG subtitle
  baseline, preserving an explicit selected-subtitle contour tradeoff.
  The SVG title lift fails first/last glyph controls and is rejected.
  AH corrects SVG title width/spacing with all 36 fixed segments and
  108 glyph checks improving; native rendering is unchanged. Fifteen local
  inter-letter gap regressions remain explicit alongside exact contours
  and printing. Integrated verification is recorded in
  [round nineteen](../docs/reference-svg-round19.md).
  See [typography measurements](../docs/entropism/store-typography.md).
  Do not replace that artwork again.
  CD identifies the selected M's unsupported x1173 stem in the fixed
  4K gap. One bounded M-only trim preserves A, `650` and every exterior
  pixel, but removes weak source ink in smaller-size controls and is
  rejected. Stem profiles support a row-varying placement/coverage model;
  a rigid shift or rectangular trim does not fit all height bands. This
  glyph has a concrete source crop for further work; other fine weapon
  echoes still need one. CH synchronizes the selected component title to
  AH's accepted parent nodes: full-size crop exact, title-only changes at
  full/half sheet size. All 24 SVG structural checks and full
  22-check/27-case verification pass; the package is identical to CF.
  Source-only join/foot measurements retain the earlier-fading source
  terminal, without assuming a hard cutoff. See [round forty-five](../docs/reference-svg-round45.md)
  and [art measurements](../docs/entropism/store-art.md).
  CM integrates one measured first-M path in the SVG and component,
  preserving the complete suffix and all exterior pixels at three sizes.
  Source RGB improves with two explicit fractional threshold tradeoffs.
  CP proves an exact native title split but rejects the original font-path
  replay as a source correction. CQ stages a native `Prim::Path` from the
  accepted SVG contour: whole-M RGB L1 improves 199136→112019,
  32808→17955 and 28052→17333 across the three sizes, and all whole-M
  IoU thresholds improve. Seven state pairs, a custom-purple selected-away
  pair and a 500 ms fade preserve suffix, semantic ink and feedback.
  Regional strict-threshold losses and the 1600 right-foot RGB loss remain;
  CQ production/package and full checks pass. Broad E4 stays open. See
  [round forty-seven](../docs/reference-svg-round47.md).
- [x] **E4 follow-up — Restore ordinary manufacturer-label width and weight.**
  At AP baseline, all three visible `BETTERLIFE TEC` runs are about
  34–35 native pixels too narrow and two pixels high, with thinner strokes. The
  right edge is already within two source pixels. Fresh packaged native
  output confirms the historical baseline exactly. Fit only these three
  visible band labels, preserving the wording, right anchor, PETROCHEM,
  fields, weapons and fourth-card cutoff. Freeze a selected-card fit before
  checking both ordinary copies; require dark-core, first/middle/last glyph
  and inter-letter-gap controls, then native/state and full verification.
  The fourth label is clipped out of the source. See the
  [bounded audit](../docs/entropism/store-typography.md#ap-manufacturer-label-audit).
  AP verifies the selected-only correction with local source/native glyph
  controls, thirteen paired states, both gates, 290 Rust tests, all 22
  repository checks and 27 first-attempt visual cases. The ordinary transfer
  was rejected for full-height dark stems in source gaps, leaving those
  copies open at AP. Selected exact contours and small gap/bottom-row
  residuals also remain explicit. See
  [round twenty-seven](../docs/reference-svg-round27.md).
  AQ verifies a shared ordinary table against both source copies, with no
  measured gap intrusion. All first/middle/last native 4K controls improve;
  small-size checks improve or tie. Fourteen paired states, thirty fixed-ink
  crops, both gates, 290 Rust tests, all 22 repository checks and 27
  first-attempt visual cases pass. It is a two-card calibration; small
  endpoint/font-edge residuals remain under broad E4. See
  [round twenty-eight](../docs/reference-svg-round28.md).
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
  CT's shared cyan fill fails the two −30° endcaps. CU separates their
  exposed sides from opaque front coverage and finds excess deeper-edge
  visibility in SVG/native. CW accepts the shared rose-field/opacity
  correction for all six nearest ghosts after three-size source/native
  and seventeen-state review; deeper fills and softness remain open. See
  [round forty-eight](../docs/reference-svg-round48.md).
  CX separates deeper-stroke ownership from the underconstrained fill
  model. CY/CZ integrate the shared stroke-mask SVG/native correction:
  all six stacks improve at three sizes, seventeen states pass, and the
  LOCATIONS specimen matches. The measured 4K cold CPU cost is +6.48 ms;
  cache hits and retained bytes are unchanged. Deeper fill and softness
  remain open. See
  [round forty-nine](../docs/reference-svg-round49.md).
  DC–DE separate deeper red deficits from nearby background error. The
  immediate boundary controls retain photo halos and repeated interior
  samples; they cannot substitute for clear-core or hidden-background
  measurements. A shared rose response is not yet established, and no
  further fill fit is accepted from these controls alone.
  DF/DG verify actual Mail/Store source donors at a subset of the frozen
  coordinates. Weapons retains a large card-red deficit where Store's
  ground is close to the model; right Products has both ground and card
  deficits. Stronger foreground guards retain those distinctions.
  Independently checked backdrop perturbations measure current-model
  transmission, not source alpha. Preserve these controls in the next
  overlapping-layer material experiment; no new fill is accepted. See
  [round fifty](../docs/reference-svg-round50.md).
  DI/DJ measure the actual deeper-fill red response and reject a standalone
  ground-dependent fill prediction for Vehicles/local losses. DK freezes
  438 cross-photo ground controls; DL's ellipse-only fit improves mean
  heldout error but worsens green, left, upper and Store controls. The
  corrected actual SVG and all source-patch records are independently
  checked. No ground or fill update is accepted; retain these controls in
  any coupled hypothesis. See
  [round fifty-one](../docs/reference-svg-round51.md).
  DM's joint rose geometry/colors and DN's separately fitted wash improve
  exposed-ground holdouts. DO preserves both frozen definitions in all
  three complete scenes at three sizes: whole-scene error improves, but
  left green and Vehicles/Locations depth-2 controls retain losses.
  The annotated alternate is a re-export, not independent material
  evidence. Keep the candidate in scratch; resolve local wash/composed-card
  controls before a native port, preserving accepted nearest material.
  See [round fifty-two](../docs/reference-svg-round52.md).
  DP then fits both gradients together on all 220 training positions.
  DQ's unchanged-parameter transfer improves average exposed ground but
  worsens Vehicles depth-3 RGB MAE 5.33734→7.90625; depth 2 also worsens.
  Astra verifies all nine scenes and fixed controls. Reject a native port
  of this exact candidate; clear-photo averages do not establish the
  hidden ground/material decomposition. Preserve accepted material and
  use the local losses to constrain any new hypothesis. See
  [round fifty-three](../docs/reference-svg-round53.md).
  DS samples original Mail/Store donors at the frozen Vehicles coordinates
  with 8/16/24-design-pixel model-foreground guards. Donor-minus-current
  red is negative at both depths for every guard, providing no support
  for DP's brightening. Nearby photographic UI and sharply shrinking
  common coverage prevent treating these as hidden-ground/alpha truth.
  The next ground proposal must preserve these local signs and composed
  controls; see [round fifty-four](../docs/reference-svg-round54.md).
  BW/BY separate overlap alignment from the exposed near ghost edge.
  A .29-design-pixel left-only widening improves 4K alignment but worsens
  smaller-size RGB/core controls in both SVG and Iced; reject that exact
  trial. The corrected fractional viewport and retained local losses are
  recorded in [round forty-four](../docs/reference-svg-round44.md).
- [x] **K2 follow-up — Fit primary front-card outline width.** AK narrows
  idle outlines from 1.8 to 1.4 while retaining selected/pressed 1.8 and
  ghost .9. Fresh-production baseline parity corrects an initially stale
  comparison image. Native review improves 296/300 core comparisons,
  including all fifty half-max widths and areas. Four other comparisons
  and thirteen complete-profile strips worsen; broader material/position
  work remains open. Eight state pairs preserve selected/pressed art and
  confine changes to idle boundaries. Both gates and the full repository
  check pass. See [round twenty-two](../docs/reference-svg-round22.md) and
  [the measurements](../docs/kitsch/dashboard-material.md#ak-current-native-outline-review).
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
- [ ] **K5 follow-up — Fit compliance cap height.** AQ's source-normalized
  crops show 11–12 source cap rows versus 13 native rows on ordinary
  compliance lines. Selected printing repeats the excess height with a
  different vertical phase. This is not a global baseline or width task.
  Preserve horizontal endpoints, tracking, gaps and adjacent artwork;
  fit one ordinary specimen before independent selected/ordinary controls,
  with separate SVG/native calibration and full state/gate verification.
  See [the bounded audit](../docs/kitsch/typography-fit.md#aq-compliance-cap-height-audit).
  AQ's ordinary .885 height fit passes 4K source/native controls but fails
  card-1 controls at 1600px, where the native caps already match
  source height. Its cache limits cost but does not cure this phase error.
  Restore the AP implementation; next fit must preserve small-text rendering
  while correcting larger caps. Selected trials also remain rejected. See
  [round twenty-eight](../docs/reference-svg-round28.md).
  AR rejects a common baseline correction after three-size native review:
  card-3/fourth strict 4K controls and some smaller-size controls regress.
  The analytic font baseline alone does not resolve rasterized edge phase;
  see the [trial record](../docs/kitsch/typography-fit.md#ar-rejected-common-baseline-correction).
  AS separates cap geometry, source-card registration and glyph shape.
  A continuous height/edge-response fit predicts M/A row profiles but
  leaves the O and two-dimensional contours wrong. It is not an accepted
  printing model. Continue with one legible glyph contour and repeated-card,
  counter/gap and small-size controls; selected ink remains separate. See
  [the mechanism study](../docs/kitsch/typography-fit.md#as-cap-geometry-registration-and-edge-response).
  The first O contour improves isolated masks but collides with N; a
  narrower revision restores the gap and fails strict small-size controls.
  Next fit a bounded O/N prefix jointly, preserving the suffix anchor and
  total advance. Neither O-only trial is accepted or ported.
  AT's paired O/N follow-up corrects a false collision from an old-font
  mask boundary, but retains real N/L and small-size losses. A structural
  audit separately supports investigating horizontal shelf registration;
  a whole-card vertical shift would disturb the already aligned yellow
  band. AU rejects the horizontal whole-card trial too: border gains hide
  band-shoulder, rifle and first-line gap regressions. Keep those controls
  in any narrower fit; no Kitsch geometry is changed. See [the updated typography record](../docs/kitsch/typography-fit.md#at-paired-prefix-and-shelf-registration).
  BC's complete `ONLY` contour and L-edge revision also fail ordinary
  O-counter and N/L-gap controls. BD independently confirms source-text
  offsets across both lines, then rejects a fixed text-only translation:
  all twelve whole-line RGB comparisons improve, but first-line gap ink
  and several words/endpoints worsen, including at 1600px. No native port
  is warranted. Keep full-line, counter/gap, endpoint and small-size
  controls in any new shape/placement model; do not repeat these rejected
  transforms. See [round thirty-nine](../docs/reference-svg-round39.md).
  BE replaces the card compliance face with a measured FreeSans approximation,
  fits line-specific placement/tracking, and applies the supported card-3/4
  text residuals. Both SVG and native source scores improve overall; all 24
  native whole-line RGB and 72 F1 comparisons improve. Fifteen paired state
  cases, production/package parity and full checks pass. Exact source font
  identity and 12 word RGB losses remain explicit. The 4K O cap matches on
  three cards, but ordinary M and smaller-size row profiles keep this narrow
  task open. Do not retry the former Rajdhani-only shifts against the new
  baseline. See [round forty](../docs/reference-svg-round40.md).
  BU’s one M contour improves the central notch and whole-line RGB but
  retains small-size gap/threshold losses and two boundary pixels outside
  the fixed M window. BV’s hidden-prefix transfer changes A/suffix raster
  even with the original M restored, so that method is rejected. Preserve
  shaping and the full A fringe before a native port. BX's central-aperture
  SVG transfer now preserves that fringe and the outer M at three sizes,
  but its frozen contour retains 1600 card-1 and fixed gap losses. BY's
  native clip partition passes 1600 no-op replay but drops four M pixels
  at 4K, so that transfer is rejected before any native contour trial.
  CA's intact-advance first-glyph scratch prototype compiles and passes
  three-size None parity in CC, but original-M path replay changes fixed
  suffix-boundary and exterior pixels. That transfer is rejected before
  the native proposed contour. CD locates these changes at the M edge,
  with later suffix columns exact, and confirms a different raster path.
  The separately named, otherwise identical font now passes full-RGBA
  replay at all three sizes, with actual shaped-font byte identity and
  weight verified. The original hints/advances stay intact. This establishes
  a transfer mechanism; the new contour remains unaccepted. CE's separate
  unchanged-outline hint-removal diagnostic fails at 4K with 81 changed
  pixels, including one outside the fixed M windows. Keep the original
  hinting. CG's operand-aware, unchanged-outline 15-point replay passes
  full RGBA at all three sizes; review hint behavior on changed geometry
  before rendering the proposed contour.
  See [round forty-five](../docs/reference-svg-round45.md).
  CI/CJ verifies the changed contour through hinted native drawing: all
  nine M/whole-line RGB fits improve with exterior and suffix exact, while
  small gap/strict-threshold losses remain. CK integrates a reproducibly
  generated, separately named face only for ordinary second lines.
  State/package and full repository verification pass; separate SVG
  calibration remains pending;
  see [round forty-six](../docs/reference-svg-round46.md).
  CP rejects two SVG-only plateau variants: they reduce one gap loss but
  weaken held-out card-four fit. CM's shared production-font Store/component
  SVG route is staged instead, with an exact original-outline control and
  nine improved M/notch RGB comparisons. The 1600 card-three gap loses
  232 RGB L1, card one's first M retains a false-bright pixel, and the
  fractional card-three gap loses six RGB L1. The package installs the
  existing licensed derivative for external SVG use; no second font is
  added. CQ package and full checks pass; broad K5 stays open. See
  [round forty-seven](../docs/reference-svg-round47.md).
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
  [art measurements](../docs/kitsch/store-art.md). AG corrects warning
  height on all four cards: all 12 whole-triangle native comparisons
  improve, with one small segment loss retained. Rounded feet and exact
  ink remain open; state/clipping controls and the integrated check pass.
  AN fits the RG5 lower opening/chamfers, raises the retained tiny type
  and adds two faint lower strokes. Whole-mark native RGB errors improve
  31.77→26.08, 37.06→32.02, 31.27→25.39 and 35.26→29.57, with all
  662 changed 4K pixels inside the four marks. Eight paired states and
  integrated checks pass. The selected lower-rule crop worsens slightly
  (native 10.18→11.27; SVG 5.55→6.38). RG5 lettering is still an
  unverified reading; exact ink, SC glyphs and warning microprint keep K8
  open. See [round twenty-five](../docs/reference-svg-round25.md).
  AO rejects a selected-only lower-rule shift after normalized source
  controls fail to establish a distinct contour. Its apparent RGB gain
  does not justify geometry changes; see the
  [phase audit](../docs/kitsch/store-art.md#ao-lower-print-phase-audit).
  AY fits the raised warning bridge with separate SVG/native subpixel
  geometry. The direct port fails small-size controls; one shallower
  native calibration preserves their tested dark masks. Local left-foot
  losses and outer rounding remain explicit. Nine paired states, production
  and packaged parity, both gates, 292 Rust tests and the full 22-check /
  27-case matrix pass in [round thirty-six](../docs/reference-svg-round36.md).
  Only 24 golden pixels change; K8 stays open for its remaining details.
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

- [ ] **K8 follow-up — Match the lighter SC letter ink.** A bounded SVG
  opacity fit improves all four source crops, but its native port and
  three less aggressive alpha trials fail independent dark-stroke checks.
  AD rejects all four; runtime and reference artwork stay unchanged.
  Inspect native glyph coverage/phase before another ink fit, retaining
  selected/custom disc colors and clipping. The source supports further
  local comparison; this is not closed by better average RGB error.
  See the [rejected-trial record](../docs/kitsch/store-art.md).

  AE fits the common SC type to SemiBold 4.3 at baseline 85.5 while
  retaining opaque ink. Seven of eight native dark-core scores improve,
  and all four RGB crops improve; the fourth card loses 11 loose-threshold
  core matches and remains a stated residual. Weight-only fitting fails.
  This bounded type correction does not close exact ink/phase fidelity.
  AQ confirms lighter normalized source contrast after the AE geometry fit,
  but finds no robust opaque glyph interior to isolate ink from blur/phase.
  One opaque-color trial fails selected/strict-core controls and is rejected;
  preserve the current artwork. See the AQ audit in the art record.

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
  The [embedded document records](../docs/sources.md#embedded-document-metadata)
  provide a bounded candidate list for Dashboard/Mail comparisons:
  Exo 2 Light, Gotham C2 Text, Myriad Pro Regular and Rajdhani Medium/Regular.
  These are inherited document records, not label assignments. The bundled
  Rajdhani faces and scratch Exo2-Light match recorded names/versions.
  DP's fixed Exo module-label substitution fails bright and 1600 controls;
  its method also misstated the size arithmetic. Reject that candidate,
  without identifying or ruling out the family. DS completes the bounded
  fourteen-letter diagnostic with EMAIL-E registration applied around
  each label's own baseline. Astra reproduces all 448 fixed-window metric
  groups and reviews the complete-glyph union audit. Rajdhani scores higher
  in these controls, but weight/glow/advances and registration remain
  confounded; neither font identity nor a new scene fit is established.
  Do not redispatch this diagnostic as unperformed. A next change needs
  a concrete contour/registration hypothesis and complete scene controls.
  See [round fifty-four](../docs/reference-svg-round54.md).
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
  [header fits](../docs/neokitsch/dashboard-header-typography.md). AG corrects
  the login A/B glyph sizes, removes the unsupported internal folds and
  restores both lower tabs through era-owned badge artwork. All local
  glyph/frame/tab controls and paired fractional states pass; the A
  was still heavy at AG; AJ corrects its contour in the follow-up below.
  Exact ink/softness limits remain in the [login measurements](../docs/neokitsch/login-branding.md). CTA remains
  unchanged. All 286 tests, six affected gates and the full check pass.
  AX fits mailbox body starts/widths and dashboard footer block placement,
  line pitch and size. Source/SVG/native review at three sizes supports
  these bounded geometry changes; eight paired state captures preserve
  other content. Fine glyphs, dim footer ink, word-gap edges and
  fractional cap placement remain open, with losses retained in the
  [text review](../docs/neokitsch/text-fit.md). Production/packaged parity,
  four gates, 292 Rust tests and the 22-check / 27-case matrix pass in
  [round thirty-five](../docs/reference-svg-round35.md).
  BF's repeated store metadata audit finds that cap-normalized FreeSans
  Bold improves whole-line overlap, but a single tracking fit intrudes into
  photographed word gaps and raises the upper first-line core one row too
  high. Actual R>190 false gap-core counts rise 10→361 at 4K, 0→43 at
  1600 and 0→29 at fractional size. The trial is rejected; no Neo-kitsch
  artwork changes. Source-font identity remains unknown, and separate
  word/letter spacing remains only a geometric lead after BG's held-out
  endpoint and cap losses. The tested SVG word-spacing property is ignored;
  explicit word runs would need actual-render calibration before native work.
  See [the metadata audit](../docs/neokitsch/text-fit.md#repeated-store-metadata-unresolved-font-and-spacing-fit)
  and [round forty-one](../docs/reference-svg-round41.md).
  BH–BJ then measures and compensates librsvg's text-boundary advance.
  The actual SVG improves all whole-line RGB and word-F1 controls against
  the original; source-gap fringes remain. Actual Store baseline parity
  passes at three sizes. Its native font/spacing trial improves every F1
  control but retains upper-C RGB and small-size gap-aperture losses.
  One upper-block position calibration is assigned before state review
  or production integration. See [round forty-two](../docs/reference-svg-round42.md).
  BK/BL's actual native calibration now improves every pair/line RGB and
  word-F1 control at three sizes, with cap, local word-RGB and counter-ink
  losses retained. All fixed source word gaps remain open. The bounded
  rest fit passes seven actual Store state pairs, and BM integrates the
  complete literal strings through a shared tracked-word representation.
  Source-font identity and listed local printing losses remain open;
  [round forty-two](../docs/reference-svg-round42.md) records integration
  checks separately from this bounded source acceptance.
- [x] **NK-05 follow-up — Fit repeated Store metadata.** BM integrates
  the six complete literal runs with common letter and word spacing,
  preserving backend-specific SVG/native placement. Three-size source and
  production review, seven state pairs, fresh packaged 4K parity, both
  gates, 294 Rust tests and the full 22-check / 27-case matrix pass. Only
  the Store golden changes, by 3787 pixels. Exact font identity, individual
  word RGB losses, gap/counter ink and small-size cap residuals stay in
  NK-05. See [round forty-two](../docs/reference-svg-round42.md).
- [x] **NK-05 follow-up — Correct mailbox section badges.** AV fits all
  four glyph contours, restores lower tabs and removes internal folds.
  Three-size source/native review, ten paired states, production/package
  parity, both gates, 292 Rust tests and the full 22-check / 27-case matrix
  pass. Only the mailbox golden changes, by 1725 pixels. A whole-outline
  brightness trial is rejected; occasional small-size fringes, exact
  outline ink/profile and photographic softness stay in NK-05. See
  [mailbox badges](../docs/neokitsch/mailbox-badges.md) and
  [round thirty-three](../docs/reference-svg-round33.md).
- [x] **NK-05 follow-up — Correct dashboard/store section badges.** AW
  fits all seven contours, lower tabs, external chamfers and footer
  registration. All 147 whole-glyph checks improve; eight paired states,
  three-size production parity, fresh packaged 4K parity, four gates,
  292 Rust tests and the full 22-check / 27-case matrix pass. Dashboard
  and store goldens change by 1988 and 1311 pixels. Local frame/corner/
  fringe losses, annotation typography and the ambiguous header backing
  remain in NK-05. See [scene badges](../docs/neokitsch/scene-badges.md)
  and [round thirty-four](../docs/reference-svg-round34.md).
- [x] **NK-05 follow-up — Correct the login A contour.** AJ replaces the
  heavy font approximation with source-derived outer/counter paths. All
  42 native glyph-region comparisons improve; B, plates, tabs and the gap
  stay unchanged. Seven fractional states alter only 71 A pixels each.
  Source glow and a small downsampled-source threshold loss remain explicit.
  All 288 tests, four gates, 22 repository checks and 27 visual cases pass
  in [round twenty-one](../docs/reference-svg-round21.md).
- [x] **NK-05 follow-up — Correct the login B contour.** AK replaces the
  wide font approximation with a slimmer spine and larger counters. Native
  review improves 87/88 fixed glyph comparisons, retaining one small
  low-threshold loss. A, plates, tabs and native gap remain identical;
  seven fractional states change only the B. SVG halo/gap losses remain
  explicit. Both gates and full repository checks pass; broader ink/glow
  fidelity stays open. See [round twenty-two](../docs/reference-svg-round22.md).
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
  AE's individually traced upper EMAIL branch improves only its middle;
  its entry/merge add unsupported ink and worsen RGB error. No geometry
  is applied. A supported branch connection or termination remains needed.
  AL maps a plausible continuation toward native x694 instead of x685;
  its scratch path improves local RGB comparisons but leaves crossings
  and the terminal join unresolved. It remains unaccepted. The source
  supports visible segments, not hidden graph connections; preserve both
  ridge families and check density/continuity before another port.
  AR confirms the visible upper EMAIL diagonal on 26 consecutive rows,
  but its clipped entry and competing lower ridges do not identify a
  supported endpoint or join. This branch is awaiting layered material or
  an unobstructed view; repeating the added-stroke trial is not actionable.
  Other visible veneer regions remain open. See the
  [bounded map](../docs/neokitsch/veneer-fit.md#ar-bounded-visibility-map).
  AS isolates the selected-store socket band: current SVG shallow waves
  and native horizontal stripes miss the source's downward-turning fan.
  Source/SVG/native measurements distinguish curve direction from contrast
  and density; early trials improve RGB by erasing texture and remain
  unaccepted. Preserve the selected body and evaluate the QR-cell/left
  flow as well as the clear text-socket windows. See
  [the band study](../docs/neokitsch/veneer-fit.md#as-selected-store-socket-band).
  The narrowed three-text-socket trial passes native locality at three
  sizes and improves A–E angles/RGB error, but creates a sparse first-cell
  wedge and loses local bends and upper-right contrast. It is rejected
  after source/SVG/native review. No geometry is ported; the next fit must
  constrain visible spacing and bends, not only the aggregate direction.
  CP's dashboard lower-panel trial improves the broad S-turn but fails
  lower-right ridge density and adjacent RGB controls at all three sizes.
  Visible source top/left entries support a two-boundary, density-constrained
  next fit; preserve the exact upper polyline and check its footer halo.
  The trial is unported. CR preserves the upper pixels and improves left
  crossing counts, but loses bottom/right density and creates unsupported
  interior endpoints. Its assumed 28 exits and 22 added strands are not
  established by photographic peak counts. It is rejected. Measure visible
  ridge continuation across multiple cross-sections before assigning new
  strands or exits; this remains actionable source work. See
  [round forty-seven](../docs/reference-svg-round47.md).
  CT's lower-left tracing initially follows contamination from the black
  frame. CU's panel-normalized field restores local lower-ridge tracing;
  independently seeded paths agree within .080 native pixels over their
  shared segment. Astra reproduces the field and reviews the overlay.
  This supports local continuation, not a full curve graph or geometry
  change. See [round forty-eight](../docs/reference-svg-round48.md).
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
  uniform opacity correction. AF's narrower, state-specific lower fifth
  stroke improves the straight span but fails selected-side and join
  controls; no frame change is adopted. The implementation gate's classifier-boundary
  failure is corrected through strict component-mask identity, preserving
  missing/moved/overlap failures and the existing thresholds. See the
  [frame measurements](../docs/neokitsch/store-frame.md) and
  [gate diagnosis](../docs/reference-svg-round11.md).
  AJ's continuous orientation/state gradient removes the hard join but
  worsens all four bottom RGB controls (ordinary RMS about 9.7–10.0→14.9–15.5;
  selected 15.9→22.2). Relative contrast improves while raw peaks and gaps
  brighten too far. No SVG/runtime change is accepted; see
  [the scratch review](../docs/reference-svg-round20.md#next-round-scratch-review).
  AU separates the photo-halo SVG baseline from native, then rejects a
  constrained narrow-core/limited-shoulder pixel model across the frozen
  top/side/bottom controls. No SVG candidate is warranted. See
  [the profile audit](../docs/neokitsch/store-frame.md#au-separate-the-frame-core-from-the-svg-photo-halo).
  BO reproduces the current native mismatch on ordinary 3/4 holdouts:
  peak, area, floor and state/orientation remain distinct. No new material
  candidate is justified. Selected middle-echo geometry still has an
  actionable 1–3px residual. BT maps the selected third echo; BU’s single
  +1.2 design-y SVG trial fails small-size phase/bend controls and its
  frozen locality guard. Native has a different baseline phase, so its
  single-contour trial is tested separately in BV and also rejected:
  it overshoots the 1600 ridge, lowers small-size area and worsens the
  fractional turn. Retain these controls in the next geometry model;
  see [round forty-four](../docs/reference-svg-round44.md).
  BZ's bend-preserving .9 flat correction restores turn/side parity and
  both smaller-size peaks, but broadens the fractional ridge too far.
  Its exact contour is rejected. CA derives coverage from the observed
  rows; CB's +1.0 native tail preserves area/width and aligns all three
  peaks. Ten CC state pairs, production/package parity, 294 local/Nix Rust
  tests, both Store gates and all 22-check/27-case verification pass.
  The separately calibrated +.8 SVG tail fails 1600 area and fractional
  width controls, so both SVGs remain unchanged. Core/halo work stays open.
  See [the current profile review](../docs/neokitsch/store-frame.md#bo-current-production-profile-confirmation).

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

- [x] **NM4 follow-up — Restore the component card background dependency.**
  AW's stricter local-reference audit finds missing `#store-ground` in
  `docs/neomil/components.svg`. The ordinary card uses the affected
  `c1fill` pattern, so the missing background is visible. The parent
  store trace defines the group; the sheet already carries its ramps
  and masks. AX restores the parent's twelve-rectangle group. Production
  renders match the reviewed candidate at two sizes, all changes stay in
  the ordinary card, and clean parent-interior controls match exactly.
  All 24 SVGs resolve local references. The card subtree and all 66
  reachable definitions match the parent; remaining rectangular-crop
  differences are exterior context and one-level QR compositing. See
  [round thirty-five](../docs/reference-svg-round35.md). The runtime and
  parent trace are unchanged. The integrated AX batch passes all 22
  repository checks and 27 first-attempt visual cases.

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
  cartridge/text contrast and echoes remain. AM's deeper terminal and
  bounded case/bed fills pass source/native review, twelve paired state
  checks, both gates and full repository verification. Exact contrast/phase and echoes stay open; see
  [round twenty-four](../docs/reference-svg-round24.md).
- [Store](neomil-store.md): **NM3 is fixed**, restoring the 25th socket
  cell and selected origin. The second batch fits cell spacing and all four
  origins. Primary artwork/materials and selected frame geometry are
  corrected; repeated/margin printing and scan echoes remain.
  AN integrates the two ordinary-title impressions with source/native
  and sixteen-state controls, correcting the renderer's skipped-mesh
  index offset and opaque-background ordering along the way. Exact title
  phase/softness and other printing remain open; title-outline reuse is
  a separate measured performance follow-up.

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

- For each correction, inspect the original, freshly rendered SVG and
  native crops with repository fonts. Preserve source quirks listed in the
  audit; unreadable marks require supported vectors, not invented text.
- Update component excerpts and source descriptions with their parent
  trace. Keep inferred interaction states explicitly identified.
- Rerun the relevant G1i gate and measure the changed region; a broad
  PASS alone cannot close a task. Never hide primary art as `photo` or
  weaken thresholds to accommodate missing detail.
- When implementing corrected references, review SVG→Iced states and
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
