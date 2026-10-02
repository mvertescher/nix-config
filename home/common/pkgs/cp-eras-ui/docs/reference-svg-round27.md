# Reference campaign batch twenty-seven — AP

AP audits the Entropism store's BETTERLIFE TEC printing. The source is
`images/entropism-dashboard.png`, the store despite its documented name.
The three visible labels start 34–35 native pixels before the old SVG
and Iced labels; their right edges already agree within two pixels.
The source also has darker strokes and a baseline about two pixels lower.
A fresh native baseline is pixel-identical to the AO packaged 4K frame.

## Selected label

A common horizontal stretch restores width but introduces stems in source
letter gaps. The retained selected-card trial instead places thirteen
Medium glyphs at independently measured spans, keeping the existing size
and fixed band ink. The source lettering remains legible as BETTERLIFE TEC;
this is a font approximation, not recovery of its original font or grain.
Only the selected-card primitive and selected SVG/component excerpt change.
Ordinary cards retain their existing label pending separate gap fitting.

SVG dark-core overlap at R,G<100 rises .035→.582. Native overlap rises
.024→.575 after lifting the stretched native glyphs one physical pixel at
4K relative to the SVG baseline. All fixed first/middle/last segments
improve at thresholds 90, 100 and 120. Native changes are confined to 1,702
pixels in the selected label, with bounds x1554–1726/y780–796 including
removed old ink. Both ordinary labels, PETROCHEM, fields, weapons and the
fourth-card cutoff remain pixel-identical at 4K.

Measured gap intrusion falls from 32 to 16 pixels in SVG and 61 to 16 in
Iced. The remaining gap pixels do not form full-height stems. Individual
gap edges still gain up to three dark pixels; rounded C contours and
photographic softness remain approximate. Iced's unstretched I retains
one extra bottom row, so exact glyph height is not declared complete.

## Rejected ordinary transfer

The selected glyph table is frozen before testing the ordinary copies.
Both then start two native pixels late. A second calibration based on
ordinary card 2 adds a shared −0.6-design-unit ordinary offset, improving
whole and segmented masks in both copies. It still changes card 3's SVG
gap intrusion 40→104, including six complete fourteen-pixel dark columns
and one twelve-pixel column in source-empty gaps. Card 2 has three complete
dark gap columns. That transfer is rejected. No per-card offsets are
introduced, and the ordinary manufacturer task stays open.

## Verification

All 290 Rust tests, both affected fidelity gates and 24 SVG parse/unique-ID
checks pass. Thirteen paired drawing states preserve every pixel outside
the selected label at 1600 and fractional sizes, through all selected
positions, hover/held feedback, custom colors and opening. The last-card
and early-opening controls are exact; five fixed-band-ink controls match
rest. These are synthetic drawing checks, not real input-event tests.
The production 1600 capture equals the reviewed preview, and only the
Entropism store golden changes (372 pixels).

Root review catches an RGBA comparison flaw in the prepared scratch state
script before captures: a zero alpha difference hides RGB changes under
Pillow's default bounding-box behavior. Explicit RGB comparison and an
alpha-unchanged negative control repair the audit before it runs.

All 22 repository checks and all 27 visual cases pass on their first
attempt. The Nix package passes all 290 tests without a retry. Direct
comparison finds 26 pixel-identical cases and the unchanged Neo-kitsch
bar's pre-existing one-level pixel at (749,20). All 293 frozen files
match the tested Nix source, and five original images remain unchanged.
A fresh packaged 4K frame matches the reviewed selected correction exactly.
Final documentation leaves tested implementation, SVGs and goldens intact.

The selected width/weight correction is verified. The ordinary labels
and exact glyph/printing limits remain open. All changes remain staged
and uncommitted; nothing is pushed or deployed.

Scratch evidence is under `/tmp/cp-eras-next/ap/` and
`/tmp/cp-eras-next/ap-entropism-print/`. The first native wrapper fails
module lookup for `resize`; a separate copy fixes the explicit module
path before captures. The accepted baseline is pixel-identical to the
packaged AO screen. Synthetic pointer previews validate drawing states,
not live event routing or hardware presentation.
