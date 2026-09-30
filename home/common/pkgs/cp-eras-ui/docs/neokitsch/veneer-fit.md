# NK-07: dashboard panel veneer fit

The photographed dashboard (`images/neokitsch-dashboard.png`) has a dense
lower grain fan. The current trace and runtime panel use 85 clipped strands;
they preserve the overall vertical direction but lose the source's lower
convergence. This note records bounded native-size attempts made on
2026-09-29. No texture change was accepted.

The strongest continuous-path prototype used 105 strands seeded from the
photograph at both the upper and lower ends. Its upper ridges recovered the
source's local count and contrast. The lower turn had a similar overall
direction to the source (horizontal/vertical orientation measures 0.617/0.274
versus 0.570/0.286), but its strands were too sparse and bunched. At native
x=1200/1280/1360 over y=560..625, source peak counts were 28/27/18;
the prototype had 19/19/8. Its surviving peaks were often much stronger
than the source. The prototype is `/tmp/nk-panel-source105-thin-candidate.svg`;
`/tmp/nk-panel-105-montage-{upper,lower,full}.png` shows source, current,
and candidate crops.

A second prototype traced dark residual contours directly from the source's
text-free lower band (design x=1171..1401, y=520..635) and paired them with
the continuous upper strands. This recovered the lower fan's density and
direction but visibly broke many strands into dashes at the convergence and
left a hard join at y=520. Local residual thresholds from 0.5 to 2 native
channel units and an orientation-guided bridge did not repair both faults.
`/tmp/nk-lower-integrated.svg`, `/tmp/nk-lower-integrated-lower.png`, and
`/tmp/nk-lower-threshold.png` hold the scratch prototype and comparisons.

The unresolved fit is a continuous, fine lower fan with the source's strand
count, width, and subdued contrast, joined to the upper grain without a seam.
Neither prototype meets that bar; keep the current 85-path texture until a
native-size source/SVG/runtime comparison supports a replacement.

## 2026-09-29: mailbox selection bar

The photographed bar has a bidirectional fan: text-free left strands turn
about 20–22° above horizontal, the lower middle is about 1°, and the right
strands turn about 13–20° the other way. The current trace and native bar
are mostly horizontal there. A continuous side-specific scratch fan restored
those local directions without crossings, but its clipped outer corners
formed dense triangular wedges absent from the photograph. Top-edge-seeded
curves with near-vertical starting tangents removed the wedges only by
leaving broad empty corner gaps; tighter edge continuations either bunched
or crossed. The source's top sweep, side continuations, and central
book-match need a branching fit with ridge-supported joins. No mailbox
veneer change is accepted. Native crops and measurements are in
`/tmp/cp-eras-resume-20260929/q-veneer/findings.md`.

## Selected store body: current scope

The earlier dominant-direction correction is already implemented and
accepted: the body uses 125 longitudinal strands plus a narrow seam in
both SVG and Iced. The socket band deliberately keeps a separate horizontal
fan. Fresh P-source/native comparison confirms this; no 90-degree rotation
is needed. In clear upper/lower body patches, source horizontal/vertical
derivative ratios are 2.55/3.74, native 23.71/7.02 and SVG 27.71/7.17.
All are chiefly longitudinal, but the current figure is much straighter.
These ratios are directional diagnostics, not full-fidelity scores.
Remaining differences are local curling/spacing/ink, the sharper seam kink
and the socket band's missing arc/density variation. Do not reopen the
already-corrected broad direction as a new implementation task.

## Selected seam bend, 2026-09-29

The source's right seam turns upward near x875–925 toward y533; the old
trace continues down to y536.2. A bounded path correction preserves its
left section and bends through `(880,535)` to `(900,533.2)`, then stays at
y533.2 through x929.2. All 125 surrounding strands, their ink, width,
clipping and the separate socket band are retained.

Using green-channel profiles with local background removal, ridge-location
SVG error improves from 2.08 to .90 design pixels on x870–895 and 2.92 to .28 on
the disjoint x900–925 comparison. Corresponding normalized profile errors
fall .256→.196 and .239→.108. The source was viewed before choosing the
model, so these spatially separate comparisons are not blind. The left
transition remains unchanged. Actual Iced ridge error improves 2.08→1.04
and 2.92→.28 design pixels (5.0→2.5 and 7.0→.67 pixels at native 4K).
Native RGB MAE improves 41.68→41.32 and 39.33→36.47. Exactly 1,084 native
pixels change, all within the seam. Fractional rest and held fourth-card
captures show the same local correction while retaining clipping/feedback.

This corrects only the seam centerline. The source branch is broader and
its strand field bends earlier above and more sharply below the seam.
At x900/920, source upper tilt near y519 is about48°/47°, versus3°/2° in
the trace; lower tilt near y541 is−68°/−70°, versus−44°/−50°. Leftward
strands have a different residual. Ridges merge and re-emerge, so the
photo does not identify unique path connections. A rendered local curvature
model still needs independent orientation, density and pixel comparisons;
centerline agreement alone does not close the veneer task.

A subsequent 38-strand cubic trial moves the upper bend earlier and
sharpens the lower return, with a smooth x850–870 blend and unchanged
left strands. Orientation improves in all three evaluated regions, but
their tight RGB errors worsen 22.68→22.94, 23.53→23.64 and 21.41→22.55.
Ridge counts also move away from the source: the upper fit has source/
baseline/trial counts 6/9/12, and the lower comparison 9/9/11. Its regular
elbows do not reproduce the photographed branching, so the trial is
rejected. No broader strand change accompanies the accepted seam bend.
