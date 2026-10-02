# Reference SVG correction — round thirty-seven

AZ adds missing bright coverage to the Neomil dashboard footer's bottom
edge. The new foreground band follows the source's 4K rows 2129–2132,
between the existing sides. Primary lettering, divider, top, sides and
faint copies retain their prior geometry. The component excerpt and its
explanatory caption are synchronized. See the
[footer measurements](neomil/dashboard-footer.md#az-bottom-edge-coverage-2026-09-30).

The unchanged scratch Dashboard matches the verified 4K package, current
1600px golden and prior fractional frame exactly. The candidate uses the
actual Dashboard and changes one primitive. Three-size source review finds
1,014/142/137 changed native pixels, all improving RGB error. Whole-footer
MAE falls 12.266→10.774 at 4K, 11.306→9.653 at 1600px and 13.508→11.484
at 1537×947. Fixed corner, divider, text and blank controls pass.

All nine paired native cases pass full RGBA locality checks: three rest
sizes, first/last selection, synthetic held feedback, custom palette,
opening and fallback. Every change stays at the bottom edge and no alpha
changes. Reference and no-config fallback agree exactly. Full/half-size
component checks isolate 142/144 band pixels and 519/183 caption pixels;
other content is identical. All 24 SVGs pass unique-ID/local-reference checks.

The correction intentionally leaves measurable residuals. At 4K the old
stroke retains one lower bright row, and the added upper row exceeds the
source's strict R>190 core. Primary top width and faint printing remain
open. The combined horizontal-edge trial and two clipping approaches are
rejected for fractional losses; the scene-wide clip changes 322 frozen
top/text pixels. No threshold or gate is relaxed to accept them.

A separate mailbox audit finds primary body-glyph registration before a
stable secondary contour. Four fixed source/SVG phrase comparisons agree
at 4K; image offsets do not establish responsive native geometry. A shared
design-space baseline trial is a separate next task. No mailbox artwork
changes in this batch; see the [audit](neomil/mailbox-fidelity.md#az-body-text-registration-and-faint-ink).

All three production sizes and the fallback match the reviewed native
candidate exactly. Both fidelity gates, 292 local and Nix Rust tests,
all 22 repository checks and all 27 visual cases pass on their first
attempt. Direct comparison finds 26 exact cases and the unchanged one-level
Neo-kitsch bar pixel. A fresh packaged 4K dashboard matches the reviewed
candidate exactly. Only the Neomil and fallback dashboard goldens change,
by 142 pixels each. All 311 frozen files match the tested Nix source and
nine original images remain unchanged. Final status prose preserves tested
artwork. The 29 broader TODO boxes remain open, including exact glyph/material
work and external live/asset/design/caller prerequisites. Changes stay staged
and uncommitted.
