# Reference SVG round fifty-three — coupled ground and Exo evidence

The coupled Kitsch ground fit improves exposed-ground averages but fails
composed-card controls. The recovered Exo2-Light font provides a reproducible
candidate; its fixed-size substitution loses bright lettering, especially
at 1600. Neither trial is integrated. CZ remains the accepted runtime
checkpoint, and all 29 broader acceptance scopes remain open.

## Coupled Kitsch ground

DP fits the existing rose and wash together: 31 geometry/color parameters,
unchanged stop positions, alpha, page color and layer order. All 220 DK
checkerboard training positions contribute their eligible photo means;
each prediction averages the original 24×24 pixel patch. Exactly two
bounded soft-L1 fits start from current and frozen DM+DN parameters.
Their costs are 1143.135514 and 1148.014746, with no active bounds. The
lower training cost selects the candidate before reserved controls are read.
The 218 reserved positions have been reused in earlier rounds and are
diagnostics, not fresh unseen validation.

| Actual ground-only RGB MAE | Current | Sequential DM+DN | Coupled DP |
| --- | ---: | ---: | ---: |
| All 1,149 eligible photo observations | 2.5621 | 1.8067 | 1.4412 |
| 576 training observations | 2.5734 | 1.7407 | 1.3658 |
| 573 reserved observations | 2.5507 | 1.8730 | 1.5171 |
| Left region | 4.5791 | 4.2524 | 2.7091 |

DP improves green/blue error over DM+DN but worsens red. Against current,
129 observations worsen, fifteen by more than three RGB levels; the worst
increase is 5.7506. The upper-left improves while other wash and right-edge
losses remain. Two distinct local solutions, with Jacobian condition
numbers about 112 and 104, do not identify the original authoring recipe.
Astra independently checks training costs, selection, all 1,149 source
patches, actual SVG samples and the two-gradient-only substitution.

DQ transfers those exact definitions through Dashboard, Mail and Store,
preserving every card, material and text parameter. All nine complete
scenes improve over current in average RGB error but score worse than
sequential DM+DN. At 4K:

| Whole-scene RGB MAE | Current | Sequential | Coupled |
| --- | ---: | ---: | ---: |
| Dashboard | 6.59851 | 5.91785 | 5.99209 |
| Mail | 9.08415 | 8.55353 | 8.69961 |
| Store | 8.80082 | 8.37216 | 8.48842 |

The frozen CX card interiors reject a native port of this exact candidate.
Vehicles depth 2 worsens 5.15888→6.50004 and depth 3 worsens
5.33734→7.90625. Of 19,196 depth-3 pixels, 18,717 worsen and 5,117
worsen by more than three levels. Locations depth 2 also worsens slightly;
right Products improves. A lower exposed-ground mean cannot establish
the ground hidden under photographed cards or the correctness of their
reconstructed translucent material. Preserve the accepted nearest-card
material; do not compensate it automatically for a failed ground fit.

Astra reproduces 35 whole/content/partition/interior metric groups and
all 1,149 composed source-patch observations. Reversing the substitutions
recovers the three SVGs exactly, including accepted nearest-card material.
Nine current/no-op rasters and all candidate alpha planes match. The
1537×947 render uses the nested 1600×900 design viewport, transparent
padding from row 865, and the established inverse-affine Bicubic source
registration. The 1600 reference uses whole-photo Lanczos.

DR tests the current SVG's compositing response with two fixed controls:
only the three ground rectangle fills become uniform black or white.
Their difference divided by 255 predicts how a ground change propagates
through the unchanged overlay, without fitting to either candidate or
source photo. Astra independently rerenders both controls exactly and
reproduces 32 statistic groups on all 38,043 frozen Vehicles pixels.

At depth 3, DP's ground shift averages (+12.099,−2.854,+3.969) RGB.
The fixed response predicts a full-scene shift (+8.386,−1.956,+2.775),
against actual (+8.503,−1.922,+2.739); 96.3% of pixels agree within
one level in all channels. Mean red render-minus-photo error rises from
+2.198 to +10.701. The regression extends across the fixed spatial bins,
not just an edge. This explains the rendered loss without a changed
card-material hypothesis; it still does not identify the photograph's
hidden ground or original alpha. A new proposal needs source-local
ground evidence and composed-card controls together.

## Version-matching font, unsuccessful substitution

The official [Google Fonts Exo2-Light file at commit 90abd17b](https://github.com/google/fonts/blob/90abd17b4f97671435798b6147b698aa9087612f/ofl/exo2/Exo2-Light.ttf)
has PostScript name `Exo2-Light`, version 1.001, weight 300 and nominal
cap height 690/1000. Its SHA-256 is
`fd2d399ec9df6b99bbd8cf7a6c4c9c521d91751646479e8e4e67b73cc0a17349`.
The [same-commit OFL license](https://github.com/google/fonts/blob/90abd17b4f97671435798b6147b698aa9087612f/ofl/exo2/OFL.txt)
is pinned with it. Astra independently retrieves both files and verifies
the font metadata and Git blob. This matches the document record's name
and version, not its unknown-checksum binary identity or any visible
label assignment. No font is added to the package.

The single Dashboard trial changes only the seven module labels' group:
Rajdhani size 17 becomes Exo 2 weight 300 at size 15.8405797101449275.
Anchors, baselines, tracking, ink and filters remain fixed. Astra finds an
arithmetic error in the method: `17*643/690` is 15.842028985507246, not
the rendered value. The difference is −0.0014492753623187582 design
pixels. Preserve the frozen trial and reject only that actual candidate;
do not call it exact cap matching or assume a corrected rerender is equal.

| Seven fixed regions | Source R160 gold area | Current → Exo area | R160 F1 | Source R210 misses |
| --- | ---: | ---: | ---: | ---: |
| 4K | 10,496 | 5,491 → 2,603 | .2288 → .2263 | 6,541 → 6,847 |
| 1600 | 1,912 | 847 → 31 | .2254 → .0257 | 1,103 → 1,134 |

RGB L1 falls, but bright source strokes disappear. Aggregate F1 worsens
at all four predeclared thresholds in both fixed and expanded regions;
none of the seven labels improves F1 at 1600. Changed glow extends beyond
the fixed boxes and is included in the expanded controls. Some 4K left
edges move closer, which does not resolve cap phase, counters or ink.
Fewer false pixels in source gaps cannot establish spacing when strokes
are also missing. Reject this substitution without ruling out the family.

Astra independently reproduces all fixed/expanded metrics and 32 aggregate
threshold groups. Installing the font and switching between isolated and
established Fontconfig preserves the complete baseline RGBA at both sizes.
The candidate changes 76,663/13,164 RGB pixels at 4K/1600, entirely within
the seven label neighborhoods, with zero alpha or exterior changes.
Source/current/candidate montages confirm the small-size loss.

The next justified font diagnostic is a bounded comparison of distinctive
letter silhouettes, only where source glyphs can be separated from their
background. Predeclare source windows and registration, retain other
labels and the 1600 projection as controls, and separate shape from glow.
This is a proposed diagnostic, not font identification or an accepted fit.

## Queue and verification boundary

The independent queue audit confirms every unchecked file/line reference:
17 source-fidelity scopes, six caller/design prerequisites, two original
asset/provenance scopes and four live verification scopes. Accepted
Entropism M, Neomil lower-A2 slit and CZ stroke ownership are not dispatched
again as independent fixes. Source work remains actionable where local
evidence supports a bounded correction.

Scratch evidence is under `/tmp/cp-eras-next/`: `dp-kitsch-coupled-ground`,
`dq-kitsch-coupled-scenes`, `dp-exo2-source`, `dp-exo2-module-trial`,
`dr-exo-next-evidence`, `dr-kitsch-vehicles-attribution` and
`dr-queue-audit`. Astra reviews are in
`dp-root-review` and `dr-root-review`. The failed trials stay in scratch.
This round changes only prose; runtime, SVGs, fonts, goldens, scripts,
lock files and all sixteen originals remain unchanged. CZ's 295 Rust
tests, full repository checks and packaged capture remain the latest
runtime validation. Changes stay staged and uncommitted.
