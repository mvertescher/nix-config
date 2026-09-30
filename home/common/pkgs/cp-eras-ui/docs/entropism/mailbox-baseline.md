# Mailbox SVG-to-Iced baseline calibration

The mailbox's source-fitted `Run` coordinates use SVG text baselines. The
native Iced canvas placed their visible Rajdhani ink above the SVG ink under
the previous `text_baseline: 0.95`. At 3840×2160, the 26 fitted E2 runs were
0.83–1.67 design pixels high at their top or bottom edge. This pass changes
only Entropism's shared mailbox baseline factor to `0.89`; the SVG and the
individual `Run` coordinates stay fixed.

`screens/mail.rs` positions a run at `Run.y * sy - size * text_baseline`, with
font size scaled by the smaller screen axis. Reducing the factor moves the
runtime ink down by an amount proportional to font size. The measured
size-weighted least-squares factor for the 26 runs was `0.88984`; `0.89`
gives the same two-decimal calibration and keeps one factor for all mailbox
text.

Measurements use the original 3840×2160 source, the unchanged SVG rendered
at that resolution with the repository font configuration, and Iced rest
captures before and after the change. Coordinates below are divided by 2.4
to the 1600×900 design frame. Each run has an isolated crop; bright primary
ink uses green >130, dark reverse-video ink uses green <140. Vertical edge
error averages the absolute top and bottom differences for each run.

| SVG-to-Iced, 26 fitted runs | Before | After |
| --- | ---: | ---: |
| Mean absolute vertical edge error | 1.210px | 0.056px |
| Maximum absolute vertical edge error | 1.667px | 0.417px |
| Source-to-Iced mean four-edge error | 0.653px | 0.076px |

All 26 measured widths are unchanged. Of the 52 top and bottom edges, 45
match the SVG to the native raster pixel and seven differ by one native pixel
(0.417 design pixels). The original source and SVG remain unchanged.

The shared factor also moves text outside the fitted set. In separate native
crops, boxed A/B/C vertical error falls 1.250→0.000px; six header/footer
strings 1.250→0.417px; ten body lines 1.229→0.500px; and four badge labels
1.823→0.469px. These figures are mean absolute top/bottom errors against the
SVG. The first eight body lines are within one native pixel at most edges.
The last two remain high because the renderer's common `21.7` line pitch and
`39` paragraph gap start the third paragraph at y533.2, while the SVG starts
it at y535. That paragraph spacing issue is independent of the baseline
factor. Badge label width and centering differences also remain.

At the fractional 1537×947 capture, 24 fitted runs with reliably isolated
primary ink move downward by one or two screen pixels; their horizontal
bounds remain stable to raster rounding. The two faint micro-print runs are
below a reliable threshold at that scale, so they are excluded from that
fractional bound count. The factor predicts a roughly half-pixel shift for
their 9px font at this size. The native measurement above includes them.

This calibration does not identify the source font or resolve Rajdhani glyph
forms, internal advances, action-label width residuals, third-paragraph
spacing, or badge centering. E2 remains open for those measured differences.

Measurement scripts and JSON results are in
`/tmp/cp-eras-round4/entropism-baseline/`; native before/after Iced captures
are `/tmp/cp-eras-round4/{before,after}-entropism-mailbox-3840x2160.png`,
with matching `1537x947` fractional captures.

## Per-action width calibration, 2026-09-29

A single 1.03 horizontal scale left independent action-width errors: REPLY
was too wide, FORWARD and DELETE too narrow, and REPORT SPAM too wide.
Each run now has its own horizontal scale and anchor correction, retaining
23.5px Medium, the accepted baseline, label contents, role colors and all
button geometry. `MailButtons::label_runs` follows the existing badge
convention: absent entries use the common label. The other eras provide an
empty list and keep their existing drawing.

| Run | SVG scale | Iced scale | Source width, native pixels | Corrected SVG width |
| --- | ---: | ---: | ---: | ---: |
| REPLY | .992681 | .978500 | 133 | 133 |
| FORWARD | 1.043857 | 1.043857 | 226 | 225 |
| DELETE | 1.075346 | 1.075346 | 166 | 166 |
| REPORT SPAM | 1.013856 | 1.013856 | 314 | 314 |

The native source and current SVG use the same clean-ink thresholds as the
prior calibration (green >130 for bright text, <140 for reverse printing).
All four left edges now match source pixels; right-edge residual is at
most one native pixel. REPLY's separate Iced scale compensates its measured
58.33 versus SVG57.50 design-pixel width before correction. The component
action strip and three REPORT SPAM state examples follow the new SVG fit.
New native/state review is still required before accepting the runtime
change. Exact glyph outlines and photographic softness remain distinct
from these measured width corrections.
## Remaining glyph review — 2026-09-29

A fresh source/SVG/native comparison does not support another global
width or baseline adjustment. In the selected first-row title, the dark
source run spans native x332–747, versus x332–750 in SVG and x332–749 in
Iced; the sender spans x332–533 in all three. At an R/G threshold of 125,
the source title contains 1,126 dark pixels, versus 1,377 in SVG and 1,234
in Iced. Lowering the threshold to 85 leaves only 231 source pixels but
1,077 SVG pixels. This indicates a substantial printing/edge difference
that a small origin change would not resolve.

Body words, line breaks and fitted action endpoints remain intact. The
source has visible fine repeated edges and material modulation; their
exact font/opacity construction is unresolved. Further work needs an
isolated, repeatable printing feature with independent holdouts, or the
original font/material layers. This audit leaves E2 open and makes no
drawing changes.
