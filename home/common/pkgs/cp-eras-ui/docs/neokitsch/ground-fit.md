# Neokitsch ground fit — 2026-09-27

The four 3840×2160 source screens share the same photographed violet and
blue ground. Coordinates here use the 1600×900 trace frame (source divided
by 2.4). This change fits its broad field in the four SVG traces and the
Rust `Prim` backgrounds. It does not attempt to recover the original
photographic recipe or copy its bitmap; fine grain, uneven local light,
printing echoes, the login's lower wire band, and other foreground remain
separate.

## Source relationship

The clear patch at design (400,20) has median RGB 108/78/123 on dashboard,
login, mail, and store. At (800,200), each is 52/55/82; at (1400,200),
each is 11/21/32. This is stronger evidence for a shared ground than the
old traces' different ellipses: dashboard used a dimmer, wider radial with
no left lift, while the other three used a second approximation. The
source images are identical in these clear regions even though their
foreground layouts differ. The login's lower circuit band begins well
below the fitted upper region and was excluded.

## Model

A page fill `#0d090c` carries an elliptical violet lobe centred at
(712.8,-61.2), radii (1015.8,393.6), turned 1.76°. Its encoded RGB
stops at 0/.2/.4/.6/.8/1 are `#7a538b`, `#5e4169`, `#524064`,
`#3c3a57`, `#131014`, `#0d0a0d`. A flatter left lift remains at
(430,-40), radii (560,168), with `#7a598a` at alpha .85/.55/0 at
0/.45/1. The cold transition is a transparent blue annulus centred at
(765.3,-316.3), radii (1188.4,800), turned 10°. Its stops at
.6/.68/.76/.84 are `#48537d` α0, `#21364e` α.85, `#0d202f` α.8,
and `#0e0d1a` α0. The existing horizontal luminance mask fades this
annulus in from the left. The same layer order is used by the four SVGs
and their Rust backdrops. The standalone bar keeps its earlier design
field. These ellipses approximate the observed field; their centres
and colors are fit parameters, not claims about the source artwork.

## Sampling and held-out comparison

Each source sample is the median of an 18×18 native-pixel patch on a
grid of 20 design pixels horizontally and 10 vertically, through y=510.
A patch entered the fit when at least three of the four screen medians
agreed within three levels per channel, its median absolute local
variation was at most four levels, it had a blue/violet ground hue, and
its channels remained below 145. This rejects the strong foreground,
shared gold wire, and the login's lower band. There were 2,965 accepted
patches. Alternating grid parity assigned 1,484 to fitting and 1,481
to holdout. The score below uses only held-out patches whose **before
and after SVG renders** also have low variation and ground hue, so each
before/after pair uses exactly the same locations. It is channel RMS
against the native source median, sampled from the actual librsvg
renders. Regions overlap.

| Screen | All | Upper y<150 | Middle y150..300 | Lower y≥300 | Left x<350 | Right x>1200 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Dashboard | 10.98 → 2.22 | 16.89 → 3.35 | 9.69 → 1.88 | 3.54 → 1.10 | 13.03 → 2.68 | 12.09 → 2.55 |
| Login | 5.20 → 2.09 | 5.60 → 3.16 | 6.42 → 1.77 | 2.39 → 1.02 | 4.75 → 2.63 | 4.56 → 2.36 |
| Mailbox | 5.24 → 2.19 | 5.63 → 3.30 | 6.43 → 1.81 | 2.49 → 1.13 | 5.23 → 2.85 | 4.59 → 2.42 |
| Store | 5.21 → 2.11 | 5.54 → 3.26 | 6.50 → 1.69 | 2.34 → 1.13 | 4.90 → 2.78 | 4.61 → 2.50 |

For dashboard at (400,20), the old SVG rendered 65/59/88, the new SVG
107/79/123, and the clear native patch is 108/78/123. At (800,200),
all sources are 52/55/82 and the new SVG is 50/55/81. At (1400,200),
all sources are 11/21/32 and the new SVG is 14/21/31. The masked blue
transition and dashboard's missing left lift account for the largest
improvements. The upper held-out score remains higher because a smooth
field cannot reproduce all spatial variation in the photograph. Native
Iced capture parity is checked in the integration pass.

## Native Iced review

Astra reviewed all four source/SVG/native 3840×2160 Iced comparisons.
On held-out clear patches accepted in all three images, source-to-Iced
RGB RMS is 2.14 (dashboard), 2.06 (login), 2.20 (mailbox) and 2.23
(store). Iced-to-SVG patch RMS is 0.58–0.85; the largest differences
are 3.5–13 levels, so this is a close broad-field fit rather than exact
pixel parity. The actual captures confirm the stronger upper violet
field and restored blue transition. Original bar field and inferred
hover preblend are preserved independently. All four G1i source gates
pass with the paired-palette ink comparison; full integration checks
remain pending.
