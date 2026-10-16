# Reference SVG round sixty-three — native rifle ink and coverage

EJ rejects a joint native port of the accepted Entropism SVG stock details.
The complete ordinary rifle improves at 4K but loses accuracy at smaller
sizes, particularly along the rail. Production art, fonts, caches and
goldens remain unchanged. The next investigation is native edge coverage,
not another ink sweep.

The scratch application uses the actual Store and the current EH library.
All 71 Rust source files, Cargo inputs and 15 embedded font files match
that library's frozen inputs. Fresh baseline captures match the packaged
Entropism Store at 3840×2160, 1600×900 and 1537×947, and Kitsch at 4K,
both in RGBA and PNG bytes. Source projections retain the uniform sampler
and metadata verified in EI.

One candidate changes ordinary bright rifle ink from semantic Select
`#a6d3a7` to the SVG's `#b3ddb1`, and adds the exact EH seam and EI band
as opaque filled polygons. Astra verifies both 78-vertex polygons against
the current production SVG at f32 precision. Mid body ink and selected
inverse art stay unchanged. The probe applies only to the reference
palette; it makes no production routing decision.

Whole-rifle RGB L1 against the source, using fixed boxes for the two
complete ordinary cards:

| Size | Card 2 before → candidate | Card 3 before → candidate |
| --- | ---: | ---: |
| 3840×2160 | 2,523,178 → 2,319,041 | 2,742,588 → 2,536,123 |
| 1600×900 | 425,085 → 432,212 | 462,420 → 469,072 |
| 1537×947 | 395,860 → 405,084 | 427,392 → 437,090 |

Stock error improves 16.5–21.6% across all six comparisons. Rail error
improves only 0.4–0.5% at 4K and worsens 14.5–17.0% at smaller sizes;
receiver and trigger crops also regress at those sizes. Astra independently
reproduces all 60 regional comparisons. The 64,411 / 13,977 / 12,879
changed pixels stay inside the three ordinary rifles, including the
clipped fourth card. Alpha, selected art, neighboring controls, custom
palette and Kitsch negative controls remain exact.

A fixed diagnostic separates changed solid pixels from the remaining
coverage pixels. Solid pixels must be within two RGB levels of both known
inks in their respective captures; the source does not choose this mask.
At small sizes, solid receiver regions improve while the remaining pixels
reverse that gain. Over 90% of the 1600 non-solid rail pixels lose, with
mean source RGB around `(119,146,117)`, baseline `(134,170,134)` and
candidate `(142,176,140)`. The brighter ink increases an existing edge
error. All 36 diagnostic groups independently reproduce.

The current SVG has fewer solid rail pixels and lower small-size rail
error than the native candidate despite sharing its bright path and ink.
This implicates coverage/translation and photographic softness, without
isolating the complete renderer cause. Preserve the accepted contours until
a controlled rendering comparison supports a change. No selection,
opening or feedback trials follow this rejected resting candidate.

## EK controlled edge probe

A small actual-Store scene renders matching opaque vector rectangles in
Iced and librsvg. Four fixed subpixel phases, three declared background
colors and a black/white control separate color blending from source
photography and font selection. Inputs, current library and font hashes
are guarded; all solid foreground/background checks pass. Coverage is
inferred separately under each blend hypothesis from the black/white
control, without fitting colors to the observed colored edges.

The native path matches linear-light blending within one channel level;
the SVG matches encoded-RGB blending within one level. Across twelve
colored edge samples per renderer, native total RGB error is 8 for the
linear prediction versus 228 for encoded; SVG error is 7 for encoded
versus 281 for linear. Astra independently reproduces all predictions.
At the half-covered edge of `#b3ddb1` over `#110c07`, native is
`(131,162,129)` and SVG is `(98,117,92)`.

This difference is intentional: the compiled graphics features exclude
`web-colors`, as documented in Cargo.toml to preserve existing glow/text
behavior. Native paths use 4× MSAA. Do not change the shared color pipeline
or compensate with font weight merely to erase a source/SVG/native edge
difference. This simple probe establishes blending behavior, not the
fraction of rifle error attributable to it; coverage quantization and
photographic softness remain separate. No extra rendering passes, caches
or production branches are added. See the [font audit](font-rendering-audit.md#ek-vector-edge-blending-check).

Evidence: `/tmp/cp-eras-next/ej-entropism-ink/` contains the guarded probe,
captures, fixed boxes, scores and diagnosis; `/tmp/cp-eras-next/ej-root-review/`
contains independent verification and rendered comparisons. The controlled
probe and independent predictions are in `/tmp/cp-eras-next/ek-native-coverage/`.
This round
changes documentation only. EI's full 22-check/28-case result remains the
latest integrated verification; EF remains the runtime checkpoint. All
29 broader scopes remain open, and changes stay staged and uncommitted.
