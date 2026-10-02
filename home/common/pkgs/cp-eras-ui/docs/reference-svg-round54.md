# Reference SVG round fifty-four — donor, silhouette and subtitle controls

The next three studies sharpen the remaining source-fidelity tasks. Kitsch
donor photos do not support the rejected Vehicles red brightening;
Neo-kitsch isolated-letter scores do not identify a replacement font;
Neomil's missing ordinary subtitle impression is confirmed, but the tested
copies retain material local losses. No runtime, SVG or golden change is
accepted. All 29 broader scopes remain open; CZ remains the latest tested
implementation.

## Kitsch Vehicles donor evidence

DS samples the original Mail and Store photos at the frozen CX Vehicles
depth-2/depth-3 coordinates. Model foreground support is any RGB difference
between the full and ground-only render. Euclidean guards of 8/16/24 design
pixels are applied before reading source/model colors; samples are not
selected by their residuals. Astra reconstructs both masks, all coordinate
and sample arrays, spatial bins and 78 statistic groups exactly.

At guard 16, depth 2 retains 9,592 Mail and 15,292 Store pixels, with 7,993
common coordinates. Original-donor minus current-ground mean red is
−3.683/−3.345; versus coupled DP it is −17.140/−16.295. Depth 3 retains
5,264/13,230 pixels, with only 944 in common: current residuals are
−6.205/−8.422, versus DP −19.952/−19.951. Negative current residuals
persist at all three guards. The depth-3 common intersection shrinks to
47 pixels at guard 24, so stronger guards also reduce coverage sharply.

The original-context images retain nearby shading and UI that the model
mask need not describe. These are donor-photo diagnostics, not direct
observations of Dashboard ground hidden under a card, and do not identify
its alpha. They provide no support for DP's large positive red shift.
Do not automatically refit accepted card material to compensate. A new
ground proposal must respect these source-local signs as well as the
composed-card controls from round fifty-three.

## Neo-kitsch distinctive-letter diagnostic

DS predeclares fourteen letter windows from the seven Dashboard module
labels. EMAIL's E supplies registration; EMAIL's M is same-label context,
and twelve other glyphs provide cross-label controls. Transparent, unfiltered
Rajdhani and Exo templates retain their prior font sizes, tracking and
anchors. The Exo size is the actual DP value, including its documented
arithmetic discrepancy; it is not silently corrected.

Source E at R210 has bounds [467,1091,481,1120). Alpha-128 template bounds
give Rajdhani dx +4 and vertical scale 29/26, and Exo dx −3 and scale 29/24.
Both bottom offsets are zero. One shared registration is applied around
each label's own unchanged baseline, avoiding a global EMAIL-centered
vertical displacement. The 1600 comparison uses directly rendered templates
and the whole-photo Lanczos reference.

| Registered fixed-window aggregate F1 | Rajdhani | Exo |
| --- | ---: | ---: |
| 4K, R140 | .419 | .277 |
| 4K, R210 | .391 | .266 |
| 1600, R140 | .329 | .093 |
| 1600, R210 | .270 | .113 |

These fixed boxes can truncate displaced glyphs or admit neighboring ink.
A declared supplemental union includes each complete template glyph, using
its known character ordinal and the same registration. Source pixels remain
restricted to the original windows; this is not an independently recovered
source contour. The 4K union R210 F1 is .352/.254, with Rajdhani winning
10 of 14 glyphs. Training E is not validation. Weight, photographic glow,
advances and per-word position still confound the comparison.

The independent worker audit verifies all 30 input hashes, 448 fixed-window
and 112 union metric groups, with 839 numeric comparisons matching. Astra
reviews that implementation and the bordered source/template montage, and
separately reproduces all 448 fixed-window TP/FP/FN/F1 groups using direct
window sampling. This rejects neither font family categorically and accepts
no new scene fit. The diagnostic is complete; do not redispatch it as an
unperformed font-identification step. A next correction needs a specific
source-supported contour or registration hypothesis with scene controls.

## Neomil ordinary subtitle impressions

Wider source crops reveal displaced `HAND GUN` printing below the accepted
`MAGNUM 650` title. It is a subtitle impression, not another lower title
copy. The selected card uses different printing and is unchanged throughout.

DS first tests one common (−5,−6) original-pixel shift at .25 opacity,
clipped to rows [500,507). Card 1 improves, but cards 3/4 worsen at 4K.
The baseline and no-op renders match in full RGBA at both sizes under the
established renderer and Fontconfig. Astra independently rerenders the
candidate and reproduces all 96 regional metric groups.

DT reuses the accepted title model's relative per-card registration, adding
the same (−2,0) offset to its main impression: final subtitle shifts are
(−5,−6), (+1,−7), (+7,−7) for cards 1/3/4. This single predetermined trial
improves regional averages and fixed gap totals at both sizes. Its upper-only
clip nevertheless truncates the impression into disconnected cap fragments.
The source supports continued contours below the clip, so this fragment
model is not ported.

DU removes only that clip, retaining the complete glyph, opacity, font,
primary and three shifts. All ordinary extended-crop RGB totals improve:

| 4K extended RGB L1 | Current | DU full copy |
| --- | ---: | ---: |
| Card 1 | 1,132,010 | 1,062,795 |
| Card 3 | 924,568 | 905,141 |
| Card 4 | 1,096,804 | 1,019,524 |

Local controls prevent acceptance. Card 3's source-dark RGB L1 rises
395,938→397,937; 361 such changed pixels worsen. At (2769,506), source
(19,33,73), current (21,35,72) and candidate (79,39,67) yield a 22.33-level
mean RGB loss. Fixed-gap and filled-counter totals improve but do not erase
those losses. The 1600 trial also retains localized dark/bright losses.
Every fully opaque primary pixel, selected card and alpha plane is exact.
New subtitle pixels at row 499 overlap the former title crop boundary;
this is not evidence that accepted title strokes changed.

### Actual native trial

DU is also built in an isolated source snapshot using the existing title
echo list and reusable glyph primitive. No renderer or outer card indices
change. Native baseline 222.4 is retained, versus SVG 223.4, with the same
relative shifts and semantic normal/held inks. This is a scratch native
trial, not an integrated fix.

The actual Store baseline exactly matches the latest package at 4K, the
accepted CJ captures at all three sizes, and the 1600 golden. All changed
pixels stay in ordinary title/subtitle crops; selected card, full alpha and
opaque primary ink are exact at 3840×2160, 1600×900 and 1537×947. Native
extended RGB L1 improves on each card at every size. However, card 3 still
has 369/84/51 source-dark losses, with maxima 18.33/18.00/18.33 RGB levels.
At 4K, 292 exceed three levels. The whole dark mask improves slightly in
native rendering, unlike SVG; that does not resolve the concentrated losses.
No state matrix, performance claim or production port is warranted yet.

### Five-phase opacity trial

DV freezes a five-original-row opacity cycle before fitting. Only card 1
trains it: model-owned samples have current red contrast below 40 and
full-opacity impression minus current red above 80. No source-color filter
selects samples. Each phase minimizes red-channel squared residual with
alpha bounded to [0,.35]; geometry and primary remain unchanged. The result
is .35/.299787/.220158/.266557/.35, with two phases at the upper bound.
These modulate strength but provide no off rows.

The exact fitted candidate fails transfer. Card 3's 4K extended RGB L1 is
908,316, worse than DU's 905,141. Its source-dark total rises to 403,021,
with 405 worsened dark pixels and a worst mean RGB loss of 31.33. At 1600,
overall error is nearly unchanged versus DU while dark losses increase.
Cards 1/4 improve overall but retain local losses. Astra and the worker
reproduce the fit and regional results; source/current/DU/DV montages show
that a stronger periodic full copy does not recover the photographed
interruption and registration. DV is not ported. These previously inspected
cards are transfer controls, not blind holdouts.

The next subtitle task is a source-supported registration/contour and
interruption model that explains card 3 locally. Do not repeat a common
translation, upper-only clip, uniform complete copy or this five-phase
amplitude fit, and do not carve out individual failed source pixels.

## Verification boundary

Scratch evidence under `/tmp/cp-eras-next/`: `ds-kitsch-vehicles-donors`,
`ds-neokitsch-letter-shapes`, `ds-neomil-store-printing`,
`dt-neomil-subtitle-registration`, `du-neomil-subtitle-impression`,
`du-native`, `dv-subtitle-periodic`, `du-worker-review`,
`du-neokitsch-audit`, `dv-worker-review`, `ds-root-review` and
`du-root-review`. Frozen input manifests, scripts, numeric checks and
montages retain the unsuccessful trials for the next task.

This round changes only prose. Runtime, SVGs, fonts, scripts, goldens,
locks and all sixteen original images remain unchanged. Existing staged
work is preserved; no commit, push, deployment or live GUI action occurs.
CZ's 295 Rust tests and complete repository/matrix checks remain the last
integrated verification. Fresh DU compilation and seven headless captures
verify only the scratch experiment. No broad checkbox closes.
