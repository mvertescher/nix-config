# Reference SVG round sixty — current text and echo controls

EG rechecks Neo-kitsch Dashboard footer text, transfers the frozen AY
Neomil echo profile to the corrected EF rim, and examines the weak
Neo-kitsch veneer bridge in original RGB. Sol lanes prepare scratch
studies; Astra independently verifies the evidence. No new artwork,
font, renderer or golden changes are accepted.

## Neo-kitsch footer text

Three fresh current-package captures match the accepted AX frames exactly;
1600 also matches the current golden. The shared source sampler and an
explicit top-left fractional SVG viewport supersede AX's separately
projected line anchors. At R>70 the native first lines sit one row below
source at 4K and one row above at 1537×947. At 1600 source includes a
faint extra top row; both smaller second lines match source row extents.
The earlier two-row fractional discrepancy was a sampling error.
Thresholded printing extents do not alone establish a font-baseline error.

The direct SVG behaves differently: its 4K first-line R>100 and fractional
R>70 extents match source, while its 1600 bright core nearly vanishes.
Native R>70 coverage is 61–63% of source at 4K, 75–80% at 1600 and
75–83% at fractional size. The photographed C1 red peak is 253/209/213
across those sizes, versus native 169/143/137. This confirms a printing
residual but does not isolate ink from photographic glow, softness and
raster phase. A shared baseline or weight change is not supported.

Fontconfig selects the bundled Rajdhani Regular file, SHA256
`6e1fc228a8318251a6e569502ec57bac1e4656c582f92f59ccecc4688e039b98`.
Native selectors use the same face verified by the completed font audit.
That establishes correct implementation of the intended font, not the
photographed font's identity. Astra reproduces all three whole-photo
projections and direct SVGs, checks 240 threshold/overlap measurement
groups, verifies native provenance and inspects source/SVG/native crops.
The accepted strings, font bytes, positions, sizes and ink remain intact.

## Neomil secondary echoes

The early lower echo remains too dim while its later tail is too bright.
On the frozen straight training interval, source red values for rows
2133–2139 are 26.5/83.1/109.1/85.0/63.7/66.0/78.7; current EF native
gives 30/59/68/74/79/79/79. An independent straight interval reproduces
the source profile within 0.3 levels. Raising both frame-copy opacities
cannot resolve this profile.

AY's seven frozen horizontal-row opacities are transferred without a
refit to the current SVG, preserving EF's corrected primary rim. A
complementary clip split changes 33/0/14 pixels at 4K/1600/fractional
size before any intended profile change. Keeping each original echo's
opacity on a group changes 35/24/764 pixels instead. Neither is an exact
replay; their artifacts are retained explicitly rather than attributed to
the proposed profile.

One bounded candidate is nevertheless measured against both the actual
current SVG and the split control, with separate seam windows. It changes
5627/1099/1060 pixels with exact alpha. At 4K the whole-footer RGB MAE
improves 10.2373→10.0422, but the text-boundary crop worsens
17.8634→18.0556 with 198 pixel losses. At 1600 the whole-footer MAE
worsens 8.0624→8.2278; fractional worsens 8.2002→8.2987. Interior bottom
holdouts and text also lose at the smaller sizes, away from clip seams.
Comparison with the split control preserves this conclusion. The
corrected primary rim therefore does not rescue AY's frozen profile.
This trial is rejected; no native port or new fit follows.

Astra rerenders all nine candidate/control SVGs, reproduces all three
source projections and independently checks the regional RGB, threshold,
alpha and changed-bound measurements. Any future echo model needs a
source-supported spatial profile that transfers through small-size
interior and text controls; repeating this frozen transfer or raising
global opacity is unsupported.

## Neo-kitsch veneer

The proposed connection toward the visible turn near (2950,1464) still
crosses an ambiguous low-contrast region. Applying the existing Gaussian
scales independently to original R/G/B gives response
(-0.26,0.85,3.42) at (2930,1483), versus (9.04,6.23,6.68) on the known
left ridge at (2920,1485). Green agrees with the saved normalized field
to floating-point precision: mask normalization is not suppressing this
interior point. The route stays at least 37 pixels from the panel boundary.

A separate transverse contrast operator fails two known left-ridge
controls, so its negative bridge response cannot establish absence.
Nearby parallel peaks suggest a local order but do not identify which
ridge reaches the right turn. Weak contrast is not evidence of a physical
endpoint or occlusion. Astra independently recomputes all nine fixed RGB
samples on a larger crop and inspects the source overlay. No curve or
truncated strand is ported.

## Evidence and verification state

Scratch evidence is under `/tmp/cp-eras-next/eg-root-review/` and sibling
`eg-neokitsch-footer`, `eg-footer-echo` and `eg-veneer-response` directories.
`independent-verification.json`, `footer-verification.json` and
`neokitsch-native-manifest.json` retain the root checks and capture
provenance. The current package remains
`/nix/store/39qpw8g5qmvi2v2j8sxrg5sbn8k4a1a8-cp-eras-ui-0.0.0`.

All 340 tracked crate files matched the EF checkpoint before this round's
prose edits. Runtime, fonts, SVGs, goldens, sampler, build inputs and all
sixteen originals remain unchanged. EF's 303 packaged Rust tests,
22 repository checks and 28 matrix cases remain the verification for
those unchanged inputs; this documentation-only round does not claim a
new full-check run. New headless captures establish current Neo-kitsch
parity, not live desktop or hardware-GPU behavior. All 29 broader TODO
scopes remain open. Work stays staged and uncommitted.
