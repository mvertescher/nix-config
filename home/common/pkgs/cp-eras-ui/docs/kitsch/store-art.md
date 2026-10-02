# Kitsch store weapon and socket trace

The reference is `images/kitsch-store.png` (3840 × 2160, 2.4 native pixels per 1600 × 900 scene unit). This page documents reconstruction of visible pixels, not the original author's asset files. The same MAGNUM illustration appears in Entropism and NeoKitsch; source-only silhouette alignment against the Entropism plain-card crop gives Kitsch IoU 0.953 at scale 1.00 and interior luminance correlation 0.970. The Kitsch plain card starts at (484,218), with the weapon at local x37–236/y129–183. The shared `magnum_art` paths are translated by (−2,+26) scene units. The selected card starts at (804,218), with its own weapon at local x38.5–237/y115–169. Its photographed inverse art is traced separately because the seam mask is not just a recolor of the plain art.

The plain source mask is mint (G>120, B>100, G>R+30) in native x1225–1760/y790–1005. Shared source-native body and bright-metal paths give 0.911 source/SVG mask IoU at the measured transform, with 0.994 recall but 0.916 precision: Kitsch prints sharper dark seams than the Entropism sample. A Kitsch-only subtraction mask is restricted to pixels inside that projected shared body whose source G and B are each <120. Its 244 rings / 1,352 vertices are simplified at 0.6 native pixels after dropping components/holes under 3 native pixels. The subtraction is filled in the source dark `#0e0e0d`. Plain mint-mask IoU rises to **0.968**, precision 0.974, recall 0.994. The base and bright tiers are source mid mint `#79cdb8` and bright mint `#93ffe4`; pressed and lifted variants recolor the same geometry with their existing card material inks.

The selected source mask is dark (R<120/G<100/B<80) in native x1990–2535/y750–970. Its outer and core contours have 199 rings / 2,792 vertices and 183 rings / 2,544 vertices. Components/holes below 3 native pixels are removed, rings below area 3 omitted, and boundaries simplified at 0.6 native pixels. The two dark inks `#3a2408` and `#302010` leave the selected amber fill visible in rail holes, receiver channels and the butt slot. Native selected source/SVG dark-mask IoU is **0.988** (precision 0.999, recall 0.989). These SVG scores derive from the thresholds used to draw the paths; a native Iced capture is still needed. No bitmap is embedded, and the large shared raw path arrays are not duplicated in Kitsch.

The card-1 socket glyph at native x1185/y1201 has 25 cells on a 9×9 occupancy grid, 3.49 scene-unit pitch and 3.8-unit cells. It repeats the same scatter as Entropism's MAGNUM cards. The old 14-cell grid's native source/SVG mint-mask IoU was 0.239 in x1183–1265/y1198–1279. The corrected grid at card-local x9.6/y282.5 reaches **0.873**, with selected sockets at the same local x and y422.5. The certification and warning marks in the yellow shelf band already had detailed SVG geometry; Rust now carries their corner ticks, two micro-label strokes, SC knockout, inner C, triangle point and seven warning rules. The source mark crop is still photographic, so their tiny letterforms and seven rules should be judged as approximate vector readings.

The shelf/card outlines, fourth-card residue and hit viewport, card typography, selected growth, ghost steps, and material timing remain as previously traced. The component sheet copies the new rifle and socket definitions with `k-` ids. Its main card and selected example use the same source-grounded paths as the parent trace. Source grain, glow and sub-native-pixel engraving remain material residuals.

## M certification pass

The source band at card-local x−17..92/y74..92 has a heavy frame around RG5, a dark square around a light disc marked SC, a double angular C, and a hollow warning triangle. The previous SVG/runtime had thin corner ticks, a bare dark disc and a solid triangle. The source crop is `images/kitsch-store.png` native x1094..1392/y696..744; `/tmp/k-band-source-large.png` is a temporary 3× inspection crop. The four marks now use the same measured paths in the parent SVG, component sheet and runtime. Knockouts follow the band's normal yellow or selected amber fill. The warning micro-print uses 97 row contours from the source's dark pixels, sampled in 2-native-pixel cells after excluding isolated grain. This retains the source's illegible print pattern without inventing words; its exact photographic softness remains unresolved. The refreshed SVG crop is `/tmp/k-band-m-crop.png`. Native M review is pending.

Native M review resolves the inset certification mark as three bars on
a vertical stem, with angular upper/lower tips, rather than a second small
C. Its native stem is near x1203, and bars near y717/725/734; the new inset
uses a 0.65 design-pixel stroke in both SVG and Rust. This also removes the
M mismatch between SVG 1.3 and Rust 0.8. Native confirmation is pending.

## Integrated validation — 2026-09-29

N native review accepts the corrected full-height inset three-bar mark and the broader certification silhouettes. The source/SVG/Iced crop confirms placement and continuity; exact tiny RG5/SC glyphs, coarse warning microprint and photographic softness remain outside that closure. All 22 repository checks pass.

## SC corner apertures — 2026-09-29

The original SC mark also has four small light apertures inside the dark square, outside the central disc. The prior square/disc path omitted them. Four compact triangular contours were fitted to card 1's source pixels and checked without moving them against selected card 2 and ordinary cards 3–4. The traced SVG uses even-odd cutouts; runtime paints the same shapes in the disc's band ink, so ordinary yellow and selected/held feedback retain their existing color mapping. The square, disc, SC letters, and adjacent certification art stay in place.

At 3840 × 2160, RGB absolute error across fixed corner patches falls by 17.1% on card 1, 8.7% on selected card 2, 15.0% on card 3, and 13.9% on card 4. The SVG trial changes 178 pixels, all inside the four corner patches on each card; its central disc and SC letter region are pixel-identical. Source/current/trial native-pixel and enlarged crops, exact bounds, and conservative false-cutout counts are in `/tmp/cp-eras-resume-20260929/ab-k8-fit/findings.md`. The lower-right aperture on card 4 is the weakest holdout because its photographed opening is softer and closer to the square edge. Exact photographic softness and the warning microprint's unreadable contours remain unresolved; fresh AB native captures now verify the geometry. Fifteen of sixteen
native corner patches improve, as does every card's combined score; the
fourth card's lower-right patch worsens20.06→22.63. Fractional held,
selected-last, custom-color and opening captures retain the band/knockout
relationship and clipping. Both gates and the full AB check pass; see the integrated acceptance below.

### AB integrated acceptance

The source/native/state review is integrated: all 286 Rust tests and 22
repository checks pass, including 27 exact visual cases on their first
attempt. All 199 frozen file hashes match the Nix source. This closes the
bounded AB correction above; its stated photographic/glyph limits remain.

## Rejected SC letter-opacity trial — 2026-09-29

The photographed SC letters are lighter than the surrounding dark square.
A local SVG trial reduced only their fill opacity to 0.4, leaving the disc,
four corner apertures and neighboring marks pixel-identical. At 3840 ×
2160, source/SVG RGB error in the fixed SC crop fell from 20.32→12.01 on
card 1, 25.04→12.25 on selected card 2, 22.06→9.94 on card 3 and
25.13→21.18 on the cropped fourth card. The card offsets were measured
from the trace as 0/768/1534/2302 native pixels. This is a valid local SVG
fit, but it did not transfer to the native renderer.

In native Iced captures, 0.4 made the lettering too faint: source/native
dark-core F1 at 25 luminance levels below each disc's blank interior fell
to zero on all four cards, and card 4's middle-crop RGB error worsened
25.32→29.72. Trials at 0.7, 0.85 and 0.95 regained visible ink and lowered
whole-crop RGB error, but none preserved dark-core agreement across the
selected and ordinary holdouts. Even at 0.95, the card-1 F1 improved
0.726→0.742 while cards 2–4 fell 0.372→0.296, 0.636→0.571 and
0.722→0.667. Every native trial changed only SC text pixels in the badge;
the surrounding badge and blank-disc controls were pixel-identical after
including native antialiasing pixels in the text exclusion.

The scene renderer converts translucent ink against the palette ground
before drawing it over the yellow disc, so the same opacity does not have
the SVG's compositing effect. No SC-opacity change is accepted. Its native
glyph coverage and phase against the source need local investigation before
another ink fit. The source's tiny contours and photographic softness remain
uncertain; source, SVG and native trial crops and metrics are in
`/tmp/cp-eras-next/ad-kitsch`.

## SC type coverage — 2026-09-29

The source's SC dark core on card 1 spans native y721–727, while the
previous Bold 4.5 native glyph reached y721–728 and covered more of the
disc. A bounded type fit keeps its x origin and opaque band ink, and uses
Rajdhani SemiBold 4.3 at baseline y85.5 in the trace, component sheet and
runtime. The square, disc, four corner apertures and neighboring marks are
unchanged. The same placement is used by ordinary and selected cards.

At 3840 × 2160, source/native dark-core F1 at 25 luminance levels below
each disc's blank interior changes across cards 1–4 from
0.726→0.838, 0.372→0.379, 0.636→0.769 and 0.722→0.649. At 35 levels it
changes from 0.538→0.714, 0.127→0.146, 0.451→0.653 and 0.564→0.679.
The card-4 loose-threshold loss is real: the native glyph matches a net 11
fewer of its 38 source-core pixels. Its strict dark core
improves, and the SC-box RGB error falls on all four cards (21.48→16.85,
29.16→20.97, 21.62→14.34 and 25.81→24.07). The comparison uses a fixed
text exclusion derived from the previous SVG opacity trial and native
baseline, with a one-pixel antialiasing margin. Blank-disc controls are
pixel-identical; all 336 changed native pixels stay inside the four SC
text boxes. A weight-only SemiBold trial at the original size and baseline
lost dark-core agreement on seven of eight card/threshold comparisons.

The source has different photographic softness and pixel phase across
copies, especially selected card 2 and cropped card 4. The common type fit
does not resolve their exact SC contour or the surrounding unreadable
microprint. Native crops and the fixed-mask comparison are in
`/tmp/cp-eras-next/ae/kitsch/primary-montage.png` and
`/tmp/cp-eras-next/ad-kitsch/ae-root-geometry-metrics.json`.

Ten integrated captures preserve ordinary/selected/held artwork, custom
disc colors, early opening and the fourth-card cutoff. Matched deltas stay
inside the SC runs; the production 4K capture exactly matches the trial.
The reviewed 1600×900 golden changes 96 SC pixels. Both store fidelity
gates pass at unchanged thresholds; see the
[AE checkpoint](../reference-svg-round16.md) for repository validation.

## Warning triangle height — 2026-09-30

The source's hollow warning triangle is shorter than the previous trace and
runtime contour. On ordinary card 1 its dark pixels end at native y736 at
R,G<170, while the previous SVG and AE native capture continue through y738.
A bounded SVG trial moves the outer tip from local y77.6 to 78.2 and the base
from y90 to 89; its inner opening moves from y81/88.7 to 81.2/87.8. The
exclamation, warning frame, microprint and other certification marks stay
fixed. The same contour is now in the store trace, component sheet and Rust.

In a fixed native-pixel triangle crop, source/SVG dark-mask F1 at R,G<175
changes .409→.596 on card 1, .250→.336 on selected card 2, .348→.499 on
card 3 and .438→.589 on card 4. All four improve at thresholds 160 and 190
as well; the first, middle and last horizontal thirds improve on every card
at thresholds 175 and 190. At threshold 160, card 3's middle third slips
.136→.126, and the selected photograph has only five dark pixels, so that
copy is judged at the looser
thresholds. The SVG trial changes only the warning contour and leaves patches
above and beside it pixel-identical. The accepted trace render is byte-for-byte
identical to the scratch trial at 3840×2160. The photograph's rounded base
feet and softness remain outside this height correction.

Astra's independent native crop uses design x29..44/y76..91 relative to
each card. At threshold 175, native F1 improves .409→.606, .255→.349,
.343→.516 and .432→.602. All 12 whole-triangle comparisons improve across
thresholds 160/175/190. Of the 36 native first/middle/last segment checks,
34 improve and one is unchanged; card 3's middle at 160 slips .1437→.1419. The selected source
has only five pixels at 160, so that threshold is weak evidence there.
Only 952 pixels change at 4K, all inside the four warning crops. The
1600×900 capture changes 168 pixels. At 1537×947, matched rest, held
ordinary/selected, selected/held last-card and custom-color states each
change 172 pixels; the early opening changes 130. All pixels outside the
warning crops are identical, including the other certification marks,
selection feedback and fourth-card clipping.

Scratch SVG, source/native montages and threshold measurements are in
`/tmp/cp-eras-next/ag/kitsch/`; state captures and integration evidence are
under `/tmp/cp-eras-next/ag/`. See the [AG checkpoint](../reference-svg-round18.md)
for all six affected gates, 286 Rust tests, 22 repository checks and 27
exact visual matches.

## AN certification lower frame and printing

The small framed mark has a deeper, chamfered inner lower opening in all
four photographs. Its former rectangular opening ended too early and made
the lower frame too thick. AN extends the outer bottom from 89.5 to 89.9,
the inner opening from 87.8 to 89.0, and joins the lower corners from 87.3.
The outer top and sides stay in place. An earlier trial that widened them
failed other-card controls and was rejected. The final values followed
those comparisons, so the four-card results are transfer checks, not blind
validation.

The existing tiny `RG5` reading is retained without claiming that the
photograph verifies those characters. Its baseline moves 83.8→83.05,
tracking becomes .45, and SVG opacity becomes .35. Two faint lower
strokes restore visible printing without inventing an unreadable word.
Their SVG opacity is .20. Native opacity .66531/.48855 compensates for
canvas alpha rebasing against a dark ground when the actual field is
yellow. These are calibrated luminance values, not identical per-channel
compositing. The knockout still uses the live selected ink. SC, warning,
weapon, socket and band artwork are unchanged.

| Source RGB MAE at 4K | Card 1 | Selected 2 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: | ---: |
| Whole mark, old native | 31.77 | 37.06 | 31.27 | 35.26 |
| Whole mark, corrected native | 26.08 | 32.02 | 25.39 | 29.57 |
| Upper printing, old native | 27.86 | 31.02 | 29.14 | 34.11 |
| Upper printing, corrected native | 15.06 | 17.91 | 14.08 | 22.17 |
| Lower frame, old native | 41.95 | 46.93 | 40.90 | 44.38 |
| Lower frame, corrected native | 25.88 | 32.60 | 24.76 | 28.09 |

Whole-mark crops are x1118..1152/y707..741, shifted horizontally by
0/768/1534/2302 for the other instances. Upper-print crops are
x1126..1142/y716..723; lower-frame crops use y733..741. Source green
thresholds below 160, 160..179 and at least 180 split the upper dark,
edge and light controls; all twelve native errors improve. Only 13 pixels
support the selected dark control, so that statistic is weak.

All 662 native changed pixels and all 1,074 SVG changed pixels stay in
the four marks. The shared lower line slightly worsens selected-card RGB
error: SVG 5.55→6.38 and native 10.18→11.27 in the fixed
x1126..1142/y728..732 crop. The other three line crops improve. Exact ink,
unreadable glyph contours and photographic softness remain unresolved;
this does not close all K8 printing. Native state and integrated checks
are recorded in [round twenty-five](../reference-svg-round25.md).


## AO lower-print phase audit

The selected RG5 lower-rule loss prompted a geometry-only scratch trial:
moving its two strokes from y86.4 to y86.65 improves the selected SVG
crop error 6.384→5.608, changing 22 pixels. Other copies and surrounding
art remain identical. A shared shift worsens two ordinary copies, and
wider strokes worsen the selected crop. None is applied.

Raw source green values initially suggested a lower selected stroke.
Independent background controls do not support that inference. In the
ten-pixel main-stroke core at native x1132..1141 plus each card offset,
interpolation from adjacent blank rows y728–729 and y732–733 gives
ink-deficit centroids 730.352/730.450/730.459/730.402. The selected copy
and ordinary card 3 therefore agree within .01 native pixel. Narrower
cores and alternate adjacent gaps reveal background/halo sensitivity,
not a stable separate selected contour. A native geometry trial is not
justified by this evidence. The existing residual stays open; retain
AN's geometry rather than fit a special offset to photographic variation.

Source-only measurements and the rejected candidate are under
`/tmp/cp-eras-next/ao-kitsch-lower-print/`; Astra's independent recomputation
is `/tmp/cp-eras-next/ao/kitsch-source-review.json`.


## AQ SC ink audit

With the AE letter geometry retained, same-card blank-disc and dark-square
normalization still finds the photographed letters lighter than native.
Matched source/native contrast ratios are .434/.720, .306/.723,
.388/.735 and .477/.735 across the four cards. There are too few opaque
interior pixels to recover ink independently of blur and glyph phase:
four-neighbor erosion leaves zero pixels in the SVG's high-coverage mask,
and only 4/0/0/2 in the photographed dark cores.

A single card-1-derived opaque SVG ink improves ordinary median contrast
but remains too strong on the selected card and at strict dark-core
thresholds. It is rejected; no SC artwork changes. The corrected trial
uses the frozen AP trace and changes exactly 274 pixels inside the four
SC glyphs. Review caught an earlier trial accidentally including AQ's
concurrent compliance experiment; that contaminated comparison is retained
separately and is not acceptance evidence. See the reproducible scratch
record in `/tmp/cp-eras-next/aq-sc-ink/`.

## AY warning bridge and feet audit

The photographed warning triangle has a raised central underside between
its two feet. A bounded SVG trial raises that bridge from local y89 to
y88.6 with two curved inner joins, retaining the tip, long sides, opening,
exclamation and outer extent. Collinear outer segments do not reconstruct
the photographed rounded outer corners; that remains a separate limit.
All four 4K whole-foot RGB and threshold controls improve in SVG. The
fractional comparison uses an exact uniform, top-left coordinate map;
two strict-mask losses remain on ordinary cards 3 and 4.

The direct native port is rejected. Its baseline matches a fresh capture
of the verified AX package at 4K and the current 1600px golden exactly.
Only 118/28/25 pixels change at 4K, 1600×900 and 1537×947, all inside the
lower triangles. The opening and neighboring marks remain unchanged.
Whole-triangle 4K RGB error improves on all four cards, but three left-foot
controls worsen. At 1600px the bridge loses too much dark coverage.

Independent source-resampling controls distinguish real losses from
sampling artifacts: 33 of the 55 overlapping crop/threshold regressions
disappear with Lanczos prefiltering instead of pointwise cubic sampling.
The other 22 persist. Card 1 bridge F1 at threshold 190 still falls
1.000→.444; card 4 bridge F1 at 175 falls 1.000→.444 and its left-foot
RGB error rises 23.667→26.241. These counts include overlapping regions,
not independent defects. The exclamation crop includes the changed bottom
row, so its regression does not imply that the exclamation geometry moved.
The direct port is not supported by the native small-size evidence.

Source/SVG trials, frozen native harnesses, measurements and montages are
under `/tmp/cp-eras-next/ay-kitsch-triangle/`,
`/tmp/cp-eras-next/ay-native/` and
`/tmp/cp-eras-next/ay-warning-review/`. No direct-port artwork is integrated.

One native calibration raises the bridge only to y88.8, with inner cubic
controls at y88.9. The source-fitted SVG retains y88.6; each renderer has
its own subpixel coverage. This calibrated native trial improves all
twelve 4K whole-triangle F1 checks and retains every tested dark mask at
both smaller sizes. All four lower-triangle RGB errors improve at every
size. Only 58/24/24 native pixels change; opening and neighboring-mark
controls remain exact.

| Native lower-triangle RGB MAE | Card 1 | Selected 2 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: | ---: |
| 4K | 26.637→24.777 | 32.834→30.418 | 26.220→24.135 | 30.117→28.807 |
| 1600×900 | 24.363→19.726 | 31.652→26.141 | 25.652→20.867 | 28.119→26.341 |
| 1537×947 | 21.778→17.069 | 31.696→26.185 | 25.444→20.264 | 29.333→25.674 |

These smaller-size values use exact pixel-center cubic source sampling;
the independent 1600px Lanczos check also preserves all dark-mask scores.
Local losses remain: 4K left-foot errors rise .886/.320/1.014 on ordinary
cards 1/3/4, with small dark-mask losses; card 3's strict bridge F1 falls
.1304→.1250. At 1600px card 4's left-foot error rises .630; at fractional
size cards 1/4 rise .037/1.289. Astra accepts the bounded bridge improvement
with those residuals, without claiming exact feet, softness or ink. The
selected source has few pixels below threshold 160, so that strict-mask
score alone is weak evidence; the separate RGB and other-threshold
controls remain necessary.
Nine paired state captures preserve every pixel outside the lower warning
regions: four first/last category/card combinations, three synthetic
pressed-material views, a custom palette and an opening frame. The first
eight pairs change 24 pixels each; the opening pair changes 12 pixels in
the two revealed marks. These are rendering-state checks, not live pointer
replay. Component-sheet changes remain inside the two lower warning
regions at full and half size (19 and 8 pixels). All three production
sizes and a fresh packaged 4K frame match the reviewed native calibration.
Both fidelity gates, 292 Rust tests, all 22 repository checks and 27
first-attempt visual cases pass. Only the Kitsch store golden changes,
by 24 pixels. See [round thirty-six](../reference-svg-round36.md).
