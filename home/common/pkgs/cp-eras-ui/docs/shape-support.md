# Shape extraction support — 2026-09-27

Entropism's mailbox source shows one left list panel. Its traced rectangle is
at `[83,204,369,483]`; the source extractor had reported two overlapping
chamfers at `[84,206,368,480]` and `[85,288,366,399]`, each with an alleged
124–125 px top cut. These are artifacts of fitting hole-filled colour-family
masks. The unfilled, morphologically closed ink mask retains both straight sides all the way
through the alleged cuts (about 100% support), while the fitted diagonal has
only 0–5% direct ink support. The second detection has no ink along its
claimed top edge, including the adjacent rows. The first has about 92% ink
support along its actual top border. A plain rectangle covers about 71% of
each filled mask, already above the extractor's unchanged 0.62 fit threshold.

The extractor now rejects a chamfer whose claimed missing corner is
contradicted by a strongly supported straight side in that pre-hole-fill ink. It
also collapses a large, near-coincident rectangular outline among the extracted
colour families only if that shorter detection has no supported top edge. A
real docked panel can share left, right and bottom edges with its parent and
still survive because its own top edge is measured. These are extraction
decisions; the matching IoU and 60% source-area requirement are unchanged.

The matcher already counts a rect/chamfer pair at the same box as equivalent.
Its missing-class check now uses the same equivalence. This avoids saying
"no chamfer" when the candidate has the equivalent rectangle; the one-to-one
box matching and source-area gate still charge for missing or moved widgets.

With the corrected extraction, the mailbox source has one left-list rectangle
at `[84,206,368,480]`. The existing candidate matches it, and accounted
source shape area rises from 59% to 84%. A synthetic genuine chamfer retains
its class; a contradicted corner does not. Synthetic shared-outline fragments
collapse. A separate full image-to-extraction fixture draws a parent outline
and a second-ink child sharing its left, right and bottom edges. The child's
measured top border preserves it as a distinct shape. Removing it leaves 58%
of source shape area matched; moving it also fails the unchanged gate.

That end-to-end fixture also caught a palette-reporting variable collision in
`spec_diff.py`: printing candidate residual bins overwrote the earlier shape
match share when the candidate used a source-anchored palette. The loop now
uses a distinct variable. The production shape G1i path uses independently
quantized candidates, but paired specifications now give the same correct
shape result.

The 16 source/SVG G1i audit used the current trace working tree and cached
Python/librsvg tools, with no build. All 16 passed: four Entropism and four
Neomil shape gates, plus four Kitsch and four Neokitsch ink gates. Full
results are in `/tmp/shape-support-all16.log`. The focused Python suite
passed 13 tests.

## Raster-scale peaks and unsupported diamond tips

The SVG/Iced inventory comparison exposed two more segmentation differences.
Kitsch's login barcode is one connected component in both images. The SVG
component fits a chamfer at `[370,632,220,63]`; the Iced component fits the
same outline at `[370,632,221,63]`, with fit scores 0.8682 and 0.9098. The
old distance-transform splitter nevertheless cut the Iced component in two.
Its distance maxima are 32 and 26 pixels high, with a connecting saddle at
23.54 pixels. The smaller peak has only 2.46 pixels of prominence, caused
by the barcode's lower lettering and raster edges rather than another body.

The splitter now merges a maximum into a connected equal or taller maximum
when their saddle is within the existing gap cleanup's spatial uncertainty:
the diagonal radius of its 5×5 square, `sqrt(2) * 2`, or 2.83 pixels. It
measures connectivity in the distance superlevel set, so proximity alone
cannot merge separate lobes. Persistent peaks with a deeper neck still
produce separate candidate Voronoi cells. This also removes unstable extra splits in
the login's small caption blocks. It does not change the input ink mask.

Distance peaks alone cannot distinguish interior texture from separate
widgets. Before committing multiple cells, the extractor therefore checks
whether the whole component has a measured rectangular or chamfered outline.
Every polygon edge must independently retain more than 80% contour support,
reusing the straight-side evidence that already rejects contradicted corners.
The check permits the fitter's existing ±2px local alignment and the same
2.83px raster radius. It neither joins components nor changes their pixels.
Overlapping diamonds lack the required straight exterior edges; overlapping
offset rectangles lack support across the resulting steps.

NeoKitsch's mailbox selected row demonstrates the distinction. Its whole
477px-wide chamfer fits at 0.9655 in SVG and 0.9395 in Iced. A small lower
texture notch and a protruding raster edge give Iced two persistent internal
peaks, although its outer panel edges remain measured. Both renderers now
report the whole panel. A deep, narrow interruption in a synthetic panel
likewise retains one panel; separate overlapping rectangle and diamond
fixtures retain two widgets and fail the gate when either is removed or moved.

NeoKitsch's store had a different artifact: one gold family joined the
rectangular stats material to thin vertical printing. Its observed component
spans x667..929, but fitting a diamond around its distance maximum invented
tips at x614 and x965. The template extended 53 and 35 pixels beyond every
observed pixel at those sides. The same material was independently detected
as a rectangle in another gold family; Iced detected two rectangles instead.

Diamond fits that exceed the whole connected component's observed bounds
by more than the same raster radius now need measured diagonal support.
At least two of the four sides must have the existing 0.62 fit fraction
within that radius of the component boundary. The bounds are those of the
whole component, not its Voronoi cell, so overlapping diamonds retain their
shared support. This is not a blanket ban on inferred tips: fixtures with
one missing tip, two adjacent missing tips, and two opposite missing tips
retain their correct diamond class, centre and size through their remaining
diagonal edges. A rectangular material patch with thin extensions no longer
invents a much wider diamond.

The focused suite now has 19 tests. New image-to-extraction-to-gate controls
keep overlapping rectangles and diamonds separate and reject both missing
and moved widgets at the unchanged 0.65 SVG/Iced matching threshold. The raster-notch
fixture retains one bar, while a pair of genuinely overlapping lobes retains
two. The earlier docked-child, palette, and missing/moved-widget controls
also continue to pass. On the two reported SVG/Iced failures, matched area
improves from 51% to 96% for Kitsch login and 45% to 89% for NeoKitsch store.
These are inventory corrections, not claims that fine typography, grain or
other raster detail is identical between the renderers.

The final four-worker cached-image audit passes all 16 source/SVG G1i gates
and all 13 affected SVG/Iced G2i gates, with the original matching thresholds
and class semantics. NeoKitsch mailbox now accounts for 97% of source shape
area. The two earlier Entropism-store and Neomil-login SVG renders also pass
against the same freshly extracted source specs; their newer working SVGs
were used in the main G1i audit. G2i uses the saved native captures and their
corresponding design images, so it does not claim native verification of
subsequent material edits. Per-pair inputs, specs, verdicts and the result
manifest are under `/tmp/cp-eras-completion/prominence-audit-v2/`.
