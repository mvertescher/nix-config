# Round eighty-two — Kitsch Store band outline reuse

The repeated PETROCHEM and BETTERLIFE TEC band labels rebuilt identical
outlines across card and clipped-layer traversals. Four `Wide` tags in the
ordinary/grown card definitions now use the existing `ReusableWide` route.
Fonts, geometry, transforms, ink, state behavior and the shared renderer
are unchanged. The cache remains local to one draw, with four entries
maximum and a content/full-font/exact-size key.

At rest and with the fourth card selected, separate 1600/4K counters verify
18 baseline direct traversals versus two successful extractions and sixteen
hits. Nine PETROCHEM paths and fourteen BETTERLIFE paths occupy two entries.
At the sampled opening state, six traversals become two misses and four
hits. These count clipped work, not distinct visible copies. Counter pairs
are pixel-exact; resting counter images also match uninstrumented captures.

One frozen warm comparison uses four fresh processes in baseline/candidate/
candidate/baseline order, 200 warmups and 256 samples for each case, with
a fixed 105-second settle and a complete-sample gate. It measures actual
uninstrumented Store `Scene.draw` foreground CPU construction:

| Case | Baseline ms | Candidate ms | Reduction |
| --- | ---: | ---: | ---: |
| 4K rest | 20.377 | 16.554 | 18.8% |
| 4K hover | 20.390 | 16.530 | 18.9% |
| 1600 rest | 20.576 | 16.364 | 20.5% |
| 1600 hover | 20.577 | 16.359 | 20.5% |

Both candidate process medians are below both baselines in every case.
Astra independently reproduces all 4,096 raw samples and statistics.
Four processes do not establish population confidence. GPU presentation,
startup, backdrop preparation and live input latency are outside this
measurement. No persistent outline cache or additional paint pass is added.

Eighteen image pairs are byte-exact: three ordinary sizes, twelve state
controls, tiny-skia, another era and no-config fallback. The ordinary
baselines also match the preceding package, and 1600 matches its golden.
Ten state controls differ meaningfully from rest, covering all selections,
ordinary/fourth hover and held, selected hover, opening and custom colors.
The disabled and selected-held controls equal rest; neither supplies
independent state evidence. The first selection hook failed to reach the
actual application closure and was stopped after vacuous captures. Its
evidence is preserved; the corrected asserted hook supplies these results.
All 267 scratch library tests pass.

All 312 packaged Rust tests (267 library and 45 integration), 22 repository
checks and 28 first-attempt matrix cases pass. Twenty-six of 27 visual
images match their goldens exactly; the prior Neo-kitsch bar residual is
the same one pixel at one channel level. The remaining case verifies
source sampling. All 367 frozen tracked crate files match the tested Nix
source. Fresh packaged Store captures reproduce the reviewed candidate at
4K, 1600 and fractional size. No goldens or reference SVGs change.
Changes remain staged and uncommitted.

LG evidence is in `/tmp/cp-eras-next/lg-kitsch-band-outline-cache/`;
independent checks are in `lk-kitsch-cache-review/`, and integration evidence
in `ll-kitsch-cache-integration/`, under the same scratch root.

The source-only LH/LI Dashboard footer studies constrain text displacement.
LJ's one position-only SVG trial improves aggregate error at three sizes,
but loses bright glyph edges and local controls and remains rejected. See
the [footer study](neomil/dashboard-footer.md#lhli-source-text-displacement-2026-10-04).
The 29 broader source, toolkit, asset and live scopes remain open.
