# Reference SVG round fifty-two — ground transfer and font provenance

DM/DN jointly examine the Kitsch rose colors and geometry, then test the
left wash while keeping that rose fixed. Both improve exposed-ground
controls, with remaining channel and local losses. The Neomil Regular
outline substitution is rejected. Source metadata supplies a bounded font
candidate list, but neither embedded fonts nor a per-label assignment.
No broad acceptance scope closes on these diagnostic results.

## Kitsch rose and wash

The frozen DK grid retains 438 candidate patches from three original
photos, with checkerboard training and held-out positions. DM fits four
rose ellipse parameters and twelve RGB stop channels on the same 111
training patches used by DL. The wash, terminal page color, material and
stop positions remain fixed. Current-geometry and DL-geometry starts
converge to effectively the same result without active bounds. The chosen
training soft-L1 cost is 1232.50657282.

The fitted rose center is (761.365971, −195.545754), radii
(1460.529753, 801.844347). RGB stops at .2/.4/.6/.8 are approximately
(178.598,78.468,98.173), (162.637,67.377,89.586),
(129.329,41.890,69.359), and (13.651,10.019,10.507).
Stop zero equals .2; the terminal color remains (12,12,11). Actual 4K
SVG renders, including rounded-versus-floating color parity, are checked.

| Actual ground-only control | Current RGB MAE | DM RGB MAE |
| --- | ---: | ---: |
| All 1,149 eligible photo observations | 2.5621 | 2.1120 |
| 573 checkerboard held-out observations | 2.5507 | 2.1024 |
| 283 held-out observations in the fitted domain | 3.3567 | 2.3533 |
| Left region | 4.5791 | 4.8578 |

Most large DM regressions overlap the unchanged wash: 18 of 22 photo
observations worsening by more than three RGB levels. Rose-only controls
improve substantially, but four large rose-only losses remain. These
results support testing the wash separately; they do not establish an
original authoring recipe or justify changing the card material.

DN freezes that rose and fits only the existing wash's center y, two
radii and twelve RGB stop channels. Its center x, four stop positions,
alpha values, rectangle and layer order stay fixed. Ninety-five DK
checkerboard training patches with x≤750 are selected by geometry,
without residual filtering. One declared initialization converges in
15 evaluations with no active bounds and cost 1281.58251053. Its center
y is 384.404170 and radii are (695.636336,343.283794). The exact RGB
parameters and actual gradient definition remain in the frozen evidence.

| Actual ground-only control | Current | DM | DN |
| --- | ---: | ---: | ---: |
| 255 held-out photo observations at x≤750 | 3.2133 | 3.3501 | 2.8345 |
| All 1,149 observations | 2.5621 | 2.1120 | 1.8067 |
| Left green-channel MAE | 5.7417 | 6.3005 | 6.6399 |

DN improves 80 in-domain held-out observations and worsens 41; six worsen
by more than three levels, with a worst increase of 4.2344. External
x>750 controls are pixel-identical to DM. The 343-position nontraining
aggregate includes those unchanged external controls and must not be
described as an independent in-domain wash holdout. Both left and upper
green-channel errors rise despite lower overall RGB error. The largest
DM wash-overlap losses improve, while some upper-left patches worsen.

Astra independently reproduces both fits' training costs, all 1,149
actual source-patch observations per candidate, split summaries, gradient
geometry and substitution locality. DM changes only the rose definition;
DN changes only the wash definition. Source/current/candidate context
inspection retains the local losses. No new noise or material layer is
introduced by either experiment.

DO transfers the frozen DM+DN definitions to all three complete SVGs,
without fitting anything else. Reversing those two substitutions restores
every source SVG byte, including the six accepted nearest-card gradients
and their uses. Nine current/no-op renders match exactly.

| Whole-scene RGB MAE, current → candidate | 4K | 1600 | 1537×947 content |
| --- | --- | --- | --- |
| Dashboard | 6.5985 → 5.9179 | 6.0060 → 5.3870 | 6.4730 → 5.8161 |
| Mail | 9.0842 → 8.5535 | 8.4114 → 7.9315 | 8.8050 → 8.3261 |
| Store | 8.8008 → 8.3722 | 8.0587 → 7.6734 | 8.5050 → 8.0893 |

The initial root-only fractional viewport exposed the 929-design-pixel
bloom below the 900-pixel scene. That preliminary render is excluded.
The corrected nested SVG clips at design y=900, with transparent padding
from row 865 onward; source registration uses the established top-left
inverse-affine Bicubic transform. Candidate/current alpha is exact at
all sizes. Full-scene, guarded-ground and foreground-support scores are
reproduced independently; the support partition describes modeled SVG
ownership, not a segmentation of the photograph.

The composed DK controls retain RGB MAE 2.5621→1.8067, but 188 photo
observations worsen and thirteen worsen by more than three levels.
The worst patch, at design (30,150), rises 4.4525 levels. In the frozen
Dashboard CX interior masks, Weapons and right Products improve, while
Vehicles depth 2 worsens 5.1589→5.5987 and Locations depth 2 worsens
6.5206→6.7173. Green/blue losses remain in other interiors even where
RGB improves. Broad context review also shows the altered upper-left
rose/wash balance. Keep this promising ground candidate in scratch:
local wash and composed-card losses need resolution before a native
port. Do not retune the accepted nearest-card material automatically.

## Neomil notice outlines

The exact editable Pango replay now separates glyph outlines from their
advances. An isolated-glyph audit finds equal 12-row source/BJ CLASS cap
support at 4K R155, but greater BJ coverage: source areas 218/195 versus
260/244 on the two inactive cards. Moving A and the last S using DL's
C-relative offsets preserves individual glyph areas and still leaves A
touching the first S. A whole-word +2-pixel shift would align C's left
edge while overshooting other right edges. Source threshold segments are
observed ink runs, not proof of original font glyph boundaries.

DM substitutes Regular outlines while preserving every original Bold
glyph advance, offset, cluster, baseline, size and tracking value. It
uses actual Pango font mappings, with no fitted spacing, opacity, scale
or registration. The Bold no-op matches the frozen alpha across both
entire first-line bands. Second lines, exterior pixels and composited
image alpha remain exact.

The four-line RGB error sum improves only 1,797,457→1,781,733 at 4K and
237,328→236,663 at 1600. Missing source R210 core pixels instead rise
2,191→2,941 and 42→62; all first-line R155 overlaps worsen. CLASS remains
worse than current native printing, and the smaller card-three visible
endpoint falls short despite fixed advances. Astra reproduces 13,608
regional numeric checks and reviews enlarged original/current/candidate
crops. Reject this unfitted substitution; no Login font or renderer is
changed. The audit also reproduces 576 isolated profile/support checks.

## Alternate Kitsch source and embedded metadata

The [annotated Kitsch source #46](https://mir-s3-cdn-cf.behance.net/project_modules/source/227dc1118663901.60e5fa669ad54.png)
is a 3840×1800 export of the same hub pictured in source #49. Three
predeclared feature centers fit one uniform scale and translation:
main = 1.1790931913 × alternate + (36.8745,24.0065). Five held-out
features have RMS residual 2.00 original pixels and maximum 3.56.
Astra independently reproduces all eight feature matches, the transform,
aligned raster and comparisons on 115,646 frozen CX source pixels.
The small channel shifts and highly correlated material do not provide
an independent ground or noise view. No alternate is added to the
sixteen main originals.

Fifteen main originals and the alternate contain XMP naming Photoshop
21.2 on Windows. No actual ICC, gAMA or sRGB PNG chunks are present;
a textual profile label is not an embedded profile. Shared document
identifiers and inherited text records cannot establish per-screen
font or layer ownership.

Kitsch Mail, Neo-kitsch Dashboard and Neo-kitsch Mail contain the same
five Extensis FontSense document records:

| PostScript name | Recorded version |
| --- | --- |
| Exo2-Light | 1.001 |
| GothamC2Text | 1.200 |
| MyriadPro-Regular | 2.007 |
| Rajdhani-Medium | 1.201 |
| Rajdhani-Regular | 1.201 |

These records contain names and opaque checksums, not embedded font
files. Their checksum algorithm is unverified and is not equated with
SHA-256. No Neomil original has a font record, and no per-label font
assignment is present. Astra independently parses the source packets and
confirms the names and versions. There is no public asset URL or layered
document path. Treat the list as candidates for bounded source-region
comparisons in the three images carrying it; do not identify the Neomil
notice font from inherited metadata.

The local package and active Fontconfig contain Rajdhani Medium/Regular
with matching PostScript names and version 1.201. This is version-level
agreement, not proof of identical source font bytes. Exo 2 Light, Gotham
C2 Text and Myriad Pro are absent from those inspected local locations.
The ancestor lists also repeat two bare JPEG names across all XMP-bearing
images; they supply neither an asset location nor visible-ground ownership.

## Evidence and verification boundary

Scratch evidence is under `/tmp/cp-eras-next/`: `dm-kitsch-joint-ground`,
`dm-neomil-class-shape`, `dm-neomil-regular-advances`,
`dn-kitsch-wash-fit`, `dn-kitsch-alternate-source` and
`do-kitsch-ground-scene`, `do-source-metadata` and `do-font-availability`.
Independent reviews are in `dm-root-review`
and `dn-root-review`; `dn-evidence-index` records the frozen inputs.

This round preserves the staged implementation, all sixteen originals,
repository HEADs and lock files. CZ remains the latest complete runtime
validation: 295 Rust tests, full repository checks and packaged capture.
Prose-only updates do not require repeating those unchanged builds.
No change is committed, pushed or deployed.
