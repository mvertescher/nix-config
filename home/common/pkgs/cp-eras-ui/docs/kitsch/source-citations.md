# Kitsch source citations, reviewed 2026-09-27

The four screen traces are source-backed schematics. Their cited local images
are `images/kitsch-dashboard.png` (#49), `images/kitsch-login.png` (#50),
`images/kitsch-mail.png` (#51), and `images/kitsch-store.png` (#52), all
3840×2160 and mapped to a 1600×900 trace canvas by dividing source pixels by
2.4. The source identities and Behance references are catalogued in
[`docs/sources.md`](../sources.md). The trace header and SVG path are the local
citation for each measurement. Raster bounds below were checked against the
local image and reported in trace coordinates. For reproducibility, the broad
cyan/green mask was `(G > 100) & (G > 1.18*R) & (G > 1.02*B)` on RGB pixels;
the bracket region crop was native `[x=500..1500, y=0..1800)` and the barcode
region crop `[x=850..1450, y=1500..1690)`. The resulting bounds describe this
threshold and crop, including antialiasing; they are not hand-traced path
centrelines or a claim that every colored pixel belongs to only that feature.

## Corrected geometry citations

- **Login bracket (#50).** In `login-trace.svg`, the path at the `bracket`
  section has centreline x=228.5 and x=611.5, starts at y=0, breaks from its
  left vertical at y=528, then the curve controlled by (338,630) ends at
  (338,650) before the vertical drop; the rounded bottom reaches y=731. The fade makes the top segment faint, not absent. A cyan/green mask
  over the local image gives approximately x=228.3..612.9, y=19.6..732.5;
  this is antialiased photo ink, not the path's exact centreline. Thus the
  old description “breaks at y 540” and any reading that visible ink starts
  at the top edge are imprecise. Keep the path and the source observation
  distinct.
- **Login barcode (#50).** Its label rectangle is x=370..377, y=632..695;
  the bars start at x=377.5 and the rightmost bar ends at x=590.4, with their
  common y extent 632..683. The digit text has baseline y=696. On the local
  source image, the cyan/green ink mask spans approximately x=370.4..590.8,
  y=632.1..699.2, including antialiasing and digits. The earlier loose claim
  that the whole barcode ends at y=695 omitted the digits; a claim ending at
  y=683 describes only the bars.
- **USER box (#49 and #51).** The dashboard path in `dashboard-trace.svg`
  is `M 155.5,189.5 H 349.5 V 232.5 H 220 L 208,240.5 H 155.5 Z`.
  The mailbox path in `mailbox-trace.svg` is
  `M 155.5,185.5 H 349.5 V 228.5 H 220 L 208,236.5 H 155.5 Z`.
  Both end at x=349.5. The old `bar.svg` citation to dashboard x=331 came
  from a superseded, earlier box measurement; it does not describe either
  current trace. The original bar's USER-shaped host cell is a design analogy,
  not a literal scaled copy of either screen box.

## Color, stroke, and implementation scope

A sampled color is local evidence for the named mark, not a global era rule.
For example, the login bracket uses its measured mint; tiny captions, readable
labels, the mailbox outline and selected controls have different values. Stroke
widths likewise vary by feature: the login bracket is about 1.2px (the SVG uses
1.3px on half-pixel coordinates); the badge outlines, barcode marks, envelope
glyphs and mailbox body have their own widths. The original bar uses 1.25px for
its principal outline paths. Neither that bar value nor a trace's sampled ink
establishes a universal kitsch stroke or text color.

`src/eras/kitsch.rs` and `src/bar.rs` were checked read-only on 2026-09-27.
The current `Style::bar` configuration includes 40px peaked/chamfered
workspace chevrons, a stepped tape cell and window-label cell, a 1.25px bar
stroke, a bracket ornament, and a root menu with a wave foot and split selected
row. `src/bar.rs` renders those style choices through its workspace, tape,
window, ornament and tray-menu paths. The SVG remains a separately authored
reference, so this source review does not claim a fresh pixel comparison or
that its original design matches every runtime detail. The dated bar
implementation and visual validation are recorded in
[`todo/bar.md`](../../todo/bar.md); remaining source-trace material and glyph
fidelity is tracked in the open K2, K3, K5 and K8 items in
[`todo/reference-svg.md`](../../todo/reference-svg.md).

## What is sourced and what is inferred

The stills source the photographed resting screens and the login field's
visible caret/focus. They do not show a mouse pointer, hover, or press. Therefore the
hover and press specimens on `components.svg` are explicitly inferred
interaction examples, based on the era's depth and selection treatments; they
are not source observations.

Preserve the source content details: the dashboard has PRODUCTS twice, with
EVENTS selected beside the BRAINDANCE panel; dashboard and mailbox text read
GUES 7702, while the login cards read GUEST 7702. The bar SVG has no Behance
screen or source photograph. It is an original composition that borrows
silhouettes and palette references from the cited traces; it must not be
counted as a fifth photographed screen or a source fidelity result.
