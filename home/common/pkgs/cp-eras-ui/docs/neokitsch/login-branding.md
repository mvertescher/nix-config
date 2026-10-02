# Login branding trace (NK-04)

The source is `images/neokitsch-login.png` (#70, 3840×2160). Coordinates below are divided by 2.4 for the 1600×900 trace. The upper-left ARASAKA mark is custom lowercase stencil artwork. Its crisp foreground occupies x 92.08..272.08, y 63.33..86.67; the older y 59..88 description included the photographic glow. Four `a` counters, the detached cap of `s`, and small cuts at the `a`/`k` joins distinguish it from stretched Rajdhani.

The mark in `login-trace.svg`, both specimens in `components.svg`, and `LOGIN_LOGO_PATH` in `src/eras/neokitsch.rs` share one contour trace. I classified gold pixels in the native logo crop by channel relation (R > 140, R > 1.22G, G > 1.15B), removed components smaller than 30 native pixels, followed the foreground boundary, and simplified it by at most 1.5 native pixels. The path has seven outer components and four opposite-winding counters. It has no embedded bitmap or generated image. SVG and Iced use the same 189 absolute design-space points.

The original two-cell plate remains [90,100]..[275,125], divider x 195. The left cell keeps `57ASD4AV15AA`. The right cell keeps `COMBAT COLONIZATION` and `DEFENCE PROGRAM`. A first width-only fit at 10px/79% fit both lines inside the box, but the native Iced capture showed it was too thin, too tall, and too widely spaced vertically. The corrected lines use Rajdhani 600 at 8.2px, start x 201.5, baselines 112 and 119.6, with width scales 0.85 and 0.88. The left code is 8.4px/600 at x 95/y 113.7, 0.5px tracking and width scale 0.98. The tagline is 9px/600 at x 118.5/y 96.5, 0.8px tracking and width scale 0.90. These independent fits match the photographed cell dimensions while preserving its words and plate.

At 1600×900, the stencil crop x 90..274/y 62..88 improved from orange-ink intersection-over-union **0.40** for the plain uppercase label to **0.86** for the path. The root's first native Iced capture confirmed all four counters render. Caption measurements exposed the follow-up: the photographed tagline extends x 286..601 and has 1,809 thresholded pixels, while that first capture ended at x 550 with 884 pixels. The D native capture verifies the revised tagline at x 285..601 with 1,754 pixels; both right lines reach their source's measured x 485..639 and x 486..616, with vertical bounds within one native pixel. The left code ended at x 346 in D against the source's x 360 because the SVG inherited 0.5px letter spacing that Rust omitted. Rust now includes that tracking. The E native capture ends at x 359, matching the SVG, versus source x 360; its vertical bounds y 260..272 are within one native pixel of source y 260..271. The native crop contains 708 gold pixels in Iced, 660 in SVG and 637 in the source at the stated threshold. Local source/SVG/Iced review accepts the primary fit; glyph-edge softness and photographed bloom remain reconstruction limits.

The photographed bloom, exact source font, and fine curved edges of the stencil are residuals. Boundary simplification preserves the logo's overall area but leaves some angular corners where the native mark is smoother. This pass leaves the login entries, wire band and footer geometry untouched.

## Boxed letters and badge plates (NK-05)

The two login badges in #70 are independent transfer controls: the right one repeats the left plate 420 design pixels to the right but contains `B` instead of `A`. Both have a single external lower-right chamfer and a solid tab rising from the bottom border. The old login drawing used the dashboard's internal fold and thin 15px letters. The login trace and component specimen now use a login-only badge outline and tab; the dashboard's shared `letterbox` symbol is unchanged. The Iced access table retains each 26×26 badge's nominal bounds and supplies two measured plates through `badge_art`.

At 3840×2160, an orange mask requiring R > 150, R > 1.18G, and G > 1.12B isolates the source letters independently from their frames. Source `A` spans x 1008..1049, y 1125..1158 with 473 bright pixels; source `B` spans x 2019..2053 over the same y range with 704. The prior native Iced letters spanned x 1021..1037 and x 2031..2044, both y 1135..1157, with 110 and 141 pixels. The fitted 20px semibold Rajdhani letters use separate horizontal scales (1.70 for `A`, 1.65 for `B`) and centres (428.4, 848.2). Their SVG bounds are x 1008..1047 and x 2019..2052, y 1127..1157, with 565 and 679 pixels. Source/SVG glyph intersection-over-union rises from 0.049 to 0.724 for `A` and 0.060 to 0.621 for `B` compared with the prior native capture. The source-to-SVG glyph gap above the tab stays 12–13 native pixels in both badges.

The source plate edges span x 1000..1060 and x 2008..2069, y 1116..1178; the revised SVG spans x 999..1059 and x 2007..2067, y 1117..1178 at the same mask threshold. The solid tab starts at y 1171 in both source badges and y 1170 in both SVG badges; the old native had only a bottom edge at y 1177. Source/SVG tab intersection-over-union is 0.847 and 0.850, compared with 0.288 and 0.290 for the old native. These bounds and disconnected glyph/frame components persist at thresholds 120 and 180. The original typeface and photographic halo are unknown, and the revised `B` contour and frame edge still differ from the photo. These local measurements support the boxed-letter correction, not a claim that all login typography is complete.

Astra's final native review confirms glyph IoU .049→.717 for `A` and
.060→.699 for `B` at threshold 150. Frame IoU improves .295→.513 and
.293→.445; tab IoU improves .288→.847 and .290→.850. Native glyph bounds
are A x1008..1048/y1126..1157 and B x2019..2052/y1126..1157. The A remains
heavy: 605 native bright pixels against 473 in the source; B has 716
against 704. Frame edges remain 1–2 pixels displaced/short and the tab
starts one pixel above the source. These ink/contour limits remain open.

Independent fixed-rectangle measurements, without component selection,
improve all 18 native and all 18 SVG first/middle/last glyph overlaps
across thresholds 120/150/180. A clear gap patch contains zero source and
new native pixels at all three thresholds, versus six unsupported pixels
from the old fold in each badge. Both native letter-to-tab gaps are 12
pixels, agreeing with the source at threshold 150. Only 4,562 pixels
change at 4K, all within the two badge crops; 844 pixels change at
1600×900. The tracked SVG renders byte-identically to the accepted
scratch proposal. Native evidence is under `/tmp/cp-eras-next/ag/neokitsch/`;
all seven paired 1537×947 rest/hover/held/disabled/long-input/blink/custom
states change the same 931 badge pixels, with all other pixels identical.
State-to-rest differences remain inside the original field/action regions.
The [AG checkpoint](../reference-svg-round18.md) records all six affected
gates, 286 Rust tests, 22 repository checks and 27 exact visual matches.

The subsequent A-only contour fit follows the photographed letter's narrow
apex, straight slim legs, crossbar and triangular counter. Its outer and
counter vertices are expressed in the same design coordinates in the login
trace, the login component specimen and the native badge art. The native
counter winds opposite the outer contour; the B glyph, both plates and tabs
retain their earlier geometry and ink. At 3840×2160, the traced A has 486
bright pixels against 473 in the source and 565 in the previous SVG. Its
orange-mask overlap rises from 0.724 to 0.914 at threshold 150; independent
left/right leg, apex, crossbar, counter and foot crops improve at thresholds
120, 150 and 180. The A badge's RGB mean absolute error falls from 27.39 to
24.72. The clear letter-to-tab gap still has zero bright pixels at all three
thresholds, although 55 low-level halo pixels raise its RGB error slightly
from 5.96 to 6.00.

AJ native review confirms the contour at source resolution: IoU rises
from .717 to .917 at threshold 150. All 42 whole/part comparisons at six
thresholds improve, as does RGB error in every glyph region. The complete
glyph crop falls from 29.36 to 24.56 RGB levels. Only 286 native pixels
change at 4K, 61 at 1600×900, and 71 in each of seven 1537×947 reference
or custom states. The plates, tabs, B and native letter-to-tab gap remain
pixel-identical. Interaction changes remain inside the original controls.
The supplemental downsampled-source check has one small high-threshold
left-leg loss among 42 comparisons; it is not a separate source capture.
The exact photographic glow/material remains open. Integrated repository
validation is recorded in [round twenty-one](../reference-svg-round21.md).

## AK B contour

The B uses a source-derived outline with a slimmer spine and two larger
counters. Native counter contours wind opposite the outer path. The
existing CAPTION ink, plate, tab and A are preserved; the component sheet
shows only the A login excerpt, so it needs no B update.

At source resolution the accepted geometry's SVG RGB MAE is 18.58,
versus 32.73 for the font approximation. It improves 87 of 88 fixed
whole/part comparisons at eleven thresholds from 100 to 200; the lower
counter at 170 loses .0026 IoU. The initial contour's high-threshold
lower-counter losses led to a bounded source-geometry correction, not a
change in ink or shared halo. These same regions are diagnostic after
that revision; they are not untouched holdouts.

The trial-disabled native preview is pixel-identical to AJ at 4K. Native
RGB MAE falls 41.73→32.68; 87 of 88 native/source comparisons improve,
with one .00292 IoU loss in the upper counter at threshold 100. Exactly
405 native pixels change, confined to x2019–2053/y1125–1158. A, both
frame bands, the solid tab and clear gap remain pixel-identical. This
corrects the letter contour; the photographed glow and ink remain open.

The SVG's shared halo changes three pixels in the broad left frame band,
69 in the right band, and 196 in the letter-to-tab gap. Clean edge-only
bands and solid tab/A controls remain identical. Gap RGB MAE worsens
3.612→4.664, with no new bright gap ink. Native rendering has no matching
halo and leaves the gap unchanged. These renderer-specific tradeoffs
remain part of the evidence, not hidden by broad gate scores.
Seven fractional states (rest, hover, held, disabled, long input, dark
caret and custom palette) each change only the same 144 B pixels. All
state-to-rest feedback deltas are identical before and after. The 1600
capture changes 121 pixels only in B. All six affected gates, 288 Rust tests and 22 repository checks pass;
all 27 visual cases pass first attempt.
Evidence is under
`/tmp/cp-eras-next/ak-neokitsch-b/`; see
[round twenty-two](../reference-svg-round22.md).
