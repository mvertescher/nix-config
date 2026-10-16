# Round eighty-one — Dashboard caption outline reuse

2026-10-04. The six SemiBold footer-caption runs now use `ReusableWide`.
Only that opt-in route admits SemiBold alongside Bold. Each caption shares
its local glyph outlines with its two echoes; font bytes, geometry,
transforms, ink, opacity and paint order remain unchanged. Ordinary `Wide`
and the existing Store title and Bold code callers retain their behavior.

The four-entry, per-draw cache still keys content, full font identity and
exact scaled-size bits. Separate actual-app counters at 1600 and 4K
verify three successful extractions and six hits: thirteen Bold code
paths, nineteen SemiBold Combat paths and fifteen SemiBold Defence paths,
retained in three entries. The preceding baseline used one code entry
and six direct caption calls. There are no failed extractions, added paint
passes, persistent caches or changes to the cache key/bounds.

A single predeclared release protocol measures actual warm foreground
`Scene::draw` construction, with 200 warmup calls and 256 samples per case
in each of four fresh candidate/baseline/baseline/candidate processes.
Case order is 4K hover/rest, then 1600 hover/rest. Values below are medians
of the two process medians per variant; the baseline already includes
[round eighty](reference-svg-round80.md)'s Bold code reuse.

| Case | Baseline ms | Caption reuse ms | Reduction |
| --- | ---: | ---: | ---: |
| 3840×2160 rest | 3.835 | 2.312 | 39.7% |
| 3840×2160 hover | 3.934 | 2.414 | 38.6% |
| 1600×900 rest | 3.541 | 2.154 | 39.2% |
| 1600×900 hover | 3.638 | 2.254 | 38.0% |

Both candidate process medians are below both baselines for every case.
Astra independently verifies all 4,096 raw samples, per-process statistics,
source, driver and binary hashes. Timed scene code has no counters. The
measurement excludes GPU submission/presentation, startup and background
preparation; these examples do not continuously redraw while idle.

Ten Wgpu state/size pairs and one tiny-skia 1600 pair remain pixel-exact.
Custom foreground, hover and held fixtures produce distinct states; their
disabled control equals the actual app. Counter builds also preserve
pixels at both sizes. All three ordinary-size baselines match the preceding
package, and 1600 matches the current golden. All 266 scratch library
tests pass. A permanent regression additionally checks that identical
text/size runs retain distinct Bold and SemiBold glyph paths on cache hits.
The primitive documentation now describes its actual opt-in semantics.

All 312 packaged Rust tests (267 library and 45 integration), 22 repository
checks and 28 first-attempt matrix cases pass, including the new font-weight
regression. Twenty-six of 27 visual images match their goldens pixel-for-pixel;
the existing Neo-kitsch bar residual remains one pixel at one channel level.
The other matrix case verifies source sampling. All 366 frozen tracked crate
files match the tested Nix source. Fresh packaged Dashboard captures match
the reviewed candidate exactly at all three sizes. No golden or reference
SVG changes. Changes remain staged and uncommitted.

LC evidence is in `/tmp/cp-eras-next/lc-footer-caption-cache/`, including
the initial test-command filename error and pre-execution harness repairs.
The successful test run is `test-v2.log`; timing uses the pinned corrected
runner with the unchanged protocol and driver. Independent checks are in
`ld-caption-cache-review/` and integration evidence in
`lf-caption-cache-integration/`, under the same scratch root.
This efficiency change does not resolve source-font identity, the remaining
[footer printing](neomil/dashboard-footer.md), or live GPU/input checks.
The 29 broader source, toolkit, asset and live scopes remain open.
