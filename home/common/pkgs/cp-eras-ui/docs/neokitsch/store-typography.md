# NK-05: store mark and card type

The source is `images/neokitsch-store.png` at 3840×2160. Measurements below
use the 1600×900 design frame unless marked native. Bright core masks keep
pixels with `R>180, G>125, R-G>25` inside each word's crop; the photographed
glow and the selected card's veneer are excluded from those comparisons.
The prior runtime reference is the L native store capture. Native runtime
verification of this correction remains pending.

The source's solid 4 and S are the same artwork as the Entropism 4ST mark,
translated by `(-68,-84)` native pixels for S and `(-68,-85)` for 4.
Independent source core masks overlap at IoU 0.968/0.975 respectively,
which supports reusing those accepted contours at design offset
`(-28.333,-35)`. This replaces the stretched Rajdhani letters in the SVG,
components, and runtime. The outlined T follows the NeoKitsch source's own
bar and stem: the source's native core covers x569..664 on the bar,
y164..186, and x603..630 on the stem. The former runtime bar extended to
about y190 and its stem to x598..634. The new outline is
`x237.5..276.2, y68.5..115` with a 2px stroke. Against the source core,
SVG IoU moves from 0.864 to 0.890 for 4, 0.864 to 0.877 for S, and
0.411 to 0.741 for T. These masks check geometry, not photographic color
or glow.

Both plain and selected cards put DPS/PNT/ACC/ROF about 11.25 design
pixels right and 2.5 pixels up from the previous SVG. In the plain
card, source DPS core spans x395.42..421.25/y471.67..482.92; the new
SVG spans x395.42..421.67 at the same y. Its bright core has 618 native
pixels against the source's 631, versus 342 before. The other three labels
show the same shift. The selected source has the same offset relative to
its card. Labels are now Rajdhani 18px weight 500, placed +11/-1.7 from
the previous anchors.

The values use +11.5/-1.5 anchors. `620` is 28px weight 600, with 1928
native core pixels against 1957 in the source. `30` and `5` are 22.5px
weight 600; the first `30` has 851 pixels against the source's 793,
versus 370 before. Plain and selected cards share the same local positions
and weights; selected values retain their dark veneer ink.

EMPTY/SOCKET uses 13px weight 500 on both card states. EMPTY moves down
2px, while SOCKET keeps its baseline. The first pair's centre moves left
2px; the other two centres retain their positions. On the first plain
cell, source EMPTY spans x427.5..459.17/y547.08..555 and has 463 core
pixels; the new SVG spans x427.92..460.42 at the same y with 443 pixels,
versus 177 before. The source's grain under selected socket text still
limits a simple dark-pixel score there, so its native visual crop is the
check for that state.

Native source, previous SVG, fitted SVG and L Iced comparisons are in
`/tmp/nk-type-tuned-{logo,plain-stat,plain-socket,selected-stat,selected-socket}.png`.
The established card contours, selected body and QR scatter, hit regions,
and state feedback remain separate. The first native runtime review found
the stat labels, `30`/`5`, and SOCKET two native pixels below their SVG
positions; `620` was three below SVG and two below the source. Rust lifts
those runs by `0.833333` design pixels (two native pixels). EMPTY's
one-pixel difference stays within raster quantization, so it keeps its
anchor. The SVG vertical coordinates remain at their measured source
positions. Review the
next native capture before closing the fit. ACC alone shifts right by
`1.25` design pixels in the runtime, trace and component specimens.
Before that change, its plain source/native warm-core bounds were
x1226..1291/x1222..1288; selected bounds were
x1961..2025/x1958..2023. A three-native-pixel shift targets
x1225..1291 plain and x1961..2026 selected, within one native pixel of
each source endpoint. This is a predicted raster result pending the next
runtime capture.

## P native verification — 2026-09-29

P native review accepts the renderer-only baseline lifts and the ACC shift, including selected socket type and fourth-card held state. Primary 4ST contours match the fitted SVG; photographic texture and glow remain distinct. P passes 269 Rust tests and all eight affected fidelity gates across four screens. Integrated repository validation remains pending for Q, which follows this snapshot.
