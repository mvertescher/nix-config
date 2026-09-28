# Store material fields

Source: `images/img-09-store.png`, 3840×2160. Coordinates below are
1600×900 design units. These are broad color-field fits; the photographed
scan bands, displaced printing and fine texture remain separate work.

The ordinary cards reveal the common blue ground through a weak red tint.
Opaque brown fills erased 13–36 levels of green/blue in clear patches.
Card 1 now uses `#ff3c3e` at .040 opacity, card 3 `#ff312f` at .043, and
the cropped fourth card `#ff312e` at .041. The unselected second card,
which is not depicted in the source, borrows card 1's material. Held-out
clear-patch channel RMSE is .73/1.56/1.44, .75/1.27/1.93 and
.71/2.31/2.25 respectively. SVG patterns include the identical common
ground, so component specimens show the same fields on their own canvas.

The selected card and navigation use two vertical sRGB ramps mixed by a
horizontal luminance mask. The selected upper corners vary at these
measured vertical stops; no raster is embedded:

| y | Left | Right |
| --- | --- | --- |
| 151 | `#562746` | `#622443` |
| 180 | `#4f2547` | `#5d2541` |
| 235 | `#472a47` | `#562843` |
| 275 | `#412a42` | `#4d283f` |
| 365 | `#362129` | `#45232c` |
| 405 | `#32191d` | `#3e1c21` |
| 516 | `#280809` | `#2f0809` |

Horizontal endpoints are local x0/282. The lower field uses x0/270,
y516/797.1, with corners `#1a0608`, `#150607`, `#130506`, `#100505`.
The upper and outer material masks retain the existing top chamfer,
inward step, upper wash's lower diagonal and selected lower chamfer.

Unselected navigation spans global x153/361 and y248/586, with corners
`#341716`, `#301315`, `#2b090c`, `#280809`. Each row clips that common
field to its own 62px silhouette. The selected row's relative 67px field
has corners `#c42d2c`, `#c12e30`, `#c5292a`, `#c52b2b`. Moving the selected
material to another row is an inferred interaction; the source shows VIDEO.

The fit uses medians of 11×11 native patches and alternating grid cells
for training and holdout. Upper-card samples use x20..240 at 20px steps
and y160/170/180/235/245/255/265/275/365/375/385/395/405/485/505. At the
three top rows x<100 is excluded; at y485/505 x<140 is excluded. Lower
samples use x20/40/80/140/180/220/240 and y520/605/695/755/765. All card
x positions are relative to the source selected origin x769.

| Rendered SVG field | Held-out patches | Total RGB RMSE | Maximum channel error |
| --- | --- | --- | --- |
| Selected upper | 78 | .73 | 2 |
| Selected lower | 17 | .61 | 2 |
| Unselected navigation | 22 | .79 | 2 |
| Selected navigation | 18 | .75 | 2 |

The corrected K1 native render uses those same held-out patches:

| Field | Source → Iced RGB RMSE | SVG → Iced RGB RMSE |
| --- | --- | --- |
| Selected upper | .695 | .314 |
| Selected lower | .524 | .464 |
| Unselected navigation | .807 | .492 |
| Selected navigation | .667 | .385 |

Every SVG/native patch differs by at most one channel level. The first
native attempt exposed an implementation error: `Prim::Ramp` takes
bounding-box fractions, while the new tables supplied SVG user-space
coordinates. That attempt flattened the fields and is rejected. The
corrected axes preserve the measured interpolation, and a pixel regression
uses independent source patches across the upper/lower and navigation
fields to detect this failure.

## Runtime contract

`StoreReference` selects one static backdrop for each of the 20 card and
category combinations. Every backdrop has the unchanged ground, one
coverage-composited card layer inside the original shelf opening clip,
and one navigation layer. The fourth card's material is permanently
masked at x1557. All rendering uses the existing software sRGB compositor
and bounded image cache; this adds no new compositing primitive.

One derived foreground scene retains the original Plates/Picks, labels,
weapon art, borders and permanent viewport, while removing only their
replaced resting fills. Its stable identity preserves pointer gestures
across selection. Existing opaque hover/held drawings still cover these
resting fields. Tests check all 20 navigation/hit mappings and opening
motions. Custom palettes, named variants and unknown theme eras retain
the original semantic scene. K1 passes all 267 Rust tests, both store
fidelity gates and all 22 repository checks. Eighteen fractional state
captures are reviewed; the production app and rest preview are identical.
The later certification, margin-glyph and typography work is separate from
this accepted broad-material checkpoint.
