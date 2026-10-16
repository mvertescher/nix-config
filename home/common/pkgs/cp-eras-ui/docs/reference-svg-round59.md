# Reference SVG round fifty-nine — footer coverage and source continuity

EF corrects excess Neomil Dashboard lower-rim coverage and studies
Neo-kitsch lower-fan correspondence and the Neomil plaque boundary.
Sol lanes prepare bounded
scratch studies; Astra reproduces the evidence and owns acceptance.
The completed font/performance work and EE sampler remain intact.

## Neomil footer lower coverage

The accepted AZ band ends at design y888.75, but the original primary
rim continues to y889. At 4K, the source's straight bottom row y2133 has
mean red26.5, versus SVG144 and native175. The preceding source rows
y2130–2132 are red251; the accepted band must remain. This is a local
coverage discrepancy, not evidence for globally shrinking the frame.

One SVG trial keeps the stroked rectangle and changes its paint below
y888.75 to .15 opacity. Its unchanged-opacity control reproduces all
three complete frames exactly. The candidate improves every one of
349 changed 4K pixels, all on row2133, with unchanged alpha. Direct1600
is exact. At 1537×947, eight blue-channel values on the upper-left side
change by one level and improve source distance. They are collateral
full-scene paint effects, not lower-edge changes; an isolated rectangle
does not reproduce them. The compositor stage responsible is unresolved.
Astra rerenders all nine SVGs and reproduces 360 regional scalar checks.

Fresh native baselines at all three sizes match the accepted CF captures
exactly. A native gradient keeps the original compound path, semantic
foreground and paint order, but fails unchanged-opacity replay: it changes
817/74/104 pixels at 4K/1600/fractional size, including source losses.
The WGPU gradient shader quantizes stops and adds dithering even for a
constant gradient. Isolating attenuation from this paint-route change
improves 349 bottom-row pixels at 4K; both smaller frames then tie.
This does not justify accepting the unrelated gradient changes.

Local canvas clipping alone would reorder the rim beneath earlier echoes:
Iced pastes child meshes before its parent frame's buffered geometry.
A three-geometry clipping probe preserves order and passes exact replay.
It improves all 349 changed 4K pixels and leaves both smaller sizes exact,
but adds unnecessary drawing batches for this small correction.

The simpler solid-path split also passes exact replay at all three sizes.
The upper compound path keeps the accepted left trim and inner opening;
a complementary lower rectangle receives .15 semantic foreground opacity.
Both remain in the original frame and paint order. All 349/145/140 changed
pixels at 4K/1600/fractional size improve source RGB distance, with exact
alpha and no exterior changes. Its native area coverage differs from the
SVG gradient's sampled paint: the smaller native bottom row changes from
red239 to211, closer to source202/205. At 4K, the straight row changes
175→30 against source26.5. Astra independently passes all 240 regional
RGB/threshold checks across both native alternatives and views the source,
SVG and native crops. The integration changes only the Neomil table and
matching SVG/specimen. It adds one small solid rectangle through existing
primitives, without a new shader, clip, cache or shared renderer change.
No new CPU/GPU timing claim is made for this bounded geometry correction.

Seven actual-Dashboard pairs pass independent review: rest, custom
foreground, .15s opening, selection with synthetic held feedback,
no-config fallback, Kitsch control and fractional custom foreground.
The five Neomil 4K pairs each change only the same 349 bottom-row pixels;
fractional custom changes 140, and Kitsch is exact. Opening preserves
the rest footer delta. Selection/held and fallback frames match their
rest counterparts; the synthetic held case does not establish distinct
visible feedback or live input behavior. Custom cyan remains semantic:
one lower-rim sample changes (35,175,162)→(6,40,37). Alpha is exact.

The component's unchanged-opacity control is exact at full and half
size. The candidate is unchanged at full size and changes only 73 footer
pixels at half size, with exact alpha and captions. Astra reproduces
all six renders. These are specimen checks, not independent source fits.

## Neo-kitsch lower-fan correspondence

Pairwise ordered assignment does not establish a ridge family. Twenty-one
of 22 previous left entries anchor to peaks at x2825; the other is three
pixels away. Five fixed cost models retain only nine common nonempty
assignments at x2830, three at x2835 and none by x2850. Counting common
missing values as stable matches would conceal that failure.

The known left1493 ridge exposes a concrete solver error. Its correct
next chord follows strong normalized source response throughout, while
the ordered assignment selects a weaker neighboring ridge to accommodate
other entries. Later tracker loss does not imply a photographed endpoint.
An orthogonal cross-section probe follows a visible turn from
(2911,1450) to (2929,1410), with strong response along every sampled chord.
That turn is a separate ridge from the known left1493 segment: six detected
peaks lie between them at x2911. A continuation probe instead points toward
a farther-right turn near x2950, but its normalized response weakens across
x2925–2935. This method does not confirm that join; the threshold failure
does not prove photographic occlusion or a physical endpoint. Astra
reproduces all four diagnostic outputs, checks ordering and uniqueness,
recounts the intervening peaks, recomputes chord samples and inspects the
source overlay. No truncated strand or inferred family is ported.

## Neomil plaque lower boundary

The frozen upper frequency and phase are tested across 55 source rows
and 48 overlapping eight-row windows, using original Store-minus-Login
red values. The phase correlation varies substantially across the lower
junction: −.021/.084/.444 in three adjacent windows near y1188, and .062
in the right window near y1192. Primary letters, hatch and horizontal
printing remain mixed in these source pixels.

This loss of stationarity does not establish a physical cutoff or
occlusion. The donor identifies ground, not the material beneath opaque
letters. A continuous visible lower stencil or separate layer/exposure
constraint is still needed; an arbitrary cutoff or per-pixel knockout
is unsupported. Astra reproduces the data and reviews the source crop.
No new candidate is made.

## Evidence and current state

Evidence is retained under `/tmp/cp-eras-next/ef-root-review/`, including
`footer-verification.json`, `native-gradient-comparison.json`,
`native-solid-verification.json`, `state-verification.json`,
`component-verification.json`, `veneer-verification.json`,
`veneer-connection-verification.json` and `plaque-verification.json`.
Worker scripts and frozen inputs are in the sibling `ef-footer-bottom`,
`ef-native-rim-probe`, `ef-veneer-order` and `ef-plaque-boundary` directories.
All 303 local and packaged Rust tests, both Dashboard fidelity gates and
24 SVG structural checks pass. Three production sizes and the fallback
capture match the reviewed candidate exactly. The first local debug 4K
capture at four seconds was entirely black and failed parity; retaining
that evidence and waiting fifteen seconds produces the correct frame.
Only the two intended 1600 Dashboard goldens change, by 145 pixels each.

All 22 repository checks and 28 matrix cases pass on their first attempt:
27 visual cases plus the cached nine-test sampler case. Twenty-six visual
cases are RGBA-exact. The Neo-kitsch bar's one-level pixel difference
matches its prior DZ render exactly. Nix fetches all 340 frozen tracked
files without drift. The new package is
`/nix/store/39qpw8g5qmvi2v2j8sxrg5sbn8k4a1a8-cp-eras-ui-0.0.0`;
its fresh 4K Dashboard capture is byte- and RGBA-exact against the reviewed
candidate. `matrix-verification.json`, `production-parity.json`,
`package-parity.json` and `full-check.log` retain the integrated evidence.

All sixteen original images, accepted fonts and earlier cache
implementations remain unchanged. The bounded lower-rim correction is
complete; broader footer printing and all 29 existing TODO scopes remain
open. The final documentation updates are prose only. Changes stay staged
and uncommitted; no live desktop or hardware-GPU verification is claimed.
