# Reference correction batch, 2026-09-21

Subsequent changes and current validation: [second correction batch](reference-svg-round2.md).

Follow-up to the [24-reference audit](reference-svg-audit.md).
The first batch corrects primary content and geometry on ten screens.
Original source images remain local; the committed artwork is SVG/vector
geometry. Repository fonts are used for every comparison. Measurements
below are in 1600×900 design coordinates, taken from native 3840×2160 crops.
Fine material, unknown typefaces and photographic echoes remain separate.

## Entropism

**E1:** Ten login masks replace eleven. Regular 25, horizontal scale 0.94045
fits the available font to the source's thinner stars. Source and SVG core
bounds both span x 577.92..670.42; 663.75 in the audit was the final star's
start. The 19×0.5 underline now matches source core bounds at
x 576.67..595.42/y 439.17..439.58. These are display fixtures; actual secret
length, editing and submission semantics remain unchanged.

**E2, partial:** Supported horizontal scaling 1.14 replaces ineffective SVG
textLength/lengthAdjust on the ten body lines. Actual words and 4+4+2 breaks
stay intact. Mean line-end error drops 77.04→2.46 px; remaining errors reflect
available-font differences. Boxed A/B/C use Medium 22 with local scales
1.65/1.65/1.60, fitting primary bounds within one native pixel. Section
heading, row/from/button and secondary typography remain open.

**E3:** Fourth-card content now ends at x 1564.6; source bright primary
content ends at 3755/2.4=1564.583. A permanent viewport wraps the entire
card, including selected/hover/held states. It constrains pointer hits and
keyboard centers without closing the open edge or adding residue. A
regression checks visible/hidden hits and centers at four scales.

## Kitsch

**K1:** Eight actual BRAINDANCE lines replace nine placeholder bars.
Local line fitting brings primary glyph bounds within 0.42 px and every
right edge within one native pixel. The connected warning ribbon spans
x 1172.5..1432/y 283..331.5. The body ends at y 667.5 with 10.5 px rounded
bottom corners; the tab's rounded transitions replace its invented tip.
Available-font contours still differ from the photographed text.

**K2, partial:** Five idle fan labels use measured mint #7cffe5; EVENTS
retains dark ink on yellow. Fan geometry, trails and interaction materials
remain intact. Face fields and stroke/material fitting remain open.

**K6:** First-row open envelope is a distinct measured vector at
x 166.95..183/y 322.5..337, including the folded V and no horizontal crossbar.
Cross-review also restores both lower folds on all four closed glyphs,
which the source shows but the old SVG omitted. Their measured 16.05×9.95
outline replaces the over-wide 20×13 approximation; row origins differ from
source by at most 0.2 px without moving message text. Optional per-era vectors
inherit resolved selected/hover/held ink; other eras retain existing glyphs.

## Neo-kitsch

**NK-01:** All 22 login strands now end in downward rounded feet, preserving
the center plateau and pitch. Source lower-left samples support x 34 and
radius 17; local centerline RMS drops 7.489→0.541 px. Existing fine luminance,
halo and ground differences remain material work.

**NK-02:** Independent native profiles resolve nine dashboard strands and
11 mailbox strands, with about 3.18 px pitch on both sides. The rise now
starts around x160 and reaches 226 instead of crossing T1 at 52..130.
Top-strand sample RMS drops 25.058→0.238 px on dashboard and 24.799→0.130 px
on mailbox. Both ends have independent rounded feet; mailbox retains its
long lowest-left terminal. Header paths share the ground’s software
composite so faded strokes blend in sRGB as the reference does. Local
geometric agreement does not establish exact strand material or shared ground fidelity.

**NK-06:** Dashboard copy now follows the source's six plus two lines,
including `adipiscing` and the visible `maece-` break. It no longer borrows
mailbox wording. The existing 85 bending SVG grain paths are also carried
into Iced, replacing its straight strips while preserving panel fade and
clipping. This is implementation parity; source typography/veneer fitting
remain separate tasks.

## Neomil

**NM2:** Reader contour now steps inward below the side bar from x 1450
at y 544.5833 to x 1440.5548 at 554.0285, then ends square at y 699.3177.
The old outline stayed at 1450 and invented an 8 px bottom chamfer. Native
profiles separate the thin primary from a neighboring echo: fitted
#fb3535 stroke width is 0.748 px at the lower right and 0.775 px at the bottom.
Independent holdout RGB RMS is 0.355 and 0.637 respectively. Both use 0.76 px;
the visibly thicker upper-right stem uses 1.8 px. Subpixel centers come from
pixel coverage, not simply the brightest pixel's index.

The existing upper-right chamfer remains; side bar, material mask and
outline agree and follow the message-opening clip. Exact upper-chamfer
fit, top-edge warping and fine echoes remain outside this correction.
An initial contour-only pass triggered a cartridge classification failure
in the global shape extractor, although no cartridge changed. The final
source-measured ink/stroke correction passes without gate modifications.

**NM3:** Both source socket patterns have 25 cells. Added missing lattice
column 2,row 6; corrected selected first-cell center from 786.5,714.5 to
782.7083,712.7083. Repeated normal symbols and component excerpts share
the correction. Source primary orientation is unchanged. Uniform 3.75 px
pitch is still approximate: selected bottom-row first center is about
0.83 px too low. Fine pitch and directional echo fitting remain open.

## Verification

All 250 Rust tests pass, including the new four-scale cropped-target
regression. All 24 SVGs parse with unique IDs and resolved local references.
The ten changed traces pass both gates below. Shape percentages and ink
IoU describe broad inventory/placement, not visual correctness; native
source crops and state review establish the scoped corrections above.

| Screen | Source → SVG | SVG → Iced |
|---|---|---|
| Entropism/login | shapes 83% | shapes 100% |
| Entropism/mailbox | shapes 88% | shapes 100% |
| Entropism/store | shapes 79% | shapes 100% |
| Kitsch/dashboard | inks 0.72 | shapes 64% |
| Kitsch/mailbox | inks 0.67 | shapes 61% |
| Neo-kitsch/login | inks 0.77 | shapes 86% |
| Neo-kitsch/dashboard | inks 0.65 | shapes 94% |
| Neo-kitsch/mailbox | inks 0.75 | shapes 92% |
| Neomil/mailbox | shapes 74% | shapes 81% |
| Neomil/store | shapes 61% | shapes 93% |

Fresh rest/opening and feedback captures were reviewed. Empty-to-long
Entropism input changes zero pixels outside the field at 1600×900 and
1537×947, including Unicode input. Its fourth-card viewport preserves the
cut in normal/selected/hover/held/opening states and at fractional scale.
Kitsch's custom envelope ink follows both selected and normal feedback.
Neomil mailbox/native-store captures cover the square corner and corrected
socket, while zero/mid-opening probes show no panel art outside the reveal.
Neo-kitsch's final grain/body share one viewport so the foreground fill
cannot erase the clipped grain; its opening fade remains intact.

Ten reviewed goldens are refreshed. No source/golden thresholds or photo
classifications changed. Full `./check` passes all 19 checks, including
all 27 visual cases at 100.000% similarity, with no retries. The packaged
matrix includes unfrozen Neomil mailbox/store startup. These checks cover
the final code; only documentation was updated afterward.
Earlier staged Neomil/performance work is preserved and all changes remain
staged; no commit or activation
has been performed. Live desktop verification remains separate.
