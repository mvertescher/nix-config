# Login SVG-to-Iced drift, 2026-09-27

This pass aligns the login scene table with the existing measured trace. It
does not reconstruct the active avatar, inactive portraits, password glyphs,
header lettering, or card material. Those remain separate source-fidelity
work in the [login plan](../../todo/neomil-login.md).

The source is `images/img-06-private.png` (3840×2160); the trace is
`login-trace.svg` at 1600×900. The photo's active-card interior at native
coordinates x925..1010/y900..1030 has median RGB (246, 51, 51), and a
second clear patch at x1360..1410/y1000..1200 has median approximately
(247, 52, 52). The trace's flat card is `#f63333`; Iced previously used the
published foreground `#de2e2e`. The active plate now carries a
reference-only `#f63333` fill. Its role remains foreground under a custom
palette, so palette edits still reach the card. The card remains
x372..625/y315..570, with its 51px top-right cut.

The existing trace and component excerpt agree on the following geometry.
The card data now records each contour directly, avoiding a renderer-wide
rule for the three source-specific shapes.

| Element | Trace coordinates | Former Iced drawing |
| --- | --- | --- |
| Card 2 top-right cut | (896,315) to (947,366), 51px | 47px |
| Card 3 top-right cut | (1185,315) to (1236,366), 51px | 47px |
| Each left notch | y390, then x+15/y405, x+15/y483, y496 | y392/402/486/497 |
| First avatar tab | x452..498, y386..393; bevel starts x492 | ended at x504 |
| Other avatar tabs | x772..824 and x1061..1113 | already 52px wide |
| Card 3 notch rail | `#8a2027`, x982/y390..496 | foreground dim ink |
| USER 01, all cards | 18px Rajdhani, 1.5px tracking | no tracking |

The first two notch rails continue to use the palette's dim role. The third
uses the trace's darker edge ink, matching its body outline and lower
divider. The notch cut reads as `#080405` in the reference palette and
uses the background role with a custom palette. The shared path and plate
renderer paints the era's explicit notch and tab data; it does not choose
values based on the era or screen.

The Login button's static open-top slot stays at
x497.3342..500.3799/y635..654.6317. It is independent of the blinking
password tail and pointer or keyboard submission. The active card's
255px height and the password field's bounds are unchanged.

The SVG trace and the component excerpt already held these coordinates and
colors, so this correction requires no geometric change to either SVG.
The source inventory gate checks the unchanged trace against the photo;
the headless implementation comparison and input regressions belong to the
integrated verification pass after the shared renderer change is built.
