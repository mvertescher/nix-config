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
