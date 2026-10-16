# Store frame echoes — source geometry, ground and paint constraints

[Round seventy-four](../reference-svg-round74.md) integrates two faint ordinary
Store frame impressions missing from the earlier rendering. Fixed affine
contours, masked coverage and a coupled lower-stem/bar correction improve
source fit at all three sizes, with explicit local residuals below. Source,
state/custom, warm CPU drawing and actual-package verification are complete.
This is a bounded reconstruction, not a unique recovered source paint recipe.
The history below distinguishes rejected trials from the accepted model.

## Source observations

GZ freezes 41 complete RGB cross-sections and corner boxes before measuring.
Independent review checks all 1,985 source samples against the original
3840×2160 image and the same positions in the parity-verified package.
Source samples use the uniform top-left sampler with retained metadata.

All three ordinary cards have separate top ridges near physical row indices
354 and 362–363, with valleys before the primary at 370. The bottoms have
copies near 1472 and 1476–1477 after the primary at 1468. Selected-card top
controls also contain the copies. The current native background is flat at
these off-primary positions. Red chroma is only a diagnostic: negative
values over blue ground do not mean absent red printing.

The first card's left copies lie outside its primary; the third and fourth
lie inside. A common translated outline cannot explain this. Literal 1% and
2% expansion about the image center also fails: several side predictions
miss by 3–8 physical pixels.

## Fixed affine hypothesis

HA derives each axis independently from first/third-card source peaks:
two left anchors for horizontal scale/translation, and top/bottom anchors
for vertical scale/translation. Coordinates below are physical pixel
centers, with array index `i` centered at `i + 0.5`.

| Copy | Horizontal map | Vertical map |
| --- | --- | --- |
| Near | `x′ = 1.0057070387x − 11.0237793` | `y′ = 1.0100182149y − 10.7117486` |
| Far | `x′ = 1.0145846544x − 27.3941027` | `y′ = 1.0218579235y − 24.0983607` |

Previously inspected fourth-card and selected-top regions are controls
excluded from calibration, not unseen evidence. A separately frozen study
adds intermediate left rows, upper/lower right rows and right-corner boxes
without refitting. Independent review verifies all 26 profiles and the
unchanged coefficients. Thirty-eight separated peak comparisons differ by
at most 1.333 physical pixels. Left and right corner crops visually support
the predicted bends. First-card right and selected-card left copies overlap
their primaries, so those regions do not establish separate echo positions.

The initial fourth-left diagnostic mixed an observed-peak calibration with
a different primary-coordinate convention. Its retained output is followed
by a correction using the same primary-peak estimator throughout. The
coefficients stay fixed. Broad bottom search windows can select a neighboring
ridge, so bottom evidence uses the raw profiles. Four-pixel flank samples
can hit the primary beside the third-card near-left copy; its intervening
valley and adjacent local maximum provide the separation check instead.

These maps describe sampled peaks, not recovered vector axes or an original
physical lens transform. They justify testing transformed existing contours;
they do not determine stroke width, alpha, ink, softness or complete joins.

## Paint diagnostics

HC tests one smooth effective profile per copy, with moments derived only
from two first-card top columns. The near/far areas are 0.6073/0.3153 physical
pixels and the Gaussian sigmas are 1.0930/1.2947 pixels. Centers remain fixed;
pixel integration and all RGB metrics are independently recomputed. An
earlier five-bin lookup is preserved as diagnostic evidence and rejected as
a paint-law test because its truncation is arbitrary.

The assumption that echoes use the bright primary RGB(251,53,53) fails
independently of profile shape. At original x1240,y363, source green is 33
while the two background endpoints have green 47 and 45. Adding that
primary ink with nonnegative alpha cannot lower green below 45. A second
first-card column and a fourth-card control reproduce the contradiction.
Many aggregate RGB scores improve, but that does not establish the ink.

HD then uses repeated first/third-card backgrounds to solve one fixed
linear mixture for shared attenuation and premultiplied RGB. Gaussian
shapes and geometry stay unchanged. All three endpoint-ground variants
have full algebraic rank; independent QR and SVD solutions agree. The
two-sided estimate yields darker near/far RGB near (153,25,37)/(157,33,37),
but the high-endpoint alternative gives an impossible far RGB near
(315,4,-11). No coefficients are clipped or accepted. Overlap is small
enough that it does not explain the sensitivity.

Two excluded-from-calibration top columns lose against their interpolated
ground baseline, but their lower endpoints contain card printing. For
example, first-card x1160 has lower-endpoint median RGB(82,40,63) versus
upper (28,46,83). Those losses remain recorded and cannot independently
falsify every shared darker ink. The next constraint is independent local
background evidence; neither a color sweep nor mandatory external assets
follows from this diagnostic.

HE validates the original Login image as a same-coordinate local ground
donor above the first and fourth cards: 1,176 unique clear pixels match
Store exactly, including complete nine-pixel-wide controls. The third-card
donor fails because Login tabs occupy that region; it is excluded. There
is no registration or color fit. Equality outside printing supports an
inference about the obscured ground, not a supplied transparency layer or
permission to use this donor inside the cards and along other edges.

HF keeps both affine maps and the HC Gaussian shapes fixed and solves the
same eight-unknown material system against this donor. Four first/fourth
columns train the model; two separately declared columns test it. Independent
QR verification reproduces the fit and all four leave-one-column-out
diagnostics. The normalized condition is 8.21. Effective near/far RGB is
about (201,24,25)/(191,33,30), with maximum summed attenuation 0.316.
Both foregrounds are feasible and reasonably stable across the diagnostic
solves. These are coefficients of the fixed raster profiles, not recovered
source paint, physical opacity or stroke widths.

The two withheld columns improve from donor-only RGB L1 379/378 to
150.3/156.9, with RMS 4.99/5.27. Source/donor clear controls remain exact;
the model's extra-blank tails are negligible but nonzero. A repeated
waveform error remains: red is underpredicted by 13–23 levels at rows
361–362 and overpredicted by 19–20 at row 364. All RGB rows, valleys and
signed losses are retained. A different constant ink or gain cannot remove
this pattern while keeping the same Gaussian. No phase or width sweep was
used, and no complete contour candidate is accepted.

HG separately observes the cropped fourth card's top endpoints. Its bright
primary falls near physical x3740, about three pixels beyond the earlier
design cut at x1557 (physical x3736.8). Faint copies extend farther still.
HI checks independent bottom/content features and the packaged/SVG baseline,
confirming
the same discrepancy at the bright bottom edge and weaker interior ground
boundaries. Both the earlier SVG and parity-verified package stop near
physical x3736.9; five strong source top/bottom rows stop near x3740.0.
Round seventy-three completes one coherent cut correction to design x1558.3,
including face, content, caps, material masks and drawing/hit viewport.
SVG/native whole-card comparisons improve at all three sizes. Native retains
a fractional bottom-endpoint L1 +1.50% / RMS +1.25% local loss; state/custom,
full checks and actual-package parity pass. The unchanged HA copies now
predict near/far top endpoints within 0.054/0.838 physical pixels, without
establishing a unique echo cap or vector transform.

HJ separately substitutes the already recorded HC moment centroids for
the peak centers, retaining all widths, areas and HF material coefficients.
The near center changes by −0.5903 physical pixels and far by −0.0312.
Complete top-profile RMS improves from 4.91–5.31 to 2.36–2.56 across ten
columns, including four new donor-validated controls. Independent numerical
integration reproduces the calculation. Small losses remain in the far
profile, inner valley and Gaussian tails; they are retained, not masked.
This supports a concrete top-waveform hypothesis, not a full-contour phase
change or recovered physical paint.

## Complete contour trials

HP freezes one SVG transfer of the top hypothesis onto all three ordinary
contours. It retains HA x maps, moves the y maps to the already measured
top centroids, and uses one physical-pixel butt-capped stroke per copy.
Near/far effective opacity is 0.8945/0.4940, RGB (201,24,25)/(191,33,30),
and Gaussian sigma 0.4393/0.5259 design units. The blur variance plus the
one-pixel stroke variance reproduces HC's effective profile moments.
These are one concrete approximation, not recovered physical widths or ink.
Copies follow the existing face and precede the primary border, retaining
the shelf clip. Exact no-op, unchanged original XML and independent distance
controls pass at all three sizes; selected art and alpha remain exact.

All nine whole-frame comparisons improve, but the first lower stem loses
14.4%/13.2% RGB L1 at 1600/fractional and the third left edge loses about
2%/8%. The fourth far cap also overprints a fixed source/donor control.
Independent review reproduces 222 regional metric sets and inspects the
corners. HP is not accepted on its whole-frame gains alone.

HQ distinguishes HA's observed primary peaks from the accepted vector axes.
First/third left anchors imply the physical map
`V(x) = 0.9999746360x − 0.4732156445`. Composing this before unchanged HA
reduces maximum near/far side-peak error from 0.960/0.986 to 0.397/0.677
pixels across 23 separable rows. The training anchors are not independent
validation, and two third-card upper-right far controls worsen. The fourth
near endpoint also worsens from +0.054 to −0.518 pixel prediction error,
while its far endpoint improves from +0.838 to +0.262. Peak and cap
half-fall observations cannot be pooled as one geometric estimator.

HR tests only that composed x map, preserving every HP y coordinate, paint,
blur and ordering choice. Independent verification reproduces 333 metric
sets, six path/filter-only edits, three exact parent replays and raw cap
RGB. All whole frames still improve over production, but relative to HP
the first 4K frame loses 46,283 L1, and all three 1600 frames lose slightly.
The first lower-stem losses versus production persist at 1600/fractional
(+3,405/+1,609 L1), as do third-left RMS losses. Fixed fourth-cap incremental
L1 changes from 323 to 337 near and 268 to 238 far. HR is also unaccepted;
coordinate conversion does not explain away the repeated local losses.
All source controls in HQ/HR were previously seen and are labeled accordingly.

HS then freezes straight rows at design y450/500/550 and separates the
exact squared-error change into `2(B−S)A + A²`, where B is production,
S source and A the added echo increment. Independent review reproduces
432 pixel positions across four images and all side-group sums. At first-card
y500 and 1600, the favorable cross term −1974 is outweighed by +2689 from
the increment, yielding +715 squared error. At 4K the primary's right fringe
is already too bright: source red 68 versus production 232 at x710.62 design.
The added left-side copy can fill a deficit without removing that surplus.
Third-left profiles likewise show both missing ink and local overshoot.
All sampled additions fall at/inside the face; no Login donor is assumed.
This identifies combined coverage and scale sensitivity, not a new width,
ink or unique layer decomposition.

HV tests the unchanged HP echoes together with GV's previously frozen
thinner lower-primary segment. Literal production/HP replays are exact;
independent review checks all 504 regional metric sets and the two primary
path replacements. The combination repairs the first lower-stem losses,
but third-card bar-side L1 at 1600 rises 12,029→20,508 (+70.5%). It remains
unaccepted. The unchanged bar polygon does not preserve the combined edge.

HW tests one geometric union: extend the accepted bar's top/bottom diagonal
supporting lines to the old primary's inner edge, keeping the thinner
continuous primary and HP paint. For the first card this yields x709.6,
y271.9217391/413.9739130; the third is translated +657.1. There is no fitted
y cutoff or new taper. Exact rational derivation, two bar attributes, three
parent replays, 396 regional and 144 raw-row metric sets are independently
verified. Third bar-side 1600 L1 improves to 10,641; every changed fan/junction
regional box improves against production. Relative to the unaccepted HV
parent, first-card 4K fan L1 worsens 24,318 and a slight tip shoulder appears.
Three smaller-size first-card raw rows also lose 9–16 L1 against production.
These local tradeoffs and inherited third-left/fourth-left echo residuals
remain explicit. The geometry is a viable bounded hypothesis, pending paint
and native review; it is not recovered source layer construction.

## Exterior ground and actual filter response

HY finds a different local donor for the first-left exterior: Dashboard
matches Store exactly at all 513 declared blank pixels near original rows
920/960/1000. Login is visibly occupied by a red panel there and is excluded.
The donor is supported only in that upper exterior strip; equality beside
ink does not prove the obscured ground or validate the card interior.

HZ independently replays those blanks and 81 complete RGB profile positions.
Source-minus-donor far/near red variances are about 2.09–2.14/1.41–1.47 px²,
whereas literal HP's windowed raster variances are about 0.96/0.69 px².
The source centroids are around x1043.16/1050.73 versus HP x1044/1051.
These normalized finite-window moments retain overlap/truncation caveats;
they identify neither physical widths nor source anisotropy. The actual
SVG output must be distinguished from the analytic Gaussian moments used
to specify its filter.

Row-dependent color is also unresolved. At the near peak, observed RGB
increments are (72,9,9), (39,19,18), (68,5,5). One scalar applied to HP's
fixed increment leaves substantial green/blue error at the middle row.
Conditional on the inferred donor, the first two rows' single-foreground
green rays intersect at foreground red about 23, below the required minimum
91; their minimum disagreement over physically allowed red is 24.56 levels.
Independent review reproduces all twelve ray comparisons and fixed-HF-ink
diagnostics. This is a failure of that literal mixture assumption, not a
unique new paint recipe. The discrepancy is exterior to the primary, so
adjusting only the primary contour cannot resolve it.

IA isolates the current filter with transparent white strokes at five fixed
phases/orientations and all three sizes. Independent numerical integration
and 720 actual alpha samples confirm a transfer discrepancy: the 4K far
variance is about 1.02 versus the analytic 1.76 px², and seven of eight
smaller-size vertical filtered profiles equal their unfiltered controls.
The eighth differs by one alpha count. This concerns the measured isolated
operator, not a proven librsvg defect or source anisotropy.

IB constructs a deterministic profile from nested strokes. The widths follow
the already specified Gaussian profile, and conditional opacities accumulate
39 far or 84 near coverage levels on transparency. There is no source fit.
All fifteen phase/size L1 and RMS comparisons improve against the analytic
profile; independent review verifies all 330 levels and thirty profiles.
Rasterization still loses area and exceeds the nominal one-alpha-count
continuous bound, especially at fractional scale. Those residuals remain.

IC transfers exactly those levels onto HW's complete contours: 369 colored
paths, with fixed Gaussian-CDF alpha masks on the two finite fourth-card
ends. It fails on opaque artwork. At original first-left y920, far red is
source/production/HW/IC **54/19/50/19**; near red is **91/19/87/39**.
Whole fourth-card L1 worsens against production by 13,183/10,234/3,013
at 4K/1600/fractional, and the fixed far top-cap band loses at every size.
Independent review reproduces 450 regional and 144 junction-row metric sets,
all raw donor/cap RGB, six reversible substitutions and three exact parent
replays. Selected art, content and alpha are unchanged; changes remain in
the echo corridor. Added IC tail/strip boxes were declared after rendering
and before scoring; inherited source controls predate this trial.

IC is rejected. Its loss is consistent with repeated small colored
source-over operations being quantized against opaque ground; transparent
alpha tests alone did not establish color transfer.

ID isolates that transfer using seven fixed opaque backings, all five probe
axes and three sizes. Accumulating the unchanged white coverage in an alpha
mask and applying the fixed color once preserves all 360 sampled IB alpha
values. Independent RGB source-over predicts the resulting color within one
channel count in all 105 cases; direct colored siblings miss by up to 39.
Total RGB L1 against that prediction falls from 8,851 to 310. This verifies
the compositing correction for straight probes, not the source paint recipe
or the underlying library implementation. IB's shape/area residuals remain.
The first diagnostic had two harness faults: the wrong alpha comparator and
backing rectangles too narrow for the complete small-size profile windows.
The preserved, newly frozen second version corrects both without changing
the effect. All three original IB no-op renders remain byte-exact.

Native software composition already retains intermediate premultiplied
colors as floats and rounds at output. The SVG failure does not establish
the same issue in Rust. Neither 369 runtime draws nor a new native blur
follows from this reference-only construction.

IE transfers only that single-color operation to the six complete contours,
preserving all IC geometry, paint and finite-end masks. Independent review
verifies six reversible substitutions, nine exact production/HW/IC replays,
600 regional metric sets and 192 junction-row metric sets. Every one of the
50 fixed regions improves or stays exact against production and IC at all
three sizes. All complete ordinary frames improve against HW as well.
Whole-card L1 reductions against production are:

| Size | First card | Third card | Fourth card |
| --- | ---: | ---: | ---: |
| 4K | 810,171 (5.83%) | 715,484 (4.28%) | 254,761 (2.71%) |
| 1600 | 109,924 (5.27%) | 113,134 (4.38%) | 54,296 (3.73%) |
| Fractional | 109,467 (5.82%) | 109,541 (4.67%) | 47,169 (3.52%) |

All fixed regions improve or remain exact against HW at 4K. Relative to
that unaccepted parent, thirteen 1600 regions and two fractional regions
retain L1 or RMS losses; the first lower stem loses 3,071/2,814 L1.
These remain below production errors. Raw cap, side and junction profiles
are retained, including small row-level tradeoffs. All changes stay in the
echo corridor and shelf clip; selected art, content and alpha are exact.
Visual review of all three contour sheets and thirty corner crops supports
the coherent improvement. Cap softness, source hue, ground assumptions and
the raster alpha deficit remain approximations.

IE is accepted as a bounded SVG hypothesis for a native prototype. It does
not require reproducing a unique original paint process, nor does that
acceptance endorse the cost of hundreds of full-contour stroke walks.

IF's first native prototype uses existing masked white strokes and ramps
inside the existing shelf material. Its fresh unmodified build reproduces
the actual round-seventy-three package byte-for-byte at all three sizes.
The candidate preserves or improves all 150 fixed region L1/RMS comparisons
against that native baseline. Whole first/third/fourth-card L1 falls by
794,022/743,228/246,247 at 4K; all three improve at both smaller sizes too.
Independent review reproduces 600 regional and 192 raw junction-row metric
sets, unchanged content/alpha, locality and the exact primary ring transfer.

Four tiny junction rows retain a worse metric despite enclosing-region
gains: first-card 1600 y405/y409 RMS +0.1052/+0.0120 with L1 −12/−11;
fractional y405 L1 +4 with RMS −0.4932, and y412 RMS +0.0965 with L1 −4.
Native and SVG remain different. Compare each effect against its own
baseline before attribution: absolute whole-card differences also contain
pre-existing text, material and renderer differences. No ink or geometry
was tuned to hide native residuals. A compiler-only repair changed eight
generated integer literals to float spelling; both versions are preserved.

IF defines separate world-affine echoes for the inferred ordinary second
card. Its 492 static band paths cover four possible ordinary shapes; a
resting selection evaluates 369. It adds no cache family or frame draw pass.
Capture wall times include fixed settling and establish no performance result.
The literal prototype is preserved in scratch; the reviewed optimized transfer is now integrated.

IG measures actual `SoftCache::prepare` in fresh processes, with two orders
and two passes at three sizes. Independent review checks 414 frozen inputs
and outputs and reconstructs all 24 raw runs. Nanosecond-resolution warm
timing corrects the first pass's whole-microsecond quantization; artwork and
cache calls stay identical. Initial baseline/prototype medians are 400/813 ms
at 4K, 68/182 ms at 1600 and 61/173 ms at the fractional viewport. The latter
prepares a 1537×865 raster within 1537×947. These are CPU preparation times,
not foreground drawing, GPU or total startup latency.

The subsequent twenty card/category choices cost 2257/4235, 356/992 and
324/973 ms respectively; those sums exclude the separately measured initial
prepare. Both variants retain ten entries and identical payload sizes:
201,338,880 / 34,899,200 / 31,656,052 bytes. Three initially missing outputs
and seven later new keys remain resident; prefix composition can still walk
earlier groups when preparing a new output. Shelf hashes are stable within
each variant, and warm handles retain their identity. Warm cache-call medians
are about 0.27–0.30 microseconds, with outliers near 1.9 microseconds. This
establishes reuse, not a zero-cost foreground draw. The literal prototype's
roughly twofold 4K cold penalty warrants optimization before integration.

IH's conservative column-interval walk keeps the per-pixel distance,
antialias and float-blend calculations unchanged. Overlapping intervals
are merged so translucent strokes never paint a column twice. Negative or
nonfinite widths, nonfinite coordinates and magnitudes above 65,536 retain
the full-column walk, preserving large-coordinate rounding behavior.
All 262 library tests pass, including full-column reference comparisons;
all three resting PNGs match IF byte-for-byte. Independent review reproduces
the twelve paired process results and confirms identical shelf hashes,
cache keys, retained bytes and band counts.

First preparation improves from 804 to 635 ms at 4K, 192 to 152 ms at 1600
and 168 to 149 ms at the fractional size. Subsequent twenty-choice totals
fall from 4259/1016/950 to 3304/797/754 ms respectively. Warm cache access
remains submicrosecond. This is a 21% 4K preparation improvement against
the literal prototype; the earlier production measurement was about 400 ms
in a separate session, so new-artwork cost remains.

## State, warm drawing and integration

II verifies ten reference states, three custom states and a disabled-custom
control. All fourteen literal/optimized pairs are byte-exact. Changes from
the production baseline stay inside the fixed frame support, with selected
art, content and alpha exact. The inferred ordinary second card uses its
own world-affine position. Opening, hover/held and fourth-card selection are
covered. Custom colors use generic materials and inherit only the coupled
primary/bar geometry. A first custom scoring control omitted the translated
second-card support; its 383 flagged pixels are retained, and the corrected
control adds exactly the first-card box translated +332. No image changed.
These are headless state hooks, not live desktop input verification.

IJ calls the real Backdrop and Scene CPU draw methods in four fresh ABBA
processes. Each size/state uses five untimed warmups and twenty timed pairs,
for 960 records. At 4K, Scene medians across rest/opening/hover/held are
19.02–20.07 ms for the previous artwork and 18.83–19.89 ms for the new version;
Backdrop is about 6.6–6.7 microseconds. Fractional Scene medians are
20.18–21.28 versus 20.09–21.11 ms. These small differences establish similar
warm cost, not a speedup. The 1600 resting baseline drifts 20.82→24.54 ms
between its two processes, making its apparent gain inconclusive. Timing
excludes geometry disposal, GPU submission, presentation and cold preparation.
The blank probe screenshots are not artwork-parity evidence.

IK integrates the reviewed runtime, trace, component and Store golden.
Unused SVG filter definitions are removed with exact three-size renders;
the component's masks and primary/bar sample match the trace. Full checks
pass: 307 packaged tests, all 22 repository checks, 28 first-attempt matrix
cases, both Store gates and three actual-package captures byte-exact to the
reviewed candidate. The existing one-level Neo-kitsch bar pixel remains the
only nonexact matrix comparison. Fonts, text caches and originals are intact.
Remaining ordinary bottom coverage and unrelated fine printing stay open.
[Round seventy-five](../reference-svg-round75.md) subsequently shares
contour-distance work across consecutive white bands while preserving float
and pixel results. Fresh paired initial preparation falls 626.703→462.161 ms
at 4K (26.3%); the subsequent twenty selections fall 3217.582→2456.071 ms.
Four shelf hashes, the cache state sequence, retained payload, fourteen
state/custom pairs and three actual-package captures are unchanged. The round passes 310 packaged
tests, all 22 checks and 28 first-attempt matrix cases. This closes the
[bounded preparation task](../../todo/performance.md), with no additional
artwork change or new warm foreground/GPU claim. Earlier timings above
describe the round-74 checkpoint.

Evidence is
under `/tmp/cp-eras-next/gz-neomil-frame-echo/`, `ha-frame-echo-expansion/`
and `hb-frame-echo-review/`; `hc-frame-echo-material/`,
`hd-frame-echo-ink/`, `he-frame-echo-ground/` and
`hf-frame-echo-donor-ink/` retain the ground/paint diagnostics.
`hg-frame-echo-cut/` and `hi-fourth-primary-cut/` retain endpoint evidence;
`hj-frame-echo-centroid/` retains the fixed waveform diagnostic. HP, HQ and
HR reside in `hp-frame-echo-svg/`, `hq-echo-vector-coordinates/` and
`hr-echo-vector-map/`; `hs-echo-local-loss/` retains the coverage diagnostic.
`hv-primary-echo-interaction/` and `hw-bar-stem-union/` retain the coupled
geometry trials; `hy-left-echo-ground/` and `hz-echo-side-paint/` retain the
new exterior-ground and filter/paint constraints. `ia-echo-filter-transfer/`,
`ib-echo-profile-bands/` and `ic-echo-profile-contours/` retain the isolated
operator and failed complete transfer. `id-echo-opaque-transfer/v2/` retains
the corrected opaque diagnostic and its preserved harness history.
`ie-echo-masked-contours/` retains the accepted SVG hypothesis and complete
loss lists; `if-echo-native/` retains the native rest transfer and its literal
repair history. `ig-echo-cache-cost/` retains the verified preparation study;
`ih-bounded-stroke/v2/` retains the equivalent optimization;
`ii-echo-states/` retains the state/custom controls, `ij-echo-warm-draw/`
the CPU draw study and `ik-echo-integration/` the integrated package checks.
Independent HP and later reviews are in
`hn-fourth-cut-review/` and `ht-echo-review/`. This work does not close the
broader printing or primary-coverage TODOs.
