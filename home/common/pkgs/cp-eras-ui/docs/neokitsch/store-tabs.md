# NK-09: store tabs and shoulders

Source: `images/neokitsch-store.png` (3840×2160), compared with `store-trace.svg` rendered at the same size. Coordinates below are in the trace's 1600×900 system unless labelled native; multiply by 2.4 for the source. Native crops and the before/after render are in the local audit evidence directory.

| Feature | Native reading | Trace after correction |
| --- | --- | --- |
| RIFLES nav tab | Narrow upper run x≈582..659 native (242.5..274.6), broadened foot x≈577..665 (240.4..277.1); top y≈939 (391.3), foot joins button's y≈951 (396.3). | Top x243..274, base x240.5..277, y391.4..396.5; two cubic shoulder joins. Repeated at 60.7 pitch for all five nav cells. |
| Plain card foot | Upper run x≈1112..1251 native, base x≈1104..1259; source tab reaches the card rule at y≈1537. | Top x0+102..160 (58), base x0+99..163.5 (64.5), y632.9..641.4; repeated on cards 1, 3 and 4. |
| Expanded card foot | Same profile shifted with the selected card; native bright run x≈1844..1986 at y1694 and broader at its foot. | Same 58/64.5 profile at y703.8..712.3; expansion and body fade remain unchanged. |
| Card upper shoulder | The outer run begins flat near x0+136, curves through the rise and levels near x0+182; six visible strokes total: outer plus five inner echoes. At native x1320, the bright stroke is y756 and the inner lines are around y764, 771, 779, 787 and 794. | Cubic with horizontal tangents at both joins, x0+136.2..182.2; five curved echoes; their right corner turns preserve rounded arcs. Their high plateau pitch is 3.2 trace px (about 7.7 native px), fitted to the native spacing. |

The original SVG had nav tabs 33.8 wide on top and 27.8 at the base, starting 2.7px too low. Its product tabs were 64.5 wide on top and 58 at the base. Both used straight side joins, and the card shoulders were diagonal. The Rust scene now uses the same corrected paths in the plain and selected states. Plate extents, selection growth, labels, hit regions and the separate inverted mailbox selection tab were left in place.

For a source comparison, a simple warm-pixel mask (`R>175`, `G>115`, `B>65`, `R>1.14G`) on native RIFLES tab crop x576..665/y937..950 changes source/trace IoU from 0.350 to 0.955. The card-1 foot crop x1101..1261/y1518..1537 changes 0.931→0.953; the expanded foot x1838..1995/y1689..1708 changes 0.893→0.917. The outer shoulder crop x1190..1300/y750..830 changes 0.043→0.347. That shoulder score remains color and opacity sensitive: the photographic inner lines are brighter than the trace's fading echo strokes, while their count and 7–8px native spacing match. These masks are crop checks, not a whole-screen fidelity verdict.

A lower-threshold native scan at x1320/1360/1400 exposes the fifth echo that the initial warm mask missed: red maxima at y756, 764, 771, 779, 787 and 794. At x1360 the final line is approximately RGB 76/58/37, so its added stroke is deliberately faint. The fifth line follows the same curved shoulder and top-right turn, then continues down the side with a small rounded foot. The store trace, component excerpt, and plain/selected Rust paths all carry it. The source-to-SVG corner crop is saved as `fifth-corner-montage.png` in the local audit evidence.

Native review also corrected the echo junction. Separate starting points
produced hooked steps; the photograph's five echoes emerge from a common
low tangent near local (136.2,345). Six native column profiles at
x1260–1310 resolve the five ridges. Fitting their cubic controls with
horizontal end tangents gives mean centerline residual 0.104 design pixels,
maximum 0.369px at those samples. The selected form uses the same x
coordinates, shifted up 81.7px. All trace copies, both component excerpts
and Rust use the fitted fan. The outer contour paints after the echoes so
their darker ink does not obscure the shared bright foot.

The corrected taper, shoulder junction, contour count and high-edge
spacing preserve the common-foot fan. The final warm-mask IoU values above
exclude the faintest stroke; native ridge profiles and final render review
assess the complete fan. Frame echo ink was subsequently fitted to the
photographed selected-card plateau (see `README.md`); the earlier statement
that the echoes were too dark is stale.

## NK-14: inner right turns

The source's three innermost echoes bend before the prior trace and reach
their vertical runs lower. Source scans of the selected card at native
x=2170/2180/2190 have the inner ridge y triples
`[583,590,600] / [584,593,605] / [587,599,615]`. The previous SVG has
`[582,590,598] / [582,590,598] / [583,590,598]`; the fitted candidate
has `[582,590,600] / [583,593,605] / [586,599,613]`. The plain card
repeats this shape against its darker background. At native
x=1435/1445/1455 its source y triples are
`[779,787,795] / [780,789,800] / [783,795,812]`, within one native
pixel of the selected readings translated to the plain card. The updated
SVG reads `[779,786,796] / [779,789,801] / [783,795,810]` there.

The fit replaces only echoes 3–5's inner right turns with cubic curves.
Their plateau heights, common shoulder root, and bottom foot endpoints
retain their former positions. The outer contour and first two echoes,
six-stroke count, card plate and hit geometry, and selected-body clip are
unchanged. The same local coordinates are used in `store-trace.svg`, both
`components.svg` card specimens, and `src/eras/neokitsch.rs`. Source,
previous SVG, candidate SVG and existing Iced corner crops are at
`/tmp/nk-right-{plain,selected}-candidate-montage.png`. Native
runtime verification remains to be done after the next build.
