# Entropism store value typography

The four value runs `86 / 30 / 5 / 5` on each product card were too short and thin in the N store SVG and native render. At 3840×2160, the photographed values occupy y1135/1136..1173; the old 22px Rajdhani Regular runs occupy y1142..1175 in SVG and y1144..1177 in Iced. This fit changes only those values. The stat labels, compliance, sockets, art, frames, inks and literal content retain their existing forms.

The SVG values now use Rajdhani SemiBold at 24.5px with local baseline 229.1667. This yields y1136..1173 on the ordinary cards and y1136..1173 on the selected card, whose source top is one native pixel higher. In Rust, the corresponding `Prim::Wide` runs start at local y228.3334: the earlier N native capture placed the same Rajdhani run two native pixels below librsvg, so the Rust-only 0.8333-design-pixel lift is a starting calibration for native review. The plain card template serves cards 2–4, including the viewport-clipped fourth; the grown card serves card 1. Hover and pressed states inherit the geometry and retain their existing semantic ink inversions.

The two-digit widths are local fits. Ordinary `86` is centered at x41.4167 and stretched 0.97; ordinary `30` keeps x102 and width 1.0. Selected `86` and `30` are centered at x41.4167 and x102.4167, each stretched 1.07. Both `5`s retain x164/225 and width 1.0 in both states. At a dark R<100/G<120 threshold, selected `86` reaches x1177..1234 against source x1178..1235 and selected `30` reaches x1323..1380, matching the source. Ordinary card 3 `86` reaches x2725..2777 against source x2725..2778. The ordinary `30` retains a glyph-dependent 1–3px horizontal residual; shifting/compressing it only marginally helped card 3 and worsened held-out card 4.

Weights were compared **after** cap height and baseline were matched, using selected/card-3 `86/30` as training glyphs. Mean dark-mask IoU at R<100/G<120 is 0.291/0.391 for Regular, 0.381/0.531 for Medium, and 0.450/0.589 for SemiBold before width adjustments. The final local width fit raises SemiBold to 0.561/0.652. Held-out selected `5`s, ordinary card-3 `5`s and card-4 `86/30` score 0.599, 0.504 and 0.663 versus the old SVG's 0.178, 0.171 and 0.131. The SemiBold preference for the training digits holds at R thresholds 80/100/120 and when source/SVG red channels are normalized to comparable local ink alpha. The `5` holdouts weakly prefer Medium at the highest R threshold, so this is a bounded improvement, not an exact source-font identification.

Compliance stays Rajdhani Medium. A SemiBold compliance trial overprinted its bright core, while four available alternative sans families were much wider and taller before squashing. The photographed grain, soft glyph edges and remaining contour differences are outside this value geometry fit. Native Iced verification of the new value size/weight and baseline is pending.

## W native review

At the middle dark-mask threshold, actual Iced/source overlap rises from
.083 to .446 for selected two-digit runs and .113 to .602 for plain card 3.
Held-out selected/plain `5`s improve .120→.608 and .129→.511; card 4's
two-digit runs improve .109→.614. All groups improve at three thresholds.
Native review also finds the stretched runs one source pixel lower than
the SVG, while unstretched `30`/`5` baselines already agree. A scoped X
calibration is being checked before final integrated validation.

## X native baseline calibration

The W capture places stretched `86`/`30` caps at y1137..1174, one native pixel below their SVG boxes at y1136..1173. Unstretched ordinary `30` and the `5`s already occupy y1136..1173. Rust therefore lifts only runs whose horizontal stretch differs from 1.0 by 0.416667 design pixels (one pixel at 3840×2160); the SVG and all unstretched runs keep their positions. This leaves the selected source's y1135 top one pixel above the candidate, while matching the ordinary source top and the shared y1173 bottom.

A scratch translation of the W native dark masks tests the intended renderer-only shift before a fresh capture. At R<100/G<120, IoU for the selected `86` first glyph rises .481→.609. Independent selected `86` second glyph rises .437→.549, selected `30` .434→.529, ordinary card-3 `86` .590→.669, and ordinary card-4 `86` .619→.718. All stretched holdouts improve at R thresholds 80, 100, and 120. Ordinary `30` and selected/plain `5` masks remain unchanged. The scratch comparison and glyph crops are in `/tmp/cp-eras-resume-20260929/w-e-type/x-baseline-trial.{txt,png}`; X native verification is still required.

Actual X native captures confirm the calibration: at the middle threshold,
selected two-digit overlap rises again from .446 to .554, plain card 3
from .602 to .642, and held-out card 4 from .614 to .664. All three
thresholds improve; the unstretched `5` controls are unchanged. This
fixes raster baseline placement, without claiming exact font outlines.

## AF product-header baseline fit, 2026-09-29–30

The two product-header runs (`MAGNUM 650`, `HAND GUN`) have a repeatable
vertical offset distinct from the already fitted value row. At native
3840×2160, isolated cap rows of the selected title are y649–683 in the
photograph, y650–685 in the frozen SVG, and y652–688 in the frozen Iced
capture. Ordinary card 2 and held-out cards 3–4 repeat these title rows. The
subtitle spans source y699/700–729, SVG y701–731, and Iced y702–733.
The source width of card 2's title is 304 pixels (x1910–2213); SVG is
321 (x1913–2233), and native is 299 (x1913–2211). The SVG title is
overwide even though the native run is close in width. Measurements use
fixed crops that include the final visible glyph. Scratch width
trials failed independent glyph controls, so this pass leaves horizontal
forms unchanged.

The bounded correction moves Iced's two local header runs up 1.5 design
pixels. Only the SVG `HAND GUN` subtitle and its copied component examples
rise 0.5 design pixel; the SVG `MAGNUM 650` title stays at its original
baseline. A title lift worsened the first and last glyph segments on all
four cards at the middle threshold. The native offset differs because
frozen Iced raster starts lower than librsvg. The retained SVG subtitle
now spans y700–730, one pixel below the source at its bottom. No letter spacing, size,
weight, ink, words, breaks, card frames, rifle, value or socket art changes.
The fourth title remains cropped at the established viewport.

Scratch trials of letter-spacing, x origin and size were rejected. Some
improved total RGB or mask overlap, but lost isolated first/middle/last
glyph segments on selected or held ordinary cards. In the frozen native
capture, a four-native-pixel upward mask translation improves all three
title segments on card 1 and all observed title/subtitle segments on
cards 2–4 at the middle stroke threshold; the first `HAND GUN` segment
on card 1 loses mask overlap. The retained SVG subtitle improves 11 of
12 first/middle/last segments at the middle threshold; its selected
last segment slips slightly (.0790→.0787). Dark blank bands at y691–693 between the two
lines remain empty in source, baseline and candidate SVG and native captures.
Exact G/0 shapes and photographic edge modulation remain open.

## AH SVG title spacing, 2026-09-30

The remaining SVG width discrepancy is inside the title run. At 3840×2160,
card 2's source `MAGNUM 650` spans x1910–2213 (304 pixels), the AF SVG spans
x1913–2233 (321), and the AF native render spans x1913–2211 (299). Source
glyph columns show a wider `MAGNUM` with tighter gaps: the first M is 36
pixels wide versus 30 in the SVG, while the source `650` glyph widths are
21/20/22 pixels versus the SVG's 21/21/22. The old SVG puts the first digit
13 pixels late. The native title's overall width is already close, so this
correction applies only to the SVG trace and its component copy.

The title is now two text runs on the same baseline and with the same inks.
`MAGNUM` has 1.12 horizontal scale, −1 design-pixel letter spacing, and a
left-edge correction; `650` starts at local x106.5 with −0.5 design-pixel
letter spacing. The final card-2 SVG spans x1911–2213 (303 pixels). This
separate digit spacing matters: scaling the whole title made the first six
letters closer but pushed the final zero too far right.

Source, AF SVG, and the retained trial were compared in fixed first, middle,
and last title crops for the selected card and ordinary cards 2–4, including
the fourth card's clipped final segment. At three dark/bright thresholds,
all 36 fixed crops improve in mask IoU. Independently cropped source glyphs
improve in all 108 glyph/threshold checks. The y691–693 blank band stays
empty on every card, the subtitle renders pixel-identically, and the changed
SVG pixels stay within the title rows. The final trace render is
pixel-identical to the measured trial. Measurements and rendered trials live
in `/tmp/cp-eras-next/ah/entropism/` (`fixed_eval.py`, `eval_split.py`,
`split-digit--0.5.png`, `store-final.png`). Astra independently reproduces
all 36 fixed and 108 glyph improvements in `root-results.json`. Fresh
native captures at 4K and 1600×900 remain pixel-identical to the accepted
baseline; the runtime title is unchanged.

Individual inter-letter gaps retain tradeoffs hidden by the total gap
count: 73 of 96 threshold/gap comparisons improve, eight are unchanged
and 15 regress. Most losses are in the first M–A gap; at the selected
middle threshold, it gains 36 unsupported dark pixels from zero. The
corresponding ordinary-card gains are only 2–5 pixels. The source word gap
before `650` improves on every card. Exact glyph contours and these small
gap intrusions remain open despite the corrected total width and improved
glyph overlap. See `root-gaps.json` and `root-gap-summary.json`.

The SVG changes 22,026 pixels only in the title rows. The existing fourth
card clip remains: its fractional edge includes seven changed antialias
pixels in native column 3755, with no changes at or beyond column 3756.

The pre-change source and trace are frozen under `/tmp/cp-eras-next/af/baseline`.
The baseline native image is
`/tmp/cp-eras-resume-20260929/x-store-entropism-3840x2160.png`.
The fixed threshold, card/glyph segment and blank-band measurement is
`/tmp/cp-eras-next/af/entropism/measure_header.py`, with detailed results
in `header-results.json`. The fixed title and subtitle crops include the
candidate tops and the fourth card's visible last glyph segment.

The fresh native capture at
`/tmp/cp-eras-next/af/store-entropism-3840x2160.png` places title rows
at y648–684 and subtitle rows at y699–730 across all cards. At the
middle stroke threshold, source/native overlap improves in 23 of 24
fixed segments, mean IoU .174→.260. The sole regression is the selected
first `HAND GUN` segment (.091→.069); at low threshold its middle
segment also falls, and at high threshold the clipped fourth title's
first segment falls slightly. All whole-run native comparisons improve
at all three thresholds. This supports the local native baseline move
while leaving exact letter contours and SVG width open. The integrated
AF checkpoint passes both fidelity gates, seven paired fractional states,
286 Rust tests, all 22 repository checks and 27 exact visual cases. See
[round seventeen](../reference-svg-round17.md) for scope and evidence.


## AP manufacturer label audit

A new local audit identifies an actionable band-label defect. The source
`BETTERLIFE TEC` runs are wider and heavier than the SVG and native text,
while their final right edges already agree closely. The source is
`images/entropism-dashboard.png`, the store image under the documented
swapped filename, at 3840×2160. Its SHA-256 is
`4a4bd40ca4f0a070d9defe4a713518a8a4618066e43b767fff35f220e8d3da12`.

At R,G<100 in native y778..799, fixed band crops give:

| Run | Source bounds x/y | AP baseline SVG bounds x/y | Source/SVG dark pixels |
| --- | --- | --- | ---: |
| Selected card 1 | 1555..1724 / 782..795 | 1589..1725 / 780..793 | 518 / 135 |
| Ordinary card 2 | 2326..2496 / 782..795 | 2361..2498 / 780..793 | 560 / 165 |
| Ordinary card 3 | 3099..3270 / 782..795 | 3134..3271 / 780..793 | 547 / 152 |

Thresholds 90 and 130 retain the large leading-edge deficit. The first
25×14 glyph boxes contain source ink where SVG/native contain none.
The fourth run is outside the existing x3755 native cutoff, so its visible
PETROCHEM and clip are preservation controls rather than a lettering
holdout. The words are legible; the original typeface is not identified.

At the audit baseline, trace and Rust both use a 9.5px right-anchored run at local
x259/y71. A fresh AO packaged 4K capture exactly matches the historical
AH capture used in the initial study; the relevant era tables and trace
also have unchanged hashes. Thus the finding is not based on an assumed
stale native baseline. Astra independently reproduces these bounds in
`/tmp/cp-eras-next/ao/entropism-brand-audit.json`.

Fit only this run, using selected card 1 before comparing the two ordinary
copies. Preserve the right edge, band fields, adjacent brand, all artwork
and clipping. Weight/stretch and baseline need separate source controls;
mean RGB error over the yellow field cannot validate glyphs or gaps.
The first scratch study under `/tmp/cp-eras-next/ap-entropism-print/` does
not establish an accepted common correction for all three copies.


## AP selected manufacturer fit

The selected label uses thirteen individually positioned Medium glyphs,
retaining size 9.5 and fixed band ink. A single stretched run put strokes
in photographed gaps; the per-glyph fit improves SVG whole-run overlap
.035→.582 and native overlap .024→.575 at R,G<100. All fixed first, middle
and last segments improve at thresholds 90/100/120. SVG gap intrusion
falls 32→16 pixels and native intrusion 61→16, with no complete dark
stems in those gaps. Exact rounded contours, soft edges and small gap
residuals remain. The native stretched glyphs rise one physical pixel
at 4K relative to the SVG baseline; the unstretched I retains its normal
font path and an extra bottom pixel row compared with source.

Only the grown card uses this group, including its existing hover/held
variants. Fixed brand inks survive those variants. The ordinary SVG/Rust
labels remain unchanged: a frozen common-table transfer plus a second
ordinary offset fit leaves six full-height strokes and one twelve-pixel
stroke in card 3's source-empty gaps. Its stronger total overlap is not
sufficient for acceptance. The manufacturer follow-up remains open for
the ordinary copies and exact glyph/printing limits.

At 4K the selected native change affects 1,702 pixels entirely within its
label. The source SVG and component specimen stay synchronized. Paired
state and integrated verification are recorded in
[round twenty-seven](../reference-svg-round27.md); that record distinguishes
completed checks from pending work.


## AQ shared ordinary manufacturer fit

The new ordinary table uses the intersection of both photographed copies'
card-local glyph supports. This is an explicit two-card fit after the
selected-table transfer and a card-2-only table failed the third copy's
source-gap controls. It preserves AP's selected printing. Native 4K
first/middle/last comparisons improve at all three thresholds, and measured
gap intrusion falls to zero on both ordinary copies. Small endpoint and
font-edge residuals remain. State and integrated verification are recorded
in [round twenty-eight](../reference-svg-round28.md).

## CD selected M gap and stem profile

The current selected title retains AH's 36 dark pixels at x1173,
y650..685, inside the source-empty gap at thresholds 80/100. Isolated
M and MA diagnostics attribute them to M; A already has the source's
horizontal bounds. The current title rows match AH exactly, while later
manufacturer changes explain whole-image differences.

A single M-only trim fixes that 4K extent and preserves A, `650`, ordinary
cards, alpha and exterior pixels at all three sizes. It is rejected for
lost source ink in the 1600 and fractional gap/right-stem controls.
Three height bands show the source left stem moving farther left toward
its foot and graded lower-right ink spreading into smaller-size pixels.
Astra reproduces all 36 source/SVG contrast centroids; per-window
normalization supports placement evidence, not absolute ink recovery.
The next contour must retain this changing edge rather than reuse the
rectangular trim. No native or selected-title contour change is integrated.
CH separately synchronizes the component sheet's older selected title to
the accepted parent trace. At 1920×1400/960×700 this changes 1,296/410
pixels only inside the selected-title box, with no alpha changes; the full
parent title crop is now RGBA-exact after translation. The source font and
photographic ink remain unresolved. CH passes all 24 SVG structural
checks and full 22-check/27-case verification; its package is identical
to CF, with no runtime or golden changes.
See [round forty-five](../reference-svg-round45.md).


## CK–CM selected M outline

The selected first M now uses one source-calibrated path with row-varying
stems and shorter feet. The original full MAGNUM run remains clipped at
the empty M–A gap, preserving A, the final M and 650. At all three sizes,
source RGB error improves and every changed pixel stays inside M. The
component title uses identical markup and matches the parent interior at
full and half size. Weak fractional threshold losses remain explicit.

Fresh packaged-native review finds a different, unstretched title whose
first M is too far right. The SVG correction does not port automatically: a
native original-run/original-outline replay and selected-away recoloring
check are still required. See [round forty-six](../reference-svg-round46.md).
CN passes both Store gates, all 24 SVG structural checks and the complete
22-check/27-case matrix. This remained an SVG correction at CN; the native
first-M transfer was a separate task.

## CP–CQ native selected M

CP's current and split-only actual Store frames match the CN package in
full RGBA at 4K, 1600×900 and 1537×947. Replaying the original font M as
a native outline changes 303/113/114 M-only pixels but worsens source RGB,
so it is a transfer control, not the selected correction. CQ instead
transcribes the accepted SVG M contour into a semantic-ink native path
inside the exact x29 title split. Whole-M source RGB L1 falls
199136→112019 / 32808→17955 / 28052→17333 at the three sizes; IoU
improves at thresholds 80/100/120 in every whole-M window. At 4K the
strict right-stem and M/A-gap IoU decline, and at 1600 the right-foot
RGB error rises 614→644. Photographic edges, A, the final M and 650
retain their earlier limits.

Seven selected-title state pairs keep all changed pixels inside M, with
full suffix including the first-A seam pixel, exterior and alpha exact.
Pressed/away use `Select`; rest/selected/custom use `OnSelect`; a separate
custom-purple selected-away pair proves the path follows the changed
`Select` role. Hover equals rest, while the early opening clips M and
retains visible motion; the 500 ms fade also passes. CQ source is staged,
with CQ production/package and full checks passing. Broad E4 remains open. See
[round forty-seven](../reference-svg-round47.md) and
`/tmp/cp-eras-next/cp-root-review/entropism-cq-review.json`.
