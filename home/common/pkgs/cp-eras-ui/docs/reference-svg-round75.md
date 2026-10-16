# Round seventy-five: share Store contour distances

The frame impressions introduced in [round seventy-four](reference-svg-round74.md)
paint many concentric white strokes over the same paths. Each repeated stroke
previously flattened its path and recalculated distances independently. The
software renderer now recognizes consecutive identical fixed-white paths with
strictly decreasing finite widths and shares that work.

Each width retains its original row bounds, column spans, coverage tests and
float blending order. Center and sixteen subpixel distances are shared only
when the ordered active segment lists agree. Other rows use the literal
per-band traversal; unsupported runs and unsafe coordinates retain the old
stroke path. Small work vectors reuse capacity across rows. No retained
distance image, artwork change, cache-key change or font change is introduced.

Three new differential tests compare float bits and RGBA against the preserved
literal walker, including nested masks, clipped/rotated origins, differing
segment lists/spans, interrupted runs and nonfinite/large-coordinate fallbacks.
All 265 library tests pass. The scratch baseline and candidate match the prior
actual package byte-for-byte at 4K, 1600 and fractional sizes. Fourteen paired
headless cases also match exactly: selections, ordinary second-card material,
hover/held feedback, opening, custom colors and a disabled-hook control.

## Measured preparation cost

Twelve fresh processes use balanced ABBA/BAAB order, two processes per variant
and size. Only actual `SoftCache::prepare` CPU time is measured. Medians below
compare the current round-74 runtime with this change in the same session.

| Viewport | Initial preparation, ms | Subsequent twenty selections, ms |
| --- | ---: | ---: |
| 3840×2160 | 626.703 → 462.161 (26.3% less) | 3217.582 → 2456.071 (23.7% less) |
| 1600×900 | 152.831 → 73.426 (52.0% less) | 782.038 → 365.790 (53.2% less) |
| 1537×947 | 146.081 → 69.158 (52.7% less) | 751.645 → 357.090 (52.5% less) |

The twenty-selection totals exclude initial preparation. All four shelf hashes,
the complete cache-miss sequence, ten final cache entries and retained payload
match. Payload remains 201,338,880 / 34,899,200 / 31,656,052 bytes respectively.
Pooled warm cache-hit medians remain about 0.27–0.29 microseconds. Those calls
are not foreground drawing: no new GPU, presentation, warm-draw or startup
claim is made.
The fractional viewport rasterizes the 1600×900 design into 1537×865 pixels.

## Packaged verification

All 310 packaged Rust tests, 22 repository checks and 28 first-attempt matrix
cases pass. Twenty-six visual cases are pixel-exact; the existing Neo-kitsch
bar difference is still one pixel at one channel level. The remaining matrix
case checks source sampling. Fresh actual-package Store captures match the
reviewed prototype byte-for-byte at all three sizes; the packaged Store
implementation gate passes. Goldens, source images, SVG artwork, fonts and
text caches are unchanged by this round.

This closes the bounded IQ preparation task. Thirty broader source-fidelity,
caller, original-asset and live-session scopes remain open. In particular,
headless hooks do not close live interaction checks, and exact output parity
does not resolve the ordinary Store bottom's remaining source mismatch.

Evidence: `/tmp/cp-eras-next/ir-stroke-band-prototype/` (source/tests/rest),
`is-stroke-band-cost/` (twelve raw process logs), `it-stroke-band-states/`
(fourteen capture pairs), `iu-stroke-band-integration/` (package/checks) and
`ht-echo-review/` (independent Astra verification). IS preserves an unused
eight-repeat/microsecond harness; only its corrected 64-repeat/nanosecond
version was timed. Changes remain staged and uncommitted.
