# Reference and interaction corrections, fourth batch — 2026-09-27

Three Sol workers handled scoped implementation, with Astra integration
and verification. The [third batch](reference-svg-round3.md) remains
staged alongside these changes.

## Kitsch store selected-card hit regions

Selected cards now use a 261×500 hit rectangle, covering the lower detail
and compliance printing; idle cards retain 261×320. `Prim::Pick` selects
the bounds using the same committed card index as the drawing. Card4's
persistent viewport still ends at x1550. Its selected keyboard center is
(1496.5,468), inside the visible region.

Tests exercise all four selections at 0.75/1/1.25/2.4 scale: lower-body
and footer hits, idle misses, clipped margins, navigation centers and
release cancellation. Independent Astra review found no substantive
issues in selection, traversal or feedback handling.

The resting 3840×2160 image is pixel-identical to the third batch. At
1537×947, hovering over selected card4's newly clickable lower body
activates the existing lifted face; holding returns the selected face.
Neither changes any pixels beyond the fade endpoint. The source supports
selected card2; other selected states remain inferred application behavior.
See [bounds and coverage](kitsch/store-hit-regions.md).

## Entropism mailbox baseline calibration — E2 follow-up

Entropism's mailbox baseline conversion factor changes from 0.95 to 0.89.
Source-fitted coordinates, text content, SVGs and other eras stay fixed.
Across 26 measured runs at native 3840×2160, mean SVG-to-Iced vertical edge
error falls 1.210→0.056 design pixels, with maximum 0.417px after correction.
All measured widths are unchanged. Source-to-Iced mean four-edge error
falls 0.653→0.076px.

Independent samples of boxed A/B/C, header/footer text, body lines and
badge labels also improve. At 1537×947, 24 reliably isolated runs move
down by 1–2 screen pixels, with horizontal edges stable within one raster
pixel. The two faint micro-print runs are excluded from fractional
measurement; the native measurements include them.

E2 remains open. The third paragraph still starts at y533.2 rather than
the SVG's y535 because of the common paragraph-spacing model. Badge label
width/centering, exact source-font forms and action-width residuals remain
separate work. See [measurements and limits](entropism/mailbox-baseline.md).

## Neo-kitsch store tabs and shoulders — NK-09

Product tabs now have narrow 58px tops and 64.5px bases. Navigation tabs
have 31px tops and 36.5px bases, and start 2.7px higher. The joins are
curved in the trace, component excerpts and Rust scenes.

Shoulders now form a smooth fan from a common low tangent. Native profiles
also recovered a faint fifth echo missed by the initial bright-pixel mask:
the source has six strokes including the outer contour. Five fitted cubic
curves match the sampled source ridges with mean centerline residual
0.104 design pixels. Hover outlines follow the revised curved shoulder;
held states retain the corrected plain/selected faces. Source, SVG and
Iced were reviewed separately, including the shared junction and corners.

Remaining echo ink and inner right-corner fitting are recorded separately
as NK-14. The main taper and fan correction does not close those material
and local contour limits. See [measurements](neokitsch/store-tabs.md).

## Validation

All 254 Rust tests pass (209 library and 45 bar-window). All 24 SVGs parse
with unique IDs and resolved local references. Neo-kitsch store passes G1i
with ink placement 0.66 and G2i with 94% matched shape area. Entropism
mailbox passes G2i with 100% matched shape area. These broad inventories do
not establish detailed source fidelity; native crops and state captures
were reviewed separately.

Only the Entropism mailbox and Neo-kitsch store goldens are refreshed in
this batch. The full repository check passes all 22 checks. All 27 visual
cases pass on the first attempt at 100.000% similarity to their goldens.
The 14 changed source and golden files match the packaged source snapshot
byte for byte. Scratch captures, measured baselines and logs are in
`/tmp/cp-eras-round4/`.

Changes remain staged and uncommitted. Verification uses headless
rendering; live desktop input/IME and presentation checks remain separate.
