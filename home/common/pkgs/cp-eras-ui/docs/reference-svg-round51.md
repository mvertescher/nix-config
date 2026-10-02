# Reference source review — round fifty-one

Three scoped lanes extend the Kitsch ground/material and Neomil inactive
notice investigations. Sol prepares rendered diagnostics and held-out
measurements; Luna indexes the evidence; Astra reviews original crops,
reproduces measurements and makes the integration decisions. No new
runtime, reference artwork or golden is accepted. All 29 broad TODOs
remain open, and changes remain staged and uncommitted.

## Kitsch: deeper fill response and local background controls

DI changes only the two inherited deeper-ghost fill red values, from 15
to 0 or 255. The 32 inherited fill uses respond; the 28 stroke-only uses
and six explicit nearest-ghost gradients retain their attributes. The
R15 SVG is byte-identical to production and its rendered pixels match
the reviewed current frame. Every green, blue and alpha output pixel
is unchanged at both endpoints. The response
`K=(render_R255-render_R0)/255` includes overlapping deeper fills;
it is not a source-photo opacity measurement. Endpoint interpolation
reconstructs R15 with red MAE 0.060 and maximum error 3 because painted
layers are quantized. Astra checks the reverse XML changes and output
planes before using the response.

DJ tests a common hypothetical deeper-fill field,
`R=clip(15+gain*max(0,ground_R-threshold),15,255)`, using the complete
current ground, including its left wash. Only the frozen 7,243 original
Weapons depth-2 pixels train the threshold and gain. The search fixes
threshold 33 and gain 1.3216715 before other regions are scored.
Prediction is `current_R+K*(R-15)`: this is an analytic diagnostic,
not an SVG or native candidate.

| Frozen region | Current red MAE | Predicted red MAE |
| --- | ---: | ---: |
| Weapons depth 2, training | 19.471 | 2.137 |
| Weapons depth 3 | 21.965 | 3.617 |
| Vehicles depth 2 | 1.782 | 1.792 |
| Vehicles depth 3 | 2.708 | 5.890 |
| Right Products depth 2 | 35.530 | 22.648 |
| Right Products depth 3 | 33.993 | 21.712 |

The two Locations regions are unchanged. Weapons depth 3 also retains
local bin losses despite its aggregate gain; 53.4% of Vehicles depth-3
pixels worsen by more than three red levels. Reject this standalone
field. Do not refit its threshold on those heldouts or present the
prediction as a rendered material correction.

The nearby Vehicles controls strengthen the reason to separate ground
from fill. Geometry is frozen before source sampling: positions must be
within 60 original pixels of the frozen depth-3 footprint and outside
modeled foreground expanded by eight or sixteen design pixels. At the
eight-pixel guard, 1,765 positions are clear in all three modeled screens,
but they form only a small upper-left strip. Dashboard minus Mail mean
red is +0.104; Dashboard minus Store is −2.490. Dashboard source is
7.805 red levels darker than the current modeled ground there.

The sixteen-pixel guard has no three-photo overlap. It retains 532
Dashboard/Mail positions with mean red difference −0.021 and Dashboard
ground residual −6.447, and 712 Dashboard/Store positions with residual
−8.455. Astra independently reproduces all six guarded intersections
and their original RGB means, then reviews the source context. These
are useful local controls; they do not observe the hidden ground under
the rest of the card or prove its original alpha.

## Kitsch: frozen ground grid and rejected ellipse-only correction

DK freezes 819 ten-by-ten-design-pixel patches on a 39×21 grid. Each
patch is 24×24 pixels in the original photos; its train/holdout label
comes from a geometric checkerboard. A photo counts as geometrically
clear only when every patch pixel lies more than sixteen design pixels
from its modeled foreground. At least two photos must be clear, and
all eligible-clear photos must have per-channel median spread at most
three. Source closeness to the modeled ground is not a selection rule.

There are 463 geometrically eligible patches: 438 agreement candidates
(220 train, 218 holdout) and 25 disagreements. The other 356 fail the
geometry rule. Astra reproduces every bound, exclusion mask, split,
eligible source mean/median and candidate flag, and views accepted and
rejected contexts. Agreement is evidence for candidate ground controls,
not proof that every photograph lacks unmodeled shading. Coverage has
large middle holes: only 24 of 168 central patches at y230–550 survive.

DL tests four rose-ellipse parameters with all six RGB stops, the page,
left wash and card material frozen. It trains only the 111 candidate
checkerboard-training patches at x≥350/y≤650. Each training target is
the mean of eligible-clear source means, and the predictor averages all
576 original pixel centers. The bounded robust fit freezes
`(cx,cy,rx,ry)=(781.0034224,-362.7919669,1529.5710331,1015.3431199)`
before scoring, versus current `(750,-331,1600,929)`.

The final ground-only SVG uses that explicit user-space ellipse. Its
opening gradient tag is the only changed text; stops and all other
bytes are exact. A preliminary bounding-box-transform render had a
coordinate mapping discrepancy and is excluded. The final render is
pinned by SHA-256
`546e548e915ca855bcc832496fa3e9bffe3f594f27ad5bc5b22e36f92736f4b2`.
The unchanged geometry expressed in user space is a separate control:
it differs from the current bounding-box raster by mean 0.228 RGB levels,
maximum 2, so this representation is not claimed to be pixel-identical.

Astra checks the final ellipse coordinates, reverse SVG edit, original
photo means and all 1,149 clear photo-patch records. Actual rendered
ground error is:

| Scope | Photo-patches / distinct patches | Current RGB MAE | Candidate RGB MAE |
| --- | ---: | ---: | ---: |
| Fitted-domain training | 282 / 111 | 3.340 | 3.030 |
| Fitted-domain heldout | 283 / 111 | 3.357 | 3.073 |
| All checkerboard heldouts | 573 / 218 | 2.551 | 2.488 |
| All candidates | 1,149 / 438 | 2.562 | 2.496 |

The fitted-domain heldout green error worsens 2.987→3.555; 150 of its
283 observations worsen, six by more than three RGB MAE. All-candidate
left error grows 4.579→5.046, upper error 3.930→3.958 and Store error
2.389→2.400. Source-context review confirms substantial local losses,
including a +6.00 Mail holdout at design (430,390). Reject this
ellipse-only correction. A future coupled ground/fill hypothesis must
retain these channel, screen and local controls; an average red gain
does not establish a valid shared background or deeper-card material.

## Neomil: exact shaped-run replay

DI investigates the failed zero-offset CLASS span split from round fifty.
The pinned [librsvg 2.62.3 text implementation](https://raw.githubusercontent.com/GNOME/librsvg/2.62.3/rsvg/src/text.rs)
creates a separate Pango layout for each span and advances subsequent
spans using measured widths. Its SVG position-list parsing retains only
the first value. Astra reviews that implementation and independently
checks eighteen complete-image comparisons. A single CLASS span changes
3,230/1,034 pixels at 4K/1600; five spans change 4,503/1,180. Preserved
whitespace does not fix segmentation. Full zero `dx` lists replay, but a
nonzero later entry is ignored. None of these is a source-fidelity fit.

DJ reproduces the original unsplit 62-glyph Pango run as a Cairo path on
an identity recording surface, then applies the viewport scale at fill.
The font, glyph IDs, cluster indices, advances, baseline and negative
tracking remain pinned. Direct layout painting had not replayed the SVG;
this path route does. Astra verifies exact alpha across both entire
first-line bands at 4K and 1600, with zero harness alpha outside them.
RGB recomposition differs from BJ by at most one channel level.

DK extends this to an editable copy of the original glyph string and
again passes the exact no-op alpha gate. Individual original glyph
paths disambiguate BJ's touching L/A without reshaping isolated text.
This resolves the experimental replay prerequisite, not the original
font identity, native renderer port or final notice fidelity.

## Neomil: distinguish absolute registration from internal spacing

In card 2 at 4K, source C/L/A/S/S R155 starts are
`1983/1995/2004/2015/2026`; isolated BJ starts are
`1981/1993/2001/2013/2023`. R155 uses signed RGB, `R>155 && R>2G`.
DK freezes `[0,+2,+3,+2,+3]` pixel offsets while leaving C fixed. This
absolute-target interpretation mixes the two-pixel C registration error
into the other letters. It improves both CLASS crops and complete first
lines, but joins A/S and worsens the following gaps: 4K false R155 ink
grows 6→15 and 14→23; both 1600 gap RGB errors also worsen. Reject DK.

After preserving that result, DL separately freezes C-relative offsets
`[0,0,+1,0,+1]` at original 4K, or `[0,0,427,0,427]` Pango units.
They follow only the frozen card-2 relative source/BJ starts;
card 3 and 1600 receive the same design-space offsets without fitting.
All glyph widths, shapes, advances and following text remain fixed.

| Scope | BJ RGB absolute-error sum | DL RGB absolute-error sum |
| --- | ---: | ---: |
| 4K four inactive lines | 1,797,457 | 1,779,134 |
| 4K two CLASS crops | 140,587 | 122,352 |
| 1600 four inactive lines | 237,328 | 234,570 |
| 1600 two CLASS crops | 17,313 | 14,647 |

At each size four of the 81 fixed regions improve and 77 tie, with no
RGB-region loss against BJ. All adjacent gaps, other words, second lines,
O holes and exterior pixels are exact. Missing source R210 core pixels
in CLASS fall 198→180 at 4K and remain one at 1600. Astra reproduces
13,608 numeric checks per trial, verifies mask recomposition and no-op
bands, and inspects enlarged source/current/BJ/candidate crops.

DL nevertheless leaves A and the first S touching on both cards and
sizes, and C remains misregistered. Its CLASS RGB error also remains
worse than the current native baseline (110,106 at 4K; 13,255 at 1600).
Retain this useful spacing diagnostic without integrating BJ or a new
native text path. The next bounded question is the remaining glyph
width/contour and word-registration error, preserving the now-exact
shaping, suffix, gaps and bright-core controls.

## Evidence and verification boundary

Detailed frozen inputs, scripts and outputs remain under
`/tmp/cp-eras-next/`: `di-kitsch-fill-response`,
`dj-kitsch-ground-fill`, `dj-kitsch-vehicles-ground`,
`dk-kitsch-ground-grid`, `dl-kitsch-ground-ellipse`,
`dl-kitsch-ground-validation`, `di-neomil-shaping-replay`,
`dj-neomil-pango-replay`, `dk-neomil-class-origins` and
`dl-neomil-class-relative`. Independent checks are in `dk-root-review`
and `dl-root-review`; the evidence index is in `dl-evidence-index`.
The DL ground `FINAL.json` pins the corrected final artifacts.

Only this report and existing prose/TODO records change. Runtime, SVG,
scripts, tests and goldens retain round fifty's exact hashes. All sixteen
original photos, repository HEADs and lock files are unchanged. CZ's
295-test, full repository-check and packaged-capture checkpoint remains
the latest implementation validation; it was not rerun for prose-only
changes. Markdown links, staged diff whitespace and source/index scope
are checked. No task is closed on these diagnostic results, and nothing
is committed, pushed or deployed.
