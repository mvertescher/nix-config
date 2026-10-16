# Reference SVG round sixty-five — warning-edge measurement limits

EO reviews three remaining areas after EM's verified material correction:
the Entropism ordinary rail border, Neo-kitsch selected Store ridges and
Kitsch warning feet. The rail and six frame ridges already have corresponding
geometry; their inspected crops do not establish another omitted branch.
The ordinary warning feet look rounded, but ED's shared curve still loses
selected-card detail. No artwork change follows from this review.

EP tests whether straight-side controls can separate warning registration
from ink and blur before another foot curve. It freezes seven side rows,
two blank-band rectangles, three inward sample depths, source repeat offsets
and ED's triangle/bridge/hollow/gap controls. The source is the original
3840×2160 RGB image; samples use pixel centers at integer + 0.5, with bilinear
evaluation where needed. There is no viewport resize or foot fitting.

The first opaque-core predicate is invalid: it uses the varying core's own
median absolute deviation as a tolerance, allowing that variation to validate
itself. Its four positive results do not establish opaque ink. The frozen
script and output are preserved as rejected diagnostics; no foot crossings
were extracted from them. A darkest sampled value cannot automatically
serve as 100% coverage.

A separately frozen centroid method uses positive green contrast beyond
twice the local blank-band deviation in fixed straight-side windows. Its
median source-minus-SVG offsets, in design units, are:

| Card | Horizontal | Vertical |
| --- | ---: | ---: |
| Ordinary 1 | +0.539 | −0.490 |
| Selected 2 | −0.004 | −0.497 |
| Ordinary 3 | +0.400 | −0.409 |
| Ordinary 4 | +0.575 | −0.525 |

These are contrast-weighted estimates, not proven asset translations. The
fixed windows truncate several SVG profiles: only 12 of 56 SVG cross-sections
contain both half-height crossings, compared with all 56 source sections.
Pooling just the available widths creates an unequal comparison. Source and
SVG also subtract different local contrast floors. The reported pooled
widths therefore cannot establish that the source vector stroke is narrower.
The left/right peak differences have opposite inward signs; their equal
0.5-native-pixel magnitude reflects the sampling grid, not subpixel precision
or a common narrowing.

Review also corrects a mislabeled blank-RGB field that was presented as a
deviation, and rejects an incorrect attribution of compliance-text wrapper
translations to the warning. The warning is in `#card-art/#bandglyphs`;
the translated `#card-compliance` contains separate lettering. Actual warning
origins are `(484,218)`, `(804,218)`, `(1123,218)` and `(1443,218)` design units.
The latter two differ from the nominal source repeat offsets by only
0.4 physical x pixels, with no wrapper-induced y offset. Preserved correction
v1 is rejected; correction v2 and the independent XML walk supersede it.

Astra reproduces 112 centroids, 112 source/SVG profile records and twelve
registered core-depth groups, checks the frozen inputs, and independently
rerenders the current Kitsch SVG byte-exact. Registered color variation does
not establish a unique opaque ink or physical foot boundary. No foot curve,
font correction, native trial or new golden is accepted. K8 remains open.
Any further edge study needs complete, corresponding outer-to-hollow profiles
with neighboring print excluded geometrically, and must distinguish local
contrast from true coverage before proposing a shared contour.

Evidence: `/tmp/cp-eras-next/eo-next-scope/` and
`/tmp/cp-eras-next/ep-kitsch-foot-locus/`, especially `root-verification.json`
and `correction.md`. Only documentation changes. EM remains the verified
runtime checkpoint: 305 packaged Rust tests, 22 repository checks, 28
first-attempt matrix cases and exact fresh packaged 4K parity. Its measured
production foreground CPU improvement is 20.5%. Original images, fonts,
runtime code and goldens are unchanged by this round; all 29 broader scopes
remain open.
