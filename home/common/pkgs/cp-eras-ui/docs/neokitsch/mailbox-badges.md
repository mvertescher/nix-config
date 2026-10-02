# Mailbox section badges — AV

The mailbox source is `images/neokitsch-mail.png`, Behance Part 1 screen
#71 (`f1104d118663901.60e5fa6e2fce8`). Its SHA-256 is
`182d83a0b8d6c07f69dd65a0192f37180c47d1d18bb1d2c7e5dd7b313a13ebe3`.
Measurements below use that original 3840×2160 image, the reference SVG,
and fresh headless Iced captures. Source coordinates divide by 2.4 for
the 1600×900 design frame. The prior native 1600 capture exactly matches
the previous mailbox golden.

## Corrected geometry

All four source badges have an external lower-right chamfer and a filled
lower tab. The former generic symbol instead drew an internal fold and
omitted the tab. The mailbox now has its own frame paths; A/B independently
require a right edge .75 design pixels left of a literal C/D translation
and a chamfer beginning 1.5 pixels lower. Other screens have separate
source requirements and keep their existing art.

A/B/C/D use sparse source-fitted contours, including reversed counter
winding. A has wider legs and a triangular counter; B has two rounded
bowls; C has straight inner rails and gently rounded outer corners; D has
a rounded outer bowl and clear counter. An earlier pixel-boundary D was
rejected in favor of coherent curves. These are local reconstructions,
not recovered original font outlines.

`Piece::LabelArt` retains each `Note` and its existing anchor. The anchor
follows both window axes; contour offsets scale uniformly by the smaller
axis, like text. The authored offsets already include size, alignment and
stretch. Empty artwork falls back to ordinary text; empty content or a
nonpositive text size suppresses the contours. Inherited ink and motion
opacity still apply. The renderer contains no era-specific glyph paths.

## Source comparisons

Fixed whole-glyph native gold-mask overlap at 4K, R>190 with R>1.1G and
G>1.1B:

| Glyph | Before | Contour |
| --- | ---: | ---: |
| A | .426 | .852 |
| B | .107 | .821 |
| C | .116 | .939 |
| D | .283 | .931 |

Across 100 fixed whole/stem/bar/bowl/counter checks at R180/190/200/210,
96 improve and four empty-mouth checks tie. The three-size whole-glyph
contrast comparisons improve at every .35/.50/.65 threshold. At .50:

| Glyph | 4K before → after | 1600×900 | 1537×947 |
| --- | ---: | ---: | ---: |
| A | .448 → .793 | .383 → .863 | .364 → .656 |
| B | .143 → .782 | .123 → .807 | .112 → .773 |
| C | .118 → .838 | .174 → .900 | .175 → .761 |
| D | .269 → .804 | .305 → .887 | .326 → .700 |

Fractional glyph comparisons map the source about the responsive anchor
using uniform glyph scale; they do not stretch the photographed letters
vertically. Frame comparisons follow both axes. Geometry-only frame masks
use fixed source/native contrast ranges, deliberately separating the dark
native ink from shape. All 180 top/left/right/bottom/tab comparisons
improve across three sizes and three thresholds. This does not establish
matching frame color or photographic softness.

The SVG also improves each whole glyph. Its 1600 A bar and legs retain
more missing source pixels despite fewer extras and better overlap. B's
upper bar is thicker than the first trial at 4K, but improves on the old
SVG and preserves its small-size bar. Fine threshold, counter-edge and
photo-bloom residuals remain open.

## Ink decision

The source frame crests are much brighter than the old native Dim role:
R227–254 on C/D and R232–250 on independent A/B controls, versus native
R138 and SVG R189. Brightening every outline at the existing width makes
several C/D edge and chamfer controls worse. That trial is rejected.
Outline profile, alignment and brightness need to be fitted together.

Only the newly recovered lower tabs use foreground ink in Iced and
`#e7c686` in the SVG. Their fixed-region RGB RMS error improves in all
12 native comparisons and all eight SVG comparisons. At 4K, native
A/B/C/D error falls from 100.07/85.43/119.46/117.70 to
30.94/28.51/42.65/43.99. Native outlines retain Dim; glyphs and tabs use
the current foreground role, including custom palettes. No global palette
color changes.

Two component-sheet A specimens copy the mailbox frame and A path.
Store-derived specimens retain their independent symbol. Provenance and
captions now identify these distinctions.

## Validation status

The integrated candidate changes 6685/1725/1640 pixels at 4K, 1600×900
and 1537×947, all inside the four badges. Its tab-only color adjustment
leaves every outline and glyph pixel unchanged outside the tab region.
Empty-art fallback matches the former native image exactly at all three
sizes. Ten paired native states preserve locality, including first/last row,
custom foreground/Dim, opening, synthetic half/zero opacity and partial
clips. Empty text and zero size suppress the same glyph pixels. A custom
empty-art fallback also matches ordinary text exactly. These motion
fixtures exercise the helper; production badge chrome remains stationary.

Integrated production matches the reviewed candidate at all three sizes.
Both fidelity gates, 24 unique-ID SVG parses and 292 Rust tests pass.
Only the mailbox golden changes, by 1725 pixels. All 22 repository checks and 27 visual cases pass on their first attempt.
Direct comparison finds 26 exact cases and only the pre-existing
one-level Neo-kitsch bar pixel. The package's fresh 4K capture
matches the reviewed candidate exactly; all 303 frozen files match the
tested Nix source and eight originals remain unchanged. See
[round thirty-three](../reference-svg-round33.md).

Final combined gap controls remove all old false-fold pixels at 4K:
source/candidate counts are zero in all four patches across R85–210.
Small captures retain occasional one-pixel fringes: C at 1600 has one
extra pixel at R120, and C/D have one faint pixel in the fractional crop.
These are retained limits, not a recovered interior fold. All higher
R170/190/210 gap controls remain clear. The bounded contour/tab/topology
correction is verified; exact printing and frame material stay open.
