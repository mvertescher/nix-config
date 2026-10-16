# Round 78 — ordinary Neomil Store compliance baselines

The two ordinary CC35 lines were about six to seven physical pixels above
the original at 4K in both SVG and Iced. This correction moves only their
local baselines from 492/500 to 494.5/502.5. It covers ordinary cards 1/3,
the inferred ordinary card 2 and the fourth card's existing clipped text.
Selected captions retain their separate positions and wording.

Fonts, size, ink, horizontal placement, advances, line spacing, renderer,
cache implementation and primitive counts are unchanged. This is an authored
placement correction, not a replacement of the shared font-baseline
calibration or a claim to have identified the source typeface. No new
text effect, font measurement or rendering pass is introduced.

## Source and native review

JV compares the original, a fresh SVG and the verified round-77 package.
The source's complete first/second lines span 608/442 physical pixels on
cards 1/3, versus roughly 527/386 native. Both source copies and the visible
fourth-card caps independently confirm the lower placement. Raw RGB and
five-threshold row/column profiles are retained; differing inks make those
threshold widths descriptive rather than exact font metrics.

JW freezes one +2.5-design-y SVG trial; JX transfers the same four native
coordinate changes. At 3840×2160, 1600×900 and 1537×947, all 54 complete-line
all/bright/dark RGB L1 and RMS comparisons improve in each renderer.
Astra independently recomputes all 972 subset records per renderer and
reviews all eighteen source/baseline/candidate contacts. Native whole-context
RGB L1 changes are:

| Size | Card 1 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: |
| 3840×2160 | 1,832,373 → 1,576,848 | 1,934,109 → 1,670,350 | 1,139,698 → 953,889 |
| 1600×900 | 300,821 → 230,005 | 317,844 → 241,604 | 187,496 → 137,888 |
| 1537×947 | 271,692 → 195,008 | 290,354 → 209,864 | 173,541 → 116,360 |

Local losses remain: 67 SVG and 93 native subset/band/gap records worsen
in L1 or RMS. Native card 3's 4K first-line dark band 7 loses 17,754 L1;
card 1's second-line source-profile gap at x1398–1405 loses 264 all-pixel
and 443 dark L1. These are retained alongside complete-line gains, not
cropped away. Profile gaps are empty strong-red source columns, not
assumed semantic word boundaries. Line-union boxes overlap and their
errors are not summed as disjoint data.

The initial fractional SVG harness used centered viewport alignment; its
outputs remain separately marked invalid. The reviewed trial uses direct
top-left viewport renders and repository-sampler source images with saved
metadata. The native baseline reuses actual round-77 package captures
after source, binary and capture provenance checks. All 266 library tests
pass. Native changes affect 16,752/4,482/5,026 pixels at the three sizes;
alpha and every pixel outside the caption contexts remain exact.

## Integration verification

All sixteen targeted headless state cases pass, including two partial
caption reveals, custom colors and the clipped fourth card. Selected
captions, exterior pixels and alpha remain exact. The disabled wrapper
reproduces the actual Store before/after frames. The source SVG, ordinary
component specimen and Store golden follow the reviewed correction; only
4,482 golden pixels change. Two component sizes change only the ordinary
caption. All 311 packaged Rust tests, 22 repository checks and 28
first-attempt matrix cases pass. Twenty-six of the 27 visual images are
pixel-exact against their goldens; the existing single-channel, one-level
Neo-kitsch bar pixel remains. The other matrix case verifies source
sampling. Fresh actual-package captures exactly match the reviewed native
candidate at all three sizes, and both Store fidelity gates pass.
Changes remain staged and uncommitted.

The substantial width, horizontal placement, glyph-contour, ink and faint
printing differences remain in the [Store printing task](../todo/neomil-store.md)
and [compliance study](neomil/store-typography.md#ordinary-compliance-lines--jv).
No live desktop or performance improvement is claimed.

Frozen evidence is under `/tmp/cp-eras-next/`: `jv-compliance-source/`,
`jw-compliance-registration/`, `jx-compliance-native/`,
`jy-compliance-states/`, `jz-compliance-integration/` and `ht-echo-review/`.
