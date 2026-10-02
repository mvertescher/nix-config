# Reference correction round thirty-four — AW

AW corrects seven independently measured section badges: Neo-kitsch
dashboard A/B/C/D and store A/B/C. Three Sol lanes prepare the contours
and placement; a follow-up Sol lane audits backing, frame profiles and
SVG specimens. Astra verifies source, SVG and native evidence and
integrates the era-owned path tables.

The badges gain correctly sized letters, external chamfers and filled
lower tabs. Footer C/D move down about 56 source pixels, with C also
moving left about 24. Fixed source profiles support thinner header
outlines and one-pixel right-edge corrections. Independent source ink
samples support caption-colored header outlines and brighter dashboard
tabs. Annotation text and interaction geometry stay unchanged.

All 147 whole-glyph checks improve at three sizes; eight paired state
checks pass. Source masks and color controls retain explicit small-size
edge, corner and counter/glow losses. The ambiguous backing-shape change
is deferred. Detailed measurements are in [scene badges](neokitsch/scene-badges.md).

All 292 Rust tests pass; production frames match the reviewed candidates
exactly at all three sizes. Both reviewed goldens are updated: dashboard
1988 pixels, store 1311 pixels. All 24 SVGs parse with unique IDs, and
references resolve in the three changed sheets. A stricter reference audit
finds a pre-existing Neomil component `#store-ground` dependency; AX is
investigating it separately. All four affected fidelity gates, 292 Rust tests and 22 repository checks
pass. All 27 visual cases pass on their first attempt. Direct comparison
finds 26 pixel-identical cases and only the pre-existing one-level
Neo-kitsch bar pixel. Fresh packaged 4K dashboard/store captures match the
reviewed candidates exactly. All 307 frozen files match the Nix source,
and eight original images remain unchanged. Final status prose preserves
tested runtime, SVG and golden content. No broader fidelity task is closed on the badge results.
Changes remain staged and uncommitted; no deployment or live GUI launch.
