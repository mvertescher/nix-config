# STORE primary printing and rifle details

The reference is `images/img-09-store.png` at 3840×2160. Trace and Rust coordinates use 1600×900 design units. This pass retains the established shelf opening, selected/cropped card contours, palette roles and interaction coats.

The photo prints `PETROCHEM` inside a 7.2×48.4 design box at card-local `(261.4,182.9)` and `BETTERLIFE TEC` immediately to its right. Both runs turn clockwise. The source PETROCHEM bright threshold spans x697.92..705.00/y182.50..231.67 on card 1; the updated SVG spans x697.92..705.83/y182.50..231.67. The source BETTERLIFE run is heavier and longer than the old trace: changing it to Bold 7.5 with 1.0 tracking increases its bright area in the clean x709..718.5/y182..245 crop from 116 to 474 native pixels, compared with the source's 374, and extends its end to y242.08, matching the source's y242.08. Its bright-threshold IoU rises from 0.04 to 0.31; the repeated printing and glyph shape still differ. Iced now carries both primary runs through `Prim::Turn` and `Prim::Tracked`, including selected and cropped card variants. The source's primary bright printing has p90 red 251 (`#fb3535`), where the old SVG microprint peaked near 150; SVG and the reference-palette STORE foreground use `#fb3535`. Custom palettes retain their own `Fg` role.

The left source margin code has bright native bounds x50.83..56.25/y382.08..468.33. The SVG's prior primary run spanned x50.83..57.92/y377.08..469.17, with an early top edge and extra width. The source's final group reads `0B CP`, with a straight left stem and two bowls in the B; the same literal appears in the unboxed STORE footer and the login margin. Earlier traces and Rust used `08 CP`, which has now been corrected in those runs. The small MASURAO word and 益荒男 brand under it preserve the source rotation. The kanji now uses 16 closed source-traced rings from native x117..133/y1317..1380, inverse-rotated into the existing `Prim::Turn` frame; its supported bright bounds are x48.75..55.42/y548.75..575.00 design units. The Latin word’s old font approximation has since been replaced by compact source-derived stencil contours, detailed below. The main Japanese MASURAO title has since been restored from native-source vectors in the [primary-art pass](store-primary-art.md), without depending on a CJK font or shear primitive. No extra unreadable microtext has been fabricated.

The corrected STORE margin run uses Rajdhani Regular 10.5, tracking 0.4, turned at (56.4,469). At native resolution, a common strong-red mask (R>180, R>2.2G, R>2B) puts the source at x120..134/y917..1123 and the updated SVG at x119..134/y918..1123. Its whole-run IoU rises from 0.043 with the old geometry and corrected B to 0.241. Independent vertical holdouts also improve: `CP/0B` y917..985 from 0.032 to 0.210, middle `54/05` y985..1060 from 0.039 to 0.105, and trailing digits y1060..1125 from 0.061 to 0.490. Source scan echoes and distressed glyph edges remain outside this primary-run fit. Native Iced baseline and width need review; login and footer geometry were retained.

The initial J pass ported the old SVG's `#gundetail` template to Iced. That schematic was then superseded by source-traced five-tone rifle contours for the ordinary and selected cards. The [primary-art record](store-primary-art.md) gives the native mask measurements and held-out threshold comparison. The art remains in the existing `GUN_OUTLINED`/`GUN_SOLID` state derivation, including card 4's permanent x1557 viewport.

The STORE source and SVG use 1 design unit of tracking on nav, title, stat and socket captions. Iced now matches that local tracking. EMPTY/SOCKET is Regular in the SVG and source appearance; its earlier Rust Bold run has been changed to Regular. The source cap heights, footer overflow and some primary label widths still need native fitting after this material/printing checkpoint. The STORE card/nav material proposal remains separate in `/tmp/cp-eras-completion/neomil-store-material-proposal.md`; no STORE material recipe was applied in this printing pass.

Source/trace review crops and candidate measurements are in `/tmp/cp-eras-completion/neomil-store-printing-review.png` and `/tmp/cp-eras-completion/neomil-store-better-bold.png`. J passed 265 Rust tests before the source-traced art integration; native acceptance of the latter remains pending.

## Integrated validation — 2026-09-29

N integrated validation confirms the restored rotated/margin printing and source-vector margin CJK. Both frozen and live store matrix cases now match 100.000%, eliminating the former fallback-font discrepancy. Primary printing presence is complete; Latin margin slant/placement and faint repeated/scan detail remain local follow-ups.

## Q native calibration

Q confirms the corrected B in the store and login, including the retained
login margin geometry. The store margin's native primary run sits about
one pixel right of the source; shifting its Rust origin one native pixel
left/up improves whole-run mask IoU from .176 to .398 at R>180. Independent
thresholds at R>160 and R>200 also improve (.193→.470 and .157→.333). The
source-fitted SVG stays fixed. The unboxed footer needs only a one-pixel
Rust baseline lift: mask IoU rises .229→.341, whereas moving it left worsens
that result. These are candidate renderer corrections pending the next
native capture, not a claim that fine glyph edges or echoes match.

## Card-edge PETROCHEM spacing — 2026-09-29

The unchanged primary run ended too early inside its box: after excluding
the box strokes, the terminal M reaches native y549 in both ordinary and
selected source cards, but y532/533 in the frozen R SVG and y533 in R native.
Increasing only PETROCHEM tracking from 0.5 to 1.3 design units puts the
scratch SVG terminal at y548 in both cards, without colliding with the box.
On locally registered bright-glyph masks, ordinary-card F1 rises
0.191→0.400 and the independent selected-card F1 rises 0.350→0.556;
upper and lower halves improve in each. Local x registration accounts for
the existing card placement offset during measurement and is not part of
the change. The candidate crop and method are recorded in
`/tmp/cp-eras-resume-20260929/s-store-margin/findings.md`. Rust and all
store trace/component card examples use the same 1.3 tracking; native
verification remains pending.

BETTERLIFE TEC needs no shared slant or position change: its ordinary
native bright-core x centers at the first, middle, and last letters match
the source within a fraction of a native pixel, and its y extent already
matches. The selected card has a separate frame placement difference;
moving this common text run would degrade the ordinary match.

## Native R code calibration

Fresh source/SVG/Iced crops accept the `0B` correction in both screens.
The store margin uses a separate one-native-pixel runtime shift in each
axis; its bright-mask source/native overlap improves .176→.398. Native
bounds are x120–135/y916–1122 against source x120–134/y917–1123. The footer
moves one native pixel upward, retaining its fitted width: native and SVG
bounds are x753–959/y2082–2096 against source x752–958/y2082–2096. Source
photographic echoes and exact glyph density remain separate. Fractional
rest and custom-palette captures preserve semantic footer ink. The R
source and implementation gates pass; final integrated validation is pending.

### T PETROCHEM native review

The updated Iced run reaches the photographed lower-letter region in both
ordinary and selected cards, retaining clearance inside its box. Fractional
held ordinary and selected captures preserve dark feedback ink and the
card clipping. Both fidelity gates pass. The selected card's independently
measured frame and side-printing offset is a separate open correction;
tracking does not justify translating its otherwise aligned contents.
The integrated T check passes all 22 checks and all 27 visual cases exactly.

## Directional socket echoes, 2026-09-29

The photographed 25-cell scatter codes have a second printing pass: ordinary card 1 trails left, ordinary cards 3/4 trail right, and the selected card trails down. The primary 3×3 cells, 3.6667-unit pitch, independently fitted origins, inks and hit geometry remain unchanged. The reference trace now adds a source-color `#fb1818` echo behind each cell through a shared horizontal falloff and a continuous vertical scan-band mask. The card-4 ordinary echo is shifted up 0.4167 design unit, fitted as a constant phase from its first isolated tail. The selected downward tail uses its own roughly five-native-pixel scan period and decay, repeated from one isolated cell. These compact gradients reproduce the broad direction and dark bands; their regularity and edge exposure do not claim exact photographic grain.

Scratch source/V-SVG/candidate measurements are in `/tmp/cp-eras-resume-20260929/z-store-echo/findings.md`. Across changed pixels, card-4 baseline/smooth/banded-with-phase RGB MAE was 26.95/15.98/15.80; held-out row 4 improved from 19.39 to 16.88 and row 8 from 15.86 to 15.07 versus unshifted banding. Selected downward periodic printing improved smooth 13.83→12.73, including a disjoint top-row column 10.56→7.36 and bottom-row 13.22→11.77. Some supported bright pixels and dark-gap negatives remain worse than their alternate approximation, so this is a bounded follow-up, not exact scan-texture closure.

`store_material.rs` places idle echoes inside the existing card soft composite and opening clip; the source-only feedback state table redraws the same local pattern after the opaque hover/held coat and immediately before unchanged `Prim::Dots`. Constant-opacity `Prim::Motion` wrappers fade `Ink::Fg` and its existing held `card_ink` color without a new renderer primitive or image layer. `Style::store_states_for` uses the same full-palette/source-backdrop eligibility as `store_layers`; custom palettes and other eras continue to use their existing semantic states. The fourth-card viewport still clips the last card. The source selected card is card 2; alternate picked slots use the same grown/downward profile as an inferred interaction state. Ordinary card 2 uses the ordinary-left profile while unselected, also inferred. Native RGB, performance, alternate-selection and feedback checks remain pending the serialized build/capture.

### AA native and feedback review

The final continuous SVG is verified against the native Iced result, not
just the earlier strip-based scratch trial. Source RGB error on a fixed
echo footprint improves on all four cards (19.76→11.92, 26.46→11.95,
20.09→12.51, 23.34→14.47); bright primary cells remain unchanged. All
19,165 native changes are inside the four socket clusters. Ordinary dark
gaps retain small residual errors, and exact grain remains approximate.
All selected slots, ordinary/selected/fourth-card feedback, custom palettes
and early opening are reviewed. Custom rest is pixel-identical to the
previous state. Ordinary feedback adds about 5.27 ms in the paired local
CPU geometry probe; selected feedback adds .13 ms. See the
[AA checkpoint](../reference-svg-round12.md) for method, gate results and
integrated validation. This correction left the separately misplaced small MASURAO margin word and adjacent rule for the bounded contour pass below.


## Small margin word and slash — bounded source contour pass

The former long 107-unit rule crossed the source word and accepted margin kanji, while the italic font word landed much lower than the photographed stencil. A native-source crop isolates the six clear Latin glyphs and a short lower slash. The revised trace and `store_margin.rs` retain the existing `translate(52,584) rotate(-90)` frame and leave the margin-kanji definition and use unchanged. The six letter paths are simplified from a strong red mask (R>100 and R>1.5G/B): components under four native pixels are removed, only one-pixel diagonal breaks are joined, and closed boundaries are simplified by one native pixel. This yields 20 compact nondegenerate rings and 181 vertices, with no bitmap or photographic grain path. The slash is a measured 1.8-unit stroke between local `(8.94,-5.19)` and `(3.00,5.06)`. Rust uses semantic `Fg` for the word, slash and provisional hatch, with a constant .67 opacity on the hatch outside the rotated group; the dark O ring uses semantic `Bg`. Custom palettes retain these ink roles.

On fixed native-resolution source/SVG crops, first-to-last glyph bright-mask IoU improves M .009→.633, A .022→.656, S 0→.679, U 0→.482, R 0→.626, and second A .042→.563. False bright pixels fall on all six (ranges 70–141 before and 37–59 after). The slash improves .119→.667, and the untouched kanji path improves .496→.947 because the old word and rule no longer overprint it. The terminal O is now a dark ring with a red counter over clipped diagonal hatch, but the broad plaque material and second-A junction remain approximate: its local RGB error falls 31.83→25.00 while false bright pixels rise 48→153 and bright intrusions into source-dark gaps rise 3→37. A rejected solid plaque fill raised those intrusions to 76. This is a primary word/placement correction, not exact O/plaque or fine photographic echo closure. Scratch source/current/candidate crops and per-glyph counts are in `/tmp/cp-eras-resume-20260929/aa-masurao-margin/`; native renderer validation is pending.


### AB native confirmation

The native word/slash correction changes3054 pixels at 4K and 699 at 1600×900,
all within the margin. Nine matched production/state comparisons change
only that region. Fractional card-held/selection/opening margin crops are
identical to fractional rest. A custom cyan foreground renders423 cyan
margin pixels and zero strong red pixels, including the hatch. Native
per-glyph RGB errors improve on all six letters; CJK overlap rises
.485→.950 and slash overlap .121→.727. The native O/plaque RGB error falls
32.06→25.55, but source-dark-gap intrusions rise3→55. This and the A2
junction remain explicit residuals, not closed exact-material work.
Both store gates and the full AB check pass: 286 Rust tests, all 22
repository checks and 27 exact visual cases on their first attempt.

### AC leading plaque boundary

The rectangular hatch end extended beyond the source's slanted leading
edge. Trimming that edge changes only 126 native 4K pixels and 32 at 1600
or1537×947. Leading-patch RGB error falls 20.20→14.52; bright intrusions
into source-dark pixels fall 88→4. The O/plaque crop improves25.55→23.09
and 55→6 intrusions, while the O counter and A2 are unchanged. Ten native
captures preserve card states, opening, clipping and semantic custom ink.
Both gates, 286 Rust tests and all 22 repository checks pass. All 27 visual
cases match 100.000% on their first attempt. The
[AC checkpoint](../reference-svg-round14.md) records the rejected wider
clip/dimming proposals and preserves the remaining fine-material limits.
