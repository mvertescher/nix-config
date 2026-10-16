# Font implementation and text-effect audit

Requested 2026-10-02 before further individual glyph corrections. Audit
earlier changes as well as current candidates. Existing accepted artwork
stays in place while the font and rendering foundations are checked.

- [x] **Verify actual font selection.** Inventory shipped and installed
  font bytes, names, versions, weights and character coverage. Check all
  production loaders and selectors, SVG Fontconfig resolution, and actual
  shaped glyph font IDs/bytes for every supported face, including the
  named Kitsch derivative and CJK fallback. Matching a family name alone
  is insufficient. Separate implementation correctness from identifying
  the original source font.
  DX verifies all 24 SVGs, 140 shaped Latin cases, eleven loaded Latin
  faces under three shaping policies in permanent tests, and the limits
  of host-dependent CJK fallback. See the [audit](../docs/font-rendering-audit.md).
- [x] **Audit text semantics and prior corrections.** Compare SVG and
  native shaping, spacing, anchors, scaling, baselines and raster paths.
  Review full-run versus character/word/prefix drawing and the earlier
  contour, font and echo corrections. Distinguish verified intentional
  calibration from a workaround for incorrect selection or shaping.
  Replacements must preserve unaffected glyphs and advances; require exact
  no-op replay before accepting another split or substituted glyph.
  Fifteen two-scale Iced cases agree between host and isolated fonts.
  Production sample placement differences are below 0.000153 pixel;
  kerning-sensitive Orbitron controls expose a latent split-painter limit.
  The named derivative changes only M; no source-font identity is inferred.
  Those advance/shaping comparisons did not establish the general Rajdhani
  baseline conversion. JN's later direct outline probe measures 0.892 ×
  size, while the scene subtracts the retained 0.84 calibration. Its local
  rotated PETROCHEM consequence and correction are tracked in the open
  [Store printing task](neomil-store.md), with details in the
  [baseline audit](../docs/font-rendering-audit.md#shaping-and-earlier-corrections).
  Do not treat the earlier checked audit as approval of a global baseline
  replacement or of every source-calibrated label. Round 77 integrates
  the local correction and a permanent CPU outline-baseline test, with
  source/state/package parity and all 311 tests/22 checks/28 matrix cases
  passing. Broader photographic printing remains in the Store task.
- [x] **Measure text/effect costs.** Count actual shaping, outline,
  opacity/clip and raster-effect work in current Dashboard, Store, Mailbox
  and Login paths at 1600 and 4K, with rest, opening and feedback controls.
  Measure first-draw and warm behavior, cache keys/invalidation/bounds and retained
  data. Separate CPU geometry from backdrop preparation, GPU upload and
  live presentation. Use measured costs to choose optimizations.
  DX's four release variants each complete 280 draw groups: Neomil's four
  screens at both sizes and four states, plus Kitsch/Neo-kitsch 1600 rest.
  A focused Store profile separates prefix measurement, character paint,
  Wide text, outline lookup and cached-path fill. Larger coverage and live
  GPU timing remain outside this evidence.
- [x] **Resolve audit findings and verify.** Fix demonstrated font or
  rendering defects and avoidable material costs within their actual
  scope. Any optimization must preserve reviewed pixels, palette roles,
  responsive placement, clipping and interaction states. Run meaningful
  regressions, the relevant visual gates and full repository checks for
  integrated changes. Record unresolved source-font identity and live
  hardware limits explicitly; do not close them with headless evidence.
  DX adds font regression coverage and skips unused Start-anchor widths
  (Neomil Store 385 to 312 calls; no measured draw-time improvement).
  Eleven native before/after pairs and both rest-wrapper controls are
  exact; all four ordinary Neomil instrumentation controls are exact too.
  DY adds a bounded per-draw advance cache, preserving existing shaped
  values and raster/outline routes. Kitsch Store drops from 1,257 to 299
  prefix measurements and 25.34 to 20.22 ms of warm foreground CPU work
  at 1600 (20.2%). Neomil Store's observed 4K change is smaller,
  22.97 to 22.56 ms. Eleven cache-specific native pairs and both wrapper
  controls are RGBA-exact. All 298 local/packaged Rust tests, 253 isolated
  library tests, 22 repository checks and 27 visual cases pass. Twenty-six
  packaged frames are exact; the known one-pixel Neo-kitsch bar residual
  is unchanged. The remaining measured paint cost is tracked separately
  below. These timings establish warm CPU redraw behavior; first-draw
  samples vary and do not establish startup or live GPU improvements.

## Measured follow-up

- [x] **Profile and optimize the remaining Neomil tracked-character painter.** The
  measured 4K Store bucket is about 6.3 ms and includes all 48 tracked
  runs; only nine are known to be rotated. Separate rotated outline
  fallback, ordinary raster text and deferred geometry work before
  choosing a change. Preserve pixels, palette roles, clipping and
  rest/opening/feedback states. The existing title outline and login echo
  caches already reuse their expensive extraction; do not replace them
  without evidence. See the [audit measurements](../docs/font-rendering-audit.md).
  DZ identifies nine rotated runs (109 character calls) at about 6.2 ms
  versus 0.011 ms of ordinary enqueueing; explicit path filling is only
  4.86–6.34% of the rotated span. All eighteen no-op pixel comparisons on
  Wgpu and tiny-skia pass. The positioned-path cache reduces extraction
  from 109 to 40 character calls, with 4K rest warm CPU time falling from
  22.63 to 19.42 ms (14.2%) in a bracketed release comparison. Fourteen
  cache-specific native pairs, four wrapper controls and five additional
  Kitsch Dashboard pairs are RGBA-exact;
  258 library tests pass with host and isolated fonts. All 303 packaged
  Rust tests, 22 repository checks and 27 first-attempt visual cases pass.
  Fresh packaged 4K Store and fractional Kitsch Dashboard frames match the
  reviewed candidate exactly. Source images and goldens are unchanged.
  Case-first samples do not establish process-cold startup behavior. See the
  [tracked-text study](../docs/neomil/tracked-text-performance.md).

- [x] **Reuse the three identical Bold Dashboard footer-code outlines.**
  LA opts only those runs into the existing bounded `ReusableWide` route.
  Actual 1600/4K counters verify one successful extraction, two hits and
  thirteen retained paths in one entry. Ten Wgpu pairs and one tiny-skia
  pair preserve pixels; fonts, captions, transforms and inks are unchanged.
  The fixed longer-warmup confirmation improves foreground CPU construction
  4.410→3.828 ms at 4K rest (13.2%), with comparable hover gains and
  12.5–12.8% at 1600. The initial mixed short-warmup result remains recorded.
  All 311 packaged tests, 22 checks, 28 first-attempt matrix cases and
  three-size package parity pass. No goldens change; see
  [round eighty](../docs/reference-svg-round80.md).

- [x] **Reuse the two repeated SemiBold Dashboard footer captions.**
  LC enables SemiBold only for marked `ReusableWide` runs and changes six
  caption tags. Three successful entries and six hits preserve actual
  fonts, shapes, ink, opacity and transforms. A permanent regression keeps
  Bold/SemiBold glyph paths distinct on same-text/size cache hits. Ten Wgpu
  pairs, tiny-skia, both counter sizes and three packaged sizes are exact.
  Against the LA baseline, one fixed warm CPU protocol improves 4K rest
  3.835→2.312 ms (39.7%), hover 38.6%, and 1600 cases 38.0–39.2%.
  All 312 packaged tests, 22 checks and 28 first-attempt matrix cases pass;
  goldens and source SVGs are unchanged. See
  [round eighty-one](../docs/reference-svg-round81.md).

- [x] **Reuse repeated Kitsch Store band-label outlines.**
  LG changes four ordinary/grown label tags. The unchanged per-draw cache
  serves eighteen traversals with two successful extractions and sixteen
  hits; opening has two misses/four hits. Warm foreground CPU construction
  improves 20.377→16.554 ms at 4K rest (18.8%), 18.9% hover and 20.5% at
  1600. Eighteen image pairs, five counter pairs and three fresh packaged
  sizes are exact. All 312 packaged tests, 22 checks and 28 first-attempt
  matrix cases pass. No font, artwork or cache-policy change; see
  [round eighty-two](../docs/reference-svg-round82.md).

- [x] **Reuse repeated Kitsch Dashboard header-note outlines.**
  LN changes six tags for three repeated SemiBold strings. Six extractions
  become three misses/three hits in the existing bounded per-draw cache.
  Warm foreground CPU construction improves 16.072→14.275 ms at 4K rest
  (11.2%), 10.9% hover and 12.2–12.3% at 1600. Seventeen image pairs, both
  counter sizes and three fresh packaged sizes are exact. All 312 packaged
  tests, 22 checks and 28 first-attempt matrix cases pass. Fonts, renderer,
  artwork and cache policy are unchanged; see
  [round eighty-three](../docs/reference-svg-round83.md).

- [x] **Reuse repeated Entropism Dashboard captions.**
  Three caption declarations opt into the existing per-draw cache:
  eighteen direct runs become three misses and fifteen hits. Fonts and
  drawing parameters stay unchanged. Warm 4K rest foreground CPU
  construction improves 11.912→6.930 ms (41.8%). Twenty-one pixel pairs,
  exact state/backend/counter pixels, three packaged sizes, all 312 packaged
  tests, 22 checks and 28 first-attempt matrix cases pass. See
  [round eighty-four](../docs/reference-svg-round84.md) for evidence and limits.

- [x] **Evaluate Neomil primary-title reuse.** The frozen LV trial preserves
  pixels and eliminates four extractions, but shows no convincing 4K
  gain: rest improves 0.6%, hover regresses 0.5%, and process ranges overlap.
  Smaller-size gains are 3.0–3.6%. The helper change and its test-only
  correction remain unaccepted in scratch; no timing retry or retuning.
  See [round eighty-four](../docs/reference-svg-round84.md).

General kerning-sensitive split text needs a deliberate layout policy
before new callers rely on it. Future CJK text requires an explicit face,
supported subset and isolated coverage check. Neither requires moving
today's calibrated Latin labels. Follow the audit's future-inspection
rules before resuming individual glyph fitting.

EK's controlled vector-edge follow-up confirms native linear-light versus
SVG encoded-RGB blending within one channel level. This is an intentional
compiled pipeline difference, not evidence of a wrong font weight. Keep
it separate from outline/coverage/photographic residuals in future fits;
the probe adds no production work. See the
[blending check](../docs/font-rendering-audit.md#ek-vector-edge-blending-check).

The preceding DW experiments remain unaccepted in scratch:
Entropism punctuation, Kitsch SC registration and Neomil subtitle
registration. Their local measurements do not establish a production fix.
Astra owns verification and integration; scoped Sol lanes inspect selection,
semantics and effect costs. Rust builds, captures and timing runs are
serialized. Keep all changes staged and uncommitted.
