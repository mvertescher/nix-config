# New-size preparation and cache retention — 2026-09-29

Three serialized release trials exercised the immutable N dashboard's
actual eight material groups through `SoftCache::prepare`, followed by the
same ordered `image`/`cut` lookups used by the backdrop. The machine exposes
32 hardware threads. Each trial used a fresh default 384 MiB cache, drew
3840×2160 through 3520×1980 in steps of −16×−9, then reversed through the
same sizes back to 3840×2160: 41 calls per trial, 123 overall. Each size was
completed before requesting the next; no display server or GPU was involved.

This measures wall-clock CPU-side preparation and cache traversal, not
interactive window latency, queued frames, presentation time, or FPS.
Material definitions and image boundaries are unchanged. It is a measured
follow-up to [empty-band removal](image-upload.md), not a new optimization.

| Requested layers already cached | Calls | Median | p95 | Range |
| --- | ---: | ---: | ---: | ---: |
| None of eight | 108 | 422.06 ms | 488.14 ms | 345.98–584.36 ms |
| Six of eight | 3 | 335.94 ms | 373.29 ms | 319.09–377.44 ms |
| All eight | 12 | 0.00376 ms | 0.00429 ms | 0.00324–0.00432 ms |

On each reversal, four recent full sizes (3536–3584 wide) remain cached;
3600 wide has six of eight layers, and subsequent older sizes miss again.
The partial-hit rows all concern that one size and are not a controlled
speedup comparison. Immediate redraws at representative sizes and five
final redraws also take microseconds, confirming the warm path independently.
New-size miss medians drift about 394→418→457 ms across trials, so small
inter-trial differences should not be interpreted as improvements.

Retention peaks at 402,553,472 bytes (383.905 MiB), below the unchanged
402,653,184-byte budget, with at most 46 entries and 753 inferred evictions.
Counts are sampled before and after preparation; for this single-threaded
caller, old entry count plus missing layer count minus new entry count
identifies evictions. The full working set fits the budget and the entry
cap never binds. Other runtime users could change this accounting.

A fresh **384 MiB** cache at 3520, 3680 and 3840 widths provides the cold
control, preserving batching. Its three-trial medians are 390.17, 455.45
and 511.89 ms respectively. A zero-budget cache would disable batch
preparation and would not be an equivalent control.

Process RSS after the three sequences is approximately 958, 984 and
1028 MiB; cumulative peak RSS reaches 1142 MiB. Those values include
allocator retention and temporary compositor allocations, not just cached
RGBA. The budget bounds retained image payload, not the whole process.
No GPU memory or software-renderer upload copies are measured here.

Recent-size reuse works as intended, but continuously requesting unseen
sizes still requires hundreds of milliseconds per completed preparation.
A larger cache would not make a first visit cheap. A future resize strategy
should evaluate latest-size coalescing and a bounded temporary preview,
then render the final size exactly. This experiment does not measure those
strategies or establish their visual/input behavior; actual GUI resize and
hardware presentation remain separate, permission-dependent checks.

The harness compiled against the immutable N release crate and copied its
scene/soft modules. Diagnostic access only exposes retained byte/entry and
current-size-key counts. The generator was checked to reproduce the final
measured harness exactly (SHA256
`d8e548aa13c98217d2fedd7d0f0db4c2697a37fbdc681ce48a94d1df87b4f816`).
Raw measurements contain each trial, step, dimensions, elapsed milliseconds,
retained count/bytes, hits, misses and evictions. No production code changed
for this experiment. Astra independently checked the method and accounting.

## Bounded resize preview prototype

The dashboard now opts into a separate backdrop canvas after the Neomil
opening has reached rest and no plate is held. Frozen-time captures and other
callers retain the existing exact backdrop. Actual `Window::Resized` canvas
events start or renew a 200 ms quiet deadline. A redraw requested at that deadline returns to exact
rendering at the current canvas size. An early redraw keeps the deadline;
changed palette or display-list identity, window rescale, or lost focus clears
the preview.

During the deadline, a preview may reuse one complete cached set of ordered
material layers at a nearby size (within 10% on each axis), only above
2560×1440. Every layer must match the full palette, source scale, source
dimensions, and preceding groups for a coverage cut. The source bands are
scaled only for this draw; no preview image or additional RGBA payload is
retained. A missing or partial set falls back to the normal exact path.
The foreground scene, hit testing, layer boundaries, and the 384 MiB cache
budget are unchanged.

An isolated headless Sway/Pixman session drove compositor resize commands
after the 3840×2160 resting dashboard was cached. Seven requested steps
were nominally 70 ms apart; actual delivered events were fewer and roughly
150 ms apart. The first 120 ms version expired during the sequence and
prepared an intermediate size for 376–433 ms. The 200 ms version trades
80 ms more quiet time before final exact preparation for a better chance
of coalescing those events. It does not guarantee that every intermediate
miss disappears:

| Final window | Preview draws | Cold exact draws during sequence | Final exact draw | Retained cache payload at capture |
| --- | ---: | --- | ---: | ---: |
| 3840×2160 | 7 | None | Warm, 0.016 ms | 86,446,080 bytes |
| 3733×2100 | 5 | 3600×2025: 404.003 ms; 3733×2100: 433.898 ms | Subsequent warm redraw, 0.016 ms | 244,162,080 bytes |

The integer final raw RGBA screenshot was byte-identical to a direct
frozen, exact 3840×2160 capture. At the fractional final size, the live
capture differed from a frozen control taken through the same resize
history at three pixels by at most one channel value. Against a direct
frozen 3733×2100 capture, it differed at two footer-text pixels by at
most ten values; those two pixels also differed in the same-history
exact control. This points toward capture or glyph history, but does not
prove the cause. A stale 3760×2115 configure arrived after the first
3733×2100 configure in that exact control, so the harness correctly
refused a wrong-size screenshot; a later compositor request for the final
size was required to finish it.

These are CPU-side backdrop draw times and final screenshots, not FPS,
drag latency, hardware presentation time, or a visual assessment of an
in-flight preview. The earlier event-driven preview screenshot was taken
after an exact target image had already been prepared and is not evidence
of preview fidelity. A separate deterministic harness then warmed the actual cache at
3840×2160, issued a fresh resize event to the actual preview canvas, and
captured its 3733×2100 draw alongside an exact-target control. The trace
confirms preview reuse of the complete old-size set. Astra reviewed the
full frame and material/text/edge crops: foreground alignment, layer order
and clipping remain intact. Temporary filtering changes 659,220 pixels,
mean absolute RGB difference .126 levels, with sparse edge differences up
to 125. Those preview pixels are intentionally approximate and are never
used as final-size goldens. This validates the preview image, not a claim
of smooth hardware presentation. The retained-byte figures count cached image payload, not
process RSS: the fractional run reported 1,372,088 kB RSS at capture and
2,443,984 kB peak RSS, including renderer, allocator, and capture work.
T passes 272 Rust tests and all 22 integrated repository checks, including
all 27 visual cases at 100.000% on their first attempt.

## AA asynchronous eligible preparation

The previous 404 ms post-quiet UI-thread preparation is replaced, when a
valid bounded preview exists, by one background worker with latest-target
coalescing. Rendering uses one temporary job-local cache; only the current
owner/context/scale/generation may publish a complete set atomically. Stale
work is discarded. Exact drawing clones a complete image set under one
cache lock, so a later eviction cannot force a second synchronous lookup.
Failure, invalid/no preview, held/frozen states and noneligible sizes keep
the exact fallback. The shared retained budget remains 384 MiB; one local
job can additionally retain up to 384 MiB, excluding compositor scratch
and renderer copies. No retained preview RGBA sits outside these caches.

A real compositor sequence without an app heartbeat discarded a stale
3600×2025 job (527.443 ms) and published the final 3733×2100 set (617.480 ms).
The completion poll drew exact pixels before the screenshot signal. Local
final payload was 80,632,800 bytes and shared payload 167,078,880 bytes.
The same-history synchronous exact control matches byte-for-byte, as does
the final 4K return. A fresh fractional launch has five history-dependent
pixels also present in the synchronous resize control; none is introduced
by the new worker. First-run missing compositor configuration and full
memory/timing limits are recorded in the
[twelfth checkpoint](../reference-svg-round12.md). Live hardware
presentation and drag/input latency remain separate.

The final trace includes the complete callback: scale/key construction,
cache lock/snapshot acquisition and image submission. In a repeat without
an application heartbeat, 21 preview callbacks take .015–.033 ms while
background jobs take 663.308/715.813 ms. No eligible resize draw enters
synchronous preparation. Shared retained payload is 245,389,280 bytes;
process RSS/peak are 1,351,052/2,546,816 kB. This repeat differs from the
earlier same-history exact control at three red-channel edge pixels by
one level, an explicit sampling limit like the earlier T resize result.
The two footer-history pixels agree. These callback numbers exclude
foreground drawing, image upload and presentation.
