# Reference SVG round sixty-nine — ordinary Neomil Store frames

The ordinary primary frame geometry is corrected independently of its
remaining stroke coverage and faint printing. The selected frame, card
contents, original images, fonts, text caches and shared renderer are
unchanged. The [source plan](neomil/store-ordinary-frame.md) retains the
pre-correction measurements and acceptance requirements.

## Source constraint and bounded correction

FF captures the current packaged Store at 4K. FG's repeated transverse
profiles expose ordinary tops at rendered row 361 versus source row 370;
the selected top already aligns. Ordinary bottoms and per-card side axes
also differ. Card 3 has an additional SVG/native disagreement: its former
SVG upper-right x1378 is x1380 in Rust. Moving whole cards would disturb
the independently calibrated contents.

FH measures primary straight supports and diagonal joins before rendering.
Astra independently reproduces 663 source run sets and eighteen line fits
across thresholds 160/180/200. The ordinary card-3 chamfer and fan fits are
card 1's shifted exactly 1577 physical pixels. Thin lower stems are sampled
after the filled bar; earlier fan-interior profiles cannot identify them.
The fourth-card axis uses its repeated two-pixel profile shared with card 3,
not a single threshold's truncated strong support. Sampling and unknown
source antialiasing leave uncertainty; measured bright-pixel centers are
not asserted to be exact vector vertices.

One frozen geometry proposal changes six SVG elements: ordinary card-1/3
face and bar paths, plus card 4's rectangle and open contour. Tops become
y154.5, bottoms y611.9; lefts become x440.0, x1097.1 and x1425.4. The fourth
right cut remains x1557. Selected art, text, icons, rifles, stats, hit bounds
and material ramp coordinates stay fixed. All primary stroke widths remain
1.2 to isolate geometry. The component's ordinary sample and descriptions
are synchronized.

Rust replaces the shared ordinary frame/bar coordinates with separate
card-1/3 constants and updates matching weak-fill masks. It preserves the
single face underlay, primitive slots, feedback helpers and material cache
structure. The number of drawing operations and cached layers is unchanged;
this round makes no new timing claim.

## Source, SVG and native comparison

Before the candidate, thirty fixed regions cover complete edges, corners,
fan joins, contents, selected art and the cut exterior. Source sampling uses
uniform top-left alignment at 3840×2160, 1600×900 and 1537×947, retaining
metadata and excluding unknown padding. Overlapping regions are diagnostics;
the table uses their union once rather than double-counting pixels.

| Viewport | SVG RGB RMS, before → after | Native RGB RMS, before → after |
| --- | ---: | ---: |
| 3840×2160 | 38.692 → 22.562 | 38.634 → 24.982 |
| 1600×900 | 29.779 → 18.266 | 31.890 → 20.429 |
| 1537×947 | 30.008 → 17.764 | 34.311 → 21.157 |

All 75 affected SVG region/size comparisons improve in both L1 and RMS.
Native improves all 75 in RMS and 74 in L1. The exception is the fourth
left edge at 1600: L1 rises 205184→216102 (+5.3%), while RMS falls
24.021→22.119. Its top, bottom and corners improve, and its left region
improves both measures at the other sizes. That local raster-coverage loss
is retained explicitly. The coherent position correction is accepted;
neither metric establishes complete edge fidelity.

Native changes 69875 / 13561 / 13064 pixels at 4K / 1600 / fractional size;
SVG changes 72445 / 14356 / 13229. All changes stay inside the fixed ordinary
frame regions. Selected art, contents, cut exterior and alpha remain exact.
Astra reviews source/SVG/native corner and join crops at all three sizes.

Sixteen paired native cases cover the three resting sizes, all four
selections, ordinary/selected/fourth-card hover and held feedback, selected
pointer exit, and zero/early opening. The inactive harness reproduces the
accepted packaged 4K frame and 1600 golden exactly. Every pair preserves
selected contents and the fourth cut; opening at zero is entirely unchanged.
These are synthetic Pointer snapshots through production drawing and Store
selection logic, not live desktop event replay.

## Remaining coverage and next rendering check

FM verifies fourteen source straight profiles using fixed primary bands
and two independently declared flank-background choices. Ordinary top
effective red area is .643–.685 design units under encoded unmix and
.417–.456 under per-pixel linear-light unmix. Left/bottom evidence also
shows excess current coverage, while right-side printing and asymmetric
ground make direct width inference less reliable. Astra reproduces all
fifty-six background/color-space cases, including raw and clamped values.
These support a bounded thinning study; they do not uniquely recover a
physical stroke width, blur or source color pipeline.

The primary coverage task therefore remains open, separately from faint
echoes. A prospective split between the filled frame/body stroke and top
rule must first isolate any cap, corner, fill or batching changes with an
unchanged-width replay. Keep the one-face underlay and primitive indices;
held feedback must suppress the new top ink while retaining its geometry.
No opacity sweep, inferred font replacement or new effect layer follows
from these measurements.

## Repository verification

Both Store inventory gates pass, with G2i using the actual newly built
package. The full check passes all 22 sections and 28 matrix cases on the
first attempt. Packaged tests pass 260 library and 45 other tests (305 total).
Direct comparison finds 26 of 27 visual outputs exact; the existing one-pixel,
one-level Neo-kitsch bar difference remains unchanged. The remaining case
validates source sampling. All 351 frozen crate files match the tested Nix
source. Fresh packaged Store captures are byte- and RGBA-exact to the reviewed
candidate at all three sizes. Final changes after verification are limited
to the six linked documentation records; runtime/artwork, fonts and originals
remain identical to that tested source.

The geometry task is complete. Primary stroke coverage remains one open
scope alongside the 29 broader source/toolkit/asset/live scopes. No commit,
push or activation was performed; all changes remain staged.

Evidence: `/tmp/cp-eras-next/ff-neomil-store-frame/` through
`fp-neomil-frame-integration/`; FJ contains Astra's independent scores,
input checks and state verification. FQ is a separate unrendered next-step
proposal and is not part of this accepted geometry change.
