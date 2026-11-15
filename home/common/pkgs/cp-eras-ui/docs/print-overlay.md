# Print overlay, 2026-11-13 (copies remeasured, and on for all eras, 2026-11-14; ground and chroma gates, Entropism refit 2026-11-15)

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

Overrides: `CP_ERAS_UI_PRINT_SET="echo1=kx,ky,gain;echo2=...;echo_centre=x,y;echo_blend=over|lighten;echo_soft=s;echo_lift=lo,hi;echo_ground=lo,hi,floor;echo_chroma=lo,hi;tex_pitch=p;tex_angle=a;tex_crest=r,g,b;tex_trough=r,g,b;tex_phase=f;..."`.

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

- No bloom (needs a separable blur, i.e. more passes). Entropism text
  regions stay darker than the reference (mail body mean G 38 vs 46).
- Non-Neomil eras: the copies are crisper than the reference's, which
  are broken by a diagonal pattern and wrapped in bloom the pass does not
  draw. Kitsch and Entropism gains corrected 2026-11-15 (Entropism also
  wears a one-family diagonal texture for the maze). Entropism's far copy
  is still about 1.6x the reference's on thin text and 0.65x beside light
  fills: one softness for both copies.
- Kitsch's BRAINDANCE panel body (yellow on dark): the reference's
  copies are dark, hatched and nearly symmetric (copy contrast 1 level);
  ours are solid (12 levels after the gain change). A centred hatched
  halo around the text, not the pass; needs a per-element treatment (see
  above). Not modelled.
- Neomil's inactive login cards hide copies in the reference (notices
  -0.8 and 1.0 levels against ours 6.4 and 10.2); not a property of the
  ground's colour, so not gated.
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
