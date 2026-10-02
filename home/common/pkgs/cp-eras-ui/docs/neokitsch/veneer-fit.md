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

## Dashboard EMAIL crossing grain, 2026-09-29

The selected EMAIL card in `images/neokitsch-dashboard.png` carries
diagonal strands crossing its longitudinal grain near the top, and a
less coherent knot farther down. The native card's 42 strands are straight;
the SVG's 42 wavy strands recover a central chevron but remain too regular.
For example, at native x615–680, y950–1040, source/SVG/native ridge
directions are 66.8°/86.2°/90.0° and coherence is .78/1.00/1.00.
At x710–770, y1225–1295, source/SVG/native coherence is .30/.73/1.00.
At y1170 across x620–690, the source has 17 local dark ridges versus
13 in both SVG and native. A broad SVG-to-native port is therefore not
yet a source fit: some left-middle RGB patches favor the SVG, while
right-middle holdouts favor the native straight strands.

Two bounded SVG trials were rejected. Adding 21 thin, clipped quadratic
strands over the existing family barely moved the x615–680, y950–1040
direction, 86.2°→84.7° against source 66.8°, and worsened RGB MAE
18.60→19.14. The separate x660–710, y970–1060 control worsened
17.88→18.41. Replacing three existing upper strands with bends through
a visible source ridge near native `(652,950)`, `(663,970)`, `(673,990)`
and `(694,1030)` improved that fit direction only to 84.8° and worsened
RGB MAE to 19.11. It also vacated a strip at x685–715, y950–1010:
dark-ridge pixel coverage fell from .327 to .172, versus .337 in the
source. The adjacent x660–710 control gained direction but lost ridge
contrast; the lower x620–690, y1150–1210 control stayed unchanged.

The source has both crossing and longitudinal lines in the same area.
Adding equally spaced curves underfits the crossings; replacing primary
lines leaves gaps. Their exact continuation through the knot cannot be
isolated reliably from the photograph, so neither trial changes the
reference or runtime grain. This remains a local topology and ink fit,
not an accepted claim of exact photographic texture.

### Upper crossing ridge follow-up

A later scratch trial retained every longitudinal strand and drew just three
diagonal ridges over native x620–715, y936–1016. It is saved at
`/tmp/cp-eras-next/ae/neokitsch/trial.svg`, with a source/SVG/trial crop in
`trial-compare.png`. The added paths improved the local dominant angle in
x620–680, y950–1015 from 86.1° to 78.0° (source 61.8°), and in x660–710
from 84.9° to 74.1° (source 63.6°). Local RGB mean absolute error worsened,
however, from 19.23 to 20.90 and from 18.81 to 20.82 respectively. A much
lighter version at 0.45 design-pixel width and 0.45 opacity still worsened
those errors to 19.64 and 19.27 while barely changing direction.

The source row profiles explain the failure more directly than the aggregate
angle. On the first proposed path at native y960 and y970, the chosen x642.5
and x648.9 land on *bright* residuals (−13.5 and −10.8 green levels), while
nearby dark-ridge peaks lie at x647 (+32.2) and x652/656 (+17.9/+24.4).
The second path at y970 similarly lands at x666.2 on a bright residual of
−17.8; nearby peaks are x663, 668 and 673. The third at y950 lands at
x670.1 on −2.9, between peaks x665 and 676. At y1015 the source also has
14/12/17 ridges across x620–680 / 660–710 / 710–765, while the trial has
11/9/11: three strokes do not recover the local density. Their blunt lower
ends form visible unsupported dashes. These particular ridge continuations
are not source-supported and are rejected; the evidence does not rule out a
better bounded fit with individually verified row anchors and supported
joins. The reference and runtime grain remain unchanged.

One narrower follow-up traced a single peak sequence from native
`(653,935)` through `(662,950)`, `(673,960)`, `(683,970)` and `(685,985)`
toward a nearby existing longitudinal strand at `(683,990)`. Its first
version jumped from x653 at y935 to a *different* peak at x665 at y940;
adjacent rows showed the originating peak moving only to x656, so that
version was discarded. The corrected scratch path is
`/tmp/cp-eras-next/ae/neokitsch/single.svg`. Its middle section improves
RGB error from 20.488 to 20.372 in x660–682, y950–975, but the proposed
entry and merge worsen from 18.646 to 19.314 and from 19.986 to 20.910.
The full x650–690, y935–990 branch worsens 20.017 to 20.180. Existing
strands under the new endpoints make them too heavy, and the nearby source
peaks do not establish that this diagonal actually joins those specific
longitudinal strands. Adjacent x625–650 and x695–720 strips over y935–990
remain pixel-identical, as does the x620–710, y1020–1080 control below the
proposed join. This limits the rejected geometry to its endpoints:
the source-supported middle segment remains a possible local fitting
target once a supported connection or termination can be found.

### AL: visible continuations and unresolved joins

A fresh source/SVG/native audit follows the upper diagonal through native
`(653,935)`, `(656,940)`, `(662,950)`, `(673,960)` and `(683,970)`.
Competing peaks at x684/688 near y975 separate into x685 and x692/694 by
y980. The right-moving sequence favors x694 at y985, while x685 appears
to belong to a longitudinal ridge. By y990 the x695 residual is weak and
x685 is strong. This improves the visible continuation over AE's proposed
x685 join, but does not establish where the diagonal terminates or merges.

One scratch path preserves all 42 existing strands and follows that
sequence from the clipped top toward `(695,994)`. Fixed entry, middle and
merge RGB MAE improve 17.665→17.223, 20.725→20.245 and 18.145→17.875;
the full branch improves 19.121→18.964. Of 173 changed branch pixels,
123 improve and 45 worsen. Losses occur at crossings as well as endpoints;
the final five rows have four improvements and four losses with a small
net loss. The SVG halo also changes above the card. Adjacent strips, lower
EMAIL and independent lower-panel controls are pixel-identical.

The gains are real, but the trial does not establish a source-supported
join. At three fixed rows, local ridge counts remain 11/9/5 versus source
9/9/3 using the same prominence/distance rule. Native and SVG still lack
the source's crossing family. Lower EMAIL knots and the panel's dense
lower fan also contain overlaps whose individual connections are hidden
by other ridges or opaque content. The one-line trial remains unaccepted;
no reference or renderer is changed and no native candidate was built.

Future work can fit visible segments without claiming an exact authoring
graph, but needs supported entry/termination handling and controls for
both ridge families. Original layered/vector material or an unobstructed
view would be needed to establish hidden connections. The coordinate map,
frozen hashes, trial, source/SVG/native montage and reproducible measurements
are in `/tmp/cp-eras-next/al-veneer-topology/`.

### AR: bounded visibility map

The current source, SVG and veneer tables still match AL. A fresh packaged
4K dashboard is byte-identical to AL's native capture, so the earlier
comparison remains applicable. An independent map of x640–705/y935–1005
confirms the diagonal's visible middle: all 26 rows from y945 through y970
have a supported dark peak near the observed ridge. It crosses several
longitudinal paths; bending one existing strand into it would erase a
different family.

The map does not recover an entry or termination. The upper entry is
clipped or mixed with other ridges; rows y975/980/985/990 retain competing
peaks near x684/688, x685/694, x685/694 and x685/695. Those observations
cannot distinguish a crossing, merge or endpoint. Fixed adjacent and lower
density controls are retained. Adding the earlier trial stroke leaves the
local excess ridge density unresolved despite its small RGB improvement.

This branch investigation stops pending layered material or an unobstructed
view of its connections. No geometry or runtime change is accepted.
Other visible veneer regions remain separate tasks. The source-only map,
measurements and frozen inputs are in
`/tmp/cp-eras-next/ar-veneer-visibility/`; current-baseline parity and path
comparisons are in `/tmp/cp-eras-next/ar-veneer-audit/`.

### AS: selected-store socket band

The socket strip is a separate fitting target from the 125 selected-body
strands and the unresolved upper EMAIL branch. Its design bounds are
x667.1–929.2, y603.8–651.7. The current SVG has 21 shallow repeated waves;
Iced has horizontal `Prim::Grain` strokes on a 2.4-unit pitch. A fresh
packaged 4K store frame confirms that difference. A scratch preview using
the unchanged production renderer reproduces the packaged frame exactly.

Clear upper windows A/B/C and independent lower windows D/E show the
source fan changing direction across the band. A green-channel structure
tensor after Gaussian high-pass filtering (.65 minus 6 source pixels)
gives these aggregate tangents, measured clockwise from horizontal:

| Window (design coordinates) | Source tangent | Source coherence |
| --- | ---: | ---: |
| A: x726–778, y606.2–618.3 | −17.5° | .442 |
| B: x792–852, y606.2–618.3 | −17.4° | .665 |
| C: x861–922, y606.2–618.3 | −48.3° | .793 |
| D: x726–778, y643–649 | −44.3° | .414 |
| E: x792–852, y643–649 | −65.7° | .653 |

SVG and native tangents stay near horizontal. The source measurements
support replacing that regular stripe with curved grain, but do not
identify every strand: fine contrast modulation also affects the sampled
peaks. A column's dark-peak count alone cannot establish how many diagonal
strands pass through it. A scout initially mistook nearly equal-y peaks
20 source pixels apart for the same strands; that correspondence is
withdrawn. The step aliases across their 3–4-pixel recurrence. One visually
plausible diagonal follows approximately (1920,1477) → (1940,1470) →
(1960,1466), about −15.4°, but even 2-pixel-step minima jump among nearby
ridges. That is not a verified individual strand. Continuity needs both
small-step tracing and visual review, not nearest-peak assignment.

Initial scratch curve families improve aggregate RGB error while losing
upper-band contrast and density. That is insufficient for acceptance.
A later direction-field trial improves A–E angles, but initially leaks
outside the band because the existing card clip covers the whole body.
The corrected trial adds a socket-only clip. Any implementation must
preserve the body, rules, text, QR cells and selection feedback, and check
left/QR-cell flow as well as A–E. Hidden paths under opaque labels need
no invented visible joins, but visible entry and exit handling still
needs source support.

The original, SVG and native comparisons and unaccepted trials are in
`/tmp/cp-eras-next/as-socket-veneer/`; the independent source profiles are
in `/tmp/cp-eras-next/as-socket-ridges/`. The native harness and its exact
baseline comparison are in `/tmp/cp-eras-next/as-socket-native/`.

#### AS native decision: reject the three-socket trial

A narrower candidate preserves the QR cell and replaces only the three
text sockets with 74 paths. Its first native preview loses all band grain:
nested canvas clips are pasted beneath the opaque body fill. This is a
scratch composition error, not evidence against the path geometry. The
corrected preview trims path centerlines to the existing socket rules and
draws them inside the existing body viewport; no shared renderer changes
are needed. Baseline previews equal the verified package/golden pixels at
3840×2160, 1600×900 and 1537×947.

The corrected native candidate improves the A–E aggregate tangents from
approximately 0° to −25.0°, −9.4°, −44.7°, −46.6° and −61.6°. Its RGB MAE
also improves in all five windows. That does not establish a faithful fan:
the first text socket develops a conspicuous pale wedge and long, regular
diagonals replace the source's dense local bends. Upper-right window C
has high-pass RMS 7.53 versus source 13.14 and coherence .969 versus .793.
At y644, profiles averaged over design y643–645 find source/native-trial
ridge counts 16/7, 18/20 and 15/21 in x725–775, x792–850 and x860–920.
These use green-channel Gaussian .6, background sigma 8, prominence ≥2.5
and separation ≥3 source pixels. Counts measure sampled density, not
strand identity; other strip widths produce different counts.

All changed native pixels stay within the text-socket band: 40,474 at 4K,
8,713 at 1600×900 and 8,316 at 1537×947. The QR cell and surrounding body
remain unchanged. Source/SVG/native visual review nevertheless rejects
this trial. No production paths, SVGs, components or goldens change, and
feedback-state captures are not run for the rejected geometry. Future
work must constrain visible spacing and local bends as well as aggregate
angles; do not repeat this continuous-family fit on RGB improvements alone.

The corrected native evidence is in
`/tmp/cp-eras-next/as-socket-native/runs/f60c584d664b42728c7fff7ca0340973/`,
including input/output hashes, three-size parity, region metrics, locality
and the final decision. NK-07 remains open.


#### AT first-socket ridge audit

Four lower ridge runs near design y640–648 can be followed locally, but
opaque lettering and convergence hide their upper connections. Four to
six added paths would still leave the rejected trial's pale wedge. No
replacement SVG is accepted. A layered veneer or matched text-free view
would establish hidden joins; the current image can still constrain
visible segments without inventing their hidden topology. Any segmented
fit must terminate under actual opaque content and preserve visible
joins, density and bends. This does not block other local veneer work.
