# Maker microtext copies and edge fit

Source: `images/img-07-dashboard.png`, 3840×2160. The existing geometric
PRECISION LIQUID and POLYMER MUSCLE paths preserve the visible monospace
letter shapes. This correction retains every primary path and adds local
copies around those two runs only. It does not infer a screen-wide glow,
noise process, or the source author's rendering recipe.

The measured region is design `(1217,730)..(1289,751)`. Each line is fitted
independently. Four copies at ±0.416667 design pixels in x or y reproduce
one native pixel of soft edge coverage. A separate displaced, softened
copy describes the dim printing below and to the right. That copy is a
5×5 finite kernel with separable binomial weights `[1,4,6,4,1]/16`.
Each shifted run is composited with group opacity, followed by the unchanged
opaque primary, so letter junctions do not accumulate opacity per stroke.

| Run | Horizontal edge opacity | Vertical edge opacity | Tail strength | Tail offset x/y | Kernel step x/y |
| --- | ---: | ---: | ---: | --- | --- |
| PRECISION LIQUID | .181299 | .296318 | .581786 | 2.430380 / 1.453668 | 0.49299 / 0.50687 |
| POLYMER MUSCLE | .184115 | .324727 | .660547 | 3.027105 / 1.437290 | 0.66595 / 0.47138 |

Offsets and kernel steps are design pixels. Tail strength multiplies each
kernel weight; it is not the opacity of an opaque rectangular field. All
copies keep the existing `#f93333` primary ink. The Rust paths stay in their
existing bounded software layer and GO HOME opening clip; no new image
layer, blur primitive, font, or bitmap is introduced.

For validation, fit on one word and measure the other, then reverse the
roles. The source comparison mask is the primary alpha above .01 dilated
by ten native pixels, separated at x1260 / x1255 for the two lines. The
first line uses crop-relative rows below 25 and the second rows above 22.
These masks include the trailing ink and edge pixels, rather than only the
bright primary cores. Full SVG renders with each independently fitted
parameter set give the following held-out RGB RMS errors:

| Trained word → held-out word | Before | Corrected SVG |
| --- | ---: | ---: |
| PRECISION → LIQUID | 20.36 | 11.46 |
| LIQUID → PRECISION | 22.46 | 12.19 |
| POLYMER → MUSCLE | 23.45 | 13.43 |
| MUSCLE → POLYMER | 23.14 | 13.00 |

Every red, green and blue channel improves in all four holdouts. The final
all-word fits have RMS 21.65→11.85 and 23.28→13.20. A single sharp displaced
copy was rejected on visual review: it left recognizable hard duplicate
letters where the source has softer faint printing. Denser finite sampling
removes that artifact without changing the primary geometry.

The source still contains horizontal striations and pixel variation that
this local approximation does not claim to reconstruct. Native and
fractional Iced captures, opening review and final repository checks are
required before accepting the runtime change.
