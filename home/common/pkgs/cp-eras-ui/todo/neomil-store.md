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
- [ ] **Refine repeated/margin printing.** Primary MASURAO, KIROSHI,
  certification marks and rifles are source-supported vectors. Remaining
  dim repeated-mark fields and faint copies need local fitting. The
  margin O/plaque residual is the separate correction above. T fits PETROCHEM tracking; a shared
  BETTERLIFE shift/shear is not source-supported. V corrects selected side
  placement as recorded above. Preserve the accepted primary paths
  and their feedback colors; do not reconstruct the main artwork again.
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

- [ ] Inspect source→SVG→Iced at rest, early opening, all card selections,
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
its solid gun shifted14px left, and the fourth-card cut at x1557.
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
