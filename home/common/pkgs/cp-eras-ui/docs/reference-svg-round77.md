# Round seventy-seven: PETROCHEM registration and native baseline

The first and third ordinary Store cards' PETROCHEM boxes and text are
registered to their measured source positions. The native glyph positions
also compensate for a verified Iced baseline offset. Inferred ordinary
card 2 inherits card 1's geometry in rest and feedback. Selected cards,
card 4 and the accepted BETTERLIFE placement are preserved.

## Source and font evidence

JF/JG establish complete source glyph apertures and persistent box rails.
JI derives one box/text correction from the rail midpoints: −0.75 design x
on card 1 and −8/3 on card 3. Both SVG contexts improve at three sizes.
The direct native transfer, JK, improves the boxes but worsens both 4K
glyph apertures. JM removes neighboring printing and confirms that card 1's
loss belongs to the glyph itself; that candidate is rejected.

JN probes the pinned Iced paragraph and actual canvas outline path. The
loaded Rajdhani Regular bytes match the bundled font. Auto, Basic and
Advanced agree at sizes 7.5/18, including an H control with font yMin=0:
the outline baseline is **0.892 × size**, while the scene subtracts its
retained **0.84** calibration. The centered 1.2-line formula independently
gives the same result from ascent/descent 930/−346. Clockwise rotation turns
the residual into **−0.936 physical x at 4K** for this word.

JO compensates with +0.39 design x on the two existing native glyph Turns,
relative to JI's SVG positions. This is one correction derived from engine
metrics, without fitting another position or changing the shared baseline.
The source-supported boxes remain fixed. Fresh native baselines exactly
match the round-76 package at all three sizes; source samples retain the
repository sampler's metadata and opaque-pixel controls.

| Viewport | Card 1 native glyph RGB L1 | Card 3 native glyph RGB L1 |
| --- | ---: | ---: |
| 3840×2160 | 153,944 → 144,061 | 148,242 → 125,353 |
| 1600×900 | 27,056 → 23,089 | 27,496 → 19,222 |
| 1537×947 | 23,844 → 21,812 | 21,142 → 19,477 |

Both complete glyph apertures and their upper/lower halves improve in
all-pixel L1 and RMS at every size. Native whole-context L1 improves by
26.1%/24.3% at 4K for cards 1/3. Every change stays inside the frozen
contexts, with exact exterior pixels and alpha.

Local losses remain. At 4K card 3's left-rail dark subset loses 2,224 L1;
at 1600 card 1's upper-glyph bright subset loses 152. Fractional card 1's
left/right bright rails lose 616/34, and card 3's bright glyph RMS worsens
61.849→64.988 despite its L1 improving. Smaller cap/band exceptions and
all per-pixel gains/losses remain in JO's frozen scores. SVG retains its
own fractional lower-glyph and dark-rail losses. This accepts registration,
not complete photographic ink, softness or repeated-print fidelity.

## Rendering and verification

Font bytes, face, weight, size, tracking, child glyph origins and active
primitive counts are unchanged. Outer Turn translations preserve the
positioned-outline cache keys; there is no new per-frame font measurement
or rendering effect. Existing calibrated labels prevent replacing the
shared baseline globally. Its comment now distinguishes calibration from
the measured paragraph baseline. See the [font audit](font-rendering-audit.md).

All 265 existing library tests pass, including recursive feedback geometry.
A permanent CPU outline test checks the measured Rajdhani baseline at
both sizes under all three shaping modes, so font or Iced changes cannot
silently invalidate this local calibration. It adds no runtime work.
Thirteen targeted headless cases pass: ordinary hover/held, selected
first/third cards, inferred ordinary second-card held, opening, custom
colors and disabled-hook parity with the actual Store. Selected printing,
card 4, exterior pixels and alpha stay exact. All 311 packaged Rust tests, 22 repository checks and 28 first-attempt
matrix cases pass. Twenty-six visual cases are pixel-exact; the existing
Neo-kitsch bar residual remains one pixel at one channel level, and the
remaining case checks source sampling. Three fresh actual-package Store
captures match JO byte-for-byte. Both Store inventory gates pass. Changes
remain staged and uncommitted. The first ordinary component specimen follows the corrected
SVG position; only the Neomil Store golden changes, by 610 pixels.

Evidence is retained under `/tmp/cp-eras-next/`: `jf-petrochem-primary-profile/`,
`jg-petrochem-source-aperture/`, `jh-petrochem-glyph-isolation/`,
`ji-petrochem-midpoint/`, rejected `jk-petrochem-native/v2/`,
`jm-petrochem-native-isolation/`, `jn-rajdhani-baseline/`,
`jo-petrochem-baseline-compensation/`, `jp-petrochem-states/`,
`jq-petrochem-integration/` and independent `ht-echo-review/` records.
The [Store printing task](../todo/neomil-store.md) remains open.
