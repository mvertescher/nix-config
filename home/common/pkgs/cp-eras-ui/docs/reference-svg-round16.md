# Sixteenth reference checkpoint — AE, 2026-09-29

Three Sol medium lanes investigated Kitsch SC lettering, the Neomil store
margin and Neo-kitsch EMAIL grain. Astra reviewed source/SVG/native
evidence and integrated two bounded corrections. The staged AD work is
preserved. No commit, push or deployment is part of this checkpoint.

## Kitsch SC coverage

The repeated SC run now uses Rajdhani SemiBold 4.3 at baseline 85.5,
with its original x position and opaque band ink. The trace, component
sheet and runtime agree. The square, disc, corner apertures and neighboring
marks retain their geometry and colors.

The candidate was fitted on the first source copy before the other three
copies were measured. At 4K, native dark-core F1 improves in seven of eight
card/threshold comparisons. At 25 levels below blank-disc luminance the
four scores change .726→.838, .372→.379, .636→.769 and .722→.649. At 35
levels all four improve. RGB error improves in every card's top, middle
and bottom patch. Only 336 pixels change, all inside the SC runs; blank
disc and neighboring-art controls are pixel-identical. The fixed exclusion
mask comes from the pre-change SVG/native footprint, independent of this
trial.

The fourth card's loose-threshold loss is explicit: 11 of 38 source pixels
previously matched no longer match. This common type correction improves
the dominant excess coverage, but does not recover exact photographic
phase, ink or glyph contours. A conservative weight-only alternative fails
seven of eight native core scores and is rejected. See the
[SC findings](kitsch/store-art.md).

## Neomil second-A counter

A narrow semantic-background stroke extends the second A's counter along
local `(84.6,.75)` to `(82.8,.75)`, width .25. The new SVG stroke has the
runtime's butt ends. Five 4K pixels change, all in the source-dark column
x126/y1198–1202. The neighboring bright column and the broader junction,
O counter, leading/middle hatch, slash, CJK and other letters are unchanged.

Native A2 RGB error improves 24.426→24.027, with source-dark intrusions
15→11 and all 65 strong source-core matches retained. In the tighter
counter crop, error improves 29.739→27.712, intrusions 9→5 and all 21
strong matches remain. The SVG independently improves the same dark gap
without losing a strong source stroke. Wider cuts erase supported bright
pixels, and an even-odd subpath toggles existing self-intersecting fill;
neither is applied. See [store printing](neomil/store-printing.md).
The broad A2/O junction and plaque material remain open.

## Neo-kitsch rejected grain trials

Three added upper diagonals improve dominant direction but worsen RGB
error. Source row profiles show several anchors landing on bright material
beside the actual ridges. A corrected single branch improves its middle
patch only: RGB 20.488→20.372. Its entry and merge worsen
18.646→19.314 and 19.986→20.910; the full branch worsens 20.017→20.180.
Nearby dark peaks do not establish that the diagonal joins those specific
longitudinal strands. Adjacent and below-join controls remain identical.
No grain geometry is changed. The [veneer findings](neokitsch/veneer-fit.md)
retain verified anchors and the unsupported endpoint/connection problem.

## Integration

All 286 Rust tests and the release build pass. Twenty native captures cover
4K/1600 and fractional rest, ordinary/selected/last-card held, last-card
selection, custom palettes and opening. Both production 4K captures exactly
reproduce their approved native trials. All matched state deltas stay in
the intended regions. The margin is invariant under shelf/opening states
and preserves custom foreground/background roles. Kitsch's disc/ink roles,
fourth-card cutoff and early opening remain intact.

All 24 SVGs parse with unique IDs. Both source and both implementation
gates pass at unchanged thresholds. Only the two reviewed store goldens
are refreshed: two Neomil pixels and 96 Kitsch pixels at 1600×900. The full
repository check passes all 22 checks and all 27 visual cases. Every case
matches 100.000% on its first attempt. All 215 frozen source, documentation,
script, test and TODO hashes match the Nix source used by that run. Evidence
is under `/tmp/cp-eras-next/ae/`, including `check-verification.json`,
`state-review.json` and the local source/native measurements. Final prose
records these results; runtime, SVGs, scripts, tests and goldens remain
unchanged from the verified snapshot. Changes remain staged and uncommitted.
