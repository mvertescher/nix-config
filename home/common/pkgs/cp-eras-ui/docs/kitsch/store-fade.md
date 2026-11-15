# Store fourth-card edge

Source: `images/kitsch-store.png` (#52), resized from 3840×2160 to the
trace's 1600×900. Measurements below are RGB samples at y284 through a
clear portion of the fourth card's yellow band.

| x | source | old SVG | revised SVG |
|---:|:---:|:---:|:---:|
| 1522 | 252,194,11 | 254,195,47 | 254,195,47 |
| 1524 | 206,212,104 | 254,195,47 | 220,168,43 |
| 1526 | 88,57,1 | 254,195,47 | 94,69,27 |
| 1530 | 53,41,10 | 254,195,47 | 72,51,23 |
| 1540 | 23,15,10 | 254,195,47 | 30,19,19 |
| 1550 | 13,12,11 | 254,195,47 | 25,15,18 |

The primary ink stays bright through x1523 and loses most of its energy
by x1526. The residual yellow remains visible to about x1544. The mint
gun and values bar have similarly short, less uniform tails; the
photograph includes a teal echo near x1525 that a single uniform opacity
cannot reproduce exactly. The SVG and Iced shelf therefore use one opaque
card slice through x1523, followed by five 2–10 px slices at opacity
0.85, 0.30, 0.20, 0.08 and 0.02, ending at x1550. The shelf still has
one interactive card-4 Plate; each of its idle, selected, hover and
pressed faces uses these same slices. The full reusable card in
`components.svg` remains unchanged.

The clipping bounds include the left-projecting flag, the full outline
stroke and the raised hover ghost. An initial implementation cropped those
at the card's left edge; native before/after review caught that regression
and the bounds were widened before acceptance.

The source's **ground** also darkens toward the right edge: at (1550,220)
it is (18,11,11), while the earlier SVG background is (41,18,26). That
broader vignette is separate from the card cutoff. This correction does
not paint a dark rectangle over the ground or footer to imitate it.
Likewise, the source supports only the default selected-card-2 shelf;
the fade on card-4 selection and feedback is an inferred continuation.

## 2026-11-15: clean cut

The review of 2026-11-13 read the five stepped slices as a streaky
artefact: each repeated the whole card, so the M of MAGNUM, the band's
glyphs and the stats stayed legible through x1550 in visible steps. The
source's residue carries no such detail -- per design px past the cut,
on native #52, the band reads 0.62 at x1525, 0.27..0.31 to x1529, 0.13
to x1535 and ground by x1545, and it is a halo, not the card. An
edge-column smear (each slice showing the last opaque column shifted
out) was tried and combed the title's stems into stripes. The card is
now cut clean: opaque through x1524 (local 81, `CARD_4_CUT`) and one
antialiasing column at 0.62 (`CARD_4_SMEAR`). The halo past x1525 is not
drawn. The card itself sits 1.2 px left of the source (left outline
x1443.0 against 1444.2), so the cut catches a sliver of the M's first
stem that the photo's cut does not.
