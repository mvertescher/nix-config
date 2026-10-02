# Reference SVG corrections — round forty (BE)

BE integrates a bounded Kitsch store compliance-font replacement across all four repeated cards and both lines. The earlier BC `ONLY` contour and BD block translation remain rejected. A corrected source audit fixes the `O` cap at 12 native pixels and the second-line anchor, then ranks FreeSans Bold as the closest installed approximation under three fixed contrast thresholds. The source typeface is still unidentified. One full-line SVG trial had repeated line-two losses; a read-only word-start fit led to one calibrated SVG candidate. Its 24 whole-line RGB comparisons improve at three sizes, while seven word RGB losses and 204 overlapping threshold/region loss records remain. The calibrated candidate retains the existing ink and card-4 clipping/faded edge.

Native review exposed two renderer issues before scoring the final candidate. The embedded FreeSans Bold file reports weight 600; asking cosmic-text for 700 selected host DejaVuSans-Bold in the shaped-glyph probe. The corrected 600 selector chooses the exact embedded bytes. The analytic .95 baseline fraction is sound, but cryoglyph rounds its layout offset after truncating the text-area top, placing the earlier trial one raster row high. A FreeSans-only `paint_text` placement correction addresses that phase without changing other faces or shared text construction.

The corrected native frame improves **all 24 whole-line RGB** and **all 72 whole-line F1** comparisons at .35/.50/.65 over 3840×2160, 1600×900 and 1537×947. At 4K, ordinary card-1 line-two MAE falls 33.988→27.939, card-3 line-two 36.749→23.628, clipped/faded card-4 line-two 20.553→17.143 and selected-card line-two 22.920→19.334. The 4K line-one gains are 45.389→26.605, 44.625→23.728, 23.675→15.694 and 32.968→21.017 in the same order. Twelve individual word RGB comparisons still worsen; the largest is fractional card-1 `ACCESS`, 26.031→29.821. The fixed metrics retain 272 overlapping loss records across scored words, holes, blank columns and thresholds; no separate final endpoint test was scored. Native RGBA changes total 32,252 / 7,132 / 6,919 pixels at the three sizes, with zero alpha changes and zero pixels outside the compliance regions. Exact contours, gaps, selected texture and photographic softness remain open.

The integrated SVG shares ordinary-card art and keeps only independent compliance text transforms on cards 3 and 4. Direct full-RGBA comparison to the frozen calibrated candidate is pixel identical at all three sizes, including its top-left fractional viewport. The initial shared-art proposal removed the existing motion clip; the structural audit caught it. Production restores the original clip and animation exactly, passes all 24 SVG ID/reference checks, and retains three-size rendered parity. The Kitsch component sheet matches the ordinary and selected type settings; its only render changes are the two specimens and two captions. Production packaging stages the tested FreeSans Bold file from nixpkgs `freefont_ttf` for both native and development builds. It ships upstream `COPYING`, `README` and `CREDITS` under `share/doc/cp-eras-ui/fonts/freefont/` and retains the embedded metadata. The tested font SHA-256 is `982534a3731416a15e2756601721f26053f68bf4239011550f3dd23ce6308215`; the metadata records GPL version 3 or later and a document-embedding exception.

Paired state review passes **15/15 nonvacuous cases**: three rest sizes and twelve additional fractional cases, including first/last selections, synthetic ordinary/selected hover and held material, custom palette and a partial opening with visible text. Every baseline/candidate pair changes only compliance printing, with zero alpha and zero outside-region pixels. Six rest captures at the three test sizes match their separately frozen baseline/candidate frames over the full image. The synthetic hover/held captures verify drawing, not pointer hit testing. Evidence is `/tmp/cp-eras-next/be-states/locality.json` and `parity-verified.json` beside it.

A separate cap-row audit retains the narrow cap-height task: the 4K O height is corrected on three cards, but ordinary M and some smaller-size profiles still differ.

BE verification passes 293 local and Nix Rust tests, both Kitsch store
fidelity gates, all 24 SVG ID/reference checks and all 22 repository checks.
All 27 visual cases pass on their first attempt: 26 are pixel-identical;
Neo-kitsch bar retains its established one-pixel, one-channel-level residual.
Production store frames at 3840×2160, 1600×900 and 1537×947 match the reviewed
native frames exactly. A fresh packaged 4K store frame also matches exactly.
Only the Kitsch store golden changes, by 7132 pixels. Component differences
stay within the two compliance specimens and two captions at full/half size.
All 314 frozen tracked files match the tested Nix source; nine local
original images remain unchanged. FreeFont COPYING, README and CREDITS in
the package match the pinned upstream source byte for byte. Final documentation
updates preserve that tested runtime/artwork. Changes remain staged and
uncommitted; all 29 broader TODO boxes stay open.

Detailed evidence is in [the typography record](kitsch/typography-fit.md#be-freesans-approximation-for-two-line-store-compliance), and scratch controls remain under `/tmp/cp-eras-next/be-font-audit/`, `be-font-trial/`, `be-calibration-review/`, `be-native-font-diagnosis/`, `be-baseline-review/`, `be-svg-integration/` and `be-states/`.

The next Neo-kitsch NK-05 lead concerns repeated store metadata. A scratch geometry fit improves five of six lines; upper-C line 1 regresses at a stricter threshold. It remains unaccepted and separate from BE.
