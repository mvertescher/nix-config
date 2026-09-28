# Entropism mailbox paragraph and badge geometry

The first eight body lines retain their existing baselines. With `line:
21.7` and `para: 39`, the common pitch puts the last paragraph's first
line at y533.2. The SVG and source place it at y535. `MailPanel` now accepts
optional first-line baselines per paragraph; Entropism sets `[325, 429.1,
535]`. The first two values reproduce the existing pitch, and the third
corrects that 1.8px displacement. An omitted value follows the original
pitch independently of earlier overrides. The words, line breaks, and
dynamic heading/sender behavior are unchanged.

The badges needed a different horizontal fit for each two-character label.
The source image, SVG rendered by pinned librsvg, and baseline Iced capture
were measured at 3840×2160. Coordinates below are divided by 2.4 into the
1600×900 design frame. Each label was isolated inside its badge; light
glyphs used green >110 in the source and >130 in SVG/Iced, while the dark
T2 glyph used green <130 in the source and <140 in SVG/Iced. The source
retains photographic noise and different letter contours, so these bounds
measure placement and outer width only.

| Label | Source x bounds | Old SVG x bounds | Revised SVG x bounds | Old Iced x bounds | Revised Iced x bounds |
| --- | ---: | ---: | ---: | ---: | ---: |
| T1 | 1362.50–1389.58 | 1361.67–1388.33 | 1362.50–1389.58 | 1365.83–1384.58 | 1362.50–1389.58 |
| T3 | 1435.42–1470.83 | 1434.58–1469.58 | 1435.42–1470.83 | 1440.42–1464.58 | 1435.42–1470.83 |
| T2 | 1358.75–1393.75 | 1357.92–1391.67 | 1358.75–1393.75 | 1363.75–1387.50 | 1358.75–1393.75 |
| T4 | 1435.00–1471.67 | 1434.58–1471.25 | 1435.00–1471.67 | 1440.00–1465.83 | 1435.00–1472.08 |

The revised SVG anchors/scales are T1 `(1377, 1.44)`, T3 `(1454, 1.48)`,
T2 `(1377, 1.47)`, and T4 `(1453.5, 1.42)`. Each revised SVG horizontal
bound lands on the measured source raster pixel; revised Iced does too
except T4's right edge, which is one native pixel (0.42 design pixels)
farther right. `MailBadges.label_runs`
supplies the corresponding relative anchor and stretch to Iced, with its
existing common label as fallback. Label strings and selected-badge
semantics still come from the mailbox data.

The SVG's glyph contours and some vertical edges still differ from the
photograph. Revised Iced's badge top/bottom errors against the source are
T1 `(+0.42, -0.42)`, T3 `(+0.83, 0)`, T2 `(-0.42, -0.42)`, and T4
`(0, +0.83)` design pixels. The runtime label y remains 43.5 relative to
the badge: copying the SVG's baseline would move several edges farther
from the source. Font contours and the remaining action-label width
residuals are outside this geometry correction.

At native resolution, the third paragraph's first line now spans
y522.92–537.50 in Iced versus y523.33–537.50 in SVG; it previously
spanned y521.25–535.83. Its second line now spans y544.58–559.17
versus y544.17–558.33 in SVG; it previously spanned y542.92–557.50.
The third-paragraph vertical-edge error across those four edges falls
from 1.46 to 0.42 design pixels on average. Its remaining line pitch
residual follows the SVG's alternating 21/22px baselines.

Source: `images/entropism-mail.png`. Baseline Iced capture:
`/tmp/cp-eras-round5/before-entropism-mailbox-3840x2160.png`. Revised
SVG render: `/tmp/cp-eras-round5/after-entropism-mailbox-svg.png` using
`/tmp/cp-reference-audit/fonts.conf`. Revised Iced capture:
`/tmp/cp-eras-round5/after-entropism-mailbox-3840x2160.png`.
