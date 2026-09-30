# Store navigation typography

Source: `images/img-09-store.png` at 3840×2160. The navigation trace and
Iced scene use 1600×900 design coordinates. These five labels use Rajdhani
Medium with individual size, baseline and tracking. The former common
15px regular face with 1px tracking was visibly too small and loose.

| Label | Source primary ink box, native pixels | x | Baseline y | Size | Tracking |
| --- | --- | ---: | ---: | ---: | ---: |
| VIDEO | 391,688–486,715 | 162.5 | 298.25 | 18 | −0.21 |
| AUDIO | 392,848–489,876 | 162.92 | 364.58 | 17.5 | −0.20 |
| GAMEPLAY | 393,1008–569,1036 | 162.5 | 431.58 | 18 | −0.30 |
| CYBERWARE | 393,1169–595,1197 | 162.5 | 498.58 | 18 | −0.36 |
| CONTROLLER | 393,1329–602,1357 | 162.5 | 565.17 | 17.5 | 0 |

The boxes isolate primary ink from the displaced scan echoes: red above
200 with red minus green above 130 for the four bright labels; red below
130 and green below 45 for dark VIDEO. The same thresholds were applied
to source and native-resolution SVG renders. This matters for AUDIO:
comparing a source threshold to a looser rendered alpha mask incorrectly
favored the thin regular face. At matched thresholds, medium 17.5 covers
88% of the source primary ink with 0.671 binary overlap; regular 17.5
covers 67% with 0.608 overlap. The medium face also matches the visible
stroke in the side-by-side crop.

The selected VIDEO and four resting labels have the same geometry across
idle, hover and pressed coats; only their ink changes. The source depicts
VIDEO selected, so other selected categories and pointer states remain
inferred. The photographed scan echoes and grain are outside this type
fit. The native Iced result still needs direct visual review against both
the image and SVG before closing the navigation fidelity task.

## Footer two-cell box

The source keeps the frame at x153.5–297.5, y851.5–873.5 and the divider
at x215. The former 10px bold code crossed that divider and sat about
4.5 design pixels below the source core. The two 8px captions also ran
longer than the source. The frame and divider remain at their measured
positions; each text run now has its own baseline, face and horizontal
scale (`Prim::Wide` in Iced).

| Run | Source primary ink box, native pixels | Face / size | x, baseline y | Horizontal scale |
| --- | --- | --- | --- | ---: |
| `68SD1D1100D1S` | 381,2053–501,2068 | Semibold 10 | 158.4, 861.8 | .82 |
| `COMBAT COLONIZATION` | 530,2053–696,2067 | Medium 8.5 | 220.2, 861 | .89 |
| `DEFENCE PROGRAM` | 530,2073–671,2087 | Medium 8.5 | 220.2, 869.2 | .91 |

Boxes use red above 150 and red minus green above 90 in narrow text
regions, excluding the frame and divider. The source has displaced print
and scan echoes, so the boxes describe the strong primary stroke rather
than every red pixel. A native-resolution SVG fit puts the code at
381,2053–501,2068 and the second caption at 530,2073–671,2086; the
first caption ends within a few pixels of its source edge. Native Iced
review remains necessary.

The unboxed `00032 05 54 0B CP` to the right has a separate print fit.
Its source primary ink box is 752,2082–959,2097 native pixels. The
previous SVG run was about 30 pixels too wide and six pixels above the
source; the previous Iced run was closer in width but four pixels above
it. Rajdhani
Regular 9.5 at x313, baseline873.6667 and horizontal scale1.1965 puts
the bright SVG core at 752,2082–960,2097. The source has broken scan
strokes, so this box and the letter-group positions are more reliable
than whole-mask overlap. The SVG prints it bright; the Rust primitive
retains semantic `Ink::Dim` for custom palettes. Reference-only color
routing and native baseline review are handled with the store material.
The decorative echo and underline are outside this type fit.

## Card title and stats

The four cards do not put their primary text on one common grid. Source
`MAGNUM 650` on ordinary cards starts six native pixels above the selected
card's title, while their stat rows start five to seven pixels below the
selected row. The third and cut fourth card also place their titles and
subtitles slightly farther left within their frames. The fits below keep
those card-local offsets; selection still uses the selected drawing at
the active shelf position.

| Card | Title x, baseline | Subtitle x, baseline | Label baseline | Value baseline |
| --- | --- | --- | ---: | ---: |
| 1 ordinary | 10.2, 204.5 | 10.7, 223.4 | 435.2 | 468 |
| 2 selected | 10.6, 207 | 10.3, 223.4 | 432.75 | 465.3 |
| 3 ordinary | 8.9, 204.5 | 8.6, 223.4 | 435.2 | 468 |
| 4 cut ordinary | 8.1, 204.5 | 8.2, 223.4 | 435.2 | 468 |

All titles use Rajdhani Bold 22.5 at .98 horizontal scale. Ordinary
subtitles use Medium 19 at .99; the selected subtitle is Medium 19.5 at
.96. Ordinary stat labels are Medium 17.75 with individually measured
x and tracking, and their values are Semibold 23 at .98 horizontal scale.
The selected labels are Medium 17.5 and values Semibold 23 at .96 scale.
Only the first two stats are visible on the permanently cut fourth card.

At 3840×2160, using red above 160 and red minus green above 100 in narrow
text regions, the selected title's primary ink box moves from the old
SVG's 1875,454–2154,485 to 1874,462–2159,497; the source is
1874,462–2158,498. Its four label boxes align within one native pixel of
the source, as do most value boxes. The third and fourth ordinary titles
now match their source boxes exactly: 2655,456–2939,491 and
3443,456–3727,491. Ordinary stat labels match their source starts and
widths within one native pixel; their bottom ink row remains one pixel
short in the SVG. The photographed displaced echoes and grain are outside
these primary-glyph fits.

The SVG records source geometry. Iced's earlier N capture placed VIDEO,
GAMEPLAY and CYBERWARE two native pixels below that geometry,
CONTROLLER one pixel below, and all three footer runs about one pixel
below. Only the Rust baselines were lifted to compensate. The O native
capture found another renderer offset in the card type: all titles were
three native pixels low; subtitles ended three pixels below the source;
ordinary values ended three pixels low, and selected values were three
pixels low throughout. Rust now lifts title baselines by 1.25 design
units, subtitles by 1, ordinary values by 1 and selected values by 1.25.
Stat-label baselines stay as fitted. The SVG coordinates remain the source
measurements; the calibrated card type awaits a new native comparison.

## Footer interior

Three clear native patches inside the two stamp cells expose an unrelated
material error: source medians are (12,3,5), (10,4,5) and (8,4,6), versus
SVG (96,24,26) and previous Iced (94,17,18). They sample x384..398 and
x658..672 at y2047..2051, and x684..698 at y2086..2090, away from primary
text and frame strokes. The surrounding traced ground is about (8,3,4).
The source stamp therefore has no supported opaque red interior. The
reference SVG and reference-only store foreground now leave it clear;
custom palettes keep their original semantic filled stamp. Fine displaced
printing and the small remaining field variation are separate from this
large opaque-fill error. Native review is pending.

The reference-only store scene maps this unboxed run to Fg while the
semantic scene keeps Dim. Eligibility still uses the complete reference
palette and variant; custom/variant routing is unchanged. The component
footer excerpt uses the same #fb3535 as its parent trace. Native reference
and custom captures are required before accepting the color correction.
