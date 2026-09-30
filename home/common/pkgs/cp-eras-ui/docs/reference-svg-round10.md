# Tenth source-reference checkpoint — P–T, 2026-09-29

T passes 272 Rust tests and the release build. All 24 SVGs parse with
unique IDs. The latest source/SVG and SVG/native gates pass for all eight
affected screens (16 gates). Astra reviewed native 4K/1600 captures and
fractional rest, selection, held/cropped, custom-palette and opening states.
Reviewed goldens are refreshed. The immutable T snapshot passes all 22
repository checks and all 27 visual cases at 100.000% on their first
attempt. The git-filtered source is 14 MiB; its Rust sources match the
reviewed T snapshot. Later U candidates are separate from this result.

## Accepted local artwork and typography

Neomil store now has independently fitted ordinary/selected title,
subtitle, stat and value prescriptions with native baseline calibration.
Navigation and boxed-footer fitting preserve custom-palette semantics.
The unboxed footer gains source-supported width, placement and
reference-only bright ink; custom palettes retain Dim. The inherited
`08 CP` is corrected to source `0B CP` in the store margin/footer and
login margin. Store-margin native mask overlap improves .176→.398, and
its bounds are within one source pixel. PETROCHEM tracking increases
.5→1.3 to fill its photographed box vertically on ordinary and selected
cards. BETTERLIFE TEC does not support an extra shared shear.
See [type](neomil/store-typography.md) and [printing](neomil/store-printing.md).

The eight tiny leading Neomil dashboard tape-mark paths use a better
local ink. Exactly 99 native pixels change, all inside their existing
footprint; source RGB MAE falls 20.49→10.07. Glyph contours and edge
softness remain approximate. See [the local review](neomil/dashboard-fidelity.md).

Neo-kitsch store replaces the approximate 4S font with source contours,
fits the outlined T, and calibrates stat/value/socket type independently
of frame geometry. Mailbox titles and ordinary sender endpoints now sit
within about one native pixel of source; its selected sender receives a
separate two-pixel native baseline correction. The dashboard customer
header loses excessive width, with native source-mask overlap .670 and
bounds within one pixel. Selection transfers and held states retain the
row/card geometry. See the [store](neokitsch/store-typography.md),
[mailbox](neokitsch/mailbox-typography.md) and
[header](neokitsch/dashboard-header-typography.md) records.

Kitsch's first store compliance line reaches source endpoints within one
native pixel on the three measured cards. Footer-brand tracking .25→.18
corrects progressive word drift; native/SVG endpoints agree and its native
source-mask overlap is .399. Five idle dashboard faces use #20858f and
trail fill opacities are halved. Independent face-strip MAE falls from
13.01–22.79 to 7.60–10.94 RGB levels; SVG/native samples agree. Measured
edge profiles support retaining the original trail stroke opacity.
See [type](kitsch/typography-fit.md) and [material](kitsch/dashboard-material.md).

## Bounded resize preview

A resting, unheld Neomil dashboard can temporarily reuse a complete cached
material set during resize, with a 200 ms quiet deadline, a 10% scale bound
and the unchanged 384 MiB cache budget. Opening/frozen/held paths stay exact;
foreground layout and input coordinates use current bounds. Actual headless
compositor events and a separate deterministic preview capture exercise the
implementation. The integer final image is byte-identical to its exact
baseline; the fractional resized control differs at three pixels by one
channel level. Two larger footer-history differences also occur without
the preview. One fractional sequence still incurs a 404 ms intermediate
preparation, so this does not establish smooth continuous resize or hardware
presentation. See [measurements and limits](neomil/resize-preparation.md).

## Remaining work

The latest review identifies a separate Neomil selected-frame/side-printing
misalignment. Its left edge and primary icons already align, so a global
translation would be wrong; the next lane fits the top/right contour and
selected printing locally. Remaining Neo-kitsch typography and veneer
convergence, Kitsch illumination/softness, Neomil chip-2/fine printing and
exact glyph/material differences stay open. The selected Neo-kitsch store
body's dominant longitudinal direction was already corrected; its TODO now
states the remaining curvature/density/seam differences.

Changes remain staged and uncommitted. No live desktop or deployment
validation is included in this checkpoint.
