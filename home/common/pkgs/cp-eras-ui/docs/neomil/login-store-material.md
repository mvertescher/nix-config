# Login and store ground correction, 2026-09-21

Sources: `images/img-06-private.png` (login, source #59) and
`images/img-09-store.png` (store, source #62), both 3840×2160. Coordinates
below use the 1600×900 design unless explicitly marked native.

## Shared ground

Both traces previously used a short separable blue glow and warm washes.
The source carries a broader cyan field, especially below the header and
between the store cards. The measured dashboard ground already describes
this fixed source asset; it is reused without changing any control colors.

Login, dashboard, mailbox and store have **identical native RGB bytes** in
three independently clear rectangles: (500,20)–(600,65),
(30,700)–(100,755), and (1475,690)–(1530,755), totaling 68,688 pixels per
screen. The last rectangle is below the store's fourth card. A right-side
patch at y360–430 that was clear in the other screens overlaps this card
and must not be used to establish store background identity.

The SVGs now carry the same twelve horizontal RGB ramps and eleven
full-frame blend masks as the dashboard. Their x controls are
0,50,100,200,300,400,600,800,1000,1200,1400,1500,1550,1600; y controls are
0,80,160,240,300,360,420,480,560,680,800,900. Rust reuses the unchanged
`dashboard_ground::BACKGROUND` slice in each screen's existing `Soft`
backdrop. The complete opaque base and full-frame masks preserve the
previously tested absence of fractional-scale strip seams.

These are coarse reconstructed composite colors, not a recovered shader,
alpha stack or grain asset. Byte identity in clear patches does not prove
identity behind opaque cards. No bitmap, arbitrary noise or screen-specific
card field is added to the ground.

## Native validation

The before and after candidates are actual 3840×2160 SVG renders of the
isolated old and replacement ground. Geometry-only masks exclude all
screen-specific header/card/nav/text/margin/footer regions. The following
rectangles are united before measuring, so overlap is counted once.

Login:

```text
(0,0)–(1600,60)        (440,68)–(1100,180)
(85,205)–(1510,298)    (90,310)–(350,820)
(1260,310)–(1480,820)  (90,730)–(1480,900)
```

Store:

```text
(0,0)–(700,60)        (1040,0)–(1600,140)
(385,155)–(420,815)   (728,155)–(750,810)
(1058,155)–(1082,810) (1385,155)–(1410,810)
(90,620)–(360,820)    (435,640)–(735,810)
(1060,640)–(1510,810) (90,812)–(1510,835)
```

| Clear-background metric | Login before | Login after | Store before | Store after |
|---|---:|---:|---:|---:|
| Native pixels | 4,264,020 | 4,264,020 | 2,262,312 | 2,262,312 |
| RGB RMS, 0–255 scale | 9.049 | 1.685 | 7.285 | 1.736 |
| Channel MAE, R/G/B | 4.72/4.73/7.47 | 0.99/1.37/1.44 | 4.48/3.79/4.96 | 1.01/1.32/1.41 |
| Channel 95th-percentile error | 9/17/34 | 2/4/4 | 11/11/19 | 2/4/4 |
| Pixels differing by >8 in any channel | 28.927% | 0.012% | 26.759% | 0.115% |

No model coefficient was fitted on these two screens. As an additional
spatial check, each mask was partitioned into two disjoint holdouts by
`(native_x // 89 + native_y // 55) % 2`. The unchanged model was scored on
both, rather than retrained between partitions:

| Holdout | Native pixels | Before RGB RMS | After RGB RMS |
|---|---:|---:|---:|
| Login A | 2,131,708 | 9.042 | 1.684 |
| Login B | 2,132,312 | 9.056 | 1.686 |
| Store A | 1,128,697 | 7.308 | 1.733 |
| Store B | 1,133,615 | 7.263 | 1.739 |

The source texture is fixed and shared, so these holdouts are not independent
noise realizations. They check transfer of the existing field to untouched
locations. Full native source/trace comparisons were also inspected;
large errors inside opaque cards remain and are outside these ground scores.
Broad shape-gate passes must not be presented as detailed source fidelity.

## Selected store lower corner

The source selected card has an approximately 18px bottom-right chamfer,
with primary bottom edge at y797.1. The previous outer SVG path ended
`V800 H769 Z`, creating a square corner three pixels too low. It now ends
`V779.5 L1021,797.1 H769 Z`. This preserves its existing x1039 stepped
vertical edge; the measured source centerline is approximately x1038.5.
The Rust selected-card contour uses the same design vertices. The source's
fainter offset outlines remain separate, unimplemented printing work.

## Remaining material work

This change closes the shared-ground discrepancy only. Login inactive-card
fields, badge fields and scan modulation still require their own source
fits. The active login card's source flat red is already supported; a new
strong gradient would not be justified. Store ordinary-card blue/cyan
fields, selected-card and navigation fields/scan modulation, and footer
material remain independent tasks. Their opaque fills still hide the
corrected ground underneath. Primary printing, portraits, weapons and
brand artwork are likewise not validated by the clear-ground metrics.

The integrated Iced rendering, shape gates and repository checks are
recorded below. Native SVG renders alone do not establish live startup
or input correctness.

## Integrated headless checks

The first batch passes249 Rust tests, including eleven new input/gesture,
selection/crop and geometry regressions. Native and1537×947 captures show
no changed pixels outside the password field for empty versus80-character
synthetic input. Rest versus long input likewise preserves all other
artwork. Login rest/hover/held/disabled previews are reviewed.

Store live startup is pixel-identical to frozen rest. Both fourth-selected
and held captures preserve every pixel of the margin past x1557; the same
holds at fractional scale. Selected lower-detail feedback, corrected outer
corner, shaped upper wash, zero-time and mid-opening clips are reviewed.
Unit hit probes confirm(900,700) targets selected card2 and(1580,350)
targets nothing. Hits retain existing rectangular Plate semantics within
the current selected bounds and permanent viewport.

| Gate | Original → SVG | SVG → Iced |
|---|---|---|
| Login |15/34 shapes,78% source area, PASS|23/32 shapes,95% trace area, PASS|
| Store |12/43 shapes,61% source area, PASS|19/29 shapes,93% trace area, PASS|

The lower store source-area match compared with the71% baseline was
reviewed: the SVG change replaces only the background and selected corner;
other primary artwork remains unchanged. Changing the ground changes the
extractor's palette and shape grouping, including weapon/header fragments.
The independent clear-ground measurements establish this pass's material
improvement. None of these gate percentages closes the remaining artwork,
printing or card-material work. No thresholds were changed.

The full repository check passes all 19 checks, including all 27 visual
cases at 100.000% similarity. The new unfrozen `storeLive.neomil` case
matches the same store golden as frozen rest. Only the Neomil login/store
goldens were refreshed in this batch; the prior staged dashboard and
mailbox work is preserved. Changes remain staged and uncommitted. Live
desktop input, IME and actual authentication remain separate checks.
