## Current remaining work (2026-09-29)

The [completion campaign](todo/completion.md) tracks the active lanes and
integrated verification for the request to work through all open items.

The linked workstream records preserve the implementation history. Earlier
"still open" lists are superseded by later completion entries, including
the five-task feedback/native-control/extractor batch committed in `de56b25`.
The earlier 82-file correction batch landed in `79c1a02`; its dated
"staged" validation notes describe the state before that commit.
The N–AC corrections and resize work are included in the current commit;
their dated "staged" notes likewise preserve the state during validation.

- **All-era reference SVG fidelity:** reviewed all 24 SVGs: 16 against
  source images, four against parent traces, and four as original bars.
  All 16 source gates pass, but
  the original audit found missing content, simplified artwork, geometry,
  typography and material differences. The later batches below resolve
  primary defects; named local typography/printing/material residuals remain. The [reference correction plan](todo/reference-svg.md)
  lists concrete work; the [audit](docs/reference-svg-audit.md) records
  coverage, source evidence and gate limits. The first correction batch
  completes nine tasks and parts of E2/K2 across ten screens. The
  [second batch](docs/reference-svg-round2.md) adds Kitsch login contours,
  Neo-kitsch badges/envelopes, Neomil socket spacing/current documentation,
  and further Entropism/Kitsch text fitting. Materials, artwork, exact
  font shapes and small typography residuals remain open. The
  [third batch](docs/reference-svg-round3.md) corrects Kitsch's fourth-card
  cutoff and Neo-kitsch envelope placement; the latter retains the
  source-supported 60.2px row pitch.
  The [fourth batch](docs/reference-svg-round4.md) calibrates Entropism's
  mailbox baseline and corrects Neo-kitsch store tabs and shoulders.
  The [fifth batch](docs/reference-svg-round5.md) corrects Entropism
  paragraph/badge geometry, Kitsch lower store-card corners and stale
  Neo-kitsch component/provenance examples.
  The [sixth batch](docs/reference-svg-round6.md) separates Neo-kitsch's
  selected mailbox inks, improves Neomil footer type/printing and closes
  Kitsch citation cleanup. Fine footer printing and font fitting remain.
  The [seventh batch](docs/reference-svg-round7.md) verifies primary login
  artwork/printing, broad grounds and lower image-upload cost; selected
  inverse weapon detail and remaining materials continue. Bars are original
  designs. The [eighth batch](docs/reference-svg-round8.md) verifies broad
  login/store materials, primary store artwork and state/clipping behavior;
  local typography, certification, veneer flow and fine printing continue.
  The [ninth batch](docs/reference-svg-round9.md) validates the recovered
  L/M work and N fixes: all 269 Rust tests, 22 repository checks and 27 exact
  visual matches pass. Source-derived margin CJK is deterministic; mailbox
  upper contours, socket scatter and maker tails are now verified. The
  [tenth batch](docs/reference-svg-round10.md) verifies store/mail/dashboard
  typography and printing, Kitsch face/ghost fields and a bounded Neomil
  resize preview: 272 Rust tests, 22 repository checks and all 27 exact
  visual matches pass. Selected-frame geometry and finer materials continue.
  The [eleventh batch](docs/reference-svg-round11.md) is verified:
  Z passes 275 Rust tests, all 22 repository checks and all 27 exact
  visual matches on their first attempt. V–Y native review accepts the selected frame,
  chip-2, dashboard labels, fan materials/ghosts, value baselines, mailbox
  ribs, lower frame echoes and veneer seam. Z calibrates the native footer
  code. The Neo-kitsch implementation gate's classification mismatch is
  corrected using strict component-mask evidence and negative controls;
  it now passes at 89%, with existing thresholds unchanged.
  The [twelfth batch](docs/reference-svg-round12.md) adds locally verified
  socket trails and asynchronous eligible resize preparation: 285 Rust
  tests, affected gates, all 22 repository checks and all 27 exact visual
  cases pass. The [thirteenth batch](docs/reference-svg-round13.md) verifies
  the Neomil margin word/slash, Kitsch badge apertures and native shelf
  brands, and Neo-kitsch action labels: 286 Rust tests, all 22 checks and
  27 exact visual cases pass. The [fourteenth batch](docs/reference-svg-round14.md)
  then corrects the small margin plaque boundary with ten native-state
  captures, both gates and another full check: 22 checks and 27 exact cases.
  Changes remain staged/uncommitted.
- **Neomil dashboard performance:** compositor/cache fixes reduce
  the measured 4K preparation median from 36.61 s to 0.404 s while
  preserving the original image layers. Route returns stay cached.
  See the [performance record](todo/performance.md) for measurements,
  fidelity checks and remaining resize/live-desktop work.
- **Neomil mailbox fidelity:** the six audited correctness defects are
  fixed: startup clock, selection heading, source typography,
  materials, cartridges and omitted printing/row shading. Fine printing
  echoes and cartridge edge detail remain. The reference recheck’s lower
  reader corner/side step and upper contour are corrected and verified.
  Exact background texture needs
  original source material. See the [mailbox record](todo/neomil-mailbox.md)
  for validation and the separate live-desktop follow-up.
- **Neomil login correctness:** password containment, mouse submission
  and shared ground are fixed. The third batch corrects the extra Iced
  card color, notch/tab/chamfer and label-tracking drift while preserving
  custom palettes. Primary avatars/portraits, mask/card/header printing and
  reference bright inks are now corrected and verified. Inactive card/badge
  broad fields and frame inks also pass native review; scan modulation and
  fine printing echoes remain. See the
  [login plan](todo/neomil-login.md) for the measured defects and preserved
  source details.
- **Neomil store correctness:** startup, selected/cropped bounds, wash
  contour, open edge, lower chamfer and shared ground are fixed.
  Broad card/navigation fields and primary source logo/rifles now pass
  native/state review. Primary certification/KIROSHI and deterministic
  margin CJK also pass. Independent ordinary/selected typography and footer
  fits are verified through T. V's selected top/right frame and side-printing
  correction passes native/state review and the full Z check.
  AB corrects the small MASURAO word and slash, preserving CJK. Its
  O/plaque leading edge is corrected in AC; inner ink and the second-A
  junction remain local follow-ups, alongside repeated printing and fine
  scan echoes.
  Socket occupancy, measured spacing and per-card origins are corrected;
  directional printing echoes now pass AA native/state and integrated
  review; irregular fine grain remains open. See the
  [store plan](todo/neomil-store.md). Frozen goldens and broad shape gates
  passed despite the audited defects; retain live/state-specific checks.
- **Neomil dashboard source fidelity:** chip-2 ink passes local native/state
  review; Z corrects footer code height/baseline. Fine footer printing and
  exact glyph contours remain. Tiny tape-mark ink and maker microtext
  tails/edge softness are verified; exact horizontal scan detail remains. The fine
  background still needs its original asset or authoring recipe. Follow
  the [measured plan](todo/neomil-dashboard.md); broad shape-area gates do
  not establish detail completeness.
- **Kitsch dashboard hover:** needs a hub-specific design. Press is done;
  the existing blade trails do not establish an additional hover state.
- **Kitsch store selected-card hit regions:** fixed with selection-aware
  bounds through the lower details and footer, retaining card4's cutoff.
  See the [toolkit record](todo/toolkit.md#current-runtime-follow-ups).
- **Animated interaction transitions:** need reviewed timing/easing and
  transition rules. Component sheets specify destination drawings only;
  existing boot-in/caret animation timings are not interaction timings.
- **Desktop verification:** native input/IME, hub navigation, feedback,
  and first-frame haze/route latency still need a live check. Headless
  tests and previews do not close these items.
- **Future widgets:** slider ticks, shared icons, spec/log rows, meters,
  tooltips/modals and tab widgets wait for actual callers or reference
  material, as detailed in the [toolkit roadmap](todo/toolkit.md).
  Do not build unused widgets to close its umbrella checkboxes.
- **Extraction diagnostic:** the Kitsch mailbox yellow extraction remains
  explicitly "do not fix" in the [pipeline record](todo/design-pipeline.md).

The three reader follow-ups are resolved: Entropism cursor wording,
Neomil's title-card index and the separate hollow Login-button slot.
See [the source audit](docs/source-followups.md); the password caret
was already correct, and the slot's state semantics remain unknown.

Follow-up audit: three native-event regression tests now exercise button
cancellation/disable, touch release/loss and selected-range Unicode commits
after input re-enabling. All five control tests pass using the headless
software renderer. No production behavior changed; OS IME/preedit and live
desktop verification remain open.

## Workstream records

- [All-era reference SVG correctness](todo/reference-svg.md): complete
  source audit, concrete corrections and derived-sheet consistency.
- [Neomil login correctness](todo/neomil-login.md): input containment,
  action behavior, source artwork, printing and materials.
- [Neomil store correctness](todo/neomil-store.md): startup, hit regions,
  clipped selection, shape corrections and source fidelity.
- [Neomil mailbox correctness](todo/neomil-mailbox.md): source-to-trace
  corrections, missing renderer details and runtime regression coverage.
- [Dashboard performance](todo/performance.md): measured cold-draw costs,
  cache invalidation, optimization work and fidelity requirements.
- [Neomil dashboard fidelity](todo/neomil-dashboard.md): the current source
  correction plan, accepted measurements, validation and remaining detail work.
- [Design pipeline and component sheets](todo/design-pipeline.md): source
  audits, trace corrections, gates and extraction diagnostics.
- [Bar styling](todo/bar.md): measured era chrome and implementation history.
- [SVG-to-iced conversion](todo/svg-to-iced.md): renderer coverage, scene
  conversion and the widget-layer decisions.
- [Toolkit, interactions and themes](todo/toolkit.md): reusable controls,
  theme plumbing, motion, caller-dependent widgets and desktop follow-ups.
- [Verification infrastructure](todo/verification.md): headless rendering,
  desktop renderer troubleshooting and golden-history notes.

## Historical validation before the reference campaign

These batch counts and staging notes describe their original checkpoints;
the current completion record above supersedes them.

The resumed batch adds six matrix/code ink fits, margin and vertical-brand
copies, and GO HOME heading/maker corrections. The earlier dashboard/reader
work was committed in `6da816e`, with the TODO split in `bc19ef8`; this batch
remains staged. The [printing audit](docs/neomil/dashboard-printing.md)
records source evidence and limitations. All 209 Rust tests and both
fidelity gates pass; native/fractional state and opening captures are
reviewed. The full repository check passes all 19 checks, including all
25 golden cases. Two dashboard goldens and Login's previously missing
hollow-slot golden are refreshed. Live verification remains separate.

The mailbox correction batch also remains staged. Its six audited fixes
and source-detail limits are recorded in [the mailbox TODO](todo/neomil-mailbox.md).
Final validation passes 238 Rust tests, both fidelity gates and all 19
repository checks, including all 26 goldens. The added unfrozen mailbox
case matches 100.000%; headless presentation scheduling is fixed without
changing the clock or comparison thresholds. Only the mailbox golden
changes in this follow-up; the prior staged work is preserved.

The first login/store correction batch is also staged. It fixes login
password containment and pointer submission, store startup and selected/
cropped geometry, and the shared ground on both screens. All 249 Rust
tests, all four screen fidelity gates and all 19 repository checks pass,
including all 27 visual cases. Only the Neomil login/store goldens were
refreshed in this batch. Remaining artwork, typography and card-material
corrections stay open in the linked plans; live verification remains
separate.
