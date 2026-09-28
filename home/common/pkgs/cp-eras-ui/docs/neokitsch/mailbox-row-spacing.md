# Mailbox row and envelope placement — 2026-09-27

Source: `images/neokitsch-mail.png` (#71, 3840×2160); design coordinates
divide native pixels by 2.4. The comparison uses
`mailbox-trace.svg` rendered directly at 3840×2160 by librsvg with the
repository fontconfig file. Both the old and corrected traces were
rendered natively for the local registration below.
The original is a photographed image with glow and uneven grain, so
subpixel fits are estimates; a native source pixel is 0.417 design px.

The earlier NK-13 premise conflated two motions. The **rule and text
rows remain on a 60.2px pitch**. In a clean rule segment x40–165,
source rule centers for rows 1/3/4/5/6/7 are approximately
309.2/429.6/490.0/550.0/610.4/670.6; the old trace centers are
309.0/429.4/489.6/549.8/610.0/670.2. Thresholded title and sender
bounds likewise stay on that pitch, with font-dependent vertical edge
differences rather than a growing downward displacement. At x40–80,
the selected veneer spans about y315–370 in the source, agreeing with
the trace's [35,315]–[512,370] bar. Moving all rows to 60.8px would
move these supported landmarks away from the original.

The **envelope glyphs** have separate placement. Comparing local
chromatic, high-pass patches at x426–448 and each glyph's y interval
finds the source-to-old-SVG vertical displacements below. The fit
searches shifts at the native 1px interval and maximizes normalized
patch correlation; row 2 is less reliable because its glyph sits over
wood grain. Rounded offsets were applied to the trace and the era's
per-row `glyph_offsets` table. The runtime still chooses the open
glyph on rows 1/3/7 and closed glyph on rows 2/4/5/6, including the
selected row's inverted ink.

| Row | Glyph | Old trace y | Source shift | New trace y | Residual shift |
| --- | --- | ---: | ---: | ---: | ---: |
| 1 | open | 271.7 | +0.42 | 271.7 | +0.42 |
| 2 | closed, selected | 335.4 | +0.42 | 335.4 | +0.42 |
| 3 | open | 392.1 | +1.67 | 393.6 | +0.42 |
| 4 | closed | 455.8 | +1.67 | 457.3 | +0.42 |
| 5 | closed | 516.0 | +2.50 | 518.3 | +0.42 |
| 6 | closed | 576.2 | +3.33 | 579.4 | 0.00 |
| 7 | open | 632.9 | +4.58 | 637.3 | 0.00 |

The strongest diagnostic rows (1 and 3–7) correlate at 0.73–0.75
after the fit; row 2 correlates at 0.38 amid the selected grain. Native
rasterization moves the best shift by roughly one source pixel compared
with enlarging a 1600px render, so the table should be read at its
0.42px source sampling precision. This
measures placement of the existing traced shapes, not glyph form or
the whole-screen inventory. The corresponding original and fresh SVG
crops and the numeric scratch scripts are in
`/tmp/cp-eras-round3/nk-rows/`. Iced placement, feedback and opening
states require a fresh binary and headless render; they are separate
from this source-to-SVG measurement.
