# Transparent image bands — 2026-09-27

The 4K Neomil dashboard currently produces eight separate software images.
The cache holds their horizontal RGBA bands, and Iced uploads each band when
the canvas names its handle. Profiling the software Vulkan run attributed
samples after compositing to `iced_wgpu` image atlas allocation and upload;
the roughly 1.6 s callback gap also includes other work, so it is not an
upload time measurement.

`SoftCache::keep` now discards a band only when **every RGBA byte is zero**.
It creates no Iced handle for that band and excludes its payload from the
cache byte count. Empty composites still get cache entries, so a warm lookup
does not rasterize them again. A retained band keeps its original full width,
absolute `y`, height, pixel bytes, draw order and linear filtering. The
backdrop uses the same clips and scale. This leaves the float prefix walk
and each layer's image boundary intact. A band with zero alpha but nonzero
RGB remains present as a conservative safeguard around filtering.

The maximum raw cache and upload reduction for one 3840-pixel-wide band is
`3840 × band height × 4` bytes. Total savings depend on the actual zero-band
count in each generated layer; an opaque ground saves nothing. The
`can_prepare` guard still uses the full-frame upper bound, so this change
does not increase the set of sizes eligible for batch preparation. Separate
measurements of retained payload, upload/presentation interval, first-frame
capture and software-GPU RSS are needed before claiming a performance gain.

The focused cache regressions cover an entirely empty image and warm reuse,
mixed empty and nonempty bands with preserved geometry and bytes, conservative
handling of nonzero RGB under zero alpha, and retained-byte accounting.
Existing tests cover preparation, cache eviction and original image cuts.

## Measured results

The native 4K scene still produces eight separate material layers. At the
cache boundary, 173 of its 256 horizontal bands contain only zero RGBA
bytes; 83 remain. No nonempty band is cropped or merged.

| Canvas | Original RGBA payload | Retained payload | Bands before → after |
| --- | ---: | ---: | ---: |
| 1600×900 | 43.9 MiB | 15.1 MiB | 256 → 86 |
| 2560×1440 | 112.5 MiB | 35.6 MiB | 256 → 81 |
| 3840×2160 | 253.1 MiB | 82.4 MiB | 256 → 83 |

Three serialized natural-clock 4K runs per variant used the same scene,
release settings and headless software Vulkan renderer. Scratch-only
instrumentation timed the backdrop/foreground callbacks and sampled
`/proc` memory every 100ms; instrumentation is absent from production.

| Measurement | Before median (range) | After median (range) |
| --- | ---: | ---: |
| First backdrop CPU callback | 491 ms (468–502) | 527 ms (512–563) |
| First foreground end → next backdrop start | 1817 ms (1816–1818) | 511 ms (509–516) |
| Process peak RSS | 3723 MiB (3722–3731) | 988 MiB (988–989) |

The later callback interval falls about 72%, with a 73% reduction in
peak process memory. Checking all output bytes adds a small CPU cost;
this optimization targets retained images and upload work, not fewer
compositor walks. `perf record` separately sampled the pre-change
process: post-compositor call chains include
`iced_wgpu::image::atlas::Atlas::upload_allocation` copying rows, followed
by llvmpipe rendering/JIT work. Sampled CPU time is not wall-clock wait,
and the entire callback gap must not be labelled image upload time.
These figures do not establish hardware-GPU first presentation or FPS.

A separate release cache harness measures 4K cold preparation at
445/445/526ms, with warm lookup/traversal medians about 0.0018ms.
Returning from store/mail takes 0.0022–0.0043ms. A fresh 3856×2169 size
costs 512ms, but returning to the original 3840×2160 now takes 0.0049ms:
the smaller retained payload lets both sizes and the sibling grounds fit
in the existing 384MiB budget. Continuous resizing to unseen dimensions
still needs interactive evaluation; this change adds no debounce or
scaled-preview behavior.

All six natural-clock 4K captures are pixel-identical. Native 1600×900
and fractional 1537×947 rest captures, plus opening captures at those
sizes and 3840×2160, also have zero changed pixels. The three-pane
fractionally positioned rest/hover/held preview differs at exactly one
pixel: its red channel changes from 38 to 39 at (978,168), with green
11 and blue 13 unchanged. Repeating both variants reproduces only that
same one-level difference. Astra review accepts this negligible
quantization difference; atlas repacking is a plausible explanation,
not demonstrated causation. No nonempty image bytes or sampling bounds
changed. This is substantially smaller than the earlier rejected image
merge (5,299 changed pixels, maximum channel difference 60).

The combined 260-test Rust run passes, including the three new cache
regressions. Full repository checks remain part of integrated acceptance.
Raw profiles and comparison files are temporary artifacts under
`/tmp/cp-eras-completion`, not repository assets.
