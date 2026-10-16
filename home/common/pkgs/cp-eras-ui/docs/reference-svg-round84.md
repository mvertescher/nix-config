# Round eighty-four — Entropism Dashboard caption outline reuse

The Entropism Dashboard now shares the three repeated SemiBold caption
outlines across its six modules. Three macro declarations use the existing
`ReusableWide` route, preserving every field. The Neomil Store primary-title
trial remains scratch-only: it preserves pixels and reduces extractions,
but its fixed timing comparison does not establish a useful 4K gain.
Neither its helper change nor its test-only correction is integrated.

Font bytes, weights, layout, clipping, effects, inks and the shared renderer
are unchanged. The cache still lives for one draw, retains at most four
entries and keys on complete text, full font and physical size. No global
cache, paint pass or source-art correction is introduced.

Actual Entropism counters at 1600 and 4K replace eighteen direct caption
traversals with three successful misses and fifteen hits. The three entries
retain 12, 19 and 15 paths, including space positions. Neomil's baseline
has four direct primary-title extractions plus one echo-cache miss and
sixty hits at both sizes. The candidate has one miss and sixty-four hits,
retaining the same single ten-path entry. Partial opening changes
four direct runs plus one miss/thirty-nine hits to one miss/forty-three
hits. Instrumented images equal the uninstrumented images exactly.

Each screen has one separately frozen baseline/candidate/candidate/baseline
comparison, using four fresh processes, 200 warmups, 256 samples per case,
and a 105-second settle. The driver follows its actual screen's selection,
layers, state and cursor behavior. These results measure uninstrumented
warm foreground `Scene.draw` CPU construction:

| Screen and case | Baseline ms | Candidate ms | Reduction |
| --- | ---: | ---: | ---: |
| Entropism Dashboard 4k/rest | 11.912 | 6.930 | 41.8% |
| Entropism Dashboard 4k/hover | 11.920 | 6.919 | 42.0% |
| Entropism Dashboard 1600/rest | 11.445 | 6.553 | 42.7% |
| Entropism Dashboard 1600/hover | 11.424 | 6.552 | 42.6% |
| Neomil Store 4k-rest | 19.296 | 19.177 | 0.6% |
| Neomil Store 4k-hover | 20.195 | 20.304 | -0.5% |
| Neomil Store 1600-rest | 20.257 | 19.531 | 3.6% |
| Neomil Store 1600-hover | 21.269 | 20.632 | 3.0% |

Both Entropism candidate process medians are below both baselines in every
case. Neomil's 4K process ranges overlap: rest improves only 0.6% and hover
regresses 0.5%, while 1600 improves 3.0–3.6%. Fewer traversals alone are
insufficient to claim a useful 4K improvement. The complete mixed result
is retained without a timing retry; only Entropism is accepted.

Astra independently checks all 4,096 raw samples per screen. Four processes
do not establish population confidence. GPU presentation, startup,
backdrop preparation, geometry drop and live input latency are outside the
measurement. Percentages are against the same preceding runtime; they
must not be multiplied with earlier optimizations.

Entropism has twenty-one exact image pairs, including three ordinary sizes,
opening, another era, fallback, fourteen state/palette controls and
tiny-skia. Eleven controls visibly differ from rest. Disabled, selected
module 2 and selected-hover 2 are no-ops; ordinary hover 0 equals selecting
0. A glyph-only aperture confirms nonzero partial caption visibility.
Counter images match uninstrumented rest, and 267 scratch library tests
pass. Baselines match the prior package and the 1600 golden.

Neomil has twenty-two exact pairs: three ordinary sizes, partial opening,
another era, default/unknown-era control, fifteen state/palette controls
and tiny-skia. Thirteen controls differ from same-size rest; disabled and
the default selected card are no-ops. The default/unknown-era capture also
equals Neomil rest and does not establish a distinct-screen comparison.
Actual packaged baselines match all three sizes and the 1600 golden.
The selected-card title has no ordinary echoes: its fixed aperture contains
120 bright pixels during opening versus 866 at rest, and none below the
opening clip. This verifies visible primary glyph clipping, not merely
a changed background. The repaired existing test suite passes all 267
tests. Custom colors and held feedback are visibly exercised.

Harness mistakes were corrected before their affected work ran. The
Entropism compiler guard used the wrong manifest key. Neomil initially
referenced unrelated Dashboard images; its corrected script uses fresh
packaged Store images. Counter-only code received an explicit Rust type,
and manifest-loading order was corrected before compilation. The first
Neomil test run passed 266 tests and failed one stale primitive-tag
assertion. A test-only overlay updates that tag and preserves its exact
held-ink assertion; production binaries remain unchanged. The failed log,
frozen originals and additive repairs remain available. No timing result
was discarded or retuned.

All 312 packaged Rust tests (267 library and 45 integration), 22 repository
checks and 28 first-attempt matrix cases pass. Twenty-six of 27 visual
images match goldens exactly; the prior Neo-kitsch bar residual remains
one pixel at one channel level. The other case verifies source sampling.
All 369 frozen tracked crate files match the tested Nix source. Three fresh
packaged Entropism captures reproduce the reviewed candidate at 4K,
1600 and fractional size. Font files, reference SVGs and goldens remain
unchanged. Changes stay staged and uncommitted.

Scratch evidence under `/tmp/cp-eras-next/`: `lt-entropism-dashboard-cache/`
and `lv-neomil-primary-title-cache/`; independent Astra checks in
`lu-entropism-cache-review/` and `lw-neomil-title-cache-review/`; combined
integration in `lx-outline-integration/`. The 29 broader source, toolkit,
asset and live scopes remain open.
