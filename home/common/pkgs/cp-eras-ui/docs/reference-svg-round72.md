# Reference SVG round seventy-two — ordinary Store left coverage

The ordinary Neomil Store left edges now use width 0.6, matching the
accepted tops. Bottom and right edges remain 1.2. This includes the inferred
ordinary second card when another card is selected. Primary centerlines,
faces, material origins, contents, selected artwork and the fourth cut stay
fixed. First/third held outlines remain hidden; the fourth held outline
remains bright.

## Source and implementation

Repeated source profiles support one bounded half-width hypothesis. A
combined left/bottom trial was rejected because the bottom regions regress
at both smaller sizes. The accepted left-only ablation preserves bottom
width and its continuous corner joins. Only outer and inner left x
coordinates change in the existing filled outlines. It adds no primitives,
effect layers, caches, runtime geometry calculations or font changes, and
makes no timing claim.

All ninety fixed region comparisons improve or remain exact in both RGB L1
and RMS, in each renderer, across 3840×2160, 1600×900 and 1537×947. The table
measures the union of the three left regions, without adding their
overlapping corner regions again.

| Viewport | SVG RGB RMS, before → after | Native RGB RMS, before → after |
| --- | ---: | ---: |
| 3840×2160 | 19.123 → 14.055 | 24.998 → 13.689 |
| 1600×900 | 14.984 → 12.328 | 19.756 → 13.751 |
| 1537×947 | 14.491 → 12.229 | 19.955 → 13.681 |

Native changes 13173 / 2745 / 2638 pixels, confined to left edges and
their corners. Alpha and all pixels outside those supports are exact.
Whole-top/bottom region gains come from their left corners; their straight
runs retain the previous widths. Astra reviews source, SVG and native
straight runs and both corner joins at all three sizes. This improves
primary coverage; it does not recover photographed blur or missing echoes.

The component's plain C1 sample uses the same outline. The sheet has no
separate C3/C4 ordinary samples. It also loses one ignored duplicate raw
attribute line after the face element. An audit of all 24 reference SVGs
finds this as the sole such raw-text instance. Cleanup alone is byte/RGBA
exact at nominal and fractional component sizes, and changes no XML
element, attribute or legitimate text. The face itself is untouched.

## State and repository verification

The fifteen Store interaction tests pass. Seven paired state cases cover
selection of the first/fourth cards, ordinary/fourth hover and held feedback,
and early opening. All changes stay in the expected ordinary left/corner
regions. Selected artwork and the hidden first-card held border stay exact;
the fourth held border remains bright. The initial locality mask omitted
the inferred ordinary second card. Its retained captures pass a corrected
mask translated by the actual 332-unit shelf-origin difference, with
selected and held cards explicitly excluded. No runtime change or tuned
image boundary resolves that test-harness omission.

Three custom-palette cases cover rest, ordinary held and fourth held.
The custom probe disabled at 1600 reproduces the native resting image
byte-for-byte. All state/custom controls preserve contents, cut exterior
and alpha. These are synthetic Pointer snapshots through production
drawing/selection logic, not live desktop input checks.

Both Store fidelity gates pass, with the implementation gate using the
new package. All 22 repository checks and 28 matrix cases pass on their
first attempt; packaged Rust tests pass 260 library and 45 other tests
(305 total). Direct pixel review finds 26 of 27 visual cases exact and
retains the existing one-pixel, one-level Neo-kitsch bar difference. The
other case checks source sampling. All 354 frozen tracked crate files
match the tested Nix source. Fresh packaged captures are byte/RGBA-exact
to the reviewed native candidate at all three sizes. Only the six final
documentation records change after verification. Fonts, originals, text
caches and shared rendering remain intact; changes stay staged/uncommitted.

## Rejected lower-stem follow-up

A separate source-only trial thins the complete lower vertical segment
of C1/C3 to 0.6, keeping bottom width and the bar polygons fixed. All 4K
regions improve, but smaller-size fan regions lose substantially; fractional
C1 lower-stem RMS also worsens. Five local losses remain recorded. That
frame segment contributes to the visible bar-side edge, so retaining the
bar polygon alone does not preserve its combined appearance. The complete
trial is rejected without native work. A later separation needs a supported
bar/stem junction, not an arbitrary cutoff, cap or taper.

A frozen source-only junction study samples eight cross-sections from each
of the first and third cards. All 920 original-size RGB samples match the
source image, and independent recomputation confirms both smaller-size
products and fixed-copy diagnostics. Both copies visibly narrow around the
accepted bar tip at design y413.4. The bar fill hides the underlying frame
contribution, while different ground and red echoes affect the flanks.
This establishes the visible transition but does not identify a separate
stroke-width, cap or taper law. No new candidate follows from these profiles.

Evidence: GR's frozen source ablation; GS v2's scoped native/component
overlay; GT's native controls; GU's independent source and state review;
GV's rejected lower-stem trial; GW's integration and full verification;
and GX's source-only junction study with independent review,
under `/tmp/cp-eras-next/`. Other primary coverage and faint printing remain
open. The total stays at 30 open scopes.
