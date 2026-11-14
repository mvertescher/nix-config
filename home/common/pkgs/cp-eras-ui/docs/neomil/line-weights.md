# Neomil line weights

Measured 2026-11-14 on the four native references (`images/img-06..09`,
3840x2160, lossless) and on this crate's own 3840x2160 renders of the same
screens, before and after the corrections below. Design px (dp) = native
px / 2.4 (the traces' 1600x900 frame). The Rust is `eras::neomil::WEIGHTS`
and the dark-plate treatments beside it; every number there cites this file.

## Method

For each straight run of an element's outline, 40 profiles perpendicular to
the line, spread along its middle 76%. Axis-aligned runs are sampled on raw
pixels (no interpolation); the 45-degree diamond insets with bilinear
sampling at 0.25 px, which adds blur, so they are compared against this
crate's own render measured the same way rather than read absolutely. Per
profile: the peak (red channel; inverted for dark-on-red), a base on each
side 4-5 px out (so a line between two different fills uses each fill), the
FWHM between the two half-level crossings, and the integrated width (area
above the base / peak height). Our renders were registered against the
reference on the mailbox row edges, header badges, tile insets and panel
edges: no global offset (below 0.1 native px); every position difference
below is the element's own geometry.

Error: on a hard 2 px line the FWHM is 2.0 whatever its sub-pixel phase, and
a 1 px line would read 1.0-2.0 depending on phase; the dim outlines read
1.86-2.10 at phases spread across the pixel, so they are 2 px lines, not
half-intensity 1 px ones. Spread between edges of one family is +-0.15 px
except where an edge sits against a different-coloured neighbour or carries
a printed echo inside the base window (noted). Lines in our renders are not
pixel-snapped, so the same 0.8333 dp line reads 2.0-2.5 px FWHM depending on
where it falls; compare intW and colour as well as FWHM.

## What the references draw

| family | n | ref FWHM px (range) | ref dp | ref intW px | ref peak (range of R) | before FWHM px / dp | after FWHM px / dp | before ink | after ink |
|---|---|---|---|---|---|---|---|---|---|
| store nav row | 16 | 2.04 (1.94-2.38) | 0.848 | 2.20 | #6a1111 (R 62-76) | 2.92 / 1.217 | 2.21 / 0.923 | #a32226 | #5e1112 |
| store card | 4 | 1.68 (1.64-1.97) | 0.700 | 1.88 | #f43434 (R d5-fb) | 2.02 / 0.842 | 2.02 / 0.842 | #fb3535 | #fb3535 |
| mailbox row | 14 | 1.92 (1.90-1.99) | 0.800 | 2.06 | #5f0809 (R 5b-66) | 2.00 / 0.833 | 2.00 / 0.833 | #660e0f | #660e0f |
| mailbox button | 12 | 1.96 (1.86-2.10) | 0.817 | 2.12 | #580807 (R 1c-5d) | 2.79 / 1.163 | 2.32 / 0.967 | #a32226 | #5e1112 |
| mailbox panel | 3 | 1.86 (1.82-4.35) | 0.775 | 2.16 | #fb3535 (R e8-fb) | 2.01 / 0.837 | 2.01 / 0.837 | #fb3535 | #fb3535 |
| dashboard GO HOME panel | 4 | 2.04 (1.92-2.26) | 0.852 | 2.16 | #f82727 (R ee-f8) | 4.20 / 1.748 | 2.53 / 1.054 | #ef3333 | #ef3333 |
| dashboard tab plate | 8 | 2.00 (1.87-2.41) | 0.831 | 2.23 | #672333 (R 5c-6b) | 2.29 / 0.954 | 2.29 / 0.954 | #68232e | #68232e |
| dashboard footer tape | 4 | 3.76 (3.60-3.82) | 1.569 | 3.67 | #fb3535 (R fb-fb) | 3.22 / 1.342 | 3.52 / 1.469 | #ef3333 | #ef3333 |
| dashboard header badge | 10 | 2.10 (1.90-2.21) | 0.875 | 2.32 | #6e203a (R 6b-9e) | 1.94 / 0.808 | 1.94 / 0.808 | #701d39 | #701d39 |
| login idle card | 8 | 1.82 (1.75-2.17) | 0.758 | 2.23 | #7d212b (R 2c-9f) | 3.71 / 1.546 | 2.08 / 0.869 | #a32226 | #a32226 |
| login password field | 4 | 1.83 (1.59-1.98) | 0.760 | 2.25 | #780d0e (R 75-b2) | 2.77 / 1.154 | 2.00 / 0.833 | #a32226 | #760e0f |
| tile inset (dark on red) | 24 | 2.46 (2.23-2.66) | 1.025 | 2.45 | #7b191a (R 3c-81) | 2.14 / 0.892 | 2.14 / 0.892 | #5f1112 | #5f1112 |
| tile seam (dark on red) | 6 | 2.15 (1.61-2.67) | 0.896 | 2.13 | #7a1b1b (R 4b-82) | 2.22 / 0.927 | 2.22 / 0.927 | #5f1112 | #5f1112 |

(n = edges measured; "before"/"after" = this crate at 4K. Peak = most common
peak colour; R range = the red channel's spread across edges.)

Observations, family by family:

- Every outline is a **hard 2 native px line**: the two core pixels at the
  line colour, the neighbours at the fills, no antialiased skirt, no glow or
  halo (the base 4-10 px out equals the base 15 px out except where an echo
  falls). Width does **not** vary with size or role: a 145x24 tab plate, a
  175x56 button, a 208x62 nav row, a 255x340 card and the 238x442 GO HOME
  panel all measure 1.9-2.3 px. Dim and bright outlines are the same weight.
- **Printed echoes**: most outlined elements carry one or two faint copies of
  the outline about 5-7 and 11-13 native px away (nav rows: one inside, left,
  R +11 over the fill; product card and GO HOME panel: two, at +6 and +12;
  footer tape: two, down/right). These are the era's printing, not a
  doubled line; the GO HOME panel, tape and header already draw theirs.
- **Corners** are plain mitres or the element's chamfer; no corner marks, no
  bevel, no inner dark separation line.
- **Dim outline ink** peaks #58..#76 over dark fills (median about #63); the
  palette's `border` #5e1112 was sampled from these. Buttons #58-#5d, rows
  #5b-#66, nav rows #62-#76, tab plates #5c-#6b (bluer, over the glow).
- **Bright outline ink** peaks #fb3535 (header rule on all three headers,
  footer tape, Urgent panel, product card); GO HOME reads #ee2929-#f82727.
- **Header rule**: one full native row of #fb3535 plus a partial one
  (dashboard rows 450 + 0.55 of 451; mailbox 449 + 0.62; login 0.32 + 451 +
  0.49) = 1.55-1.81 px, FWHM 1.67-1.81 px, centred y 187.81 (dashboard),
  187.41 (mailbox), 188.19 (login). A hair under the outline weight.
- **Footer tape**: the one heavy line, 3.60-3.82 px FWHM = 1.50-1.59 dp.
- **Diamond tiles** (the inverted tile's own parts): face #ef3333; inset
  contour 2.3-2.7 px FWHM on the diagonal where our 1.0 dp line reads
  1.9-2.5 px by the same method, peak #7b191a median but varying per tile
  from #3c0c0e to #811f20 (a soft, flat-bottomed profile: lighter and a
  touch wider than ours); tab seam 1.6-2.7 px, #4b-#82; glyph ink core
  near-black (darkest 1% #07..#12, median core #19..#3f); outer edges soft
  because of the striated echo fringe outside them (10-90% rise 4-8 px vs
  ours 1.3 px); separators 15.2-18.3 dp of ground between tips (ours 15.9).

## The era's weights

| name | dp | native px at 4K | used by |
|---|---|---|---|
| `hairline` | 0.7 | 1.68 | the full-width header rule |
| `line` | 0.8333 | 2.0 | every outline: nav rows, list rows, buttons, field, tab plates, badges, cards, panels, the selected diamond |
| `inset` | 1.0 | 2.4 | the diamonds' inset contour and tab seam |
| `heavy` | 1.5 | 3.6 | the footer tape frame and divider |

The selected diamond's old 1.5 was the tape's heavy weight. `hairline`
and `line` differ by 0.25 native px, at the limit of resolution; they are
kept separate because all three header rules agree on it.

## Treatments (`style::DarkPlate`, constants in `eras::neomil`)

| treatment | face | edge | inset | marks | elements |
|---|---|---|---|---|---|
| `REST_PLATE` | #2e1012 | `border` #5e1112, line | = edge | fg | store nav rows; mailbox buttons (`GHOST_PLATE`, face #1a0607, marks dim) |
| `LIT_PLATE` | #551719 | #f63333, line | = edge | #f63333 | hover rung (inferred, components.svg section 9) |
| `INVERTED_PLATE` | #2e1012 | #fb3535, line | `dim` #a32226, inset | fg | the selected hub diamond |
| `INVERTED_LIT_PLATE` | #551719 | #f63333, line | `dim` #a32226, inset | #f63333 | the selected diamond hovered |

Why the inverted diamond is drawn so: the red diamond has three tones
(face, a mid-dark secondary line about half way from face to marks, near-
black marks). Inverted, the face goes dark, the marks bright and the
secondary line stays secondary: `dim` sits ~60% of the way from #2e1012 to
#ef3333. A dark plate in this era is always outlined, at the one outline
weight; the bright-outlined dark plates of comparable size (GO HOME
238x442, product card 270x457, login cards 255x340; the diamond is
208x193) take the bright ink. Alternatives considered: the dim `border`
edge (as the nav rows) would make the selected tile recede behind its red
neighbours; the heavy 1.5 has no precedent outside the 145x24 tape.

## Pixel comparison and corrections

| element | reference | before | after |
|---|---|---|---|
| store nav rows | 2.04 px, #62-#76 | 2.92 px, #a32226 (1.2 dp, mid red) | 2.21 px, #5e1112 (`REST_PLATE`) |
| mailbox Confirm / Jump | 1.96 px, #58-#5d | 2.79 px, #a32226 (1.0, mid red) | 2.32 px, #5e1112 (`REST_PLATE` edge) |
| login idle cards | 1.82 px | 3.71 px (1.5 dp) | 2.08 px (line); inks unchanged |
| login password field | 1.83 px, #72-#7b | 2.77 px, #a32226 (1.0) | 2.00 px, #760e0f |
| GO HOME outline | 2.04 px, #ee-#f8 | 4.20 px (1.5 dp) | 2.53 px (line) |
| header rule (dash / mail / login) | 1.71 / 1.81 / 1.67 px, #fb3535 | 4.85 / 3.67 / 3.84 px of #a32226 / #a8282b | 1.86 / 1.83 / 2.30 px, #fb3535, centred within 0.1 dp |
| footer tape divider | 3.77 px | 3.00 px (1.2) | 3.81 px (heavy), centred within 0.05 dp |
| mailbox rows, tab plates, header badges, Urgent panel, product card | match | unchanged | unchanged |
| `metrics.stroke` (built-in surfaces, the mail screen) | -- | 1.5 | line |

Left as found, with reasons:

- **Geometry, not weight** (each needs a trace and layout change): store nav
  rows sit at x 154.64..361.21 in the reference (ours 153..361) and their top
  edges 0.6-1.3 dp lower; the Confirm / Jump buttons span y 703.3..754.9
  (ours 702..758, 4.4 dp taller) and are 174.1 wide (ours 175); login card 2
  spans x 693.3..946.7, bottom 653.7 (ours 692..947, 655); the GO HOME left
  edge is x 1128.9 (ours 1128); tab plates' left edges are 476.2 / 1131.6
  (ours 475 / 1132.5).
- **Ink, not weight**: the GO HOME outline peaks #f82727 where our fg is
  #ef3333; the tape and its divider peak #fb3535 (ours #ef3333); the login
  idle card edges peak #7d-#9f / #78-#8a, ours #a32226 / #8a2027.
- **Tape outer frame** right side 2.58 px vs 3.76 (composed of a 1-wide
  stroke and coverage rects; footer record dashboard-footer.md).
- **Product card** bottom edge 2.56 px vs 1.64 (the card frame is a 0.6 ring
  plus coverage masks; store-ordinary-frame.md).
- **Diamond insets** are slightly darker and narrower than the reference's
  soft, per-tile-varying line (#5e vs #3c..#81).
- **Echoes**: the nav rows, buttons, login cards and the inverted diamond
  draw no printed echo copies; the reference's dark plates carry them.

## Adopting this in another era

Measure the era's references with the same method, set its
`metrics.weights` (and `metrics.stroke = weights.line`), define its own
`DarkPlate` constants for the outlined controls it has, and replace the
literal widths in its scene tables with them. `Weights::uniform` keeps an
unmeasured era exactly as it was.
