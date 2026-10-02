# Reference SVG review — round twenty-one

AJ starts from the verified AI working tree. Sol implementation lanes own
scoped changes; Astra reviews source images, SVGs, native captures and the
integration. Changes stay staged and uncommitted.

## Login notice echo

The Neomil active notice gains a lower-left secondary print candidate.
The source fit uses offset (−2.5,+2.5), stroke .7, Gaussian sigma .45 and
opacity .30 in design units. The primary text definitions stay intact.
Both fitting spans and four independent echo-only spans improve; three
blank controls are unchanged. Complete SVG notice regions improve RGB RMS
in all six spans and F1 in forty of forty-two threshold comparisons, with
one tie and one small loss. High-alpha primary fringes retain small losses.

The isolated native preview with the echo disabled exactly matches AI at
4K. With the echo enabled, all six complete-notice RGB errors improve and
all eighteen echo-only F1 checks improve. Whole-notice masks have some
small losses (up to .0017 F1); the photo's exact glyphs and repeated scan
texture remain approximate. The 14,311 changed pixels stay inside the
active notice area. A 900×1200 preview follows the fitted primary text,
changing 1,088 pixels only in that notice.

The production candidate uses era-owned note-echo data and a cached local
glyph tile beneath the primary artwork. It follows the same block fit,
omits the source printing for custom palettes, and avoids a full-screen
raster allocation. Native/state and cache checks pass; full repository verification passes. See [the source study](neomil/login-primary.md#active-notice-echo-investigation--aj).

## Neo-kitsch A contour

A dedicated A contour replaces the broad Rajdhani approximation while
keeping B, both badge frames and tabs unchanged. The fixed SVG/source
orange-mask IoU improves from .724 to .914 at threshold 150. Both legs,
apex, crossbar, counter and feet improve at three thresholds. Gap pixels
stay below all bright thresholds; a faint halo raises its RGB error from
5.96 to 6.00. The native representation uses existing badge paths, with
opposite winding for the counter. Native review improves all 42 fixed glyph comparisons; only 286 pixels
change at 4K and 71 in each fractional state. Plates, tabs, B and the
clear gap remain identical. A supplemental downsampled-source comparison
has one small high-threshold left-leg loss, recorded separately.
See [the badge measurements](neokitsch/login-branding.md).

## Other investigations

The Kitsch ghost-stack reorder is rejected. Its front-outline width has a
separate measured discrepancy; profile losses and position/material
residuals are recorded in [the fan study](kitsch/dashboard-material.md#aj-front-outline-and-overlap-review).
E2 mailbox global font/ink changes and the NK-14 frame gradient remain
rejected as recorded in [round twenty](reference-svg-round20.md#next-round-scratch-review).

## Integration

All 288 Rust tests and four affected gates pass. Neomil's source shape
area is 91% (previously 94%), while its SVG/native area is 99%. Direct
1600 SVG comparison changes only 2,282 notice pixels; the changed shape
classification outside that area comes from extraction, not artwork.
Neo-kitsch's source ink-placement IoU is .74 and SVG/native shape area
87%. Gate thresholds and scripts are unchanged.

Twenty-two native captures cover source sizes, fitted narrow layouts,
preview parity, Dim-only custom colors and seven fractional states per
era. Neomil changes the same 2,319 notice pixels in all six reference
states; both custom controls are byte-identical. Its production 4K and
portrait images match the reviewed scratch preview exactly. Independent
left/right overflow probes find zero notice pixels outside the active
card. Native A controls stay unchanged, with state differences confined
to the original inputs/actions. Two reviewed login goldens are refreshed.
An instrumented 4K preview draws 86 times while cycling interaction/input
states, copying slot data with half opacity, restoring it, applying a
custom Dim palette, then restoring the reference. It prepares exactly
three tiles: initial, changed data and restored data. Their checksums are
A/B/A, with B different; 16 custom-palette draws prepare no tile, and
returning to the reference reuses the cached original. The final image
exactly matches production. A tile is 602×66 pixels (158,928 RGBA bytes)
and takes 5.24–6.00 ms to prepare in this run. These are local CPU tile
costs, not full-frame timing or hardware presentation measurements.
All 22 repository checks pass. All 27 visual cases pass on their first
attempt at the reported 100.000%. Direct RGBA comparison finds 26
pixel-identical cases, including both changed logins. The remaining
Neo-kitsch bar differs by one channel level at one pixel (749,20), already
present in AI: the AI and AJ bar renders are pixel-identical. Its golden
is retained. Rounded similarity alone did not expose this difference.
All 222 frozen source, document, script, test, golden and TODO hashes
match the tested Nix snapshot; all
three local reference images are unchanged. All 24 SVGs parse with unique
IDs. The two bounded login follow-ups close. Remaining exact glyphs,
photographic softness and scan repetitions stay open. Final prose records
these results without changing tested runtime, SVGs, scripts, tests or
goldens. Changes remain staged and uncommitted.

Evidence for this round is under `/tmp/cp-eras-next/aj/`, with source-fit
studies in the adjacent `aj-login-echoes`, `aj-neokitsch-a` and
`aj-kitsch-front` directories.
