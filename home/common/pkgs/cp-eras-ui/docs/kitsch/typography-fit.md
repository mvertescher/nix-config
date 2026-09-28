# Kitsch source printing fit

This pass compares the photographed login and store at 3840 × 2160 with
their SVG traces and the prior native Iced captures. `images/kitsch-store.png`
is the 4ST store, and `images/kitsch-login.png` is the access screen. Values
below describe the clean source glyph mask and the resulting SVG rendering;
they do not claim recovery of photographic glow or exact letter outlines.

The three `EMPTY / SOCKET` pairs in both card states have a native cap about
20 pixels high. The prior 9-design-pixel text rendered only 14 native pixels
high. Each source pair uses approximately 79 native pixels for `EMPTY` and
95 for `SOCKET`; the old SVG widths were 56 and 66. Rajdhani Semibold at
13 design pixels, with centers fitted separately at card-local x 82.75,
153.5, and 224.5, matches the source extents much more closely. In the first
cell, source/SVG mask intersection-over-union rises from 0.122 to 0.437
for plain `EMPTY`, 0.145 to 0.500 for plain `SOCKET`, 0.126 to 0.520 for
selected `EMPTY`, and 0.150 to 0.535 for selected `SOCKET`. The QR scatter
and cell geometry are unchanged.

The plain card's `MAGNUM 650 / HAND GUN` trace already used Rajdhani Medium
24.4/19.5 with tracking −0.75/+0.6, but Iced still used 19/17 Regular.
The old Iced first line ended around native x 1431 against the source's
x 1480; the SVG reached x 1482. Rust now uses the traced sizes, weight, and
tracking. Source title ink is brighter than the prior SVG/Iced green, so the
plain titles use the independently measured gun highlight `#93ffe4`.

The plain `DPS` source bbox is x 1226..1291, y 1036..1064. The old SVG
was x 1230..1289, y 1037..1062; 19px Medium, baseline y 226, and the
local `#81fee7` stat ink raise first-label mask IoU from 0.340 to 0.613.
The first pass used 25px Regular and baseline y 256.2 for `86`: its source
bbox is x 1233..1284, y 1098..1137, and source/SVG IoU rose 0.161 to
0.474. The native K1 calibration below refines its weight and Rust baseline.
Selected stat runs retain dark ink.

The two CC35 lines beneath the cards were much smaller and shorter than the
source. Plain first-line source pixels span x 1171..1774, y 1330..1341;
the old SVG ended at x 1663. An 8px Bold run with 0.3 tracking and local
`#65e5c8` reaches x 1763 and raises first-line mask IoU from 0.102 to
0.347. The second plain line rises 0.133 to 0.500. On the amber card,
the same type at y 483/491 and local `#e9a50d` raises first-/second-line
IoU from 0.080/0.092 to 0.313/0.506. The remaining first-line width
shortfall is about 11 native pixels, and photographic edge softness remains.

On login, the clock's native clean-ink area was 1,425 pixels against 913
in the Regular SVG; Medium gives 1,419 while keeping its measured bounds.
The `ENTER` bar glyph had 1,147 dark SVG pixels against 778 in the source;
Medium reduces it to 860. Its shape overlap falls slightly (IoU 0.650 to
0.630), so this is a stroke-density fit rather than a claim of contour
parity. The `PROTECTED` bounds and coverage already fit and retain Semibold.

The store foot line remains 9px Bold with 0.25/0.24 tracking as traced.
Iced's previous untracked first and second runs ended at native x 1510 and
2572, compared with source x about 1534 and 2628; the store primitives now
carry that tracking. The matching login foot line was already tracked.

## Native K1 calibration

The K1 native capture confirms the corrected card title widths: source
`MAGNUM 650` spans native x1191–1480 and Iced x1191–1482. Iced's plain
title rows y561–598 remain about two to three native pixels below the
source y559–595; the selected card shows the same offset. Both Rust title
baselines move up one design pixel. The plain `DPS` label spans source
y1036–1064 and Iced y1038–1067, and the selected label has a similar
two-to-three-pixel offset. Rust stat-label baselines also move up one design
pixel; the already source-fitted SVG baselines stay fixed.

The first plain `86` value spans source x1233–1284/y1098–1137, versus
K1 Iced x1234–1285/y1103–1140. The selected value is similarly low,
y1051–1088 against source y1045–1085. Rust value baselines move up 1.5
design pixels on plain cards and 1.8 on the grown card. K1's Regular
value had only 466 clean pixels against 641 in the plain source. A local
SVG Medium trial raises first-value source/SVG mask IoU from 0.474 to
0.536; Bold overfills. SVG and Iced therefore use Medium for the four
values in each state. The card's feedback recoloring still selects the
same eight stat/value positions and changes their inks only.

The mailbox's `from: Jackie` source bbox is x530–687/y843–866, whereas
the prior SVG and Iced ended near x654. `from: Mom` spans source
x530–670/y987–1008 and the prior render ended near x639. Rajdhani
13.5px with a one-design-pixel higher baseline gives the source widths.
The selected sender uses Semibold; the four plain senders use Bold and
source-local `#87f9d8`. In the source/SVG clean-ink masks, the selected
sender IoU rose from 0.092 to 0.602 with Semibold and the first plain
sender from 0.075 to 0.605 with Bold. The row title, hit frame, pitch,
envelopes, and feedback geometry are unchanged.

The two mailbox CC35 notice lines span source x1389–1875/y866–876 and
x1389–1895/y884–896. Their old SVG ends were x1760 and x1772, and the
old Iced ends were x1724 and x1735. The fitted SVG uses 8px Bold,
0.34px tracking, x578.4, baselines 365.5/373.3, and local `#e6b522`;
the first line reaches x1869 and the second x1889 in the trial render.
Iced uses the corresponding 8px Bold run with 1.08 horizontal stretch,
which requires native review because its letter spacing differs from
SVG tracking. Source glow and glyph contours remain outside this fit.

Selection review caught a runtime distinction: the lighter sender face must
follow the selected fill, rather than remain attached to the first message.
The optional selected-row type now resolves through the same path as sender
fill contrast; ordinary rows keep Bold and the current selected row uses
Semibold. Resting source geometry and inks are unchanged. Selection-transfer
native review and the regression test are part of the following checkpoint.

## M mailbox notice renderer fit

The source/SVG first notice line already shares x1389..1875 and y865/866..876 native bounds. L Iced was x1389..1870/y863..875 and 2997 clean ink pixels against 2345 source and 2364 SVG; its second line had 3140 against 2448 source. Only the Iced `MESSAGE_FLAG` runs change from Bold to Semibold, width stretch 1.08→1.09, and baselines 365.5/373.3→366.1/374.0. The parent SVG and component stay at their source-fitted Bold values. Native M ink and bounds remain to be reviewed.
