# Reference edge corrections — round forty-three (BO–BV)

This round tests the Entropism T2 foot and Neomil dashboard's inner-left
footer edge. BT integrates both accepted corrections after native state
review; combined production, package and repository verification passes.
The 29 broader TODO boxes remain open; these are bounded corrections
within E2 and NM1.

## Entropism T2 numeral

BO starts from AU's rejected source contour and changes four foot
vertices. It restores the leading and bottom rows that AU missed at
fractional size. Fresh actual-MailBox baselines reproduce production
RGBA at all three sizes, and the fixed T2 aperture matches AU's original
baseline. The accepted T and digits 1/3/4 remain unchanged.

The BO foot is too dark on 4K row 836: 36 pixels fall below green 100,
versus four in the source. BP changes only its two bottom vertices from
design offset +.25 to +.1875. The measured four-sample coverage staircase
predicts a medium-strength 4K row while preserving the recovered smaller
rows. Actual captures confirm the prediction: only 36 pixels change at
4K, to green 118; both smaller outputs are identical to BO. The source's
four darker pixels on that row remain unmatched.

| Native size | Original → BP whole-digit IoU, green <120 | Original → BP whole RGB MAE | Changed RGBA pixels |
| --- | ---: | ---: | ---: |
| 3840×2160 | .623350 → .926205 | 34.68901 → 16.64938 | 508 |
| 1600×900 | .554913 → .867769 | 31.73704 → 13.40741 | 146 |
| 1537×947 | .744828 → .899083 | 24.05882 → 14.00413 | 120 |

Whole-digit comparisons improve at green 100/120/140 at all sizes; all
fixed-region RGB errors improve. Fractional foot IoU at green 140 retains
a small loss, .940→.914894. Its final two rows each have 14 green<120
pixels versus source 15. The separately labeled affine-Bicubic sensitivity
also retains fractional upper/foot losses. These do not disappear into a
whole-digit average. No alpha or pixels outside T2 change; six removed
4K font-fringe pixels lie just left of the fixed whole-digit score box.

AU's primary method samples the mapped whole source aperture once using
local Lanczos, then slices its upper, sweep and foot controls. The gap
is sampled separately. Per-region resizing gives the same threshold masks
at all sizes, but one fractional sweep RGB pixel differs, so it must not
silently replace the established RGB control. Astra independently
reproduces all 360 native regional metric sets.

The matching SVG replaces only the painted digit with the same 27-point
contour; the complete T2 text remains nonpainting. Its component specimen
uses the existing (+284,+478) translation. All fixed SVG regional RGB and
green-threshold comparisons improve or tie at three sizes. Full/half
component renders change only the digit, with unchanged alpha. SVG
fractional comparison uses a uniform top-left viewport; native comparison
uses the responsive badge anchor and uniform glyph scale.

Nine paired actual-MailBox state cases pass in BQ. Rest, first/last row
selection, synthetic unselected badge material and custom selected and
unselected colors each change the same 120 digit pixels. Row-selection
signed RGBA deltas equal rest; custom role colors remain exact. Hidden art
is identical. Upper/lower opening clips reveal 28/102 changed pixels,
ending at rows 353/364 versus rest 366. Alpha and all other glyphs/screen
pixels stay unchanged. Astra independently verifies these results and
accepts BP for integration. BT ports the exact contour to the era table
and matching SVG/component drawings. Integrated verification passes below.

## Neomil dashboard inner-left footer edge

BP's fixed source corridors expose continuous missing coverage at 4K
x2904 on rows [2077,2131), between the accepted BF top and AZ bottom
joins. The existing x2902–2903 core and aligned right edge must remain.
The x2901 false core is a separate residual; BB's whole-frame translation
already failed independent controls.

One SVG strip starts at design (1210,865+1/2.4), has width 1/2.4 and
height 22.5. BQ extends BP's first strip by two source rows because the
bottom band begins farther right and cannot fill that small left join.
Against the original SVG, exactly 54/23/22 pixels change at 4K/1600/1537.
Every changed pixel improves RGB error, all 45 fixed regional RGB and
135 threshold-F1 comparisons improve or tie, and alpha/exterior pixels
stay unchanged. Astra independently verifies the combined result.

Three actual-Dashboard baseline captures reproduce frozen production
RGBA exactly. The full foreground-role trial changes 54/23/22 pixels and
improves every changed pixel's RGB error, but fails five 1600 R>170
controls: the three straight left windows and both joins. Middle/lower
F1 falls 1.000→.667 because the new shoulder becomes false bright core.
Astra independently reproduces all 270 regional metric sets. The full
native fill is rejected. BR calibrates one constant .88 foreground-role
opacity with geometry and SVG fixed. It improves or ties all 45 regional
RGB and 135 threshold-F1 controls at the three sizes. Every changed pixel
improves RGB error: 54/23/22 pixels, with zero alpha/exterior changes.
The 1600 shoulder stays below R170 while the fractional missing core
remains above R190. This does not fix the outer-left x2901 false core,
exact edge ink or secondary striations.

Eight BS pairs cover first/last module updates, synthetic held feedback,
custom colors, custom-held, opening at .12s, no-config fallback and a
Kitsch negative control. First/last/held whole frames equal rest because
these Neomil settings have no distinct visible feedback; they establish
artwork locality after update calls, not pointer behavior. Opening changes
the panel but preserves the footer delta. Custom colors reroute the strip
to foreground cyan; custom-held equals custom. The fallback exactly
matches Neomil rest, and Kitsch stays pixel-identical. Astra reproduces
all pair deltas and limits the state claims accordingly. BT integrates
the strip and the separately calibrated SVG/component drawing.

## Neo-kitsch frame audit

BO independently confirms six ridges and the accepted turn direction on
ordinary card 1, the selected card and ordinary 3/4 holdouts. The fifth
bottom's source red contrast is about 36 ordinary and 66 selected, versus
native 19; its five-row area has a smaller relative deficit. Selected
side contrast is weaker than ordinary despite the much stronger selected
bottom. Peak, area, dark gap, orientation and state therefore cannot be
resolved by one opacity adjustment. Prior material trials remain rejected.

Selected middle bottom echoes are still 1–3 source pixels high, with
ordinary bottoms within one pixel. A separate side/turn/bottom geometry
map remains actionable; the material audit does not block all of NK-14.
No frame artwork changes.

## BT integrated verification

Seven fresh production captures match the reviewed candidates at every
RGBA pixel: MailBox and Dashboard at 3840×2160, 1600×900 and 1537×947,
plus the no-config dashboard at 1600. Fresh packaged 4K captures of both
screens also match. The packaged FreeFont notices remain exact.

All 294 local and packaged Rust tests, four affected fidelity gates and
24 SVG structure checks pass. The full repository check passes all 22
checks and 27 visual cases on their first attempt. Direct pixel review
finds 26 exact cases and the unchanged historical one-level Neo-kitsch
bar pixel. Only three goldens change: Entropism mailbox by 146 pixels,
Neomil and fallback dashboards by 23 each. Alpha stays unchanged. All
317 frozen files match the tested 16 MB Nix source; nine original photos
remain unchanged. Final documentation does not change tested artwork.

## BU next-lane diagnostics

The selected Neo-kitsch third echo is high in native at all three sizes.
One SVG trial lowers its bottom by 1.2 design pixels, but SVG has a
different baseline phase: its 1600 peak already matches source y702,
and the trial moves it to 703. It also worsens the 1600 lower bend and
fractional turn, while the existing filtered halo spreads changes beyond
the frozen corridor. Astra reproduces all 42 regional metric sets and
20 local losses. The exact SVG trial is rejected; the separately prepared
native hypothesis is subsequently rejected in BV: it overshoots the
1600 ridge and worsens fractional turn controls. See
[round forty-four](reference-svg-round44.md). No frame change is integrated.

A frozen ordinary Kitsch M contour opens the source's central notch and
improves all nine whole-line RGB comparisons. Its font-to-vector control
itself changes 127/30/30 pixels; small-size M/A gap and threshold losses
remain. Two 1600 boundary pixels lie outside the fixed M window, and
the suffix control begins after the partially covered A boundary pixel.
Astra reproduces 135 regional and 405 threshold comparisons and keeps
the result diagnostic. BV then tests a hidden-M `tspan` while restoring
the original M outline. It alters the A/suffix at all three sizes and
ordinary cards, including 1714/1711/953 suffix pixels at 4K. Root's full
RGBA review confirms all 18 control/candidate suffix comparisons fail,
while alpha, selected text and screen exterior stay exact. That transfer
method is rejected. A shaping-preserving replacement remains necessary;
no Kitsch artwork is integrated.

## Remaining work and evidence

The 29-box audit distinguishes 17 boxes containing further source-based
work from six toolkit boxes needing callers or transition design, two
exact-background boxes needing assets/provenance, and four live-only
verification boxes. Some fidelity boxes overlap; they are not 17
independent fixes. No graphical session was available at the preflight.
The open source work is not collectively blocked by unknown font identity.

Scratch evidence under `/tmp/cp-eras-next/`: `bo-entropism-foot/`,
`bo-entropism-foot-method/`, `bp-t2-bottom/`, `bp-entropism-svg/`,
`bq-t2-states/`, `bo-neokitsch-frame/`, `bo-remaining-audit/`,
`bp-neomil-footer/`, `bp-neomil-footer-svg/`, `bq-neomil-footer-svg/`
and `bq-neomil-footer-native/`, `br-neomil-footer-native/`,
`br-neomil-footer-native-review/`, `bs-neomil-footer-states/` and
`bt-validation-plan/`, `bu-neokitsch-frame-svg/`, `bu-kitsch-m-svg/` and
`bv-kitsch-m-transfer/`. Original photos are unchanged; prior staged work
is preserved. No commit, push or deployment.
