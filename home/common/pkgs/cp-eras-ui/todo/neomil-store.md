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
  at most one. All20 category/card combinations preserve hit geometry,
  opening and permanent card4 clipping; custom themes retain semantic
  fields. Native state review, both gates and the K1 full check pass.
  Fine scan modulation remains in its own task below. See
  [material measurements](../docs/neomil/store-material.md).
- [ ] **Correct typography, footer fit and reference ink.** Source cap
  heights are roughly12–23% taller than trace. Footer code crosses its
  divider and caption spills past its cell. Match weight, glyph width,
  tracking and baseline independently; a global size increase would
  over-widen labels. Source bright bars are #fb3535 vs trace #df3131.
  Preserve custom palettes while matching reference printing.
- [ ] **Restore omitted SVG printing and weapon details in Iced.**
  Rotated BETTERLIFE TEC/PETROCHEM and margin code/branding are absent;
  the old no-transform justification is obsolete (`Prim::Turn` exists).
  Port rifle rail/grid/screws/contour strokes in `#gundetail`, the Japanese
  logo skew, and missing label tracking/regular socket-label weight.
- [ ] **Reconstruct source branding, symbols and weapons.** MASURAO,
  KIROSHI and repeated certification/chip marks remain schematic, as do
  gun receiver/sight/barrel/trigger/grip/stock parts and fasteners.
  Preserve measured object bounds and filled-vs-outline variants; do
  not invent unreadable microprinting. Source artwork is available locally.
- [ ] **Fit remaining scan modulation and printing echoes.** Keep this
  separate from primary geometry/art corrections. Use actual source
  holdouts; do not add generic glow/noise or relax fidelity thresholds.
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
states, including1537×947. Zero-time/mid-opening captures are reviewed.

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
