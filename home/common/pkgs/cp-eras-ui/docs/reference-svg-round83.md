# Round eighty-three — Kitsch Dashboard header outline reuse

The three small header-note strings appear twice in the Kitsch Dashboard.
Six `Wide` tags now use the existing `ReusableWide` route, sharing identical
Rajdhani SemiBold outlines within one draw. Fonts, geometry, transforms,
ink, state behavior and the shared renderer are unchanged. The cache still
retains at most four entries, keyed by content, full font and exact size.
The preceding Kitsch Store reuse remains intact.

Separate 1600/4K counters verify six direct baseline traversals versus
three successful extractions followed by three hits. The three entries
retain 30, 29 and 16 paths, including character/space positions. Counter
images match the corresponding uninstrumented resting images exactly.

One frozen warm comparison uses four fresh processes in baseline/candidate/
candidate/baseline order, 200 warmups and 256 samples per case, with a
105-second settle and a complete-sample gate. It measures actual
uninstrumented Dashboard `Scene.draw` foreground CPU construction:

| Case | Baseline ms | Candidate ms | Reduction |
| --- | ---: | ---: | ---: |
| 4K rest | 16.072 | 14.275 | 11.2% |
| 4K hover | 16.041 | 14.295 | 10.9% |
| 1600 rest | 15.750 | 13.805 | 12.3% |
| 1600 hover | 15.776 | 13.849 | 12.2% |

Both candidate process medians are below both baselines in every case.
The later baseline is about 0.4–0.5 ms slower than the first; that drift
remains in the raw report and does not erase the process-range separation.
Astra independently reproduces all 4,096 raw samples and statistics.
Four processes do not establish population confidence. GPU presentation,
startup, backdrop preparation and live input latency are outside this
measurement. No persistent cache or additional paint pass is added.

Seventeen image pairs are byte-exact: three ordinary sizes, opening,
another era, no-config fallback, ten interaction/palette controls and
tiny-skia. Ordinary baselines also match the preceding package, and 1600
matches its golden. Seven interaction controls differ visibly from rest,
covering ordinary hover/held, changed selection and selected held. Opening
is also nonvacuous. Disabled, selected-hover and custom-palette controls
equal rest; the custom palette does not alter this frame's fixed inks and
does not establish changed-ink coverage. All 267 scratch library tests pass.

The first warm driver incorrectly selected module 0. Astra caught this
before timing; the compiled version is preserved and was never timed.
Version two corrects only that fixture to the production initial module 4.
No timing result is discarded or retuned.

All 312 packaged Rust tests (267 library and 45 integration), 22 repository
checks and 28 first-attempt matrix cases pass. Twenty-six of 27 visual
images match their goldens exactly; the prior Neo-kitsch bar residual is
the same one pixel at one channel level. The remaining case verifies
source sampling. All 368 frozen tracked crate files match the tested Nix
source. Fresh packaged Dashboard captures reproduce the reviewed candidate
at 4K, 1600 and fractional size. No goldens or reference SVGs change.
Changes remain staged and uncommitted.

LN evidence is in `/tmp/cp-eras-next/ln-kitsch-dashboard-cache/`;
independent checks are in `lq-dashboard-cache-review/`, and integration
evidence in `lr-dashboard-cache-integration/`, under the same scratch root.

The source-only Neo-kitsch frame study rejects exact donor-background
identity but passes an independently frozen interval check on 3,200 new
clear pixels. Conditional excess profiles expose ink in old gap/tail
controls; they do not identify a stroke width or glow model. See the
[frame study](neokitsch/store-frame.md#lolp-background-intervals-for-lower-frame-profiles-2026-10-04).
The 29 broader source, toolkit, asset and live scopes remain open.
