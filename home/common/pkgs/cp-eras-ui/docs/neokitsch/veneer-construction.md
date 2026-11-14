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
