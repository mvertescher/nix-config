# Reference SVG round sixty-seven — warning hollow and ordinary placement

Round sixty-six rejected the direct Kitsch warning-hollow enlargement because the fourth ordinary card's left side and foot lost source fidelity. EU then measured the neighboring frame edge and exclamation stem independently of the triangle. Their card-1 source-minus-native midpoint offsets, +0.939549 and +1.239842 physical pixels, fix **one** ordinary x translation at their equal-weight mean divided by 2.4: `0.454039972301` design units. The selected card-2 controls straddle zero, so its warning remains at x0. This is a bounded visual hypothesis, not an inferred global asset transform or opaque-ink recovery.

EV keeps ER's one widened hollow, including inner vertices `(32.108745,87.8)`, `(36.5,79.519348)`, `(40.891255,87.8)`, and moves only the ordinary triangle and exclamation (stem and dot) together. The frame, RG5/SC/C marks, microprint, ink, y coordinates, outer shape and feet remain unchanged. One shared warning definition serves ordinary and selected cards. A structural no-op with the old hollow and dx0 renders pixel-identically to the current Store SVG at 3840×2160, 1600×900 and 1537×947, and to the current component sheet at its native and 2.4× sizes. Astra independently verifies the SVG ancestry, frozen geometry, 13 PNG replays and source samples. The selected warning is pixel-exact to ER's widened-hollow candidate.

EW v1's native extraction added a top-level `CARD` primitive and was rejected **before build** because it shifts hardcoded `card_face` indices. EW v2 nests the warning `Prim::At` in its original `band_marks!` position before microprint. Both top-level `CARD` and `GROWN` definitions remain byte-for-byte unchanged. V2's candidate changes only the ER inner vertices and the ordinary macro invocation x; the selected invocation stays x0. The native bridge retains its accepted y88.8 calibration. Fresh copies of all 72 Rust sources, 15 font-directory files and Cargo inputs, plus pinned compiler/dependency/vendor fingerprints, were guarded. The unmodified production Store example supplied the captures. Baseline and v2 structural no-op are byte-exact to ET's accepted native captures at all three sizes before candidate scoring.

Against uniformly sampled source images and frozen regional boxes, native whole-warning RGB L1 improves on **all twelve card/size comparisons**. Candidate-minus-baseline L1 values are:

| Size | Card 1 | Selected 2 | Card 3 | Card 4 |
|---|---:|---:|---:|---:|
| 3840×2160 | −25,785 | −21,707 | −26,099 | −22,267 |
| 1600×900 | −4,758 | −3,666 | −4,866 | −3,770 |
| 1537×947 | −3,005 | −3,228 | −4,041 | −2,676 |

The former fourth-card left-side loss becomes a gain at every size (−3,176/−283/−341 L1). Full right-foot boxes improve on all twelve; full left-foot and full exclamation-plus-dot boxes improve on eleven each. Bridges improve or remain exact; frame and exterior controls are exact, and no changed pixel lies outside the warning boxes. Astra independently checks all nine native PNG hashes and 216 fixed source-region scores, accepting the coherent source improvement with the local residuals below.

Retain every true local exception. At 4K, card 4's legacy narrow left-foot box worsens +78 L1 (2.0%, 10 changed pixels), though its full left-foot box improves −71. At the fractional size, card 1's legacy narrow exclamation worsens +268 L1 (38.1%, eight changed pixels) and its full exclamation-plus-dot box worsens +14 L1 (1.0%). The same card's left foot worsens +73 L1 in both narrow and full boxes, from one changed pixel. The EV SVG trial has a separate fractional card-1 legacy narrow-exclamation +26 L1 loss while its predeclared full support improves −425. These residuals rule out a claim of exact foot, blur or ink identity.

## State and integration verification

Twelve paired native cases cover first/fourth-card selections, selected and
ordinary hover/hold feedback, selected-away retention, fourth-card clipping,
and two opening frames. These are synthetic Pointer rendering snapshots
through the production Store selection update and Scene drawing code, not
live OS input replay. The inactive fixture matches the unmodified production
example exactly. Every changed pixel stays inside a warning region; at zero
opening time the two frames are identical. Selected/ordinary hover controls change material relative to rest. Held and
away selected states correctly retain the resting selected face, as declared
by `selected_pressed = GROWN` and `selected_away = None`. The first verifier
incorrectly required away to differ; that assertion was corrected against
the actual state definitions, preserving all capture inputs and outputs.

| Case | Changed pixels |
| --- | ---: |
| inactive-gate | 191 |
| selected-first | 166 |
| selected-fourth | 811 |
| selected-hover | 191 |
| selected-held | 191 |
| selected-away | 191 |
| ordinary-hover | 191 |
| fourth-hover | 814 |
| fourth-held | 164 |
| fourth-selected-held | 165 |
| opening-zero | 0 |
| opening-early | 70 |

The component sheet changes only its two warning marks: 113 pixels at
1920×1392 and 449 at 4608×3341. Its shared warning paths and selected/ordinary
uses match the Store trace. The native integration differs from the reviewed
candidate only in comments and indentation; its 1600×900 golden changes
191 pixels. No font bytes, shaping, width/path cache, shared renderer or
global blending behavior changes. The warning uses existing vector paths
and one nested translation; no raster layer or per-glyph effect is added.

Both Store inventory gates pass. The full repository run passes all
22 checks and all 28 matrix cases on the first attempt. The packaged Rust
suite passes 305 tests (260 library and 45 binary tests). Direct visual
comparison finds 26 of 27 images exact to their goldens; only the existing
one-level Neo-kitsch bar pixel differs. The remaining matrix case validates
the source sampler. A fresh packaged 4K Store frame exactly matches EW v2
(`f96e700f…`). The verified package is `/nix/store/q465ksv5gqssm7z8g24841npr0zdnig8-cp-eras-ui-0.0.0`.
All 349 frozen crate files match the Nix source tested by this run;
only these completion notes are updated afterward.

Evidence is retained under `/tmp/cp-eras-next/ev-kitsch-warning-placement/`,
`/tmp/cp-eras-next/ew-kitsch-native-placement/v2/`,
`/tmp/cp-eras-next/ex-kitsch-states/` and
`/tmp/cp-eras-next/ey-kitsch-integration/`. The first three hold frozen input
hashes, independent source scores, no-op replays and captures; the last owns
integration verification. All sixteen originals are preserved. K8 stays
open for exact outer feet, ink and remaining certification printing; this
bounded improvement does not close any of the 29 broader scopes.
