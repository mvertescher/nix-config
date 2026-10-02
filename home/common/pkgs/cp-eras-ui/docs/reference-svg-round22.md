# Reference SVG review — round twenty-two

AK starts from the verified AJ working tree. Sol lanes investigate Kitsch
front outlines, the Neo-kitsch login B, and Neomil's small margin O.
Astra reviews source fidelity, native rendering and integration. No item
closes on a scratch measurement alone; changes remain uncommitted.

## Kitsch front-outline review

The idle-only 1.4-design-pixel candidate retains selected/pressed outlines
at 1.8 and ghost outlines at .9. Coordinates and inks are unchanged.
The first native comparison exposed a stale baseline image: it differs
from a fresh production capture in 4,042,814 pixels, including background,
ghosts and tracked headings. The isolated preview with the trial disabled
is pixel-identical to fresh production at 3840×2160. Native measurements
must use that verified before image; the old native measurements do not
establish current-runtime fidelity. Original/source SVG measurements and
their documented losses remain separate evidence.

Against the corrected native baseline, 296 of 300 strip-level width/area
measurements improve. All fifty half-max widths and fifty half-max areas
improve; four other comparisons and thirteen complete-profile strips
worsen. The 12,142 changed pixels stay within 1.081 design pixels of idle
outlines, with selected EVENTS identical. Eight paired state captures
preserve selected/pressed geometry and confine changes to idle outlines.
Hover and the panel-only custom probe are visually inert controls, while
held, alternate selection and opening exercise distinct images. See
[the detailed evidence](kitsch/dashboard-material.md#ak-current-native-outline-review).

## Neo-kitsch B contour review

The source has a slimmer spine and larger counters than the font-based B.
The first path improves all 48 initial region/threshold comparisons, but
eleven-threshold review reveals top-edge and lower-counter losses. A
source-based intermediate lower counter improves 87 of 88 comparisons
against the current SVG and 86 against the current native image; those
cross-renderer comparisons are preliminary. Whole-glyph SVG RGB MAE
falls from 32.73 to 18.58. The final SVG retains one lower-counter loss
against the previous SVG; diagnostic regions used during revision are
not untouched holdouts.

Original broad frame bands include faint letter halo changes (3 left,
69 right pixels for the first path). Clean edge-only bands, the solid tab
and A are unchanged. The first path changes 196 gap halo pixels and
worsens gap RGB MAE from 3.612 to 4.677. These are rendered differences,
even though frame geometry is unchanged. The selected path's gap error
is 4.664.

Native review supports the bounded contour correction. The before preview
matches AJ exactly; the candidate improves 87 of 88 native glyph checks
and whole RGB MAE from 41.73 to 32.68. One upper-counter comparison at
threshold 100 loses .00292 IoU. Only 405 pixels change at 4K; A, both
frame bands, tab and gap are identical. Seven fractional states each
change the same 144 B pixels, preserving all feedback deltas. The 1600
capture changes 121 pixels only in B. The component excerpt shows A only
and remains unchanged. See [the badge record](neokitsch/login-branding.md#ak-b-contour).

## Neomil margin O investigation

Whole-ring shifts and uniform ink lightening are rejected. All eight
translations worsen the upper source fit; seven worsen the lower holdout.
The exception marginally improves lower RGB MAE from 35.526 to 35.203
while worsening upper error from 25.740 to 27.712. Uniform lighter ink
improves upper aggregate color but worsens the lower O and valid dark core.

An upper-only gradient improves upper-left and independent upper-right
RGB error. It also reduces upper dark-footprint F1 from .5915 to .5455
at red below 50. In the right holdout, 14 valid dark pixels worsen and
only five improve. Four lower pixels change, two better and two worse,
despite identical rounded RGB error. A2, the rest of the word and CJK
stay pixel-identical. Both gradient candidates are rejected. A local
aperture/contour study remains open; photo wear and antialiasing limit
what can be inferred from this tiny source region.

A subsequent short widening of the inner aperture improves all five
changed SVG pixels. The rebuilt native trial changes only three pixels
at x129/y1177–1179, all toward the source, with every neighboring pixel
identical. At 1600 it changes no pixels. The native before image matches
AE at both sizes. This corrects the local opening only: it has no
independently changed native holdout, and the broader O/plaque material
and A2 junction remain open. Five fractional states each change only one
O pixel and preserve the rest of the screen, including custom ink and
selection/held/opening feedback. See [the margin study](neomil/store-printing.md#ak-upper-o-aperture-study).

The initial shared-target Cargo trial reused the before artifact. Equal
binary hashes and `fresh=true` exposed it. Those captures were discarded;
cleaning only the crate package forced a new trial compile. Final
integration likewise rebuilds the crate before production comparison.

## Verification status

Studies and isolated captures are under `/tmp/cp-eras-next/ak/` and the
adjacent `ak-kitsch-front`, `ak-neokitsch-b` and `ak-neomil-margin`
directories. All three bounded corrections pass integrated verification;
no broader material task is marked complete.

Fresh production 4K captures are pixel-identical to all three reviewed
trials. Only the Kitsch dashboard and Neo-kitsch login goldens are
refreshed (2,957 and 121 pixels respectively). The Neomil store golden
is retained unchanged. Final matrix captures verify those 1600
results against the integrated Nix snapshot independently.


All 288 Rust tests, six fidelity gates and 22 repository checks pass.
Kitsch and Neo-kitsch source ink-placement scores are .74 and .73;
Neomil source shape area is 72%. Native shape areas are 100%, 87% and
89% respectively. Thresholds and scripts are unchanged. Neo-kitsch's
coarse source score was .74 in AJ; direct 1600 SVG comparison changes only
320 B-region pixels at x839–857/y466–486. Local contour measurements,
not that aggregate score, establish the correction.

All 27 visual cases pass first attempt at reported 100.000%. Direct
pixel comparison and immutable-source proof are recorded below. All 223
frozen file hashes match the tested Nix snapshot; three original images
are unchanged and all 24 SVGs parse with unique IDs. Final prose changes
leave tested runtime, reference traces, scripts, tests and goldens intact.
One Kitsch component legend changes ghost stroke 1.2 to the already drawn
.9; a structural SVG comparison verifies that only this text changes.
All changes remain staged and uncommitted.

Direct matrix comparison: 26/27 cases are pixel-identical.
The Neo-kitsch bar differs by one channel level at pixel (749,20).
Its AK and AJ renders are pixel-identical, confirming that this difference
predates the current changes. Both refreshed goldens and the unchanged
Neomil store case are pixel-identical to their matrix captures.
