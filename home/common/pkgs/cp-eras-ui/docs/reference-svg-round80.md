# Round eighty — Dashboard footer outline reuse

2026-10-04. Three identical Bold footer-code runs now use the existing
`ReusableWide` path. One local glyph extraction serves the primary text
and its two echoes. The font, size, stretch, position, ink, opacity and
paint order stay exact. Captions and the shared renderer are unchanged.

The existing cache lasts one `Scene::draw` and retains at most four
entries, keyed by content, full font and exact scaled-size bits. Separate
counters at 1600 and 4K verify one successful extraction of thirteen paths,
two hits and one retained entry, versus three direct text calls. No
fallback occurs. This avoids repeated outline extraction; it does not
eliminate the three fills, add an effect pass or cache across frames.

The uninstrumented release comparison measures warm foreground
`Scene::draw` construction. The confirmation fixes 200 warmup draws and
256 samples per case, with fresh candidate/baseline/baseline/candidate
processes and reversed case order. Values below are medians of the two
process medians per variant.

| Case | Baseline ms | Reuse ms | Reduction |
| --- | ---: | ---: | ---: |
| 3840×2160 rest | 4.410 | 3.828 | 13.2% |
| 3840×2160 hover | 4.525 | 3.928 | 13.2% |
| 1600×900 rest | 4.050 | 3.532 | 12.8% |
| 1600×900 hover | 4.153 | 3.634 | 12.5% |

Both candidate process medians are below both baselines in each case.
The initial eight-warmup, 64-sample ABBA run is retained: 4K improved, but
1600 was mixed and hover worsened by 7.1%. Its 1600 rest baseline varied
4.082–4.959 ms. The single longer-warmup confirmation was frozen before
execution to investigate that drift; no candidate parameter changed.
Neither protocol measures startup, GPU work or live presentation, and the
examples do not continuously repaint these scenes while idle.

Astra independently verifies all 5,120 timed samples and source/driver
hashes. Ten Wgpu baseline/candidate pairs are RGBA-exact: three resting
sizes, fractional opening, fallback, another era, and disabled/custom/
hover/held fixtures. The disabled fixture equals the actual app; custom,
hover and held produce distinct states. Tiny-skia also matches at 1600.
Both counter variants preserve the uninstrumented captures at both sizes.
The current 1600 golden and round-79 packaged baseline match exactly.
All 266 scratch library tests pass.

All 311 packaged Rust tests, 22 repository checks and 28 first-attempt
matrix cases pass. Twenty-six of 27 visual images match their goldens
pixel-for-pixel; the existing Neo-kitsch bar difference remains one pixel
at one channel level. The remaining matrix case verifies source sampling.
All 365 frozen tracked crate files match the tested Nix source. Fresh
packaged Dashboard captures match the reviewed candidate exactly at all
three sizes. No golden or reference SVG changes. Changes remain staged
and uncommitted.

LA evidence, including the initial timing result and versioned probe
repairs, is in `/tmp/cp-eras-next/la-footer-outline-cache/`; independent
checks are in `ht-echo-review/la-*-verification.json` under the same scratch
root. LB integration evidence is in `lb-footer-cache-integration/`.
The separate [footer source studies](neomil/dashboard-footer.md#ky-shared-source-ground-2026-10-04)
validate shared ground and localized trailing ink, but do not identify its
paint mechanism. The 29 broader source, toolkit, asset and live scopes
remain open.
