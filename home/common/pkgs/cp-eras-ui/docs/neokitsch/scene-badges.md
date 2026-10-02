# Neo-kitsch dashboard and store section badges

AW fits the dashboard A/B/C/D and store A/B/C independently against
`images/neokitsch-dashboard.png` and `images/neokitsch-store.png`.
Each had undersized font approximations, unsupported internal folds and
missing lower tabs. Dashboard footer C/D also sat about 56 source pixels
high; C was about 24 pixels right. The runtime now uses source-derived
paths in two era-owned modules, with the same uniform scene scaling and
fixed reference inks. The two corresponding header-A component specimens
are synchronized; login and mailbox specimens retain their own provenance.

## Registration and edge controls

Source footer origins are C `(1379,1972)` and D `(2809,1972)` at 4K.
Their design anchors are `(574.58333,821.66667)` and
`(1170.41667,821.66667)`. Header A/B and all three store glyphs use their
own measured placement. Nearby annotations and interaction geometry are
unchanged.

Fixed straight-edge source controls exposed defects in the first trial.
The 1.4 dashboard header stroke covered three or four pixels where the
photo's core covers two. A 1.0 stroke, B's right edge one source pixel
right, and both bottom rims one source pixel down correct those
centerlines and widths; the final brighter ink retains threshold fringes.
Moving the tab bottoms as well produced an unsupported B bottom row and
was rejected. Store right edges likewise move one source pixel right:
A `925–926`, B `1679–1680`, C `2889–2890` now replace the trial's
`924–925`, `1678–1679`, `2888–2889`.

The thinner header trial retained a dim ink that lost small-size contrast.
Six independent source edge cores have combined median RGB
`(218,163,115)`, supporting existing `CAPTION #d9a877` for A/B outlines.
C/D retain their independently reviewed outline ink. Dashboard tab
interiors independently support fixed `#fcbe6d`; their 4K median red is
251–255 and green about 189. B retains a blue-channel residual. Store
letters and tabs use the existing fixed `BRIGHT` and `TAB` colors.

## Acceptance and remaining work

Acceptance compares original photos, final SVGs and native captures at
3840×2160, 1600×900 and 1537×947. Fractional scene coordinates use a
uniform scale from the top-left; the photo is resized to scene content,
not stretched across the taller window. Small source resizes are proxies
for photographic edge behavior, not independent original designs.

All 147 whole-glyph checks improve (seven badges, seven warm-red
thresholds, three sizes). At 4K with warm R>190:

| Badge | Baseline glyph IoU | Final glyph IoU |
| --- | ---: | ---: |
| dashboard A | 0.024 | 0.825 |
| dashboard B | 0.028 | 0.846 |
| dashboard C | 0.000 | 0.829 |
| dashboard D | 0.000 | 0.709 |
| store A | 0.080 | 0.863 |
| store B | 0.106 | 0.951 |
| store C | 0.086 | 0.944 |

All changed native pixels stay inside the seven old/new badge windows.
Eight paired fractional states cover first/last selection, dashboard held
feedback, custom colors and opening. The badge windows are pixel-identical
to their respective resting candidates in every state; pixels outside the
windows are identical between baseline and candidate. Both scenes' SVG
changes also stay inside the windows at 4K/1600. SVG structure and reference
checks pass for the three changed sheets.

The header-frame refinement has genuine local tradeoffs: native 1600
A/B top R150 IoU is .5 versus 1 in the first wide/dim trial, and several
small-size RGB edge controls worsen slightly. The bright 4K frame core
is recovered at R190, with extra R150 fringe pixels. These are retained
profile/phase limitations, not claimed exact matches. Store right-edge
RGB errors improve at all three native sizes. Dashboard tab-interior RGB
errors improve in all twelve native controls; 4K errors fall from
46.8–55.2 to 4.2–10.2 per channel.

All 292 Rust tests pass. The production frames match the reviewed
candidates exactly at all three sizes. Only dashboard (1988 pixels) and
store (1311 pixels) goldens change. All four fidelity gates and all 22 repository checks pass; all 27 visual
cases pass first attempt. Both fresh packaged 4K frames match the reviewed
candidates exactly. The broader NK-05 fidelity task remains open.


Exact ink profiles, photographic glow and fine glyph/counter fringes
remain in NK-05. The header backing is still an approximation: clipping
both violet masks to the new outlines improved A's source RGB error but
worsened B's, so that material change was deferred. The B lower corner
retains a small local overlap loss. Footer C/D top edges and some store
edges fall below strict thresholds at smaller sizes; ordinary bright
edge geometry must not be inferred from one threshold alone. Nearby
footer annotation widths/heights also remain separate typography work.
