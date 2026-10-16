# Reference SVG round fifty-six — ground controls and paint locality

EC validates a local photographic background estimate for Neomil Store
subtitles and narrows the Entropism complete-run painting interaction.
Two subtitle candidates still fail local controls. Mailbox phase and
Neo-kitsch grain studies likewise provide no accepted geometry change.
Production fonts, SVGs, Rust, source images and goldens remain at the
verified DZ implementation; all 29 broader scopes stay open.

## Neomil subtitle ground and impression

The original Login image has blank background at the ordinary Store
subtitle coordinates. It is not pixel-identical to Store there. Fit one
constant RGB offset using only each card's lower-left blank rectangle;
reserve a lower-right rectangle and a same-height upper-right rectangle
as independent controls. The offsets for cards 1/3/4 are (+10,0,−1),
(+11,0,−2) and (+11,+1,0). All six held-out rectangles have red p90 error
2, RGB p90 error at most 2 and absolute channel bias at most 1.402.
The card-3 upper control has RGB MAE .681 over 1,680 pixels.

This validates a local ground estimate, not direct observation beneath
Store ink or recovery of the original background recipe. Login and
Dashboard pixels are identical in these subtitle boxes, so they are not
independent donors. Controls immediately outside the model's glyph mask
contain wider source printing: they cannot be treated as blank simply
because the current model draws nothing there. Earlier ground estimates
that extrapolated from those controls are superseded by the clean-rectangle
donor result.

Subtracting adjusted Login supports faint red printing throughout the
fixed upper echo windows and beyond the sharp DW copy. All 163 H/A/N and
190 G/U/N model-support pixels in card 3 have donor-relative source red
excess >8; another 313/220 source-positive pixels lie outside that support.
The historical `R−max(G,B)<15` mask includes blue-washed red printing and
does not identify blank ground reliably.

Two separately frozen complete-impression trials are rejected:

- Adding a Gaussian underprint at opacity .10 and design sigma .60 beneath
  DW's sharp .25 copy improves broad RGB error but overstates ink locally.
  Post-rejection attribution puts most losses on donor-relative ink rather
  than blank ground. It does not invalidate the donor estimate.
- Replacing the sharp copy with one softened impression uses a finite
  5×5 width/opacity fit on all RGB pixels in card-3 upper H/A/N, including
  negative space. The selected design sigma .40 and opacity .34 transfer
  unchanged to G/U/N and cards 1/4. The opacity reaches the grid boundary;
  the fit does not identify a precise physical ink strength. Its fitting
  proxy blurs an already composited foreground increment, so the actual
  SVG renders, rather than that approximation, decide acceptance.

The replacement's card-3 upper H/A/N RGB L1 improves by 1,235/206 and
held-out G/U/N by 1,201/208 at 4K/direct 1600. Its full subtitle box improves
by 6,930/367, but 532/95 pixels lose more than three mean RGB levels.
Donor-relative blank error worsens by 34 at 1600, including a loss of 8
in blank gap corridors. Cards 1/4 also lose on small-size blank controls.
The zero-effect replay, alpha, exterior, selected card and fully opaque
primary pixels are exact at both sizes for both trials. Source 1600 is
resized from the whole RGB photograph; SVG 1600 is rendered directly.

Astra checks both manifests, reproduces the actual SVG renders and score
files, independently computes the six donor controls and replacement's
fixed card-3 scores, and reviews source/current/DW/replacement crops.
No candidate is ported. Preserve the validated donor method; another
uniform blur/opacity search lacks a new mechanism for the remaining local
losses. Do not mask individual failed pixels or infer absent ink from low
red contrast.

## Entropism whole-run paint locality

The rejected EB hinted-parentheses candidate has 36 exterior changed
pixels at 4K, with a maximum channel delta of three. Earlier prose mixed
this with an unhinted trial's two-level maximum; round 55 now records the
correction. The source-score table had used the intended hinted candidate.

An independent Pango probe selects the intended Regular/Medium font bytes
and produces identical glyph IDs, clusters, positions, offsets and
advances for both complete 22-glyph runs. Unchanged used glyph outlines
and metrics are exact. This probe recreates the layout settings; it is
not a capture of librsvg's internal layout object.

Minimal transparent SVGs containing the complete runs reproduce all 36
full-frame exterior coordinates as alpha changes. They have 37 exterior
alpha changes in total; one is hidden by full-frame composition. Removing
the edited punctuation from those diagnostic SVGs makes all four
unchanged-prefix pairs RGBA-exact at 4K/direct 1600. A separate unused-@
outline edit also leaves the complete production SVG exact at both sizes.

The evidence narrows the interaction to painting a run containing edited
outlines, and is consistent with a path-fill/antialiasing interaction.
It does not identify the precise internal stage or prove a Cairo defect.
The font binary rewrite alone, changed prefix contours and changed probe
placement do not explain the effect. Astra reproduces the probes and
isolated renders and independently checks their coordinate correspondence.
Do not split production text into extra draws to hide these pixels: the
contour also fails source-fidelity controls and no safe replacement is
established. Keep exact no-op and exterior checks as separate requirements.

## Neomil mailbox terminal phase

The first phase grid compared slightly different footprints after each
shift. Astra rejects that variable-support comparison; the corrected
study uses the same 169 x samples for every baseline, candidate and row.
Fitting ordinary row 2 gives shift −.325 native pixel and pitch ×1.010.
Correlation improves .378→.449 there, but falls .466→.424 on row 3 and
.467→.349 on row 7. It does not support a shared rib-path correction.

The observed photographic upper edge varies by row while the SVG and
native authored origins agree to within .00008 native pixel of rounded
coordinates. Aligning to that observed edge as a diagnostic changes the
preferred row-2 shift to +.025; correlation improves only .447→.455 and
still loses row 3. The edge measurement is a raster feature at .125-pixel
resolution, not the original authoring contour. Apparent phase therefore
does not isolate a common geometry error from row-local registration and
material variation.

Astra reproduces five result files and reviews the four-row source/SVG/
native montage. Native evidence is the pinned accepted-AM 4K capture,
whose terminal geometry is still current; this is not fresh full-frame
runtime parity. Its 1600 view is downsampled, unlike the direct SVG.
Preserve X's sloped-top conversion, AM's deeper bed/rim, twenty full ribs
and two cap strokes, and the separate selected twenty-two-rib artwork.
Exact printing remains open; do not repeat a shared phase/pitch adjustment
without evidence that transfers across rows.

## Neo-kitsch lower fan correspondence

Twenty-two paired left-edge source peaks seed a family diagnostic. A
strict local tracker stops inside the panel even where visible contrast
continues: those stops are algorithm limits, not supported strand ends.
A momentum bridge follows the broad S-turn but merges paths and creates
return loops. Eleven tracks reach the top at only eight distinct positions;
nine return through the left boundary and two hit the step limit.

Astra reproduces the family data, independently checks boundary counts
and reviews the full-panel overlay. The known density deficit remains,
but this trace is not a complete SVG candidate. Distinct, ordered strand
correspondence through the turn is unresolved; reaching a clip boundary
by momentum does not establish a source-supported exit. No arbitrary
interior taper, duplicate upper line or native port is accepted.

## Verification and next work

The font audit and bounded caches remain complete. Unchecked scopes still
comprise 17 source-fidelity items, six caller/design prerequisites, two
shared-background asset items and four live desktop checks. No new font
identity or original material asset is recovered. User choices and live
verification cannot be replaced by headless source comparisons.

Scratch evidence is under `/tmp/cp-eras-next/`: `ec-subtitle-ground/`,
`ec-font-locality/`, `ec-font-locality-worker/`, `ec-mail-terminal/`,
`ec-veneer-family/` and `ec-root-review/`. The latter holds independent
donor/score/coordinate/count checks and byte-exact reproduction records.
Only documentation changes in this round. Runtime inputs and all sixteen
source photos retain their frozen hashes. DZ's 303 packaged Rust tests,
22 repository checks and 27 first-attempt visual cases remain the latest
runtime verification; they are not rerun for these prose changes.
