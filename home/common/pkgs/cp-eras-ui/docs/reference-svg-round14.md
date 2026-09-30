# Fourteenth reference checkpoint — AC, 2026-09-29

AC trims the unsupported rectangular end of Neomil's small margin plaque.
The source's leading edge slopes from approximately native `(113,1146)`
to `(136,1153)`; AB's hatch continued to y1142 across that width. The
trace/component clip now follows the slant. Iced shortens eight existing
hatch segments and removes the last segment, retaining the ink, opacity,
pitch, primary letters, O ring, adjacent A2, slash and CJK.

## Source and native evidence

All 126 changed 4K native pixels lie at x112–136/y1142–1152. At 1600×900
and the fractional 1537×947 viewport, only 32 pixels change. On the fixed
leading-edge crop, source/native RGB error falls 20.20→14.52 and bright
intrusions into source-dark pixels fall 88→4. The O/plaque crop improves
25.55→23.09, with those intrusions falling 55→6. The O counter and A2
native crops are pixel-identical to AB; this is a boundary correction,
not a new material fit. All three cross-edge sections improve RGB error
and mask F1 at red thresholds 80/100/140. At the lowest threshold four dim
source pixels are lost; all source-core matches at 100/140 are preserved.

The matching SVG leading crop improves 18.63→14.11 and 55→3 intrusions.
Its full change is 200 pixels: converting the rectangular clip to a path
also changes 31 remote edge pixels by at most three RGB levels. The native
explicit endpoints avoid that rasterizer side effect. The wider
four-corner plaque proposal is rejected because it changes the A2
junction without adequate source support. Only the leading edge is fitted.

An algebraic diagnostic of uniformly dimmed native letter ink improves
average RGB error but worsens several independent bright-stroke controls.
It is not a rendered candidate and is not applied. Neither the original
photographic modulation nor exact glyph softness has been recovered.

Evidence scripts, source/SVG/native crops and measurements are under
`/tmp/cp-eras-resume-20260929/ac-margin-plaque/`.

## State and integration checks

All 286 Rust tests and the release build pass. Ten native captures cover
production 4K/1600, fractional rest, ordinary/selected/fourth-card held,
last-card selection, custom palettes and early opening. All matched
changes stay in the leading-edge region. The margin is identical across
shelf-only state changes. The custom foreground has 406 cyan pixels and
zero strong-red pixels in the margin crop, preserving semantic ink.

All 24 SVGs parse with unique IDs, and both store fidelity gates pass
without threshold changes (72% source shape area, 89% implementation
shape area). All 22 repository checks pass, including 27 visual cases
matching 100.000% on their first attempt. All 200 frozen file hashes agree
with the Nix source. Only the reviewed Neomil store golden is refreshed
in this follow-up. Changes remain staged/uncommitted;
fine material and live-desktop limitations remain in their owning TODOs.
