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

## Ordinary compliance lines — JV

A fresh 4K original/SVG/current-package comparison exposes a primary
typography mismatch in the two ordinary CC35 lines. Both lines are
implemented, but the source runs are visibly wider and brighter, with
rounder glyphs than the current Rajdhani approximation. Card 3 repeats
the discrepancy; the fourth card also starts farther right in the render
and reaches its cut with different letters. This is separate from the
accepted title and side-label printing, and from faint secondary copies.

JV retains full RGB contexts and five-threshold row/column profiles.
At R>120 with red-dominance controls, the first source line spans 608
physical pixels on both cards 1/3, versus 527 native; the second spans
442 versus 386. Source first-line tops are y1175 versus native y1168,
and second-line tops are y1194 versus y1187. Lower-threshold profiles
corroborate a six-to-seven-pixel vertical difference. The fourth card's
left/cap support repeats it, while its right ends are clipped. These
different inks make threshold widths descriptive, not exact font metrics
or evidence for changing weight. Astra independently verifies all 90
threshold groups and nine raw contexts.

JW's one +2.5-design-y translation improves both complete SVG lines on all
three cards at three sizes. All 54 complete-line all/bright/dark RGB L1
and RMS comparisons improve; alpha and exterior pixels remain exact.
Astra independently recomputes 972 subset records and reviews nine
contacts. The 67 local exceptions remain explicit: card 3's 4K second-line
dark band 5 loses 9,008 L1, and card 1's second-line profile gap at
x1398–1405 loses 141 all-pixel L1. The profile gaps are source-empty
strong-red columns, not assumed semantic word boundaries; overlapping
line unions are not summed as disjoint data. An initial fractional
viewport-alignment error is preserved separately; the accepted comparison
uses direct top-left renders matching canonical source sampling.

JX's identical native coordinate correction improves all 54 complete-line
comparisons; Astra recomputes all 972 records and retains 93 local losses.
Sixteen headless state cases pass, including custom colors, selected-card
controls, the clipped fourth card and two partial caption reveals. The
disabled wrapper reproduces actual Store frames. [Round seventy-eight](../reference-svg-round78.md)
integrates the correction after three-size package parity, both Store
gates, 311 packaged tests, 22 checks and 28 first-attempt matrix cases.
Font, ink, width, x properties, drawing routes and primitive counts stay
fixed. This addresses only vertical registration; the wider source glyphs
and different horizontal positions remain open. Font byte selection,
source-font identity, layout and ink remain separate questions. No
replacement font or new text effect is accepted.

KA audits the current caption implementation before further width fitting.
Each line is one `Prim::Text` using bundled Rajdhani Regular, with no
per-character splitting, explicit width measurement or outline-cache entry.
The existing font regression checks actual selected bytes; no new fallback
defect is demonstrated. The pinned Wgpu backend sends uniform text to its
cached-text route. Layout keys include content, font, size, line height,
bounds dimensions, shaping and alignment, excluding position, color and
clip. Moving these baselines therefore introduces no new layout-cache key.
This code-path review does not measure GPU time or runtime cache-hit rates.
The source's roughly 15% wider threshold support
does not measure shaped advances, especially with different ink strengths.
The source also repeats at +1,577 physical pixels versus roughly +1,581
native, exposing a separate card-origin discrepancy. Compare exact-run
glyph origins and contours before choosing a width correction.

`Prim::Wide` would switch these ordinary runs to filled outlines. Its
current reusable optimization is restricted to marked Bold title runs,
so Regular captions would not receive that optimization automatically.
Measure any resulting cost; a stretch is not a free correction or evidence
of source-font identity. The accepted vertical move retains the simple
text route and introduces no new effect. KA's pinned evidence is in
`/tmp/cp-eras-next/ka-compliance-font-audit/`.

KB then probes the exact two complete and two truncated caption strings at
sizes 8 and 19.2. All 24 cases and 1,008 glyph records select the bundled
Regular bytes with nonzero glyph IDs. Auto and Basic geometry agree exactly;
Advanced differs by at most 0.00007 physical pixel in glyph x and 0.000062
in outline bounds. The complete runs advance 529.959/387.802 pixels at 4K.
These are shaped advances, distinct from the photographed ink widths.

Matched words confirm a contour/layout discrepancy. At the declared R>120
red-dominance threshold, their half-open pixel-index widths are:

| Token | Source, both complete cards | Native card 1 | Native card 3 |
| --- | ---: | ---: | ---: |
| ONLY | 45 | 37 | 38 |
| ALLOWED | 86 | 72 | 71 |
| TO | 22 | 16 | 17 |
| DEVICE. | 65 | 55 | 54 |

Native windows come from actual whole-run glyph clusters, including the
terminal period, and exclude the other caption line. A descriptive connected
component check separates the source's rounder O at R>120: 12 pixels wide,
versus 7/8 native. Lower-threshold source fringes merge letters, while
higher-threshold native ink fragments or vanishes; neither gives a complete
comparable glyph. Differing ink and rasterization remain confounders, so
these bounds do not establish a safe 1.15× stretch or the source typeface.
Astra verifies the probe, all 60 source/native word-mask records, twelve
component controls and both raw contact sheets. The initial scratch probe's
space-callback assumption and the terminal-token window correction remain
recorded in `/tmp/cp-eras-next/kb-compliance-layout/`; they were diagnostic
harness repairs, with no production font change.

The source repeat of +1,577 pixels versus authored +1,581.6 remains a
separate position question. KC tests only the third ordinary caption pair
at x9.1 instead of x11, preserving the accepted baselines, fonts and ink.
Fresh baselines exactly match the accepted SVG at all three sizes. Whole
contexts and all-pixel line unions improve, but three of eighteen complete
line/subset comparisons fail: 1600 second-line bright loses 178 RGB L1,
fractional first-line dark loses 425, and fractional second-line bright
loses 184; RMS also worsens in each. All 79 local exceptions among 972
records are retained, including the 4K first-line gap loss of 1,243.
Astra recomputes the complete inventory and reviews three source contacts.
Alpha and every pixel outside the third caption context remain exact.
The standalone shift is rejected under the frozen gate; no native trial,
retune or production change follows. This preserves the evidence for a
repeat-position discrepancy while leaving complete font/layout fidelity
unresolved. Evidence is in `/tmp/cp-eras-next/kc-compliance-repeat/`.

JV's initial baseline is the verified round-77 package; its SVG is freshly rendered
with the pinned font environment. Original-resolution contacts and input
hashes are in `/tmp/cp-eras-next/jv-compliance-source/`; the bounded trials
are in `jw-compliance-registration/` and `jx-compliance-native/` under
the same parent.

KD independently checks the exact Rajdhani, FreeSans Regular and Exo2 Light
font tables. None is identified as the Neomil source face. FreeSans Regular
is a plausible generic shape comparison: at the same nominal cap height,
its O outline is roughly 11.92 physical pixels wide versus Rajdhani's 7.74.
This compares font outlines, not photographed threshold bounds. Earlier
FreeSans Login and Exo2 Neo-kitsch rejections do not answer the Store question.

KE tests only FreeSans Regular at the predeclared cap-normalized size
`8 × 643 / 729`, preserving current placement, ink and clipping. Merely
making the exact font file available leaves the accepted round-78 SVG
byte-exact at all three sizes. The six ordinary text-node changes preserve
alpha and every exterior/selected pixel, but fail 20 of 54 complete-line
checks. All eighteen dark subsets worsen both L1 and RMS; card 3's 4K first
line also loses 20,222 all-pixel RGB L1. The first/third source O-hole errors
increase 3,081→3,856 and 3,433→4,123. Astra verifies all 1,071 regional records,
the enclosed source holes and nine contacts; 394 local exceptions remain.
Reject this unfitted face/cap swap. It is not evidence against a separately
justified complete-layout model. Source-only diagnostic window repairs are
retained; word/O contexts are not isolated glyph scores, and overlapping
profile-gap unions do not identify every increased red pixel as false ink.

KF checks the same four complete/truncated strings with actual Iced:
24 cases and 1,008 glyphs select the exact FreeSans Regular bytes. Auto/Basic
agree; Advanced differs below 0.00005 pixel. Its actual baseline is 0.95 ×
size, and its cap-normalized full-line advances are 584.155/430.843 pixels.
A separate shared origin/tracking fit uses only ten first-line source
word-lefts. Held-out word edges have 2.024-pixel RMS and 4.066-pixel maximum
error at 4K, versus 3.194/7.885 for Rajdhani under the same fitting rule.
Repeated cards are correlated observations. These remaining errors prevent
claiming an exact layout or source-font identity. A tracked native transfer
would also need full-run/split-painter and cost checks; it would add work
to the current six complete text runs. Frozen evidence is in
`/tmp/cp-eras-next/kd-caption-font-evidence/`, `ke-compliance-face/` and
`kf-caption-shaping/` under the same parent.

KG tests that frozen two-parameter model on the six ordinary SVG nodes.
Both the accepted baseline and a zero-letter-spacing identity replay are
byte-exact at all three sizes before the fitted render. All 36 all-pixel
and bright complete-line comparisons improve against the accepted SVG,
but ten dark comparisons fail: seven worsen both metrics and three gain
L1 while losing RMS. In total, 44/54 improve both. The first O-hole L1
still increases 3,081→3,272; third/fourth holes improve. A hole score alone
does not separate the outline from faint printing or the underlying field.
All 54 line comparisons improve against the rejected KE swap, which is
useful layout evidence but does not replace the accepted baseline.

Astra verifies 1,071 records against each baseline and all nine source
contacts. The 185 local L1/RMS exceptions against production remain in
`/tmp/cp-eras-next/kg-compliance-layout/`, including second-line dark losses
at every size and a 4K third-card DEVICE. context loss. Alpha, selected
captions and exterior pixels remain exact. Reject the transfer under the
frozen gate; there is no native font, tracked painter or performance change.
The current six cached text runs stay intact. Further work needs an
explanation of the remaining word/contour and printing differences before
another parameter fit, rather than moving individual failed glyphs.
