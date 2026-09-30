# Dashboard blade-label fit

The source's six blade words are broader in stroke and mostly narrower in
extent than the earlier Rajdhani Regular 19 / tracking 2 trace. This pass
keeps the words, inks, card contours, rose face fields, ghosts, motion and
plate hit regions unchanged. Each visible run now uses bundled Rajdhani
Medium with its own local coordinates; the same run geometry is used in the
idle, selected and pressed branches, so state changes cannot move a label.

| Word | Local center `(x,y)` | Size | Tracking |
| --- | ---: | ---: | ---: |
| VEHICLES | `(0.83, 5.25)` | 21 | 0 |
| WEAPONS | `(1.67, 5.67)` | 20 | 1 |
| Left PRODUCTS | `(-3.33, 6.50)` | 20 | 1 |
| Right PRODUCTS | `(-4.17, 8.58)` | 20 | 1 |
| EVENTS | `(-1.25, 6.50)` | 20 | 0.75 |
| LOCATIONS | `(-2.50, 6.08)` | 20 | 0.5 |

The fit used `images/kitsch-dashboard.png` and native-size (3840×2160)
librsvg 2.62.3 renders of the accepted U dashboard trace. A card was
rectified to its own 162×50 frame; the first, middle and last third of the
word along the card axis were scored separately. The grid covered bundled
Regular, Medium and SemiBold; sizes 19–23; tracking 0–2; and local
translation. It fit the middle region at two conservative ink thresholds,
then checked the first and last regions. The final settings were chosen from
the balanced candidates after those checks, so the endpoint results are
diagnostic glyph holdouts rather than a second untouched validation set.

For idle mint, the mask requires green above 220, blue above 165, and green
above 1.2×red within `|t|<70, |v|<15`. This excludes the teal face and most
photographic halo. At that threshold, F1 overlap on source versus trace
changed as follows (first / middle / last):

| Word | Earlier Regular 19 | Fitted Medium |
| --- | --- | --- |
| VEHICLES | .059 / .125 / .083 | .760 / .641 / .674 |
| WEAPONS | .088 / .196 / .200 | .744 / .675 / .588 |
| Left PRODUCTS | .138 / .078 / .090 | .723 / .524 / .632 |
| Right PRODUCTS | .064 / .059 / .033 | .621 / .556 / .494 |
| LOCATIONS | .370 / .153 / .070 | .483 / .715 / .331 |

The gains hold at green thresholds 205 and 235 as well. Selected EVENTS
needs a separate dark-ink mask on its yellow face. At red below 180, green
below 150 and blue below 40, its first/middle/last overlap changes
`.757/.448/.156 → .713/.749/.628`. Its first glyph loses .044 while the
middle and last improve substantially. SemiBold scores slightly higher in
some dark masks but visibly over-inks the selected word; Medium is the
bounded choice. The selected photograph is lighter/softer than the fixed
trace ink, so exact core-pixel counts cannot establish font weight alone.

The scratch candidate `v-k-labels/candidate-events-medium-075.svg` and
source/U/candidate native crops were reviewed before transfer. The source
retains glow and photographic softness; this fit addresses the letter
geometry, not those image effects. RSVG verifies the trace candidate;
native Iced glyph placement and state transitions need their own capture
checks. No observed idle EVENTS source exists; its off branch uses the same
Medium run as the selected branch with the existing mint ink.

## Native glyph-origin calibration

The V native Iced capture retained the fitted Medium contours but rasterized
four label origins slightly below the correctly placed SVG reference.
Source/native mask comparisons at three mint thresholds fitted the first
third of each run and checked the middle and last thirds independently.
The runtime label origins alone move in local card coordinates: VEHICLES
`(0,−0.5)`, WEAPONS `(−0.25,−0.5)`, and both PRODUCTS `(0,−1.0)` design
units. In the pre-W projection, central-threshold middle/last F1 scores change
`.520/.378 → .572/.528` for VEHICLES, `.590/.521 → .722/.578`
for WEAPONS, `.392/.416 → .530/.641` for left PRODUCTS, and
`.387/.345 → .541/.520` for right PRODUCTS. The gains hold at all three
tested thresholds. The SVG trace keeps its source-fitted origins; both
runtime on/off branches share each calibrated position. Proposed EVENTS and
LOCATIONS offsets failed the middle/last holdouts and were rejected.

The W native 3840×2160 rest capture verifies those four runtime-only
calibrations against the V native baseline. At the central mint threshold,
the first/middle/last F1 scores change as follows:

| Run | V native | W native |
| --- | --- | --- |
| VEHICLES | .722/.520/.378 | .772/.573/.524 |
| WEAPONS | .707/.590/.521 | .780/.725/.590 |
| Left PRODUCTS | .479/.393/.416 | .752/.533/.635 |
| Right PRODUCTS | .419/.387/.345 | .695/.555/.547 |

Every first, middle and last segment improves at all three tested mint
thresholds. EVENTS and LOCATIONS remain unchanged in W native, as intended.
These are mask-overlap measurements of native glyph placement; the source's
photographic bloom and softness remain outside this calibration.

## Store shelf-brand native fit — 2026-09-29

The store SVG already gives `PETROCHEM` and `BETTERLIFE TEC` broad, bold lettering, but the runtime still printed unscaled size-8 text; the second run was Regular. In the frozen T native capture, the top run's bright core ends 39 native pixels before source on each of the first three cards. The lower run on ordinary card 1 ends at x1662 instead of source x1724, and is visibly too thin. The literal content and the compliance sentences agree with the source; this correction is limited to the two shelf-brand runs.

A scratch StorePreview compiled against the frozen AA rlib substituted `Prim::Wide` for only these two `Prim::Text` runs. Card 1 set the geometry, while selected card 2 and ordinary card 3 checked the same settings. The runtime uses Bold size 8 for both, top stretch 1.42 at `(163, 78.416667)`, and lower stretch 1.50 at `(160.416667, 89.416667)`. Ordinary and selected branches share those coordinates and keep their pre-existing inks. The source-fitted SVG remains unchanged because its transform and tracking already place the text near the photographed extents.

At 3840 × 2160, the top bright-core box becomes x1558–1683/y699–712 versus source x1558–1684/y699–712 on card 1. Selected card 2 reaches exactly x2326–2451; card 3 differs by at most two pixels at the right edge. Source/native F1 at the central bright threshold improves .382→.729, .392→.818, and .375→.684 on cards 1–3. The first, middle, and last thirds each improve on all three cards and at the tested bright thresholds 170/180/190.

The lower dark-core box on ordinary card 1 becomes x1554–1725/y726–738 versus source x1556–1724/y725–738. The selected and third-card right endpoints agree within one pixel. Central-threshold F1 improves .076→.311, .148→.427, and .106→.394 on cards 1–3, with each third improving over the old native run. At the strictest ordinary-card-1 dark threshold 145, source has only 90 core pixels and F1 remains about .05; this is a material/photographic contour limit, not evidence of exact letterform recovery. The trial still prints darker, sharper glyphs than the photograph. Scratch source/SVG/T-native/trial crops and threshold results are in `/tmp/cp-eras-resume-20260929/ab-k5-audit/compare.json` and its `top-trial2-3x.png` / `bottom-trial2-3x.png` panels. The production port matches trial2 pixels exactly in all three brand regions. Fractional ordinary/selected/last-card held and custom-color captures preserve the original inks and clipping. Both store gates and the full AB check pass; see the integrated acceptance below.

### AB integrated acceptance

The source/native/state review is integrated: all 286 Rust tests and 22
repository checks pass, including 27 exact visual cases on their first
attempt. All 199 frozen file hashes match the Nix source. This closes the
bounded AB correction above; its stated photographic/glyph limits remain.
