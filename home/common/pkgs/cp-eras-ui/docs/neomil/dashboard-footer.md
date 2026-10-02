# Dashboard footer type and printing (NM1)

Source: `images/img-07-dashboard.png`, screen #60, 3840×2160. The affected
design crop is `(1195,862)..(1372,899)` on the 1600×900 canvas. The trace,
component excerpt, and Neomil scene table use the same literal code,
captions, two-cell frame, and divider. The native-only code calibration below
changes its three scene runs; no shared text or scene renderer was changed.

The former Rajdhani Bold 8 code and Semibold 7.5 captions rendered too
short. In the SVG, each run has its own baseline and horizontal fit: Bold 9.8 at
`0.844×` for `68SD1D1100D1S`, Semibold 9.5 at `0.79×` for `COMBAT
COLONIZATION`, and Semibold 9.5 at `0.807×` for `DEFENCE PROGRAM`. The
horizontal fits preserve the narrow source lettering while raising its cap
height. A size-only change would have made the already correct first caption
too wide.

Native source and SVG render use the same `R > 170, R > 2G` red-core mask.
Coordinates below are in design pixels after dividing native pixel positions
by 2.4. These are thresholded ink boxes, so the exact equality does not
establish identical glyph contours or photographic presentation.

| Run | Source box `(left,top,right,bottom)` | Revised SVG box | Source / SVG cap height |
| --- | --- | --- | ---: |
| Code | `(1214.58,868.75,1265.00,875.00)` | same | `6.25 / 6.25` |
| First caption | `(1276.67,868.75,1345.83,874.58)` | same | `5.83 / 5.83` |
| Second caption | `(1277.08,877.08,1335.42,882.92)` | `(1277.08,877.08,1335.83,882.92)` | `5.83 / 5.83` |

The old frame copy was a single sharp, dark 1px rectangle shifted three
pixels. In the source, dim frame ink spans several rows under both the top
and bottom edges and several columns beyond the right edge. Two broader
red copies with small offsets and `0.25/0.13` opacity fit those regions;
two local text copies at `(+0.8,+1)` and `(+1.8,+2)` with `0.23/0.11`
opacity fit the visible low-intensity lettering around the red cores.
These are translucent local canvas copies under the primary marks. They use
the dashboard foreground role in the app, so custom palettes still recolor
the whole footer together. They describe observed ink, not the source
renderer. No glow, grain, or global noise was inferred.

The table reports mean absolute red-channel error in separate top, bottom,
right-edge, and text crops after resizing the source with Lanczos to
1600×900. The numbers compare complete crops,
including local background and antialiasing.

| Crop | Before | Revised |
| --- | ---: | ---: |
| Top frame, left / right | 43.9 / 45.0 | 26.8 / 27.7 |
| Bottom frame, left / right | 56.9 / 58.1 | 35.1 / 36.2 |
| Right frame edge | 37.1 | 26.9 |
| Code | 53.1 | 33.5 |
| First / second caption | 69.6 / 56.1 | 32.6 / 27.1 |

The final native 3840×2160 Iced capture shows the
same direction of improvement. In source / before-Iced / revised-Iced
order, red-core cap heights are `6.25 / 5.42 / 6.67` for the code,
`5.83 / 4.58 / 6.25` for the first caption, and `5.83 / 4.58 / 6.67`
for the second. The canvas rasterizer therefore paints the fitted runs
0.42–0.84 design px taller than SVG; the source-to-Iced match is improved,
but remains less exact than source-to-SVG. Native red-channel crop error
improves for code `55.7→49.0`, first caption `90.9→48.7`, second caption
`71.8→42.3`, and every separately sampled frame edge.

At 1600×900, the changed SVG pixels are confined to
`(1210,865)..(1358,893)`. Before/after Iced capture changes are likewise
local: native `(2904,2076)..(3259,2143)` and fractional 1537×947
`(1162,831)..(1305,858)` in output pixels. The source image's compression, downscaling,
and unknown printing process leave the exact edge profiles uncertain.
The source also shows fine horizontal striations and softer repeated ink
that these two broad copies do not reproduce. The local copies are a
measured approximation; exact faint printing and live desktop behavior
remain open for the orchestrator's final review.

## Native code cap-height calibration (2026-09-29)

At 3840×2160, the first source `6` has a bright-core box of
`x=2915..2923, y=2085..2099`. The restored native capture painted the same
horizontal box at `y=2086..2101`, even though the SVG box matched the source.
Four native previews changed only the primary code run and its two local
copies. The unmodified control was pixel-identical to the restored capture.
The chosen Bold 9.2 preview moved all three baselines up by `0.833333` design
px and used the computed horizontal stretch `0.844 × 9.8 / 9.2` to retain
their nominal width. Rust now uses these exact expressions; the trace and
component SVGs retain the source-aligned Bold 9.8 fit.

| Independent native region | Restored / calibrated bright-core IoU | Restored / calibrated RGB MAE |
| --- | ---: | ---: |
| First `6` | `0.500 / 0.702` | `25.017 / 15.694` |
| Held-out `8` | `0.475 / 0.680` | `23.501 / 18.109` |
| Remaining code | `0.416 / 0.544` | `29.064 / 24.191` |
| Whole code | `0.428 / 0.569` | `28.237 / 22.956` |

The calibrated first `6` and held-out `8` match the source's thresholded
vertical boxes `y=2085..2099`. The trial changes 1,749 pixels, confined to
`x=2915..3040, y=2085..2106`; the frame, divider, captions, and all pixels
outside the code region remain identical. The later glyphs still have
horizontal phase and contour differences, particularly the last `S` at
`x=3025..3036` versus source `3026..3035`. Semibold improved that later
region further, but worsened the first `6` and left a one-pixel bottom
overshoot. This is a bounded native code correction, not a claim of exact
photographic printing fidelity. The Z production capture matches the
approved trial at every pixel. Fractional rest, custom, opening and held
comparisons change only the code lettering; frame/caption controls are
unchanged, and the no-config fallback matches the reference dashboard.
Both fidelity gates pass. Integrated verification is recorded in the
[eleventh checkpoint](../reference-svg-round11.md).

## Native caption cap-height calibration (2026-09-29)

The two remaining captions still painted taller in Iced than the source
and SVG. At 4K and the `R > 170, R > 2G` mask, the first source caption
occupies y2085–2098, versus Iced y2085–2099; the second occupies
y2105–2118, versus Iced y2105–2120. Native SemiBold sizes 9.0 and 8.75,
with both baselines shifted by −0.416667 design px, recover those vertical
boxes. The stretch expressions preserve each nominal width. The SVG's
source-aligned sizes, both local echo offsets, foreground role, code,
divider and frame remain unchanged.

The unmodified scratch preview is pixel-identical to the prior native
dashboard. The accepted preview reduces whole-line RGB MAE 22.73→21.16
and 18.72→16.59. Red-core F1 improves .645→.676 and .708→.783; both
lines also improve at thresholds 150 and 190. Independent first, middle
and last sections all improve RGB error. Five of the six sections improve
core F1 at all three thresholds; the last part of COLONIZATION drops
.519→.514 at 170, losing nine matching core pixels while removing twenty
excess ones. That residual and the one-pixel right-edge overshoot remain
explicit. A first trial moved DEFENCE PROGRAM one pixel too high and is
rejected. No per-glyph warping was introduced to compensate for the font.

Source/SVG/native crops and scripts are in `/tmp/cp-eras-next/ad-footer/`.
The production capture is pixel-identical to the approved preview. All
2,531 changed 4K pixels stay at x3064–3234/y2085–2125, within the two
captions and their local copies. Fractional rest, held, custom and opening
comparisons likewise change only those captions; the frame, divider and
code remain identical to their prior state captures. The reference and
no-config fallback agree at every pixel. AD passes 286 Rust tests, both
dashboard fidelity gates, all 22 repository checks and 27 exact visual
cases. The [fifteenth checkpoint](../reference-svg-round15.md) records the
frozen-source verification. Exact glyph contours and faint photographic
printing remain separate from the bounded height correction.

## AY secondary-frame audit (2026-09-30)

The source's horizontal striations repeat across both footer cells. Fixed
4K interior windows use x2935–2981 for fitting and x3079–3125 and
x3170–3216 as opposite-cell controls. An additive foreground-strip trial
improves the 4K samples but worsens all three bottom samples at 1600px;
it is rejected. Its initial fractional render was incorrectly centered
and is discarded as evidence. Repeating that diagnostic with a top-left
1537×947 scene improves the edge samples but does not cure the 1600px
failure.

A second trial replaces only the broad copies' straight horizontal
interiors, retaining their corners, vertical edges and all primary/text
geometry. One seven-row profile fits both top and bottom training windows
and transfers to both opposite-cell controls at 4K. It still worsens the
1600px whole-footer RGB error from 10.940 to 11.101 and the bottom-right
control from 5.626 to 5.637. The text-boundary crop also worsens. Exact
top-left fractional coordinates produce favorable diagnostic results;
the unknown photographic downsampling process remains a limitation.

Replacing secondary coverage exposes a separate primary-edge mismatch:
at 4K y2076 the source red excess is 198 versus 19 in the original SVG,
and removing the broad copy leaves zero. The fixed primary-top crop
therefore worsens from 38.453 to 41.258 despite unchanged primary geometry.
Do not compensate for that mismatch with brighter secondary strips.
Both proposals are rejected before a native implementation. Astra
reproduces the final replacement metrics and reviews the source/SVG
crops; all production artwork remains unchanged. The next investigation
must isolate primary edge registration from secondary ink and retain
independent size, text, corner and blank controls.

Scripts, frozen profiles, all trial variants and every measured loss are
in `/tmp/cp-eras-next/ay-neomil-footer/` and
`/tmp/cp-eras-next/ay-neomil-footer-replacement/`.

The follow-up primary-edge audit separates bright cores from faint copies
using R>170 and R>2G in the same three windows. Source/SVG/native 4K top
spans are y2073–2076 / 2074–2075 / 2073–2075; bottom spans are
y2129–2132 / 2131–2132 / 2131–2133. Top width and bottom registration/width
therefore need separate treatment. At 1600px the source bottom retains
rows 887–888 versus row 888 in both renderers. Fractional diagnostics show
the same direction; filtering prevents inferring exact geometry from a
single threshold. The verified AD native footer region matches a fresh
AX packaged 4K capture exactly. Frozen profiles in
`/tmp/cp-eras-next/ay-neomil-primary/` support a narrow primary-horizontal
frame task, preserving vertical sides, divider, lettering and echoes.

## AZ bottom-edge coverage (2026-09-30)

The primary bottom is missing bright coverage above its current stroke.
The source's 4K core spans rows 2129–2132; native previously covered
2131–2133 at R>170 and R>2G. An added foreground rectangle occupies
x1210.5–1352.5 and y2129/2.4–2133/2.4 in design coordinates, retaining
the original rim, divider, text and faint copies. The SVG, native table
and dashboard component excerpt use the same bounds.

Fixed left-cell training and two opposite-cell controls improve in SVG
and native at 3840×2160, 1600×900 and top-left 1537×947. Native whole-footer
RGB MAE changes as follows:

| Size | Before | After | Changed pixels |
| --- | ---: | ---: | ---: |
| 3840×2160 | 12.266 | 10.774 | 1,014 |
| 1600×900 | 11.306 | 9.653 | 142 |
| 1537×947 | 13.508 | 11.484 | 137 |

Every changed native pixel improves the source RGB comparison; all fixed
regional RGB/core-overlap checks improve or tie. The top/text crop and
alpha are identical. Fractional source resampling is a diagnostic with
an unknown original photographic filter, not exact authoring evidence.
Nine paired native cases cover these three rest sizes, first/last selection,
synthetic held feedback, five custom palette roles, opening at 0.12s and
the no-config fallback. Changes stay in the bottom band in every case;
the fallback equals the reference at every RGBA pixel. These are headless
drawing-state checks, not live pointer or session verification.

This is a bounded improvement, not a completed frame reconstruction.
The retained native stroke still paints bright row 2133 at R>150/170,
while the source stops at 2132. At R>190 the new upper row 2129 is brighter
than the source core, which starts at 2130. Exact edge softness, the
top's missing row and secondary striations remain open. A combined top
and bottom trial fails fractional top controls. Both a corner clip and
a separate scene-wide bottom clip also alter unrelated fractional primary
edge rasterization. The latter changes 322 frozen top/text pixels and
worsens the fractional bottom crop; neither is integrated.

The component sheet's band changes 142/144 pixels at full/half size. Its
updated explanatory caption changes a separate 519/183 pixels; all other
RGBA pixels match. Source/native controls, immutable capture inputs,
rejected clips and component comparisons are under `/tmp/cp-eras-next/az/`,
`az-native/`, `az-bottom-review/`, `az-state-review/`, `az-neomil-bottom/`,
`az-bottom-clip/` and `az-components/`. All three production sizes and the
fallback match the reviewed candidates. Both gates, 292 Rust tests, all
22 repository checks and 27 first-attempt visual cases pass. A fresh
packaged 4K dashboard also matches exactly. Only the Neomil and fallback
dashboard goldens change, by 142 pixels each. See
[round thirty-seven](../reference-svg-round37.md).


### Subsequent fixed-height control

Before another native trial, a single scratch SVG shortens only the primary
rectangle from height 24 to 23.75, retaining the accepted band. This moves
the old stroke's lower boundary to y888.75 without a clipping surface.
Top/text/blank controls stay exact; the side-to-bottom joins necessarily
move. At 4K, footer RGB MAE improves 11.160→10.758 with 355 pixel gains and
two losses. At top-left 1537×947 it worsens 10.414→10.453 with 137 gains
and 140 losses. All three straight-bottom windows and both lower joins
worsen there; the strict 4K upper-core excess also remains. The trial is
rejected before native work. Astra reproduces every regional RGB result
and verifies the single XML change. Evidence is in
`/tmp/cp-eras-next/ba-footer-height/` and
`/tmp/cp-eras-next/az/ba-height-source-review.json`. This later diagnostic
does not change the tested AZ artwork.

## BA top-edge and side audits (2026-09-30)

After AZ's accepted bottom band, the source's 4K top primary core still covers
y2076 across a left-cell train and two independent right-cell windows. The AZ
native row there is dim (R≈48 versus source R≈251), while native y2073 is
already close to source (R≈175 versus 181). At 1600, the lower-adjacent row
y865 is source/native/SVG R≈154/59/47; at explicit top-left 1537, y831 is
≈99.5/59/51. This supports one lower-edge coverage trial without changing the
upper edge. An earlier band spanning the whole 4K source core is rejected
because it paints fractional y829 above the source edge and worsens all three
top windows.

The one-row top-only SVG adds `#ef3333` at x1210.5–1352.5, y865–865.416667
after the unchanged rim. Frozen 4K/1600/fractional whole-footer RGB MAE
changes 11.160→10.318, 8.701→7.933 and 10.414→10.141. Every changed 4K/1600
pixel gains; 271 of 272 fractional pixels gain, with one loss at the left
corner `(1163,831)`. The source shows continuous bright ink into both inner
rim joins. A second, single-width audit extends **only** that band to
x1210–1353: all four 4K, two 1600 and two fractional changed endpoint pixels
improve RGB error, with no endpoint loss. Direct comparison of the joined
proposal against AZ gives whole-footer MAE 11.160→10.313 (342 gains/0 losses),
8.701→7.928 (142/0) and 10.414→10.138 (271/1). The same fractional corner loss
remains; this is not exact edge filtering. The accepted bottom band, frame,
divider, text, caption copies and secondary frame copies remain fixed in both
SVG trials. The proposed component top-band/caption patch is **not** applied
because the native joined-top trial fails below.

**The joined-top native trial is rejected.** The frozen actual Dashboard
harness adds only `fill_rect(1210.0, 865.0, 143.0, 1.0/2.4, Ink::Fg)` after
the old frame, retaining AZ's accepted bottom band. Its baseline matches AZ
production at 4K, 1600 and 1537. Candidate changes 341/142/137 RGBA pixels
within the top edge, with no alpha, text, bottom or blank changes. All 341/142
changed 4K/1600 pixels improve source RGB L1 error; 136 of 137 fractional
pixels improve. Whole-footer RGB MAE falls 10.774→9.982, 9.653→8.968 and
11.484→11.355. Those averages hide a decisive 1600 threshold failure: source
row865 has mean red ≈154, while native rises from ≈59 to ≈180, creating an
extra R>170 core row. Each straight train/holdout F1 drops 1.000→.667,
top-primary F1 .9958→.6742, and the strict checker records ten regional/join
F1 losses. No further state/fallback trial was run, and no top-band,
component, golden or production change is accepted. A future bounded native
calibration would need to preserve the 4K missing-row gain without creating
that 1600 false core. The old native bottom stroke still has bright row 2133,
and the AZ upper-bottom row remains too bright at strict R>190. A separate
fixed-height attempt improves 4K but worsens fractional bottom windows and
joins, so it also remains rejected.

BB's read-only side audit isolates another primary mismatch. Source left 4K
vertical core is x2902–2904; AZ native is x2901–2903, with excess outer-left
and missing inner-left coverage. At 1600 the shared bright column is x1209 but
the adjacent source red is stronger; at 1537 source covers x1161–1162 while
native covers x1161. The right native core aligns with source at 4K. This is a
left-specific phase/registration problem, **not** evidence for widening both
sides. No side geometry trial was accepted. Top joins do not close that
long-side deficit, and the faint secondary striations and unknown photographic
filter remain open.

Frozen evidence and all losses: `/tmp/cp-eras-next/ba-footer-top/`,
`ba-footer-joins/`, `ba-dashboard-joined-native/`, `ba-dashboard-review/`, and
`bb-footer-sides/`. The 1537 source uses an affine/Bicubic diagnostic
approximation; native uses its responsive scene transform. This rejected trial
and read-only side audit do not close NM1 exact printing or live desktop
verification.

### BB left-edge phase trial

A subsequent BB SVG trial moves only the original frame's left edge right by
1/2.4 design pixel, reducing its width by the same amount to keep the right
edge fixed. It is rejected: all three left straight-window RGB errors and both
left joins worsen at all three sizes. At 4K it changes 178 pixels, with 58 RGB
gains and 120 losses; at 1600 the middle/lower left F1@170 falls from 1 to 0.
The missing inner-left column gains ink, but the existing source-supported
left core loses it. Right, text, distant top/bottom and blank controls remain
exact. Astra reproduces all regional RGB measurements and verifies the sole
x/width mutation. No native trial or production edit follows; this does not
prove how a separately calibrated native contour would behave. Frozen evidence
is in `/tmp/cp-eras-next/bb-left-phase-trial/` and
`/tmp/cp-eras-next/ba/bb-left-source-review.json`.


## BF calibrated top-edge coverage (2026-09-30)

The missing primary top row now has a bounded native correction. A constant
.80 opacity wraps only the new 143×(1/2.4) foreground-role band at (1210,865).
The original rim, sides, divider, accepted bottom band and all lettering
stay unchanged. At 4K the added train row rises R48→194 versus source251;
at 1600 it rises R59→149 versus source mean154.21. This avoids BA's false
R>170 row while retaining the missing-row gain. The 1600 R>150 overlap
still equals .667, so the edge filter is not an exact reconstruction.

| Whole-footer RGB MAE | 4K | 1600×900 | 1537×947 |
| --- | ---: | ---: | ---: |
| Native before → after | 10.773708→10.177267 | 9.653327→8.744122 | 11.484323→11.147865 |
| SVG before → after | 11.159583→10.312567 | 8.701183→7.927803 | 10.414115→10.137865 |

Both backends improve or tie all 39 fixed regional RGB and 117 red-mask
comparisons at R>150/170/190. The separately calibrated SVG uses the full
#ef3333 band; its opacity cannot be equated to native compositing. Native
changes 341/142/137 RGBA pixels, with no alpha or exterior changes. Every
changed 4K/1600 pixel improves source RGB L1; one fractional pixel at
(1163,831) worsens 39→43. That retained corner loss and the existing lower
overshoot, side phase, strict bottom-core excess and fine secondary ink
keep NM1 open.

Nine paired drawing cases cover three rest sizes, first/last selection,
synthetic held feedback, custom palette, partial opening and no-config
fallback. Production parity at all sizes, fallback parity and fresh packaged
4K parity are exact. Components change only in the footer specimen and its
caption at full/half size. Both gates, 293 local/Nix Rust tests, all 22
repository checks and 27 first-attempt visual cases pass. The two dashboard
goldens change by 142 pixels each. See [round forty-one](../reference-svg-round41.md)
for full evidence and the unchanged historical bar residual. This is
headless drawing verification; live desktop behavior remains separate.

## BT inner-left coverage (2026-09-30)

The missing 4K x2904 coverage spans rows [2077,2131), between the accepted
top and bottom joins. A strip at (1210,865+1/2.4), width 1/2.4 and height 22.5
adds that coverage without moving the source-supported core or right edge.
The SVG uses full foreground ink; native uses separately calibrated
constant .88 foreground-role opacity. Full native opacity was rejected
because it creates a false R>170 shoulder at 1600, despite improved RGB.

The calibrated strip improves or ties all 45 fixed regional RGB and 135
R>150/170/190 comparisons. Every changed pixel improves RGB error; changes
are confined to 54/23/22 pixels at 4K/1600/1537, with unchanged alpha. The
existing outer-left false core, strict bottom excess, photographic softness
and secondary striations remain open. The matching component changes only
23/12 specimen pixels at full/half size.

Eight paired state/fallback controls preserve locality. Selection and held
fixtures equal rest because this theme does not expose distinct feedback
for those settings; they are not live pointer verification. Custom colors
prove foreground-role routing, opening exercises the panel animation, the
no-config fallback matches rest, and the Kitsch control is exact. BT ports
the reviewed artwork and verifies production/package parity, both gates
and the full repository check. Only Neomil/fallback dashboard goldens
change for this correction, by 23 pixels each. Combined verification is in
[round forty-three](../reference-svg-round43.md).

## CF straight outer-left coverage — 2026-09-30

The native primary frame's 4K x2901 shoulder was (175,35,35), while the
source is about (106,24,23) and SVG about red 98. BT's inner strip affects
x2904 and leaves this shoulder unchanged. An exact three-size replay
replaces the stroked rectangle with an equivalent filled ring; moving
only the outer boundary to x1209.1 between y868.5 and 884.5 then reduces
the straight shoulder to (128,23,23).

The native correction changes 38 pixels at x2901/y2084..2121. Every
changed pixel improves source RGB distance; all 57 regional RGB and
171 threshold comparisons improve or tie. Both smaller full frames,
alpha, inner strip, joins, right side, text and exterior stay exact.
Six actual-Dashboard state/control pairs pass, including custom color,
partial opening, synthetic selection/held, fallback and a Kitsch negative
control. Astra independently verifies the measurements and frames.

Only the native primary frame changes. SVG/component already have a dim
outer shoulder and remain unchanged. The three sampled pixels nearest
the preserved joins, remaining shoulder error and faint striations stay
open. Three production sizes and packaged 4K match the reviewed candidate
exactly. Both Dashboard gates, 294 local/Nix Rust tests, 24 SVG structure
checks and the full 22-check/27-case repository run pass; 26 visual cases
are exact and the one-level Neo-kitsch bar residual is unchanged. See
[round forty-five](../reference-svg-round45.md).
