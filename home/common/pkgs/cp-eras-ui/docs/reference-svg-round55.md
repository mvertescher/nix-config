# Reference SVG round fifty-five — font replay and local printing controls

EB resolves the Entropism mailbox SVG font-replay prerequisite and narrows
two material/printing mechanisms. None of the artwork candidates passes
acceptance. Production fonts, SVGs, Rust, source images and goldens remain
unchanged from the verified DZ implementation. All 29 broader scopes stay
open. This round adds evidence and dispatch constraints only.

## Entropism complete-run font replay

Renamed copies of bundled Rajdhani Regular and Medium, selected on the two
complete `URGENT INFORMATION (!)` runs, reproduce the current SVG exactly
at 3840×2160 and directly rendered 1600×900. Only the font family selectors
change in the SVG. The copied fonts differ only in `name` and the checksum
in `head`; outlines, advances, cmap, GPOS and GSUB are unchanged.

Keep baseline and candidate fonts in separate sibling directories with
separate Fontconfig files. Scanning their common parent risks resolving a
different same-name candidate recursively. Astra identifies and corrects
this scratch setup, checks resolved file paths and hashes, independently
inspects the tables, and reproduces six baseline/no-op/candidate renders.
Both no-op frames are RGBA- and PNG-byte-exact. This proves the SVG route,
not a native glyph substitution or the original photographed font identity.

A bounded candidate shortens the parentheses' straight sides by moving
four existing points per glyph. It preserves topology, hint instructions,
glyph bounds and all shaping/advance tables; only `parenleft` and
`parenright` outline records change. It still fails the frozen controls:

- At 4K, 36 of 368 changed pixels lie in other letters, with a maximum
  per-channel delta of three. The cause is unresolved. At 1600, all 123
  changed pixels stay inside the punctuation regions.
- Row-2 RGB MAE improves 16.296→15.788 at 4K and 14.074→13.818 at 1600,
  but 1600 bright-core IoU at green >150 falls .522→.429.
- Selected-heading 4K dark IoU at thresholds 90/110/130 falls
  .064/.110/.128→.040/.093/.120. Smaller-size shape controls also lose.

Astra independently reproduces 168 regional numeric values, checks full
alpha/exterior pixels and reviews the source/base/candidate crop. Keep
this contour rejected. A next source-supported contour can reuse the
exact complete-run SVG baseline, but must still preserve exterior pixels
and pass both source repetitions before any native port.

EC corrects an initial reporting mix-up: the two-level maximum and rows
579/736/751 described the earlier unhinted trial. The reported hinted
candidate changes rows 579/582/752, with a three-level green difference at
(544,752). Both have 36 exterior pixels and both remain rejected. The
source-score table above was computed from the intended hinted candidate.

## Neomil subtitle rows and photographic ground

DW's card-3 registration (+4,−6 original pixels) is retained as the scratch
baseline. Seven fixed glyph windows have no source red-contrast >40
contacts on rows 502–503. That does not establish missing ink: at changed
pixels, source mean red is 73.55/65.07 versus about 22 in the current SVG.
The blue photographic bands can suppress `R−max(G,B)` even with faint red
printing present.

Removing those two rows from the echo worsens fixed card-3 RGB L1
262,802→269,765 at 4K and 40,090→41,311 at 1600. Source-dark and all six
gap-corridor totals also worsen. The full-coverage clip wrapper is exactly
neutral at both sizes, so the row deletion causes these losses. All
157/39 changed pixels stay in the intended projected band; alpha and
bright-primary controls are exact.

Astra verifies all 43 manifest hashes, independently rerenders the wrapper
and rejected candidate at both sizes, reproduces every regional score and
reviews the crop. Do not use a binary red-contrast mask to infer absent
echo ink, or remove isolated failed pixels. A next fit needs a photographic
ground estimate validated outside translated glyph contours, with RGB,
gap and letter-transfer controls. No native candidate is warranted yet.

## Neo-kitsch lower fan

The normalized CU trace supports a nearly horizontal lower-left ridge
from the clipped boundary near (2812,1493) through (2881,1490). A second
seed follows the same basin toward (2912,1487), where its residual drops
below the frozen gate inside the visible panel. This does not identify a
safe visible exit or justify an isolated added stroke.

Fixed vertical profiles at x2820/2850/2880/2910 find 16/18/15/15 source
peaks, versus SVG 5/7/5/4 and native 5/7/5/5. Horizontal profiles instead
find source 15/12/14 versus 16/15/16 in both renders. Astra reruns the
hash-guarded audit and reviews the crop: current grain is diagonal where
the source has a dense local fan. These are sampled density counts, not
authoring-strand counts. Adjacent controls show the same deficit.

The next useful geometry task is a bounded family with observed entries,
supported exits or terminations, and cross-section density controls. A
single truncated CU ridge does not repair the family. Direct small SVG or
native acceptance was not performed; the 1600 overview resizes the full
4K images and is only a scale diagnostic.

## Verification and source-access boundary

The [primary-source follow-up](sources.md#process-image-access-follow-up)
recovers no new process images or original font/material assets. Caller
choices and live desktop verification remain separate prerequisites.

Scratch evidence: `/tmp/cp-eras-next/eb-entropism-printing/`,
`eb-neomil-subtitle/`, `eb-neokitsch-veneer/`, `eb-source-provenance/` and
`eb-root-review/`. Root verifies all 333 previously frozen tracked inputs
before reconciliation. Afterward only documentation changes; DZ remains
the latest runtime check, with 303 packaged Rust tests, all 22 repository
checks and 27 first-attempt visual cases passing. Those earlier runtime
checks are not repeated for this documentation-only round.
