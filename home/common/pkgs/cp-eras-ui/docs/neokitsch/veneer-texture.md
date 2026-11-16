# NK-07: the veneer as a generated texture (2026-11-13)

> 2026-11-14: [veneer-construction.md](veneer-construction.md) recovers the
> reference's actual construction (blended polylines, twin combs, book-match
> mirrors) and adds a candidate generator, `scripts/make-veneer-blend.py`.

Owner decision 3 of [the 2026-11-13 review](../review-2026-11-13/DECISIONS.md):
the selected Neo-kitsch surfaces wear a **pre-rendered texture clipped to
their shapes**, not drawn vector grain. Accepted costs: an image asset, and
less clean scaling at odd sizes. [veneer-fit.md](veneer-fit.md) is the
history of the vector fits that did not converge.

The reference art is third-party and never redistributed, so the texture is
**generated**: `scripts/make-veneer.py` computes every texel from the
parameters below, reads no image, and writes `veneer/*.png`. The references
were used only to measure (numbers below) and to compare by eye.

## Third pass (2026-11-15): the noise-field figure with the measured line

The owner chose to combine the two generators: the noise field's
*figure* (where the grain flows, the streak positions) drawn with the
blend generator's measured *line, ground and grain*
([veneer-construction.md](veneer-construction.md)). `scripts/make-veneer.py`
now does that; `make-veneer-blend.py` is unchanged and still imports it
for `SURFACES` (size, seed), `R` and `png_indexed`. The six PNGs in
`veneer/` were regenerated (same names, sizes, 8-bit indexed format, so
`src/veneer.rs` is unchanged; 1.13 MB on disk, was 714 KB: the grain
compresses worse).

The pipeline, per surface (everything numpy, no image read):

1. **Field.** As before, families of `phi(s, t) = s + D(s, t)` in design px
   with windows along `t` and an optional `mirror` (book-match). The axes
   are now the measured ones: store card t 121.46 (texel 291.5), panel
   154.4 (370.5), EMAIL 150.8 (362), mailbox 238.1 (571.5), SMG 99.6
   (239); the BASKET's two leaves are no longer mirrored. The spacing
   noise `B` is off (`SPACING['amp'] = 0`); the per-line weights and the
   tone terms (blotch, edge shade, palette ramp) are gone. New term
   `lean`: past `ts` (a number, or `(at s=0, at s=S)`) the lines run at
   slope `amount` (ds/dt) after a softplus knee `w` wide, steepening by
   `accel` (power `accel_power` across the grain). It drives the store
   streak and the EMAIL hook, and it does not fold back like a symmetric
   `bend` does.
2. **Lines.** Levels `phi = (k + offset) * pitch`. `pitch` is the measured
   per-comb spacing in native px / `R`: store card 11.6 (two combs give
   the measured 5.8 gaps), panel 10.2, EMAIL 8.9, SMG 4.9, mailbox 5.7,
   BASKET 4.4 (one comb). `twin = dict(offset, shift)`: the second comb at
   `offset` of a pitch (the measured gap ratio: panel 0.42, EMAIL 0.62,
   SMG 0.61, mailbox 0.74, store card 0.5), its figure terms moved `shift`
   design px along the grain so the combs knee on alternate rows (store
   card -2, panel -1.5, EMAIL -1). Each level is traced as a contour from
   where it crosses its family's box (`PERIM`, `STEP`); a contour is kept
   once.
3. **Straight runs.** `QUANT`: direction held in 30-degree bins (3 degrees
   of hysteresis), a vertex where the bin changes, chords between,
   Douglas-Peucker at `tol` 0.5 design px, every corner filleted at
   `radius` 9.5 texels. A gentle lean stays one straight run; a turn
   becomes 30-degree knees whose rows across neighbouring lines are the
   folds.
4. **Render.** 1-texel box strokes over the flat `look['ground']`,
   compounding towards `CORE` (150, 61, 30) with per-channel opacity from
   `look['single']`; per-channel `[1, 3, 1]` grain `look['grain']`;
   256-colour k-means palette. `ink` on a family scales its opacity.

Authoring a surface: measure `look`, `pitch` and the twin ratio
(veneer-construction.md), place the axis, then build the figure from
terms and judge it at native 4K against the reference. Avoid profiles
with a corner on a seam the lines cross (`bend` with `c` or `p <= 1`
inside the box): their contours hairpin there; the tracer steps through,
but the line comes out doubled. A `fold`'s `c` knee is fine (one-sided).

Numbers (4K, reference / committed second pass / now):

- Pitch, mean gap of G minima: store card 5.76 / 4.88 / 5.88; panel
  5.14-5.22 / 4.85-4.99 / 5.10-5.19; EMAIL 4.10-4.17 / 4.57-4.68 /
  4.20-4.38; SMG 2.88-3.27 / 4.55-4.60 / 2.63-2.94; mailbox 3.25-4.70 /
  3.94-4.40 / 3.67-4.64; BASKET 4.0 / 4.3-4.4 / 4.3-5.0.
- Store streak, pixels per column with G < 80 (G < 140), rows 240-345, at
  x 200 / 300 / 400 / 500 / 540 / 600 / 620: reference 3(10) 0(13) 0(25)
  7(22) 9(19) 4(25) 3(30) (x 200 is text); committed 0(4) 0(6) 0(15)
  6(24) 7(32) 11(38) 12(43); now 0(12) 0(13) 0(18) 3(22) 6(23) 7(25)
  7(25).
- Brightest 20% mean RGB, store card: 254,211,133 / 255,210,135 /
  255,211,131.

Still wrong: the store core does not thin at the right edge (7 px at x
600-620 against 3-4) and starts later (x 500: 3 against 7); its upper
streak (t 42) is weaker than the reference's sweep; the panel fold is
smaller and lower than the reference's and its bottom-left lean less
extensive; the EMAIL crossing is narrower and its right-edge core lighter;
the mailbox and SMG seams are thinner creases than the reference's hooks;
overall the reference reads a little darker (more lines in the figure).

### Refinement (2026-11-15, after the merge)

Two surfaces refit; the other four PNGs are byte-identical.

- **Panel fold.** The committed fold leaned the wrong way: traced with a
  structure tensor on the reference (native crop, 15 px rows), the lines
  above the crease run down-*left* (angle +40..+82 deg from vertical),
  ours ran down-right (-40..-89). The fold is now two `ramp` terms, a new
  kind: a band between curves `a(s)` and `b(s)` across which the lines
  step sideways by `amount`, softplus knees `k` at both edges (curves and
  amounts piecewise linear across the grain, `[(s, v), ...]`). A1: knee
  row from (14.6, 220.8) to (187.5, 260.8) design px, crease from
  (83.3, 286.7) up to the tip at (187.5, 262.5), amount 95 / 125 / 105 /
  70 / 30 at s 14.6 / 56.2 / 97.9 / 139.6 / 187.5. R2 below the crease,
  to t ~330, amount 220 at the left falling to 150 at s 79 and 70 at
  s 94 (the drop makes the bottom-left jog at native x ~200), 60 at the
  tip. Right of s 187.5 a separate strip (`swindow`, new: a family cut
  across the grain; its lines end at the cut, the reference's jog at
  native x ~450) that only knees to a gentle lean (amount 40 then 20).
  A1 tapers towards the strip edge by less than 1 per px: a steeper
  taper makes `phi` non-monotone across the grain and the lines hairpin;
  a large amount at the edge feeds lines into the crease from the strip
  edge, a solid dark wedge. Knee row (first row off vertical) reference /
  committed / now at native x 35, 235, 435: 530, 553, 605 / none (the
  committed fold starts ~560 at the right only) / 515-530, 545, 590.
  Not mirrored into the upper leaf: checked, the reference's upper half
  under the text is vertical grain with only small edge bends.
- **Store streak.** `STREAK_LEAN`'s `accel` and `STREAK_BEND`'s amount are
  piecewise now, peaking at s 208-235 and easing at the right edge:
  accel 0 / -0.45 / -1.6 / -1.6 / -1.3 at s 0 / 150 / 208 / 235 / 262,
  bend 0 / -3 / -4 / -3.5 at s 0 / 200 / 235 / 262. Pixels per column
  with G < 80 (G < 140), x 200 ... 620 as above: reference 3(10) 0(13)
  0(25) 7(22) 9(19) 4(25) 3(30); third pass 0(12) 0(13) 0(18) 3(22)
  6(23) 7(25) 7(25); now 0(8) 0(15) 1(23) 7(25) 7(25) 5(25) 3(22).

Still wrong after it: the panel's crease is a soft convergence, not the
reference's thin dark line, and the lines below it run flatter (~75 deg
against 30-60); the store core is not pointed and the reference's knees
rise towards the right edge (ours are level; raising them widened the
band past the reference's G < 140 counts); the store card's upper streak
and zigzag, the EMAIL crossing, the small surfaces' seams and the
overall line density were not revisited.

### Second refinement (2026-11-15)

Four surfaces refit (EMAIL card, mailbox row, SMG button, BASKET band);
`dashboard-panel.png` and `store-card.png` are byte-identical. Angles
below are line directions from a structure tensor on the native 4K crop
(texel coordinates; for the EMAIL card degrees from vertical, negative =
going down to the right; for the horizontal surfaces degrees from
horizontal, positive = rising to the right). Reference / previous / now.

Two generator changes, neither of which alters the panel or store card:

- **`comb`.** A term marked `comb='main'` (or `'twin'`) shapes that comb
  only; the other runs straight through it, so the two cross.
- **Out-and-back traces trimmed.** A trace that ends where it started
  (the tracer stepped through a cusp and came back up the same arm) is
  cut at its farthest point, so the arm is not stroked twice. The EMAIL
  card's knee at the axis made six such traces; the old SMG left bend
  (`p=1` with its corner at t=0, inside the tracing box) made three.
  The left-end bends of the SMG, mailbox and BASKET now sit at
  `ts=-3`, outside the box.

- **EMAIL card.** The previous seam bent every line nearly horizontal
  (angles to +-90 from x 10 to 210) into a dark horizontal band, which
  is not what the reference does: its lines lean only 15-20 deg into a
  crossing lattice from texel y ~300 to ~384 at the left, and only
  towards the right edge do they curve into a spindle whose point is on
  the right edge at the axis. Now: `EMAIL_LEAN_U` / `EMAIL_LEAN_L`, a
  lean of -0.3 at the left easing to -0.15 at the right (ts (122, 110)
  for the upper leaf, (140, 118) for the lower in its mirrored
  coordinates, so the band sits a little above the axis as the
  reference's does), windows (None, 160) / (125, None); `EMAIL_CUSP`, a
  knee bend at the axis (w 18, c 0.6, amount 0 to -50, power 3: only the
  right edge gathers). Angle at rows 300 / 340 / 380, x 10 ... 210:
  reference -5 -12 -17 -17 -12 -22 / 0 4 12 12 8 -37 / 5 13 17 13 6 52;
  previous -11 -29 -55 -75 -85 90 / -55 -75 -88 88 -90 90 / 61 79 89
  -87 89 -89; now -3 -7 -10 -15 -19 -23 / 2 -3 -17 -40 -66 -71 / -2 2
  13 36 67 72. The top-left swirl is `EMAIL_SWIRL`, a `shear` on the
  main comb only (above t 44, w 9, amount 0.5 to 1.3), the twin running
  straight through it: the reference's crossing arcs under the chamfer
  (its angle at y 0-60 is negative, up-left; the previous top bend leaned
  the other way, +20..+35). `EMAIL_TOP` (a bend proportional to s, real,
  t 70, w 110, 30) packs the upper half tighter and `EMAIL_EDGE` (a
  knot centred past the right edge, r 14, 5) the right edge, as measured:
  pitch at y 100-260 in six bins across, reference 5.5 4.2 3.8 3.7 3.9
  3.1, previous 4.5 4.1 4.4 4.5 4.4 4.6, now 4.6 3.7 4.1 3.9 3.9 4.0. The
  foot's bend now leans the lines a little left (amount 8 to 0), as the
  reference's do (+3..+12 at y 560-660; the previous -5..-26).
- **Mailbox row.** The seam's lines curved the wrong way: in the
  reference they curve *down* into the seam, more towards the bottom
  edge; ours rose. `MAIL_SEAM` is now w 6, amount -1 to -12 (was w 12,
  4 to 40), `MAIL_CREASE` 0 to -4. Angle at x 530 / 545 / 560 (left of
  the seam), rows 90 and 130: reference -80(text) 87(text) -60 / -2 -19
  -54; previous 33 50 63 / 58 74 65; now 0 -13 -53 / -2 -19 -72. The
  left end gains the reference's step: a `ramp` (amount -16) across a
  band 36 wide whose centre runs t 50 / 70 / 60 / 35 at s 0 / 17 / 32 /
  55 (`MAIL_STEP`), the lines descending ~30 deg through it. A band
  that bulges more (the reference's centre swings 60 in t over 30 in s)
  makes `phi` non-monotone across the grain and the lines overlap into
  dark bundles: keep `amount * dcentre/ds / width` under ~0.6.
- **SMG button.** The seam is narrower (w 6, amount 4 to 9; was w 10,
  3 to 26): angle at x 230 / 250, rows 45 / 65 / 85: reference 44 -54 /
  20 -25 / 1 -1; previous 41 -45 / 76 -53 / 68 -57; now 40 -37 / 59 -52
  / 69 -61 (still too steep at the bottom). The right end now falls
  towards the edge (bend w 20, p 2, amount -16 to -8): at x 435 / 465,
  rows 20 and 50, reference -30 -29 / -45 -32, previous -3 -5 / 1 2, now
  -22 -16 / -17 -16.
- **BASKET band.** The right end fell in the reference and rose in ours;
  now a bend at the right edge w 26, p 2, amount -4 to -18: at x 400 /
  430 / 460, rows 53 and 77, reference -43 -44 -10 / -33 -19 -12,
  previous 5 9 9 / 4 4 11, now -5 -12 -18 / -9 -21 -22. The seam bows
  evenly top to bottom (amount 4.5 to 5.5, crease 2.5 to 3) as measured
  (+16..+26 / -13..-19 at x 190 / 220 on every row); the left end's
  lower lines fall to the edge (amount 0 to -18).

Line density, mean G and pitch (native px) in clear windows, reference /
now (the four surfaces above) or unchanged: EMAIL lower half 177.2 4.38
/ 178.1 4.29, upper half 173.7 3.53 / 175.3 4.01; panel 177.6 5.81 /
182.9 6.28 (unchanged); store card 177.7 6.80 / 191.8 6.27 (unchanged;
the reference's lines are darker, not denser); SMG 179.0 4.12 / 174.0
2.77; BASKET 162.3 4.03 / 165.9 4.52; mailbox 174.7 4.29 / 182.9 4.04.

Still wrong after it: the EMAIL band's top and bottom edges are hard
horizontal cuts (window ends) where the reference's are softer, and its
crossing lines are straight where the reference's curve; the EMAIL
swirl's arcs are 30-deg chords, not the reference's nested curves. The
mailbox seam lacks the reference's dark full-height crease line, the
lines meet in a V rather than turning vertical, and the left-end step
is shallower than the reference's S. The SMG seam is still too steep at
the bottom and the SMG lines are denser than the reference's (2.8 vs 4.1
px). The panel (bottom-left clump, kinks at x ~135 / ~200, the lines
under the crease, the crease line) and the store card (pointed core,
rising knees, upper streak, zigzag) were not revisited, and neither was
the overall density: the panel reads ~5 G levels lighter and ~8% sparser
than the reference, the store card's lines ~14 levels lighter.

## Surfaces

| Texture | Surface | Design box | Texels |
| --- | --- | --- | --- |
| `dashboard-email` | selected cascade card (`#ncardsel`), card-local, so a held card wears it too | 90.5 x 325.2 | 218 x 781 |
| `dashboard-panel` | detail panel body | 230.4 x 309 | 553 x 742 |
| `mailbox-row` | selected row bar (`#selclip`), row-local, follows the selection | 477 x 55 | 1145 x 132 |
| `store-nav` | selected category (SMG), button-local, used by every category | 200.6 x 38.6 | 482 x 93 |
| `store-card` | selected card body + socket band, card-local; the inferred held compact band reads the same texture at the same card-local position | 262.1 x 241.7 | 630 x 581 |
| `store-basket` | BASKET plate's lower band | 204.6 x 45 | 492 x 108 |

Not veneered: the login's ENTER / LOGIN bars. Measured, they are a flat
fill with fine noise only (green std 5.3 levels in clear windows, no line
structure; `#ffc470` mean), so a figured texture would contradict the
reference. The BASKET's upper half is likewise flat (std ~1). The widget
mail app (`panels/mail.rs`, not a reference screen) keeps its synthesised
`Fill::Veneer`.

## Measurements (references at 3840x2160, reported in 1600x900 design px)

Per surface: base = median of pixels above the 60th green percentile; line
core = mean line profile at its minimum; pitch = local-minimum spacing in
text-free windows (FFT where clean).

| Surface | Base | Fill median | Line core | Pitch | Between-line std (G) |
| --- | --- | --- | --- | --- | --- |
| dashboard EMAIL card | `#ffbf6c` | `#ebab5b` | `#d19348` | 1.9-2.7 | 6.3 |
| dashboard panel | `#ffc26e` | `#f6b664` | `#e2a354` | 2.12-2.18 (calm), 3.4-3.8 under the figure | 3.9 |
| mailbox row | `#ffcb84` | `#efb873` | `#d99f63` | 2.24 (1.4-3.7, bunched) | 6.0-7.3 |
| store SMG | `#fac883` | `#ecb670` | `#d49955` | 2.0 | 8.6 |
| store card | `#ffce84` | `#f6bc71` | `#e1a253` | 2.2-2.4 | 4.4 |
| store BASKET band | `#fab56b` | `#e89e5f` | `#d78854` | 2.0 | 8.0 |

Common to all: a dense field of thin dark lines (core about 1 native px,
30-35 green levels below the base), pitch about 2.2 design px (5.3 native),
fine pore noise, and figure made of seams where the lines bend, converge and
cross. That is what one generator with per-surface parameters serves.

Per surface:

- **Seams.** EMAIL: horizontal book-match about halfway down (local y ~160,
  design y ~545), the leaves converging to the right edge and crossing in a
  ~25 px band; a second crossing swirl at the top-left. Mailbox: vertical
  open-book seam at design x ~274 (local 239), lines fanning away from the
  bar's middle height into it. SMG: vertical seam at design x ~193 (local
  100, the button's middle; the first pass put it at 168). Store card:
  horizontal streak at design y ~531 (local 120), converging to the right
  edge, dark core ~4 design px tall at the right. BASKET: vertical seam at
  design x ~1377 (local 85; the first pass put it at 1329). Both corrected
  positions are the darkest smoothed column of the band, text rows
  excluded.
- **Figure.** Panel: a fold in the lower third, the ridge from local
  (184, 259) to (84, 299), lines leaning down-left above it. Mailbox: the
  left end's lines rise to the top edge.
- **Edge shading.** Weak: 1-9% L* over 5-10 design px (EMAIL darkens toward
  its right edge 0.768 -> 0.698; the mail row lightens to the left,
  0.826 -> 0.78). The panel shows no measurable top/bottom darkening.
- **Rim.** Edges are hard (1-2 native px) with a faint light line inside:
  +10..15 green levels for 1-2 native px on the mail bar's top and bottom
  and the card's right edge; none on the panel. Drawn as a 0.5 design px
  `#ffe2a6` at 0.55 inside the edge; EMAIL keeps its traced 1.2 `#f2b463`
  outline instead.
- **Ink on the fill** (text box, ink = darkest 3%, fill = median of the
  brightest 40%): panel copy CR 7.32 on fill Y 0.605; mail title 5.64 / Y
  0.603; card "Recoil" 6.67 / Y 0.670; SMG 6.25 / Y 0.622; BASKET fine print
  3.94 / Y 0.511.

### Second pass: how the reference draws lines (2026-11-14)

Measured on the 4K references, design px unless stated; scripts in the
pass's scratch, numbers only kept here.

- **Line width.** Row profiles in text-free windows of near-vertical grain
  (FWHM at half depth, native px, p10 / p50 / p90): store card 1.17 / 1.89
  / 3.36, panel 1.19 / 1.81 / 2.48. Depth p50 44 and 33 green levels (p90
  66, 51). So the lines are not uniform: some are a hair, some twice that,
  and some lines are bolder than their neighbours along their whole
  length.
- **Spacing.** Store card mean 2.16, p5-p95 1.05-2.79, coefficient of
  variation 0.27; panel 2.22, 1.44-2.85, CV 0.30; mail row 1.77, CV 0.58
  (bunched). The first pass's lines had CV 0.11-0.12.
- **Converging lines.** Down columns through the store streak (design x
  790 -> 925): the darkest pixel goes G 111 -> 57, the core (below 55% of
  the base) 0 -> 7.5 design px tall, the band below 75% 8-27. Above it the
  lines keep their own minima (G ~140-150) while the gaps between them
  close first (gap peaks fall from ~205 to 175-190 at a 3 native px pitch);
  only then is there a core, and it is far darker than any one line (G 57
  against ~140): overlapping lines compound like ink, they do not average.
  Its edges are 4-5 native px. Colour: a single line darkens R : G : B in
  log ratio about 0.45 : 1 : 1.5 (EMAIL 0.66 : 1 : 1.4); the core is
  `#94391f`, blue near its floor.
- **Knees.** Lines run straight until 10-20 design px from a streak or
  fold and then turn within 4-5: a sharp knee, not a long lean. The panel
  fold is vertical lines that turn left at a knee (local y ~215-230),
  sweep down-left nearly horizontal to a crease (from local (175, 263)
  leftward), and leave it leaning down-right with a wider pitch.
- **Wander.** Long: the panel's right strip drifts 25 px over its top 50
  and is then straight (residual 2.1 px rms, carried by wavelengths of
  110-220); an EMAIL strip drifts under 1 px over 100. The store card's top
  40 px has a cathedral zigzag of about 30-40 px along the line, +-2-3 px.

## The generator (first and second passes; superseded by the third pass above)

`scripts/make-veneer.py`, numpy only. A texture is the iso-lines of scalar
fields `phi(s, t) = s + D(s, t)` (`s` across the grain, `t` along). A surface
holds one or more line *families*, each with a window along `t`; a family
can be the mirror of the field about `t = m` (the book-match), and where
windows overlap both are drawn and cross. `D` sums a long, low-gain lattice
wander (evaluated at the reflected position for a mirrored leaf, so the
leaves meet instead of doubling), random small `kinks` (gaussian knots the
lines bend around) and the figure terms: `bend` (profile `exp(-(d/w)^p)`, a
tent, or a *knee* `exp(-(d^2 + 2cd))` that keeps the lines straight until
close and then turns them hard), `fold` (the panel ridge, with a knee above
and a relaxation `below`), `shear` (one-sided lean off an edge), `ripple`
(the cathedral zigzag), `knot` and `fan`.

Second pass (2026-11-14), against the first pass's soft bands and even
grain:

- **Lines are strokes, not distances to the nearest line.** Each line is a
  gaussian of fixed width in native px (`core` 1.2 FWHM, times a per-line
  width factor 0.7-1.15), whatever `|grad phi|` does; the four nearest lines
  on each side are summed as optical depth `tau`, families add, and a texel
  is the mean transmission `exp(-tau)` over a 4x4 supersample. Bunched
  lines keep their own cores and their gaps close first; where several
  overlap the ink compounds into a core, softly saturating (`sat`, ~1.35-1.6)
  so a pile of lines stops at the reference's G ~57 instead of black. The
  first pass took the *maximum* of gaussians over a 3x3 supersample, so a
  convergence became a uniform band at single-line darkness, blurred.
- **Irregular spacing.** The lines are the levels of `phi + B(phi, t)`, `B`
  smooth noise in `phi` itself (|dB/dphi| <= `amp`, 0.38-0.5), so a bunch
  of lines stays a bunch along the grain: local pitch runs pitch/(1 +- amp),
  about the measured 1.4-3.7.
- **Per-line weight and width,** from an integer hash (splitmix64) of the
  line index, the weight drifting along the line over 70 px: bold and faint
  lines, as the reference's.
- **Colour by optical depth.** The palette is light -> base below `base_at`
  and `base * exp(-tau * tint)` above it, `tint` the measured channel ratio,
  so index is linear in `tau` (up to `taumax` 2.2). Pore noise and blotch
  are scaled to the new ramp (2.4 and 1.5 index units, ~5 G levels).
- Sharp **knees** for the streaks, the fold and the seams (two `bend`s: a
  wide knee for the hook, a narrow exponential for the core); `shear` and
  `ripple` for the store card's slanted, zigzagging top; crossing leaves
  around the EMAIL book-match; the mailbox and SMG seams now bend the lines
  *down* into a vertical crease (the first pass's fan opened them into
  chevrons and left stray dark dashes, gone with the fan).

Output: 8-bit indexed PNG, 2.4 texels per design px: 3840x2160 samples it
1:1, and nothing above 4K is a target. Total 714 KB on disk (1.21 M texels,
unchanged; the first pass's were 942 KB).

Reproducible: hand-written PNG encoder (fixed filter, zlib 9), float64 in
fixed 32-row chunks, fixed seeds, integer hashing. Two runs are byte-identical; `default.nix`'s
`passthru.tests.veneer` re-runs the generator in the sandbox and `cmp`s each
file against the committed one. Regenerate with

    nix-shell shell.nix --run 'python3 scripts/make-veneer.py docs/neokitsch/veneer'

## Drawing

`Prim::Veneer { x, y, texture, shape }` (`src/style.rs`) and the mailbox's
`Veneer::texture`. `src/veneer.rs` decodes the PNG (indices + palette, plus
2x2 box mips for small scales), and rasterises the surface's own fill prims
with `soft.rs`'s coverage, sampling the texture per pixel with the rim:
bilinear at or above 1:1 (4K); below it an *exact box* over the pixel's
footprint, every texel weighted by the area it covers, on level 0 while the
footprint is under 4 texels (1600x900 reads 2.4) and on the mip that brings
it to 2-4 texels beyond that. That is the downscale the reference gets at
1600x900. The first pass sampled the 2x2 mip with a 2x2 tap grid of
bilinears, an effective blur about 3.5 texels wide, which smeared bunched
lines into a uniform tone (the "muddy" panel). A baked design-scale texture
was the alternative: it would cost a second asset set (+17% texels) to buy
what an exact box of the full-resolution texture already gives. The result is a
premultiplied image of just the surface's pixel box, cached per surface,
position and scale (64 MiB / 64 entries).

A canvas draws images above every mesh in its layer, so the scene is painted
in layers (`Pass` in `screens/scene.rs`, `widgets/layered.rs`): veneer `i` is
the last thing in layer `i`; a fill touching an earlier veneer's pixel box
(or an earlier raised fill) goes above it; everything else stays in layer 0
so its antialiasing is unchanged. The mailbox draws its veneer in a layer
under the sheet. A scene without a veneer is one layer, exactly as before.

## Containment, legibility, cost (2026-11-13)

- Golden matrix, rendered headlessly: 22 of 25 cases are pixel-identical.
  The three Neo-kitsch cases differ only inside the veneered boxes (plus a
  1 px rim), except one pixel: (1455, 19) in the store, the top of the BASKET's
  vertical hairline, which joins the band's hairline and so moves to the
  band's layer; its antialiased top pixel against the plate edge resolves
  separately (delta 78 on one pixel).
- Fill under text, 1600x900, Y (encoded lightness `srgb(Y)`): panel 0.556
  (0.78), mail 0.638 (0.82), card 0.597 (0.80), SMG 0.593 (0.80), BASKET
  0.484 (0.73). The ink correction assumes 0.78. Contrast moved toward the
  reference everywhere (mail title 3.80 -> 4.35 vs 5.64, card 620 7.26 ->
  8.90 vs 7.90), except the panel copy, 6.84 -> 6.52 vs 7.32.
- Binary: +0.98 MB per binary (dashboard 30.61 -> 31.60 MB release).
- Cold, release, CPU: decode + mips of all six 11.5 ms; per image at
  1600x900 1-11 ms, at 3840x2160 2-29 ms (panel the largest, 553x742 px,
  1.6 MiB RGBA). Retained: at 4K about 3.4 MiB of images per screen plus
  ~1.6 MiB of decoded indices and mips; GPU atlas the same image sizes.
  No full-screen image is added.
- Per frame: one extra walk of the display list per veneer on screen (store
  3, dashboard 2), each drawing only its own leaves. Not timed.
- Real-GPU cost is unmeasured; all numbers are software Vulkan / CPU.

## Second pass: containment, legibility, cost (2026-11-14)

- Golden matrix, rendered headlessly: 22 of 25 cases pixel-identical; the
  three Neo-kitsch cases differ only inside the veneered boxes (plus the
  1 px rim), with **no** pixel outside. The first pass's stray pixel at
  (1455, 19) is gone: the BASKET's vertical hairline is now listed before
  the veneer in `neokitsch.rs`'s `CONTENT`, so it stays in layer 0 (it ends
  where the band begins and shares the rule's ink, so the order changes
  nothing else).
- Fill under text, 1600x900, Y, then contrast, reference / first pass /
  now: panel copy 0.605 / 0.556 / 0.602, CR 7.32 / 6.52 / 7.01; mail title
  0.603 / 0.638 / 0.655, 5.64 / 4.35 / 4.45; mail from 5.28 / 3.61 / 3.68;
  card "Recoil" 0.670 / 0.597 / 0.636, 6.67 / 7.33 / 7.69; card 620 0.561 /
  0.591 / 0.540, 7.90 / 8.90 / 8.19; SMG 0.622 / 0.593 / 0.603, 6.25 / 6.65
  / 6.78; BASKET fine print 0.511 / 0.484 / 0.481, 3.94 / 2.33 / 2.39 (its
  ink, not its fill, is what is light there).
- Line statistics at 4K (FWHM p50 native px, pitch CV), reference / first
  pass / now: store card 1.89, 0.27 / 1.93, 0.12 / 2.00, 0.23; panel 1.81,
  0.30 / 1.38, 0.11 / 1.78, 0.31; mail row 2.03, 0.58 / 1.25, 0.53 / 1.53,
  0.46. Store streak, texture columns at design x 790 -> 925: min G 126 ->
  61, core 0 -> 10 design px (reference 111 -> 57, 0 -> 7.5).
- Assets: 714 KB (first pass 942 KB), same six files and sizes. Generation
  ~15 s for all six (4x4 supersample, chunked; peak memory small).
- Cold, release, CPU, measured while the machine ran at load ~60-80 (the
  unchanged 4K bilinear path read 1.8x slower than the first pass's
  unloaded figures, so scale accordingly): decode + mips 20 ms; per image at
  1600x900 1.6-15 ms (panel), at 4K 4-52 ms. Load-normalised, the exact box
  at 1600 costs about what the mip path did (panel ~9 ms against 11).

## Open

- The look is the owner's call. Known differences after the second pass:
  the store card's top figure (slanted zigzag grain and the small upper
  streak) is an approximation; the panel fold's crease is softer and its
  right-hand knee rounder than the reference's right angle, and the
  reference's faint vertical jogs (x ~190 and ~450 native) are not
  modelled; the EMAIL top swirl is a crossing leaf, not the reference's
  nested arcs; the store streak's core is a little taller at the right edge
  (10 design px against 7.5). The mail bar keeps its traced chamfered end.
- Two recipes were rendered for the owner on the store card and the panel:
  "closer to the reference figure" (the one in `veneer/`) and "calmer" (streaks at
  60%, no zigzag, gentler fold, less bunching).

## First pass (2026-11-13), for the record

- The look was the owner's call. Known differences: the store streak's dark
  core merges into a soft band where the reference keeps resolvable lines;
  the EMAIL top swirl and panel fold are simpler than the reference's; the
  mail bar keeps its traced chamfered end, not the reference's tapered one.
- Three store-card variants (calmer / fit / stronger) were rendered for the
  owner; the generator takes any of them as a recipe change.
