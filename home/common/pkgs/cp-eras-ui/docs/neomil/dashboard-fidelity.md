# Dashboard source fidelity, 2026-09-14

Source: `images/img-07-dashboard.png`, 3840×2160, Behance asset
`3fc4ef118663901.60e5fa6a7f2f7.png`, screen #60. Measurements below use
the trace's 1600×900 coordinates: 2.4 source pixels per design pixel.
The source image stays gitignored; the reconstruction uses vectors and
sampled gradient stops. [The dashboard TODO](../../todo/neomil-dashboard.md) tracks completion.

The original shape gate passed at 19/30 counted shapes and 94% of source
bounding-box area while missing glyphs and actual text. That is a layout
measure, not a visual-fidelity percentage. Local comparisons below isolate
the named feature; none claims the whole dashboard is complete.

## Menu printing and panel content

Six source matrix patterns replace identical rectangular placeholders.
VEHICLES carries `161-9A`; the others carry `61-9A`. Their observed module
occupancy, numeric outlines, circular marks and offsets are retained. The
Rust matrix modules are compound paths: the canvas rectangle fast path
collapses squares under the artwork's −45° rotation.

GO HOME contains five plus four actual lines, Rajdhani Medium 16.6667 at
x1138.75, with measured line ends and baselines. BETTERLIFE TEC and boxed
PETROCHEM are vertical text. The panel surface uses a sampled cool-to-red
gradient with a 1.984934px scanline pitch. Its lower empty-body sample is
approximately RGB 33,7,7, against the previous uniform fill's 59,15,20.
The surface and foreground share the existing panel-open clip.

## Tile contours and edge tabs

Keep the measured outer silhouettes, centres, 104px half-diagonal and
outward plateau at 89px. Inset contours have two flat tips, not one:
the outward tip and one side. Dark source-pixel deficits give diagonal
stroke coverage of 0.82–1.25px, supporting a common 1px stroke instead
of the previous 2px. Source-fitted inset centres differ from the accepted
outer centre by up to 1.95px; this does not move the tile or its printing.

| Tile | Centre | Inset side cut, relative x | Outward flat, relative y | Solid tab |
|---|---|---:|---:|---|
| VEHICLES | 334,460 | +59.33 | −58.15 | Bottom-right |
| LOCATIONS | 530,460 | −56.45 | −58.15 | Bottom-right |
| FACTIONS | 725,460 | −56.44 | −58.15 | Top-right |
| WEAPONS | 431,593 | −58.75 | +58.99 | Top-right |
| PRODUCTS | 628,592 | −57.59 | +59.99 | Top-right |
| CORPORATIONS | 822,592 | −57.85 | +59.99 | Top-left |

Each crisp red tab is offset 14.5–15px along one design axis, about
10.3–10.6px perpendicular to the diagonal, with a 1px joining seam. It
fits inside the existing plate bounds. Hover and pressed drawings retain
all contours, tabs and glyphs while using the existing inferred inks.
Faint repeated scanline edges are separate from these solid tabs.

Native-resolution dark-pixel IoU restricted to the inset band improves
from 0.094–0.275 to 0.959–0.999 across the six tiles. This excludes glyphs
and most other content. Normalized outer scale leaves roughly 2px of
source variation at some tips; no per-tile resizing is implied.

## Menu typography

The measured label face is Rajdhani Medium 500, size 20.625, without
added tracking. Regular, Medium and SemiBold were fitted independently
to all six source runs; each selected Medium. The previous 19px SemiBold
with 1.2px tracking was shorter, heavier and more widely spaced.

| Label | x, middle anchor | Baseline y |
|---|---:|---:|
| VEHICLES | 333.6712 | 347.4587 |
| LOCATIONS | 533.8568 | 347.4505 |
| FACTIONS | 726.9810 | 347.4172 |
| WEAPONS | 434.3844 | 720.4167 |
| PRODUCTS | 630.7829 | 720.4271 |
| CORPORATIONS | 824.6770 | 720.4541 |

The decimal precision records the fit; source accuracy is approximately
±0.3 design pixels. Label centres are not exactly their tile centres.
Bright-core mask IoU improves from 0.076–0.279 to 0.865–0.899. The mask
excludes dim trails and therefore measures lettering, not echo fidelity.

## Header and small chrome

CUSTOMER, #NC488402 and SECURITY LEVEL use Rajdhani Medium 12.9167,
without added tracking. Their text origins are x123.75,253.75,1136.6667
at baseline 90.4167. Source bright-core fits favor Medium over the
previous lettering, with local mask IoU 0.77–0.85.

| Element | Measured geometry in design pixels |
|---|---|
| `next` | Bright envelope x257.5–377.08, y99.58–128.33; four smooth source vector outlines, 0.72px stroke |
| TECHNOLOGY | Bright core x258.33–371.67, y133.75–139.17; individually placed capitals |
| Customer badge | x119.5833,y104.5833,56.6667 square; bottom-left cut 15.4167 |
| Security badges | x1132.6667,1193.5,1254,1315; y104.1667–160.4167; widths 56.25–56.6667 |
| Code tape | Bright core x254.58–379.17, y150.42–160, with small rounded corners |
| Left numbered chip | Main square x56.25,y243.3333,12.5 square; adjacent 4.5833 square at x46.25,y243.75 |
| Right numbered chip | Main square x1539.1667,y242.5,12.5 square; adjacent 4.5833 square at x1529.1667,y243.3333 |
| COMPUTER SYSTEMS | Square frame x475,y237,211×21; text baseline 252.5 |
| DESCRIPTION | Square frame x1132.5,y237,221.6667×21; text baseline 252.5 |

Pass 3 used sampled mean badge fields: customer `#4d2c31`; security
`#3c2247`, selected T2 `#7e273d`, T3 `#392346`, T4 `#382444`. Pass 4
replaces those flat fields with the local material described below. Frames
remain dim, 0.8333px wide. LEVEL uses reusable open vector paths at
0.625px stroke; the tier marks use source vectors with cap heights
12.1–12.9px, including the counter in 4.

The tape reads `JHN 102 CKC 151 CC10 5111`. Its tiny leading wordmark
is not confidently readable, so source vector art preserves its visible
shape. Both margins read `JHN 102 CKC 151 CC10 AS5`, clockwise on the
left and counterclockwise on the right. Their source bright envelopes
are x45.42–51.25,y462.92–575.42 and x1545.83–1551.67,y555.83–668.33.
The right margin also restores the downward arrow and KIROSHI wordmark.

Pass 4 replaces the remaining font substitutions with compact source
vectors: rounded geometric TECHNOLOGY capitals, a reusable chamfered
monospace alphabet for the tape and margins, the wide KIROSHI letters,
and filled chip digits. The N stems and diagonal are separate open paths
to keep miter joins inside the cap height. Chip 1 has its narrow stem and
short flag; chip 2 has a flat shoulder, heavier diagonal and straight foot.

The earlier `KIRØSHI` transcription overstated the source evidence. At
native resolution, the bright interior stroke in O curves left and back
right. The corrected drawing is an outer oval with a cubic inner arc,
rather than a diagonal slash. The source supports this visible geometry;
it does not identify an original font or supply a logo file.

| Local source-core mask IoU | Before pass 4 | Revised SVG |
|---|---:|---:|
| TECHNOLOGY | 0.451 | 0.846 |
| Left margin code | 0.157 | 0.770 |
| Right margin code | 0.149 | 0.761 |
| Tape code | 0.119 | 0.766 |
| KIROSHI | 0.112 | 0.808 |
| Chip 1 | 0.552 | 0.974 |
| Chip 2 | 0.455 | 0.911 |

These fixed native-resolution crops use bright red cores for the exposed
lettering and dark cores inside tape/chips. They exclude surrounding echo
trails and measure the isolated SVG proposal, not compiled app fidelity.
The secondary runs are only about 11–14 source pixels high, limiting exact
stroke and font recovery. Smooth curves and straight segments preserve the
legible structure without tracing raster stair steps. The component sheet
shares the same measured header, margin and chip groups.

## Local badge and panel material

The five badge fields have independently measured stripe periods of
1.981–1.985 design pixels. They share the established 1.984934px pitch,
with a 1.17px warm band starting at local phase 1.658333px. Four-corner
bilinear fields capture the smooth variation; the customer's upper-left
RGB 74,55,63 falls to 71,42,46 below, while its right side retains more
blue. Warm stripes add roughly +9 red and −3 to −4 green/blue. Selected
T2 has roughly twice that modulation.

| Fixed clear badge field | Before RGB MAE | Revised RGB MAE |
|---|---|---|
| Customer | 3.77,3.22,4.10 | 1.68,1.82,1.50 |
| T1 | 4.34,2.22,1.98 | 2.09,1.90,1.57 |
| Selected T2 | 6.74,3.23,3.17 | 1.79,2.54,2.44 |
| T3 | 4.39,2.09,2.01 | 2.09,1.79,1.68 |
| T4 | 4.30,1.96,2.13 | 2.05,1.70,1.64 |

Each source-resolution mask contains 6,249–6,427 pixels, excluding frames,
lettering, printing trails and bright outliers. The actual SVG proposal's
95th-percentile channel error is 4–7 levels. Hidden colors are interpolated
from the clear field; these are observed composite colors, not recovered
shader layers. The existing frame strokes and typography remain separate.

GO HOME also varies horizontally: around y342, the left/right field is
approximately RGB 45,31,43 versus 37,23,27; near y610 the difference is
mostly a few red levels. The field now has x stops at 1128,1240,1366 and
rows at y314,350,400,450,540,620,700,756. Its 24 cool control colors retain
the existing warm delta +6,−4,−4, pitch 1.984934px, origin 313.180711px
and warm-band height 0.992467px.

| Fixed clear panel mask | Before RGB MAE | Revised RGB MAE |
|---|---|---|
| Whole available field, 299,903 pixels | 2.20,1.70,1.90 | 1.72,1.46,1.38 |
| Upper field y<450, 56,559 pixels | 2.61,3.01,4.11 | 1.80,1.78,1.46 |

The SVG proposal's 95th-percentile error is RGB 4,4,3 over the whole mask
and 4,4,4 above y450. Masks exclude text/trails, brands, maker mark, frame
and chamfer exterior. Badge and panel blends cover their full local boxes
over opaque bases, avoiding adjacent-strip edges at fractional scales.
They use existing sRGB `Ramp`/`Masked` primitives; the panel material stays
under the same opening clip as its foreground.

## Connected panel contour echoes

Two connected, scan-modulated contour copies and side-bar copies replace
the four solid echo rectangles. Source crops show diagonal joins, bottom
returns and a bar trail that the rectangles omitted. The primary frame,
bright side bar and animation bounds remain unchanged.

A local fit expands x by 1.0052/1.0104 about x800 and y by
1.00455/1.0091 about y482 for the first/second copies. Below the side bar,
the resulting x offsets are approximately 2.90/5.81px; above it they are
2.94/5.89px. First-copy positions on held-out upper panel, header-rule,
customer-left and T4 edges agree within 0.03–0.21px, comparable to native
peak quantization of ±0.20833px. Weak second copies outside the panel
depart by up to roughly 0.94px, so this is a local contour model.

The echo lines independently measure the same 1.984934px scan period.
Their mask uses phase 0.267px, minimum luminance 144/255 and eight linear
intervals per cosine-like cycle. An opaque minimum mask sits below the
periodic ramps. Both SVG and Rust draw 233 explicit ramp rectangles:
an SVG repeating pattern quantized the fractional tile pitch at 1600px
and erased its modulation. Explicit ramps retain the native-source fit
and reduce native SVG/app bar-trail red MAE from 22.89 to 0.79 levels.
Four nested strokes, widths 2.8,2.0,1.2,0.4px, approximate
the measured softened edge profile. This is a compact reconstruction of
visible modulation and spread, not evidence of the original renderer.

| Fixed exterior source mask | RGB RMS before | Revised RGB RMS |
|---|---:|---:|
| Upper edge, 2,496 pixels | 29.52 | 6.49 |
| Side-bar trail, 2,771 pixels | 39.62 | 11.15 |
| Lower edge, 6,300 pixels | 28.59 | 4.75 |
| Bottom return, 2,688 pixels | 21.00 | 10.42 |

The masks exclude the accepted primary frame and compare isolated SVG
proposals at source resolution. The red geometry dominates improvement;
bottom green/blue MAE increases from 7.04/7.45 to 8.08/7.83. All contour
copies remain inside the existing panel-open clip and precede the primary
foreground. Their source supports the local drawing while leaving the
effect's UI-versus-presentation origin unresolved.

## Dashboard background

The source glow changes hue in both axes. The old horizontal blue ramp
under one vertical mask missed the lower cyan and overstated the warm
left field. The dashboard now uses twelve sampled horizontal color rows
cross-faded over eleven vertical intervals, replacing all three previous
background layers together. Shared Neomil glow constants remain the
other screens' reference data.

The mesh has x stops at 0,50,100,200,300,400,600,800,1000,1200,1400,1500,
1550,1600 and y rows at 0,80,160,240,300,360,420,480,560,680,800,900.
Thirteen-pixel patch medians on a 20px grid exclude foreground, labels,
panel and exterior echoes. A local interpolator supplies coarse control
colors, rounded to RGB bytes. Hidden colors are inferred from surrounding
clear areas; this is a reconstruction of the visible composite, not a
recovered original shader or alpha stack.

| Clear-background metric | Previous SVG | Revised SVG |
|---|---|---|
| RGB mean absolute error | 4.11,4.72,6.89 | 0.71,0.90,1.07 |
| RGB 95th-percentile absolute error | 8,17,33 | 2,3,3 |
| Pixels differing by more than 8 in any channel | 29.96% | 0.080% |

These measurements use the isolated background proposal and a fixed mask
of 992,750 pixels. The mask predates the tile-tab corrections; a few tab
boundaries remain in it, so it should be revised before reusing it to score
the combined image. Both SVG ramps and Rust `Ramp`/`Masked` primitives
blend the sampled colors in sRGB; the Rust group stays inside the
dashboard's leading `Soft` layer.

The row blends use a full-canvas opaque base and eleven full-canvas
masked layers. Each mask is black above its interval, fades to white
across it, and stays white below. Adjacent strip rectangles looked correct
at native size but left antialiasing seams at fractional scales: compiled
samples at 0.37,0.4875,0.83 had minimum interior alpha 207 instead of 255.
The continuous base removes those internal edges without changing the
sampled field. The native SVG render is byte-identical after this fix;
the native app changes only 146 pixels, by at most one channel level.
A narrow-column renderer regression crosses every join at those scales
and at native size to require opaque interior pixels. Fresh scaled
hover/held previews confirm the horizontal seams disappear.

## Validation before the latest pass-4 batch

After passes 2–3 and the earlier label/background work, the unchanged
gates reported the following. These results predate the secondary vectors,
local badge/panel fields and connected contour echoes described above.

| Comparison | Counted shapes matched | Matched shape area | Median centre error | Weighted ink placement IoU |
|---|---:|---:|---:|---:|
| Source → SVG (G1i) | 18/30 | 89% | 1.0px | 0.79 |
| SVG → app (G2i) | 18/24 | 83% | 0.0px | 0.72 |

Both gates passed. Source ink placement improved from 0.66 before pass 1
and 0.74 after it, while inventory area fell from 94% to 89%. The newly
unmatched WEAPONS inset remained visible; its local source-contour IoU was
0.982. Correcting the old 2px stroke to 1px changed whether palette
segmentation closed the contour and filled its interior.

G2i had previously matched 36/41 shapes and 99% area. In this batch, three
visible upper-row insets accounted for 27,228 of 32,423 unmatched area:
SVG extraction filled their roughly 900 outline pixels into roughly
9,200px diamonds, while the app's palette split the outlines among clusters.
The other three unmatched boxes were two fragments of COMPUTER SYSTEMS (76px and
136px wide versus one 213px app box) and the GO HOME left rule. That rule
occupied all 598 pixels of its reported box in both renders; app extraction
merged it into larger panel regions. All six outer tile bounding boxes
matched exactly between SVG and app. Direct review found no omitted drawing
behind these inventory changes. The app still uses its existing published
palette, including `#de2e2e` for the source trace's `#ef3333` main ink.

That batch passed all 203 Rust tests. Reviewed captures covered SVG/app
rest frames, upper- and lower-row hover/held states, and both panel-open
frames at 0.15s. Geometry remained in every state, with panel material
and content revealing together. Only the Neomil and fallback dashboard
goldens were refreshed. Its repository check passed all 19 checks,
including all 25 golden cases.

## Latest pass-4 validation

The final source→SVG and SVG→app gates pass with unchanged thresholds:

| Comparison | Counted shapes matched | Matched shape area | Median centre error | Weighted ink placement IoU |
|---|---:|---:|---:|---:|
| G1i: source → final SVG | 14/30 | 81% | 1.4px | 0.79 |
| G2i: final SVG → release app | 12/18 | 90% | 0.0px | 0.59 |

The count changes require visual interpretation. New local colors change
whole-image palette clustering, which changes connected components even
where drawing is unchanged. G1i now misses both tab frames and T3/T4
badge matches; those frames remain visible. G2i's unmatched CORPORATIONS
inset, T2 grouping, maker-mark halves, vertical brands and left panel rule
are also present in the reviewed captures. In particular, this batch
changes no app pixels outside its badge, small-lettering and panel regions;
the six menu tiles and tab frames are byte-identical to the preceding
accepted app. The fixed local source measurements above describe the
actual material/detail changes more directly than these coarse scores.
The lower G2i ink score is retained here rather than treated as completion
evidence; the existing main-ink palette difference remains.

All 203 Rust tests pass. Source crops, both SVG sheets, native Neomil and
fallback app captures, upper/lower hover and held states at fractional
scale, and SVG/app panel-open frames at 0.15s have been reviewed. The
native release capture is byte-identical to the reviewed Neomil golden.
Only the Neomil and fallback dashboard goldens change. Initial concurrent
debug captures at the script's 4s settle were blank; 15s captures showed
the complete drawing. The release build passes the normal G2i script at
its unchanged 4s settle. No capture timing or gate thresholds were changed.
The full repository check passes all 19 checks, including all 25 golden
cases. Live desktop verification remains open.

## Local tape and chip printing

The tape face, both numbered chip faces and their two adjacent square
fragments are independently measured `#fb3535`. Held-out clear source
patches contain 290 tape pixels and 69/23/18/12 chip/fragment pixels;
all five medians are RGB 251,53,53. The chip patches are flat, while the
tape patch has less than one level of median channel error. This corrects
only these five bright shapes; the main tile red remains `#ef3333`.

Inside the existing tape-code and chip-1 contours, a constant plus two
harmonics of the measured 1.984934px scan period captures local RGB ink
variation. The mask fit accounts for contour alpha and the independently
measured bright face. It uses 298 tape-core pixels and 71 chip-1 pixels,
holding out alternating whole letters or scan cycles, then reversing the
folds. Compared with a separately fitted best constant ink, held-out red
MAE improves from 16.12–18.66 to 9.88–11.40 for tape and from 20.39–22.64
to 7.04–7.39 for chip 1; green and blue improve in both folds too.

Actual native source→SVG core RGB MAE is 18.51,17.54,18.78 →
10.06,2.47,2.95 for tape and 29.13,10.44,10.59 → 6.42,3.65,3.79
for chip 1. These are fixed local printing masks, not whole-feature scores
or gate exclusions. Seventeen ramp stops represent the two harmonics;
there is no per-row fit or random texture. Rust reuses the accepted glyph
paths as masks and composites their color over the local bright faces.

Chip 2's dark ink remains open: the equivalent model worsens held-out red
MAE from 18.52 to 23.66 in one fold despite improving the reverse fold.
Eroding the contour mask or adding drift does not resolve that inconsistency.
Its dark digit and the eight tiny leading tape-mark paths remain unchanged.
Exterior tape/chip copies are also outside this correction.

## Six menu-label printing echoes

Two dim, scan-modulated copies now sit beneath each unchanged primary
Rajdhani Medium 20.625 label. Their paths come from the bundled font:
17 reused glyphs, preserving quadratic curves, counters and advance widths.
Opaque paths at the original text positions differ from the SVG text by
only 0.218–0.280 mean alpha levels out of 255; even-odd and nonzero fills
render identically. Rust uses those paths in the existing software
compositor so the echoes blend over the actual sampled ground.

The first and second copies expand by (1.0052,1.00455) and
(1.0104,1.0091), with alpha 0.4392283 and 0.2776018. Their translations
are predicted from VEHICLES, LOCATIONS, WEAPONS and PRODUCTS. FACTIONS
and CORPORATIONS are held out because their near-centre copies overlap;
unstable independent fits are not used. The 1.984934px scan mask has
peak phase 0.267px and minimum luminance `#909090`. Explicit ramps over
an opaque mask base prevent seams at fractional render scales.

| Label | Actual source→SVG exterior RGB RMS, before → after |
|---|---:|
| VEHICLES | 13.126 → 4.692 |
| LOCATIONS | 11.277 → 4.018 |
| FACTIONS, held out | 6.270 → 3.851 |
| WEAPONS | 14.480 → 4.525 |
| PRODUCTS | 10.038 → 3.890 |
| CORPORATIONS, held out | 7.180 → 4.055 |

These fixed masks exclude the union of strong source/current primary
glyphs, dilated by one native pixel, and contain 13,898–23,154 pixels per
label. Full-crop RGB RMS also improves for all six. Remaining softness,
bright-core coverage and background granularity are visible in native
crops. This local fit does not establish a universal screen transform or
the original effect's origin. Echoes are static source appearance; menu
pointer bounds, hover/held behavior and primary text remain unchanged.

## Header printing echoes

Sixteen local groups now have two attenuated copies: next, TECHNOLOGY,
CUSTOMER, #NC488402, SECURITY LEVEL, five badge frames, five LEVEL/tier
groups and the horizontal rule. Accepted primary vectors, text and fields
remain unchanged. Caption copies use outlines from the bundled Rajdhani
Medium fit; this does not identify the source's original font or renderer.
All badge material is composited before the copies, with primary printing
afterward in both SVG and Rust.

Each group has a fitted first translation and doubled second translation;
the rule has independently measured endpoints and vertical offsets.
Four finite stroke/expansion profiles approximate softness, applying each
profile's alpha once to the union of opaque fill and stroke. Deduplicated
SVG profiles render pixel-identically to the measured candidate at both
3840×2160 and 1600×900. Rust uses existing paths and masks with the same
group order. Printing masks retain the global 1.984934px scan period;
explicit ramps over a gray floor avoid fractional-scale seams. The
horizontal rule has no additional scan modulation.

| Group | Actual source→SVG held-out RGB RMS, before → after |
|---|---:|
| Rule | 15.249 → 4.867 |
| Customer frame | 7.231 → 3.139 |
| Security T1 / T2 / T3 / T4 frames | 7.673 / 14.072 / 9.782 / 7.990 → 7.132 / 6.133 / 5.563 / 2.735 |
| next / TECHNOLOGY | 12.906 / 10.737 → 5.574 / 4.472 |
| Customer LEVEL/T1 printing | 9.255 → 5.003 |
| Security T1 / T2 / T3 / T4 printing | 11.937 / 3.664 / 10.125 / 12.194 → 7.868 / 3.230 / 5.166 / 6.121 |
| CUSTOMER / #NC488402 / SECURITY LEVEL | 10.732 / 12.355 / 12.723 → 4.431 / 4.593 / 4.665 |

Fixed masks omit primary bright printing; frame masks also exclude badge
interiors. Training/held-out pixels alternate spatially in 17px printing
bands, 17×19px frame checker bands or 143px rule bands at source scale.
Each group has its own local fit; these are held-out pixels within one
source image, not independent screenshots or wholly unseen badges.
The rule endpoints are measured separately from its profile-fit mask:
first copy x37.8704–1562.1439, second x34.5255–1565.4393. Finite vector
profiles approximate their soft terminal transitions.

All sixteen held-out masks improve, but security T1's frame and selected
T2's printing improve only slightly. Overlapping printing/field residuals,
dim edge widths, scan contrast and primary-caption coverage remain visible.
These local copies do not recover one global effect or establish whether
the source trails came from the UI, display processing or presentation.

## Printing-batch validation

All three proposals above are integrated into both SVG sheets and the
dashboard's Rust tables. All 203 Rust tests pass. The latest gates report:

| Gate | Matched shapes | Matched source box area | Median centre error | Weighted ink IoU |
|---|---:|---:|---:|---:|
| Source→SVG, G1i | 16/30 | 82% | 0.5px | 0.84 |
| SVG→release app, G2i | 17/22 | 98% | 0.0px | 0.65 |

These coarse shape/ink scores still depend on palette clustering. G2i
lists the maker mark halves, next's e and two narrow panel rules as
unmatched; all remain present in the reviewed app crops. The panel and
tile pixels are byte-identical to the preceding accepted app, while this
batch changes exactly 27,280 pixels inside header/rule, tape/chip and
menu-label regions. The existing main app/source ink difference remains.
The higher gate percentages do not establish source-exact typography or
effect recovery; the fixed local measurements above are stronger evidence
for this batch's intended corrections.

Native SVG/app and fallback captures, fractional-scale upper/lower-row
rest/hover/held previews, and both 0.15s panel-open frames are reviewed.
Parallel debug captures at 15s were blank; a single 45s debug capture
shows the complete drawing and is byte-identical to the release capture
at the normal 4s settle. The release fallback also matches Neomil. Only
those two reviewed dashboard goldens change. Capture defaults and gate
thresholds are unchanged. The full repository check passes all 19 checks,
including all 25 golden cases. The normal G2i capture also matches the
reviewed release golden pixel-for-pixel. Live desktop verification remains
separate.

## Dashboard-local primary ink

The source and SVG ordinary foreground is #ef3333, while the shared app
palette foreground is #de2e2e. Six separate 10×8-design-pixel cap patches
contain 456 native pixels each: two source patches are exactly (239,51,51),
the other four have means within roughly one level and channel standard
deviation at most 0.466. Every SVG patch is (239,51,51); the preceding
release app is (222,46,46), an RGB deficit of 17,5,5.

The app now projects the unmodified reference dashboard's foreground to
#ef3333 in one drawing Style shared by its foreground and software
backdrop. The stored Style and other screens retain their shared palette.
Eligibility requires a recognized Neomil reference theme and an exact
match of the entire resolved Palette to its compiled reference. Other
variant names, unknown eras and any custom palette role therefore retain
their supplied colors, even when their foreground happens to match the
reference. Forcing Neomil on a different desktop still chooses compiled
reference defaults, following the existing CLI behavior. Fixed sampled
material and fixed hover/held inks remain unchanged. Six regression tests
cover these loading and isolation cases, including the no-config fallback.

## Tape and chip exterior echoes

Two upper/outward face copies now sit behind each accepted bright tape,
chip and fragment. Primary face geometry, #fb3535 face color, tape/chip-1
ink cycles, chip-2 digit and the unreadable tape mark remain unchanged.
Four finite widths (3.0, 1.6, 0.6 and 0.0 design pixels) approximate the
soft profile, with one opacity applied to each opaque fill/stroke union.
Both copies share a silhouette; the second translation is twice the first.

| Family | First translation | Scan-mask floor | Held-out source→SVG RGB RMS, before → after |
|---|---:|---:|---:|
| Tape | −1.4992, −1.5188 | 134/255 | 23.457 → 6.053 |
| Chip 1 | −3.5312, −1.3200 | 123/255 | 22.248 → 5.429 |
| Fragment 1 | −3.3995, −1.3730 | 117/255 | 18.487 → 4.845 |
| Chip 2 | +3.4116, −1.1335 | 156/255 | 23.817 → 6.627 |
| Fragment 2 | +3.5119, −1.9370 | 145/255 | 20.968 → 7.589 |

Scan masks retain the 1.984934px pitch with local phase/contrast and
explicit transparent-ended ramps over an opaque gray floor. They remain
outside the translation groups so their row phase is global. Existing
component group references include the corresponding tape or chip copies.
Rust reuses the accepted tape path and composites all copies before the
opaque primary faces, inside the first software backdrop.

These are actual full-SVG measurements on fixed exterior masks, omitting
the union of source/current primary pixels with R>190, dilated two native
pixels. Alternating 11×13-native-pixel checker bands separate training and
held-out pixels; held-out counts are 3,184, 1,119, 883, 1,583 and 500.
They are local holdouts within one image, not independent screenshots.
The neighboring chip-1 copy also contributes to fragment 1's measured
improvement. Native and design-size source/current/candidate crops were
reviewed. Separate component insertion renders identically to the fitted
candidate at both scales.

The improvement chiefly restores red-channel trail mass. Green/blue MAE
remains about 2.7–5.3 levels and can worsen by up to 0.35 on fragment 2.
The right fragment's overlap and small primary corners remain approximate.
These exterior face copies do not explain interior dark-code modulation,
identify the original renderer, or establish a global screen transform.
Chip-2 ink and the tiny tape mark therefore retain their unresolved status.

## GO HOME body and maker-shape echoes

All nine body lines now have two scan-modulated copies beneath the original
text. The independently fitted first body map is an expansion about
(1200,450) by (1.0047133,1.0045944), followed by translation
(+2.2239349,−0.1549230). The second copy doubles those deltas; it does not
apply the map twice. Opacities are 0.3249131 and 0.2249453, in #f93333.
The mask uses the 1.984934px scan pitch, phase 0.6453047 cycles and
floor 0.4762390, with a 17-stop cosine approximation and explicit rows.
Twenty-five bundled Rajdhani Medium glyph outlines are reused across the
runs and copies. At original positions they agree with the bundled-font
SVG text at alpha MAE 0.097–0.244/255 and hard-mask IoU 0.9814–0.9988.

Fixed exterior masks omit primary glyph alpha >0.25 and source red cores
>170, dilated one native pixel, and include their 24-pixel neighborhood.
Each line has 8,059–15,491 measured pixels. Actual SVGs made from an odd-line
fit improve held-out even-line RGB RMS from 12.08–12.85 to 5.00–5.27;
an even-line fit improves held-out odd lines from 11.86–12.98 to
4.77–5.17. All nine final all-line candidate masks improve to 4.73–5.20.
These whole-run holdouts come from one source image. Some source softness
and primary glyph coverage remain different; the acceptance measurements
use actual SVG rendering, whose error is about half a level higher than
the optimizer's resampled-mask surrogate.

The maker M and square dot use an independent translation-only model:
(+2.9678791,+1.4086453) and twice that, with opacities 0.3652943 and
0.2426586 in #ef3333. Its scan phase is 0.6373027 cycles and floor
0.4842112. Actual SVG exterior RMS improves 21.458→7.808 on 7,561 native
pixels; complementary 17×19-pixel checker holdouts improve
23.020→8.868 and 19.637→8.054. Rust unions the existing two primary
silhouettes before drawing each copy, avoiding double alpha at their
small overlap. If the primary shape changes, regenerate that union too.
Body and maker copies are in the panel's leading software backdrop under
the existing GO_HOME_OPEN clip in both SVG and Rust. The component panel
excerpt contains the same additions.

The heading, maker microtext and two vertical brands receive no new echoes.
Source/current/candidate crops show primary differences that must be fitted
before their trails. Bright envelopes below use R>190 and G<100 at native
resolution; right/bottom are exclusive and the bounds are approximate:

| Primary | Source envelope | Current envelope |
|---|---|---|
| GO HOME | (1137.083,320.000)..(1210.000,333.333) | (1141.250,320.417)..(1212.083,332.917) |
| PRECISION LIQUID | (1220.000,732.500)..(1284.583,737.083) | (1223.750,732.500)..(1280.000,738.333) |
| POLYMER MUSCLE | (1224.167,740.833)..(1280.417,745.417) | (1223.333,740.833)..(1280.833,745.833) |

The heading starts about 4.17px too far right and its current bright cap
height is about 0.83px short. Applying the body echo model worsens its
surrogate exterior RMS, 9.380→9.818, so it is rejected for that group.
The two maker microtext runs have 408/363 source bright pixels versus
661/636 currently: both need a lighter fit, while their differing width
errors rule out one shared stretch as a complete correction.

The maker primary's broad bounds already match, but its bright-mask IoU
is 0.9383 (265 source-only and 567 current-only pixels). Rounded shoulders,
notches and corners differ locally, and dark scan modulation inside the
source M remains absent from the flat primary. Current exterior echoes
improve the bounded surrounding region without resolving those faults.
These are separate, concrete primary-shape/typography tasks. Vertical-brand
trails also need their own source check; no body model is assumed there.

## Primary-ink and detail-batch validation

All three proposals are integrated into both SVG sheets and Rust. All 209
Rust tests pass (164 library, 45 binary), including the six new palette
loading/isolation regressions. The latest measured gates are:

| Gate | Matched shapes | Matched source box area | Median centre error | Weighted ink IoU |
|---|---:|---:|---:|---:|
| Source→SVG, G1i | 17/30 | 82% | 0.5px | 0.86 |
| SVG→release app, G2i | 17/23 | 98% | 0.0px | 0.70 |

G2i still fragments the maker halves, next's e, narrow panel rules and
part of the last body line differently. All are present in the reviewed
SVG/app crops; the high area score does not establish exact small shapes.
The six 10×8-design-pixel app cap patches now equal (239,51,51) with zero
variance, eliminating the previous 17,5,5 RGB deficit. Compared with the
preceding accepted release capture, 158,222 pixels change, all inside the
intended dashboard foreground or new echo regions; sampled ground outside
those regions is unchanged. Fallback and reference captures are identical.

The release app renders fully at the normal 4s settle. Source/SVG/app crops,
the component sheet, native rest and 0.15s opening captures, and fractional
upper/lower rest/hover/held previews are reviewed. The latter use one scoped
drawing Style for both foreground and backdrop while retaining the shared
shell Style. Fixed hover/held colors and glyph geometry remain intact;
the new panel copies follow its opening clip. Only Neomil and fallback
dashboard goldens are refreshed. The full repository check passes all 19 checks, including all 25 golden
cases. The other 23 goldens remain unchanged. The normal G2i capture is
pixel-identical to the reviewed release image.

Independent transcription review checked tape/chip profiles, masks and all
body/maker outlines/transforms. Rust applies body-copy opacity per glyph;
SVG applies it per complete copy. An equivalent SVG experiment finds at
most two alpha levels of antialias overlap on a few pixels at small
fractional scales, with none at native source scale. This small raster
limit does not justify adding expensive per-line mask stacks. Source
softness, primary contour/typography and other stated residuals remain.
Live desktop verification is still separate.

## Matrix and margin follow-up evidence

Native source/current crops expose localized printing variation inside all
six matrix/code groups. In eroded current dark-glyph cores, additionally
requiring source R<160, source red-channel standard deviations are about
40.0, 37.2, 17.0, 40.2, 31.8 and 18.9 levels for VEHICLES, LOCATIONS,
FACTIONS, WEAPONS, PRODUCTS and CORPORATIONS. These masks contain
1,952–2,245 pixels per tile and the current SVG is nearly constant there.
The masks can still contain coverage/registration differences; this is a
diagnostic of missing spatial variation, not a recovered ink process.

A constant plus two Fourier harmonics at the established 1.984934px scan
pitch improves held-out red MAE by only about 0.1–1.3 levels across both
alternating 17-native-pixel column folds. Typical residuals remain 26–34
levels on the four strong tiles. A uniform row cycle alone is therefore
insufficient. The next source fit should separate overlapping dark artwork,
primary coverage and periodic ink while preserving module occupancy and
flat primary tile fills. No new matrix model is accepted in this batch.

Both vertical JHN codes and the KIROSHI wordmark also retain visible outward
trails. Their bounded source crops are (41,456)..(54,588),
(1540,551)..(1558,671) and (1540,722)..(1559,770) in design coordinates.
These remain concrete local follow-ups; no whole-screen transform or new
photographic exclusion follows from those observations.

## Remaining uncertainty

The large tile interiors are flat `#ef3333`: two source patches have
exactly zero channel variance and the other four vary by less than one
RGB level. Adding whole-tile scanlines or grain would introduce variation
absent from the source. Keep this red and the flat fills.

Matrix artwork, margin printing, GO HOME heading, maker primary/lettering
and vertical-brand trails still need local examination, as detailed above.
Slight expansion explains several direction changes but does not establish
one whole-screen recipe. Chip 2's dark ink and the tiny leading tape mark
also retain unresolved local ink variation. Header profiles and primary
text still have the localized residuals documented above.

Four clear background patches have native high-pass channel RMS of about
1.06–1.95 levels, with more green variation on blue/cyan ground. Red
row/column mean RMS is below 0.12 levels, unlike the coherent panel and
badge stripes. A flat VEHICLES patch has exactly zero variation. Uniform
whole-screen grain would therefore add material absent from the source.

A subsequent cross-screen check compares the existing 3840×2160 login,
dashboard, mailbox and store sources (`img-06-private.png` through
`img-09-store.png`). Two clear patches are pixel-identical across all four:
upper blue (500,20)..(600,65), 25,920 native pixels, and left warm
(30,700)..(100,755), 22,176 pixels. Every RGB byte agrees, including the
noise; high-pass residual correlations are 1.0. The mid-cyan patch also
matches login exactly; the other two screens have foreground overlap
there, so that patch does not establish a four-screen comparison.

This is evidence for a shared fixed background asset or composite,
rather than four independent noise realizations. It does not distinguish
an authored UI background from a common presentation background, nor
recover the original shader, texture, dithering step or authoring recipe.
The other local screen exports therefore cannot supply independent noise
samples. The smoothed sampled ground and flat tiles are retained; original
background material or authoring inputs are still needed. No arbitrary
noise or new `photo` tags hide unresolved pixels from a gate. Favorable
local measurements do not by themselves close pass 4 or its end-to-end
validation.
