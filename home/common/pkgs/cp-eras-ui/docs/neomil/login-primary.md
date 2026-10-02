# Login primary card fit, 2026-09-27

Source: `images/img-06-private.png` (#59), 3840×2160. Design coordinates
below divide native pixels by 2.4. The SVG is
`docs/neomil/login-trace.svg`, rendered at 3840×2160 with the bundled
Rajdhani font through librsvg. This record covers the active avatar,
password printing, USER 01, prompt and card captions. The component
excerpt is a translated copy of the trace's first card. The source image
is cited and measured here; no source raster is embedded in the artwork.

## Active avatar

The dark mark is two **filled** hexagonal halves, separated by a shallow
diagonal slit. The former wire hexagon and diagonal stroke covered only
about 645 design square pixels inside x452..535/y398..459, against about
2,974 dark pixels in the source. A clipped wedge enters from the avatar
box's left edge at x452..475/y418..456. Four short stepped bars reach
it from above, and the small lower marks at y468..480 are retained.
The traced filled halves, wedge and marks cover about 2,947 square
pixels after refining the diagonal and upper-left contour. A narrow
dark-red outer rim follows the photo at x480.5 and x534.2. These counts
use the same native-pixel dark threshold
`R<100, G<50, B<55` on both images; the source is photographed and
has softened edges. Pixel intersection-over-union over that crop rises
from 0.169 to 0.878. That is a local coverage check, not a whole-card
fidelity score.

Held-out horizontal scans of the central dark figure, in design units:

| y | Source dark x intervals | New SVG dark x intervals |
| --- | --- | --- |
| 408 | 491.7..522.1 | 492.1..522.9 |
| 425 | 481.7..508.8, 515.8..532.1 | 482.1..507.9, 515.8..531.7 |
| 435 | 481.7..491.2, 498.3..532.1 | 482.1..490.8, 497.9..531.7 |
| 449 | 491.7..521.7 | 490.8..522.1 |

The source's soft edge and local red scan texture are unresolved. The
inactive cards use the source-traced portrait described below; its
primary native geometry was accepted in the H capture.

## Header printing

The photo's five badges carry the same thin LEVEL and tier glyph
shapes measured on the dashboard, but their positions and surrounding
material are login-specific. The login uses a local copy of those
paths, fitted against its own source crop and painted after the five
badge surfaces. This keeps the badge vectors in the theme's foreground
role. The former Rajdhani LEVEL label was about 24px wide; the native
login label is about 43px wide and only 7.5px high.

| Header ink | Native source x/y | New SVG x/y |
| --- | --- | --- |
| CUSTOMER | 124.58..179.58 / 81.67..90.00 | 124.58..179.17 / 80.83..90.00 |
| SECURITY LEVEL | 1137.08..1220.00 / 81.67..90.00 | 1137.08..1220.00 / 80.83..90.00 |
| #NC488402 | 254.58..316.67 / 79.17..90.00 | 254.58..316.67 / 80.83..90.00 |
| first badge LEVEL | 126.67..169.58 / 109.58..117.08 | 126.67..169.58 / 109.58..117.50 |
| first badge T1 | 137.50..157.50 / 125.83..139.17 | 137.50..157.08 / 125.83..138.75 |

The SVG's former 1.5px tracking made the top captions 16–26px too
wide, while Iced's untracked runs were 4–6px too wide. The new SVG
removes that tracking and both representations scale these three runs
locally by 0.93–0.935. LEVEL and tiers use their source-supported
vector outlines in both. The first native Iced header capture had a
fragmented LEVEL stroke: the scene's closed-path renderer joined and
filled the glyph's open branches. LEVEL now uses `Plate::open_path`
with each source line drawn once at 0.625 design pixels in both
representations. H confirms continuous runtime linework at the
source's placement. At bright threshold 180, source/H LEVEL areas are
79.5/71.9 design square pixels, with matching vertical bounds
y109.58..116.67. The tier
contours use a 0.35 runtime outline and a
0.15 SVG outline; the SVG then has 84 bright pixels versus the
source's 85. The different tier widths compensate for the two stroke
rasterizers while keeping one measured path. The primary top runs
match their source horizontal bounds. The three top-caption copies are
measured separately below; other secondary printing remains unresolved.
Five clear badge-field medians support local
reference fills `#422e34`, `#2f224a`, `#722942`, `#2c2448` and
`#2b2546` from customer through T4. On seven held-out clean patches
per badge, channel MAE falls from roughly 18–57 RGB levels under the
palette fields to at most 1.6 with these flat reference fills. Their
`Border`/`Dim` roles remain the custom-palette fallback. The photo's
soft field grain is left unresolved.

The I native enlargement exposed a separate frame problem after the
field fit: all five 1.5-design-pixel, source-bright `Fg` outlines were
three fully bright native pixels wide (five including antialias), while
the photograph's clean top frame is two dim native pixels. Source top
core RGB medians, customer through T4, are `(121,42,49)`,
`(107,26,55)`, `(115,34,60)`, `(105,27,54)`, `(105,28,54)`; Iced used
`(246,51,51)` on every frame. T2's frame nearly merges into its red
field. The source trace and reference Plate edges now use those local
colors at 0.75 design pixels for customer and 1.0 for T1–T4. Each Plate keeps its original
`Ink::Fg`/1.5 edge for custom palettes and feedback. This new edge fit
awaits the next native capture; the I capture established the defect,
not validation of its correction.

The F native header capture showed the PROTOCOL and 6520-A44 runs
8–9px too narrow and 2.5–5px too low. Source bright-ink bounds are
x257.92..300.42/y122.50..127.08 and
x258.33..299.58/y130.42..135.00. Their labels now use Rajdhani
Bold 8.5, 1.2 tracking and baselines 127.7/135.6, with starts
x257.4/257.8. The native SVG fits those bounds within about 0.4px,
and the first line's bright area is 73 versus the source's 81 design
pixels; the second is about 70 versus 76. H Iced spans exactly the
source x extents x257.92..300.42 and x258.33..299.58, with bright
areas 83.9/81.4 versus source 80.9/75.7. Its top edges sit about
0.42px high. Keeping both strings as legends preserves text semantics
and foreground palette ink.

The protocol block now uses twelve source-measured bright bars in the
Dossier's era-owned art table. The four x columns start at 257.5,
268.33, 279.58 and 290.42; bright row starts are 103.75, 105.42,
107.08, 108.75, 112.50, 114.17, 115.83 and 117.50. The old renderer
had eight bars 18–27px wide, while each native bar is about 10.5px.
The SVG's 11.25×1.7 rectangles reach bright-mask overlap 0.971 with
the source in x254..303/y102..120, with 211 versus 205 design square
pixels. The H runtime uses the same geometry and source-bright red.

The source's code tape is a continuous rounded capsule, with bright
extent x257.08..376.25/y150.42..160.00. Its new cubic Plate path
has native SVG bright-mask overlap 0.900 in x253..378/y150..161.
Five separate subdued lines approximate the visible scan echoes
immediately above it. The large dark code spans
x282.08..371.25/y152.08..156.25, and the tiny left printing spans
x257.92..278.33/y156.25..159.58. Both are contours traced from
source pixels in the era's art module, with `OnSelect` custom-palette
fallback. The former 7px Rajdhani code ended at x355.83 in Iced,
and its five tall pseudo-barcode ticks did not resemble the tiny
source printing. This source threshold recovery directly constructs
the SVG paths; the overlap alone is not independent validation.

## Margin printing

The native left and right numbered chips have opposite flanking marks.
The left has two dim bars at x45.5 and x50 plus a short bright block
before the x60 square; the right has the bright block before its x1541
square and two short dim bars after it. These are separate ink-role
plates in the scene table, so custom palette colors still apply. At
native size, thresholded source and SVG chip regions span
x45.42..72.50 versus x45.42..72.08 on the left, and
x1530.83..1557.50 on both at the right. The sharp SVG marks contain
about 25–30 fewer design square pixels than the blurred photo crops.

The down-arrow stem starts at x1548/y411 and its narrow triangular
head ends at y439. The source bright core has about 60 design square
pixels in its local crop; the narrowed SVG has about 55. The dim
echo marks now use the palette's border ink. The rotated
left code spans x46.67..52.92/y383.75..470.83 in the photo and
x46.25..52.92/y383.75..470.83 in SVG. The right code and KIROSHI
fit their source vertical extents within about 0.4px.

## Inactive cards: two-axis material and traced portraits

The unselected card fill is two-dimensional. On card 2, clear 7×7
source medians at y342 change from RGB (121,42,48) at x710 to
(103,42,52) at x890; by y540 they change from (108,24,25) to
(92,20,21). Card 3 similarly changes from (91,41,52) to (74,35,44)
at y342, and (81,19,20) to (67,16,16) at y540 over x1000..1180.
The horizontal slope is mainly in red while the vertical slope is
strongest in green and blue. A two-stop diagonal gradient ties the
channel slopes together and cannot fit both directions.

On checkerboard-held-out clear patches from 36 samples per card,
vertical linear interpolation has RGB root-mean-square error
(7.14,1.61,2.41) for card 2 and (6.17,2.43,3.20) for card 3.
A single diagonal reaches (1.15,7.69,10.40) and
(1.00,7.42,10.49), while an independent x/y plane reaches
(1.00,1.55,2.43) and (1.00,1.90,2.56). A clipped two-axis material
is therefore supported; its additive scene data and palette handling
need shared renderer review before implementation. These numbers
exclude portraits, labels, the feet, and the top-right chamfers.

The reference material now uses two vertical `Prim::Ramp`s per card,
combined by an x-axis grayscale `Prim::Masked` ramp in sRGB and clipped
through each card's top-right chamfer. Card 2 top corners at y315 are
`#7d2b31` / `#612d38`, its y569 corners `#6c1516` / `#540d0e`;
card 3 uses `#5f2c38` / `#45222b` and `#500f0e` / `#3c0b0a`.
The existing body Plate remains the custom-palette fallback; its
reference alternative draws only the border over this material. The
foot, notch and portrait draw as before. On checkerboard-held-out
clear patches the procedural SVG reduces current vertical-fill RGB
RMSE from (7.40,4.56,6.52) to (0.97,1.76,1.97) on card 2, and
from (6.10,5.33,7.87) to (0.91,1.86,2.31) on card 3. A separate
`Prim::Soft` in the reference backdrop holds both clipped materials.
No photographed pixel grid is embedded. The I native source/SVG/Iced
review confirms the broad reference material and portrait composition;
quantitative native clear-patch validation remains for the next capture.

Clear-card row medians show a red 2px periodic residual of only about
1.6 RGB levels on card 2 and 1.2 on card 3; green/blue row residuals
are below 0.75 level standard deviation. The proposal omits that
small scan modulation until the large material field is verified.

The inactive cards reuse six source-traced red-tone masks from
the real x772..868/y393..487 portrait crop. The masks contain 105
closed regions and about 1,750 vertices, retaining the woman's face,
hair and uniform insignia without embedding pixels. Every tone has
source RGB through `reference_fill` and a semantic ink-role fallback
for custom palettes. On the unobstructed x774..866/y395..485 crop,
the generic silhouette's red-channel RMSE against the source was
83.98; the new native SVG is 13.67. The four dark-mask overlap scores
at red thresholds 65, 100, 140 and 180 improve from
(0.000,0.528,0.566,0.593) to (0.927,0.955,0.966,0.959). H Iced's
open-card red RMSE is 13.71 versus SVG's 13.67, and its dark-mask
overlap scores are (0.919,0.951,0.966,0.961). The locked-card H RMSE
is 17.68 versus SVG's 17.48, with mask overlaps
(0.894,0.932,0.953,0.951). The higher locked error reflects the
source crop's darker photograph under the same traced layers, an
unresolved tone difference rather than a displaced contour.

The F capture confirmed the contours but exposed a separate backing
color mismatch: the photo and SVG's clear avatar square/tab patches
read about RGB (246,51,51), while Iced used palette foreground
(222,46,46). Both inactive slots now give those two plates
`reference_fill(#f63333)`, retaining foreground ink for custom
palettes. H Iced's clear square median is RGB (246,51,51), matching
the source exactly; the tab is within one level per channel.

## Mask, name and captions

The photo has ten separate stars. The first occupies
x383.33..390.00/y617.08..623.33 and subsequent stars start on an
approximately 11.25px pitch; the tenth ends at x491.67. One horizontal
blinking stroke follows at x499.17..512.92/y622.92..623.75. The old
trace's 12px Rajdhani stars were about 3.33px wide on a 7.92px pitch,
and its two underscores ended at x475. The new trace uses 23px
Rajdhani, 1.93px tracking, x381.9/y632.1; the rendered first star is
x383.33..389.58 and tenth x484.58..491.25, all at
y617.08..623.33. The final x error is about 0.42px. The tail is a
single 13.75×0.83 rectangle. Runtime uses the same mask metrics and a
separate `Caret::AfterMasks` plate so typed input moves the stroke
without changing the full stored secret or the static hollow slot in
the Login button. The existing field clipping and reservation still
bound long input.

At native size, rectangular ink bounds for the first card are:

| Print | Source x/y | New SVG x/y | Setting |
| --- | --- | --- | --- |
| USER 01 | 469.58..528.33 / 515.83..528.33 | 469.58..528.33 / 515.42..528.33 | Rajdhani medium 20, tracking −0.4, centred at x499, baseline 528.3 |
| password: | 380.00..439.58 / 583.33..596.25 | 380.00..438.75 / 583.33..596.25 | Rajdhani 15, x379, baseline 594 |
| first notice | 377.92..620.42 / 696.25..701.25 | 378.75..620.42 / 695.83..701.25 | Rajdhani 8.43, x378, baseline 701.4 |
| second notice | 375.83..537.08 / 704.58..711.25 | 378.75..537.08 / 705.00..710.83 | Rajdhani 8.43, x378, baseline 710.4 |

USER 01's native dark-ink area is about 217 design square pixels.
Rajdhani medium renders about 200; semibold would render about 275,
visibly too heavy at the same measured bounds. The caption samples
include photographed color bleed and soft echoes,
which the trace's single sharp run does not reproduce. Width and
placement are now close at native scale; a separate pass can fit those
echoes after the primary geometry is reviewed. The inactive card names
and notices use the same corrected type metrics in the trace and scene
table. Their lower contrast and backing material still need local
source fitting.

SVG verification used native librsvg output at
`/tmp/cp-eras-completion/login-trace-final.png` against the source;
the earlier trace is at `/tmp/cp-eras-completion/login-trace-before.png`.
The first integrated native Iced capture is
`/tmp/cp-eras-completion/b-login-neomil-3840x2160.png`. Its avatar
dark-region area is about 2,878 design square pixels and its overlap
with the source is 0.872, versus 0.878 for the SVG. USER 01 has the
same x bounds as the source and about 198 dark pixels of area, close to
the medium-weight SVG's 200 and the photo's 217. Its top reaches
y515.0 rather than the source's y515.83. The first Iced star occupies
x383.33..390.00/y616.67..622.92 and the tenth ends at x491.25;
the source stars sit 0.42px lower and the tenth ends at x491.67.
The moving tail exactly matches the source rect at rest. These local
images are temporary evidence, not repository assets. Fractional size,
long-input and pointer-state comparisons were also captured at
1537×947. The synthetic empty-to-80-mask change touched 563 pixels,
with zero outside the password field; rest-to-dark blink changed only
27 caret pixels, leaving masks and the separate static hollow button
slot intact. Root review of hover, held and disabled previews found the
existing button behavior intact; disabled feedback changes the prompt.
The H build passed all 263 shared crate tests. The new inactive-card material still needs native runtime capture;
the photo's secondary printing echoes remain a separate task.
The preview evidence is in
`/tmp/cp-eras-completion/login-state-results.json` and
`/tmp/cp-eras-completion/login-state-review.png`. The combined Rust
regression set passed 257 tests before the later source-detail work;
final repository gates and source comparisons belong to the integrated
review after that work.


## Top-caption secondary copies — AG, 2026-09-30

The login photograph has two faint copies above CUSTOMER, #NC488402 and
SECURITY LEVEL. They were missing from both the trace and native drawing.
The dashboard's analogous copies are not interchangeable: same-position
login/dashboard crops differ by 7.6–10.0 RGB levels on average. This fit
uses the login's Regular Rajdhani outlines and source image independently.
The primary captions, badge artwork, ground coefficients and other printing
are preserved.

Each caption has two translated copies, with rounded stroke profiles at
1.4, .7, .25 and 0 design pixels. A profile receives opacity once over its
fill/stroke union. The shared 1.984934px scan period has an independently
fitted phase and strength per caption. Nine-stop ramps over an opaque gray
floor preserve fractional pitch without tiled seams. The copies share the
existing reference ground's first Soft preparation; they add no full-screen
pass and custom palettes retain the existing semantic backdrop.

| Caption | First-copy dx/dy, design pixels | Native held-out middle RGB RMS | Native held-out last RGB RMS |
| --- | --- | --- | --- |
| CUSTOMER | −4.1173 / −3.6830 | 13.83 → 6.09 | 11.06 → 5.93 |
| #NC488402 | −3.5767 / −3.6281 | 14.85 → 6.75 | 10.99 → 7.53 |
| SECURITY LEVEL | +2.1264 / −3.6159 | 14.09 → 6.51 | 11.97 → 6.09 |

Only each crop's first horizontal third was used for fitting. Before any
trial, the primary strokes were excluded using source/trace red thresholds
and a one-pixel dilation. Middle/last thirds, native rendering, echo masks
and bright primary masks were then independent checks. Across red-minus-
green thresholds 8/15/25, native held-out echo F1 rises from zero to
.675–.809. SVG holdouts improve independently as well. Native bright masks
at red 180 and 210 are exactly unchanged for all three captions. The 1,212
strong primary pixels above red 235 are byte-identical. At the looser 150
threshold, SECURITY gains two unsupported fringe pixels and F1 slips
.80843→.80762; CUSTOMER and the code improve. This small fringe tradeoff
is retained explicitly, not hidden by the average error.

The native 3840×2160 change affects 12,782 pixels, all within the three
caption crops, bounding x277..2939/y175..208. All 5,016 blank control pixels
above/below the copies are unchanged. SVG compaction to reusable outline
definitions renders byte-identically to the measured trial. The component
sheet's header specimen is explicitly the dashboard header, so it retains
its dashboard-specific copies; the login card excerpt is unaffected.

Scratch evidence: `/tmp/cp-eras-next/ag/neomil/`, including the frozen
pretrial masks, fit parameters, actual SVG measurements, native comparison
and threshold results. The 1600×900 capture changes 2,532 caption pixels.
At 1537×947, matched rest/hover/held/disabled/80-mask/hidden-caret captures
each change the same 2,817 caption pixels, with every other pixel identical.
The matched custom-palette capture is byte-identical. State-to-rest checks
confine all interactive differences to the password/action region in both
revisions. The [AG checkpoint](../reference-svg-round18.md) records all six
affected gates, 286 Rust tests, 22 repository checks and 27 exact visual
matches.
These three copies are a bounded correction; exact
photographic texture, other login echoes and softened edges remain open.

## Inactive card notices — AH, 2026-09-30

The notices on cards two and three sat about ten native pixels below the
source photograph in both the SVG and the Iced capture. This is distinct
from the active card notice, whose position remains unchanged. Card two
and three now place the first line at design baseline 584.2 (formerly
588.4), and the second at 592.8 (formerly 597.4). The two-line separation
was adjusted by 0.4 design pixels to fit the SVG's second line independently.

At 3840×2160, the source's strong red primary ink occupies y1389..1400
on line one and y1410..1421 on line two. The adjusted SVG occupies
y1389..1401 and y1410..1422. This holds for the first, middle and last
70-pixel-wide sampled spans on each inactive card. Before adjustment,
the first line began at y1399 and the second at y1420 in both the SVG
and Iced capture. The source mask uses R>180 and R−G>110; card two's SVG
uses the same cut, while card three's dimmer SVG requires R>105 and
R−G>60. Comparing the adjusted SVG with the frozen previous rendering,
changed pixels lie only within native x1679..2955/y1389..1435;
blank controls above and below those notices are unchanged.

Color and glyph material remain separate from this placement correction.
For strong notice pixels, the source's median RGB is approximately
(229,48,48) on card two and (227,48,48) on card three. The previous
native capture measures (202,43,44) and (155,32,36), respectively;
card three uses semantic `Ink::Dim`; its reference-only ink correction is
recorded below. The source's softened glyph edges and small secondary
ink also differ from the current sharp Rajdhani rendering. Evidence is
in `/tmp/cp-eras-next/ah/neomil/login-trace-adjusted.png` and the
frozen source/current images cited above.

Astra's fresh Iced capture confirms the placement improvement. At a common
red threshold of 130, occupied native rows are 1389–1401 and 1409–1421,
versus source 1389–1400 and 1410–1421: the first line retains one extra
lower row and the second starts one row high. Across five thresholds and
first/middle/last notice crops, 24 of 30 native comparisons improve and
six are unchanged; SVG has the same count. The unchanged cases are high
thresholds at which card three's dim ink has no bright core. No segment
regresses. This validates placement, not exact letter contours or color.

Only 16,262 native 4K pixels and 4,619 pixels at 1600×900 change, all in
the two inactive notices. Blank bands and the active card are unchanged.
Seven matched fractional rest/hover/held/disabled/long-input/dark-caret/
custom comparisons each change the same 4,910 notice pixels. The rest
preview matches production exactly, and interactive differences stay in
the password/action region. Evidence is in `root-whole.json`,
`root-notice-montage.png` and `../state-review.json` under the AH directory.

Fractional review also exposes an existing width issue: at 1537×947,
the last notice letters extend approximately 5.3 and 5.7 pixels beyond
the two inactive card edges, identically before and after this change.
Login anchors/cards scale per axis, while text size uses the mean of
those scales. The comparison mask must account for that actual glyph
extent; assuming all text scales with x alone excludes 52 notice pixels.
`fractional-overhang.json` records the unchanged overhang. Containing those
runs without losing text is separate follow-up work.

## Locked notice reference ink — AI, 2026-09-30

The source's two inactive notices have effectively the same red print:
foreground medians over their two lines are RGB (229,48,48) on card two
and (227,48,48) on card three. Card three's former dim role produced
about (155,32,36) in the AH native capture and no pixels above R180 in
the SVG notice. Its two `Legend`s now retain semantic `Ink::Dim` and use
the existing `NOTICE` (#e63132) only for the source reference palette.
The trace uses that same fill, matching card two. Custom palettes retain
their dim role; the shared reference-ink resolution is checked in the
integrated review.

With the AH placement fixed, six source-aligned spans sample the first,
middle and last runs of both lines. At R>180 and R−G>85, SVG mask F1
against the source changes from zero to .390/.370/.089 on line one and
.339/.254/.096 on line two. In co-located bright cores, mean absolute
red-channel error falls from 88/75/85 to 47/31/44 levels on line one,
and from 76/97/82 to 42/59/42 on line two. At R>150, all six spans
also improve. The last spans remain weak because the source glyphs are
wider and softer than the current thin Rajdhani outlines. Trial fills
#f63333 and #ff3333 improve co-located bright cores further, but also
brighten unmatched SVG contours; a local color fit cannot resolve the
font-shape mismatch. `NOTICE` preserves the measured match between the
two cards and leaves that contour work explicit.

The final source comparison uses fixed first/middle/last spans and red
thresholds 105,130,155,180,210. All 30 comparisons improve in each renderer.
The SVG changes 5,114 pixels and the native 4K image changes 4,529, all
inside the locked notice. On source/old-SVG bright-core intersections
chosen independently of the new ink, native red-channel error drops
80.9→17.5, 80.3→16.2 and 80.6→18.0 on the first line, and 86.7→24.1,
98.1→43.5 and 87.7→29.5 on the second. These are small samples of
67/92/65 and 32/15/11 pixels; they support ink correction, not exact
font recovery. Evidence: `/tmp/cp-eras-next/ai/neomil/root-ink-results.json`.

## Narrow-window notice containment — AI

All three cards can overflow at narrower aspect ratios because anchors
use separate x/y scales while font size uses their mean. The renderer
measures complete note runs against each card interior, then uses the
smallest required font-size factor for the whole notice block. Tracking
scales proportionally; text, anchors, baselines, stretch and inks are
preserved. Reference 16:9 geometry returns unchanged. Notes without card
bodies and rotated margin legends are not fitted.

A horizontal-compression trial passes width tests but visibly breaks tiny
strokes when Iced switches from hinted glyphs to outline meshes. It is
rejected. Uniform size keeps the hinted path; using one block scale also
avoids different sizes for lines of the same notice. Font shaping is
measured again after shrinking to account for rounding. Seven layout
sizes, custom role/override resolution and interaction-coat precedence
have Rust coverage. Final native/state and repository verification are
recorded in [round twenty](../reference-svg-round20.md).

Final native review finds zero notice ink beyond all three card edges at
1537×947, 1200×900 and 900×1200, compared with 15/18/15, 84/76/82 and
326/80/326 pixels before the fit. Probes exclude card borders and nearby
cards. All six reference fractional state pairs change the same 5,489
notice pixels, while custom colors change 5,466; all other regions are
unchanged. A Dim-only custom palette at 1600×900 is byte-identical before
and after, and both canonical sizes preserve their original geometry.
`/tmp/cp-eras-next/ai/native-review.json` records the bounds, pixel-region
and state checks. The complete notice becomes smaller in a narrow window;
this fit does not recover the remaining source glyph/softness details.

## Active-notice echo investigation — AJ

A scratch trial places an active-notice copy 2.5 design pixels left and
down, using .7 stroke width, .45 Gaussian blur and .30 opacity. It retains
the primary text definitions. A small twelve-candidate grid uses the first
echo-only span of each line for fitting and middle/last spans as holdouts.
The chosen trial improves RGB RMS on both fitting spans and all four
holdouts, while three blank controls remain identical. It also brightens
some primary antialiased fringes: source-bright samples improve, but
already-mismatched glyph edges become slightly worse.

Corrected alpha diagnostics use the actual #e63132 fill and each
pixel's background; the four exact-fill pixels in the full primary window
remain unchanged. High-alpha samples have small losses, while fixed
source-bright samples improve. Astra's independent complete-notice spans
improve RGB RMS in all six regions and 40 of 42 threshold comparisons,
with one tie and one small F1 loss (.00168 at R180 on line two's middle).
That fringe tradeoff remains explicit.

The accepted lower copy keeps the primary renderer unchanged and applies
the same notice-block fitting to the secondary copy. Era-owned parameters
select a local glyph tile in a separate canvas below the primary artwork.
The cache follows bounds, palette and static slot data; custom palettes
omit the source-only copy. It does not add a full-screen raster layer.
Opacity is an approximation to SVG's sRGB compositing against the local
background, so native source measurements are separate.

A disabled 4K preview exactly matches AI. The enabled preview changes
14,311 pixels only inside the active notice and improves whole-region RGB
RMS in all six spans. Echo-only RMS changes 23.67→10.92, 22.71→15.14,
26.06→13.43, 13.51→9.07, 14.11→8.75 and 13.36→8.33; all eighteen
echo-only F1 comparisons improve. Complete-region masks retain some small
losses, up to .0017 F1. The final production 4K image matches that preview
exactly. A 900×1200 preview stays attached to the fitted primary and
changes 1,088 notice pixels. Fine scan repetitions and exact glyph forms
remain open. State/cache and integrated checks are recorded in
[round twenty-one](../reference-svg-round21.md).

Reproducible source scripts, twelve trials and primary/echo measurements
are under `/tmp/cp-eras-next/aj-login-echoes/` (`fit_softness.py` and
`diagnose_primary.py`). Native captures and review scripts are under
`/tmp/cp-eras-next/aj/`.

The final state review changes only the notice: 2,319 pixels in each of
six fractional reference states, while both custom-palette controls are
byte-identical to AI. The echo remains inside the active card at 1537×947,
1200×900 and 900×1200. The 4K cache audit exercises 86 draws and prepares
three 602×66 tiles only for initial and changed/restored slot data. Hover
and input changes reuse the cache; custom colors draw no echo. Local tile
creation takes 5.24–6.00 ms, with 158,928 retained RGBA bytes. This does not
measure total frame time or live hardware presentation.


## BG inactive-notice weight audit

The two inactive notice lines have fewer red-threshold pixels than the
photo. At R>155 in the 4K first line, source/current SVG/frozen AJ native
areas are 2472/1433/1838 on card 2 and 2306/1292/1792 on card 3. These are
ink-area differences, not proof that font weight is the cause. Contours,
hinting, phase and ink can all contribute; the source font remains unknown.

One scratch SVG changes only the two notice groups from Regular to Medium.
All fixed line-mask comparisons improve, but card 2 line-one RGB MAE worsens
17.249821→18.599176 at 4K, 13.753388→13.843722 at 1600 and
16.782153→17.097828 at 1537. The held-out card 3 line two also worsens at
all three sizes: 18.923392→20.352698, 15.379232→16.055896 and
17.719745→18.293465. The first-O control on card 2 has a small 4K RGB loss;
card 3 gains six false R>130 pixels inside a fixed source-dark counter mask.
The weight-only proposal is rejected before native work.

The trial changes 12739/3607/3311 full-RGBA pixels only inside the two
notice envelopes, with zero alpha, frame, interline-gap or exterior changes.
Astra independently reproduces every regional SVG RGB and threshold result
and verifies the only XML changes are two font-weight attributes. Original
4K pixels, full-photo Lanczos at 1600 and uniform top-left inverse-affine
Bicubic at 1537 support these SVG comparisons. The Login runtime's responsive
anchors and mean font scale do not follow that fractional projection;
unregistered fractional native scores are excluded. The AJ native evidence
at 4K/1600 is frozen historical output, not a fresh production trial.

Fixed pretrial controls, source/native provenance, renders and all changed
line-pixel RGB losses are in `/tmp/cp-eras-next/bg-neomil-login/`. This audit
supports examining glyph/counter shape before another weight or opacity
fit; it does not close exact printing or change the accepted AI/AJ artwork.

An isolated O/C/M study then compares cap-normalized Rajdhani Regular with
the already-bundled FreeSans Bold. One training occurrence per letter on
card 2 and four held-out repeats/card-3 glyphs use 12-row source caps at
R>155. With independent top-left glyph registration, FreeSans improves
every held-out threshold mask; average F1 at R>130/155/180/210 changes
.439/.364/.210/.020→.851/.819/.820/.606. Astra reproduces all 56 glyph/font/
threshold comparisons and verifies the actual resolved font bytes.

This supports one full-notice fit experiment, not font identification.
The FreeSans O hole has 41 pixels versus source 48/51, painting 7/10
source-dark counter pixels; C/M are one pixel wider than their source
glyphs. Sentence spacing, common baseline phase, native shaping and
responsive behavior remain untested. The study uses native-size 16px
FreeSans as a cap-derived starting point, with no horizontal stretching.
Evidence is in `bg-neomil-login/font-shape/` beside the preceding audit.

## BH–BJ full-notice geometry and local losses

BH's shared-tracking FreeSans trial fails three whole-line RGB holdouts.
Independent source-to-source registration then finds a +694-native-pixel
copy displacement, and both source second lines have a 380-pixel ink span.
BJ transfers that copy geometry and fits line two separately using actual
plain renders. All twelve whole-line RGB comparisons improve against the
original SVG, but 40/29/37 regional RGB losses remain at 4K/1600/1537.
Card 3's first-word MAE worsens 22.651→34.127; its first O paints 17/51
source-hole pixels, versus the original 5/51.

Severity review retains those scores while distinguishing contour/phase
losses from erased separators. All 30 word gaps remain open at four
thresholds and three sizes; both 4K initial O glyphs retain one counter.
The card-3 R>155 counter is smaller than source, 42 versus 51 pixels, and
small-size gaps can be only one pixel wide. This supports a bounded native
trial, not exact typography closure. Login's measurement and drawing need
the same per-legend face and metrics; FreeSans's .1-em hhea line gap makes
its centered baseline offset .95 for a 1.2-em line. Active AJ artwork and
responsive containment must be independently preserved. See
[round forty-two](../reference-svg-round42.md) for the full evidence and
the source/SVG/native registration limits.

BK's actual Login trial preserves production exactly in baseline mode at
all three sizes. With the new face, the first-line native glyph baseline
lands one row above the source at 4K/1600; three 1600 whole-line RGB controls
regress. The four inactive runs alone change, with active AJ artwork and
alpha preserved. A separate BL trial applies independently rounded baseline
placement only for the new unrotated, unstretched FreeSans glyph path.
Source geometry, font size, tracking and ink stay frozen. Fractional native
source scoring remains excluded until registered. Changed R>130 pixels have
only .51/.13 output pixel of right clearance; that difference mask excludes
unchanged pixels and does not prove full notice containment. Responsive
checks must measure the complete new notice before production acceptance.

BL's rounding aligns the first-line ink rows and improves all four 4K
whole-line RGB comparisons, but three 1600 lines still regress against
original production (13.030→13.186, 13.835→14.431 and 12.417→15.609 MAE).
Whole-line R>155 F1 improves for all eight measured lines. Fixed regional
losses remain 41/46 MAE and 98/75 threshold-F1 at 4K/1600; the initial O
still paints 18/48 and 19/51 source-hole pixels versus production 8/48 and
5/51. Astra reproduces all 1944 regional metrics. BL and BK are identical
at 1537×947. The candidate is not accepted for production; spacing, glyph
contours and full responsive bounds remain the next diagnostic tasks.

BM confirms that all 35 fixed gap control boxes retain a clear column at
four thresholds in both canonical sizes. The full R>155 notice masks at
1537×947 also remain within their card interiors, independently of changed
pixels; subthreshold and all-layout/state containment is still unverified.
BN's CLASS/TO profiles retain repeated contour and antialias losses. CLASS
loses internal clear-column runs, while the terminal O retains a smaller
counter when its shifted right edge is included in the existing source-line
fringe. This is diagnostic evidence, not permission to retune weight or ink
from thresholded area. Astra reproduces the gap, whole-word profile and
counter findings; all canonical losses remain recorded. Evidence is under
`bm-login-diagnosis/` and `bn-login-glyph-plan/` in the round's scratch root.
