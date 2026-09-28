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
- [ ] **Fit remaining scan modulation, printing echoes and edge softness.** Do primary
  shape/type/material corrections first; validate any secondary copies
  on held-out source crops. Do not add global glow/noise or weaken gates.

## Acceptance and preserved findings

- [ ] Inspect original→SVG→Iced at rest, with long input and pointer
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
