# Reference SVG round seventy-three — Neomil fourth-card cut

The ordinary fourth card stopped about three physical pixels early at 4K.
The correction extends its primary face, open top/bottom caps, content clip,
reference material masks and persistent drawing/hit viewport together from
design x1557 to x1558.3. The newly visible sliver is clickable under the
existing viewport convention; selection and feedback behavior are retained.

## Evidence and bounded acceptance

HG's original-minus-validated-Login top band first exposed the discrepancy.
HI independently checks the raw original top and bottom against the current
SVG and parity-verified package. Five strong source rows end near physical
x3740.0; the previous SVG/native endpoints lie near x3736.9. Weak interior
ground boundaries corroborate the direction, while content-contaminated
rows remain inconclusive. No ground donor is assumed inside/below the card.

HK freezes one candidate, rounded to the existing 0.1-design-unit convention,
before three-size rendering. Only the face width, cap endpoints and content
clip change. Unchanged no-op and current baseline renders agree exactly.
All fixed changed SVG source regions improve in L1 and RMS, including the
complete card, both endpoints and right interior strip. Other cards and
far exterior remain exact. At 4K the wider SVG clip also changes 240 interior
text rasterization pixels, by at most eight channel levels; these are kept
in the comparison. No font, glyph, text metric or ink is changed.

HL ports the same cut to Iced and its reference materials. Native changes
are entirely confined to the fourth right edge: 4,297 / 917 / 842 pixels at
3840×2160 / 1600×900 / 1537×947. Native interior text, other cards and alpha
remain exact. Whole-card source RGB L1 improves by 64,093 / 11,573 / 10,497.
Top endpoints and right-strip controls improve at every size. The fractional
bottom endpoint retains one explicit local loss: L1 3665→3720 (+1.50%),
RMS 22.2009→22.4794 (+1.25%), in a 121-pixel region with 12 changed pixels.
The same bottom endpoint improves at 4K and 1600. This is accepted as a
coherent primary-cut correction with a bounded residual; bottom coverage
remains open. No cap, width or phase sweep was used to remove the loss.

Native antialias coverage may extend within one physical pixel of the new
cut. No changed pixel lies beyond that bound. Fine source printing extends
farther than the primary and remains a separate echo task.

## Runtime and verification

The change reuses existing paths, masks and viewport primitives. It adds no
draw layers, effect passes, cache structures or per-frame font work. No new
timing improvement is claimed. Original images, bundled fonts, accepted
glyph outlines and text caches remain unchanged.

Fifteen Store interaction tests and the selected/ordinary cut-hit test pass;
inside/outside points straddle the new boundary across four selections and
five scales. The open-border coverage test checks ordinary, hover and held
drawings without adding a right-hand border. HM verifies seven paired
reference states, including selected fourth, hover, held, inferred ordinary
second and opening; three custom pairs preserve semantic palette routing.
A disabled-reference replay is exact. Changed pixels remain confined to
the right cut; interiors, neighbors, selected contents and alpha are exact.

Production SVG renders match HK at all three sizes. The component's two
visible cut-coordinate labels are synchronized and reviewed. Both Store
shape-inventory gates pass; source fidelity rests on the direct comparisons
above as well as these coarse gates.

The first full check found a stale test expectation: the center of the
newly 133.3-wide fourth navigation region is x1491.65 rather than x1491.
The assertion was corrected without changing runtime geometry. The complete
rerun passes all 22 repository checks, 305 packaged Rust tests (260 library
plus 45 additional) and all 28 matrix cases on their first attempt in that
rerun. Of 27 visual outputs, 26 match their goldens exactly; the previously
known Neo-kitsch bar one-pixel/one-level difference remains within its gate.
The remaining matrix case is the source sampler.

Fresh captures from the actual new package match HL byte-for-byte and
RGBA-for-RGBA at all three sizes. Its Store implementation gate also passes.
The final prose updates do not alter the tested source, SVG or golden bytes.
Work remains staged and uncommitted.

## Remaining work

This completes the bounded primary-cut correction, while broader frame
coverage and printing TODOs remain open. The separate
[echo study](neomil/store-frame-echo.md) validates local top ground, constrains
darker ink and improves its waveform with a previously measured centroid.
With the corrected primary endpoint, unchanged affine copies predict the
near/far top endpoints within 0.054/0.838 physical pixels. Complete-contour
SVG trials improve whole frames but retain smaller-size edge losses; neither
the direct nor composed-x variant is accepted. Local loss diagnosis and any
native transfer remain future work. No echo rendering cost is added here.

Scratch evidence: `/tmp/cp-eras-next/hi-fourth-primary-cut/`,
`hk-fourth-cut-svg/`, `hl-fourth-cut-native/`, `hm-fourth-cut-controls/`,
`hn-fourth-cut-review/`, and `ho-fourth-cut-integration/`.
