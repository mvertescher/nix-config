# Reference SVG corrections — round forty-one (BF)

BF calibrates the Neomil dashboard footer's missing top-edge coverage. BA's
full-opacity native band improved RGB error but created a false R>170 row at
1600px, so it stayed unapplied. A bounded interpolation study suggested an
opacity below that threshold; the actual renderer is tested independently
because its ink rebasing differs from RGB interpolation.

One native trial wraps the existing proposed 143×(1/2.4) rectangle at
(1210,865) in constant .80 opacity, preserving the foreground palette role.
The baseline matches the frozen BA dashboard at all three sizes. Across
3840×2160, 1600×900 and top-left 1537×947, all 39 fixed regional RGB comparisons
and all 117 red-mask F1 comparisons improve or tie at thresholds 150/170/190.
Whole-footer RGB MAE changes 10.773708→10.177267, 9.653327→8.744122 and
11.484323→11.147865. The 4K train's added row rises R48→194 versus source251;
its three F1 scores reach 1. At 1600 the same row rises R59→149 versus source
mean154.21, preserving F1 at 170/190. Its R>150 F1 remains .667, so this is a
bounded correction rather than an exact edge-filter reconstruction.

Native changes are 341/142/137 RGBA pixels, all inside the frozen top envelope,
with no alpha changes. Every changed 4K/1600 pixel improves RGB L1 error;
136 fractional pixels improve and one worsens: (1163,831), L1 39→43.
Frame sides, lower coverage, divider geometry, code, captions and blank
controls remain unchanged. Nine paired cases pass, including three rest
sizes, first/last selection, synthetic held feedback, custom palette, partial
opening and no-config fallback. The fallback images match the reference
palette rest images exactly, for both baseline and candidate. These are
headless drawing checks, not live pointer or desktop validation.

The dashboard SVG and component specimen now include the separately measured
full-alpha band. This is a backend-specific calibration: native .80 and SVG
1.0 do not imply equal compositing. Against fresh pre-integration SVG renders,
all 39 regional RGB and 117 threshold comparisons improve or tie. SVG changes
342/142/272 RGBA pixels at the three sizes, all within the top envelope, with
no alpha changes. Full/half component changes are 487/200 pixels: 142/72 in
the band and 345/128 in its caption; nothing changes elsewhere. Exact footer
softness, side phase, secondary striations and font shapes remain open.

Evidence: `/tmp/cp-eras-next/bf-neomil-top-opacity/` retains the prediction;
`bf-neomil-top-native/` pins the actual harness, runtime, font, binary and
captures; `bf-dashboard-review/` preserves the unchanged BA regions and
thresholds, full-frame baseline parity, all pixel losses and nine-pair result.


## Rejected Neo-kitsch metadata trial

At matched cap height, FreeSans Bold is a useful contour candidate, but an
equal-em comparison cannot identify the photographed font. A fixed scratch
fit trains its origin and tracking only on upper A line one's word starts;
the five other metadata lines are holdouts. A full-color SVG trial preserves
all six sentences, their fill and halo. Every whole-line threshold overlap
improves at three sizes, but actual R>190 false core in fixed source word
gaps rises 10→361, 0→43 and 0→29 pixels. Upper C pair RGB error worsens at
4K and 1537, and upper first-line cores begin one row too high. The candidate
is rejected; Neo-kitsch artwork and runtime remain unchanged.

Astra reproduces all pair/line RGB and threshold values, gap counts and full
RGBA locality from the frozen renders. The 1600 reference uses full-photo
Lanczos; an earlier Bicubic diagnostic is retained separately. Fractional
source sampling uses uniform top-left inverse-affine Bicubic at 1537×947.
The study motivates separating word spacing from letter spacing; it does
not establish the source font or an accepted native port. See the
[metadata record](neokitsch/text-fit.md#repeated-store-metadata-unresolved-font-and-spacing-fit).

## Next-lane login audit

BG finds fewer bright-threshold pixels in the two inactive login notices
than in the source. That alone does not identify a weight error: shape,
hinting, registration and ink all affect the mask. A single Regular→Medium
SVG trial improves bright overlap but worsens whole-line RGB error in two
fixed controls at every size and adds false ink to one O counter. It is
rejected before native work. Astra reproduces the SVG comparisons and
locality; frozen AJ native captures are supporting historical evidence at
4K/1600 only. Fractional native anchors are not registered to the uniform
SVG projection, so co-located fractional native scores are excluded. See
[the notice audit](neomil/login-primary.md#bg-inactive-notice-weight-audit).

## Integrated verification

All 293 local and Nix Rust tests, both dashboard fidelity gates, 24 SVG
ID/reference checks and all 22 repository checks pass. All 27 visual cases
pass on their first attempt: 26 are pixel-identical, and the Neo-kitsch bar
retains exactly its previously recorded one-pixel, one-channel-level
residual. Production dashboard frames at all three sizes plus the no-config
fallback match the reviewed native candidate exactly. A fresh packaged 4K
dashboard also matches exactly. Only Neomil and fallback dashboard goldens
change in BF, by 142 pixels each, with no alpha changes.

All 315 frozen tracked files match the tested 16 MB Nix source; nine
local original source images remain unchanged. The packaged FreeFont
COPYING, README and CREDITS still match the pinned upstream source byte for
byte. Final documentation changes preserve tested runtime, SVGs and goldens.
All 29 broader TODO boxes remain open. Changes stay staged and uncommitted;
nothing is pushed or deployed.

Integration and SVG component proof are in `/tmp/cp-eras-next/bf-integration/`;
production, package, complete repository and source-hash evidence is in
`bf-validation-plan/`. The independently rejected metadata trial and its
frozen RGB/gap review are in `bf-neokitsch-full/`.
