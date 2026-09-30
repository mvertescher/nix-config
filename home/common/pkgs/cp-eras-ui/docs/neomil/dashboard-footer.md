# Dashboard footer type and printing (NM1)

Source: `images/img-07-dashboard.png`, screen #60, 3840×2160. The affected
design crop is `(1195,862)..(1372,899)` on the 1600×900 canvas. The trace,
component excerpt, and Neomil scene table use the same literal code,
captions, two-cell frame, and divider. The native-only code calibration below
changes its three scene runs; no shared text or scene renderer was changed.

The former Rajdhani Bold 8 code and Semibold 7.5 captions rendered too
short. In the SVG, each run has its own baseline and horizontal fit: Bold 9.8 at
`0.844×` for `68SD1D1100D1S`, Semibold 9.5 at `0.79×` for `COMBAT
COLONIZATION`, and Semibold 9.5 at `0.807×` for `DEFENCE PROGRAM`. The
horizontal fits preserve the narrow source lettering while raising its cap
height. A size-only change would have made the already correct first caption
too wide.

Native source and SVG render use the same `R > 170, R > 2G` red-core mask.
Coordinates below are in design pixels after dividing native pixel positions
by 2.4. These are thresholded ink boxes, so the exact equality does not
establish identical glyph contours or photographic presentation.

| Run | Source box `(left,top,right,bottom)` | Revised SVG box | Source / SVG cap height |
| --- | --- | --- | ---: |
| Code | `(1214.58,868.75,1265.00,875.00)` | same | `6.25 / 6.25` |
| First caption | `(1276.67,868.75,1345.83,874.58)` | same | `5.83 / 5.83` |
| Second caption | `(1277.08,877.08,1335.42,882.92)` | `(1277.08,877.08,1335.83,882.92)` | `5.83 / 5.83` |

The old frame copy was a single sharp, dark 1px rectangle shifted three
pixels. In the source, dim frame ink spans several rows under both the top
and bottom edges and several columns beyond the right edge. Two broader
red copies with small offsets and `0.25/0.13` opacity fit those regions;
two local text copies at `(+0.8,+1)` and `(+1.8,+2)` with `0.23/0.11`
opacity fit the visible low-intensity lettering around the red cores.
These are translucent local canvas copies under the primary marks. They use
the dashboard foreground role in the app, so custom palettes still recolor
the whole footer together. They describe observed ink, not the source
renderer. No glow, grain, or global noise was inferred.

The table reports mean absolute red-channel error in separate top, bottom,
right-edge, and text crops after resizing the source with Lanczos to
1600×900. The numbers compare complete crops,
including local background and antialiasing.

| Crop | Before | Revised |
| --- | ---: | ---: |
| Top frame, left / right | 43.9 / 45.0 | 26.8 / 27.7 |
| Bottom frame, left / right | 56.9 / 58.1 | 35.1 / 36.2 |
| Right frame edge | 37.1 | 26.9 |
| Code | 53.1 | 33.5 |
| First / second caption | 69.6 / 56.1 | 32.6 / 27.1 |

The final native 3840×2160 Iced capture shows the
same direction of improvement. In source / before-Iced / revised-Iced
order, red-core cap heights are `6.25 / 5.42 / 6.67` for the code,
`5.83 / 4.58 / 6.25` for the first caption, and `5.83 / 4.58 / 6.67`
for the second. The canvas rasterizer therefore paints the fitted runs
0.42–0.84 design px taller than SVG; the source-to-Iced match is improved,
but remains less exact than source-to-SVG. Native red-channel crop error
improves for code `55.7→49.0`, first caption `90.9→48.7`, second caption
`71.8→42.3`, and every separately sampled frame edge.

At 1600×900, the changed SVG pixels are confined to
`(1210,865)..(1358,893)`. Before/after Iced capture changes are likewise
local: native `(2904,2076)..(3259,2143)` and fractional 1537×947
`(1162,831)..(1305,858)` in output pixels. The source image's compression, downscaling,
and unknown printing process leave the exact edge profiles uncertain.
The source also shows fine horizontal striations and softer repeated ink
that these two broad copies do not reproduce. The local copies are a
measured approximation; exact faint printing and live desktop behavior
remain open for the orchestrator's final review.

## Native code cap-height calibration (2026-09-29)

At 3840×2160, the first source `6` has a bright-core box of
`x=2915..2923, y=2085..2099`. The restored native capture painted the same
horizontal box at `y=2086..2101`, even though the SVG box matched the source.
Four native previews changed only the primary code run and its two local
copies. The unmodified control was pixel-identical to the restored capture.
The chosen Bold 9.2 preview moved all three baselines up by `0.833333` design
px and used the computed horizontal stretch `0.844 × 9.8 / 9.2` to retain
their nominal width. Rust now uses these exact expressions; the trace and
component SVGs retain the source-aligned Bold 9.8 fit.

| Independent native region | Restored / calibrated bright-core IoU | Restored / calibrated RGB MAE |
| --- | ---: | ---: |
| First `6` | `0.500 / 0.702` | `25.017 / 15.694` |
| Held-out `8` | `0.475 / 0.680` | `23.501 / 18.109` |
| Remaining code | `0.416 / 0.544` | `29.064 / 24.191` |
| Whole code | `0.428 / 0.569` | `28.237 / 22.956` |

The calibrated first `6` and held-out `8` match the source's thresholded
vertical boxes `y=2085..2099`. The trial changes 1,749 pixels, confined to
`x=2915..3040, y=2085..2106`; the frame, divider, captions, and all pixels
outside the code region remain identical. The later glyphs still have
horizontal phase and contour differences, particularly the last `S` at
`x=3025..3036` versus source `3026..3035`. Semibold improved that later
region further, but worsened the first `6` and left a one-pixel bottom
overshoot. This is a bounded native code correction, not a claim of exact
photographic printing fidelity. The Z production capture matches the
approved trial at every pixel. Fractional rest, custom, opening and held
comparisons change only the code lettering; frame/caption controls are
unchanged, and the no-config fallback matches the reference dashboard.
Both fidelity gates pass. Integrated verification is recorded in the
[eleventh checkpoint](../reference-svg-round11.md).
