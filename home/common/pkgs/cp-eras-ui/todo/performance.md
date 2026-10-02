[All workstreams and current status](../TODO.md). Paths below are relative
to the crate root. Performance changes must preserve the accepted artwork,
sRGB compositing, interaction states and opening clips.

## Store title printing cost — AN

- [x] **Reuse repeated title glyph outlines without changing pixels.**
  The source-supported ordinary Neomil title impressions add 61 clipped
  text runs. At 4K, a warmed 20-call `Scene.draw` comparison measures
  median CPU paint/geometry time 20.734→26.280 ms in the final candidate
  (20.669→25.315 ms before the clipping/order corrections). This excludes backdrop,
  GPU and presentation work; it does not describe an ongoing frame loop.
  Skipping unused width shaping for start-anchored text measures
  20.631/25.357 ms and stays scratch-only because the change does not
  materially reduce this cost. Iced's transformed text path creates a
  paragraph, Swash cache and glyph outlines for each run. Investigate a
  bounded cache of its own origin-relative glyph paths, keyed by exact
  font, text and size, then reuse those paths under existing scan clips,
  transforms and semantic ink. Keep the primary rendering unchanged until
  parity is established. Require exact baseline/candidate pixel checks at
  1600, 4K and fractional sizes, custom palettes, opening and hover/held
  states, plus fresh CPU timing. Do not drop scan rows or approximate the
  glyphs to improve the timing.
  AO's per-draw four-entry outline cache passes local glyph regressions
  and exact 4K/1600/fractional state comparisons. A new paired 20-call
  4K median falls 25.674→22.823 ms (11.1%), recovering part of the cost.
  Primary text, source artwork and goldens remain unchanged. All 290
  Rust tests, 22 repository checks and 27 first-attempt visual cases pass;
  the fresh packaged 4K image exactly matches. See
  [round twenty-six](../docs/reference-svg-round26.md).

## Neomil cold-draw investigation (2026-09-21)

Three optimized release trials per size measured the actual
`SoftCache::image` / `cut` sequence on a Ryzen 9 7950X3D (32 available
threads). This is CPU preparation with an empty application cache, not
launch-to-present time or GPU FPS. The baseline at `f9ccf3a` has four Soft
surfaces; the pending fidelity batch has eight.

| Canvas | Committed baseline median | Pending fidelity median |
|---|---:|---:|
| 1600×900 | 0.644 s | 1.366 s |
| 2560×1440 | 2.584 s | 5.574 s |
| 3840×2160 | 15.535 s | 36.611 s |

Natural-clock headless app traces confirm first backdrop callbacks of
1.477 s / 37.564 s at 1600×900 / 4K. Warm combined CPU geometry at 1600
is 1.207 ms median / 1.431 ms p95 (42 samples); both runs become idle.
These traces use software Vulkan and do not establish hardware-GPU
presentation or live interaction latency. The earlier 102 ms measurement
in `toolkit.md` describes older artwork, not this dashboard.

The pending version retains 253.1 MiB of raw cached RGBA at 4K; the
standalone compositor harness reaches 1014.9 MiB process peak RSS.
Full-app software-GPU peak RSS is about 3.8 GiB and must be kept separate
from the cache payload and live hardware-GPU memory usage.

## Fixes and acceptance criteria

- [x] **Prepare panel layers from one accumulated prefix.** Seven panel
  Soft siblings plus the main ground cause 44 group traversals: each
  `composite_over_bands` rebuilds its prefix and walks its own material
  twice. Accumulate the original float prefix once, then emit each
  original coverage-cut RGBA image without modifying that prefix.
  **Do not merge the images:** the initial merge experiment improved
  CPU timing and preserved integer-position pixels, but sampling seven
  separate cuts is not equivalent to sampling one merged image when a
  canvas is fractionally positioned. A three-canvas interaction preview
  exposed 5299 changed pixels, maximum channel delta 60. Preserve the
  separate GPU draws, clip and sampling behavior; compare each batched
  output to its former independent composite and review full captures.
- [x] **Limit mask work to affected pixels.** `soft.rs` previously allocated
  two full-band float RGBA buffers per mask, even for tiny glyphs. The
  pending scene's 312 masks expand to 2384 frame-area passes through prefix
  replay. Use conservative transformed bounds and skip disjoint bands;
  preserve fractional pixel coverage, nested masks, rotations, group
  opacity and absolute sample coordinates. Test against the unoptimized
  compositor and compare all affected visual cases. Consider geometry
  reuse and ramp fast paths only after measuring this change.
- [x] **Retain bounded cache entries across route palettes.** `scene.rs`
  previously removed different palettes/sizes on every lookup. Reference
  dashboard foreground #ef3333 differs from store/mail #de2e2e, so each
  return rebuilds the dashboard (1.323–1.376 s at 1600×900), even on repeated
  trips. Key entries by all rasterization inputs, retain recent routes
  within a memory budget, and test palette/size/context isolation and
  eviction. The mismatch exists in the committed baseline too.
- [x] **Stop dashboard ticks at its last actual motion.** Neomil's reveal
  ends at 360 ms, but its 16 ms subscription continued until global REST
  at 2400 ms. Derive the dashboard's completion time from its motion tree;
  preserve other eras, frozen clocks and any repeat semantics.
- [x] **Remeasure combined changes and preserve fidelity.** Serialize
  release benchmarks and builds; repeat the same three sizes and route /
  resize sequences. Compare current pixels with the pre-optimization
  staged artwork at rest, opening and interaction states. Run meaningful
  Rust regressions, visual gates and the repository check. Record measured
  4K time, memory, limits and any remaining work here before completion.
- [ ] **Live desktop verification remains separate.** Check actual
  hardware-GPU first presentation, resize and navigation when authorized
  to launch the app. Headless draw timing does not close this item.
- [x] **Profile remaining image upload/presentation work.** The optimized
  headless 4K app still spends about 1.60 s between its first foreground
  callback and the next backdrop callback, outside the measured CPU
  compositor. Sampled software-GPU process peak remains 3.60 GiB. Locate
  upload/atlas allocation and presentation costs before choosing a fix;
  trimmed image bands with transparent padding are a candidate, but must
  preserve fractional sampling and separate-layer semantics. Do not
  attribute the whole interval to a driver or infer live GPU performance.

Implementation is authorized for this round; changes remain staged, with
no commit, push or deployment. Work is split across disjoint compositor,
cache and panel-table files; benchmark/build execution is serialized.

## Implemented results (2026-09-21, staged)

The accepted change preserves all eight original image boundaries and
their GPU filtering, clips and paint order. One band worker accumulates
the float prefix once and emits only missing original images; Neomil now
needs 15 material-group walks rather than 44. Mask buffers use conservative
absolute pixel bounds, including transformed curves, stroke extent and
fractional coverage. Regression tests compare with the uncropped walker
and independent per-image composites, including actual 4K maker pixels.

The cache retains 384 MiB of RGBA payload with a 128-entry LRU cap, enough
for the eight 4K dashboard layers and both sibling grounds (316.4 MiB).
Keys include palette, dimensions, exact raster scale, primitive slice
identity/length and ordered prefix context. Oversized images bypass
retention; batches exceeding the retention budget use the individual
draw path. Opening layers are prepared together even behind an empty
clip, but their visible clip and image order remain unchanged.

| Canvas | Before median | After median (three-trial range) | Speedup |
|---|---:|---:|---:|
| 1600×900 | 1.3656 s | 0.0883 s (0.0861–0.1081) | 15.46× |
| 2560×1440 | 5.5745 s | 0.1703 s (0.1613–0.2012) | 32.72× |
| 3840×2160 | 36.6112 s | 0.4043 s (0.3912–0.4621) | 90.56× |

These are the same empty-cache CPU preparation workload, with the after
harness calling the new batch preparation before image/cut lookups just
as `Backdrop::draw` does. They are not full-frame GPU or presentation
times. Observed compositor-process peak RSS at 4K falls from 1014.9 to
517.0 MiB. Raw cached dashboard RGBA stays 253.1 MiB: preserving the image
boundaries is deliberate, and the retention budget excludes GPU copies,
live canvas handles and temporary compositor buffers.

Repeated 4K dashboard returns after store/mail take 0.0023–0.0044 ms in
cache preparation/traversal, without recompositing. A 3856×2169 resize
still costs 0.448 s; returning to 3840×2160 costs 0.380 s because two full
4K dashboard sizes do not fit the cache together. At 1600×900 the tested
old size remains cached. Interactive resize remains a follow-up: measure
debouncing or a scaled cached preview before selecting a behavior change.

229 Rust tests pass. Seven full-render comparisons have zero changed
pixels against the pre-optimization staged artwork: native and fractional
rest/opening, 4K rest/opening, and the fractionally positioned interaction
preview that rejected the image merge. No golden images were refreshed
for these optimizations. The Neomil implementation fidelity gate passes;
the full repository check passes all 19 checks, including all 25 golden
cases. Live hardware-GPU checks remain separate.

Natural-clock headless app confirmation (one run per size): the first
backdrop callback falls from 1476.6 to 103.5 ms at 1600×900, and from
37564.4 to 457.8 ms at 4K. At 1600 the scene now produces four draw pairs
rather than 43, showing intermediate opening positions and then stopping
at 368 ms. The 4K run still skips intermediate reveal frames; cold work
and the later rendering interval exceed its 360 ms duration. Both runs
become CPU-idle. These callback times exclude upload/presentation. The
software-GPU 4K process settles at 1.24 GiB RSS, so the compositor's peak
memory reduction does not imply an equivalent full-application reduction.


## Empty-band upload investigation (2026-09-27, verified)

A CPU profile identified row copies in Iced's atlas upload path after
compositing, followed by software-renderer work. The cache now omits only
all-zero RGBA bands, preserving every nonempty band's geometry, pixels,
filtering and original material-layer ordering. At 4K this reduces retained
payload from 253.1 to 82.4 MiB (256 to 83 bands). Three serialized runs give
a median post-foreground callback interval of 1.817→0.511 s and process
peak RSS 3723→988 MiB. The first CPU backdrop callback changes from 491 to 527 ms;
scanning for empty bands has a small cost. These are headless software-GPU
measurements, not live presentation or GPU frame rates.

Native/fractional rest and opening comparisons are pixel-identical. A
three-pane fractional interaction preview has one reproducible pixel
whose red channel differs by one level; Astra review accepted it as
negligible quantization, with the cause not proven. A new 4K size still
costs 512 ms, but returning to the old size is now 0.0049 ms because both sizes
fit in the unchanged cache budget. Continuous resize and live hardware
verification remain separate. See [upload measurements](../docs/neomil/image-upload.md)
for the complete method, ranges and limitations. The integrated H snapshot
passes 263 Rust tests and all 22 repository checks, including 27 visual
cases at 100.000% on their first attempt. Live hardware presentation and
continuous resize are not covered by that checkpoint.

## Continuous new-size characterization (2026-09-29)

- [x] **Characterize repeated size changes after empty-band removal.**
  Three serialized 41-size sequences confirm recent-size retention and
  bounded payload. Across 108 complete misses, median preparation is
  422.06 ms; 12 full hits take 0.00376 ms. Peak retained RGBA is 383.905 MiB
  and 46 entries, within the existing budget. Each reversal reuses four
  recent sizes, then a partial size, then returns to misses. Equivalent
  fresh-cache controls preserve batching. No production code changed.
- [x] **Implement and evaluate a bounded resize preview.** A resting,
  unheld Neomil dashboard reuses a complete cached material set within
  10% scale and a 200 ms quiet deadline. Palette/context checks, separate
  layers, opening clips and the 384 MiB budget are preserved. Real headless
  compositor events exercise it; an independent deterministic preview is
  visually reviewed. Final integer pixels match the exact control, while
  a fractional resized control differs at three pixels by one level. One
  sequence still incurs a 404 ms intermediate preparation; this does not
  establish smooth continuous resize. T passes 272 Rust tests, all 22
  repository checks and all 27 visual cases exactly. Live hardware/event
  presentation remains in the separate desktop verification task.
- [x] **Keep eligible resize preparation off the UI thread after the
  quiet timer expires.** The 3600×2025 trace first reuses the complete
  3840×2160 preview, then clears the deadline and synchronously prepares
  the target for 404 ms. The ratio is valid and retained payload is below
  budget; eviction is not the cause. Another new-size draw precedes its
  resize event, so the actual canvas bounds must participate in target
  tracking. AA uses one worker, latest-target coalescing, owner/generation
  checks and atomic complete-set publication. Tests cover stale completion,
  missing preview, palette/scale changes and failure fallback. The actual
  no-heartbeat compositor sequence redraws the exact snapshot before its
  screenshot signal. Full preview callbacks take .015–.033 ms, with no
  eligible synchronous preparation. CPU preparation itself is not faster;
  the shared cache stays at 384 MiB while one temporary job-local cache
  may retain an additional 384 MiB. Final 4K pixels match exactly;
  fractional repeat controls retain three one-red-level edge differences.
  AA passes 285 Rust tests, all 22 repository checks and all 27 exact visual
  cases. Live hardware presentation and smooth dragging remain separate.

See [method and results](../docs/neomil/resize-preparation.md). The original
N characterization added no artwork or golden change; AA completes the
eligible preparation offload. Live hardware verification remains separate.
