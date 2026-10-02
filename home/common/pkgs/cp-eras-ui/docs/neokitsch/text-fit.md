# Neo-kitsch paragraph and footer alignment — AX

AX compares `images/neokitsch-mail.png` and
`images/neokitsch-dashboard.png` with the current traces and actual Iced
widgets at 3840×2160, 1600×900 and 1537×947. The preview's six baseline
frames exactly match the previously verified implementation. This review
accepts bounded geometry corrections; NK-05 still owns exact printing.

## Mailbox body

The body starts 0.8 design pixels farther left, at x=735.2, and its
horizontal stretch changes from 1.06 to 1.054. Words, line breaks, font,
vertical metrics and semantic ink remain unchanged in the SVG and Rust.
At 4K, mean absolute line-end error falls from 3.292 to 0.250 design
pixels at the warm R≥175 core; left-edge error falls 0.750→0.083.

All 150 whole-line overlap controls improve: ten lines at five thresholds
(R≥130/150/175/190/210), at three sizes. Native changes stay inside the
three paragraphs; title, sender, badges, list rows and buttons are exact.
Fractional source comparisons place each line with separate x/y anchors
and scale its glyph crop uniformly. They are diagnostic projections,
not photographs of the resized application.

There are local losses. The short `borum.` line narrows by 0.42 design
pixel at 4K and one or two physical pixels at smaller sizes. Endpoint
controls record 4/7/16 losses at 4K/1600/fractional size, mostly in width
and strict bright-core coverage. Word-gap controls record 0/10/4 losses;
the 1600 strict-threshold line-3 gap exceeds the width limit used for
matching word gaps and becomes unmatched. Two fractional gap comparisons
lack a baseline match. These limits remain in NK-05.

Gap measurements exclude inter-letter holes: the original word gaps are
5.83–7.50 design pixels wide, while nearby letter gaps are about 1–3.
The initial nearest-gap matcher selected one of those holes and was
corrected before acceptance. This is a geometry fit, without a new
weight, ink or glow approximation.

## Dashboard C/D annotations

The two footer blocks use size 7.5 and baselines 827.6/834 instead of size
8 and baselines 826/837. C moves to x=618; D uses x=1205.5 in Iced and
1205.8 in SVG, which both rasterize the first bright pixel at source
x=2895 at 4K. SVG tracking becomes zero. All four sentences, their line
breaks, fixed MICRO ink and the recently corrected badges stay intact.

Native 4K right-tail excess shrinks from 31/25/33/11 pixels to 1/4/1/0
for C1/C2/D1/D2. Every whole-line R>100 overlap improves at all three
sizes; all twelve fixed first/middle/last spans improve at 4K and 1600.
The fractional D1 first span worsens .236→.159. First lines sit one pixel
below the photograph at 4K, align at 1600, and remain two pixels above
the projected source at 1537. All changed pixels stay in the annotation
windows, with identical badges.

Brightness and photographic softness remain unresolved. Fractional R>130
overlap worsens on all four native lines; 1600 R>160 has no native core
pixels before or after. The SVG's 1600 first-line R>100 overlap also
worsens (C .038→.028, D .023→.012), although native first-line overlap
improves strongly. The geometry correction does not close these small
printing differences or claim exact fidelity. Whole-line measurement
windows include the old right tails; fractional second-line windows
exclude the first line's final pixel row.

State and integrated validation are recorded in
[round thirty-five](../reference-svg-round35.md).

## Repeated store metadata: unresolved font and spacing fit

The six `SPARE TIME MANAGER…` / `SERVING CUSTOMERS…` lines beside the upper A/C and lower B badges remain open under NK-05. The photographed M and O caps occupy 11 native rows. An earlier installed-font comparison held all faces at the same em size; it cannot identify the source face. At matched cap height, FreeSans Bold's isolated terminal O is closer to the photographed contour than Rajdhani Bold (F1 `.852/.824/.721` at R>130/160/190 after independent glyph registration). The exact source font and photographic edge treatment remain unknown.

A scratch FreeSans Bold fit kept that cap height and used only upper A line-one word starts to solve one origin and one tracking value (`−0.1336` design pixels). Actual librsvg prefix renders place the first-line word starts within two native pixels on the repeated copies, but second-line internal starts drift four to five pixels. A single full-color SVG trial preserved the original words, fill, halo filter and surrounding artwork. All six line-mask overlaps improved at 4K, 1600 and 1537, yet actual R>190 extra core in the source's fixed inter-word gaps rose from `10→361`, `0→43` and `0→29` pixels. The upper C pair's RGB error worsened at 4K and 1537, and the upper first-line core rose one row above the photo. The trial is rejected; no trace or runtime typography changed. Further work must retain the source's word gaps and cap rows across all six runs, not infer acceptance from matched endpoints or whole-line F1 alone.


The 4K full-color candidate also worsens upper A's second-line RGB fit.
Astra independently reproduces the three-size pair/line RGB, threshold,
gap and full-RGBA comparisons, and confirms exactly six text nodes change.
The earlier 1600 Bicubic diagnostic is archived; the reported 1600 values
use full-photo Lanczos. Evidence is in `/tmp/cp-eras-next/bf-neokitsch-full/`.
No native trial is accepted from this font study.

BG separates letter tracking from an equal additional word-space advance,
using A1-only starts and ends. Its analytic fit gives tracking −.258542 and
extra word advance +.760234 design pixels. Held-out second-line start drift
falls to 2–3 native pixels, but one B2 word endpoint still misses by five.
The shifted-prefix masks retain O-contour, cap-row and gap discrepancies;
they are geometry diagnostics, not actual-color false-core measurements.
This does not support a full-scene/native candidate yet.

Minimal fixtures under the pinned librsvg show no pixel change for the
tested word-spacing attribute, CSS property or per-character dx list.
Splitting into tspans supports explicit word movement, but even zero dx
changes 34/39 pixels at integer/fractional size. An actual calibrated render
is required before treating the analytic spacing model as drawing evidence.
Astra reproduces the fit and independently compares every fixture's full
RGBA output. Evidence is in `/tmp/cp-eras-next/bg-neokitsch-spacing/`.

## BH–BJ actual spacing and native transfer

BI isolates a renderer-specific boundary advance in BH's negative-tracking
tspans. Compensating that advance improves every whole-line RGB/F1 and every
word F1 against the original SVG at all three sizes. All source word-gap
apertures retain a clear column, despite 127/5/5 extra R>190 pixels versus
the original 10/0/0. This is a bounded font/spacing approximation; cap phase,
O-counter ink and second-line endpoints remain imperfect.

BJ's actual Store baseline reproduces production at all three sizes. Its
native candidate uses the intended +.760234 word advance, not the SVG's
renderer-specific +.311511 dx. Every pair/line/word F1 improves or ties, but
upper C RGB and small-size source-gap apertures retain losses. The next
single native calibration moves both upper blocks left one native 4K pixel
and their second lines up one, keeping lower B and all font/spacing values.
Neither native trial nor source-font identity is accepted yet. See
[round forty-two](../reference-svg-round42.md) for measurements and evidence.

BK/BL calibrate only those upper positions. BL keeps a common −1/2.4
design-pixel x adjustment and uses −.25 for upper second-line y, avoiding
a fractional rounding jump while preserving the 4K correction. Every native
pair/line RGB and word-F1 comparison improves or ties production at three
sizes. Individual-word RGB, fixed-counter ink and cap-height losses remain;
the fractional span is 145–148 versus source 144–148. All 27 fixed source
word gaps retain a clear column. Seven actual Store state pairs then pass:
rest, first/last selections, two synthetic held materials, custom palette
and partial opening. All change the same 3524 metadata pixels at 1537×947,
with no alpha/exterior changes. Selected-held equals rest because the
intended material is the same; this is not pointer replay.

BM integrates the six metadata runs through `TrackedWords`, retaining the
complete literal strings and the same common letter/word spacing. The SVG
keeps its separately calibrated boundary dx. This is a bounded correction;
the unidentified source font, 2/2/5 individual-word RGB losses, excess ink
in fixed source gaps/counter and the 1600/fractional cap residuals remain
in NK-05. See [round forty-two](../reference-svg-round42.md) for integrated
verification; a better whole-line score does not close those residuals.

BM verification passes all 294 local/Nix Rust tests, both gates, the 22
repository checks and all 27 visual cases on their first attempt. Three
production sizes and a fresh packaged 4K frame match the reviewed candidate
exactly. Only the Store golden changes, by 3787 metadata pixels. The 26
exact matrix images and unchanged one-level bar pixel are checked directly.

## DP/DR module-label font evidence

The document-level font list supplies Exo2-Light as a candidate, without
assigning it to a visible label. [Round fifty-three](../reference-svg-round53.md)
pins the official 1.001 font and license and reviews one fixed SVG
substitution across seven Dashboard module labels. Baseline Fontconfig
no-ops are exact at 4K/1600, and changes stay in the label neighborhoods.

The actual size is 15.8405797101449275. The method's stated `17*643/690`
would instead be 15.842028985507246; do not call the frozen render exact
cap matching. Its lower RGB L1 accompanies lost bright strokes: 1600
R160 gold area falls 847→31 against source 1,912, and F1 falls .2254→.0257.
All four threshold aggregates worsen in both fixed and expanded regions.
Reject this candidate without ruling out a separately calibrated family.

Before another full-scene substitution, compare distinctive source glyph
silhouettes where they can be separated from background/edge treatment.
Predeclare windows and registration; preserve other labels, counters,
gaps and the 1600 projection as controls. Matching a left endpoint or
removing false gap ink while losing strokes is insufficient evidence.

## DS distinctive-letter diagnostic

DS completes that bounded comparison with fourteen frozen windows from
seven module labels. EMAIL E alone supplies shared horizontal registration
and cap scaling, applied around each label's own baseline. Transparent
templates remove the scene's glow/background from the model side. Rajdhani
uses dx +4 and cap scale 29/26; Exo uses −3 and 29/24. The Exo size remains
the actual DP value. Whole-photo Lanczos and directly rendered template
masks supply the 1600 control.

Registered R210 fixed-window F1 is .391/.266 at 4K and .270/.113 at 1600
for Rajdhani/Exo. A supplemental complete-template-glyph union has 4K
R210 F1 .352/.254; source pixels remain confined to the original windows.
The union addresses template truncation, not source segmentation. Training
E is not independent evidence, and weight, glow, advances and per-word
position still confound family/shape conclusions.

Astra independently reproduces all 448 fixed-window TP/FP/FN/F1 groups,
reviews the worker's 112 union groups and views the bordered source/template
montage. All 30 manifest hashes match. No scene or font change is accepted.
The diagnostic is complete; a next correction needs a specific source-backed
contour or registration hypothesis with complete scene controls. Details
and reproducible evidence are in [round fifty-four](../reference-svg-round54.md).
