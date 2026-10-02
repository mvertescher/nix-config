# Reference SVG round forty-six — bounded Store lettering

Sol implementation lanes continue the selected Entropism M, ordinary
Kitsch compliance M, and Neomil margin A2 gap. Astra independently checks
source crops, measurements and native output before integration. CH is
the preceding completed repository checkpoint. CN completes integration
and verification of these three bounded corrections. The broader typography,
printing and native Entropism work remain open. Changes stay staged and
uncommitted; nothing is pushed or deployed.

## Entropism selected M

One outline follows the measured row-varying stems and shorter feet.
A separately named, otherwise identical Rajdhani font first reproduces
the original SVG exactly at three sizes. Changing the whole font also
changes the final M; a first-letter span disturbs later shaping. Those
transfer mechanisms are rejected. A contextual first-M variant preserves
shaping, but changes a few later-title antialias pixels. Splitting intact
runs at the empty M–A gap removes those exterior changes.

The production SVG uses the same measured M as an inline path, followed
by the unchanged full MAGNUM run clipped beyond card-local x29. It needs
no experimental font. An original-outline path control quantifies the
text-to-path raster difference: 288/93/85 changed pixels, all inside M,
at 3840×2160, 1600×900 and 1537×947. The actual correction changes
531/152/135 pixels, again entirely inside M, with alpha unchanged.
RGB absolute error in the fixed M region falls from 157880/26335/25696
to 86101/13453/14670. A, the final M, 650 and ordinary cards stay exact.
Two fractional pixels lose G<120 coverage while becoming closer to the
source in RGB; that soft-edge tradeoff remains explicit.

The selected component uses the identical path and suffix. Full/half
sheet renders change 152/47 pixels, only inside M. Interior title crops
match the parent exactly at both scales. An earlier crop included the
left border, where the half-size sheet has 24 unrelated background-edge
differences of at most three channel levels; excluding that border does
not change the artwork or renders.

The current packaged native title is a distinct, unstretched full text
run. Its first M is too far right and misses all 12 dark source pixels
in the fixed left-foot window. It already clears the SVG's unsupported
x1173 stem. Its M RGB error is 199136 and G<100 overlap/union is .220,
versus .610 for the corrected SVG. This SVG change is not a native fix.
Native transfer still needs an unchanged-run clip replay, an explicitly
mapped original-outline control, and selected-away recoloring checks.

Evidence: `/tmp/cp-eras-next/ci-entropism-m-font/`,
`cj-entropism-m-diagnosis/`, `ck-entropism-m-svg/`,
`cl-entropism-native-compare/`, `cm-entropism-m-patch/` and
`ck-root-review/` beneath the same scratch root.

## Kitsch ordinary compliance M

The CG unchanged-outline topology control is followed by an actual Swash
hint probe. The raised notch survives hinting; small sizes are unhinted
equivalents, and the 4K exterior anchor moves by 1/16 pixel. This probe
is geometry evidence, not source acceptance by itself.

Three fresh actual-Store baselines match BE in full RGBA. The single BS15
candidate changes 166/42/40 pixels at the three sizes, entirely inside
the three fixed ordinary-M windows. Alpha, the full A fringe and suffix,
and selected printing remain exact. Astra reproduces every regional RGB
gain and views all nine card/size comparisons. All nine whole-M and
whole-line RGB comparisons improve; the shallower notch follows the
source more closely.

Retain the small losses: two 4K card-three gap pixels worsen total RGB
distance by four levels, and one 1600 card-one gap pixel by three, without
threshold-mask changes. Card one's strict 1600 M F1 falls by .009702,
while its lower-threshold scores and RGB error improve. This is a bounded
M correction, not completion of compliance cap height or photographic
printing. The separately named production font is integrated, with a pinned input/output
hash, preserved license metadata and deterministic FontTools recipe.
All tables except names and checksum match the native-tested trial.
The actual Basic/Advanced shaping test and all 295 Rust tests pass.
All three production sizes exactly match the reviewed native candidate.
Six fractional state pairs pass: rest, ordinary hover, ordinary held,
alternate selection, custom roles and opening. Both rest wrappers match
the actual Store. Pair changes number 40/40/40/38/40/26, entirely inside
ordinary M windows; selected printing, suffix and alpha remain exact.
Every non-rest case visibly changes the screen. The fresh packaged 4K
frame also matches exactly.

Evidence: `/tmp/cp-eras-next/ci-kitsch-m-hints/`,
`cj-kitsch-m-native/`, `cj-kitsch-m-review/`,
`ck-kitsch-m-integration/` and `ck-root-review/`.

## Neomil margin A2 gap

A continuous ground-colored slit erases a photographed bright crossing
and is rejected. Two disconnected segments preserve it. The first
fractional SVG render was stretched by explicit renderer width/height
arguments; its score is invalid. Correct renders use a top-left uniform
SVG viewport, matching the original-photo inverse-affine source sampling.

The segmented SVG changes 12/5/5 pixels. All twelve 4K changes improve
RGB distance and preserve the bright crossing. At 1600, one pixel worsens
RGB distance by three levels despite net improvement; at the fractional
size, one R>180 mask pixel is lost while RGB distance improves. These
SVG limits remain separate from native results.

The actual-Store harness reproduces current packaged output exactly at
all three sizes before the candidate. Native changes 8/2/4 pixels; every
one improves source RGB distance, totaling 821/88/251 channel levels.
All changes stay in the fixed lower-gap region, alpha is unchanged, and
the bright crossing at native (128,1205) is exact. Fixed threshold scores
improve at 4K and tie at both smaller sizes. Astra independently checks
all changed pixels and source-owned bright controls and views the crops.
Six fractional state pairs pass: rest, ordinary hover, ordinary held,
fourth-card selection, custom colors and early opening. Both rest wrappers
exactly match actual Store captures. Every non-rest case visibly changes
the screen, while the pair delta remains four pixels inside A2. Custom
roles produce 257 cyan margin pixels and no bright red pixels.
All three production sizes and the fresh packaged 4K frame match the
reviewed native candidate exactly. The component correction changes five
full-size and two half-size pixels, confined to A2. Full-size translated
changed support matches the parent; four of five RGBA deltas are exact,
with one green-channel difference of one level at the fifth pixel.

Evidence: `/tmp/cp-eras-next/cj-neomil-margin/` and
`ck-root-review/neomil-native-review.json`. The initial native scoring
script used an incorrect region key; a separate root copy corrects only
that key, preserving the frozen captures and original script.


## CN integrated verification

All 295 local and Nix Rust tests pass (250 library, 45 bar), including
actual Basic/Advanced shaping of the named font. Both changed-SVG source
inventory gates and all three Store implementation gates pass. All 24 SVGs
parse with unique IDs and resolved local references.

The full repository check passes all 22 checks and all 27 visual cases on
the first attempt. Direct pixel comparison finds 26 exact cases and the
unchanged one-pixel, one-channel-level Neo-kitsch bar residual. A held-open
matrix log preserves the complete run across temporary-file deletion.
All 322 frozen tracked files match the tested Nix source; its size is
16 MB. All nine original source photos remain unchanged.
Only the Kitsch and Neomil Store goldens change, by 42 and two pixels.

The package is `/nix/store/j7j6whng1gjssjcrxk9gi0wmpx5bpk2s-cp-eras-ui-0.0.0`.
Fresh packaged 4K Store frames match production and the reviewed candidate
in full RGBA for both changed native eras. Its Store binary contains the
exact generated font bytes once; the pinned upstream font, installed
FreeFont licenses/credits and derivative notice also match their inputs.
Final prose updates preserve all tested runtime, artwork, build recipes
and goldens. Both repository heads and the existing staged lock stay fixed.
All 29 broader TODO boxes remain open.

Verification evidence: `/tmp/cp-eras-next/cn-validation-plan/` and
`/tmp/cp-eras-next/ck-root-review/`. Next prepared investigations are the
portable Kitsch SVG M and Entropism's unchanged-run/native-outline replay;
neither is included in this completed checkpoint.
