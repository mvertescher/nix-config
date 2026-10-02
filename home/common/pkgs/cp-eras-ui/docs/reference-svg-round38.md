# Reference SVG correction — round thirty-eight

BA tests a single design-space baseline correction for the ten Neomil mailbox
body lines: move their existing group up `1/2.4` design pixel. Glyph strings,
sizes, x positions, paragraph spacing, panel art and ink remain fixed. The SVG
and component excerpt carry the same transform. This is a primary registration
correction; the source does not establish a separate body-text echo.

At 4K, the guarded actual Mailbox preview changes 67,488 RGBA pixels within
the body and no alpha or outside pixels. Full-body RGB MAE against the source
falls 8.742→7.014. The fixed full-body red-core F1 rises .587→.720 at
threshold 100 and .513→.637 at 150; 49,880 full-body pixels gain RGB L1
accuracy and 17,506 lose. Every one of four fixed phrases and ten whole-line
regions improves red MAE and F1 at thresholds 100/150/200/230. The frozen
counter control has **six red-MAE losses**: “ut labore et” and lines 2, 3, 6,
7 and 10. Those holes were defined by the old native glyph mask. A separate
source-defined counter diagnostic, constructed after capture, improves all 28
region/threshold comparisons (14 regions at thresholds 150 and 200), while
retaining localized losing pixels. It explains the old-mask conflict but is
**not** a frozen acceptance holdout or proof of exact contours. [Mailbox
measurements](neomil/mailbox-fidelity.md#ba-body-baseline-source-and-native-review)
preserve both results.

At 1600×900, native baseline and candidate are byte-identical: the 0.417
design-pixel shift has no raster effect at that size. At 1537×947, responsive
Mailbox placement uses separate `sx` and `sy`; the full photo cannot be
uniformly resized for a valid source-fit comparison. Rest, first/last
selection, held-first/held-last and custom-palette pairs each change 9,194
body pixels, with no alpha or outside-body changes. Opening at 0.12s and zero
reveal show no body change because the text is still clipped. A supplemental
0.20s opening capture shows 4,018 changed pixels inside the partially revealed
body, no alpha or outside-clip changes. These are headless drawing-state
observations, not live pointer or desktop verification.

The SVG evidence has local limits: at 1600 the training phrase's union-mask
blue MAE rises 1.320→1.536, “Nemo enim” F1@200 falls .34646→.33333, and the
five changed line-gap pixels slightly increase blue MAE .723→.728 despite
red/green gains. The component-sheet copy alters only the body-text group in
its two visible specimens. Source and native checks support a bounded
registration change, not completed faint printing or material recovery.

The Neomil dashboard footer's joined-top candidate is **rejected** after
native review. Its SVG improves the three-size whole-footer averages, and the
extension to the inner rim edges improves all four/two/two local endpoint
pixels, but SVG did not predict the 1600 native bright-core threshold. The
guarded actual Dashboard baseline matches AZ production exactly at all three
sizes. Candidate changes 341/142/137 RGBA pixels at 4K/1600/1537, all within
the top edge; source RGB L1 gains are 341/142/136 with one fractional loss.
Yet at 1600, source top-adjacent row 865 is R≈154 and native moves from R≈59
to R≈180, creating a false R>170 core. Each fixed straight-edge train/holdout
F1 falls 1.000→.667; top-primary F1 falls .9958→.6742, with ten frozen
regional/join F1 losses in total. The whole-footer RGB MAE improvement
(10.774→9.982, 9.653→8.968, 11.484→11.355) does not override those losses. No
dashboard top, component-caption, golden or state/fallback update is accepted
from this trial. [Footer
measurements](neomil/dashboard-footer.md#ba-top-edge-and-side-audits) retain
both SVG and native evidence.

BB's separate read-only audit finds a left-specific vertical phase mismatch:
source 4K left core x2902–2904 versus accepted native x2901–2903, while the
right core aligns. It proposes no coordinate; widening both sides is
unsupported. The AZ bottom stroke's extra row 2133, strict upper-bottom-band
excess and faint secondary printing remain. The dashboard exact-fidelity item
stays open.

The subsequent one-trial left-edge SVG translation is rejected: it repairs the
inner column at the expense of the existing core, worsening every left
straight RGB control and both joins. At 1600 the middle/lower F1@170 falls
from 1 to 0. Astra reproduces the losses; no native or production change
follows. See the footer record's BB trial.

All three production Mailbox sizes match the reviewed candidate exactly, as
does a fresh packaged 4K capture. The 1600 Mailbox and every other golden
remain unchanged. Both Mailbox fidelity gates, 292 local and Nix Rust tests,
all 24 SVG ID/reference checks and all 22 repository checks pass. All 27
visual cases pass on their first attempt: 26 are pixel-identical and the
Neo-kitsch bar retains exactly its prior one-pixel, one-channel-level
difference. All 312 frozen tracked files match the tested Nix source; nine
local original images remain unchanged. Final verification prose preserves
that tested artwork. Work is staged and uncommitted; no push or switch. The dashboard remains at the accepted AZ geometry.
Broader fidelity and live-desktop tasks remain open. Scratch evidence:
`/tmp/cp-eras-next/ba-mailbox-baseline/`, `ba-mailbox-review/`,
`ba-mailbox-components/`, `ba-footer-top/`, `ba-footer-joins/`,
`bb-footer-sides/`, `ba-dashboard-joined-native/`, and `ba-dashboard-review/`.

The original scratch locality helper expected a nonzero change in every
nonzero-time pair. Its 1600 failure was an incorrect harness assumption:
strict full-RGBA comparison establishes equality, while the guarded source
substitution and 4K result establish that the candidate was used. Independent
provenance, state and locality checks retain that equality without weakening a
fidelity threshold.
