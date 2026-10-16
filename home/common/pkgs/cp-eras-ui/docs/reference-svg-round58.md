# Reference SVG round 58 — source sampling and remaining gap ownership

EE adds a shared source-image sampler after two fractional studies used
incorrect viewport geometry. It also updates the Entropism gap count and
separates the Neomil plaque's SVG compositing losses from native behavior.
No font, source artwork, SVG, runtime primitive or golden changes.

## Shared source sampler

`scripts/sample_source.py` fits the original RGB image with the same
uniform, top-left scale as the Scene. A floating source box preserves
fractional content bounds; the resampled content tile is placed on an
RGBA viewport. Unknown padding is transparent, and positive partial
coverage never rounds to fully opaque or fully transparent. Only the
content tile is filtered, avoiding a large source-space allocation for
a narrow, tall viewport. Input/output hashes and both sampling boxes are
available as JSON. See [the pipeline instructions](PIPELINE.md#sampling-originals-at-a-viewport-size)
for filter-edge limitations and CLI/API use.

Nine regression tests cover canonical RGB identity in both scale
directions, fixed-width coordinate invariance, rejection of rounded
content height, height-limited fitting, partial/unknown alpha, extreme
aspect ratios, metadata and source preservation. The `sourceSampling`
Nix passthru case runs them in the existing matrix.

Astra additionally checks all sixteen 3840×2160 originals: every
1600×900 RGB sample is exactly the historical whole-image Lanczos result.
The Neomil Store sample at widths 1537 and viewport heights 865, 947 and
1000 has identical content pixels. Its true bottom is y864.5625; row864
has alpha143 and row865 onward is unknown. Compared with ED's larger
floating-box filter canvas, the new tile yields 180 one-level RGB pixel
differences above row860, from floating-box filtering precision. The
reviewed plaque crop is exact. Thus canonical identity does not imply
all historical fractional samples are byte-identical. Historical kernels
and rejected controls remain recorded; the structural gates are unchanged.

## Entropism Store title gaps

The source is the misnamed `images/entropism-dashboard.png`. Direct SVG
renders at 4K and 1600 are independently reproduced with the pinned
font configuration. AH's manually observed source glyph boundaries are
reused, with no new fit to current rendered edges.

All 96 frozen gap/threshold cases are recounted. The accepted selected-M
contour improves three cases and leaves 93 unchanged. In the first M–A
gap, AH had 36 dark pixels at each threshold 80/100/120; current has zero,
against source counts 0/0/2. This returns to the pre-AH error, retaining
the two-pixel source miss at threshold120. **Twelve of AH's fifteen
historical regression cases remain**, rather than fifteen current losses.
The title suffix is exact against AH on all four cards.

Remaining gaps cluster near G–N, N–U, U–M and 5–0. Ordinary glyph ink
centers differ from source in both directions: for example, U is left by
2.69/.78/3.08 physical pixels across the three copies, while final M
differs by +.60/−.06/+1.98. These thresholded photographic measurements
do not support one global tracking or translation change. Exact source
face, contours and photographic edge reconstruction remain unresolved;
native raster coverage is a distinct consideration. No candidate is made.

The current 1600 native golden matches the actual CQ production capture
as both PNG bytes and decoded RGBA. A similarly named earlier package
capture differs by 145 selected-M pixels and is excluded. No fresh native
capture is claimed. Future contour work still requires complete-run font
replay and adjacent-glyph/exterior controls from the completed font audit.

## Neomil plaque junction

This study retains ED's rejected hatch candidate and frozen source fit.
Actual A2, O-ring and plaque-clip alpha masks separate ownership. The
fixed-frequency source-minus-Login fit explains .677/.666 of detrended
red variance above and beside O, but only .133 in the lower window and
approximately zero at the leading junction. The lower masks have only
81/48 pixels and use SVG exclusions against differing source geometry;
weak fits do not establish where the photographed grating stops.

The SVG paints A2 `#df3035`, then the .67-opacity hatch `#fb3535`, then
the dark ring. Different colors let the hatch brighten opaque SVG A2.
Native A2 and hatch both use `Ink::Fg`, so this particular SVG interaction
does not establish native brightening. Root verifies the code and order.

The rejected candidate loses eight dark true matches in the fixed A2
box. None is inside opaque A2; only one touches its antialiased edge.
Four lie on the clip edge and four within the clip. At (128,1193), source
red34 and current23 become candidate166: the upper phase predicts a bright
ridge where the source is dark. A source-supported lower boundary,
primary contour or material layer is needed before transferring that
grating. An arbitrary cutoff or image-derived knockout is not established.

The initial diagnostic used the wrong frozen-phase sign; the corrected
projection is `cos(t − atan2(sin_coefficient, cos_coefficient))`. Corrected
correlations are .822/.815 above/beside O, .364 below and −.009 at the
leading junction. Free-phase fits and all ownership/error counts are
unchanged. No new SVG candidate or native port is made.

## Verification and state

Astra reviews code and source crops, reproduces both studies, independently
recounts all 96 title cases and all eight plaque dark-match losses, and
checks the accepted native baseline. Nine sampler tests pass locally and
in Nix. All 22 repository checks pass, including the now 28-case matrix:
27 visual cases plus the sampler. Existing visual derivations are reused;
the application remains the exact DZ package
`/nix/store/lh492hk4d383y80xif1034z4hdnyhnl9-cp-eras-ui-0.0.0`.
This adds no new draw-time measurement or live desktop verification.

Scratch evidence is under `/tmp/cp-eras-next/ee-root-review/`, with
`studies-verification.json`, `sampler-verification.json` and `full-check.log`;
worker scripts live in the sibling `ee-entropism-title-gaps`,
`ee-plaque-junction` and `ee-source-sampler` directories. All sixteen
originals and previously tested runtime/artwork inputs are unchanged.
The bounded sampling-tool task closes; all 29 broader TODO scopes remain
open. Changes stay staged and uncommitted.
