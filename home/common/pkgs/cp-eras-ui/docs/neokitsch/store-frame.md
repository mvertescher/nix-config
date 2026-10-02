# NK-14 store frame lower/right outer contour

The photographed store (`images/neokitsch-store.png`, 3840×2160) places the outer right stem slightly farther right and its bottom turn lower than the earlier trace. This correction affects only the outer contour of each card. The three plain cards share one path in Rust; the expanded selected card has its own path. The five echo paths, fitted upper turns, tab, left cutoff diagonal, inks, art, and text retain their existing values.

At native rows y=1490, 1510, 1520, 1525, 1530 and 1535, card 1's outer ridge is x=`1495,1495,1495,1495,1494,1491` in the source versus `1494,1494,1494,1493,1491,1484` in the previous SVG. The corrected SVG gives `1495,1495,1495,1495,1494,1491`. Independent plain cards 3 and 4 move from mean x errors 3.50 and 3.33 px to 1.17 and 1.00 px. On the selected card, rows y=1660, 1680, 1690, 1695, 1700 and 1705 move from mean x error 3.50 px to 0.00 px. Across all 24 readings, mean absolute error falls from 3.21 to 0.63 px. These are the rightmost red-channel ridges using a three-row mean and peak prominence above 20, so they measure position rather than opacity.

The source's outer bottom ridge is y=1538 on all three plain cards; the previous SVG/native ridge was y=1536. The selected source is y=1709 versus previous SVG/native y=1706. The corrected SVG gives y=1538/1709. The plain path transitions smoothly from its old stem x at design y=590 to x+0.4 before the lower turn, which starts at y=634.4 and ends at y=641.2. The selected path transitions from y=660 to x+0.8 and turns from y=705.3 to y=712.55. The tighter lower radii align the photographed bend. At the left cutoff, the path makes a short return to its original y (0.8 plain or 1.25 selected design pixels) before closing, preserving the accepted diagonal and tab exactly. That return lies at the inner end of the bottom stroke, not at a new x position.

The previous frame-ink audit found incompatible source ridge brightness on plain versus selected backgrounds, so no uniform color or echo-opacity change accompanies this geometry. W native review confirms the outer bottom ridges at y1538/1709; the later X echo review is below.

## Five lower/right echoes

The source resolves six bottom ridges (five echoes and the outer contour) at native x=1350, 2834, and 3578 on the plain cards: y≈`1501, 1508, 1515, 1523, 1530, 1538`, inside to outside. Before this correction, the trace resolves only five because the faint fifth echo ends at design y=631.2 and overlaps the third. The revised plain-card paths end at design y=`637.8, 634.7, 631.2, 628.6, 625.5`, outside to inside, with turns beginning at `632.4, 630.5, 627.4, 622.7, 621.0`. A short cubic below the unchanged upper/right corner eases the first three stems right by 0.4, 0.4, and 0.6 design pixels. The other two stems keep their x position. All five retain their original left endpoints, count, ink, and upper/shoulder geometry. The selected card uses the same local path shape with its established +70.9 design-pixel lower extension.

The corrected SVG resolves all six plain-card bottom ridges at those source rows; card 4's photographed echo 1 is one native pixel lower. At selected x=2100, source ridges are y≈`1671, 1679, 1687, 1694, 1702, 1709`, while the corrected SVG gives `1671, 1678, 1685, 1693, 1700, 1709`. The selected middle echoes therefore retain 1–2px vertical differences. Side-bend holdouts also support the shared shape: plain card 1 at y=1490 has source and corrected echo x positions `1459, 1467, 1474, 1481, 1488`, and plain cards 3 and 4 stay within about one pixel. At selected y=1661, source and corrected x positions are `2194, 2202, 2209, 2216, 2223`. At y=1505 plain / 1676 selected, the fifth echo has already turned in the source and corrected SVG; the earlier trace still shows an extra side peak.

The fifth ridge remains too dim in the trace: its measured source prominence is 36 on plain card 1 and 69 on the selected card, versus 14 and 13 in the corrected SVG. This is a photographic ink/halo residual, not evidence for a uniform opacity increase. The native echo review below retains the selected position residual and softness as open work.

## X native verification

All six lower ridges now resolve in Iced. On all three ordinary cards,
native y positions are `1500,1508,1514,1522,1530,1538`, within one pixel
of the measured source (card 4's second-outer ridge is one pixel lower).
The selected native positions are `1671,1678,1684,1693,1700,1709`,
retaining 1–3px errors on four inner ridges. A selected-only trial aligns
the straight bottom span more closely but worsens independent side-bend
probes, so it is rejected. No uniform radius/spacing adjustment is applied.
Fractional resting/held selected/held fourth-card captures retain clipping
and feedback. The W shape-gate classification boundary is documented in
the [checkpoint](../reference-svg-round11.md); no threshold was changed.

### AA fifth-echo opacity trial rejected

Raising only the fifth path's opacity .08→.20 keeps geometry fixed and
approaches the ordinary straight ridge: local red prominence changes
16.33→40.50 against source 35.33–37.67 on three independent cards. The
selected ridge remains too weak, 15.33→41.00 against source 67.81. All
four neighboring-gap RGB errors worsen, and the shared path/halo changes
175,569 pixels across the full frames, including upper shoulders whose
errors also increase. Side-bend gains do not justify those regressions.
Retain the accepted geometry and existing ink. A local or state-specific
material model would need independent evidence; this uniform fifth-path
trial supplies no accepted implementation.

### AF lower-fifth profile trial rejected

The source's fifth bottom ridge is narrower and more concentrated than the
current native stroke. Across 21-pixel horizontal averages centered at
native x=1350/2834/3578, the three plain-card source ridges peak 35.7–36.4
red levels above the adjacent dark gap and integrate to 58.1–59.0 levels
over five rows. The selected ridge at x=2100 peaks 65.7 and integrates to
83.6. The current native gives 19/49 on each plain card and 19/45 on the
selected card. The photographed line therefore has a distinct selected
strength, while its narrower peak means opacity alone cannot fit both peak
and integrated ink.

A scratch native preview split only the fifth path at design y=590 (plain)
and y=660 (selected), leaving its upper shoulder and upper side unchanged. It
continued the lower turn and straight span at width 0.55 instead of 1.0,
with opacity 0.16 on plain cards and 0.28 on the selected card. The frozen
trial source is `/tmp/cp-eras-next/af/neokitsch/frame-preview-590.rs`; its
capture is `/tmp/cp-eras-next/af/neokitsch/native-trial.png`, scored by
`/tmp/cp-eras-next/af/neokitsch/score.py`. It was not applied to the trace
or runtime.

| Fifth-ridge probe | Source red peak / area | Previous native | Trial native |
| --- | ---: | ---: | ---: |
| Plain card 1 bottom, x=1350 | 35.8 / 58.1 | 19 / 49 | 30 / 53 |
| Selected bottom, x=2100 | 65.7 / 83.6 | 19 / 45 | 63 / 90 |
| Plain card 3 bottom, x=2834 | 35.7 / 58.5 | 19 / 49 | 30 / 53 |
| Plain card 4 bottom, x=3578 | 36.4 / 59.0 | 19 / 49 | 30 / 53 |
| Plain card 1 side, y=1490 | 39.6 / 50.9 | 17.4 / 42.6 | 27.1 / 50.6 |
| Selected side, y=1661 | 32.4 / 78.4 | 16.3 / 43.6 | 45.4 / 92.1 |
| Plain cards 3/4 side, y=1490 | 38.7–41.9 / 83.9–95.7 | 17.4 / 43.4 | 23.0 / 51.1 |

Peak is the red-channel maximum above the neighboring gap; area is the
positive red excess over that gap across five rows or columns. The source
retains photographic softness and texture, so a pixel peak alone is not an
exact ink target. Native lower-ridge gaps remain [13,9,12] RGB in both the
previous and trial captures; the source gaps are approximately
[17–18,12–13,13]. Unchanged upper-shoulder probes at x=1360 and x=2100
are byte-identical between the two native captures. The trial side peaks
remain one native pixel left of the source on all four cards.

The original split introduces a visible strength step after the source
ridge has already reappeared below the socket rule. At card 1's side,
source red contrast rises from about 0 at y=1394 to 24 at y=1397 and 52
at y=1400; the trial stays at 19 through y=1412 and jumps to 30 by
y=1418. On the selected side, source contrast rises from 5 at y=1568 to
36 at y=1574, whereas the trial remains 19 through y=1580 and jumps to
54 by y=1586. Moving the split up to the socket-rule crossing would
remove the late step but would also advance the selected side's excess
brightness. The trial therefore fails independent side and seam controls
despite the straight-bottom gain. A future fit must measure orientation
and transition through the bend in both states; the selected straight
and side cannot share this opacity/width pair. The exact photographic
stroke or glow recipe remains unknown.

## AU: separate the frame core from the SVG photo halo

AF's fifth-echo experiment measured native pixels; AJ measured the full
SVG with its photo-only line halo. AU explicitly separates those baselines
across twelve fixed top/side/bottom strips on ordinary cards 1/3/4 and the
selected card. Native deliberately omits the broad halo; comparing the
two as though they shared a surrounding floor obscures the defect.

At the ordinary fifth bottom ridge, source red peak/gap/contrast is about
53/17/36, native 32/13/19, full SVG 44/33/11, and SVG without the line
halo 30/13/17. Selected source is 84/18/66 while native remains 32/13/19.
Removing the halo alone does not fix the core and worsens full-strip RGB
error in eleven of twelve windows. Source/SVG ordinary bottom peak
positions already agree; selected middle bottom ridges remain 1–2 pixels
high, top outer ridges up to 2 high, and side ridges about 1 left.
Intensity fitting must keep those positional errors separate.

One fixed-geometry pixel-profile diagnostic fits six narrow cores plus a
limited shoulder falloff to ordinary card 1's bottom. It fails even that
training strip (RGB RMS 21.69→24.33), both bottom holdouts (21.79/21.82
→24.23/24.13), the selected bottom (38.70→43.55), and every top window.
Only card 4's side improves slightly. This sampled profile model is not
an SVG implementation and does not warrant one. No frame art changes.
The next fit still needs separate individual ridge peaks, gaps, areas
and positional controls; do not repeat a single opacity or this failed
limited-shoulder model as a correction.

## BO: current-production profile confirmation

The BM production capture retains six ridges and the accepted lower-turn
direction on ordinary card 1, the selected card and ordinary 3/4 holdouts.
Astra independently reproduces twelve fixed RGB profiles and four fifth-
bottom probes. Ordinary fifth-bottom contrast is about 36 in the photo
versus 19 native, while its five-row area is 58–59 versus 49. Selected
contrast is 66 versus 19 and area 84 versus 45. Peak and integrated-area
deficits therefore cannot share one opacity correction. Selected side
contrast is weaker than ordinary despite the stronger selected bottom.
The rule crossing also mixes the narrow core with a gradual photographic
floor; a raw peak there is not an isolated stroke measurement.

Ordinary bottom positions remain within one source pixel; selected middle
echoes remain 1–3 pixels high. The innermost vertical peak disappears at
the source-supported turn in both images. No new material recipe follows
from these profiles, and AA/AF/AJ/AU trials remain rejected. A separate
source-owned map of selected side, turn and bottom positions is still
actionable; this audit does not establish that all frame work is blocked.
See [round forty-three](../reference-svg-round43.md) and scratch evidence
in `/tmp/cp-eras-next/bo-neokitsch-frame/`.


## BU selected third-bottom SVG trial

The native third bottom peaks three source pixels high at 4K and one at
both smaller sizes, while its straight side and turn position agree. A
single +1.2 design-y SVG trial improves bottom RGB but overshoots source
at 4K/1600; the SVG's 1600 baseline already matches y702, unlike native 701.
The 1600 lower bend and fractional turn worsen, and the existing `#lines`
halo spreads changes beyond the frozen corridor. Astra independently
reproduces 42 regional metric sets and 20 local losses. The trial is rejected;
its native counterpart is tested separately in BV below. No material
or production change follows. See [round forty-three](../reference-svg-round43.md).

## BV selected third-bottom native trial

Fresh actual-Store baselines exactly reproduce the verified production
frames at 3840×2160, 1600×900 and 1537×947. Lowering only the third
selected echo's lower quadratic control/end and flat bottom by 1.2 design
pixels changes 2905/725/468 pixels, all inside the frozen stroke envelope.
Alpha, its straight side, other five selected ridges and ordinary controls
remain exact. Astra independently reproduces 78 regional metric sets and
234 threshold comparisons.

Bottom RGB improves at all sizes, but the source/baseline/trial third peaks
are 1687/1684/1687, 702/701/703 and 675/674/675. The 1600 trial overshoots.
Its local red area also falls from 144 to 139 against source 155.6; fractional
area falls 103→90 against 108.7. These are three-sample areas at the smaller
sizes, versus five samples at 4K. Fractional turn RGB RMS worsens
24.926→25.754 and R80 F1 .697674→.666667. Two other threshold losses remain.
Reject this exact offset despite the straight-bottom gain. Continuous ridge
position and turn shape need separate review; argmax alone is insufficient.
See [round forty-four](../reference-svg-round44.md) and scratch evidence
`/tmp/cp-eras-next/bv-neokitsch-frame-review/root-review.json`.

BY's continuous profiles then separate an already aligned right bend from
the too-high straight span. BZ preserves the old quadratic and inserts a
short cubic into a .9-design-pixel-lower flat. Three-size actual-Store
baselines match production; side/turn, ordinary cards, other five ridges,
alpha and frozen locality remain exact. Both smaller-size peaks align,
but fractional area becomes 139 versus source 108.722 and baseline 103;
half-prominence width grows 1→1.582 against source 1.030. The exact BZ
contour is rejected despite RGB and bend gains. Astra reproduces regional,
threshold, third-area and x-station evidence; see
[round forty-four](../reference-svg-round44.md#by-frame-mechanism-and-bz-native-trial).

## CC selected third native flat correction

CA's coverage analysis predicts one +1.0-design-pixel flat displacement
after the unchanged quadratic bend. CB confirms source peaks at all three
sizes, improves 4K local area 304→283 against 281.548 and preserves smaller
areas and widths. The side, turn, other five ridges and ordinary frames
remain exact. The tiny lower-frame R120 threshold loss and core/halo
material differences remain documented. Ten CC state/control pairs pass,
including first/last selection, actual feedback materials, custom palette
and a visible partial opening. CC integrates only this native path. Three
production sizes and packaged 4K match exactly; both Store gates, all 294
local/Nix Rust tests, 24 SVG checks, 22 repository checks and 27 first-attempt
visual cases pass. Only the Store golden changes, by 479 lower-ridge pixels.

SVG's separate +.8 trial aligns position but worsens 1600 area and
fractional width. It is rejected. The trace/component stay unchanged;
their different raster phase and filtered halo still need source review.
See [the measured result](../reference-svg-round44.md#cacb-frame-coverage-and-native-candidate)
and `/tmp/cp-eras-next/cc-validation-plan/` for integrated verification.
