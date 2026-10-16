# Round seventy-six: third-card BETTERLIFE placement

The ordinary third Store card's `BETTERLIFE TEC` word is moved 1.9 design
units left in SVG and Iced. First-card, inferred ordinary second-card and
selected printing retain their accepted positions. PETROCHEM and its box
stay fixed: the whole-label shift failed their 4K source controls.

## Source evidence and rejected trials

IX measures the two ordinary source box edges repeating at 1577 physical
pixels. Its initial word window accidentally included 150 pixels of the
third card's neighboring frame. IY corrects that measurement with paired,
boundary-clean supports: source BETTERLIFE repeats at about 1577 pixels,
while SVG/native repeat at about 1581–1582. Upper/lower halves agree.
The fixed −1.9 design-unit displacement follows the authored 659 versus
observed 657.1 spacing; no displacement sweep is used.

JA moves the whole side label and improves overall error, but its 4K
PETROCHEM box and glyph/interior regions worsen by 7,474 and 10,102 RGB L1.
That candidate is rejected. Independently, IZ's ordinary `Dim`→`Fg` box-ink
trial fails all 24 edge/cap regions at unchanged geometry. Neither failure
changes production ink, fonts or PETROCHEM placement.

JB isolates the complete BETTERLIFE word. JC ports the same placement to
its existing native `Turn`, including const hover/held drawings. Source
samples use the repository sampler with retained metadata and alpha255
controls. Fresh SVG baselines match round 74, and fresh native baselines
match the actual round-75 package, byte-for-byte at all three sizes.

| Viewport | SVG word RGB L1 | Native word RGB L1 |
| --- | ---: | ---: |
| 3840×2160 | 363,364 → 309,781 | 341,758 → 268,415 |
| 1600×900 | 60,019 → 48,090 | 58,697 → 44,632 |
| 1537×947 | 52,612 → 40,485 | 55,635 → 43,070 |

All eight fixed 20-pixel bands containing ink improve, including their
bright/dark subsets; the ninth is unchanged. This does not make every
pixel closer to the photograph: native losses remain at 322/70/53 pixels,
with worst mean RGB losses 86.33/85.00/59.33 levels. The fixed PETRO control
rectangle overlaps moved-word antialiasing: 49/22/32 native pixels change
there, and its dark subset loses 465/212/505 RGB L1 while its total improves.
SVG has the analogous overlap only at fractional size. The original scorer
assertion is retained; the repaired scorer reports the same frozen rectangle
without deleting failed pixels or changing artwork. PETRO primitives are
unchanged. All changed pixels stay within the declared BETTERLIFE support;
alpha, neighboring frame, first/selected printing and other exterior pixels
are exact.

## Fonts and rendering cost

The bundled face, weight, size, advances and tracking are unchanged. The
native correction changes the existing outer `Turn.x`; its child still
paints at local origin zero on the same quarter-turn path. Its tracked
outline cache key therefore remains unchanged. Const state lists retain
the same active primitive count. No new text effect, glyph measurement,
renderer/cache modification or performance claim is introduced. The
[font audit](font-rendering-audit.md) and earlier performance results stand.

## Verification

All 265 existing library tests pass, including the recursive test that
ordinary and selected feedback preserve each card's geometry and content.
Ten targeted headless states pass, including ordinary hover/held, selected
third-card rest/held, inferred ordinary second-card, custom colors, opening
and disabled-hook parity with the actual Store. Selected third-card frames
are exact; other changes stay within the word support. All 310 packaged Rust tests, 22 repository checks and 28 first-attempt
matrix cases pass. Twenty-six visual cases are pixel-exact; the existing
Neo-kitsch bar difference remains one pixel at one channel level, and the
remaining case checks source sampling. Fresh actual-package Store captures
match the reviewed candidate byte-for-byte at all three sizes. Both Store
inventory gates pass. Only the Neomil Store golden changes, by 251 pixels.
Changes remain staged and uncommitted.
The broader margin-printing task stays open; this corrects one independently
measured word's registration, not its complete photographic printing.

Evidence: `/tmp/cp-eras-next/ix-side-label-registration/`,
`iy-side-word-support/`, `iz-ordinary-box-ink/`, `ja-side-label-translation/`,
`jb-betterlife-translation/`, `jc-betterlife-native/`, `jd-betterlife-states/`,
`je-betterlife-integration/` and independent `ht-echo-review/` records under
the same parent. Original images and prior staged changes are preserved.
