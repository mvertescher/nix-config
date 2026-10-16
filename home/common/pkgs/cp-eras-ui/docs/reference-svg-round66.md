# Reference SVG round sixty-six — complete warning profiles

EQ repairs the incomplete side windows from round sixty-five before proposing
another Kitsch warning contour. Each source/SVG comparison uses corresponding rows
with both half-height crossings inside a complete outer-to-hollow window. The
windows exclude the exclamation, tip, feet, frame and neighboring print. The
measured signal is local blank green minus sampled green, without subtracting a
contrast floor or treating it as physical ink coverage.

At actual card positions, all 52 valid paired source spans are narrower than the
current SVG. The median difference is −2.044 physical pixels; individual differences
range from −2.591 to −0.976. Four right-side source profiles reach the window
boundary and are excluded from both sides of the comparison. Background sensitivity
at ±2 local median absolute deviations retains the narrowing result; one additional
pair becomes unavailable at the positive extreme. EP's separately labeled
provisional registration gives 56 valid pairs and the same direction, but its
offsets remain diagnostic rather than proven asset translations.

Astra independently verifies 8,000 samples, 224 profiles and 1,120 contrast cases.
The source/SVG half-height crossings vary between cards, so neither a common phase
nor a unique opaque ink follows. The horizontal distances are explicitly
distinguished from perpendicular distances to the sloping side.

ES then captures the current packaged Kitsch Store at 4K. It exactly reproduces the
accepted native baseline. Applying the same windows to that capture gives 52 valid
pairs, each with a wider native span than the source. Native blending reduces the
apparent SVG width by roughly 0.2 physical pixels in these controls, but does not
explain the roughly two-pixel source discrepancy. This establishes a native
follow-up worth testing without changing any font or shared blending behavior.

ER freezes one SVG hypothesis from card 1 alone: enlarge the existing triangular
hollow with one common parallel inner-edge offset. The measured horizontal intercept
shift is 0.891255 design units, equivalent to 0.787390 perpendicular units. New
inner vertices are `(32.108745,87.8)`, `(36.5,79.519348)` and `(40.891255,87.8)`.
The outer contour, inner base height, bridge, frame and exclamation node stay
intact. This is a visual hypothesis, not recovery of the photographed physical
outline.

The other three cards and both smaller viewports are holdouts. Source sampling uses
the repository's uniform top-left Lanczos sampler with retained metadata; fractional
padding is excluded. Current/no-op renders are identical at 3840×2160, 1600×900 and
1537×947. Astra reproduces all nine SVG renders, the three source samples and 168
fixed-region scores.

| SVG whole warning RGB L1 reduction | Card 1 | Selected 2 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: | ---: |
| 3840×2160 | 16.3% | 31.2% | 23.9% | 3.1% |
| 1600×900 | 21.5% | 34.1% | 29.3% | 5.7% |
| 1537×947 | 25.9% | 34.3% | 31.8% | 7.1% |

The fourth card remains a weak holdout. Its left-side error increases 16.0%, 4.2%
and 4.9%; its apex also worsens. Its left-foot region also worsens, with three
changed pixels at 4K and two at each smaller size, while the exterior foot geometry
is unchanged. These local losses are retained explicitly rather than hidden in the
whole-mark gain. The bridge, frame and exterior blank controls are exact, and all
526/136/141 changed SVG pixels stay inside the warning boxes. Unchanged exclamation
geometry does not imply identical rendered pixels where the underlying hollow
changes.

ET transfers precisely those three vertices to the native path, preserving its
accepted bridge at y88.8. Fresh copies contain all 72 current Rust sources, 15
font-directory files and the unchanged production Store entry point, including
navigation subscriptions. Pinned compiler, dependency and vendored renderer hashes
pass. Baseline 4K exactly matches ES's packaged capture; baseline 1600 exactly
matches the checked-in Kitsch golden. The six native captures use the same three
viewport sizes and frozen source regions.

| Native whole warning RGB L1 reduction | Card 1 | Selected 2 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: | ---: |
| 3840×2160 | 15.0% | 25.9% | 23.0% | 2.0% |
| 1600×900 | 19.4% | 28.5% | 29.1% | 3.7% |
| 1537×947 | 22.9% | 26.9% | 27.8% | 5.0% |

The native fourth-card left side worsens **22.0%, 19.4% and 14.1%**. Its left-foot
region also worsens at every size: RGB L1 increases 494/42/163, with three/one/one
changed pixels. The bridge, frame and exterior controls remain exact; all 412/104/93
changed native pixels are confined to the four warning boxes. Astra independently
checks the build inputs and image hashes, scores all 168 native regions, and reviews
the source/SVG/native crops.

A supplementary check applies the unchanged EQ profile method to both native
captures on the same 52 source-matched rows. Median absolute contrast-width error
falls from 1.897 to 0.208 physical pixels, improving 47 of 52 rows. Astra
independently reproduces all 112 native profiles, their blank controls and
crossings. This helps identify why whole marks improve, but does not resolve the
location and ink differences: narrower contrast alone is not sufficient source
fidelity. It cannot override the fixed-region losses.

**The direct hollow-only trial is rejected for production.** Strong selected and
whole-mark gains do not outweigh the repeated fourth-card side and foot losses. No
smaller offset, card-specific compensation or new foot curve is silently
substituted. A next placement study must use controls independent of the fitted
triangle, such as its neighboring frame and exclamation stem, with complete paired
support. It must distinguish contrast displacement from an actual asset translation
before another geometric correction.

EU supplies those independent placement controls at the original 4K resolution.
The frame's left straight edge and exclamation's central stem use frozen windows
that exclude the triangle. All 48 frame rows and 16 of 20 stem rows have complete
paired crossings. Four source-invalid stem rows are discarded from both comparisons.
Global midpoint differences include the true +0.4-physical-pixel source phase on
cards 3 and 4.

| Source minus native horizontal midpoint, physical pixels | Card 1 | Selected 2 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: | ---: |
| Left frame | +0.940 | −0.187 | +0.828 | +1.401 |
| Exclamation stem | +1.240 | +0.204 | +1.036 | +1.531 |

Astra independently checks 5,704 samples, 136 profiles and 680 contrast cases.
The signs persist under ±2 local blank deviations. These controls corroborate
card-dependent horizontal contrast offsets, without proving an exact shared asset
transform. An initial analyzer variable-shadowing typo stopped before loading
pixels; the corrected analyzer was refrozen without changing bounds or thresholds.
The rejected version remains in scratch.

This supports one next bounded hypothesis: combine the same hollow with an ordinary
warning/exclamation x offset derived only from card 1's two independent controls.
The selected mark keeps its current position, which its independent controls bracket.
The frame and other lettering stay fixed. Structural no-op, all-card and smaller-size
comparisons must pass before a native port; no such placement correction is yet
integrated. EU evidence is in `/tmp/cp-eras-next/eu-kitsch-placement-controls/`.

The scratch Nix shell emitted a missing-font-generator warning from its old staging
path. The trial uses already staged font bytes, and guards verify all fifteen
font-directory files unchanged in the live tree and both copies before and after
building. No font regeneration is claimed from that shell entry. This warning does
not invalidate the exact baseline captures.

Evidence lives in `/tmp/cp-eras-next/eq-kitsch-complete-profiles/`,
`er-kitsch-hollow/`, `es-kitsch-native-baseline/` and `et-kitsch-native-hollow/`
under the same scratch root; the root verification records include the geometry,
profiles, independent render replays and native scores. Only documentation is
integrated. Current SVGs, runtime code, fonts, originals and goldens remain
unchanged. EM remains the verified runtime checkpoint with 305 packaged Rust tests,
22 repository checks, 28 first-attempt matrix cases and exact fresh packaged 4K
parity; its production foreground CPU improvement is 20.5%. A full rebuild is not
repeated for these prose-only changes. K8 and all 29 broader scopes remain open.
