# Group opacity audit

The existing Neo-kitsch Dashboard panel fade has a verified overlap
defect. `Scene::paint` multiplies each descendant ink's alpha, while the
adopted SVG animates the opacity of the assembled group. Dark paragraph
printing therefore lets the gold body show through during the fade.
This is independent of source-font identity, photographed softness and
the remaining glyph fits. The earlier limitation in `PIPELINE.md` is
still present in the current package.

## Current-package evidence

MZ captures the published reference Dashboard at 1600×900 with clocks
fixed at 0.4 s (panel hidden), 0.55 s (mid-fade), and 2.4 s (rest).
The fresh rest capture matches the current golden exactly. All captures
use the same packaged binary, palette and font configuration. The fixed
panel aperture is x1140–1429/y235–714; the independently animated cascade
lies to its left and does not overlap it.

For a static assembled group over an unchanged backing, every channel
during a fade must stay between its hidden and visible endpoint values.
This necessary condition holds for both encoded-RGB and linear-light
interpolation, regardless of the chosen easing curve. In the current
native panel, 2,529 of 139,200 pixels exceed that interval by more than
one channel level. All violations are brighter; the largest excess is
55 levels. The fixed paragraph region contains 2,189 of those pixels;
the separate EMAIL label control has none.

At x1319/y347, the actual RGB values are:

| Panel state | RGB |
| --- | --- |
| Hidden, 0.4 s | 13 / 9 / 12 |
| Mid-fade, 0.55 s | 144 / 106 / 61 |
| Fully visible, 2.4 s | 89 / 63 / 37 |

No interpolation of the two endpoints can produce that middle value.
The sampled pixel is evidence of the composition error, not a request to
adjust its font, ink or position. Astra independently reproduces every
fixed-region count and error summary with a separate pixel loop. Actual
headless SVG timeline captures at 0.55 s and rest provide a visual
comparison: the SVG retains assembled-panel text contrast. Existing
SVG/native glyph, grain and named-easing differences remain separate;
whole-screen RMS is not an acceptance gate for this defect.

Plans, binary/source/font hashes, three native captures, two SVG captures,
results and `ROOT-VERIFIED.json` are retained in
`/tmp/cp-eras-next/mz-current-panel-fade/`.

## Implementation route under investigation

Iced's canvas clip drafts geometry; it does not create a composited alpha
layer. The current software compositor excludes text, grain, dots and
viewports, so routing the panel through it would not preserve native
glyph rendering. Ordinary image opacity also requires explicit treatment
of premultiplied alpha and the renderer's linear-light blending. Neither
is an established replacement.

The scoped NB feasibility spike tests cached GPU endpoint textures:
render a fixed backing and that backing plus the complete native group,
then interpolate the encoded endpoint colors in a dedicated GPU pass.
The native mesh/text pipelines supply both endpoints; repeated alpha
frames should reuse them without rerasterizing glyphs. This is a scratch
fixture, not a production fix. Capturing the correct preceding layers,
draw order, clipping, physical scale and state-dependent backing remains
an integration requirement. The read-only API review is in
`/tmp/cp-eras-next/na-group-opacity-route/`; the prototype is in
`/tmp/cp-eras-next/nb-group-fade-prototype/`.

## NB/ND native-text GPU fixture

Both frozen implementations pass on headless Vulkan (Mesa llvmpipe).
The 320×200 fixture contains a nonuniform opaque backing, overlapping
native shapes, a clipped strip and native Rajdhani Regular text. Every
shaped glyph resolves to the exact bundled font bytes used by the tested
package. Direct endpoint rendering uses Iced's existing mesh/text
pipelines and sRGB render targets.

V2 explicitly converts sampled linear colors to encoded RGB before
mixing. V3 instead uses compatible `Rgba8Unorm` views of the same native
`Rgba8UnormSrgb` texture storage, and an Unorm output view. Its shader
reads, mixes and stores encoded values directly, removing the transfer
function calculations. This format change leaves both native endpoint
images byte-identical to V2; neither variant adjusts fixture colors or
font rendering.

Astra independently checks all output channels against scalar endpoint
interpolation. Both cached endpoints are byte-exact, every capture is
opaque, and no intermediate channel leaves its endpoint interval. At
alpha 0.25, 0.5 and 0.75, V3's maximum error is 0.5 of an 8-bit channel
level, consistent with rounding. V2's maximum errors are 0.75, 0.5 and
0.75. Visible native text, distinct backing fields and the clipping
boundary are also checked; these are not empty-scene passes.

Call-site counters record two native endpoint renders, one native text
submission and five mix passes. The five passes reuse two endpoint
textures, one output texture, a parameter buffer and the bind group;
no additional native draw or retained-resource creation occurs between
them. The three texture payloads total 768,000 bytes at this fixture
size. Counters exclude Iced's internal atlases and separate the seven
test-only readbacks. This establishes reuse, not a production memory
budget or a measured frame-time improvement.

V3 is the preferred route for the next application experiment. Shader
sources, exact diffs, compiler/dependency hashes, logs, all seven captures
per variant and independent checks are retained in the NB directory and
`/tmp/cp-eras-next/nd-prototype-review/ROOT-REVIEW.json`. Neither prototype
is integrated into a screen. Actual group extraction, physical scale,
backing invalidation and backend coverage remain below.

One integration shortcut is ruled out by the pinned renderer code:
`Primitive::prepare` runs under the engine's primitive-storage write
lock, while a child `Renderer::present` calls `Engine::trim`, acquiring
that same lock. Native endpoint rendering must occur outside those
primitive callbacks; cloning the engine alone does not remove the
reentrancy hazard.

## NG/NI real-panel layer boundary

The next scratch candidate separates the Dashboard's final `panel-fade`
group into a visual-only canvas, leaving the real ground and Backdrop
under it. The foreground keeps the original full primitive list for
hit testing, hover and held-state identity; only its paint slice stops
before the panel. The separate panel still uses the original Motion,
fonts, geometry and per-ink alpha. This deliberately tests layer order
before changing composition.

Six captures are file- and RGBA-exact to the unchanged package: disabled
1600 rest; enabled 1600 rest, hidden panel at 0.4 s and mid-fade at
0.55 s; enabled 3840×2160 and 2000×1125 rest. Each enabled capture logs
both actual WGPU draw sites and the successful source-identity guard;
disabled capture logs neither. The initial candidate's guard did not
activate, so that attempt was rejected as evidence. V3 gives the
Dashboard table stable static identity and passes the same checks.

This establishes a usable panel boundary without changing its current
pixels, including the existing fade defect. The software backend keeps
its historical draw path in the candidate, but has not been captured in
this experiment. Interaction coverage, cache behavior and frame cost
remain integration requirements. The source snapshots/diffs and failed
attempt are retained in `/tmp/cp-eras-next/ng-panel-split/`; fresh packaged
size controls are in `/tmp/cp-eras-next/nh-panel-split-baselines/` and
Astra's independent six-case review is in
`/tmp/cp-eras-next/ni-panel-split-review/ROOT-REVIEW.json`.

## NJ/NM parent-renderer bridge

The next frozen scratch bridge passes through the ordinary parent
`Renderer::present` on both RGBA and BGRA sRGB targets. It prepares the
two native endpoints before normal primitive preparation, outside the
primitive-storage lock. Ordered endpoint layers preserve text beneath
later geometry. A passive display primitive draws the composed output
inside a separate clip; a continuation layer preserves later foreground.

Astra compiles and runs the vendor test, independently normalizes the raw
BGRA bytes, and checks all sixteen parent frames. Both direct endpoints
are byte-exact, RGBA and BGRA results agree, and a repeated midpoint is
exact. Intermediate errors are at most half an 8-bit channel level
against encoded interpolation; the later foreground's antialiased
perimeter is excluded from that particular oracle because it blends
after composition. A separate whole-frame endpoint-interval check
includes that perimeter and finds no violations.

Visible controls check native Rajdhani panel text, earlier text covered
by the panel, surrounding parent text omitted from the endpoint scenes,
the clipped panel strip, and later opaque foreground. Source review also
checks prefix flushing and endpoint-layer order. Per format, six fade
draws use two native endpoint renders, seven endpoint geometry submissions
and three endpoint text-geometry submissions. Five cache hits create no
additional endpoint textures or native draws. Three texture payloads
total 768,000 bytes at 320×200; equivalent full 4K storage would be
99,532,800 bytes (94.92 MiB), excluding pipeline, atlas and test-readback
overhead. This is an experimental allocation, not a production budget
or a frame-time measurement.

The first compile failed on an immutable test iterator; its source/log
are retained. V2 changes only that binding to mutable and passes. Frozen
sources are in `/tmp/cp-eras-next/nj-native-group-bridge/`; compiler,
dependency, binary and capture hashes, the independent checks and
`ROOT-REVIEW.json` are in `/tmp/cp-eras-next/nm-bridge-review/`.

The bridge still permits only identity transforms and no enclosing clip.
NN traces the actual panel canvas through two enclosing widget layers
and its canvas translation, so it cannot yet use that bridge. Simply
dropping the guard would make the extra continuation consume a caller's
clip pop. The next NO scratch lane adds scoped insertion that preserves
those layers and transforms, plus bounded cache-invalidation checks.
The source trace and selected design are in
`/tmp/cp-eras-next/nn-group-canvas-contract/`. Actual endpoint extraction,
application cache keys, bounded texture allocation, other callers and
backend coverage remain unverified; no application fix is accepted.

## NO/NQ scoped-layer verification

The scoped candidate records ancestor clips and resumes the caller's
layer after each pop, applying a captured translation/uniform scale to
native endpoints. A completely clipped-away group is intended to do no
work. Source review rejects the first fixture before compilation: its
direct foreground was behind the panel, two invalidation steps changed
multiple inputs, and post-pop controls did not overlap the group.
V2 repairs those controls while preserving the renderer and shaders.

The root-run V2 test compiles but fails exact visible-endpoint parity at
viewport scale 0.8. All six hidden endpoints and the first five visible
endpoints are exact. Their completed midpoint pairs also repeat exactly
and satisfy the whole-frame endpoint interval. The failing case changes
91 pixels in column x165, rows y25–115, by up to 11 channel levels. The
failure stops execution before BGRA, the original NJ replay and the
clipped-away control, so none is claimed as passed for this candidate.

That column follows the bright strip's antialiased left edge inside the
output clip. Independent source traces find different MSAA pass grouping:
the direct panel puts its shapes and strip in one native triangle layer;
the child endpoint opens a layer for each Geometry. Resolving the shape
before drawing the strip can change partial-coverage compositing. This
is a strong source/path attribution, awaiting the corrected render.

NO V3 preserves explicit endpoint layer batches, including through
geometry caching. It passes the unchanged fractional-scale case without
shader, color, artwork or equality-tolerance adjustment. Targeted controls
also keep the effective clip fixed while changing transform, and change
actual content with the revision.
The earlier transform case changes derived clip bounds too, and the
earlier revision case redraws identical content; neither proves those
stronger cache properties alone.

Frozen sources, rejected attempts and the diagnosis remain in the NO
directory. Build logs, partial raw captures and Astra's independently
reproduced failure are in
`/tmp/cp-eras-next/nq-scoped-group-review/ROOT-REVIEW.json`. The scoped
V2 remains rejected; its repaired successor is verified below. The
application fade defect stays open.

### Batch-preserving successor and feature coverage

Astra independently checks all 104 normalized frames from V3 and V4:
88 scoped and supplemental frames plus the sixteen original bridge
frames. Native RGBA/BGRA bytes normalize identically. All direct hidden
and visible endpoints are exact; the scoped direct controls also exactly
match V2, including its formerly failing case. Repeated midpoints are
exact, with no whole-frame endpoint-interval violations and at most half
an 8-bit level of encoded interpolation error outside later foreground
AA edges. The effective group clips are opaque; larger targets leave
the area beyond the fixture's fixed-size backing transparent.

Cold cached geometry at fractional scale matches the direct endpoint.
Its following warm draw reuses the two endpoint renders and all retained
resources. Six scoped key cases each miss once and hit three times;
separate transform controls hold the effective clip fixed. A content
change with a new revision produces different pixels and rebuilds the
endpoints. Supplemental direct equality is asserted in the compiled
fixture; those direct bytes are not separately saved. A wholly clipped
group matches the no-group image and leaves all group counters at zero.

The NS API review catches an optional-feature compile defect: the new
pass-preservation field was geometry-gated, while its read was not. Root
reproduces the V3 failure, then compiles V4 without geometry both with and
without image support against pinned dependencies. V4 adds only the
matching conditional branches. Its 208 raw native/normalized captures
are byte-identical to V3. The two independent pixel reports, feature
compile logs and accepted fixture record are under
`/tmp/cp-eras-next/nq-scoped-group-review/`, with final acceptance in
`v4/ROOT-REVIEW.json`. This proves the crate's conditional code paths,
not a separately rebuilt transitive feature graph.

The bridge remains an experiment with one group, explicit native pass
batches, positive uniform transforms and a caller-owned content revision.
The NT application successor below connects the real Dashboard Ground,
Backdrop, pointer-resolved prefix and full-opacity panel. Full-viewport
three-texture retention remains experimental; tiny-skia and production
cache policy remain separate acceptance gates.

## NT/NV first actual Dashboard result

The opt-in scratch application preserves the existing pointer-owning
canvas and supplies its resolved paint state to the later panel canvas.
That canvas builds native endpoints from the existing Ground, Backdrop,
prefix and panel draw helpers, retaining their pass boundaries. The
content key includes palette/ground, table identities, resolved pointer
state, bounds, actual endpoint motion and Iced's public font version.
It caches geometry handles for subsequent alpha-only draws. This policy
still requires the same-process controls below before it is accepted.

Source review repairs three initial problems before building: a font face
count was weaker than the available font version, an unconditional native
assertion broke the software fallback, and counting the whole backdrop
table included the terminal fade even though Backdrop paints only leading
Soft groups. The corrected endpoint clock ends at 500 ms; alpha continues
through the existing 400–700 ms fade. The actual Cargo/Nix build then catches
four ignored return values under the vendor's deny-unused-results lint.
V4 explicitly discards those values and passes all 312 package tests.

Astra verifies six actual application captures. Disabled and enabled
1600 rest, enabled 1600 hidden, enabled 4K rest and enabled 2000×1125 rest
exactly match the current package. At 550 ms, the original 2,529 pixels
brighter than both endpoints fall to zero. All 78,333 changed pixels lie
inside the panel's complete support, x1170–1401 and y259–725; nothing
outside the panel changes. Across the right side, 1,620,000 channels stay
inside the endpoint interval and differ from encoded interpolation by at
most 0.499 levels. The original curve gives alpha 0.707107. At x1319/y347,
the previously washed-out RGB 144/106/61 becomes 67/47/30, between hidden
13/9/12 and resting 89/63/37. Native font bytes, artwork and timing tables
are unchanged.

Sources and preserved attempts are in `/tmp/cp-eras-next/nt-dashboard-group/`.
The isolated package is
`/nix/store/i1m315506i52g0h7gknyj8kmaam6c42i-cp-eras-ui-0.0.0`.
NV's build/source hashes, package test log, capture plans, raw app logs,
independent midpoint calculation and `v4/ROOT-REVIEW.json` are in
`/tmp/cp-eras-next/nv-dashboard-group-review/`.

These application captures show one cold draw each and do not prove cache
hits or interaction invalidation. Separate controls below address those
properties. The candidate stays in scratch, and no production runtime
change is accepted.

## NW/NY cache controls and OA larger fades

The ignored NW test calls the actual canvas draw/update methods with one
Pointer and one panel cache. Source review corrects its first draft's
assumptions about the default selection and absent held-backdrop overrides,
and adds a settled no-hover frame to isolate the next pointer change.
NY first compiles the test without executing it: the original Crane check
phase had already captured its command before the attempted argument
override. An explicit check phase then runs the intended test successfully.

Astra independently parses all thirteen observed rows. The 550→600 ms
alpha-only transition leaves revision, endpoint geometry/text submissions
and group resource-creation counts unchanged while recording a GPU cache
hit. Isolated clock, hover, press, release, selection, palette, size and
font-version changes create new revisions and render both endpoints.
There are twelve misses, one hit and 24 native endpoint renders in total.
The font check reloads byte-identical bundled Rajdhani through the public
API, increasing its version from 12 to 13. The final restore changes
several inputs together and is not another isolation claim.

This proves the tested cache transitions, not state-image correctness or
widget-tree message routing. The actual style has no held-backdrop
overrides, and ordinary parent rendering/internal backend allocations are
outside these counters. The test-only package preserves NT runtime code;
its CSV, adapter record, compiler log and independent `v3/VERIFIED.json`
are under `/tmp/cp-eras-next/ny-dashboard-cache-review/`. The passing test
package is `/nix/store/qmdjlfra2vlzfbs3s0nsicz9cn04l4c7-cp-eras-ui-0.0.0`.

OA captures both the unchanged package and NT V4 at 400 and 550 ms for
3840×2160 and 2000×1125. Hidden images remain byte-exact; earlier resting
images are also exact. At 4K, 21,898 over-bright pixels become zero; at
fractional size, 4,785 become zero. All 442,738/122,060 changed pixels lie
within the panel, with no changes outside it. Across the right side,
both sizes have zero endpoint-interval violations and maximum encoded
interpolation error 0.499. Raw captures, plans and the combined review are
in `/tmp/cp-eras-next/oa-group-size-review/`.

The NZ source review distinguishes nominal retained group pixels from
actual peak GPU memory. Three full 4K textures retain 99,532,800 pixel
bytes, excluding native MSAA surfaces, other renderer data and driver
overhead. OB implements a scratch exact-alpha bypass/release: draw
the original empty/full panel at alpha zero/one and release the retained
group resources. Fade re-entry must rebuild, and all endpoint/intermediate
pixels must remain exact. Cropped endpoint textures remain a later design
option requiring complete support and transform/AA proof; no crop or
memory/performance result is accepted from this source review. Its later
release verification follows below; pointer and palette images, software
behavior and cold/warm cost remain open.

## OB/OE endpoint bypass and OD/OF release controls

OB changes only the panel helper and group-state resource API. Exact zero
consumes the prefix's resolved state, clears the CPU endpoint entry,
releases backend group resources and submits no panel geometry. Exact one
does the same release and uses the existing direct panel painter. The
intermediate key, native passes, fonts, artwork and timing remain unchanged.
CPU revision and cumulative backend counters survive release.

The isolated package passes all 312 tests. Astra verifies its 134 filtered
build inputs against the frozen candidate, then compares nine actual app
captures: hidden, resting and midpoint at 1600×900, 3840×2160 and 2000×1125.
Every entire frame exactly matches its previously accepted counterpart.
Thus the release path preserves both endpoints and the corrected fade.
The package is `/nix/store/qrj5lj0nn3s9hvhpw9jplia5939fg0gv-cp-eras-ui-0.0.0`;
source checks, capture plans and the combined `ROOT-REVIEW.json` are under
`/tmp/cp-eras-next/oe-endpoint-release-review/`.

OD extends the real-program same-process test to 23 steps. Astra's separate
CSV verifier confirms fifteen misses, two alpha-only hits, six endpoint
bypasses and thirty native endpoint renders. Every bypass clears the CPU
entry, reports zero retained group bytes and preserves revision and all
cumulative counters. Each re-entry increments the revision, rebuilds both
endpoints and creates exactly three textures, six views, two pipelines,
one buffer and two bind groups. Warm hits add only mix/display work and
the cache-hit count. The original pointer, selection, palette, viewport
and font-version controls continue to pass.

The 4K fade→full→fade sequence observes 99,532,800→0→99,532,800 nominal
application-owned group pixel bytes. This proves release of retained
handles, not immediate GPU residency: queued submissions, native MSAA,
atlases and driver storage are excluded. Intermediate fades still retain
three full-viewport textures; no peak-memory or draw-time improvement is
claimed. Test sources are in `/tmp/cp-eras-next/od-dashboard-release-controls/`;
the passing test package, output hashes and independent `VERIFIED.json`
are recorded under `/tmp/cp-eras-next/of-release-controls-review/`.

No production runtime change is accepted by the endpoint-release result.

## OG/OI actual state pixels and palette limit

The ignored OG test renders 112 raw RGBA frames across sixteen cases:
default, hover, held and selected rest/hover/held at 1600×900, plus default
and selected-held at 2000×1125. Each runs with the reference palette and
with background red increased by 0.05. It invokes the actual pointer and
Dashboard state methods, then paints either the original full foreground
or the candidate prefix/panel through the native renderer. It uses the
same bundled fonts, source tables, Engine and MSAA, with padded GPU readback.

Astra independently checks all raw bytes and counters. All sixteen hidden
and full endpoint pairs are exact, as are the sixteen cold/warm midpoint
pairs. Every midpoint leaves pixels outside the panel unchanged, has zero
right-side endpoint-interval violations, and differs from encoded
interpolation by at most 0.499. Warm repeats add no native endpoint/text
or resource creation; full endpoints release group storage. The default
original-full and candidate-midpoint frames at both sizes also match the
accepted actual app captures, anchoring the manually reconstructed layers.
Hover, held and selection controls produce nontrivial pixel differences.

The stronger custom-backing check fails: the comparison changes only two
resting pixels at 1600 and none at 2000. Its palette and viewport history
both change, so the two pixels cannot be attributed to palette alone.
The later OQ/OS controls below demonstrate a separate size-history effect.
The leading Soft backing contains an opaque fixed PAGE rectangle, and the
panel/card inks are fixed. The palette-dependent Ground is covered.
Original midpoint changes therefore do not establish a different opaque
endpoint backdrop.
Root preserves the failed nontriviality gate and accepts only the tested
state/palette behavior; no artwork or backdrop is replaced to manufacture
that coverage. General alternate-backing coverage remains separate.

The passing test package is
`/nix/store/5y3da5rv9j09cy44a7ljrjskjhcw54l2-cp-eras-ui-0.0.0`.
The frozen harness and source limitation are under
`/tmp/cp-eras-next/og-dashboard-state-pixels/`; raw-output hashes, independent
pixel/counter checks, preserved `V1-REVIEW.json` and the scoped final
`ROOT-REVIEW.json` are under `/tmp/cp-eras-next/oi-state-pixel-review/`.

## OL/OM cost result and OQ/OS size-history controls

The fixed cost run finishes all 1,664 samples in four fresh processes with
alternating route/size order. Astra independently checks every counter row,
sample count, ordering and first/last phase hash. Each route/size block
reuses its renderer and target. Eight cold-group samples follow eight
untimed native rest draws; 64 warm samples reuse one primed group; 32 rest
samples exercise endpoint release. Cold means fresh group resources, not
process/font startup. Canvas construction, geometry submission and native
present are measured separately; GPU completion waits, readbacks, output
formatting and target creation are outside those CPU spans. Unchanged
scratch diagnostic logging remains included.

The initial whole-benchmark pixel gate fails: rendering 1600 after 4K
changes two grain-edge pixels relative to the accepted fresh app image.
Original and bridge rest frames remain identical to each other. All four
4K app anchors are exact. The new 600 ms bridge frame also preserves all
pixels outside the panel, has no endpoint-interval violations and differs
from encoded interpolation by at most 0.499 at both sizes when compared
with its own resting frame. The failed gate and fixed samples are retained;
there is no tolerance change or selectively repeated timing series.

Six untimed OQ/OS original-route captures isolate the history to retained
Engine state. Initial 1600, another fresh Renderer at 1600 and 4K each
match their accepted app frames exactly. Returning to 1600 reproduces
OL's two pixels at (1220,562) and (1331,592), changing RGB from
230/171/97 to 218/160/91. A new Engine on the same Device restores the
entire accepted 1600 frame. A new-Device trial is therefore unnecessary.
The native MSAA pipeline shares a target through Engine clones, grows it
without shrinking and projects against its retained dimensions. That is
a source-supported explanation, not a proven MSAA-specific cause: no
targeted MSAA ablation or production resize fix has been run. All 135
compiled inputs match the frozen test source.

The exact 4K portion is enough to reject this prototype as an efficiency
improvement. Ranges below are the four process medians of summed canvas,
geometry and native-present CPU spans, in milliseconds:

| 4K phase | Original | Group prototype |
| --- | ---: | ---: |
| Fresh group at 550 ms | 2.13–3.20 | 441.40–587.46 |
| Reused group at 600 ms | 2.16–3.26 | 8.50–11.62 |
| Settled at 700 ms | 2.22–3.27 | 2.73–3.65 |

Warm CPU work is slower in every paired process, so cold preparation never
pays back under the planned CPU criterion. The broad rest-spread criterion
passes, but both sizes fail the planned warm/payback cost tests. The 1600
whole-benchmark app-anchor gate remains failed independently. These are
llvmpipe submission CPU measurements, not hardware GPU or window latency.
Completion waits are recorded separately and vary substantially across
processes; they do not establish a live performance improvement.

Evidence is under `/tmp/cp-eras-next/om-group-cost-review/` (fixed raw
samples and preserved `PARITY-FAILED.json`),
`/tmp/cp-eras-next/or-cost-history-review/DIAGNOSTIC.json` (independent
counter, pixel and cost calculations), and
`/tmp/cp-eras-next/os-size-history-review/ROOT-REVIEW.json` (six-frame
history controls and compiled-input verification).

## OU/OV/OY shared mix encoder

OT's source review finds that warm hits already avoid retained texture and
pipeline creation, but still create and submit a separate mix encoder
before parent presentation. OU moves that unchanged mix pass into the
parent encoder. Only two vendor files change; endpoint submissions,
shaders, keys, resource counts, release and native layer batches stay the
same. It removes one command encoder and one queue submission per group
frame. The opacity write precedes submission, and the mix precedes display
within the parent command buffer. Group-state locking still ends before
parent preparation and rendering.

OV replays the frozen actual-state harness: all 112 raw frames and the
entire counter manifest exactly match OB. All 135 compiled inputs match
the candidate. PA also replays the native renderer fixture: all 220 output
files match NQ v4, including 104 normalized images and their native RGBA/BGRA
counterparts. Independent endpoint, clip, parent-order, fractional-scale,
interpolation and cache-invalidation checks pass. Geometry-disabled
default and default-plus-image builds are checked separately against pinned
dependencies; this does not rebuild the transitive feature graph.

OW/OY then compares OB and OU in eight fresh processes, paired in
OB/OU, OU/OB, OU/OB, OB/OU order. Each has one 4K group block and retains
OL's eight cold, 64 warm and 32 rest samples: 832 measured rows total.
This tests mix scheduling within the corrected group implementation, not
speedup over the original per-ink scene. A single 4K block avoids the
separate 1600 resize-history issue. Original OL/OM samples remain intact.

All rows, resource counts and first/last hashes pass. Six raw phase frames
are identical across OB, OU and the prior 4K controls; cold/rest also match
the accepted actual app captures. Warm midpoint error remains below half a
channel level. The prespecified warm CPU gate passes in every pair with
nonoverlapping process-median ranges:

| Pair | OB warm CPU ms | OU warm CPU ms | Reduction |
| --- | ---: | ---: | ---: |
| 1 | 7.80 | 2.57 | 67.0% |
| 2 | 10.31 | 2.85 | 72.4% |
| 3 | 11.21 | 3.10 | 72.3% |
| 4 | 11.48 | 3.37 | 70.6% |

This is a calling-thread submission improvement, not an overall frame-time
win. CPU-plus-completion time improves in two pairs and worsens in two.
Cold calling-thread medians increase from 392–575 ms to 586–830 ms, while
OU's separately measured cold waits fall to roughly 0.065 ms. Driver work
can block inside the calling-thread spans; the apparent movement of wait
time needs profiling. Rest results are mixed too. No cold payback, total
latency or hardware-GPU improvement is claimed.

The verified scratch change is retained for its warm submission result.
Production integration remains unaccepted: cold preparation, intermediate
texture cost, alternate backing and software-backend behavior still need
work. Cold native-present time includes two endpoint renders and resource
setup; existing spans cannot assign its cost specifically to pipeline
creation. PB defines a small subspan investigation before another cold
optimization. Compatible-miss allocation reuse and software composition
have separate source plans, with no implemented or measured result.

Frozen source and source review are under
`/tmp/cp-eras-next/ou-shared-mix-encoder/`; state verification is under
`/tmp/cp-eras-next/ov-shared-mix-state-review/`. The fixed harness is under
`/tmp/cp-eras-next/ow-shared-mix-cost-controls/`; paired samples, compiled
input hashes and `ROOT-REVIEW.json` are under
`/tmp/cp-eras-next/oy-shared-mix-cost-review/`. Fixture and feature evidence
is under `/tmp/cp-eras-next/pa-shared-mix-fixture-review/`.

## PD/PE cold attribution and PH hardware comparison

PD instruments the frozen OB and OU schedules with 50 fixed trace slots.
PE runs four fresh 4K processes in OB/OU/OU/OB order, each with eight
untimed rest frames followed by cold, warm, release and re-entry draws.
All 16 raw frames exactly match the prior software controls. All resource
counts, 800 trace rows and inclusive-span checks pass; all 136 compiled
inputs per variant match the frozen source. These are individual
attribution observations, not benchmark medians. Calling-thread wall
spans can include driver waits; nested spans must not be added together.

On llvmpipe, cold/re-entry resource setup takes about 1.0–1.3 ms and child
construction/destruction stays below 0.25 ms each. The large spans are the
complete endpoint's target-plus-view creation after background submission
(172–201 ms), its queue submission (197–227 ms), and OU's parent submission
(180–188 ms). OB instead waits another 156–170 ms after submission. This
supports wait migration; it does not identify allocation itself as that
much work. Warm OB parent submission takes 5.9–6.1 ms versus about 0.10 ms
for OU. OB's separate mix submission itself takes only about 0.05 ms.
Pipeline caching is therefore not the main measured software cold cost.

PH runs the exact same binaries and fixed order offscreen on an RTX 4070
with NVIDIA 595.99.02. It opens no window. All 16 frames match their
corresponding hardware OB controls byte for byte, including re-entry.
All counter and trace gates pass. Hardware images differ from llvmpipe
at roughly 165,000 pixels, with maximum channel differences of 12–17;
cross-driver golden equality and source fidelity are not claimed.

Hardware cold/re-entry calling-thread time is roughly 72 ms, except the
first OB observation at 87 ms; warm time is 1.81–1.88 ms plus a separate
2.25–2.36 ms completion wait. Each cold endpoint spends about 24–29 ms
in preparation. Target creation no longer absorbs hundreds of milliseconds.
The first OB resource setup is 10.9 ms, later ones about 1.0–1.3 ms;
process/driver-cache startup is not controlled beyond the recorded order.
This changes the next optimization priority to endpoint preparation.
It is not a measured desktop/window result or a Neomil hardware benchmark.

PF's source audit initially overgeneralized atlas sharing. The corrected
report distinguishes Engine-shared live-text atlases from the child-owned
atlases, buffer caches and uploads used by `Geometry::Cached`. The actual
Dashboard endpoints use the cached path. Both endpoints contain the same
cached background geometries, but separate child renderers prepare them
again. This motivates PI below: reuse one child for those two endpoint
draws within a miss, preserving native passes, full-size projection and
separate submissions, and retain no child beyond the miss.

PG also reviews child-layer scissors. The current Dashboard group bounds
cover the whole canvas, so simply reusing those bounds cannot reduce its
work. A smaller clip needs an independently verified panel-support bound;
the old diagnostic aperture is insufficient. Full clears, intermediate
texture memory and the fullscreen MSAA resolve would need separate care.
No scissor optimization is accepted from that source review.

Evidence: `/tmp/cp-eras-next/pe-cold-profile-review/ROOT-REVIEW.json`,
`/tmp/cp-eras-next/ph-hardware-profile/ROOT-REVIEW.json`, corrected source
inventory `/tmp/cp-eras-next/pf-child-renderer-lifetime/REPORT.md`, and
`/tmp/cp-eras-next/pg-endpoint-scissor/REPORT.md`. Production code, fonts,
SVGs and goldens remain unchanged by this profile.

## PI/PK/PL/PN shared endpoint child

PI changes one vendor file to use the same child Renderer for background
and complete endpoint draws within each miss. The complete Dashboard
endpoint starts with cloned background cache IDs. PM verifies that the
first native present retains their text atlas, buffer cache and mesh
uploads; the second text prepare skips the unchanged prefix. New panel
text still prepares normally. Each endpoint keeps its full-size clear,
projection, native layer batches and separate queue submission. The child
is dropped before mix-resource setup and is never retained between misses.
No font, shader, geometry or resource-key changes are involved.

PK's native fixture matches all 220 OU outputs exactly, including 104
normalized frames and their native RGBA/BGRA counterparts. Independent
clip, fractional-scale, cached/live input, parent-order and interpolation
checks pass. PL then matches all 112 actual-state frames and their entire
manifest across sixteen cases. Both cost binaries contain the exact same
state and cost harnesses; only the vendor implementation differs. All 136
compiled inputs per variant are verified.

PN runs four fresh processes per backend in OU/PI/PI/OU order. Each uses
the fixed eight cold, 64 warm and 32 rest samples: 416 per backend, 832 in
all. Every resource row and first/last phase hash passes. Six raw phase
frames per backend match both variants and the previous backend controls
exactly; software cold/rest also match the actual app anchors. There is no
cross-driver equality claim. Cold means new group resources after native
rest warmup, not initial application/font startup.

The prespecified hardware gates pass: cold calling-thread and
CPU-plus-completion medians each improve at least 20% in both pairs, while
warm/rest completion medians stay within the 15% regression allowance.
The RTX 4070 cold CPU-plus-completion results are:

| Pair | OU ms | PI ms | Reduction |
| --- | ---: | ---: | ---: |
| 1 | 71.27 | 43.80 | 38.5% |
| 2 | 70.76 | 42.83 | 39.5% |

Hardware calling-thread medians fall from 70.7–71.2 to 32.3–33.6 ms.
About 10 ms then appears in the separate completion wait; the table keeps
that wait included. Warm completion stays about 3.8–4.1 ms; rest changes
from about 4.00 to 4.02–4.05 ms. These are two balanced offscreen pairs,
not live window measurements or an original per-ink scene comparison.

The software result is mixed and does not justify a general speedup:
cold completion changes 507→513 ms in one pair and 688→596 ms in the other.
Warm/rest worsen in the first pair and improve in the second; the software
15% guard fails. Keep these fixed results rather than selectively repeating
pairs. PI is retained as a pixel-preserving scratch optimization with a
measured hardware benefit; full production acceptance remains open.

The three retained fade textures still total 99,532,800 nominal pixel
bytes at 4K. PO's source inventory finds the static panel paths and clipped
grain fit a conservative candidate box, but its eleven unclipped native
text runs need actual glyph-ink/AA bounds before that box is safe. PR is
the next measurement lane. A changed resolved font must not silently
invalidate a future crop. Other backends and live behavior remain unverified.

Evidence: `/tmp/cp-eras-next/pi-shared-endpoint-child/`,
`/tmp/cp-eras-next/pk-shared-child-review/ROOT-REVIEW.json`,
`/tmp/cp-eras-next/pl-shared-child-app-review/PIXEL-REVIEW.json`,
`/tmp/cp-eras-next/pn-shared-child-cost/ROOT-REVIEW.json`, and the PM/PO
source reports. Production code, fonts, SVGs and goldens remain unchanged.

## PR/PS panel support and PT/PU retained crops

PR renders the actual terminal panel and its eleven text runs separately,
using cached native geometry at 1600×900, 2000×1125 and 3840×2160. Each
paragraph uses the scene's font, size, line height, alignment and shaping;
every resolved glyph is checked against the bundled Regular 400 or
SemiBold 600 bytes. PS verifies all 72 captures across llvmpipe and RTX
4070. Every nonzero-alpha pixel fits the proposed canonical rectangle
`[1000,200,1550,780)`, including a two-physical-pixel guard. The smallest
observed clearance is 54 pixels. All support boxes match between drivers;
this does not imply cross-driver pixel equality. These standalone captures
cover the pinned faces and three sizes, not arbitrary fonts or transforms.

PT retains a smaller texture for an existing partial group clip while
preserving full-viewport native endpoint drawing and MSAA. Each endpoint
is copied on the GPU to an integer crop in the same command submission,
without resampling or CPU readback. Mixing uses cropped coordinates;
display subtracts the integer origin. Full clips retain the original
three-texture route. Partial clips create five textures per miss: two full
temporary targets, two retained crops and the mixed output. Nominal retained
bytes exclude queued references, temporary targets, MSAA and font atlases.

PU compares PT with the previous PI runtime using the same native fixture,
including an added full-clip control. All 106 normalized images and their
106 native-format counterparts match exactly within each driver. Independent
interpolation, clip, transform, cache, parent-order and RGBA/BGRA checks
pass; software also preserves all 208 earlier raw anchors. The eight changed
metadata files per backend differ only in the expected texture-creation
counts and retained bytes. The fixture's partial clip retains 419,136 bytes
instead of 768,000; its full-clip control still retains 768,000 bytes and
creates three textures. Both frozen candidates and compiled artifacts are
verified. This backend proof does not yet reduce Dashboard memory: its
group bounds still cover the viewport.

The next app experiment keeps the semantic clip and adds a separately
guarded storage hint. It must fall back for unverified faces, sizes or
outer transforms, avoid repeated shaping on warm frames, and preserve
complete Dashboard state and 4K pixels before claiming a memory benefit.

Evidence: `/tmp/cp-eras-next/ps-panel-ink-review/ROOT-REVIEW.json` and
`/tmp/cp-eras-next/pu-cropped-fixture-review/ROOT-REVIEW.json`.

## PW/PX guarded Dashboard crop

PW keeps the semantic group clip intact and carries a separate retained
storage hint through native and cached geometry. The backend accepts it
only for its exact measured physical size, unit device scale and identity
outer transform. Invalid bounds or a different projection use the previous
storage path. The effective integer crop is part of the resource key;
display intersects its scissor with that crop while leaving parent pixels
outside the support region intact.

The Dashboard guard requires its exact static scene and one of the three
measured canvas sizes. It shapes the eleven actual text runs with the same
parameters as normal drawing and verifies resolved glyph weights and font
bytes. Successful byte comparisons are shared across runs for each resolved
face. One bounded cache stores the result by canvas bounds and font-system
Version; warm frames and endpoint re-entry reuse it. A font load invalidates
the guard, and a substituted face disables the hint. Direct raw database
edits that do not update Iced's Version are outside this cache contract.

PX preserves all 112 actual-state frames against PI on each driver. Only
the expected retained-byte and texture-creation columns change. Five paired
4K captures per backend cover hidden, cold fade, warm fade, rest release and
re-entry. Every pair is exact, and fade/rest also match the earlier PI
backend anchors. At 4K the three retained textures drop from 99,532,800 to
22,049,280 nominal bytes, a 77.85% reduction; rest still releases them to
zero. Temporary full-size native targets, queued GPU references, MSAA and
atlases remain outside that measure.

All 106 existing native fixture images per backend and their metadata still
match PT. The scratch app passes 267 library tests plus an isolated font
substitution/cache test. A separate vendor test verifies transform, scale,
viewport and invalid-bound fallbacks. Warm/re-entry controls observe one
font verification, and all 136 compiled app inputs are verified. PW v1 was
preserved after a pre-run review corrected the fractional-size byte oracle
and widened byte-comparison reuse; only PW v2 supplies this evidence.

Evidence: `/tmp/cp-eras-next/px-guarded-crop-review/ROOT-REVIEW.json`.

## PY/PZ fixed crop-cost comparison

Both runtime variants use the identical PY harness. Its only changes from
OW are explicit full/cropped storage oracles, checked outside timing:
99,532,800/22,049,280 bytes and three/five textures per cold miss. The
sample schedule, timing boundaries, phase clocks, readback and image gates
are unchanged. PZ runs PI/PW/PW/PI in four fresh processes per backend,
with eight cold, 64 warm and 32 rest samples each: 832 total. Every counter,
phase hash and historical backend image passes; software fade/rest also
match the accepted app captures. All 135 PI and 137 PW compiled inputs are
verified.

The prespecified hardware cost guard passes: no phase's median CPU or
CPU-plus-completion time regresses more than 15% in either pair. The latter
measure includes the explicit completion wait:

| Pair | Phase | PI full ms | PW cropped ms |
| --- | --- | ---: | ---: |
| 1 | Cold fade | 44.99 | 44.68 |
| 1 | Warm fade | 4.02 | 3.69 |
| 1 | Rest | 4.00 | 3.99 |
| 2 | Cold fade | 43.65 | 44.42 |
| 2 | Warm fade | 3.75 | 3.70 |
| 2 | Rest | 4.03 | 3.99 |

This supports the measured memory reduction without a material hardware
cost increase in these runs. Warm completion improves about 8.3% and 1.3%,
below the separate prespecified 10%-in-both-pairs speedup threshold. Do not
claim a demonstrated warm-speed improvement from this experiment. Cold
preparation remains about 44–45 ms and is not solved by retained cropping.

Software remains mixed: cold completion changes 475→553 ms and 692→619 ms;
warm changes 157→178 ms and 235→203 ms; rest changes 174→202 ms and
248→225 ms. The 15% guard fails in the first pair. Preserve all fixed
results without selective repetition or a general speedup claim. Nominal
retained memory is distinct from temporary/in-flight GPU storage, and
these offscreen timings do not establish live latency or GPU timestamps.

Evidence: `/tmp/cp-eras-next/pz-crop-cost-review/CONTRACT.md` and its two
backend `ROOT-REVIEW.json` records. Early fade behavior, other backends,
general-size coverage and production integration remain open. Production
code, fonts, SVGs and goldens remain unchanged by these scratch studies.

The QB source review identified a specific early-fade cost risk: `cards-open`
continues until 500 ms while `panel-fade` starts at 400 ms. The endpoint key
includes `min(scene.at, 500 ms)`, so distinct times during that overlap
invalidate it. OD/OF already verifies an isolated 450 ms resource miss;
it does not measure the cadence or capture early-fade pixels. Compare
adjacent 400–500 ms frames and counters before changing invalidation or
extrapolating the settled 550/600 ms costs to the whole animation. QD/QF
implements that comparison below.

## QD/QF early fade and cropped-content revisions

QD captures full and cropped 4K output at 400, 401, 417, 433, 450, 451, 467,
483, 499, 500, 500, 550, 600, 700 ms. All 14 pairs match exactly on software
Vulkan and the RTX 4070. The baseline incurs nine endpoint revisions and GPU
misses while the cards finish opening: 18 native endpoint draws and 45
created textures on the cropped path. The settled frames match earlier
per-driver anchors. These are work counts, not timing measurements.

QE gives accepted crop hints their own content revision. Only the exact
`cards-open` clip may remain active; its maximum canonical x940 lies left
of the measured crop starting at x1000. The helper requires at least two
physical pixels of separation. Panel children, leading soft backing and
state variants must settle before the fade. Unknown motions or changed
clips retain a time-sensitive key. The font, source, size and projection
guards from PW remain in force.

For proven static crop content, the key omits only endpoint time. Palette,
ground, bounds, source/backdrop/state identity, selection, interaction and
font version remain. The proof is cached by those fields; a cached false
proof still uses a changing time. Backend full/crop revisions have distinct
enum variants so equal numbers cannot alias; device, format, viewport, transform,
semantic bounds and effective crop still participate in resource matching.
CPU endpoint geometry continues to track the original nine revisions,
preserving the full-render fallback.

QF preserves every QD raw frame on both drivers while reducing cropped GPU
misses 9→1, native endpoint renders 18→2 and created textures 45→5. Twelve
mixed/displayed frames and 22,049,280 retained bytes are unchanged; endpoint
release still reaches zero. The proof and font check each run once. All112
existing state frames, 10 paired release/re-entry frames and 212 native
fixture raw files per driver remain exact. The 273 library tests, isolated
font substitution test and two native projection/revision-domain tests
pass. All 137 compiled source inputs match the frozen candidate.

QH v3 then varies selection, palette, font version and leading backdrop in
one process at 4K. Ten full/cropped frame pairs remain exact per driver.
The cropped path misses on each relevant change and reuses the intervening
static frames; its font check runs again only after the font load. A
supported animated clip over software fills changes visible backing pixels
between 500/550 ms (1,232 interior pixels per driver); the static proof fails
and both times rebuild endpoints.
The palette control establishes key invalidation, not broad visible palette
coverage; the unrelated owned font load establishes version invalidation,
with face substitution separately covered by PX.

The first QH fixture placed Motion inside Soft, which the fill walker does
not support. It established counters but no visible animated backing; its
results are retained with that limit. The time-only v2 was built but not
run. V3 corrects the fixture grammar to Motion::Clip around Soft fills and
requires a nonzero pixel change in the clip interior. No runtime change
was needed for that fixture correction.

Evidence: `/tmp/cp-eras-next/qd-early-fade-review/ROOT-REVIEW.json`,
`/tmp/cp-eras-next/qf-static-crop-review/ROOT-REVIEW.json` and source review,
and `/tmp/cp-eras-next/qh-crop-invalidation-review/v3/ROOT-REVIEW.json`.
Elapsed early-frame cost remains unmeasured; QI/QJ defines the fixed paired
schedule before execution. Production code, fonts, artwork and goldens
remain unchanged by this experimental candidate.

Before accepting a runtime implementation:

- Preserve alpha-zero/one native pixels, fonts and rest goldens at
  canonical, 4K and fractional sizes; retain existing curve/timing data.
- Correct overlap through the fade against the assembled-group reference,
  including Dashboard, selected Store body and applicable Mailbox fades.
- Preserve nested clips, layer order, selection, hover/held behavior and
  custom palettes; invalidate cached backing/group data when they change.
- Bound retained textures and measure cold preparation and repeated-frame
  CPU work. Demonstrate texture reuse and absence of repeated glyph work;
  do not infer live GPU performance from headless timings.
- Verify supported backends explicitly. A WGPU-only prototype cannot
  establish tiny-skia correctness or complete the Motion scope.

No runtime, font, SVG or golden change has been accepted from this audit.
The Motion TODO owns this actionable renderer follow-up; its separate
transition-design and live-desktop requirements remain open.
