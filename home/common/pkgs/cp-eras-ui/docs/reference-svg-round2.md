# Reference corrections, second batch — 2026-09-21

This continues the [first batch](reference-svg-fixes.md) and the
[all-reference audit](reference-svg-audit.md). Coordinates and errors use
the 1600×900 design frame unless stated otherwise. Original sources were
reviewed at native 3840×2160 resolution with repository fonts.

## Entropism mailbox — E2

Fitted 26 primary text runs: section labels, two micro-print lines, seven
subjects/senders, the reader heading/sender and four action labels. The
source's words, line breaks, `ENCRIPTION` spelling and `from: Mom` casing
are preserved. The previous body fit and boxed A/B/C remain byte-identical
in the native SVG render.

Rajdhani now uses measured size, weight, baseline and horizontal scale.
Subject/sender templates carry per-row typography while the renderer keeps
ownership of content and selected/feedback ink. There is no duplicate
fixed text over the interactive rows.

| Primary-ink bounds | Mean edge error before → after | Mean width error before → after |
|---|---|---|
| Section labels | 1.953 → 0.000px | 3.229 → 0.000px |
| Micro-print | 1.094 → 0.156px | 1.458 → 0.208px |
| Subjects | 2.113 → 0.134px | 3.690 → 0.119px |
| Senders | 4.985 → 0.060px | 17.976 → 0.060px |
| Reader heading/sender | 3.021 → 0.365px | 9.583 → 0.208px |
| Action labels | 1.510 → 0.677px | 2.708 → 2.083px |

Bounds use native green >130 for bright printing and <140 for dark
reverse-video text within isolated crops. Edge error averages the four
box edges. Across all 26 runs, mean edge error falls 2.760→0.196px and
width error 7.596→0.401px. No pixels change outside the targeted text boxes.

Native Iced preserves the fitted widths: mean SVG-to-Iced width error is
0.192px, maximum 0.833px. Its text baselines remain roughly 0.83–1.61px high
under the existing mailbox line-height/baseline model. Source-to-Iced mean
edge error is 0.713px. A shared baseline calibration remains follow-up work;
source-fitted Run coordinates were not shifted to hide it.

E2 stays open for font-form and action-label residuals: the source has
rounder bowls/parentheses and different internal advances. The shared
button run leaves widths +2.08/−1.25/−2.92/+2.08px relative to source;
REPORT SPAM's individual edge error changes 0.83→0.94px despite the mean
improvement. These are disclosed fitting limits, not recovered source fonts.

## Kitsch login — K4 and the login portion of K5

The field now steps upward along its lower edge, y463.7→455.7, with
rounded shoulder joins and about 2.3px outer corners. ENTER has matching
top runs y470.4/462.4, leaving 6.7px gaps, plus rounded 2.2px corners.
Both PROTECTED controls use the same source-supported contour language.
The full-height bracket and three-card arrangement are preserved.

Native median-column profiles agree at nine of eleven sampled x positions;
the other two differ by one native pixel (0.417 design pixels). The source
mask selects the luminous face, G>175/B>120, separately from surrounding glow.

ENTER, PROTECTED and clock sizes/weights/glyph widths now fit the source
within about one design pixel. ENTER's width changes 43.75→54.17px against
53.75px in source; PROTECTED changes 77.08→100.83px against 100.42px.
The clock's primary-ink bounds match the native source at the stated mask.
These are local font metrics, not exact photographed glyph shapes.

`Plate` supports an optional closed curve contour. Drawing and pointer
hits use the same path; hit testing flattens it at 0.025 design-pixel
tolerance. The layout rectangle remains available for input fitting.
Regression coverage checks the gap, rounded corner, successful click and
drag-out cancellation at native, design and fractional sizes. Existing
long Unicode input and secret/submission tests still cover all four eras.
K5's store/mail typography and finer source-font matching remain open.

## Neo-kitsch T2 badge and closed envelopes — NK-03 / NK-10

Independent dashboard/mailbox measurements replace the overly tall
18px badge shoulder with an approximately 8.4px rise. Echo contours advance
down/right about 1.23px and converge into fixed lower corners, replacing
the outward straight rails. Twelve hub and fourteen mailbox echoes fit
the resolved source extents; the lower tab has rounded joins and a wider top.

| Badge front landmarks | Before RMSE | After RMSE |
|---|---|---|
| Dashboard | 6.656px | 0.151px |
| Mailbox | 6.305px | 0.143px |

The five landmarks are left edge, shoulder y, top y, bottom y and right
edge, measured from native red-minus-blue profiles. Source pixel spacing
is 0.417 design pixels; extra decimals do not imply finer source certainty.
Badge paths share the existing header software composite for alpha parity.
Badge typography, photographic strand brightness and interference remain
under the existing typography/material tasks.

Closed envelopes gain both lower folds and a deeper 7.2px top V in the
measured 16×10.5px box. Open rows1/3/7 and closed row2/rows4/5/6 retain
their states and feedback inks. Row4 shape-mask IoU improves 0.385→0.566
after translation alignment for this diagnostic; that alignment is not
applied to the screen and does not conceal actual placement drift.

The existing 60.2px mailbox pitch is shorter than the approximately
60.8px source pitch. Displacement grows from about 0.4px at row2 to 4.4px
at row7. **NK-13 records this new placement follow-up**; this batch leaves
whole-row layout intact while correcting envelope topology.

## Neomil socket spacing and current documentation

The 25-cell scatter now uses a measured 3.6667px pitch and per-card origins
in SVG, component excerpts and Iced. Center RMS across the four symbols
falls from 0.385/0.495/1.917/2.569px to approximately
0.098/0.126/0.098/0.102px. See [method and limits](neomil/socket-spacing.md).
Directional printing echoes remain separate.

NM4 refreshes README, source catalog and component descriptions: all four
screens share the measured ground; the old chart layout was removed; the
reader clip, selected-card bottom, hollow Login slot and socket geometry
are described consistently. Historical gate counts and bar migration notes
are explicitly dated. The bar remains an original composition, and the
component sheet's legacy backdrop is identified as display context.

## Validation

All 252 Rust tests pass, including the new rounded-click/drag-cancellation
and template-sender contrast regressions. All 24 SVGs parse with unique IDs
and resolved local references. The final eight G1i and five G2i gates pass:

| Changed screen | Source → SVG | SVG → Iced |
|---|---|---|
| Entropism mailbox | shapes 88% | shapes 100% |
| Kitsch login | inks 0.65 | shapes 72% |
| Neo-kitsch dashboard | inks 0.66 | shapes 96% |
| Neo-kitsch mailbox | inks 0.75 | shapes 91% |
| Neomil store | shapes 61% | shapes 93% |

The unchanged Neomil login/dashboard/mailbox source gates also pass at
78%/83%/74% shape area, refreshing the documentary gate claims. These broad
inventory percentages are not detail-completeness scores.

Native SVG/Iced crops, fractional rendering and opening/feedback states
were reviewed. Long Kitsch Unicode input changes zero pixels outside the
field at 1537×947. Fourth-card Neomil feedback changes zero pixels beyond
its persistent viewport. Entropism text/ink stays contained through row
feedback without switching the reader; Neo-kitsch envelopes retain
open/closed state and polarity through feedback and their opening fade.
Its panel grain and six-plus-two source copy remain visible and clipped.

Five reviewed 1600×900 goldens are refreshed. Full `./check` passes all
19 checks, including all 27 packaged visual cases at 100.000% similarity,
with no retries. No thresholds or photo classifications were changed. Earlier
staged work is preserved; no commit, push or activation has been performed.
Live desktop checks remain separate from headless verification.
