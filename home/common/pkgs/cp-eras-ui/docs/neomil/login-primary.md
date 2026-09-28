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
match their source horizontal bounds. The source's secondary print
copies remain unresolved. Five clear badge-field medians support local
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
