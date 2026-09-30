# Entropism hub printing fit

`images/entropism-store.png` is the dashboard source, despite its name; see
`docs/sources.md`. This pass compares it at 3840 × 2160 with the final frame of
`dashboard-trace.svg` and the Iced dashboard capture at the same size. The
existing geometry, selected BRAINDANCE tile, panel heading, badges, and
animation remain as traced. Only the idle tile and detail-panel printing
described here changes.

Clean upper-quartile samples from source letter interiors separated the three
green inks that the previous trace had grouped with brighter neighboring
features:

| Run | Source RGB | Prior Iced RGB | Local fitted ink |
| --- | --- | --- | --- |
| Idle caption text | 162, 207, 167 | 168, 215, 167 | `#a2cfa7` |
| Idle tile labels | 162, 210, 168 | 172, 221, 180 | `#a2d2a8` |
| Detail body | 173, 210, 170 | 159, 192, 156 | `#add2aa` |

The caption rules retain `#a8d7a7`, so the glyphs and rules have separate
colors. The panel heading and badge glyphs retain `#acddb4`, and selected
caption and label ink stay dark. These are local fixed inks, following the
existing dashboard trace; they are not palette-role changes.

The SVG caption `<use>` also passes a stroke to its definition. Without an
explicit override, its 1.7px rule stroke paints over each letter and closes
the counters. The three caption `<text>` children therefore set
`stroke="none"`; the path still inherits the rule stroke. At native size,
the text-only crop joins x 315..524 and 549..762 over y 999..1047, excluding
both rules. With a green>125 mask, source/old SVG/fitted SVG/Iced J text areas
are 3,966/5,649/3,604/3,601 pixels. Source-to-SVG IoU rises 0.558 to
0.569; at green>140 it rises 0.514 to 0.542. Rajdhani 700 in the SVG brings
the stroke-free SVG's density close to Iced's Rajdhani 600 rasterization,
which already had no extra outline. The source still has photographic glow
and different letter edges, so this fit is about legible counters and local
ink coverage, not pixel equality.

The body also needed a small geometric fit. The first source line's clean
green pixels end near native x 3000; the old SVG run extended to x 3014.
Rajdhani Medium at 17 design pixels, positioned at x 1032.2 rather than
1033, shifted down 1.25 design pixels, and stretched 1.135 rather than
1.16 brings its glyph positions closer. Against a green>150 mask in native
pixels, first-line source/SVG intersection-over-union rises from 0.084 to
0.429, and the ten-line panel region rises from 0.099 to 0.257. The full
panel mask is still affected by the source photograph's softened glyph edges
and local shape differences. A 600-weight trial overfilled the first line:
4,141 SVG pixels versus 2,580 clean source pixels, and gave a lower
first-line IoU of 0.389, so the trace keeps Medium. These metrics establish
the direction and size of the fit, not pixel parity with the photograph.

The matching Iced runs live in `src/eras/entropism.rs` under `HUB_TILE_LABEL`,
`HUB_CAPTION_TEXT`, and `HUB_BODY`. `docs/entropism/components.svg` carries
the same tile and panel excerpts at translated coordinates.

The J native Iced capture placed the ten body lines about two native pixels
below the fitted SVG and source. For the first three lines, the clean source
rows are y 752–772, 802–822, and 853–873; SVG rows are 753–772, 803–823,
and 854–873, while Iced rows are 754–775, 805–825, and 855–875. Their Rust
baselines move up 0.85 design pixels. The source-fitted SVG baselines remain
unchanged. This is a renderer calibration of the panel copy, not a different
source layout.

## Integrated validation — 2026-09-29

L/N native review accepts the ten-line baseline correction. SVG/native text-mask IoU improves .509→.801; all copy remains complete and unclipped. The integrated N repository check passes, including the refreshed dashboard golden at 100.000%. Exact source glyph contours/glow remain separate.
