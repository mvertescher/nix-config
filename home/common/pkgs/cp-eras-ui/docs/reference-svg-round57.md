# Reference SVG round fifty-seven — plaque grating and warning feet

ED identifies a source-supported Neomil plaque hatch pitch and phase.
A complete SVG trial improves broad plaque error but loses leading-edge
and A2 detail, so it remains unapplied. A Kitsch warning-foot trial also
fails the selected-card controls. The Neo-kitsch frame revisit supplies
no new accepted contour. Production artwork, fonts and runtime remain
at DZ; all 29 broader scopes stay open.

## Neomil margin plaque grating

Six clear rectangles beside, above and below the plaque are RGB-exact
between original Store and Login images: 4,600 pixels in total. The left
side has separate Store printing and is excluded. This supports a local
blank-ground estimate, not direct observation beneath the occupied plaque.

The upper plaque has a coherent diagonal grating. A source-only fit uses
signed Store−Login red at original pixel centers in x[115,134),
y[1153,1166). A linear ground term plus a two-dimensional sinusoid gives
frequencies .2365/.3235 cycles per physical x/y pixel, corresponding to
periods 4.228/3.091 pixels. The grating explains 94.8% of residual variance
beyond the linear ground. Fixed-frequency phase estimates across top,
bottom, left and right portions differ by at most .019 radian. These
partly overlapping checks support local coherence; they are not independent
candidate acceptance data.

Under the existing margin rotation, the source periods imply local
pitches 1.288/1.762 design units rather than the current equal 1.5/1.5.
An earlier exploratory anisotropic transform also scales stroke widths
and inspects its purported holdout while selecting phase. It is rejected;
those inspected regions cannot subsequently be called pristine holdouts.

A separate frozen trial derives phase from the source alone and bakes
the resulting centerlines into 35 segments spanning 22 ridges. It retains
the original .45/.60 stroke-width partitions at local x89/99, opacity .67,
clip polygon, O ring, A2 paths, colors and draw order. Identity replay is
byte- and render-exact at 4K/direct 1600. The generated endpoints agree
with the fitted ridge equation to within 0.0000004 cycle after rounding.

The full-plaque RGB MAE improves 21.498→15.183 at 4K and 12.638→11.218
at direct 1600. However, the leading-edge control worsens
23.077→24.228 and 15.981→17.667. At 4K, A2 error rises 17.325→17.626
and its R<50 true matches fall 187→179; the lower hatch loses five valid
dark pixels. Alpha, M, CJK and neighboring artwork remain exact. The new
hatch changes composition beneath unchanged primary paths; preserving
path coordinates alone does not preserve their visible edge pixels.

The first fractional comparison stretched both the source and sampling
coordinates and is invalid. Correct explicit 1537×947 SVG viewports use
`xMinYMin meet`; the source uses whole-RGB Lanczos with a floating source
box so both axes scale by exactly 1537/3840. With the candidate unchanged,
plaque error improves 11.954→10.789 while the leading control worsens
16.533→17.486. The adjacent upper strip also loses slightly. Correcting
the comparator does not rescue the trial.

Astra reproduces the source fit and all six final renders, independently
checks donor pixels, endpoint-to-phase mapping, 99 regional RGB values and
A2 dark matches, and reviews the source/current/candidate montage.
Keep the measured grating evidence, but do not port this complete
trial or hide its leading/A2 losses with source-derived pixel cutouts.
The remaining task is to distinguish source phase and coverage near that
junction from the already established upper grating.

## Kitsch warning outer feet

The initial rounding trial inadvertently moves its long sides: its new
join coordinates are not on the accepted side equations. Astra catches
this before integration. A mechanical correction puts the joins at
x31.092222222222 and x41.907777777778 for local y87.62, retaining the
chosen foot curves, accepted bridge, tip, opening and ink.

The corrected trial changes 89/32/36 pixels at 4K, direct 1600 and
1537×947. None lies outside the whole triangle. There are 25/8/12 changes
above the narrow foot window, with maximum channel deltas 1/1/2 despite
collinear side geometry, consistent with path-subdivision coverage.
The original report mislabeled the narrow-foot mask as the whole mark;
the corrected masks and actual pixel counts supersede it.

The selected right foot still worsens: RGB MAE 31.856→33.634 at 4K and
26.422→27.800 at direct 1600. All three selected lower-edge dark-mask F1
controls fall at 4K. Ordinary-card gains do not establish a shared contour
that matches the selected source. The bridge and outside gaps stay exact.
The fractional diagnostic likewise loses locally, but uses pointwise
cubic source sampling; it is not a Lanczos-prefiltered source comparison.

The first scratch native-control table also used AY's rejected direct
port. Astra identifies the provenance error and checks the accepted
calibration instead: calibrated and production captures are byte-identical
at all three sizes, and all four warning crops equal the current 1600
golden. These are historical accepted captures, not newly rendered native
trials. Astra rerenders all six corrected SVG PNGs byte-exact, independently
checks 512 regional values at 4K/1600, verifies locality at all three sizes
and reviews the selected source crop. No warning change is accepted.

## Neo-kitsch selected frame

Fresh direct SVG renders match the reviewed CB baseline at all three
sizes. The third flat remains low in integrated contrast, with its 4K
centerline above the photographed ridge. The full SVG halo raises the
surrounding floor, so endpoint-based area and half-height width are not
isolated geometric measurements. A small-size endpoint can itself lie
on a shoulder; those area values cannot replace the earlier neighbor-gap
controls. No distinct centerline/width correction follows from this study.

The first rounded-height fractional source resize is corrected before
candidate fitting to the prior uniform inverse-affine Bicubic comparator.
Astra reproduces the profile file and adds a separate uniform Lanczos
sensitivity image using a floating source box, preserving identical x/y
scale. That kernel comparison is diagnostic and does not override the
rejected CB SVG trial's 1600 area and fractional-width controls. Preserve
the accepted CC native flat and unchanged SVG/component.

## Verification

Evidence is under `/tmp/cp-eras-next/ed-kitsch-warning-feet/`,
`ed-neomil-margin-material/`, `ed-neokitsch-svg-frame/` and
`ed-root-review/`. No native trial or runtime build runs in ED. Only
reviewed documentation changes; all prior staged work is preserved.
Runtime/assets/goldens and all sixteen originals retain their frozen
hashes. DZ remains the latest full verification: 303 packaged Rust tests,
22 repository checks and 27 first-attempt visual cases. These runtime
checks are not repeated for prose-only updates.
