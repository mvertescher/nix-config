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

- [ ] **Restore primary avatar artwork and portraits.** Source active
  avatar contains two large filled dark hexagonal halves, a shallow
  diagonal gap, a left wedge and small marks. The trace substitutes a
  small hollow hexagon and omits the wedge; central dark area is about
  2,024 source vs 511 trace square design pixels. The other cards replace
  the same source face/clothing portrait with generic silhouettes.
  Reconstruct supported source contours; do not invent a replacement
  person or hide missing primary art with `photo` annotations.
- [ ] **Correct password and card printing.** Source ten stars are about
  6.25px wide on 11.25px pitch, ending at x491.67; the tail spans
  x499.17..512.50 at y622.92..623.75. Trace stars are only 2–3px wide,
  with the tail ending x475 (Iced about484). Remeasure glyphs, pitch,
  prompt origin and baseline together. USER01 needs source weight,
  placement and contrast; source first-label bounds are
  x469.6..528.3/y515.8..528.3. Captions are about11% too short.
- [ ] **Restore header, badge and margin printing.** LEVEL is about36%
  too narrow; tier lettering, protocol, tape and margin codes are older
  approximations. Reuse accepted dashboard/mailbox vector vocabulary
  only where source crops support it: whole headers are not identical.
- [x] **Correct the shared ground.** Multiple unobstructed source patches
  match the dashboard exactly. Reusing the existing source-fit model
  improves native clear-ground RGB RMS9.049→1.685 across4,264,020 pixels,
  with both spatial holdouts improving. SVG and Iced share the same data;
  see [measurements](../docs/neomil/login-store-material.md).
- [ ] **Fit screen-specific materials.** Independently fit inactive
  card/badge fields and scan modulation against their source regions.
  The active card's SVG flat #f63333 is already source-supported; do not
  invent a strong gradient there. Exact shared texture remains dependent
  on source material or an independently validated reconstruction.
- [ ] **Remove extra SVG→Iced color and geometry drift.** Reference-mode
  active card is #de2e2e instead of #f63333. Preserve custom palettes
  while correcting reference roles. Inactive top-right chamfers are47px
  instead of51px; notch vertices are392/402/486/497 instead of
  390/405/483/496. First avatar tab is6px too long. USER01 drops1.5px
  tracking; card3 notch rail should retain its darker ink. Correct these
  in source-aware data rather than hardcoding screen names in the renderer.
- [ ] **Fit remaining printing echoes and edge softness.** Do primary
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
