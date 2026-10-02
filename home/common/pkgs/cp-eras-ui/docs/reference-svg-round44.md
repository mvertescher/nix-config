# Reference geometry and glyph transfer — round forty-four

This round reviews three bounded hypotheses against BT. CC integrates the
native Neo-kitsch selected third-bottom correction after source and state
review. Production/package parity, local/Nix tests, fidelity gates and full
repository checks pass. CC is the latest verified checkpoint. The 29 broader TODO boxes
remain open. Workers use GPT-6 Sol medium; Astra verifies and integrates.

## BV Neo-kitsch selected frame

The exact +1.2 design-y native trial passes three-size production-baseline
parity and changes only the selected third echo's lower stroke. Bottom RGB
improves, but the 1600 ridge overshoots source by one pixel, smaller-size
red area falls farther below source, and fractional turn RGB/F1 worsens.
Astra reproduces 78 regional and 234 threshold comparisons and rejects
this exact offset. Other five ridges, ordinary controls, alpha and stroke
exterior stay unchanged. This is a geometry/coverage result, not evidence
for changing opacity. See [frame measurements](neokitsch/store-frame.md#bv-selected-third-bottom-native-trial).

## BX Kitsch ordinary M transfer

The full original text run remains intact. A central aperture replaces
only font x230..640/y0..729 with the corresponding vector strip. An
original-M replay control is rendered before the frozen contour candidate.
Both preserve the outer M, the A/suffix including its partially covered
left boundary pixel, selected text, alpha and all pixels outside the
ordinary M windows at 3840×2160, 1600×900 and 1537×947. Fractional SVG
uses an explicit top-left uniform viewport. Astra independently reproduces
162 regional/whole-line RGB sets and 486 threshold sets and inspects the
three-card, three-size source/baseline/control/candidate crops.

The original-M replay itself changes 107/42/37 pixels and loses RGB L1
394/263/78 inside the aperture. It establishes locality, not raster identity.
The proposed contour changes 190/54/50 pixels and gains aggregate changed-
pixel RGB L1 14394/1522/1984. It visibly opens the source's high M notch,
but 1600 card 1 loses RGB L1 70 in the M and whole line. Fixed M/A-gap
and threshold losses remain; all 104 overlapping control/candidate loss
records are retained. The fixed small-size A box can include M pixels;
its loss must not be described as a changed A when full suffix parity passes.

Transfer feasibility is verified only for SVG. The contour remains
experimental pending a native original-M replay and independent source,
state and locality review. No source font identity is claimed. Evidence:
`/tmp/cp-eras-next/bx-kitsch-m-aperture/`, including `root-review.json`
and `root-montage.png`.

BY's native transfer probe uses five clipped copies of the intact tracked
line, with the original text also filling the central aperture. All three
actual-Store baselines match production. Its 1600 no-op is byte-identical,
but 4K loses four M bottom-row pixels on card 1, x1176..1179 at y1361.
Astra verifies those exact RGBA differences and unchanged cards 3/4,
suffix, alpha and exterior. The run stops at the failed replay gate;
fractional no-op and new-contour native captures are not made. Feedback
fill ordering is a code-level concern, not a tested disappearance in rest.
The clipping method is rejected under its exact-replay requirement. See
[the native transfer record](kitsch/typography-fit.md#bxby-ordinary-m-transfer).

## BW Kitsch fan edge diagnosis

Fresh current native frames match the verified 1600 golden and establish
4K/fractional baselines. Four source/SVG/native strips separate the nearest
right PRODUCTS ghost's overlap edge, exposed opposite edge, opaque front
edge and other fan. Astra independently reproduces all twelve complete RGB
profiles and 36 flank-crossing sets and inspects the strips.

The overlap edge's source/native half-max centers differ by only .131
physical pixel. Native has more outer positive area than source, so added
blur has no support. The opposite exposed edge is .693 physical pixel too
far right at half-max, while source background levels are also brighter.
The front-card control shifts in the opposite direction; a fan-wide
translation is unsupported. A single .29-design-pixel widening of only
the nearest ghost's left side is a bounded new experiment. The former
opposite-side holdout becomes its fitting region; other depths, corners,
overlap and independent fan/front controls must remain explicit.

No opacity, blur or shared card change is justified by this audit. Evidence:
`/tmp/cp-eras-next/bw-kitsch-fan-audit/root-review.json` and fresh frames in
`/tmp/cp-eras-next/bw-kitsch-fan-native/`.

## BY fan SVG and native trial

The one .29-design-pixel widening preserves the right edge and changes only
the nearest right PRODUCTS ghost. At 4K it improves exposed-edge RGB and
position in both renderers, and the upper-left corner's mean RGB improves
at all three sizes. However, the small-size straight-edge errors worsen.
Native target RGB MAE changes 23.312→20.588 at 4K, 19.396→20.559 at 1600,
and 18.615→19.704 at 1537. The native 1600 peak error grows 2.550→33.472
levels; 4K half-height width and core-area errors also grow. SVG has the
same small-size RGB failure. Astra independently reproduces 45 profile
sets and 18 box scores per renderer and inspects both three-size montages.
Reject this exact geometry in both renderers. No state or production port
follows, and the corner gains do not close K2.

Native baselines match the current production frames exactly. The native
trial changes 203/46/44 pixels, SVG 219/51/48; all stay within the fixed
left-edge/corner region with unchanged alpha. The opposite overlap edge,
other depth, front edge, other fan and selected EVENTS stay exact. Native
changed-pixel losses number 74/24/20. These local errors remain visible
despite improved total changed-pixel RGB.

The review also corrects the initial BW fractional SVG viewport. Rendering
at 1537×865 stretched its y scale and differed in 162294 pixels from the
top content of an explicit 1537×947 uniform top-left viewport. The former
fractional backend scores are superseded. Corrected primary comparisons
use that explicit SVG viewport and full-size inverse-affine Bicubic source,
with BW's original fixed row corridors. Evidence:
`/tmp/cp-eras-next/by-kitsch-fan-svg/` and
`/tmp/cp-eras-next/by-kitsch-fan-native-review/root-review.json`.

## BY frame mechanism and BZ native trial

Astra reproduces 27 continuous ridge profiles and 54 along-bottom profiles
from the original photo and existing BV captures. The source's straight
third bottom is about two 4K pixels lower than baseline, but the right bend
is already near source. Moving both together explains BV's turn failure.
At 1600 the BV argmax overshoot is larger than its continuous-centroid
overshoot (.137 pixel); the independently measured area and turn losses
still reject it.

The BZ trial preserves the original quadratic bend and adds a short cubic
transition to a flat bottom .9 design pixel lower. Actual-Store baselines
again match production at all three sizes. It preserves the side, turn,
ordinary cards, other five ridges, alpha and frozen stroke envelope. The
new third peak matches source at 1600 and fractional size, and remains
one pixel high at 4K. Third-bottom RGB RMS improves 47.240→16.778,
38.342→12.663 and 48.959→14.992. The only regional threshold loss is the
tiny 4K lower-frame R120 F1 .699012→.698971.

The fractional coverage guard nevertheless fails: three-sample red area
rises from 103 to 139 against source 108.722, and half-prominence width
from 1 to 1.582 against source 1.030. Peak ink weakens 116→103 against
source 124.111. The 4K area improves 304→278 against source 281.548;
1600 stays 144 against source 155.6. The exact .9 contour is rejected.
Its bend correction remains useful evidence, not production artwork.
Astra reproduces 78 regional/234 threshold sets, nine third-ridge area
profiles and 54 station profiles and inspects all three source crops.
Evidence: `/tmp/cp-eras-next/bz-neokitsch-frame-review/`, including
`root-review.json` and `root-area-stations.json`.

## CA–CB frame coverage and native candidate

CA explains the baseline, BV and BZ rows using four-sample coverage and
linear-light color blending. The observed native rows agree with the
standard four-sample pattern; the device's standardSampleLocations flag
was not queried. The common coverage interval is [.99375, 1.025) design
pixels. A single +1.0 flat-tail trial retains the original bend and avoids
BZ's fractional split across two rows. This is a coverage prediction,
not a search over offsets.

CB's native third peaks match source at 1687/702/675. Source/baseline/trial
red areas are 281.548/304/283, 155.6/144/144 and 108.722/103/103.
Half-prominence widths improve at 4K and remain unchanged at both smaller
sizes. Third-bottom RGB RMS improves 47.240→19.406, 38.342→12.663 and
48.959→13.337. The tiny 4K lower-frame R120 F1 loss .699012→.698971
remains, as do the source's brighter core and softer material. All ten
rest guards pass at each size: exact production baseline, stroke locality,
alpha, side/turn, ordinary controls, six ridges, other-five-ridge parity,
source third-peak position and nonworsening third-area error.

Astra reproduces 78 regional and 234 threshold sets, nine third-ridge
areas and 54 station profiles, and inspects the source/native montage.
Only 3420/479/457 pixels change in the rest candidates.
Evidence: `/tmp/cp-eras-next/ca-neokitsch-frame-coverage/` and
`/tmp/cp-eras-next/cb-neokitsch-frame-review/`.

CC passes ten paired actual-Store fixtures at 1537×947: rest, first/last
category and card selection, ordinary/selected hover and held material,
custom palette, partial opening and an unchanged Kitsch control. All
changes remain within the selected card's lower stroke, with exact alpha.
Rest, hover, held, palette and opening change 457 pixels; first/last
selection change 458 each; Kitsch changes none. The partial opening shows
the tail and differs from rest elsewhere, so locality is not a hidden-tail
pass. The selected pressed material equals rest by design. Material
fixtures use the actual state tables without replaying pointer events.
CC integrates only the native selected third path. Three production sizes
and a fresh packaged 4K frame match the reviewed candidate exactly. All
294 local/Nix Rust tests, both Store fidelity gates, 24 SVG structural
checks and all 22 repository checks pass. All 27 visual cases pass on
the first attempt: 26 exact, plus the unchanged one-level Neo-kitsch bar
pixel. Only the Store golden changes, by 479 pixels with exact alpha. All
318 frozen files match the tested 16 MB source; nine original images are
unchanged. Final prose preserves tested artwork. State evidence:
`/tmp/cp-eras-next/cc-neokitsch-frame-states/root-review.json`.

## CB separate SVG trial

The SVG backend has a different baseline phase. Its one separately
calibrated +.8 tail preserves the bend and aligns the three third peaks,
but fails retained material controls: 1600 area falls 91.5→90 against
source 155.6, and fractional half-prominence width changes .953→1.167
against source 1.030. Small regional threshold losses remain. Reject this
exact SVG candidate; the production trace and component stay unchanged.
The native geometry correction can proceed independently, with SVG phase
and halo work still open under NK-14.

The original tight SVG locality guard still reports 7031/684/635 exterior
pixels. Before rendering CB, a separate guard was derived from the
changed core plus four standard deviations of the existing anisotropic
halo and one physical pixel of rounding allowance. Full SVG changes all
fit that declared filter envelope; alpha stays exact. The diagnostic
without the halo fits the old tight guard. Core-only scores do not replace
full-SVG acceptance. Full changes number 21539/3300/2503 versus core-only
3424/483/462. This explains filter propagation without erasing BU's earlier
guard failures. Evidence: `/tmp/cp-eras-next/cb-neokitsch-frame-svg/`.

## CA Kitsch first-glyph prototype

A scratch additive `TrackedGlyph` primitive preserves the full text run's
measured advances and paints an optional first-glyph outline directly in
the parent frame. `None` repeats the existing tracked text calls; the
selected line stays unchanged. Sol's source review confirms that layout
property. CC compiles the isolated library against the exact BT direct
dependencies, then verifies full-RGBA `None` parity at all three sizes.

The original-M path replay changes 250/83/78 pixels with unchanged alpha.
It fails locality: 12 pixels at 4K and five at 1600 fall outside the fixed
M windows. All twelve 4K exterior pixels belong to card 4's right stem at
x3487, y1351..1362. The 1600 exterior pixels occupy x1453, y563..567.
The full suffix boundary guard also changes five pixels each on 1600 cards
1/4 and fractional card 3. These are failed composite-pixel guards; an
unchanged measured suffix origin does not make them passes. The suffix
interior and boundary need separate diagnosis before attributing them to
a text-layout shift. Astra inspects all nine source/replay crops and
reproduces the exterior coordinates. The proposed contour is not rendered.
Reject this exact full-glyph transfer; no renderer change is integrated.
Evidence: `/tmp/cp-eras-next/ca-kitsch-m-prefix-prototype/` and
`/tmp/cp-eras-next/cc-m-prefix-review/native/`, with independent locality
results in `/tmp/cp-eras-next/cc-kitsch-m-review/original-transfer.json`.

CD localizes every failed boundary pixel to the M's right-edge column;
all later suffix columns remain exact. At small sizes that output column
contains both the M's right edge and the A's farther-right starting edge.
The embedded M has 138 bytes of TrueType instructions, and the existing
cached text route enables hinting. The canvas path bypasses that font
rasterizer. Astra confirms the instruction count and code path; this
supports a raster/phase explanation without assigning every difference
to hinting alone. The failed composite-pixel guards remain failures.

The next scratch lane tests a separately named, otherwise identical font
through the existing text route, keeping glyphs, advances, hint programs,
weight 600 and baseline handling intact. Original-clone full-RGBA parity
must pass before any outline edit. The diagnostic alone does not establish
acceptance of a new contour or production font. Evidence:
`/tmp/cp-eras-next/cd-kitsch-m-replay-diagnosis/`; preparation lane:
`/tmp/cp-eras-next/cd-kitsch-font-replay/`.
