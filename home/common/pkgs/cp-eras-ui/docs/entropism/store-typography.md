# Entropism store value typography

The four value runs `86 / 30 / 5 / 5` on each product card were too short and thin in the N store SVG and native render. At 3840×2160, the photographed values occupy y1135/1136..1173; the old 22px Rajdhani Regular runs occupy y1142..1175 in SVG and y1144..1177 in Iced. This fit changes only those values. The stat labels, compliance, sockets, art, frames, inks and literal content retain their existing forms.

The SVG values now use Rajdhani SemiBold at 24.5px with local baseline 229.1667. This yields y1136..1173 on the ordinary cards and y1136..1173 on the selected card, whose source top is one native pixel higher. In Rust, the corresponding `Prim::Wide` runs start at local y228.3334: the earlier N native capture placed the same Rajdhani run two native pixels below librsvg, so the Rust-only 0.8333-design-pixel lift is a starting calibration for native review. The plain card template serves cards 2–4, including the viewport-clipped fourth; the grown card serves card 1. Hover and pressed states inherit the geometry and retain their existing semantic ink inversions.

The two-digit widths are local fits. Ordinary `86` is centered at x41.4167 and stretched 0.97; ordinary `30` keeps x102 and width 1.0. Selected `86` and `30` are centered at x41.4167 and x102.4167, each stretched 1.07. Both `5`s retain x164/225 and width 1.0 in both states. At a dark R<100/G<120 threshold, selected `86` reaches x1177..1234 against source x1178..1235 and selected `30` reaches x1323..1380, matching the source. Ordinary card 3 `86` reaches x2725..2777 against source x2725..2778. The ordinary `30` retains a glyph-dependent 1–3px horizontal residual; shifting/compressing it only marginally helped card 3 and worsened held-out card 4.

Weights were compared **after** cap height and baseline were matched, using selected/card-3 `86/30` as training glyphs. Mean dark-mask IoU at R<100/G<120 is 0.291/0.391 for Regular, 0.381/0.531 for Medium, and 0.450/0.589 for SemiBold before width adjustments. The final local width fit raises SemiBold to 0.561/0.652. Held-out selected `5`s, ordinary card-3 `5`s and card-4 `86/30` score 0.599, 0.504 and 0.663 versus the old SVG's 0.178, 0.171 and 0.131. The SemiBold preference for the training digits holds at R thresholds 80/100/120 and when source/SVG red channels are normalized to comparable local ink alpha. The `5` holdouts weakly prefer Medium at the highest R threshold, so this is a bounded improvement, not an exact source-font identification.

Compliance stays Rajdhani Medium. A SemiBold compliance trial overprinted its bright core, while four available alternative sans families were much wider and taller before squashing. The photographed grain, soft glyph edges and remaining contour differences are outside this value geometry fit. Native Iced verification of the new value size/weight and baseline is pending.

## W native review

At the middle dark-mask threshold, actual Iced/source overlap rises from
.083 to .446 for selected two-digit runs and .113 to .602 for plain card 3.
Held-out selected/plain `5`s improve .120→.608 and .129→.511; card 4's
two-digit runs improve .109→.614. All groups improve at three thresholds.
Native review also finds the stretched runs one source pixel lower than
the SVG, while unstretched `30`/`5` baselines already agree. A scoped X
calibration is being checked before final integrated validation.

## X native baseline calibration

The W capture places stretched `86`/`30` caps at y1137..1174, one native pixel below their SVG boxes at y1136..1173. Unstretched ordinary `30` and the `5`s already occupy y1136..1173. Rust therefore lifts only runs whose horizontal stretch differs from 1.0 by 0.416667 design pixels (one pixel at 3840×2160); the SVG and all unstretched runs keep their positions. This leaves the selected source's y1135 top one pixel above the candidate, while matching the ordinary source top and the shared y1173 bottom.

A scratch translation of the W native dark masks tests the intended renderer-only shift before a fresh capture. At R<100/G<120, IoU for the selected `86` first glyph rises .481→.609. Independent selected `86` second glyph rises .437→.549, selected `30` .434→.529, ordinary card-3 `86` .590→.669, and ordinary card-4 `86` .619→.718. All stretched holdouts improve at R thresholds 80, 100, and 120. Ordinary `30` and selected/plain `5` masks remain unchanged. The scratch comparison and glyph crops are in `/tmp/cp-eras-resume-20260929/w-e-type/x-baseline-trial.{txt,png}`; X native verification is still required.

Actual X native captures confirm the calibration: at the middle threshold,
selected two-digit overlap rises again from .446 to .554, plain card 3
from .602 to .642, and held-out card 4 from .614 to .664. All three
thresholds improve; the unstretched `5` controls are unchanged. This
fixes raster baseline placement, without claiming exact font outlines.
