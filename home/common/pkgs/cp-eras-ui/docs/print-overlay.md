# Print overlay prototype, 2026-11-13

Owner decision 6 (`docs/review-2026-11-13/DECISIONS.md`): one shared
print-texture pass per era instead of element-by-element echo and grain
fitting. This records what the reference surface measures as, what the
prototype draws, and its limits. Code: `src/print.rs` (era-side data and
CPU mirror), `src/eras/{neomil,entropism}.rs` `PRINT`,
`vendor/iced_wgpu/src/print.rs` + `shader/print.wgsl` (the pass;
`vendor/iced_wgpu/PATCHES.md`). Prototype for the owner to judge; the
goldens were not refreshed.

## What the references show

All eight references are 3840x2160 lossless exports, not photographs:
the dashboard's flat red tiles are exactly `#ef3333` with zero variance.
The "printed" look is designed, and it is not uniform across the screen.
Measured on the native files (design px = native / 2.4):

| Effect | Neomil | Entropism |
|---|---|---|
| Faint copies of bright ink | everywhere; displaced **outward from the screen centre** in proportion to distance: a whole-frame scale of 0.50-0.55 % (dashboard, store), 0.8 % (login), second peak near 2x (copies at k and 2k; matches the earlier per-element dashboard fits, e.g. JHN codes -3.6/+3.6 dp at x 45/1548) | same geometry, correlation peak 0.6 % on all four screens; visible ghosts on isolated stems sit farther (~0.8-1 %) |
| Rows of the copies | modulated at 4.764 native = 1.985 dp, depth ~55-60 % (amplitude/mean of the echo residual) | banded at ~8 native = 3.3 dp, depth ~20-30 %; plus a dot/moire structure not modelled |
| Fill scanlines | **not global**: clear ground 0.01 levels, dashboard tiles 0, mail selected row 0; present only on some fills (login active card +-3.5 on 246, store VIDEO +-6.5 on 197, GO HOME panel +-3.5 on 32), already drawn per element for the panel and badges | none on fills or ground |
| Grain | ~1.1-1.7 levels per channel on dark ground, ~0 on bright fills | sage fills RGB std 4.4/6.0/9.0 on 167/212/171 (~2.3 % of Y, chromatic, correlation ~1-2 native px); dark ground 0.7 |
| Softness / bloom | copies soft (~1-2 native px) | copies soft; clear bloom around light text and fills |

Vertical copy offsets are ambiguous in every fit (the copies' scanlines
comb the correlation at the 4.76 px pitch), so the scale is isotropic.

## The pass

Per pixel, in encoded sRGB: two copies of the frame scaled about the
centre by `1+2k` then `1+k`, each a 4-tap box (softness), lighten-composited
at alpha x row cosine x a fill gate (encoded luminance on a 6 dp ring: a
copy is suppressed inside light fills so dark ink stays firm); then an
optional darkening scanline multiply; then bilinear value-noise grain,
multiplicative plus additive. Pitches are in design px converted per axis,
snapped to whole device pixels below 3 px (2 px at 1600x900: no moire).
Zero strengths skip the pass and the frame is byte-identical.

Fitted values are the `PRINT` constants. Neomil: k 0.0055, alpha
0.24/0.14, copy rows depth 0.6 at 1.985 dp, soft 1.5 dp, no fill
scanlines, grain 1 level. Entropism: k 0.008, alpha 0.32/0.18, depth 0.3 at
3.33 dp, soft 2 dp, grain 2.8 % + 0.75 levels at 0.8 dp cells.

Tuning knobs (read once at start): `CP_ERAS_UI_PRINT=<strength>`,
`CP_ERAS_UI_PRINT_SET="key=v;key=a,b"` (`Print::with_overrides`).

## Limits

- No bloom (needs a separable blur, i.e. more passes). Entropism text
  regions stay darker than the reference (mail body mean G 38 vs 46).
- The pass cannot tell per-element copies from its own: Neomil dashboard
  (GO HOME body, labels, header, margin and brand copies), store titles and
  login notice copies are now drawn twice.
- Entropism mean colour rises 1-3.6 levels (G) against a reference ~1-2
  above the goldens: its frame strokes were already brighter than the art.
- Lavapipe cost: +~45 ms at 1600x900, +~350 ms at 3840x2160 per frame
  (13 taps/pixel); real-GPU cost unmeasured.
- `Renderer::screenshot` and the tiny-skia backend do not run the pass.
