# Printing on veneer: ink, tab, copies

Measured 2026-11-16 at 3840x2160 against `images/neokitsch-{mail,store}.png`,
our renders with `CP_ERAS_UI_PRINT=0` unless stated. Answers the second
fidelity review's selected-mail-row and SMG gaps
(`docs/review-2026-11-16/neokitsch.json`).

## Ink: `VENEER_PRINT #69432d`

Stroke interiors (pixels darker than the fill/ink midpoint, eroded by one
pixel so the photo's edge blur drops out), median colour:

| Mark | Reference | Ours until 2026-11-16 | Now |
| --- | --- | --- | --- |
| "Urgent information (!)" | `#6a4530` (luma 74) | `#7b5438` (88) | `#69432d` (72) |
| "FROM: MOM" | `#6d462e` (75) | `#895f3b` (97) | `#69432d` (72) |
| envelope (thin strokes) | `#774f33` (83) | `#865c39` (94) | `#69432d` (72) |
| SMG label | `#66412a` (70) | `#3a2410` (37) | `#69432d` (72) |

Why the two were pale: the mail row's three inks were fixed colours
sampled where strokes do not reach their plateau, and all three are
lighter than the glyph shader's dark-ink window (`cryoglyph::ink::OFF_Y`
0.085; `#7b5438` is Y 0.108), so they were drawn exactly as specified with
no coverage correction. Nothing was hiding or blending the text; the
colours were wrong. The mail title's stem widths already matched (FWHM
4.0-5.6 native px reference, 3.6-5.4 ours).

SMG's ink was darker than the reference, not lighter: its label was
Regular, stems 3.1 px against the reference's 4.3, so at 1600x900 the
thin dark strokes blurred to grey. Now Medium at 17.5 (stems 3.9;
SemiBold gives 5.0), 0.8 left and 1.2 up to the reference's caps
(`NAV_SELECTED_*`).

The selected store card's body keeps `ON_VENEER #3a2410`: the reference's
is `#63411e`, but our Regular body is lighter than its strokes (ink mass
2775 against 4017 px at the midpoint threshold), and the darker ink is
what holds it. Moving it to `VENEER_PRINT` wants a heavier face with it.

## Tab: `VENEER_TAB`

The reference's selected mail row and SMG button carry the same tab: a
light tab hanging from the plate's foot inside a dark rim that flares into
the foot. Not a dark notch (what the mail row drew) and not a flat patch
(SMG).

| | Mail row | SMG | Shared value |
| --- | --- | --- | --- |
| tab x | 421.0-456.0 | 241.25-275.75 | width 35 / 34.5 |
| rise above foot | 3.1 | 2.7 | 3.0 |
| hang below foot | 2.1 | 1.4 | per use |
| rim width (FWHM) | 3.0 px | 3.6 px | 1.4 |
| rim colour (peak, blurred) | `#6b4b29` | `#5c421d` | `VENEER_PRINT` |
| tab fill | `#f1bb73` mean, `#f7c175` median | `#fed492` | per use |

Corners r1.2; the rim's foot reaches about 4 native px out along the edge,
a fillet r1.7. `crate::style::VeneerTab` draws it: the tab, then the rim as
a ring (its hole is the tab's outline above the foot).

The rule tab above the selected row (each row's) was re-measured too: top
at 304.4, 30.7 wide at the top, sides falling 0.37 per unit of height
(`RuleTab`, `inset` 1.7 over 4.6), where the 0.7 slope drawn before made
it narrower at the top.

## Copies over the text: present in the reference, kept

The review saw "a faint doubled copy" of the row's dark text. The print
pass's near copy (0.60 %, about 7.6 px left and 1.6 px up here) lays the
fill over ink whose copy source is fill. Signature: mean luma of ink
interiors whose pixel at (+8, +2) is fill, minus those whose source is
ink, in the title box.

| Image | (+8, +2) | control (-8, -2) |
| --- | --- | --- |
| reference | +6.9 | -0.5 |
| ours, print on | +1.3 | -0.6 |
| ours, `echo_gate=0.45,0.60` | +0.2 | |

The reference's peak sits at the copy's offset (+2.3 at (5, 1), +0.5 at
(11, 2)); the SMG label shows it too (+5.9 at (8, 0)). So the reference
carries the copy over dark text on veneer, more strongly than ours.

The enclosure gate does treat veneer differently from a flat fill: its
grain lines (encoded luma 0.61 at their cores against a 0.75 base) break
the "thick light" test at `echo_gate (0.60, 0.75)`, so the copies are only
partly suppressed there; lowering the gate to (0.45, 0.60) suppresses
them. That would move away from the reference, so the gate is unchanged.
With the darker ink the copy reads as the reference's does.
