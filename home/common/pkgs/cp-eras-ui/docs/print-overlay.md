# Print overlay, 2026-11-13 (copies remeasured, and on for all eras, 2026-11-14; ground and chroma gates, Entropism refit 2026-11-15; enclosure gate, far-copy softness, measured grain 2026-11-16; elements above the print and the bloom, 2026-11-16)

Owner decision 6 (`docs/review-2026-11-13/DECISIONS.md`): one shared
print-texture pass per era instead of element-by-element echo and grain
fitting. This records what the reference surface measures as, what the
prototype draws, and its limits. Code: `src/print.rs` (era-side data and
CPU mirror), `src/eras/{neomil,entropism}.rs` `PRINT`,
`vendor/iced_wgpu/src/print.rs` + `shader/print.wgsl` (the pass;
`vendor/iced_wgpu/PATCHES.md`). On by default for every era since
2026-11-14 (Matt: "On for all eras"); see "On by default" below.

## What the references show

All eight references are 3840x2160 lossless exports, not photographs:
the dashboard's flat red tiles are exactly `#ef3333` with zero variance.
The "printed" look is designed, and it is not uniform across the screen.
Measured on the native files (design px = native / 2.4):

| Effect | Neomil | Entropism |
|---|---|---|
| Faint copies of bright ink | everywhere; displaced **outward from the screen centre** in proportion to distance: a whole-frame scale of 0.50-0.55 % (dashboard, store), 0.8 % (login), second peak near 2x (copies at k and 2k; matches the earlier per-element dashboard fits, e.g. JHN codes -3.6/+3.6 dp at x 45/1548) | same geometry, correlation peak 0.6 % on all four screens; visible ghosts on isolated stems sit farther (~0.8-1 %) |
| Rows of the copies | modulated at 4.764 native = 1.985 dp, depth ~55-60 % (amplitude/mean of the echo residual) | banded at ~8 native = 3.3 dp, depth ~20-30 %; plus a dot/moire structure not modelled |
| Fill scanlines | **not global**: clear ground 0.01 levels, dashboard tiles 0, mail selected row 0; present only on some fills (login active card +-3.5 on 246, store VIDEO +-6.5 on 197, GO HOME panel +-3.5 on 32), already drawn per element for the panel and badges | none on fills or ground |
| Grain | ~1.1-1.7 levels per channel on dark ground, ~0 on bright fills | sage fills RGB std 4.4/6.0/9.0 on 167/212/171 (~2.3 % of Y, chromatic, correlation ~1-2 native px); dark ground 0.7 |
| Softness / bloom | copies soft (~1-2 native px) | copies soft; clear bloom around light text and fills |

Vertical copy offsets are ambiguous in every fit (the copies' scanlines
comb the correlation at the 4.76 px pitch), so the scale is isotropic.

## Registration copies, measured 2026-11-14

This section supersedes the copy geometry, blend and row figures above and
in "The pass" below; the grain and fill-scanline figures stand.

The owner's reading of the echo was "2 or 3 separate ghostings, and then
they overlap, and that increases the intensity". Measured on the native
references: **two** separate copies, not three; and where they overlap
the result is brighter than either copy alone but less than the sum --
ordinary alpha compositing, `A = 1 - (1 - a1)(1 - a2)`. Neither "lighten"
(max, no gain on overlap) nor "add" fits as well.

### Method

A forward model per patch, fitted by least squares on the non-ink pixels:
the crisp ink mask `M` (the ink channel above a threshold, with its
anti-aliased edge), shifted by `d_i` and Gaussian-blurred, at gain `g_i`,
combined by over / add / max; a free per-row gain absorbs the row texture.
For the glowing eras a centred blur term (the bloom) is added. Patches:
13 hand-picked dashboard sites, plus an automatic tiler (220x110 native
tiles with 1-22 % ink on dark ground) over all four Neomil screens. The
geometry is then a line fit of `d` against position per axis. Tools are in
the session scratch (`gfit.py`, `auto.py`, `bfit.py`, `rule_reg.py`), not
the repository. The same tiler run on our own 4K mailbox render recovers
our parameters (copy 1 0.595 % / 0.99 %, copy 2 1.22 % across, gains 0.30
/ 0.14 against the reference's 0.30 / 0.15, over better than max on 36 of
44 sites), which checks the method end to end.

### Neomil

- **Count.** Two. On the dashboard's VEHICLES label one copy leaves rms
  0.030, two 0.013; a third takes no weight (gain -0.01..0.003). The header
  rule on the mailbox and login shows them cleanly apart: copies at -6.6 and
  -12.7 native rows (mailbox), -7 and -13 (login), nothing beyond.
- **Geometry.** Each copy is the whole frame scaled about a point near the
  screen centre; residuals 0.1-0.2 native px across the dashboard. The
  registration differs per reference:

  | screen | copy 1 kx / ky | copy 2 kx / ky | centre (native) | red gain 1 / 2 | sites |
  |---|---|---|---|---|---|
  | dashboard | 0.50 % / 0.505 % | 0.92 % / 0.93 % | 1935, 1160 | 0.46 / 0.30 | 13 |
  | mailbox | 0.60 % / 1.00 % | 1.22 % / 1.9 % (rule; tiler 1.73 %) | 1920, 1117 | 0.30 / 0.15 | 37 |
  | store | 0.54 % / 1.07 % | 1.45 % / 2.25 % | 1920, 1091 | 0.41 / 0.23 | 32 |
  | login | 0.59 % / 1.05 % | 1.3 % / 1.95 % | (mailbox's) | 0.34 / 0.23 | 8 + rule |

  So only the dashboard is isotropic and only there is copy 2 near 2x copy
  1 (1.85x); elsewhere the copies are stretched about twice as much
  vertically, and copy 2 sits at 1.8-2.7x. The login fit is the weakest
  (its ink mostly sits on the red cards, which hide the copies).
- **Blend.** Normal alpha. Dashboard marker boxes, where the two copies of
  a 30 px box overlap over most of their width: rms 0.0074 over, 0.0141
  add, 0.0181 max (box 1); 0.0071 / 0.0116 / 0.0128 (box 2). Over beats max
  on 34/37 mailbox and 31/32 store tiles. Over a coloured ground the copies
  lower the channels the ink lacks: per-row regressions across the
  dashboard rule's length (r^2 = 1.00) give one slope for all three channels,
  `out = (1 - a) bg + a C`, with green and blue pulled *below* the blue
  ground -- not lighten, not screen, not add.
- **Copy and original.** The copies lie under the art: ink pixels are
  exactly the ink (251, 53, 53) wherever a copy falls on them, and the flat
  tiles (#ef3333) have zero variance. Exception seen and not modelled: dark
  text on the mailbox's selected red row carries red row stripes, i.e.
  copies of the fill do show over that dark ink (the fill gate suppresses
  this).
- **Blur.** Gaussian sigma 1.05-1.14 native px (0.45 dp).
- **Texture.** Rows of 4.764 native px (1.985 dp): the period is 4.7640 and
  the phase the same (red crest on native row 4.68 + n 4.764) on all four
  screens and in both halves of each. It colours the copies rather than
  modulating their opacity: over black the copies' red swings 0.72-1.27 of
  its mean while green and blue swing 0.2-2.2 in antiphase; the rule
  regressions give the colour itself, about (240, 40, 40) to (215, 15, 15)
  near the red crest and (140-150, 50-56, 50-56) at the trough. Modelled as
  a factor on the copied colour, crest (1.0, 0.30, 0.30), trough (0.58,
  0.98, 0.98); the per-row colours also carry a second harmonic in green
  that the model does not have.

### The other eras (bloom-aware fit, fewer sites)

| era | sites | copy 1 kx / ky | copy 2 kx / ky | gain 1 / 2 | blur sigma |
|---|---|---|---|---|---|
| Entropism (store file `entropism-dashboard.png`) | 4 + 19 | 0.61 % / 0.68 % | 1.52 % / 1.60 % | 0.36 / 0.14 | 1.5-2 px |
| Kitsch | 7 | 0.60 % / 0.65 % | 1.58 % / ~1.8 % (0.6-2.9) | 0.28 / 0.08 | ~1.6 px |
| Neo-kitsch | 9 | 0.60 % / 0.60 % | 1.57 % / ~1.5 % | 0.21 / 0.12 | ~1.7 px |

Two copies in each, the second at about 2.5x the first, about isotropic,
over better than max on nearly every site. A third copy near 3.5x took
0.1-0.2 on some sites but lowered the residual only 2-4 %: possible, not
established, not drawn. Gains scatter widely between sites (e.g. Entropism
0.19-0.60): the copies are broken up by a diagonal circuit-like pattern
that also lies on the ground, and the text's bloom (a centred blur, sigma
about 5-8 native px, gain 0.3-1.9) overlaps them. Neither is drawn.

### The pass now

Up to three copies, each `Echo { kx, ky, gain }`, scaled about
`echo_centre`; blurred (4-tap box, `echo_soft` = sigma in dp); coloured by
`Texture { pitch, angle, crest, trough, phase }`; composited farthest
first by `echo_blend` (`Over`, or the old `Lighten`); gated by
`echo_lift` (a copy shows only where it is lighter than the pixel under
it -- what puts it under the art) and by the fill gate. `print::of(era)` is
the era's treatment for a new screen (Neomil: the mailbox fit);
`print::of_screen(era, Screen)` returns the per-screen fit, which the four
reference binaries wear. `print::DEFAULT_STRENGTH` is 1: the fit is worn
by default (below).

Overrides: `CP_ERAS_UI_PRINT_SET="echo1=kx,ky,gain;echo2=...;echo_centre=x,y;echo_blend=over|lighten;echo_soft=s;echo_soft_far=s;echo_enclose=reach[,thick];grain_mul=r,g,b;grain_add=r,g,b;echo_lift=lo,hi;echo_ground=lo,hi,floor;echo_chroma=lo,hi;tex_pitch=p;tex_angle=a;tex_crest=r,g,b;tex_trough=r,g,b;tex_phase=f;..."`.

Two more gates since 2026-11-15 (next section): `echo_ground` (a copy
landing on a mid-tone ground is weakened to `floor` of its opacity) and
`echo_chroma` (Neomil: copies only of saturated colour).

## Copies over mid-tone grounds and neutral marks, 2026-11-15

### What was wrong

Kitsch's dashboard tiles: the light VEHICLES / WEAPONS / LOCATIONS /
PRODUCTS labels on the mid-teal fill (encoded luminance 0.41-0.46) got a
plainly visible copy to their left that the reference barely shows; and
the tile outline got a grey band outside it that read as a dark halo.

### Method

A model-free *copy contrast* per site: on the ground pixels next to a
light mark, the mean of those the mark's copy 1 lands on (the ink mask
shifted outward by `k (p - centre)`) minus the mean of the mirror pixels
(the mask shifted inward by the same amount, so the same distance from
the ink). Anything centred on the ink -- the references' glow, the blur
of the mark's own edge -- cancels; what is left is the copy. Measured in
levels of the channel where the mark is brightest, on the reference and
on our 4K render of the same screen (our own mask from the render), 4K
native. Tools in the session scratch (`ghost-gate/tools/cc.py`,
`sites.py`), not the repository.

### What the references do

The copies are **present, at reduced strength**, not absent and not
hidden by the fill being as light as the copy. Reference against our
render before this change (ratio = reference / ours, copy 1):

| ground | sites | ground luminance | ratio |
|---|---|---|---|
| Kitsch, dark (B block, USER, GUES box, login GUEST, digits) | 6 | 0.06-0.24 | 0.43-0.68, mean 0.55 |
| Kitsch, pink field (A/C/D notes, SECURITY LEVEL, DESCRIPTION, 10:20 PM, ...) | 9 | 0.29-0.37 | 0.22-0.57, mean 0.39 |
| Kitsch, mid-teal tiles (VEHICLES, WEAPONS, LOCATIONS, PRODUCTS) | 4 | 0.41-0.46 | 0.35-0.49, mean 0.40 |
| Neo-kitsch, dark (labels, mail row, store) | 7 | 0.04-0.10 | 0.56-1.97, median 1.19 (1.07-1.26 on the dashboard labels) |
| Neo-kitsch, purple sky | 3 | 0.19-0.31 | 1.0, 1.07 at 0.19-0.21; 0.58 at 0.31 |
| Entropism, dark (store and mail text) | 3 | 0.05-0.07 | 0.52-0.77 |

In levels, VEHICLES' copy is 6.7 in the reference against 18.7 ours
(fill G 142, ink G 255). So two things, both larger than the copies' own
scatter:

1. **Kitsch's copies were about 1.8x the reference's everywhere** (dark
   ground ratio 0.55). The 2026-11-14 bloom-aware fit gave 0.28 / 0.08,
   but in Kitsch the glow overlaps copy 1 and took some of its weight the
   wrong way; the mirror measure cancels the glow. Neo-kitsch, measured
   the same way, sits at 1.07-1.26 on its dashboard labels, which checks
   the method.
   Kitsch's gains are now 0.155 / 0.044 (x 0.55).
2. **Over a mid-tone ground the reference's copies are weaker again**,
   about 0.6 of their dark-ground strength relative to the mark's own
   contrast: Kitsch 0.39-0.40 / 0.55 = 0.7 on the pink field and the
   tiles, Neo-kitsch 0.58 / 1.17 = 0.5 on the brightest sky (against its dashboard labels); at ground
   luminance up to about 0.21 there is no reduction in either era. That
   is the ground gate.

The tile outline's "dark halo" is not a darkening: the pass never lowers
a pixel's luminance on that screen (0 pixels by more than 5 levels). It
is the copy of the tile and its outline laid on the dark ground outside
the outline -- the reference has one too (G 62-68 over a ground of 44,
glowing teal) -- which at 1.8x strength, without the reference's glow
and copying our whiter outline (169, 230, 223 against the reference's
111, 237, 211), read as a grey shadow ring (G 69-79, R 46-52 over a
ground of 31-39). With the Kitsch gains corrected it is G 52-58, R 37-40
over 35-38: the same excess over its ground as the reference's. The
outline colour itself is an art difference, not the pass's.

### The ground gate

`Print::echo_ground = (lo, hi, floor)`: a copy's opacity is scaled by
`mix(1, floor, smoothstep(lo, hi, L))`, `L` the encoded luminance of the
pixel the copy lands on (the frame before any copy). Every era wears
`print::GROUND = (0.22, 0.30, 0.6)`. It is independent of the fill gate,
which still removes copies inside light fills (luminance 0.6-0.75 for
Kitsch / Neo-kitsch, measured on a ring) so dark ink on them stays firm.

After (reference / ours, copy 1, Kitsch dashboard): VEHICLES 6.7 / 6.3,
WEAPONS 5.6 / 5.4, LOCATIONS 5.4 / 4.9, PRODUCTS 8.7 / 6.2; pink-field
notes and labels ratio 0.75-1.22; dark ground (B block) 18.0 / 14.9, USER
13.5 / 11.7. Ratio median 1.06 over 13 dashboard sites (was 0.39);
mailbox, store and login sites 0.61-1.44 (was 0.22-0.55). Neo-kitsch's
brightest sky label (CUSTOMER, ground 0.31) 0.58 -> 0.80; its dark-ground
sites unchanged.

Neomil and Entropism hardly reach the gate: their grounds are darker than
0.22 nearly everywhere (Neomil 90th percentile 0.16-0.22, its red fills
already removed by its own fill gate at 0.20-0.32), and the light fills
are gated already.

### The chroma gate (Neomil)

Neomil's copies are coloured by a row texture fitted on red ink only
(crest `(1, 0.30, 0.30)`, trough `(0.58, 0.98, 0.98)` per channel). On a
neutral mark those factors make cyan: the mail client's light title tab
got pale cyan-striped copies. None of the four Neomil references has a
neutral mark to fit -- no pixel has all three channels above 90, none is
both light and low in chroma -- so there is no measured copy of one to
match. `Print::echo_chroma = (lo, hi)` fades a copy in as the copied
colour's saturation `(max - min) / max` rises from `lo` to `hi`; Neomil
wears `(0.25, 0.45)` (its red ink is 0.79), so a neutral mark has no
copies. Other eras: off. A hue-preserving texture (a luminance-only row
modulation for neutral copies) is the alternative if neutral marks should
carry copies; nothing in the references says which.



## Entropism refit; Neomil cards, Kitsch BRAINDANCE, Neomil code cells, 2026-11-15

Same model-free copy contrast as above (session scratch
`ghost-refit/tools/`: `sites.py` for marks, `fills.py` for light fills --
the median profile outside a fill's outward edge minus outside its
inward edge, so the glow cancels -- and `acf.py`, a masked
autocorrelation of the copies' texture).

### Entropism

The 2026-11-14 bloom-aware fit gave 0.36 / 0.14; as with Kitsch, the glow
had taken part of the fit. Reference / ours, 4K, channel G:

| | sites | before (0.36 / 0.14) | after (0.20 / 0.09 + texture) |
|---|---|---|---|
| copy 1, light text on dark ground (all four screens) | 20 | median 0.55 (0.41-1.18) | median 0.97 (0.63-2.18) |
| copy 2, same sites | 21 | median 0.39 | median 0.60 |
| light sage fills (T2 x2, BRAINDANCE, REPORT SPAM, mail selected row, SMG row, store card, NEXT) | 7 usable | 0.53-0.83 | 0.85-1.47, median 1.14 |

The two copies do not scale alike. Beside a fill the reference's copy 2
(the band from d1 to d2 outside the edge) is as strong as ours was
(about 0.14 of the fill's contrast), and its copy 1 only adds to about
0.2 in total; on text copy 2 is 0.39 of ours, and on thick marks (4ST,
T4) 0.7-0.86, on thin ones 0.1-0.2. So the reference's far copy is
softer than its near one and thin strokes lose it to blur; the pass has
one softness for both. 0.09 is the compromise: text copy 2 still about
1.6x the reference's, the fill bands about 0.65 of it. A per-copy
softness (the far copy blurred more) would fit both; it needs more than
the 4-tap box to blur by ~4 native px without breaking into taps, and is
not done.

Texture. The copies are broken up by a maze-like pattern. Its
autocorrelation inside the copies beside four fill edges (dashboard T2,
mail REPORT SPAM / T2 / selected row, store SMG row / card) is the same
on all three screens: a lattice with peaks near (+-2.5, 3) and (5, 0)
native px and troughs at (+-2, 0) and (0, +-3) -- two diagonal families
of pitch about 3.8 native px (1.6 dp) with normals near +-40 degrees --
and its rms is 0.16-0.24 of the copy's mean (two families: about 0.2-0.3
each). The store's left-edge copies lean "/". Drawn as one family,
`Texture { pitch: 1.6, angle: 40, crest 1.3, trough 0.7 }` per channel
(`ENTROPISM_TEXTURE`): regular diagonal hatching, not a maze, but
compared by eye at 4K (dashboard EMAILS and T2, store RIFLES / SMG, mail
rows and REPORT SPAM) it is closer to the reference than the solid
copies. At 1600x900 the 1.6 dp pitch snaps to 2 device px.

Not changed: blur (`echo_soft` 0.75) and the bloom, which is still not
drawn.

### Neomil login: no gate on the ground's redness

The premise was that Neomil hides copies on its dark red fills. Measured
on the mailbox, it does not: the message rows are dark red fills too,
and the copies of their text are at full strength there (reference /
ours, copy 1: row subjects 0.87, 1.06, 0.95; "Jackie" 1.01, 0.92; NEW
0.90). The rows' ground chroma along red (`R - max(G, B)`, encoded) is
0.11-0.17 in the reference and 0.11-0.17 in our art; the inactive
login cards' is 0.14 (card 3) to 0.25 (card 2). Card 3 and mailbox row 3
are the same colour within 2 levels, and one hides copies while the
other shows them, so no gate on the ground's colour (redness, chroma
along the copy's hue, luminance) can separate them. What hides the
copies on the inactive cards is not in the frame's colours -- most
likely the cards are composited above the printed layer -- i.e. a
per-element property. The gate was prototyped (`echo_hue_ground`, the
ground's chroma projected on the copy's) and removed again. So
`PRINT_LOGIN`'s gains were **not** raised to the free-fit 0.45 / 0.18:
with copies on the inactive cards that would worsen two notices to fix
one. Its scale was not moved either: the header rule's copies (7 and 13
native px) fix copy 1's vertical scale at 1.05 %, against which the
notices' "a pixel farther out" is within the copies' scatter.

The notice ink was #e63132 (red 228 at its peak) by oversight: it came
in with the canvas conversion with no measurement behind it, and all
three notices in the reference peak at (251, 53, 53), the line ink. It
is now `LINE_INK`. Copy contrast after (reference / ours): active
notice 33.6 / 25.1 (was 22.5), inactive notices -0.8 / 6.4 and 1.0 /
10.2 (were 4.8 and 8.4; the brighter ink copies brighter).

### Kitsch BRAINDANCE panel body: not the pass

At 8x the reference's body text (yellow on a maroon panel, about
(45, 15, 35)) has a dark olive, diagonally hatched halo hugging every
glyph on all sides, 1-6 native px out: a centred treatment, which is why
the copy-minus-mirror contrast is about 1 level (R 1.0, G 0.4; ours 11.8
and 9.4). It is not the pass's copies weakened by a gate: the same
panel's yellow outline has ordinary outward copies in the reference
(31.5 against our 20.8), and the copy of its left outline that lands
inside the panel body is there too (R 34-40 over 12). So a gate on the
copied colour's hue or luminance would remove copies the reference
keeps. What it needs is a per-element text treatment: a hatched halo
(the same diagonal maze as Entropism's copies, dark olive, about 6
native px, symmetric) on the panel's body text, and no outward copy of
that text -- i.e. the text drawn above the printed layer. Also an art
difference: our panel body is dark grey, the reference's maroon. Left.

### Neomil store scatter-code cells: left

The hypothesis "copy gain rises with the mark's brightness or
saturation" has nothing to vary on Neomil: every mark on the four
screens is the same red ink (peak (251, 53, 53)), cells and text alike.
With our current render's masks the cells' copy ratio is 1.15 (copy 1)
and 1.18 (copy 2), inside the 0.87-1.36 spread of the mailbox rows'
sites; the earlier 0.75 used a mask from the pass-on frame. No general
parameter explains it; not changed.

## Regressions from the 2026-11-16 re-review: enclosure gate, far-copy softness, measured grain

Tools in the session scratch (`print-regress/tools/`: `grain.py` high-passed
std and neighbour correlation per channel, `emb.py`, `nkprof.py`,
`glow.py`), not the repository. All numbers 4K native unless "1600"
(the reference box-downsampled to 1600x900, the goldens' size).

### Dark ink inside a light fill: the enclosure gate (Neomil login emblem)

The reference's emblem on the active card is flat black (R 17, p99 25 over
14k pixels) with no copy anywhere inside it. Ours carried copies of the
photo square (197, 42, 42; encoded luminance 0.29) and of its diagonal
stripe into the black: R p90 45, p99 76 against our flat ink of 32. Cause:
the fill gate averaged luminance on a 6 dp ring, and the hexagon is about
70 dp across, so its interior saw only black on the ring and the gate
opened; the lift gate then let the lighter copy in.

The fill gate is now an **enclosure** test (`echo_enclose`, every era
`print::ENCLOSE` = 90 dp; `echo_gate_radius` 0 retires the ring). Along
eight directions, at 13 steps out to the reach, the lightest *thick*
sample (three samples `echo_enclose_thick` = 2 dp apart, all light, so a
thin line of ink is not a fill) is found; the least of the eight is how
completely a light fill encloses the pixel. Copies are suppressed as it
rises from `echo_gate.0` to the middle of `echo_gate`. It runs only where
a copy changed the pixel. Dark ink inside a fill stays firm at any size;
ground outside a fill is open in some direction and keeps its copies --
including right up to the fill's own edge, where the ring used to cut
them short (that cut made the hard-edged strip beside Neo-kitsch's bars).

After: emblem R p50/p90/p99 32/33/34 at 4K, 32/32/33 at 1600 (was
32/45/76 and 32/49/77): the ink is flat. Neomil's other dark-on-red marks
(dashboard tile codes, mailbox selected row, store MASURAO bar) are as
firm as before by eye at 1600. Not the pass: our emblem ink is (32, ...)
where the reference's is 17, an art difference.

### Grain

The grain cell was clamped up to one device pixel, so at 1600x900 a
0.8 dp Entropism cell (1.9 native px) became 2.4 native px of interpolated
value noise: neighbour correlation 0.43 and the full 4K std, where
downsampling the reference averages its grain to near-white and ~0.7 of
its 4K std. Now the cell has no clamp and the noise is integrated over
each device pixel on an m x m grid (`grain_taps`: m = ceil(1.5 / cell),
1-4), which is what downsampling the print does. Grain is per channel
(`grain_mul`, `grain_add: [f32; 3]`).

Measured (high-passed std, levels; neighbour correlation a1):

| | reference 4K | ref 1600 | before 4K | before 1600 | after 4K | after 1600 |
|---|---|---|---|---|---|---|
| Entropism sage R/G/B | 3.54/4.75/7.05, a1 0.5 | 2.57/3.27/4.56 | 4.51/5.43/4.56, a1 0.7 | 4.31/5.20/4.35, a1 0.43 | 3.33/4.57/6.87, a1 0.53 | 2.32/3.18/4.79, a1 0.11 |
| Entropism dark ground | 0.73/0.65/0.56 | 0.58/0.50/0.45 | 1.16/1.00/0.90 | 1.11/0.96/0.86 | 0.75/0.68/0.68 | 0.57/0.51/0.50 |
| Neomil dark ground | 1.06/1.20/1.74, a1 0.15 | 0.63/0.69/0.93 | 1.05/1.06/1.06 | 0.99/0.98/0.99, a1 0.4 | 1.10/1.24/1.76, a1 0.24 | 0.65/0.71/0.98 |
| Kitsch pink field | 3.38/1.77/4.22 | 2.53/1.28/2.76 | none | 0.29/0.26/0.25 | -- | 2.30/1.28/2.79 |

Kitsch, Neo-kitsch and Entropism measure as **one grain**: std
`mul * v + add` with mul about (0.021, 0.024, 0.046) -- blue twice the
others -- and add about 0.5 levels (Kitsch pink and ENTER bar, Neo-kitsch
sky and ENTER bar, Entropism sage and ground all fit within ~10 %). They
share `print::GRAIN_MUL` / `GRAIN_ADD` / `GRAIN_CELL` (0.55 dp); Kitsch and
Neo-kitsch had no grain at all before. Neomil's is additive only (its red
tiles are flat), blue strongest: `GRAIN_ADD` (0.0052, 0.0059, 0.0085) on
a 0.33 dp cell. The stored values are the noise field's; integration
lowers them ~0.9x (0.8x for Neomil) at 4K, which the fit includes.

The reviewer's "horizontal striations" on Entropism's dark ground are not
in the grain: row means vary 0.037 levels against 0.026 for white noise
(columns 0.048 / 0.038). Not modelled. The sage's faint vertical streaks
(column means ~1.9x white) are not modelled either. The TV fitting was
not rendered; the pass's design-to-device scale comes from the target
size, so its grain sits as at 4K.

### Far copy softness (Entropism captions, Neo-kitsch bar ends)

Beside Neo-kitsch's right ENTER bar (median over the bar's middle rows,
copy side minus the mirror side at the bar's left end, every 2 px):
reference 24 28 22 26 24 23 18 10 12 10 9 2 4 2 0 -- the copies reach
22-26 px; ours ended at 16 px with a crisp outer edge (48 38 33 23 23 18
11 1 0), the "detached sliver". The reference's far copy is softer than
the near one; with `echo_soft_far` = `print::SOFT_FAR` = 1.7 dp (4 native
px; drawn as a 4x4 tap grid above 1.6 device px) ours tapers to 18 px:
46 40 30 22 17 15 11 8 6 -1. It also matches the 2026-11-15 finding that
the far copy keeps its strength beside fills but falls to 0.1-0.2 on thin
strokes, so Entropism's far gain goes back up to the fill-band fit, 0.09
-> 0.14. Kitsch and Neo-kitsch wear the same softness; Neomil does not
(its two copies of the header rule measured equally soft).

Not fixed: Neo-kitsch's near copy at the bar end is still ~1.7x the
reference's (46 vs 24-28 in the first 6 px), as the reviewer said of the
T2-T4 header labels; the 2026-11-15 measurement on its dashboard labels
gave the opposite (reference 1.07-1.26 of ours), so the gain was not cut
on one site. The reference's halo there is mostly bloom (below). The
Entropism 7-8 px captions changed little at 1600: our two caption lines
sit closer and the glyphs are taller and bolder than the reference's,
which is the art, not the copies.

### Not done in this pass

- Neomil header badge's legible second "T1"/"LEVEL" (re-review item 5):
  done later the same day, "Above the print, and the bloom" below.
- Kitsch copies grey vs the reference's mint halo (item 6): it is the
  bloom, which glows the mark's chroma; drawn since, same section.

### Bloom: what it would take

(Superseded by "Above the print, and the bloom" below: re-measured per
channel, the glow is the mark's chroma, with sigma 6-7 native px.)

Measured as the median profile away from a light fill's edge on the side
its copies do not reach (so it is the glow alone), relative to the fill's
contrast over the ground, fitted with a blurred half-plane
`g * erfc(x / (sigma sqrt 2)) / 2`:

| era | site | rel. excess at 2 / 4 / 8 / 12 px | gain g | sigma (native px) |
|---|---|---|---|---|
| Neo-kitsch | ENTER bar, left end | 0.23 / 0.17 / 0.08 / 0.03 | 0.33 | 5.6 |
| Entropism | login footer band, top edge | 0.36 / 0.14 / 0.06 / 0.01 | 0.24 | 3.9 |
| Kitsch | ENTER bar, right end (every 3 px: 0.31 / 0.15 / 0.07 / 0.02) | -- | ~0.3 | ~4 |
| Neomil | active card, right edge | 0.004 everywhere | 0 | -- |

So Neomil has none and the other three a glow of gain about 0.25-0.35 and
sigma 4-6 native px (1.6-2.3 dp), hatched by the same diagonal maze as the
copies (visible in the Neo-kitsch and Entropism crops). Implementation: a
separable Gaussian of the frame's light part (`max(0, L - threshold)` or
the frame itself, screened): two extra full-screen passes (horizontal,
vertical) into one intermediate texture, at half resolution to keep 4K
cheap (sigma 2-3 half-res px, 9-13 taps per pass), then added in the
print pass as `out = 1 - (1 - out)(1 - g * glow * texture)` with the
era's diagonal texture. Cost on lavapipe, extrapolated from the pass's
measured 347-422 ms at 4K: a half-resolution 13-tap separable blur is
about 2 x 13 / 4 = 6.5 taps per output pixel, against the print pass's
~13-30, so roughly +100-150 ms at 4K and +20-30 ms at 1600. Parameters
per era: gain, sigma, threshold. It would also restore Entropism's mean
level in text regions (mail body G 38 vs 46) and Kitsch's mint halos.

### On by default, 2026-11-14

Who wears it:

- `cp-eras-ui-login`, `-mailbox`, `-store`, `-dashboard`: their screen's
  fit (`print::wear_screen`). The dashboard binary's hub re-wears on each
  route, so the mailbox and the store reached from it wear their own fits
  (`screens::hub`, `Hub::wear`), as their own binaries do.
- `cp-eras-ui-mail` (the working mail client): the era's treatment for a
  new screen, `print::of` (Neomil: the mailbox fit).
- Not the status bar (`cp-eras-ui-bar`, `-bar-window`): a live desktop
  surface designed separately, with its own rule echo and the
  `PanelEcho` menu motifs. Its render is unchanged byte for byte.
- `CP_ERAS_UI_PRINT=0` turns the pass off in any of them.

### Per-element copies removed (they doubled the pass)

With the pass on, these era-drawn copies were drawn as well as the pass's
own (Neomil dashboard marker boxes showed four copies instead of two).
Each was checked at 4K against the reference with the pass on; all are
radial registration copies the pass reproduces, so all were deleted:

- Neomil dashboard: `neomil_dashboard_{margin,header,tape_chip,label,brand}_echoes.rs`,
  `neomil_dashboard_echoes.rs` (`PANEL_ECHOES`),
  `neomil_dashboard_panel_printing.rs` (`PRINTING_ECHOES`, the whole file),
  `HEADING_ECHOES` in `..._panel_primary.rs`, and the footer
  `DASHBOARD_FOOTER_{FRAME,TEXT}_ECHO_{1,2}`. The dashboard now has four
  software layers instead of eight.
- Neomil login: `neomil_login_echoes.rs` (`HEADER_ECHOES`) and the notice
  copy (`NoteEcho`, `Slot::reference_note_echo`, `src/screens/login/echo.rs`).
  Its offset (-2.5, +2.5) dp was exactly copy 1 of `PRINT_LOGIN` at the
  notes' position.
- Neomil store: `neomil/store_echo.rs` (scatter-code trails: "directional"
  only in that they point away from the centre), `store_title_echo.rs`,
  `store_frame_echo.rs`, and the nav rows' `NAV_ECHO` (measured -8.8/-3.2
  and -22.7/-7.5 native px; `PRINT_STORE` puts copies at about -8.4/-5.2
  and -22.5/-11 there: the same across, a few px more up). With them gone
  the store's feedback states equal `STORE_STATES` again.

What looks different from the hand-drawn copies, by eye at 4K:

- Store scatter-code cells: by eye the reference's trails are a fairly
  continuous smear and the pass gives two discrete, fainter steps. Measured
  2026-11-15 (median red profile leftward from the left edge of 7 cells,
  levels above a ground of 25): the reference has steps too -- about 82
  for 4 px, then about 22 to px 13, then about 8 to px 21 -- and the pass
  gives 77 for 3 px, about 16 to px 11, about 7 to px 21. So the shape is
  the pass's (copy 1 then copy 2); the middle step is about 0.65 of the
  reference's and the near one ends a pixel early. Copy contrast over the
  cells: reference 66.5, ours 49.9 (0.75). Not changed: matching it needs
  more copy for these cells only (about 1.4x the store fit), i.e. a
  per-element weight the pass does not have; raising the store gains would
  overshoot the 32 sites they were fitted on.
- Login notices: the copies are fainter than the reference's. Measured
  2026-11-15 on the notice under the active card: copy 1 contrast 33.6
  levels in the reference, 22.4 ours (0.67); copy 2 23.1 / 13.8. Part is
  the ink: the reference's notice peaks at red 251, ours at 228 (0.91).
  A free fit puts the reference's copies at 0.45 / 0.18 of the ink and a
  pixel farther out ((-5.2, 7.5) native against the login fit's (-4.2,
  6.4)), ours at 0.38 / 0.15. But the notices on the two inactive cards
  (on the dark red card fill) have **no** copies in the reference
  (contrast -0.8 and 1.0) while ours have 4.7 and 8.4: the Neomil
  references hide copies on their red fills, dark ones too, not only on
  the bright tiles. So raising `PRINT_LOGIN`'s gains would fix one notice
  and worsen the other two; not changed. (A gate on the ground's redness
  was tried 2026-11-15 and does not work: the mailbox rows are the same
  dark red and keep their copies; see "Neomil login: no gate" above. The
  notice ink is now 251.)
- GO HOME vertical brands: the pass draws copies to their right that the
  reference barely shows (the hand-drawn ones were similar).
- Gained: copies the hand drawing never had and the reference does (store
  nav labels AUDIO / GAMEPLAY, login PROTOCOL glyph and tape, HAND GUN).

Kept, as designed motifs rather than registration copies: the bar's
`PanelEcho` (`EdgeBar`, `Rings`, `Wave`; the bar does not wear the pass),
and the mail `MailRowEcho` hover / pressed ghosts (Kitsch: one copy lifted
20 dp up-right; Neo-kitsch: seven nested rings), which are feedback-state
drawings at offsets the pass does not produce.

Evidence (native 4K crops, 4-10x zooms, profiles, model diagram) was
written outside the repository; no reference pixels are committed.

## Above the print, and the bloom, 2026-11-16

Tools in the session scratch (`print-bloom/tools/`: `badge.py` zone
means, `halo.py` ring colour by distance from ink, `hfft.py` hatch
spectrum, `gprof.py` per-channel glow profile, plus `ghost-refit/tools/cc.py`
for copy contrast), not the repository. 4K native unless "1600".

### Elements above the printed layer

Three places in the references show copies the frame's colours cannot
explain (each was measured and no gate on luminance, chroma or hue
separates them; see the sections above): Neomil's header badge plates
(nothing of T1 / LEVEL doubled inside the plate, while the plate's own
ghost shows outside it), Neomil's inactive login cards (notices without
copies, while mailbox rows of the same colour keep theirs), and Kitsch's
BRAINDANCE body text (no outward copy; a hatched olive halo instead).
They are elements laid *after* the print. That is a property of the
element, so it is stated per element.

**Mechanism: a short list of shapes in the pass's uniforms.**
`print::AbovePrint { x, y, w, h, bevel, cast, halo }` -- a rectangle with
45-degree corner cuts in design px, the shape of a `Plate` -- up to
`MAX_ABOVE` (12) per screen, carried on the material (`Print::above`) and
sent as three `vec4`s each. In the pass:

- a copy landing inside any region is dropped (antialiased over one
  device pixel): the element covers the print;
- a copy whose *source* lies in a region with `cast: false` is dropped:
  the element casts nothing (text set over everything). With `cast: true`
  (a plate printed underneath as well) its own copies still go outward;
  for those the lift gate reads the channel in which the copy is lightest
  instead of luminance, because a dim plate on a ground of about its own
  luminance (badge plate 0.20 against the glow's 0.18) otherwise loses its
  ghost to the gate -- the reference shows it plainly;
- `halo` (per channel, the fraction of the way from the ground to the mark
  at the mark's edge) draws a hatched halo hugging the light marks inside
  the region: the lightest neighbour on four rings (8, 8, 12, 16 taps at
  0.4 / 0.9 / 1.5 / 2.2 `halo_sigma`), Gaussian in distance, hatched by
  the material's `hatch` at `halo_hatch` depth. Only region pixels pay.

Why not a mask render target the scene writes: iced's canvas buckets each
layer's geometry into meshes, images and text and draws them in its own
order through its own pipelines; a second colour attachment would mean
patching every pipeline (quad, triangle, text, image) to write a coverage
channel, and the screens' canvases cache geometry, so a flag on a draw
call would not be re-issued each frame. A reserved alpha channel fails the
same way (the frame's alpha is the surface's). The regions are few, large
and static per screen, and their geometry is already data on the era
tables, so uniforms are the simplest thing that is correct; the cost is a
loop over at most 12 regions on pixels a copy touches.

**How a screen uses it.** List the elements beside their data and attach
them to the screen's material:

```rust
const LOGIN_ABOVE: &[print::AbovePrint] = &[
    print::AbovePrint::plate(&LOGIN_BADGE_CUSTOMER),   // casts its ghost
    print::AbovePrint::plate(&LOGIN_CARD_2),
];
const PRINT_LOGIN: Print = Print { above: LOGIN_ABOVE, ..PRINT_MAILBOX };
// or, for a text block: AbovePrint::block(x, y, w, h).with_halo(Halo { gain })
```

`AbovePrint::plate` reads the plate's own `Plot` and `Bevel`, so the
plate is defined once (the login's badges and inactive cards are now named
constants used by both the drawing and the list). `print::of_screen`
returns the screen's list; `Print::above(list)` attaches one to any
material, e.g. `print::of(era).above(MY_SCREEN_ABOVE)` for a new screen
worn through `print::wear`'s internals. `CP_ERAS_UI_PRINT_SET="above=off"`
drops them for measuring. Limits: rectangles with corner cuts only (no
rounded or free paths), in the design frame, static per screen -- an
element that moves needs its screen to restate the material.

Applied (2026-11-16):

| Case | Measure | Reference | Before | After |
|---|---|---|---|---|
| Neomil login, customer badge (T1) | red mean / max, up-left of T1 inside the plate | 69 / 79 | 85 / 155 | 66 / 69 |
| | plain plate | 66 / 69 | 66 / 69 | 66 / 69 |
| | outside the plate, up-left (its ghost) | 47 / 77 | 38 / 46 | 42 / 63 |
| Neomil login, inactive card 2 notice | copy 1 / copy 2 contrast, levels R | -0.8 / -0.8 | 5.6 / 5.7 | -0.1 / -0.1 |
| inactive card 3 notice | | 1.0 / -1.0 | 10.5 / 11.0 | -0.1 / -0.1 |
| active card notice (not a region) | | 33.6 / 23.1 | 25.0 / 16.0 | 25.0 / 16.0 |
| Kitsch BRAINDANCE body text | copy 1 contrast R / G | 1.0 / 0.4 | 11.2 / 9.2 | 0.2 / 0.2 (1.4 / 1.0 with the bloom on) |
| same panel's outline (not a region) | copy 1 contrast R | 31.5 | 21.3 | 20.4 |

All five login badges are regions (the four security badges too: the
same plates). The rest of each frame is unchanged: on the Neomil login
the pixels that differ lie only in the five badges and the two inactive
cards (the active card and its emblem untouched); on the Kitsch dashboard
only inside the text block.

The Kitsch halo, measured on the eight lines (2920..3432 x 880..1240):
excess over the ground (17, 11.5, 11.5) by distance from the ink's edge,
R / G levels: reference 30 / 18.5 at 2-3 px, 22 / 12.4 at 3-4, 15 / 7 at
4-5, 8 / 2.7 at 5-6, gone by 8; ours 34 / 20, 22 / 13, 17 / 10, 7 / 4.
Hatch: G std in the ring 14.4 reference, 15.5 ours; pitch 3.96 native px
(1.65 dp), normal at -45 degrees (the spectral peak). `Halo { gain:
[0.34, 0.25, 0.08] }`, `halo_sigma` 1.4 dp, `halo_hatch` 0.75. By eye at
5x the reference's hatching is less regular (a broken maze) than our
straight lines; the strength and reach match.

### Bloom

Re-measured on the side of a light fill the copies do not reach (the side
facing the screen centre), per channel, as the excess over the ground
relative to that channel's fill-to-ground contrast, every px from the
edge (`gprof.py`):

| Era, site | R | G | B |
|---|---|---|---|
| Neo-kitsch login, right ENTER bar, left end (2..12 px) | .25 .22 .19 .17 .14 .12 .10 .07 .06 .05 .03 | .22 .18 .16 .13 .12 .10 .08 .06 .04 .03 .02 | .04 .03 .02 .02 .01 0 |
| Neo-kitsch login, left bar, right end | the same within .02 | | |
| Kitsch login, ENTER bar, right end (2..11) | 0 | .19 .17 .14 .12 .09 .07 .05 .03 .01 0 | .20 .17 .14 .11 .08 .06 .05 .03 .01 0 |
| Entropism login, footer band, top edge (3..12) | .02 .01 0 | .17 .15 .12 .10 .07 .06 .04 .02 .01 .01 | .02 .01 0 |

So the glow is not the mark's colour: it is its **chroma**. Kitsch's mint
bar (124, 246, 216) glows green and blue with no red, Entropism's sage
(135, 170, 138) green alone, Neo-kitsch's amber (255, 195, 112) red and
green with no blue -- in each, the colour less its grey part `min(r, g,
b)`. That is the mint the Kitsch reviewer saw where our copies were grey:
it is the glow, not the copies. Fitted as a half-plane through a
Gaussian, `gain * chroma * erfc(x / (sigma sqrt 2)) / 2`: Neo-kitsch sigma
about 7 native px (2.9 dp), gain about 1.1-1.2 of the chroma; Kitsch sigma
about 6 (2.5 dp), gain about 1.0, blue a little stronger than green;
Entropism sigma about 7 (2.9 dp), but its sage has so little chroma (35
levels of G) that the chroma model needs gain 2.3, and at that gain the
store's yellow PETROCHEM / BETTERLIFE bands (chroma 200) blazed and their
dark tag text was lost under the glow (seen at 1600; the reference's bands
have no such glow). Entropism therefore glows green whatever the colour:
`tint (0, 1, 0)`, chroma 0, gain 0.4 of the colour itself. Kitsch went
the same way: its yellow BRAINDANCE tab (left edge, the side copies do not
reach) glows G .19 .16 .13 and R .11 .08 .06 of its contrast at 2-4 px --
G about as strongly as the mint bar, not in proportion to its larger
chroma -- and the chroma model at the mint's gain put 2.3x that on the
yellow and smeared the BRAINDANCE body text. Kitsch therefore glows its
colour with red held down: `tint (0.25, 1, 1)`, chroma 0, gain 0.55. The
chroma model stays for Neo-kitsch, where it fits (no blue in the glow). The earlier figures (gain 0.24-0.33, sigma
3.9-5.6) were fits of the luminance with a free edge offset; they are
superseded. Neomil: none (0.004).

The pass (`vendor/iced_wgpu`, `shader/bloom.wgsl`): three extra
half-resolution passes into two `Rgba16Float` targets -- threshold and
2x2 downsample (each full-resolution pixel weighted by `smoothstep(knee)`
of its encoded luminance before averaging, so thin light strokes glow),
then a separable Gaussian across and down (taps out to three sigmas, at
most 24 each side) -- and in the print pass, after the copies and the
halo and before scanlines and grain, the glow sampled bilinearly, hatched
and screened: `out = 1 - (1 - out)(1 - gain * glow * tint * (1 + hatch
cos))`. The glow lies *around* light parts, not on them: it is held off
a pixel that is itself light, and off a pixel mostly surrounded by light
(the blurred light fraction, carried in the glow's alpha, above 0.5-0.7),
so a fill and the dark ink on it (Entropism's NEXT, Kitsch's ENTER) keep
their colour, as in the references. A first version without that hold
turned Entropism's NEXT button bright green and greened its letters.
Inside an element above the print there is no glow either (its own, or
neighbours'): Kitsch's BRAINDANCE text has its halo and no glow.

Parameters (`print::Bloom` on the era's `Print`; `print::GLOW` the shared
knee 0.35-0.55 and chroma 1):

| era | gain | sigma (dp) | tint | hatch |
|---|---|---|---|---|
| Neo-kitsch | 1.0 | 2.9 | 1, 1, 1 | 0.25, default hatch (1.6 dp, -45) |
| Kitsch | 0.55, of the colour (chroma 0) | 2.5 | 0.25, 1, 1 | 0.25, the halo's hatch (1.65 dp, -45) |
| Entropism | 0.4, of the colour (chroma 0) | 2.9 | 0, 1, 0 | 0.25, 1.75 dp at -42 (the footer glow's spectral peak, weak) |
| Neomil | 0 | -- | -- | -- |

Sigma is in design px, so it holds at 1600x900, 4K and the TV fitting.
`CP_ERAS_UI_PRINT_SET="bloom=off"` removes the glow alone;
`bloom_gain`, `bloom_sigma`, `bloom_knee`, `bloom_tint`, `bloom_hatch`,
`bloom_chroma`, `hatch=pitch,angle` tune it; `CP_ERAS_UI_PRINT=0` still
turns everything off.

After (same sites, ours at 4K, relative excess every px from 2):

| | channel | reference | ours |
|---|---|---|---|
| Neo-kitsch bar | R | .25 .22 .19 .17 .14 .12 .10 .07 .06 | .26 .26 .23 .18 .13 .10 .09 .08 .07 |
| Kitsch bar | G | .19 .17 .14 .12 .09 .07 .05 .03 .01 | .20 .16 .13 .11 .08 .06 .04 .03 .01 |
| | B | .20 .17 .14 .11 .08 .06 .05 .03 .01 | .20 .16 .14 .11 .08 .06 .05 .03 .01 |
| | R | 0 | .05 .05 .05 .03 .02 .01 0 |
| Kitsch BRAINDANCE tab (yellow), left edge | G | .19 .16 .13 .10 .08 .06 .04 .03 .02 | .24 .19 .17 .14 .11 .09 .06 .05 .04 |
| | R | .11 .08 .06 .04 .04 .03 .03 .01 0 | .05 .05 .04 .04 .03 .02 .01 .01 .01 |
| Entropism footer | G (from 3) | .17 .15 .12 .10 .07 .06 .04 .02 .01 | .15 .13 .11 .09 .07 .06 .05 .04 .02 |

Neo-kitsch's is 1.1-1.2x in the first 6 px; Kitsch's yellow 1.2x in G and
half the reference's red (one tint cannot hold both the mint's zero red and
the yellow's), its mint on the reference; Entropism's is 0.9x near and its tail runs a few
px longer. By eye at 4K the three read like the references' glows (soft,
hue-carrying, faintly hatched), not like a blur of the frame.

**Copies refitted with the bloom on (Neo-kitsch).** The outward profile at
the right ENTER bar's end, every 2 px (copy side; glow and copies
together): reference 68 62 49 46 38 34 28 20 22 20 18 10 13 11 9; ours
before the bloom 65 53 40 31 26 24 20 17 15 9; with it 86 71 53 39 31 27
22 18 15 10. The mirror side (glow alone) now matches (reference 44 34 27
20 14 11 10; ours 47 35 22 21 15 12 12), so what remains is the copies:
near 1.5x too strong, far too short. Copy gains 0.17 / 0.17 (from 0.21 /
0.12) were tried: the bar end's copy part went to 39 38 36 26 23 20 15 11
7 (reference 24 28 22 26 24 23 18 10 12 10 9) -- the far band better, the
near no lower -- and the dashboard labels' copy-1 ratio (reference / ours)
from median 1.10 to 1.25 (dark-ground labels 1.25-1.57). Not adopted: the
copies are unchanged at 0.21 / 0.12. The reference's near copy beside a
fill is weaker than beside text, which one gain cannot fit (the same
finding as Entropism's far copy, 2026-11-15).

### Cost

The pass alone on lavapipe (llvmpipe, LLVM 21, this 32-thread machine
otherwise idle), best of 8 after 2 warm-up frames, timed by the ignored
test `print::bench::time_print_pass` (`iced_wgpu::print::bench`; how to
run it is in its doc comment) on captures of each screen. "Neither" is the
material with `bloom=off;above=off`, i.e. the pass as it was before:

| screen | size | neither | + regions | + bloom | as worn |
|---|---|---|---|---|---|
| Neo-kitsch login (bloom) | 1600x900 | 20.0 | -- | 23.0 | 23.0 |
| | 3840x2160 | 130.0 | -- | 148.6 | 148.6 |
| Entropism login (bloom) | 1600x900 | 19.4 | -- | 22.9 | 22.9 |
| | 3840x2160 | 132.5 | -- | 152.1 | 152.1 |
| Kitsch dashboard (bloom, 1 halo region) | 1600x900 | about 20 | 22.7 | 23.4 | 25.6 |
| | 3840x2160 | about 108 | 115.4 | 123.0 | 130.8 |
| Neomil login (7 regions, no bloom) | 1600x900 | 18.9 | 24.5 | -- | 24.9 |
| | 3840x2160 | 71.6 | 102.1 | -- | 101.0 |

(ms; "about" where only the two single-off runs exist. The Kitsch row was
timed before its glow moved to the tint model and was kept off the
regions; the passes run are the same.) The bloom costs
about 3 ms at 1600x900 and 15-20 ms at 4K, well under the proposal's
estimate. The regions cost more than expected on Neomil's login, +30 ms at
4K: the shader tests every region for every copy of every pixel (two
point-in-region tests per copy, seven regions). A bounding-box test or a
coarse tile list would remove most of it; not done. The 2026-11-14
figures (347-422 ms at 4K) were taken under heavy unrelated load and are
not comparable.

## The pass (2026-11-13 prototype; superseded where the section above says)

Per pixel, in encoded sRGB: two copies of the frame scaled about the
centre by `1+2k` then `1+k`, each a 4-tap box (softness), lighten-composited
at alpha x row cosine x a fill gate (encoded luminance on a 6 dp ring: a
copy is suppressed inside light fills so dark ink stays firm); then an
optional darkening scanline multiply; then bilinear value-noise grain,
multiplicative plus additive. Pitches are in design px converted per axis,
snapped to whole device pixels below 3 px (2 px at 1600x900: no moire).
Zero strengths skip the pass and the frame is byte-identical.

Fitted values are the `PRINT` constants. Neomil: k 0.0055, alpha
0.24/0.14, copy rows depth 0.6 at 1.985 dp, soft 1.5 dp, no fill
scanlines, grain 1 level. Entropism: k 0.008, alpha 0.32/0.18, depth 0.3 at
3.33 dp, soft 2 dp, grain 2.8 % + 0.75 levels at 0.8 dp cells.

Tuning knobs (read once at start): `CP_ERAS_UI_PRINT=<strength>`,
`CP_ERAS_UI_PRINT_SET="key=v;key=a,b"` (`Print::with_overrides`).

## Limits

- Bloom since 2026-11-16 (Kitsch, Neo-kitsch, Entropism; Neomil none):
  the glow's hatching is straight lines where the references' is a broken
  maze; its gains rest on one or two sites per era.
- Non-Neomil eras: the copies are broken by a diagonal pattern the pass
  draws only for Entropism (one family of lines). Kitsch and
  Entropism gains corrected 2026-11-15 (Entropism also wears a one-family
  diagonal texture for the maze). The far copy is softer than the near
  one since 2026-11-16 (`SOFT_FAR`); Neo-kitsch's near copy beside its
  bars is still ~1.7x the reference's.
- The enclosure gate costs up to 8 x 13 x 3 extra samples on pixels a copy
  changed (early exits make the usual case one ray); the grain up to 16
  taps per pixel at 1600x900. Not re-timed.
- Elements above the print (Neomil login badges and inactive cards,
  Kitsch BRAINDANCE body text) are listed per screen as rectangles with
  corner cuts, static: an element that moves needs its screen to restate
  the material. The Neomil mailbox and dashboard badges are not listed
  (not measured there). Kitsch's BRAINDANCE panel body is dark grey where
  the reference's is maroon (art).
- Neomil copies of a neutral mark are dropped, not drawn (chroma gate):
  the references have no neutral mark to fit.
- Entropism mean colour rises 1-3.6 levels (G) against a reference ~1-2
  above the goldens: its frame strokes were already brighter than the art.
- Lavapipe cost of the pass alone (timed 2026-11-14 with a throwaway
  wgpu test on llvmpipe, a 32-thread machine under heavy unrelated load,
  best of 8): Neomil 68 ms / Entropism 98 ms at 1600x900, 347 / 422 ms at
  3840x2160, against 4 / 8 ms for a bare clear pass. The 2026-11-13 figure
  was +~45 / +~350 ms. Real-GPU cost unmeasured.
- `Renderer::screenshot` and the tiny-skia backend do not run the pass.
