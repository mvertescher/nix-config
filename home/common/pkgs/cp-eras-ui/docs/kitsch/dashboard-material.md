# Dashboard fan material fit

The idle fan faces in `dashboard-trace.svg` and `src/eras/kitsch.rs` now
use a red field tied to the dashboard's shared rose lobe, with green and
blue held at the accepted `#20858f` baseline. The ghost fill remains
`#0f9f80`, with its seven fill opacities
halved to 0.035 / 0.06 / 0.105 / 0.15 / 0.20 / 0.24 / 0.29, farthest to
nearest. The `#6cc4bd` stroke retains its original 0.16 / 0.24 / 0.34 /
0.45 / 0.56 / 0.68 / 0.80 ramp. The selected EVENTS face, labels, card
geometry, ghost spacing and motion are unchanged.

The comparison used `images/kitsch-dashboard.png` and librsvg 2.62.3 renders
of the dashboard trace at their native 3840×2160 size. Clear strips on each
idle card have `|t| < 60` and `18 < |s| < 21` in card coordinates, outside
the labels and outline. These strips were excluded from the earlier color
fit. Mean absolute RGB error, in channel levels per pixel, changed as follows:

| Card | Pixels | Previous face | Fitted face |
| --- | ---: | ---: | ---: |
| VEHICLES | 4,148 | 13.01 | 10.01 |
| WEAPONS | 4,149 | 13.50 | 8.57 |
| Left PRODUCTS | 4,092 | 21.52 | 9.97 |
| Right PRODUCTS | 4,032 | 14.45 | 7.60 |
| LOCATIONS | 4,146 | 22.79 | 10.94 |

Ghost interiors were selected from the original trace where ghost fill added
more than 20 green levels and its stroke fewer than 3. Halving fill opacity
reduced mean absolute RGB error from 21.67 to 9.19 over 262,190 native pixels
in the left fan, and from 23.43 to 12.48 over 228,885 pixels in the right fan.
The mask includes source/trace ground differences; the two fans agree on the
direction of the correction.

The first edge mask suggested halving strokes too, but it compared exact
pixels on outlines whose source and trace positions differ. Native line
profiles allowed a small normal-direction alignment. The near VEHICLES edge
peaked at 69 levels over its local field in the source, 77 with the original
stroke and 39 with a half stroke. The near right PRODUCTS edge was 59 / 73 /
37; the far VEHICLES edge was 24 / 26 / 13. The far right PRODUCTS edge
favored some fading (19 / 28 / 14), but its source outline was displaced
about 10 native pixels. Its isolated result does not justify changing the
shared stroke ramp. Source edge widths are about 1.5–2 native pixels versus
2–2.5 in the SVG, and changing opacity would not correct that softness.

A single translucent face was also tested against the rendered ground and
ghosts under each blade. A shared RGBA least-squares fit gave alpha 0.984,
effectively opaque. The remaining mismatch is the source's changing red rose
illumination, especially across right PRODUCTS, and its photographic edge
glow. A flat material does not reproduce either effect; `#20858f` was the
bounded common color that improved all five independent clear strips before
the rose-field fit below.

## P native verification — 2026-09-29

P native clear face strips reproduce the SVG exactly (maximum channel difference zero) on all five idle faces. Native/source MAE is 10.01, 8.57, 9.97, 7.60 and 10.94 RGB levels in the independently selected strips. Rest, held VEHICLES and opening at fractional size retain the original geometry and state behavior. P passed 269 Rust tests and all eight affected fidelity gates across four screens. This is the flat-face baseline for the following material correction.

## U rose-field correction — 2026-09-29

The source's idle-face red channel changes with the shared rose illumination:
right PRODUCTS falls roughly R58 to R16 down its clear strips, while WEAPONS
rises toward its upper/right end; the accepted flat face is R32 throughout.
The existing rose lobe is centered at (750, −331) with radii (1600, 929).
Using its sRGB red value at each design point, the bounded face model is
`R_idle = clamp(round(−3.777 + 1.234 × R_rose), 0, 255)` with G133/B143.
This is an empirical illumination fit, not an alpha or opacity model.

The two coefficients were fit only to alternating 12-unit bins on one side
of the clear strips on VEHICLES, WEAPONS and right PRODUCTS. On the opposite
side, red MAE falls from 12.37 to 3.21; on the alternating bins it falls
from 10.59 to 5.42. The entire left PRODUCTS and LOCATIONS strips were
excluded from fitting and improve from 19.34 to 1.38 red levels. A scratch
3840×2160 SVG render with 11 per-card gradient stops also improves mean
RGB error on wholly unused inner/end patches of every idle face:

| Idle blade | Flat face | Rose-field face |
| --- | ---: | ---: |
| VEHICLES | 10.51 | 7.17 |
| WEAPONS | 8.04 | 6.69 |
| Left PRODUCTS | 10.55 | 4.64 |
| Right PRODUCTS | 12.28 | 5.84 |
| LOCATIONS | 10.91 | 4.62 |

The face fields render in one leading software layer over the ghosts. The
foreground plate still paints the same outline and label; its selected and
pressed gold face covers the field. A scratch SVG layer split is byte-identical
at native resolution to filling the five idle faces directly, including the
selected EVENTS face over its extrapolated idle field. Its red model has no
source idle sample for EVENTS. At 3840×2160 the field adds 12,533,760 bytes
of retained RGBA payload, shared across all six held backdrops; this is a
cache payload figure, not total RSS. Three adjacent cold-draw trials with
the field (208–246 ms) and without it (210–237 ms) overlap, so this evidence
does not establish a preparation-speed change. Native edge and state fidelity
are checked separately. The source's finer glow,
photographic grain and residual 5–7 red levels on WEAPONS/right PRODUCTS
remain; no ghost stroke or geometry was changed.

## W ghost-edge core width — 2026-09-29

Native-size SVG trace comparison against the source showed a repeatable
outline-width residual after allowing each straight ghost edge a small
normal-direction registration shift. The bounded correction narrows the
shared ghost stroke from 1.2 to 0.9 design pixels while preserving all seven
stroke opacities, the ghost fill ramp, card geometry, positions and states.
The source bright-core FWHM is generally 1.75–2.25 native pixels; the prior
trace spans 2.5–2.75 and the 0.9 candidate about 2.0.

For three clear fit segments (near/far VEHICLES and near right PRODUCTS),
mean aligned G+B high-pass profile MAE falls from 6.23 to 5.14 channel
levels. Six independent clear segments across WEAPONS, LOCATIONS, VEHICLES,
left PRODUCTS and EVENTS improve from 7.14 to 5.70. The mean absolute
four-pixel core-area error falls from 34.5 to 13.1 on fit segments and
35.0 to 12.2 on holdouts. Red, green and blue channel errors each improve.
A 0.8-width variant has virtually the same profile MAE but worse core-area
fit, so 0.9 is the conservative result. Source high-pass profiles show no
repeatable positive outer tail beyond roughly three native pixels; no
separate blur/glow is supported. Far right PRODUCTS and LOCATIONS segments
need about 11–12 pixels of registration or intersect other outlines, so
they cannot validate this width correction. The RSVG profile result was
checked against the native renderer below; it does not establish a
whole-image or photographic-texture match.

## Right-fan depth registration and seventh silhouettes

The source has one more far contour behind right PRODUCTS and LOCATIONS than
the six previously drawn for each. Native straight-edge profiles measure
the source-to-trace shift at right PRODUCTS' top and right sides; fitting
near, middle and far depths gives an additional (−2.11,+2.39) native-pixel
shift per step. The untouched middle-depth positions miss that fit by about
0.20 native pixel, and the opposite left sides miss its horizontal positions
by 0.40 pixel on average. LOCATIONS' upper edge and left end independently
give (−1.98,+2.43) native pixels per step after rotation; its opposite end
agrees in slope. A shared (+19.15,−19.0) design-pixel pitch replaces
(+20,−20) only for these two stacks. The other four remain in place.

The seventh right PRODUCTS edge is visible independently on its top,
right and left sides. A .16-stroke copy is much brighter than the source:
candidate peaks 18.2/17.5/17.9 versus source 7.1/6.8/7.6. The bounded
.065 edge gives 7.2/6.8/7.3, with the left side held out. The seventh
LOCATIONS contour is visible on its upper edge and both ends. A .09 edge
gives peaks 13.7/14.3/13.7 versus source 18.9/9.1/15.0; overlapping
source contours limit this fit. All six previously drawn ghost opacities
on each blade remain unchanged.

Exposed seventh-card interiors were checked in separate local `t=−40..−10`
fit and `t=10..40` holdout strips at `s=−20..−13`, avoiding the strokes.
Mean source/RGBA-trace RGB error for right PRODUCTS falls from 12.90 to
12.07 on the fit strip and 14.53 to 13.43 on the holdout with the existing
.035 far fill. For LOCATIONS, .035 changes 6.04 to 5.91 on the fit strip
but worsens the holdout from 2.32 to 5.21; its new far contour therefore
has no fill. These errors include source/trace ground differences and are
local evidence, not whole-screen scores.

The right fan's resting clip expands to x663..1143/y200..635 to reveal the
new far ends. Its 0.45-second left-to-right opening and held-trail removal
semantics stay the same. The native rest result is checked below; opening and
held-state captures are separate checks.

## Left-fan far contours and count audit

The source VEHICLES trail has a seventh, faint far outline beyond the six
previously drawn. At the extrapolated position its upper, left-end and
right-end high-pass peaks are 10.9/8.5/11.8 native channel levels; the next
extrapolated step falls to roughly 2–3. A stroke-only 0.9-pixel outline at
(504.7,274) with .09 edge opacity gives 10.6/10.1/11.6 on those three
sides. Separate exposed interior strips reject adding .035 fill: mean RGB
error rises 9.69→10.48 on the fit strip and 7.20→7.76 on the holdout.

Left PRODUCTS likewise shows a seventh faint vertical contour. Its upper
long edge has source peaks 14.20 and 14.38 on separate along-edge fit and
holdout runs; a .10-edge copy at (598,435) gives 14.41 and 14.63. Its top
and bottom ends cross WEAPONS and neighboring ghosts, so those endpoints
cannot validate the added opacity independently. Exposed interior fit and
holdout strips support its .035 fill: RGB error falls 11.31→10.05 and
3.43→1.70. Both additions leave the six earlier positions, strokes and
fills intact; the left clip already contains them with margin, so its
opening bounds and timing do not change.

The remaining WEAPONS stack has seven supported visible ghosts; a putative
eighth lacks a coherent upper/left-end outline. EVENTS has five visible
ghosts; a putative sixth has no independent edge above local field, and
farther existing edges run under right PRODUCTS. Across the six earlier
VEHICLES depths, upper and end-edge registrations drift by only about
0.2 native pixel per step; left PRODUCTS' exposed upper edge stays within
0.5 native pixel and its exposed bottom-end drift is about 0.2 pixel per
step. WEAPONS' clear upper and right ends stay within about one pixel
across seven depths. These measurements do not support changing their
existing (+20,−20) pitch. EVENTS' near edges suggest a small shift but its
far holdouts are occluded, so its pitch and count remain unchanged.

## W native rest verification — 2026-09-29

The W native 3840×2160 capture confirms the narrower stroke core on all
seven preselected near/far profile segments. Source bright-core FWHM is
1.75–2.25 native pixels; V native measured 2.5–2.75 and W native 2.0.
Aligned high-pass G+B profile MAE improves on every segment, including
VEHICLES near 12.00→9.85, left PRODUCTS near 6.11→4.57, and EVENTS near
7.61→5.98 channel levels. Newly exposed right clip regions follow the SVG
change with native/SVG RGB-delta correlations .990 (PRODUCTS) and .992
(LOCATIONS), rather than merely showing an unclipped blank area.

An initial right PRODUCTS opposite-side probe appeared to miss by 5.5–8.25
native pixels at several depths. That median crossed the middle of cards
under nearer ghosts: the source high-pass edge peaks there are only
0.20–1.08 levels at depths 1–6, so its reported positions are noise maxima.
Two independent exposed strips near each card's upper end give the same
source-minus-W opposite-edge offsets across all seven depths:
`+1,+1,+1,0,+0.75,+1,+1` native pixels. The other exposed long side stays
within 0.25 pixel. There is no accumulating pitch error; the roughly
one-pixel fixed opposite-side offset could be apparent width or photographic
antialiasing. W still makes some occluded middle strokes more visible than
the source, and its exposed edge amplitudes differ at several depths.
The correction improves measurable core width and registration without
reproducing the source's photographic softness or all overlap material.
