# Neomil tracked-text painting

2026-10-02, DZ. Follow-up to the [font and effect audit](../font-rendering-audit.md).
The baseline includes DY's per-draw advance cache. This work measures the
remaining character-paint cost before choosing an optimization.

## Measured route

The pinned Wgpu renderer queues ordinary, uniformly scaled text for later
raster preparation. Rotated text instead calls `Text::draw_with`, which
constructs a paragraph and a fresh Swash outline cache for each call, then
fills its positioned paths. The enabled tiny-skia backend uses the same
outline callback for the quarter-turn labels examined here. Both backends
create a fresh, untransformed frame for a clip draft.

Neomil Store has nine rotated tracked runs: the margin code and two edge
labels on each of four cards. Together they make 109 character calls.
The other 39 tracked runs enqueue 189 characters on the ordinary path.
The inspected Store clips precede the turns, so they do not reset a
rotated label to the ordinary route.

Two release profilers each complete 56 draw groups: Store at 1600 and 4K,
rest/opening/hover/held, with a first draw and six repeated draws per case.
The control keeps `fill_text` intact. A separate replay makes its exact
outline callback explicit and times path filling within it. Scene ancestry
is reset at clip drafts; static source review establishes the actual routes
for these tables. This is not a general renderer-branch counter.

Representative 4K rest warm medians:

| Span | CPU time |
| --- | ---: |
| Control: nine rotated tracked runs | 6.216 ms |
| Control: ordinary tracked enqueueing | 0.0115 ms |
| Replay: rotated preparation/extraction and loop overhead | 5.796 ms |
| Replay: rotated path-fill callbacks | 0.352 ms |
| Control: clip draft/paste outside child paint | 0.0274 ms |
| Control: root geometry finalization | 0.0037 ms |

All 112 accounting identities pass without overlapping spans, and the
run/character counts agree across states and variants. Across all measured
draws, replay path filling accounts for 4.86–6.34% of the rotated span.
The remaining span includes shaping, outline extraction, allocation, setup
and loop overhead; it does not isolate each of those costs individually.
Ordinary text preparation, GPU work and presentation happen later and are
outside this Scene benchmark.

The 1600 replay rest/opening samples show substantial drift in unrelated
buckets too. The profiler therefore establishes cost ownership, not an
optimization speedup or clean incremental instrumentation overhead. A
case's first draw is not necessarily process-cold.

## Exact replay verification

The original, control and replay release builds pass all eighteen full-RGBA
comparisons: Wgpu Store at 1600 and 4K, fractional opening and held previews,
and tiny-skia Store at 1600 plus a fractional held preview. Each backend
uses its own original image. The fresh Wgpu 4K original also matches the
previously verified DY native frame exactly. Source review and pixel checks
cover the same text construction, character order, local coordinates,
colors, parent transforms and clips.

## Positioned-path cache

The integrated cache retains already-positioned paths for direct quarter-turn
tracked runs for one Scene draw. It preserves the existing prefix advances,
per-character float-addition order, anchors, font selection and FreeSans
baseline rounding. The key includes complete content, full font identity,
face, exact size/tracking/origin/baseline bits and anchor. Color and alpha
are applied on replay, so feedback and palette changes use current ink.
Ordinary text, nested/other turns and unsupported runs keep their prior route.
Clip drafts reset the eligibility state just as both backends reset transforms.

Four FIFO entries retain at most 32 paths per run and 8,192 total path events.
Runs also have byte, character, size and finite-coordinate limits. The
collector stops retaining callbacks immediately on raster fallback or a bound
failure and stops before extracting another character. Negative entries
remember unsupported runs within the draw. No cross-draw font-registration
assumption or backend cache invalidation is introduced.

The counted sweep completes 280 groups and verifies unchanged existing
primitive, text, clip, title-outline and prefix-advance counts across all 140
foreground Scene draws. Each Neomil Store draw has six hits and three misses:
40 character extractions replace 109, while all 109 paths are still painted
in their original order. Three entries retain 40 paths and 972 events, with
no eviction, raster fallback or limit rejection in the measured cases.
Kitsch Dashboard also enters this path for two unique rotated labels:
16 extractions, no hits, 16 retained paths and 428 events. It gains no
extraction reuse; its small observed timing change is not credited to
caching. Other measured scenes do not enter this path.

## Release timing

Four timer-only release sweeps run serially in baseline/candidate/candidate/
baseline order, with the same compiler, dependencies, fonts and 280-group
driver. Every case has one first draw and six warm draws. The figures below
average the two warm medians on each side; diagnostic counters run separately.
These are Wgpu foreground canvas CPU times through geometry finalization,
not complete frame, GPU or presentation times.

| Neomil Store case | Baseline | Cache | Reduction |
| --- | ---: | ---: | ---: |
| 1600 rest | 24.347 ms | 20.651 ms | 15.2% |
| 1600 opening | 24.271 ms | 20.689 ms | 14.8% |
| 1600 hover | 25.387 ms | 21.868 ms | 13.9% |
| 1600 held | 25.366 ms | 21.810 ms | 14.0% |
| 4K rest | 22.630 ms | 19.419 ms | 14.2% |
| 4K opening | 22.612 ms | 18.802 ms | 16.8% |
| 4K hover | 23.657 ms | 19.884 ms | 15.9% |
| 4K held | 23.622 ms | 19.883 ms | 15.8% |

The 4K rest candidate medians are 19.949 and 18.890 ms; the first repeat
has greater sample spread. Both improve on both baseline medians, 22.648
and 22.612 ms. The other Store cases show smaller repeat differences.
Unchanged work also drifts, so small changes elsewhere are not credited to
this cache. First draws vary and are not a process-cold startup measurement.

## Cache verification

All 258 library tests pass with both host and isolated bundled fonts.
Regression tests exercise exact positioned path events for Rajdhani,
FreeSans and the Kitsch derivative, precise key changes, cache reuse,
FIFO/event eviction, raster rejection and conservative transform eligibility.
Fourteen native before/after pairs and four actual-Store/rest-wrapper controls
are RGBA-exact: Wgpu rest at 1600/4K/fractional sizes, fractional opening,
hover, held, custom palette and selected-card previews; tiny-skia rest and
held controls; and a Kitsch Store sentinel. This is in addition to the
eighteen explicit no-op replay comparisons above. Five further Kitsch
Dashboard pairs are exact at 1600, 4K, fractional rest/opening and tiny-skia
fractional rest, covering its two unique rotated labels. There are 23
cache-specific native comparisons in total.

The packaged build passes all 303 Rust tests. The full repository check
passes all 22 checks and all 27 visual cases on their first attempts.
Independent comparison finds 26 RGBA-exact goldens and the same one-pixel,
one-channel-level Neo-kitsch bar residual as before. Fresh packaged Neomil
Store at 4K and Kitsch Dashboard at fractional size match the reviewed
release candidate exactly. All 333 tracked test inputs match the Nix source,
and all sixteen originals retain their frozen hashes. Only five Markdown
files receive completion/SC-study notes after that test freeze; executable
code, assets and test inputs remain unchanged. Changes stay staged and
uncommitted. Source images, SVGs, font outlines and goldens are unchanged.
Evidence is under
`/tmp/cp-eras-next/dz-store-profile/`, `dz-text-routes/`, `dz-cache-perf/`,
`dz-verify-plan/`, `dz-root-review/` and `dz-validation/`. Sources, dependencies,
binaries and logs are pinned. `root-cache-native/results-recovered.json`
reconstructs the 18 comparisons after fixing a summary-key shadowing bug in
the scratch runner; all completed captures were preserved and independently
compared again, without repeating or changing them.
