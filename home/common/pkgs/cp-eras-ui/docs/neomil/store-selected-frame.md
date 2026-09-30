# Selected STORE frame and card-edge printing: source plan

This records the source measurements and reviewed correction for the
selected card in `images/img-09-store.png`; the V native result is below.
The source, frozen R SVG, and R native
captures are all 3840×2160. The scratch SVG and four-way crops are in
`/tmp/cp-eras-resume-20260929/u-selected-frame/`; its T baseline already
includes the accepted PETROCHEM 1.3 tracking. No ordinary-card contour,
left edge, certification icon, title, gun, or fourth-card crop changes.

The source's **bright primary stroke** was measured with R>180, R>2G,
R>1.7B in clear local strips. Dim scan echoes and glow do not set these
coordinates. The selected left edge is x1845 in 130 consecutive source rows,
versus x1844..1845 SVG and x1844..1846 native. The top-left certification
icons already have source/SVG and source/native mask IoU .987/.982 at zero
translation. The card cannot be moved as a whole.

| Selected anchor | Source native | T SVG native | Scratch candidate native |
| --- | --- | --- | --- |
| Top straight, x1880..2460 | y370, 580 bright pixels | y361..362 | y370, 580 bright pixels |
| Chamfer to right stem | starts near (2487,370), reaches x2515..2517 by y400 | starts x2491..2492/y361..362, reaches x2522..2523/y393 | top/chamfer mask IoU 0→.818 |
| Right stem, y470..625 | x2515..2517 | x2522..2523 | aligned at x2514..2517; mask IoU 0→.779 |
| Upper fan, x2488..2528/y625..680 | inner edge reaches x2493 by y651 | reaches x2494 by y666 | mask IoU .505→.893 |
| Lower fan, x2488..2528/y958..1007 | right edge starts retracting near y970 and reaches x2493 by y993 | reaches inner edge near y999 | mask IoU .776→.859 |
| PETROCHEM box | vertical sides x2461 and x2478 | x2472..2473 and x2489..2490 | translated with both words by −4.6 design px; combined box/brand mask IoU .082→.390 |

The SVG-only proposal uses selected outer path local top y154.5833,
upper-right top x267.25, right x279.3333/y166.6667, and inner side
x269.75/y271.25. It keeps the left edge x0 and accepted lower-right
corner. Its fan bar begins at local x279.3333/y262.9167, reaches the
inner side at x269.75/y271.25, and ends with a lower diagonal from
x279.3333/y403.75 to x269.75/y413.3333. The selected upper-wash path
follows that same top/chamfer/side contour and retains its accepted
lower y492→516 diagonal. A separate SVG outline over the wash restores
the source's bright chamfer and stem core; the scratch uses 1.2 design
width on the chamfer and 1.4 on the stem because SVG raster quantization
changes their native pixel coverage differently. Rust stroke widths need
native review rather than direct adoption from this raster fit.

The candidate changes 20,527 rendered pixels overall, confined to the
selected top/right contour, its material boundary, and the two boxed
side-print runs. The certification icons, title, gun, selected left
stem, and an ordinary-card crop are pixel-identical to the T SVG baseline.
At independent strong-red thresholds 160/180/200, source/candidate mask
IoU improves for top, chamfer, right stem, both fan ends, and box/brand.
The middle wash is unchanged. In a clear strip immediately above the
primary top line, mean source RGB absolute error changes from
(52.19,8.27,6.20) to (40.30,3.97,11.22); the source has dim red echoes
there, so the new clip is an outline/material boundary approximation,
not a claim that the echo field matches.

The selected box and both words move together. The box's y endpoints
already match the source; PETROCHEM's vertical letter spacing is the
separate accepted T correction. Source side-printing sits about 11
native pixels left of T, around four pixels farther inward than the
right-frame discrepancy alone. BETTERLIFE on an ordinary card has
source/native first, middle and last x centers within a fraction of a
pixel, so moving the shared card-edge printing would be a regression.
The selected-only offset should keep all feedback colors and text paths.
The scoped Rust and store SVG/component paths now carry this geometry.
The Rust foreground paints the selected contour last, above its one-pixel
wash bands. U native review showed that a closed 1.2-design outline made
the top straight bright on y369–372, against the source's single strong
row y370. The V correction keeps the 1.2-design chamfer, right stem,
lower edge and left edge in an open body path, then draws only the top
as a 0.4-design-pixel filled line centered at y154.5. Both live in one
final `GROWN` array slot, preserving feedback indices. In the native
scratch B trial, the bright core appears only on y370. Its median red
is 221/154 on y370/y371 against source 207/167, versus U's 251/251;
independent left, middle and right top-strip red MAE over y368–373 falls
from 61.3 to 20.0/20.7/21.1. The remaining softness mismatch is visible,
but the overpainted two extra rows are gone. The U SVG already has the
source-aligned top core and keeps its contour unchanged.

The selected PETROCHEM box was aligned but too dim in native U: its
straight pixels used fixed `GUN_LIT` (176,44,48), while source and SVG
box cores are (251,53,53). Only this selected box now uses foreground
ink; ordinary boxes and the selected gun keep their prior inks. The
reference material changes only its upper and lower contour masks; its
ramp origins and stops remain at their fitted
world coordinates. Selected hover and held states share the new wash
contour and side-print offset, while plate hits and permanent fourth-card
clipping retain their existing bounds. The combined V source receives
native state and material-clip review below.
## V native result

The production 4K render matches the reviewed split-top/bright-box trial
at every pixel. Independent left/middle/right top-strip red MAE is
20.03/20.71/21.08, versus 61.37/61.26/61.34 before the narrow top rule.
Fractional rest, held ordinary/selected, fourth-card selection/press,
custom palette and opening captures retain the intended frame state and
cutoff. The held outline's geometry remains in the display list with
its paint disabled; the existing recursive feedback regression passes.
Integrated validation is recorded in the eleventh checkpoint; fine
printing/edge echoes remain open.
