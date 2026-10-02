# Entropism mailbox badge lettering

The source is `images/entropism-mail.png` (3840×2160), with `T1 T3`
above `T2 T4`. The old stretched Rajdhani labels fitted the overall
bounds but had 11–12-pixel T stems against source cores of 7–8 pixels.
AR corrects the four T silhouettes and digit 1. Frames, wording,
selection and inks stay intact. AT fits digit 3, AU fits digit 4, and BT
fits digit 2 after correcting the earlier AU trial’s small-size foot failure.

## Source calibration

The common T was fitted on T1, then checked on the other badges. Initial
placement failed local controls: T3 needed a −3-source-pixel vertical
shift and selected T2 needed +2.5 pixels. Those are additional source
calibration, not independent holdout successes. T1/T4 keep their phases.
The first native candidate improved whole-T overlap but joined the wider
T1 crossbar to the old digit 1; its selected T2 stem also sat too far right.
That candidate was rejected.

The accepted digit 1 follows the source's narrow vertical and angled
shoulder. In the 1600×900 design frame it is:

```
M1387.5 253.333 H1389.167 V270.833 H1386.667 V257.917
H1382.917 V255.833 L1385.417 255.417 L1386.25 254.583 Z
```

T2 retains its bar and shifts the stem left by two source pixels on its
left edge and one on its right. Its contiguous polygon is:

```
M1358.75 332.292 H1376.667 V334.792 H1368.75 V348.958
H1365.417 V334.792 H1358.75 Z
```

The component excerpt applies the parent geometry at translation
(+284,+478); the old comment's +465 was incorrect. Its 578 changed pixels
stay within the four lettering regions. Both SVGs have unique IDs.
Converting T2's touching bar/stem subpaths to one outline preserves every
rendered pixel at 4K and at the component sheet's native size.

## Local native review

Fresh baseline frames match the previously verified package and golden.
At 4K, all sixteen whole-region threshold comparisons improve. The table
uses green 130 for bright labels and dark 135 for selected T2. T1 includes
both replaced glyphs; the other regions cover their T alone.

| Region | Previous source IoU | Corrected source IoU |
| --- | ---: | ---: |
| T1 label | .5524 | .9741 |
| T3 T | .6394 | .9088 |
| T2 T | .5820 | .9155 |
| T4 T | .7609 | .8746 |

Source and corrected T1 have two large connected ink components at green
100/115/130/145. At row 614, the corrected six-pixel gap matches the source
at the first three thresholds; the source's strictest edge has seven.
Digit-1 IoU improves at all four thresholds, including .5052→.9490 at
130. All fixed selected-T2 stem segments improve. T3's left bar edge has
two residual losses: .8559→.8454 at 100 and .7526→.7174 at 145. Its other
segments and overall silhouette improve; no additional edge tuning is
claimed. Photographic softness and exact original letterforms remain open.

All 1,834 changed 4K pixels lie in the intended glyph regions. At
1600×900 and 1537×947, 523 and 422 pixels change in those regions only;
full retained-digit RGB regions are exact. Small-size source crops show
the narrower stems and restored T1 separation. Fractional comparisons
place source crops about the responsive anchors using uniform glyph
scale, rather than stretching their letterforms vertically with the window.
All twelve normalized 1600px source controls improve at .35/.50/.65.
Fractional source crops use antialiased Lanczos downsampling at the same
uniform scale: ten of twelve controls improve. T1's loose .35 overlap
falls .7692→.7476, while .50/.65 improve .6344→.8994 and .6636→.9615.
Selected T2's .65
core loses .7034→.6250. Its strict stem is two pixels wide against three
in the resampled source; the old stem is four pixels wide and offset left.
The bar also retains fewer strict core rows. Its .35/.50 overlap improves
.7770→.7843 and .7548→.8354. This is a remaining small-raster/printing
tradeoff, not an all-threshold success or a reason to invent a viewport
breakpoint. The earlier bicubic affine diagnostic is retained separately;
correcting its downsampling method did not eliminate this strict loss.
Fresh production frames at all three sizes exactly match the reviewed
candidate.

## Runtime and verification

`MailBadgeArt` carries the expected text, replaced prefix length and paths
in era data. The renderer retains the suffix paths from the original full
label, preserving shaping, advances and raster phase. Mismatched text,
unsupported shaping and unstretched runs use the original text path.
Art uses uniform text scaling about the responsive label anchor and the
current foreground/selected role, including opening opacity and clipping.
Other eras provide no badge artwork.

All thirteen paired states confine changes to the approved art regions,
including row selection, ordinary/selected hover and press, custom colors
and two partial opening times. Retained digits are RGB-exact even where
their test regions overlap the broader art bounds. Four custom stem probes
match their current palette colors exactly. The fully hidden frame is
unchanged. These are synthetic drawing states, not live desktop input.

All 290 Rust tests, 24 SVG structural checks and the source gate pass.
The implementation gate also passes. All 22 repository checks and all 27
visual cases pass on their first attempt; the Nix package passes 290 Rust
tests. Direct comparison preserves only the pre-existing one-level
Neo-kitsch bar pixel. All 296 frozen files match the tested Nix source,
six originals are unchanged, and a fresh packaged 4K mailbox frame matches
the reviewed candidate exactly. Only the Entropism mailbox golden changes
in AR, by 523 pixels. The bounded contour/gap task closes; the explicit
edge/printing residuals remain in E2.


## AT: the T3 numeral

The old font 3 adds heavy left bars absent from the source. A single
32-point outline now follows the photographed digit's connected core,
relative to the existing label anchor. T3 replaces both glyphs through the
existing `MailBadgeArt` path; its accepted T is unchanged. SVG and component
art use the same digit, with the component's (+284,+478) translation.

Six frozen 4K source regions improve at green 100/115/130/145. Whole-digit
native IoU at 130 rises .6212→.9337; upper/lower-left excess ink falls
60→0 and 48→1 pixels. The empty T/3 gap loses all 18 extraneous pixels.
These are primary silhouette corrections: tiny source-edge pixels remain
missing, and no photographic softness model is claimed. The source/SVG
comparison also improves, with a one-pixel upper-left strict-threshold
loss in the downsampled 1600px SVG control.

At normalized contrast .50, native whole-digit IoU improves .6380→.9249
at 4K, .6390→.9296 at 1600×900 and .5103→.9091 at 1537×947. All three
thresholds improve at every size. Exactly 554/129/135 pixels change,
confined to digit 3; the other glyphs and screen remain pixel-identical.
Nine paired state checks cover actual mailbox row-selection messages,
synthetic badge selection and custom palette roles, hidden art and two
partial opening frames. They preserve the same locality and leave the
hidden frame unchanged. They do not simulate live desktop pointer input.

The existing renderer supplies current foreground/selected ink and opening
clipping. The correction changes era artwork only. All three integrated
production frames exactly match the reviewed candidate. Both fidelity
gates, 24 SVG structural checks and 292 Rust tests pass. Only the
Entropism mailbox golden changes, by the reviewed 129 pixels.
All 22 repository checks and 27 visual cases pass on their first attempt.
Direct pixel comparison finds 26 exact cases and only the pre-existing
one-level Neo-kitsch bar pixel. All 299 frozen files match the tested Nix
source; seven frozen source images are unchanged. The package passes all
292 Rust tests, and its fresh 4K mailbox frame exactly matches the reviewed
T3 candidate.
The digit-3 silhouette correction closes; broader E2 printing remains open.

## AU: the T4 numeral and rejected T2 trial

T4 now follows the source's lighter diagonal, triangular counter, crossbar
and stem. One closed path with a reversed counter replaces the font digit;
the accepted T stays separate. The SVG uses the same contour and the
component translates it by (+284,+478). The existing native renderer uses
its normal winding fill, so the counter's reversed direction matters.
There are no overlapping fills and no change to shared drawing code.

Every nonempty fixed 4K native region improves at green 100/115/130/145;
the empty T/4 gap remains empty. At 130, whole-digit IoU rises
.5234→.9105, counter .2241→.6944, diagonal .4298→.8038, crossbar
.6303→.9308 and lower stem .7800→.9750. Source/SVG whole-digit IoU
rises .5285→.9135. The revised path loses a few boundary pixels versus
the rejected three-overlapping-polygon representation, which doubled
antialiasing along shared edges. It still improves each source control.

At normalized contrast .50, native whole-digit overlap improves
.5565→.8903 at 4K, .4933→.8176 at 1600×900 and .5000→.8594 at
1537×947. All three contrast thresholds improve at each size. Exactly
587/161/122 native pixels change, confined to digit 4. The comparison
maps the source about the responsive anchor with uniform glyph scale.
Source photographic softness remains a separate E2 task.

The candidate for digit 2 improves the 4K core but fails the fractional
foot. With a fixed green threshold of 120, fractional whole-digit IoU
falls .745→.685 and the foot .900→.400. Foot row coverage in the fixed
crop is [2,13,15,15] in the downsampled source, [5,15,15,15] in the old
font and [2,2,14,0] in the candidate. The source mapping is independently
reproduced; removing crop normalization or shifting by one pixel does not
remove the loss. The candidate removes extra ink at the cost of missing
middle-strength source coverage. Digit 2 remains unchanged; a viewport
breakpoint or whole-label shift is not justified by this evidence.

Integration checks are recorded in [round thirty-two](../reference-svg-round32.md).

AU validation: eleven paired drawing states preserve the T4 region and
current foreground/selected colors, including pre-reveal and partial-clip
frames. Production parity holds at all three sizes; the fresh packaged
4K frame matches too. Both fidelity gates, 24 SVG structural checks,
292 Rust tests and all 22 repository checks pass. All 27 visual cases pass
first attempt. Direct comparison finds 26 exact cases and only the pre-existing
one-level Neo-kitsch bar pixel. All 300 frozen
files match the tested Nix source; seven originals remain unchanged.
Only the mailbox golden changes, by 161 pixels. The bounded digit-4
silhouette task closes; exact printing and digit 2 remain in E2.

## BN: T2 foot diagnosis, no new contour

The frozen AU source/native controls still reject digit 2. BN localizes
the fractional loss to the foot's leading and bottom rows; a global
one-pixel shift worsens the whole digit. AU's local-Lanczos acceptance
scores above stay primary. A separate uniform inverse-affine Bicubic
sample around the same responsive label anchor gives whole-digit IoU
.786→.649 and foot .940→.383. Only the source resampling changes; native
frames, threshold, anchor and apertures are identical. Astra reproduces
both methods. They support the same rejection and must not be mixed into
one acceptance comparison.

The next bounded hypothesis is a foot-local subpixel edge profile. It must
restore the missing fractional bottom row and medium-strength leading row
without painting a false solid bar at strict thresholds, harming the sweep
or T/2 gap, or regressing either smaller-size whole-digit control. The
original AU controls and the separate resampling sensitivity both remain
visible. No path, ink, layout or golden changes from this diagnosis. See
[round forty-two](../reference-svg-round42.md) and scratch evidence in
`/tmp/cp-eras-next/bn-entropism-foot/`, including `METHOD-CLARIFICATION.md`.

## BT T2 foot correction (2026-09-30)

The 27-point source contour replaces the remaining font digit while keeping
the accepted T, complete literal label, anchors and palette roles. Its
foot top is at relative y−2.458333; bottom y+.1875 retains the small-size
rows without making all 36 pixels of the 4K bottom row dark. The source
has four green<100 pixels there; native retains zero, with 36 below120.
Whole-digit IoU at green120 improves .623→.926 / .555→.868 / .745→.899
at 4K/1600/1537. Every regional RGB comparison improves. Fractional foot
IoU at green140 retains .940→.914894, and the separately labeled Bicubic
sensitivity retains upper/foot losses. These remain photographic-printing
limits within E2.

Nine actual-MailBox pairs verify selection, synthetic unselected material,
custom roles, hidden art and two partial opening clips. The unchanged
T and all other artwork stay exact. The SVG and component use the same
contour; native uses responsive anchors with uniform glyph scaling.
BT verifies production/package parity, both gates and the full repository
check. Only the mailbox golden changes by 146 pixels within this screen.
Combined verification is recorded
in [round forty-three](../reference-svg-round43.md).
