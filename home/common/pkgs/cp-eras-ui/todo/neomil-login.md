# Neomil login correctness

Audit: 2026-09-21. Reference `images/img-06-private.png`, source #59,
3840×2160; trace `docs/neomil/login-trace.svg`, 1600×900 design units.
The working greeter accepts one active account's password; the alternate
account cards are source artwork, not an account-management backend.

## Runtime and interaction

- [x] **Confine password masks and caret to the field.** `draw_slot`
  previously printed the entire masked run without clipping or scrolling. An
  80-character synthetic input crosses the second and third account
  cards. Preserve the full secret, show a bounded trailing portion, and
  keep the caret inside the field at native and fractional sizes. Test
  long Unicode input, deletion/clear, and unchanged submission semantics.
- [x] **Make the visible Login action accept pointer submission.** The
  canvas previously only drew it; Enter was the sole submit gesture.
  Use release on the original target, cancellation on drag/loss, a pointer
  cursor and existing inferred feedback coats. Disable interaction while
  submitting. Keep keyboard submission and the static hollow slot
  independent. This completes a documented keyboard-only limitation,
  not a missing authentication backend. Use synthetic events, not real
  credentials, for regression checks.

## Source fidelity

- [x] **Restore primary avatar artwork and portraits.** Reconstructed the
  filled halves, slit, wedge and small marks, then replaced both generic
  silhouettes with the source face/clothing contours. Native active-avatar
  dark-mask overlap improves .169→.872. Six vector tone masks preserve 105
  portrait regions without embedding a raster. Native source/SVG/Iced review
  accepts the primary art; posterization, scan texture and softer edges
  remain in the separate fine-printing/material work below.
- [x] **Correct password and card printing.** Ten source-sized stars now
  use the measured pitch, origin and baseline, followed by a separate moving
  tail. Native bounds agree within about .42px; the tail matches the source
  rectangle. USER 01, prompt and captions are fitted independently. The
  fractional 80-mask case changes no pixels outside the field, and dark
  blink removes only the caret. Full-secret semantics are unchanged.
- [x] **Restore header, badge and margin printing.** Open, non-retraced
  LEVEL paths, tier contours, twelve measured protocol bars, rounded tape
  and code contours replace the older approximations. Top captions, margin
  marks and rotated labels are fitted to this screen's source. Reference
  bright foreground and explicit plate fills preserve custom palette roles.
  Native Iced review accepts primary printing; badge/card materials and
  secondary echoes remain separate below. See
  [primary measurements](../docs/neomil/login-primary.md).
- [x] **Correct the shared ground.** Multiple unobstructed source patches
  match the dashboard exactly. Reusing the existing source-fit model
  improves native clear-ground RGB RMS9.049→1.685 across4,264,020 pixels,
  with both spatial holdouts improving. SVG and Iced share the same data;
  see [measurements](../docs/neomil/login-store-material.md).
- [x] **Fit inactive card and badge broad materials.** Source-fitted
  biaxial fields preserve the active card's supported flat #f63333 fill.
  Native inactive-card source RGB RMSE is 1.60/1.84, SVG/native .44/.56;
  measured badge frame inks/weights are restored. Changes stay inside the
  two inactive cards and five badge fields at native and fractional sizes.
  K1 passes both gates, 267 tests and all 22 repository checks. Fine scan
  modulation and exact shared texture remain separate below.
- [x] **Remove extra SVG→Iced color and geometry drift.** Active-card
  reference fill is now #f63333; custom palettes retain their foreground
  role. Inactive chamfers are51px, notch vertices390/405/483/496, the
  first avatar tab ends atx498, USER01 retains1.5px tracking, and card3's
  rail uses its darker edge ink. Explicit era-owned notch/tab plates
  replace hardcoded renderer dimensions. The existing trace and component
  excerpt already held these targets. See the
  [drift record](../docs/neomil/login-rendering-drift.md).
- [x] **Restore the three top-caption secondary copies.** AG independently
  fits the two source-visible copies of CUSTOMER, #NC488402 and SECURITY
  LEVEL, with held-out segments, echo/bright-core masks and blank controls.
  Reference fractional states change only caption pixels; custom output
  is byte-identical. A two-pixel loose SECURITY fringe loss remains explicit.
  All six affected gates, 286 Rust tests, 22 repository checks and 27 exact
  visual matches pass. See [round eighteen](../docs/reference-svg-round18.md).
- [x] **Correct inactive-card notice placement.** AH corrects both notice
  baselines on cards 2/3 with source/SVG/native masks and seven fractional
  states. The active card and blank controls are unchanged. One-pixel
  edge differences remain explicit. All 286 Rust tests, four affected
  gates, 22 repository checks and 27 exact visual cases pass. See
  [round nineteen](../docs/reference-svg-round19.md).
- [x] **Fit the locked-card notice ink.** AI uses the source notice red
  only for the reference palette while preserving custom Dim semantics.
  All 30 fixed source comparisons improve in both SVG and native output;
  all changed 4K pixels lie in the locked notice. Dim-only custom output
  is byte-identical. Exact contours and softness remain separate. Both
  gates, 288 Rust tests, all 22 repository checks and 27 exact visual
  cases pass. See [round twenty](../docs/reference-svg-round20.md).
- [x] **Fit the active-notice lower printing copy.** AJ adds the measured
  offset/stroke/blur/opacity behind the primary text, through era-owned data
  and a cached local tile that follows the notice-block fit. Source and
  native holdouts improve; fractional/reference/custom states preserve
  controls and containment. Cache reuse/invalidation, all 288 tests, four
  gates, 22 repository checks and 27 visual cases pass in
  [round twenty-one](../docs/reference-svg-round21.md).
- [ ] **Fit remaining scan modulation, printing echoes and edge softness.** Do primary
  shape/type/material corrections first; validate any secondary copies
  on held-out source crops. Do not add global glow/noise or weaken gates.
  AJ's active-notice lower copy passes local source/native/state review,
  including fitted narrow layouts and unchanged custom palettes. Small
  primary fringe losses remain documented. Other repetitions, exact
  glyphs and scan material stay open. See
  [the measurements](../docs/neomil/login-primary.md#active-notice-echo-investigation--aj).
  BG's inactive-notice audit rejects a Regular→Medium SVG trial: bright
  overlap improves, but two whole-line RGB controls worsen at every size
  and one O counter gains false ink. A smaller thresholded ink area does
  not establish the weight as the cause; glyph shape, phase and hinting
  need separate evidence. No native change is proposed. See
  [the rejected trial](../docs/neomil/login-primary.md#bg-inactive-notice-weight-audit).
  A subsequent cap-normalized O/C/M study supports one FreeSans Bold
  full-notice experiment with held-out words and counter controls; its
  isolated O counter is too small, so no font or sentence fit is accepted.
  BH–BJ's full-notice studies improve all whole-line SVG RGB comparisons
  after independent source-copy registration and a separate line-two fit.
  Fixed glyph/word losses remain, but severity review preserves all word
  separators and both 4K O counters. Prepare the actual Login native trial
  with consistent face/measurement/baseline handling and unchanged active
  AJ artwork; no native acceptance yet. See
  [round forty-two](../docs/reference-svg-round42.md).
  BK's actual baseline replay passes at three sizes, but the new native
  first line lands one row high and regresses canonical RGB controls.
  BL's FreeSans-only glyph-baseline rounding aligns the first-line rows
  and improves all four 4K whole-line RGB controls, but three 1600 lines
  still regress. The candidate remains unaccepted. Source fit parameters
  and active AJ artwork stay frozen; native glyph/spacing diagnosis,
  complete responsive bounds and state review precede integration.
  BM/BN retain clear word gaps and thresholded rest containment, but find
  repeated CLASS contour/raster losses and a smaller terminal O counter.
  No new candidate or production change follows; lower-threshold ink and
  all-layout/state containment remain unverified.
  CT replays BJ's exact font geometry as unhinted SVG outlines. The text
  control and exterior are exact, but both first lines and repeated CLASS
  crops worsen, and O false ink remains. Astra reproduces the regional
  RGB results; no native vector port is justified. Complete-line native
  contour/placement/coverage diagnosis continues in
  [round forty-eight](../docs/reference-svg-round48.md).
  CV/CW separate coverage from placement: fixed linear blending predicts
  most of BL's small-size threshold-area inflation, but native coverage
  redistribution and CLASS/O/gap errors remain after sRGB recomposition.
  Astra reproduces those controls. Neither a global blend change nor a
  local native font/path port is accepted from this diagnosis alone.
  CX compares every inactive line against current native printing and
  retains real CLASS/O/gap losses. CY's exact Regular face is too narrow;
  CZ's one shared spacing correction still fails the held-out 1600 first
  line. Both remain unapplied. DA's common opacity improves aggregate
  error but erases the source's bright letter cores; it is rejected.
  DB's shared contour thinning also loses bright cores and retains local
  glyph/gap losses despite lower whole-line error. Neither justifies new
  local printing machinery; see
  [round forty-nine](../docs/reference-svg-round49.md).
  DF's common `a^gamma` coverage response preserves opaque pixels but
  worsens every 1600 line and loses source-core detail; Astra reproduces
  the error/coverage controls and rejects it. DG identifies repeated
  internal CLASS spacing differences separately from stroke coverage.
  DH's zero-offset glyph-span split changes following words and fails
  the no-op gate; no origins are fitted. An exact shaping/coverage/suffix
  replay must precede any card-2-only origin fit, with card 3, 1600,
  neighboring gaps and source cores held out.
  No new font or renderer is accepted. See
  [round fifty](../docs/reference-svg-round50.md).
  DI identifies the per-span layout and single-value `dx` mechanism.
  DJ/DK now replay the full editable glyph run with exact first-line alpha
  at both canonical sizes. The absolute-onset CLASS trial crowds the
  following gap and is rejected. Separately frozen C-relative offsets
  improve both cards/sizes without any fixed-region RGB loss against BJ;
  adjacent text and gaps stay exact. A/S still touch, C registration is
  wrong and CLASS remains worse than current native printing. Preserve
  this diagnostic; investigate remaining contour/width and registration
  before a native port. No new font or renderer is accepted. See
  [round fifty-one](../docs/reference-svg-round51.md).
  DM's isolated CLASS audit separates the early C and touching A/S from
  cap height. Regular outlines at exactly the original Bold advances
  still lose source bright strokes and worsen both first-line overlaps;
  reject the substitution despite its small aggregate RGB gain.
  No Neomil original contains a font record; shared document-font records
  in other eras do not identify these notices. Preserve the exact replay
  and fixed controls in [round fifty-two](../docs/reference-svg-round52.md).
- [x] **Contain card notices at narrower aspect ratios.** AI fits each
  complete notice block with a common font-size/tracking factor, retaining
  its hinted glyphs, anchors, baselines, stretch and text. All three cards
  can overflow under the old mean text scale; native edge probes now find
  zero overflow at 1537×947, 1200×900 and 900×1200. Seven layout sizes have
  Rust coverage, both canonical sizes retain their geometry, and seven
  fractional states preserve other artwork and input feedback. Custom
  palettes pass. The full repository check and all 27 exact visual cases
  pass; see [measurements](../docs/neomil/login-primary.md).

## Acceptance and preserved findings

- Inspect original→SVG→Iced at rest, with long input and pointer
  states, and at fractional resolution. Keep component excerpts and
  source docs consistent; refresh only intentional reviewed goldens.
  Run relevant Rust regressions, fidelity gates and full repository checks.
- [ ] Live desktop keyboard/pointer/IME and actual greetd/session check
  remain separate from headless checks; do not use real passwords in
  capture tooling or start desktop applications without authorization.

Baseline: frozen Iced equals its existing golden. G1i matches16/34 shapes,
79% source area; G2i26/31 shapes,96% trace area. Those are extracted-shape
area, not detail fidelity. Live startup works: only10 blinking-tail pixels
differ from the frozen capture. No missing subscription here.

Preserve the corrected source identity, active card extent y315..570 and
51px chamfer. The separate hollow Login-button slot at
x497.3342..500.3799/y635..654.6317 is already fixed. Its meaning is unknown;
it is not the blinking password tail and must not gain invented semantics.

## First implementation batch

The input fix only bounds the displayed mask count; the complete Unicode
secret still goes through the existing submission path. Awake input is
clipped above the well, reserving tail/caret space in both blink phases.
Mouse activation uses the drawn action polygon and existing inferred
coats. Drag/loss, keyboard edits and disabled authentication intervals
invalidate held gestures. The hollow slot stays static.

Headless native and1537×947 long-input captures have zero changed pixels
outside the password field compared with empty input. Native rest versus
long input also leaves all other artwork unchanged. Rest/hover/held/disabled
button captures are reviewed. The new tests cover all four eras, Unicode
editing, full-secret preservation, pointer cancellation and re-enabling.
The initial batch passes all 249 Rust tests, both login fidelity gates,
and all 19 repository checks, including all 27 visual cases. G1i matches
15/34 shapes (78% source area); G2i matches 23/32 (95% trace area). These
are broad shape checks, not completion of the remaining source work.
Only the login/store goldens changed in this batch. All changes remain
staged, with no live authentication; repeat the acceptance checks for
subsequent visual corrections.

## Primary-art checkpoint, 2026-09-27

The H snapshot passes 263 Rust tests and all 22 repository checks. All 27
visual cases match 100.000% on their first attempt. Source/SVG and SVG/Iced
login gates pass; native and fractional primary art plus synthetic input,
blink and button states are reviewed. Only intentional reviewed goldens
are refreshed. The subsequent card/badge material work is under separate
native verification and is not covered by this checkpoint. See
[seventh-batch record](../docs/reference-svg-round7.md).
