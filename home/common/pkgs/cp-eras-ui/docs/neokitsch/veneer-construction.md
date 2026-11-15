# NK-07: how the reference veneer is constructed (2026-11-14)

The owner rejected both noise-field generators (`veneer-texture.md`) and
asked for "the exact splines used and how they replicated". This note covers
what the 4K references show, how it was measured, what
`scripts/make-veneer-blend.py` rebuilds from it, and what is still unknown.
No reference pixels are kept. The measurement scripts and evidence images
are in the pass's scratch. The numbers in the generator are fitted
coordinates (texels = native 4K px of each surface's box, 2.4 per design px).

## Method

All six surfaces were cropped at native 3840x2160 using the boxes in
`veneer-texture.md`. Centrelines were traced on the dark-line channel
(255 - (G+B)/2): seeds come from scanline minima, and a ridge follower
(Hessian tangent, sub-pixel re-centring along the normal) traces each line,
de-duplicated by occupancy. That gave 1035 / 1120 / 423 / 438 / 123 / 198
lines on store card / panel / EMAIL / mailbox / SMG / BASKET. The tracer
stops where lines converge below the pixel pitch (streak cores, seams) and
at text.

## What the data supports

1. **Lines are straight segments joined by short knees, not smooth noise.**
   Away from knees, traced runs deviate from a straight line by 0.01-0.05 px
   rms over 25-110 px on every surface. Knees turn 25-35 degrees (median)
   with a radius of about 6-15 native px (median 8-11, slightly inflated by
   the 1.5 px smoothing). The knee radius is about the same on all six
   surfaces.
2. **Spacing is constant within a family.** On the store card at y=150,
   x=250-500, 40 consecutive gaps are 5.8 +- 0.3 native px. The previous
   pass measured a coefficient of variation of 0.27-0.58 for spacing; that
   is not noise. It comes from two regular combs interleaving: adjacent gaps
   alternate (panel 4.3/5.6, EMAIL 5.5/3.0, SMG 3.0/1.9, mailbox 4.2/1.5)
   while each pair sums to a constant (panel ~10.2, EMAIL ~8.9, SMG ~4.9,
   mailbox ~5.7). The BASKET shows one comb at ~4.4.
3. **Knee loci are straight lines, and crossings along them are equally
   spaced.** RANSAC on the knee points finds straight "fold" lines, for
   example 303 px long with 35 knees at rms 0.75 px, or 257 px with 38 knees
   at rms 0.76 px. The gaps along a fold are 1.0x the median, with 2.0x
   where one knee was missed. This is the signature of a **blend between
   polylines**: the corresponding vertices of the intermediate lines are
   linear interpolations, so they lie on a straight segment at equal steps.
   It is not a parallel offset. On the store card, the fold at -9 degrees is
   not the -28.5 degree bisector of its corner, and the line pitch changes
   between panels.
4. **Folds kink where one blend ends and the next begins.** The knee loci
   are polylines, not single lines. This is consistent with a multi-key
   blend: key polylines C_0..C_K with matched vertex counts, and n_i steps
   between C_i and C_i+1. On the store card, a two-key blend with 5 vertices
   per key fits 4,850 traced points in the central region (x 190-520,
   y 95-290) at **rms 0.44 px**. That is the tracer's noise floor, using 10
   control points plus one step count.
5. **A twin family.** Above the store streak, adjacent lines' vertical runs
   end alternately at y = 268.5 and 263.7. This was measured from raw
   intensity, not the tracer, and holds for 30 consecutive pairs (+-+-).
   Their upper fold is shared: 138.7 vs 140.2 at x=300, both with slope
   -0.16. The lower knee rows differ: 266.8 vs 260.8, slopes 0.031 vs 0.043.
   So the second comb is **another blend that shares the upper vertices and
   has its own lower knee**, not a pure translate. Fitting a pure translate
   gives rms 0.44-0.53 for (5.8, 0-6); a single blend with 52 steps gives
   0.37 (it fits only the runs and cannot alternate the knees); two
   independent sheets give 0.43.
6. **Book-match mirrors.** For each candidate axis, the test counts the
   share of knee points whose reflection lands within 1.5 px of another
   knee, against the median over all axes:

   | Surface | Axis (crop px) | Knees with a partner | Chance |
   | --- | --- | --- | --- |
   | panel | y = 370.5 (its exact middle) | 51% | 0% |
   | SMG | x = 239 | 23% | 0% |
   | mailbox | x = 571.5 | 23% | 0% |
   | store card | y = 291.5 (the streak) | 17% | 1.1% |
   | EMAIL | y ~ 344-370 | 8% | 1.3% |
   | BASKET | none found | 9% at x=87, matched by 8% at x=310 | |

   On the SMG, intensity correlation of line detail peaks at 0.90 at x=239
   and falls to 0.73 at +-1 px. On the store card, the lower diagonal fold
   reflected about y=291.5 lands on the upper diagonal fold to within 1.6 px.
   The leaves extend past the axis: where they bend in opposite directions,
   they cross, and that crossing is the lattice in the store streak and the
   EMAIL moire. The "dark core" is many converging strokes compounding (a
   blend row whose two endpoints coincide, so all its lines meet at one
   point).

## The construction, as rebuilt

A surface is a list of **sheets**. Each sheet is a set of keys (K+1
polylines x V vertices), steps n_i, and a fillet radius per vertex row.
Each sheet is drawn with a list of **copies** (affine transforms: identity,
plus the mirror about the book-match axis). Line k between keys i and i+1
is the polyline `lerp(key_i[v], key_i+1[v], k/n_i)` over v, with each
corner rounded. Strokes are Gaussian (sigma 0.55 texel, FWHM ~1.3; measured
FWHM 1.2-1.9), ink 0.5 optical depth, summed and softly saturated (1.5).
They are coloured with `make-veneer.py`'s measured palettes, on its light
ground colour.

| Surface | Sheets x copies | Lines | Provenance |
| --- | --- | --- | --- |
| store card | 4-key middle sheet + twin, top-band sheet + twin; x {id, mirror y=291.5} | 198 | middle **fitted** (ICP; rms 0.44 centre, 0.80 whole upper half); top band fitted on 550 points only, approximate |
| panel | left sheet + twin, 3-key fold sheet + twin; x {id, mirror y=370.5} | 112 | fold sheet ICP-initialised, then edited by hand; left sheet hand-set |
| EMAIL | 3-key sheet + twin (offset 3.4); x {id, mirror y=362} | 48 | **hand-set** from the trace; ICP did not converge with crossing leaves (rms 0.87 at best) |
| SMG | 2-key half sheet + twin (offset 2.0); x {id, mirror x=239} | 40 | ICP rms 0.39 on only 53% of points, then seam rows set by hand |
| mailbox | 2-key half sheet + twin (offset 1.5); x {id, mirror x=571.5} | 46 | hand-set, rough |
| BASKET | left leaf + right leaf meeting at x=205 (not a mirror) | 50 | hand-set, rough |

`scripts/make-veneer-blend.py` (numpy only, reads no image) imports
`make-veneer.py` for `SURFACES` (sizes, palettes, seeds), `ramp` and
`png_indexed`. It writes the same six 8-bit indexed PNGs at the same sizes,
so it is a drop-in for `docs/neokitsch/veneer/`. Output is deterministic
(two runs byte-identical) and takes ~9 s for all six.

## Integrating (not done in this pass)

1. Review the textures against the references (the pass's evidence has
   side-by-sides and overlays). Then
   `nix-shell shell.nix --run 'python3 scripts/make-veneer-blend.py docs/neokitsch/veneer'`.
2. Point `default.nix`'s `passthru.tests.veneer` at `make-veneer-blend.py`.
   It also needs `make-veneer.py` beside it in the sandbox, because it
   imports helpers from it.
3. `src/veneer.rs` is unchanged: same files, format and sizes.

A vector path is possible too, since a sheet is ~20-60 numbers. iced could
stroke the filleted polylines directly (each surface has 40-200 lines of
5-6 vertices, clipped to the surface's shape), with ink compounding done by
alpha. That would scale cleanly and drop the PNGs. The texture route was
kept here because owner decision 3 chose textures.

## Misses and open questions

- Store card: the upper streak is drawn too heavy and too straight, and the
  top-band zigzags (stair-step polylines with ~30-40 px period) are not
  modelled. The left strip under the stat text is extrapolated.
- Panel: the fold's crease and bottom-left are approximate. The reference's
  top half (under the text) reads quieter than the mirrored model, even
  though 51% of knees match under the mirror.
- EMAIL: the top-left swirl is not modelled, and the axis is only loosely
  constrained (355-370 within 0.04 px rms).
- SMG / mailbox / BASKET: the end curves are radial fans, while the
  reference's are closer to nested arcs (perhaps a third key, or offset
  paths of the outline). The mailbox's S-wave and the BASKET's right-end
  rise are missing.
- Whether all surfaces come from one master sheet was **not tested**:
  per-sheet pitches differ (11.6, ~10.2, ~8.9, ~4.9, ~5.7, ~4.4 native px),
  which argues against a single sheet at one scale. Each surface looks
  separately drawn with the same technique.
- The twin's exact rule (why it shares the upper fold but not the lower
  knee) is unknown. It is fitted as an independent sheet.
- Tracer limits: cores and seams below the pixel pitch are untraced, so
  the convergence rows there come from extrapolation, not measurement.

## Line, ground and grain (third pass, 2026-11-14)

Measured on the 4K crops (store card unless stated), scripts in the pass's
scratch. These replace the Gaussian strokes, flat cream ground and 1-D ramp
palette of the second pass, which read as "flat cream with brown lines".

- **A line is a 1-texel box stroke, drawn over the ground.** Averaged over
  40 rows of near-vertical grain, a line centred on a pixel column drops
  that one column to G 138-151 (ground 202-209) and leaves its neighbours
  at 197-207; a line straddling two columns drops both to 170-185. Summed,
  the deficit is the same (~55-65 G levels x px) either way, so the stroke
  is about one texel wide with box anti-aliasing, not a soft 1.9 px
  Gaussian (the earlier FWHM figures measured the straddling cases).
- **Ink compounds towards a floor, not towards black.** One centred line:
  `(211, 144, 67)` on ground `(255*, 206, 131)` (`*` R clipped; 262 is
  used before clipping). The densest streak core reads `(148-159, 60-73,
  30-34)`. That is "over" compositing with a constant line colour
  `CORE = (150, 61, 30)` at per-channel opacity
  `alpha = (ground - single) / (ground - core)` = 0.46 / 0.43 / 0.63
  (R / G / B): n overlapping strokes leave `prod(1 - alpha*coverage)` of
  the ground, so a pile of lines darkens to the core colour and stops. The
  second pass's soft saturation of a Gaussian sum is replaced by this.
- **Core profile (the convergence to match).** Columns through the store
  streak, G below 80: 8 px tall at x 540, 3-4 px at x 600, 2-3 px at x 620
  (minimum G 60 throughout); around it a band of compounded lines at G
  80-140, ~25 px tall at x 300-500, which at 6x is a lattice: lines from
  above and below bend in with a ~10-15 px radius (not a sharp knee) and
  cross at shallow angles, leaving rows of light diamonds.
- **Ground tone is flat.** The ground (80th percentile in 9x9, blurred
  sigma 12, text masked) varies by +-2 G levels across the clear field of
  the store card (rows 130-240 and 340-450: 202.7-209.9). The visible
  lighter/darker banding is line density (convergences, bunching, the
  twin comb), so it comes from the geometry; no low-frequency tone field
  is added.
- **Grain.** Per-pixel noise on the ground, measured as the residual from
  the column mean in line-aligned windows: std R 3.4-4.4, G 5.2-5.7,
  B 6.2-6.9; the channels are nearly independent (correlation 0.03-0.23),
  lag-1 autocorrelation 0.53-0.59 and lag-2 0.01-0.14. Independent normal
  noise per channel through a separable `[1, 3, 1]` filter has lag-1 0.55
  and lag-2 0.09, so that is the model. It is chroma speckle, which a 1-D
  palette ramp cannot hold, so the PNG palette is now a deterministic
  256-colour k-means of the rendered RGB (still an 8-bit indexed PNG;
  `src/veneer.rs` only needs indices and a palette).

| Surface | Ground (R*, G, B) | One centred line | Grain std (R, G, B) |
| --- | --- | --- | --- |
| store card | 262, 206, 131 | 211, 144, 67 | 3.7, 5.3, 6.4 |
| panel | 262, 194, 110 | 217, 154, 76 | 4.1, 4.6, 4.9 |
| EMAIL | 262, 191, 108 | 199, 138, 65 | 3.7, 4.8, 5.2 |
| mailbox | 258, 202, 130 | 207, 146, 83 | 3.4, 5.0, 6.1 |
| SMG | 249, 198, 128 | 202, 140, 74 | 6.0, 5.2, 6.2 |
| BASKET | 249, 181, 107 | 202, 120, 79 | 6.0, 6.0, 6.0 (few clear px) |

Grounds are the median of clear pixels above their 20th percentile; single
lines the median of traced centre pixels in their darkest 30%; grain from
a sigma-2 high-pass on clear pixels, scaled by 0.86 (the store card's ratio
between that estimate and the line-aligned one). The core colour of the
other surfaces is the store core scaled by their ground (none has a core
dense enough to measure). Check: the panel's rendered rows 450-550 have
mean RGB (248.6, 185.9, 103.1) against the reference's (246.6, 184.8,
102.7), p10 and p90 within 2 levels per channel.

## Surface description (generator parameters)

`scripts/make-veneer-blend.py` takes, per surface:

- `LOOK[name]`: `ground` (R, G, B before clipping), `single` (one
  centred line's colour; with the shared `CORE` it fixes the per-channel
  opacity) and `grain` (per-channel std). `WIDTH` is the stroke width in
  texels (1.0).
- `GEOMETRY[name]`: `copies` (affine copies every sheet is drawn with:
  `['id']`, `['mirror_y', m]`, `['mirror_x', m]`, `['translate', dx, dy]`)
  and `sheets`. A sheet is `keys` (K+1 polylines with the same vertex
  count, texels of the surface box), `steps` (K counts: lines between
  consecutive keys, plus the last key), `radii` (fillet per vertex row,
  0 = sharp). Optional: `smooth` and `smooth_upto` (the first
  `smooth_upto` vertices are a smooth key part, drawn through `smooth`
  rounds of Chaikin corner cutting instead of fillets), `ink` (opacity
  multiplier), `copies` (overrides the surface's). A twin comb is a second
  sheet offset by part of a pitch.
- `render_rgb(name, geo=None)` returns the float image before palette
  quantisation, so a new surface can be previewed and compared without
  writing a PNG.

Authoring a new surface: trace the reference, fit keys (below), measure
`ground` / `single` / `grain` as in the previous section, add both entries.

## Fitting (third pass) and where it stands

Tools (pass scratch, `lib/`): the previous pass's polyline ICP
(`blend.fit_sheets`); `gfit.py`, a graph-sheet fitter for grain that runs
along one axis (lines are u(t) between knot rows, keys are knot values,
all sheets solve one sparse least squares; region-growing ICP from a seed
band, initialised from the traced crossings at a seed row so the comb's
phase is right); `sfit.py`, smooth keys (Chaikin) fitted by nonlinear
least squares of signed distance to traced points; and a darkness-map fit
for convergences the tracer cannot see (rendered vs reference optical
depth, blurred sigma 2, plus a traced-residual term).

| Surface | Fit | rms / matched | Adopted in GEOMETRY |
| --- | --- | --- | --- |
| store card, middle | ICP (previous pass) | 0.44 px, centre | yes |
| store card, streak end row + radii | darkness map + traced residual | depth-map loss 0.260 -> 0.053 | yes |
| store card, top family (top edge -> right verticals) | smooth keys, 9 control points | 1.56 px on 38% of its points | yes (replaces the top-band wedge sheets) |
| store card, top-left | not fitted (two attempts failed: straight segments from the left edge make a fan; a graph fit along x crosses its own lines) | | no: the middle sheet's converging left ends still draw a dark wedge |
| panel | graph sheet + twin, knots 10/20 | 0.95 / 1.02 px, 96% of the 82% usable points | no: the fold's lines run near-horizontal and are not graphs over y, so the fit has no fold; neither does it look like the reference |
| EMAIL | graph sheet + twin, knots 20 | 0.82 px, 97.5% | no: fits the vertical grain and the top-left crossings but not the book-match hook |
| mailbox, SMG, BASKET | not fitted: the tracer resolves 4-19 crossings per column (pitch 1.5-4 px, text) | | hand-set as before |

## Still wrong (third pass)

- Store card: the streak band has dark lines along its upper and lower
  edges where the horizontal runs stack, and a light middle; the reference
  is a lattice of crossing lines with no edge lines and a core only on the
  right. The construction needed is post-knee runs that keep descending
  across the axis (the leaves crossing), which the end-row fit did not
  find. A vertical seam at x ~500 where the top family meets the middle
  sheet. The top-left wedge.
- The other five surfaces keep the hand-set geometry and only gain the
  new line, ground and grain rendering.
- The committed textures were **not** regenerated: side by side, the
  committed noise-field store card still reads closer to the reference
  than this one.

## Store card, fourth pass: the streak as crossing leaves

Oriented ridge filters on the 4K crop (17 angles from -40 to +40 degrees)
show the store streak's construction. Above the axis, rows 276-288 carry
lines descending to the right at +15 to +20 degrees from x 220 to 540. Below
it, the mirror image carries lines at -12 to -17 degrees. In rows 288-296,
x 100-260, both orientations are present: lines from each leaf continue past
the axis and cross there. From x 340 rightwards, near-horizontal ridges at
y ~283 and ~302 converge on the axis (288 and 294 at x 500), and they meet
in the core at x 500-620.

**Model (in GEOMETRY).** Each middle-sheet line now ends in a tail
`[knee, crossing knee, C]`. From the bottom of its vertical run at
(x_v, y_k), a straight run descends at theta until it reaches
y = 291.5 + h, on the far side of the axis. From there it runs to a shared
convergence point C = (640.3, 291.5 + 0.55). The mirror copy is the other
leaf, so the two leaves cross.

| Key | y_k | theta (deg) | h |
| --- | --- | --- | --- |
| x_v ~20 | 268 | 24 | 17.7 |
| x_v ~198 | 265.2 | 11.9 | 9.2 |
| x_v ~498 | 271.5 | 7.2 | 5.1 |
| top family, right | 245.8 | 10 | 0.8 |

The fillets are 30 at the knee and 16 at the crossing knee. The twin uses
the same rule from its own knee (capped at +-6 px from the main sheet).

How the parameters were fitted: Powell on the band rows 240-345, with text
masked. The loss has three terms: oriented-filter energy (blurred sigma
2.5), darkness (blurred sigma 2), and the per-column counts of G < 80 and
G < 140 at x 160-620 against the reference. Bounds keep h > 0, which forces
the crossing. key 0's knee and angle were then set by hand (268 / 24) to
shrink the empty diamond at the left end of the band. Scripts:
`e3/model.py`, `fit2.py` and `build.py` in the pass scratch.

**Top-left.** Middle keys 0 and 1 gained 10-point smooth heads. Key 0 enters
from the left edge (y 186). Key 1 enters from the top-left corner and runs
straight to its fold, because S-shaped initialisations fitted worse
(coverage 63% vs 70%). Key 2's head is the top family's key 0, so the top
sheets skip their first line (`skip`, a new sheet option). This removes
the wedge and the doubled line at x ~500.

| Region (traced points) | Previous rms / coverage | Now |
| --- | --- | --- |
| centre (x 190-520, y 95-262) | 0.57 / 98.2% | 0.81 / 97.9% |
| streak band (x 150-629, y 262-322) | 0.93 / 81.1% | 1.01 / 90.5% |
| top-left (x 0-200, y 0-150) | 0.89 / 52.2% | 1.18 / 70.1% |
| top band (x 250-629, y 0-100) | 1.35 / 56.4% | 1.24 / 74.6% |
| whole card | 1.08 / 77.7% | 1.14 / 85.7% |

Streak darkness, 3 columns averaged: pixels with G < 80 (and G < 140).

| x | 200 | 300 | 400 | 500 | 540 | 600 | 620 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| reference | 0 (3) | 0 (12) | 0 (23) | 7 (21) | 10 (17) | 4 (21) | 3 (27) |
| now | 2 (15) | 4 (18) | 4 (23) | 6 (21) | 8 (17) | 9 (13) | 8 (13) |

**Still wrong.**

- The band edges are drawn as crisp single lines where the crossing knees
  stack along a straight fold row. The reference's band edges are soft.
- A thin dark axis line runs from x ~200, where the reference has no core.
  The core does not thin past x 560 (8-9 px at x 600-620 against 3-4).
  Moving C, or letting the crossing knees fan rather than lie on one fold
  row, is the next thing to try.
- The top-left is straight diagonals, not the reference's stair-step
  S-curves. The wedge is gone, but the curves are missing.
- At x ~500 a fold discontinuity remains between the middle sheet's sharp
  fold and the top family's smooth heads.
- The centre rms rose from 0.57 to 0.81 px, because the knee fillet grew
  from 7 to 30.
- The committed noise-field PNG still reads closer to the reference
  overall. It was not regenerated.
