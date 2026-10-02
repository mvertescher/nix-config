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


## AQ compliance cap-height audit

The K5 follow-up tests the two card compliance lines, not the shelf brands,
certification marks or footer. A first endpoint audit finds only 1–2 native
pixels of horizontal difference. Local-background normalization exposes
a more specific vertical issue: at 50% of foreground contrast, ordinary
card 1's second-line source cap spans y1350–1360 (11 rows), versus native
y1348–1360 (13). Card 3 spans source y1350–1361 (12), versus the same
13 native rows. Fixed M/A crops and neighboring first-line O controls
repeat the upper-fringe mismatch. It persists at 35–65% contrast.

Selected card 2 supplies a different phase control: its second line starts
at y1689 in both source and native, but native reaches y1701 versus source
y1700. This supports a bounded cap-height/edge study, not a common baseline
shift. A suggested horizontal-width issue disappears at stronger contrast
and is not established. Keep current endpoints, tracking and word gaps.

The native baseline is the AN packaged Kitsch 4K capture, matched to its
reviewed preview. The current Kitsch table and trace still match AN hashes;
AO's outline cache is opt-in to Neomil title echoes and AP changes only
Entropism. Root reruns the source normalization and inspects the literal
source/SVG/native compliance crop. Evidence is in
`/tmp/cp-eras-next/aq-next-audit/`. No Kitsch artwork changes in this audit.

The next scratch lane tests a source-supported vertical fit with separate
SVG/native calibration. Freeze ordinary-card-1 choices before selected,
ordinary-card-3 and visible fourth-card controls. Require first/middle/last
and top/bottom improvements at multiple contrast thresholds without new
word-gap ink or endpoint drift. Exact original font contours and softness
remain separate; a failed local fit must remain unapplied.


## AQ rejected compliance height fit

A scratch .885 vertical factor improves ordinary compliance at 4K without
changing advances or tracking. A bounded per-draw glyph cache reduces its
warmed Scene.draw cost from 66.67 to 27.42 ms (old native: 24.45 ms),
with identical trial pixels. All thirteen state pairs confine their changes
to ordinary compliance runs, and opaque fixed-ink controls pass.

Independent small-size source review rejects integration. At 1600×900,
card 1's old first and second caps already match the source's five rows;
the trial adds a first-line fringe and moves the second line down one
pixel. Normalized source overlap falls at all three thresholds on both
lines, including .6285→.5190 on line 2 at 50% contrast. Card 3 and the
1537×947 controls improve, which does not erase that regression. A height
correction must preserve the native small-text rasterization before it can
ship. All Kitsch artwork and shared renderer changes were restored to the
AP checkpoint; the complete experiment is retained in scratch. Selected
height/phase trials remain rejected separately. See
[round twenty-eight](../reference-svg-round28.md).

## AR rejected common baseline correction

A second scratch trial keeps the .885 height and moves both ordinary
lines by −.36816 design units. This aligns the analytic outline bottom
with the nominal baseline using the font's ascent/descent and Iced's line
placement. The analytic offset applies to both cached and outline text;
it is not a measured difference between their rasterized baselines.

Three native captures reject this common correction. Some 1600px cap
bounds and central-threshold overlaps improve, but stricter local controls
still lose. At 4K, first-line source overlap at green >180 falls
.181118→.168466 on card 3 and .187845→.175532 on the visible fourth card.
Their strict bottom edge ends at y1340 against source y1342. All 13,199
changed 4K pixels stay in ordinary compliance; selected text, brands and
footer remain unchanged. That locality does not resolve the source losses.

The existing implementation stays in place. A common geometric baseline
formula cannot account for the observed card, edge and raster-size
differences. Further correction needs measured edge behavior across sizes,
without introducing an arbitrary resolution cutoff. The frozen trial and
root review are under `/tmp/cp-eras-next/ar-kitsch-trial/` and
`/tmp/cp-eras-next/ar/kitsch-independent-review.json`.
## AS: cap geometry, registration and edge response

The fresh packaged 4K store frame is byte-identical to the AQ/AR baseline
(`decf5f82…`). On ordinary card 1, the first M/A in the second compliance
line occupy source y1350–1360 versus SVG/native y1348–1360 at 50% locally
normalized green contrast. At 1600px both source and native already occupy
five rows. Card 3's source print is one row lower, but its bar and socket
rule also shift by about one row. That is artwork registration evidence,
not support for changing only that card's text baseline.

A continuous vertical scale/shift plus Gaussian response, fit only to
card-1 M/A row sums, gives scale .8543, shift −.121 source pixel and sigma
.7147 source pixel. Card-3 row-profile errors remain small after the
independently measured registration, and predictions at 1600/fractional
size need no refit. This diagnostic does **not** validate a print model:
each glyph has a separately fitted amplitude, row sums omit counters and
gaps, and the material edges have different measured response widths.
Scale/shift without blur explains almost as much; the Gaussian width is
not an identified authoring parameter.

Two-dimensional controls expose the limitation. The modeled first-card
M reaches .860 mask IoU at 50%, A .690 and O only .426; third-card scores
are .427/.453/.290. Source O is rounder and wider than the tall bundled
glyph, and A's source sides also extend farther. A common vertical
transform and blur cannot supply those contours. AQ's outline trial also
changed the rendering path from cached text to filled glyph outlines;
its small-size failure does not isolate TrueType hinting as the cause.

Keep the production compliance print unchanged. A bounded source-derived
glyph contour, with repeated-card, counter/gap and small-size controls,
remains feasible without the original font; selected printing needs its
own calibration. Frozen inputs, row/edge fits, no-refit predictions and
two-dimensional checks are in `/tmp/cp-eras-next/as-kitsch-caps/`.

The subsequent O-only contour trial is also unaccepted. Its rounded outer
boundary and counter improve isolated 4K and downsampled masks, but widen
the O into the unchanged N: the source's one-column central gap becomes
zero on card 1 and sometimes a one-column overlap on card 3. An unchanged
neighbor mask does not establish preserved separation. A single .85-width
revision restores the gap but loses the source right edge and counter;
card-1 strict 1600px overlap falls from .526 to .444. Neither path has been
ported to native rendering. The next bounded route fits O and N together
within a fixed prefix advance, keeping the suffix anchor unchanged. The
source-card registration remains a diagnostic, not a per-card text shift.
See `ordinary-o-trial.md` and its mask/gap data in the same scratch directory.


## AT: paired prefix and shelf registration

The paired O/N vector improves the first-card masks, but remains
unaccepted. The first evaluator split the wider vector O at the old font's
O/N boundary, mislabeling its last pixel as N and reporting a false card-3
collision. Separate glyph rasters restore a real 1–3-column O/N gap.
The actual failures remain: N/L has one clear 4K column against two in the
source; fractional N controls regress; and absolute card-3 prefix overlap
at 1600×900 falls .477→.429 at .50 contrast despite improved registered
shape. No native candidate is accepted. Preserve the original full suffix
shaping if a future prefix is tried.

A separate structural audit finds card-3 horizontal source/native drift
of +1.93 source pixels at the stat bar, with held-out outer/socket edges
at +1.64/+1.63. The fourth card agrees near +2.0. A shelf-origin correction
is therefore a distinct layout investigation, not a text-only adjustment.
The approximately +.9 vertical drift in the border/stat/socket does not
apply to the yellow band (+.07); moving the entire card down is unsupported.
This investigation changes no production artwork.

## AU: whole-card horizontal registration rejected

A stat-left fit proposed moving SVG card 3 from x1123 to x1123.47125
and card 4 from x1443 to x1443.502083, with all y positions unchanged.
Card 1's outer-right edge already agrees within 0.09 source pixel and
rejects its corresponding stat-only fit. The proposed card 3/4 offsets
improve held-out border and socket edges, but fail wider material controls.

At the middle ink thresholds, card 3's band-shoulder F1 falls
.95238→.94198; rifle center/right fall .80939/.74131→.80195/.72963.
All three thresholds agree on those losses. First-line compliance ink
intrudes into 51→68 source-blank columns on card 3 and 6→13 on card 4,
although the second line and lower border turns improve. Card 4's
visible rifle also regresses. The fixed fourth-card cutoff remains
unchanged, but locality alone does not establish a better source match.

The suggested native relative offsets were not captured or integrated:
native card 1's raster phase differs from the SVG, so its relative fit
would need independent calibration. Whole-card movement is unsupported
by the band, rifle and first-line gaps. Preserve those controls in any
narrower geometry investigation; the O/N and cap-height work remains open.

## BC–BD: separate glyph shape from two-line placement

Neither the complete `ONLY` contour nor the later text-block translation
is accepted. Production SVG, Iced code, components and goldens stay at BA.
The source supports a placement difference between repeated text blocks,
but applying that difference to the current font still loses individual
words and gaps. A lower whole-word or whole-line error is insufficient.

BC expands the earlier O/N experiment to one compound `ONLY` path while
preserving the original full-line shaping and C/suffix anchor. The first
candidate improves most whole-word RGB comparisons but worsens card 4 at
4K. One revision moves only L's leading edge by half a source pixel. That
revision improves whole-word RGB error against the production SVG in all
twelve card/size cases, but source-derived O-counter false ink rises
14→17 on card 3 and 16→23 on card 4. N/L gap false ink remains worse too:
39→40 and 38→42. These source-owned masks are later diagnostics, not
predeclared holdouts. The fractional C/suffix control overlaps one changed
Y-fringe pixel before the C boundary; its RGB L1 error rises 11→30. This
is a real gap loss, although it does not establish a changed C glyph.
No native contour candidate is warranted from these results.

The BC source-to-source comparison then fits translation on card 1's
`ONLY`, avoiding the confound of comparing source lettering to Rajdhani.
Relative to the existing card origins, card 3 is approximately 1.49 source
pixels right and .93 down; card 4 is 1.87 right and 1.15 down. The same
transforms improve independent words on both lines. An independent integer
gradient-correlation check agrees across all six card-3 windows: its
design-relative displacement is (1.4, 1) source pixels, and correlation
rises from .186–.285 to .974–.979. Two clipped card-4 windows support the
same direction with less precision. This is evidence for testing text
placement, not permission to move whole cards or selected text.

BD tests exactly one SVG candidate, moving both complete text lines on
card 3 by (.621228604, .385855901) design units and card 4 by
(.777558727, .481019430). Font, ink, tracking, strings, card 1, selected
card 2 and all other artwork remain unchanged. Fourth-card clipping and
fade remain fixed. At 3840×2160, 1600×900 and 1537×947 respectively,
full RGBA comparison finds 15,080 / 3,629 / 3,272 changed pixels, all in
the two intended text blocks, with zero alpha changes. XML review also
confirms that only the text wrapper differs inside each cloned card.

Astra's independent three-size comparison finds better whole-line RGB
error in all twelve cases, while retaining these decisive losses:

| Fixed control | Before | Trial |
| --- | ---: | ---: |
| 4K card-3 first-line false ink in source-blank columns, 50% contrast | 315 | 460 |
| 4K card-4 first-line false ink in source-blank columns, 50% contrast | 53 | 97 |
| 4K card-3 second-line final-word RGB MAE | 27.308 | 28.577 |
| 4K same final-word counter false ink, 50% contrast | 17 | 21 |
| 1600 card-3 whole first-line mask F1, 50% contrast | .5445 | .4944 |
| 1600 card-3 first-line final-word mask F1, 50% contrast | .5986 | .4276 |
| 1537 card-3 second-line final-word RGB MAE | 21.063 | 24.008 |

These root controls are independent review diagnostics. Baseline levels
normalize both render states; the candidate cannot redefine the source
gaps or counters. Source sampling is original at 4K, Lanczos at 1600,
and an explicit uniform top-left affine at 1537×947. The worker's rounded
1537×865 source resize slightly changes vertical scale, so its fractional
numbers are not used for acceptance. Its original endpoint window was
also beyond the text; a reviewer-added final-word window exposes the
loss and is recorded as a later diagnostic. BC's native sampling with a
positive offset is not a rendered right/down correction and supplies no
native-candidate evidence.

Reject the text-block translation before native integration. It confirms
that source correspondence and the current font's geometry must be
considered separately. Future glyph work must cover both complete lines,
all repeated glyphs, their counters, source-blank columns and endpoints;
`ONLY` alone cannot close the cap-height task. Do not repeat either this
translation or the rejected contour as an accepted correction. A combined
shape/placement model would be a new hypothesis requiring independently
frozen controls and separate SVG/native calibration.

Reproducible scratch evidence: `/tmp/cp-eras-next/bc-kitsch-only-review/`,
`bc-kitsch-line-registration/`, `bd-kitsch-text-placement/`,
`bd-registration-audit/` and `bd-root-review/` under the same parent.
The BD candidate SHA-256 is
`382e5f61f9cf1f298b595c55b0d11e12aacfd17a4723817178624334e6c7d573`;
the unchanged production trace is
`d231dddd890588b29ce8e9e45b02d9104b4b2abebfccfbd377d596471f681f4f`.

## BE: FreeSans approximation for two-line store compliance

The source is `images/kitsch-store.png` (#52). The two compliance lines repeat on ordinary cards 1, 3 and card 4, whose right edge remains clipped and faded, and on selected card 2. The prior BC `ONLY` contour and BD whole-text translation were rejected because they worsened source-owned counters, gaps and words; their geometry alone did not solve the type mismatch. A corrected bounded audit measured the source `O` cap at 12 physical pixels (y1330–1341) and fixed an earlier second-line anchor error. FreeSans Bold ranked above the installed Rajdhani, Orbitron, Liberation Sans and TeX Gyre Heros Cn candidates at fixed .35/.50/.65 contrast thresholds, with card-1 word-start RMSE 1.10 physical pixels. Its rounded O/C forms are closer to the photograph. **The source font has not been identified**; FreeSans is an approximation and photographic glow, selected-card material and individual contours remain imperfect. The audit evidence is `/tmp/cp-eras-next/be-font-audit/REPORT.md`.

The first full-two-line SVG trial improved first lines but lost several second-line endpoints, gaps and words. A read-only fit of the six card-1 second-line word starts then suggested +1.26555 physical pixels of origin and −0.10672 physical pixels per-character tracking relative to that trial; it did not use final glyphs or gaps as fit targets. One calibrated SVG candidate incorporates that line-specific correction and a half-physical-pixel ordinary second-line lift. It improves all 24 whole-line RGB comparisons across four cards and three sizes. At the middle .50 threshold, 23/24 whole-line F1 comparisons improve; clipped card-4 line 2 at 1537×947 falls .635→.627. Seven word RGB comparisons worsen, with fractional selected `MANIPULATE,` rising 20.176→24.951 MAE. The 204 overlapping threshold/region loss records are retained; they are not 204 independent defects. The candidate changes only compliance printing, with zero alpha changes. Calibration details are in `/tmp/cp-eras-next/be-font-trial/line2-spacing-study/REPORT.md` and `/tmp/cp-eras-next/be-calibration-review/REVIEW.md`.

Native testing first used a requested 700 weight even though the exact embedded `FreeSansBold.ttf` has OS/2 weight 600. A shaped-glyph font-ID probe showed cosmic-text 0.15 chose DejaVuSans-Bold for the 700 run; requesting 600 selected the embedded FreeSans bytes. A hermetic probe loading only the crate's eleven faces selected FreeSans for either request, demonstrating why the original result depended on host fallback. The 600 request is the deterministic selector. The face's hhea metrics give the existing analytic baseline fraction .95, but cryoglyph's rounded `line_y` and a truncated text-area top put the initial native ink one row high. The scoped direct `paint_text` correction computes the nearest pixel baseline for FreeSans runs only; other faces and the shared text construction are unchanged. See `/tmp/cp-eras-next/be-native-font-diagnosis/REPORT.md`, `HERMETIC-AND-FONT-METADATA.md` beside it, and `/tmp/cp-eras-next/be-native-baseline/REPORT.md`.

After selector and raster-baseline correction, the native candidate improves all **24/24 whole-line RGB** and all **72/72 whole-line F1** comparisons at .35/.50/.65 across 3840×2160, 1600×900 and 1537×947. The fixed 4K whole-line controls below use the same photo and regions as the baseline; positive RGB MAE reduction means improvement. Card 4 retains its clipped and faded edge.

| Card | Line 1 MAE before→after | Line 2 MAE before→after | Line 1 F1 .50 before→after | Line 2 F1 .50 before→after |
| --- | ---: | ---: | ---: | ---: |
| Ordinary 1 | 45.389→26.605 | 33.988→27.939 | .436→.779 | .634→.730 |
| Ordinary 3 | 44.625→23.728 | 36.749→23.628 | .421→.780 | .538→.778 |
| Clipped 4 | 23.675→15.694 | 20.553→17.143 | .462→.722 | .530→.653 |
| Selected 2 | 32.968→21.017 | 22.920→19.334 | .467→.757 | .675→.775 |

The whole-line result does not erase **12 word RGB losses**. The largest are fractional card-1 `ACCESS` (26.031→29.821), fractional selected `MANIPULATE,` (20.469→24.076), fractional selected `THIS` (20.821→23.266) and fractional selected `ACCESS` (22.089→23.836). The fixed audit also records **272 overlapping metric-loss records** across words, holes, blank columns and thresholds; they are overlapping observations, not distinct defects. No separate final endpoint control was scored. Native changes are 32,252 / 7,132 / 6,919 RGBA pixels at the three sizes, with zero alpha changes and zero pixels outside the compliance regions. Full per-region values and losses are in `/tmp/cp-eras-next/be-baseline-review/metrics.json`. These measurements support the printing change while keeping exact glyph shape, soft edges, selected-card texture and local gap losses open.

The calibrated `docs/kitsch/store-trace.svg` shares unchanged ordinary-card art through `#card-art` and keeps the two compliance lines in `#card-compliance`, with only text translations on cards 3 and 4. `docs/kitsch/components.svg` shows the same FreeSans size, origins and tracking on its ordinary and selected specimens. This structure has zero full-RGBA pixel difference from the calibrated SVG candidate at all three viewport sizes, including the top-left 1537×947 viewport; the component render changes only its two specimens and two captions. The scratch integration and hashes are in `/tmp/cp-eras-next/be-svg-integration/REPORT.md`.

The independent cap-row audit keeps the narrower cap-height follow-up open. At
4K and the fixed .50 contrast level, ordinary cards 1/3 and selected card 2
now match the source O's 12-row bounds, replacing 13 native rows. Ordinary
M still spans 12 rows against 11 source rows; fractional card-3 M remains
five rows against four. Some small-size row profiles worsen even when cap
height is unchanged. These source-owned controls separate a useful overall
font fit from exact two-line cap closure. All three fixed thresholds and
row profiles are in `/tmp/cp-eras-next/be-cap-closure/metrics.json`.

The first shared-art SVG proposal accidentally removed the existing
`cards-extrude-clip` definition and animation. Rest-frame parity could not
expose the missing motion. Astra's ID/reference check caught it; production
restores the exact original definition. All 24 SVG ID/reference checks now
pass, and fresh production SVG renders remain pixel-identical to the frozen
calibrated candidate at all three sizes. The corrected evidence is
`/tmp/cp-eras-next/be-production-plan/svg-check.json` and `svg-parity.json`.

Production builds stage the exact `FreeSansBold.ttf` bytes from nixpkgs `freefont_ttf` into the crate's `fonts/` before `include_bytes!` compiles, and use the same package in the development shell. The tested bytes have SHA-256 `982534a3731416a15e2756601721f26053f68bf4239011550f3dd23ce6308215`; they must be the bytes loaded by the app and tests, independent of host font installation. The package ships upstream `COPYING`, `README` and `CREDITS` under `share/doc/cp-eras-ui/fonts/freefont/`; the embedded font retains its own metadata. Its name table identifies GNU FreeFont contributors, GPL version 3 or later and a document-embedding exception. The local extracted ID-13 text is preserved as investigation evidence at `/tmp/cp-eras-next/be-native-font-diagnosis/EMBEDDED-NOTICE.txt`, not proposed as a separate shipped file.

Paired state review now passes **15/15 nonvacuous cases**: first/last category and card choices, ordinary-card and selected-card hover/held material, custom palette and a partial opening with visible text. Each baseline/candidate pair changes only the four compliance regions, with zero alpha and zero outside-region pixels. Six rest captures at 4K, 1600×900 and 1537×947 are full-frame identical to their separately frozen baseline/candidate references. The hover/held cases use synthetic Plate material substitutions; they verify drawing locality, not pointer hit testing. Evidence: `/tmp/cp-eras-next/be-states/locality.json` and `parity-verified.json` beside it.

BE verification passes 293 local and Nix Rust tests, both Kitsch store
fidelity gates, all 24 SVG ID/reference checks and all 22 repository checks.
All 27 visual cases pass on their first attempt: 26 are pixel-identical;
Neo-kitsch bar retains its established one-pixel, one-channel-level residual.
Production store frames at 3840×2160, 1600×900 and 1537×947 match the reviewed
native frames exactly. A fresh packaged 4K store frame also matches exactly.
Only the Kitsch store golden changes, by 7132 pixels. Component differences
stay within the two compliance specimens and two captions at full/half size.
All 314 frozen tracked files match the tested Nix source; nine local
original images remain unchanged. FreeFont COPYING, README and CREDITS in
the package match the pinned upstream source byte for byte. Final documentation
updates preserve that tested runtime/artwork. Changes remain staged and
uncommitted; all 29 broader TODO boxes stay open.

The separate Neo-kitsch NK-05 next lead is repeated store metadata geometry. A scratch five-of-six-line fit improves bounded source comparisons, but upper-C line 1 loses F1 at the stricter threshold and the font/ink mismatch remains. It is not accepted or part of this Kitsch change; see `/tmp/cp-eras-next/be-neokitsch-next/REPORT.md`.

## BX–BY ordinary M transfer

A central-aperture SVG method keeps the original full text run and replaces
only font x230..640/y0..729. Original-M replay and the frozen higher-notch
contour both preserve the outer M, full A/suffix boundary, selected text,
alpha and exterior at three sizes. Astra reproduces 162 RGB metric sets
and 486 threshold sets. Replay still changes central/seam pixels, and the
contour retains a 1600 card-1 whole-line/M loss plus fixed gap losses.
Transfer locality is verified; the contour is not accepted.

Native testing partitions the same full tracked run into five viewports
without changing any glyph. All three actual-Store baseline captures
exactly reproduce production. The 1600 no-op is byte-identical, but 4K
loses four central bottom-row M pixels on card 1, x1176..1179 at y1361.
Cards 3/4, the full suffix, alpha and exterior stay exact. The runner stops
at this failed exact-replay gate; no fractional no-op or new native contour
is tested. This demonstrates a clip-seam failure in rest, not disappearance
of the whole line. Parent-fill ordering in feedback remains a separate
architectural concern, not a captured result.

CA implements a scratch first-glyph painter that retains the intact
line's measured advances and rounded FreeSans baseline and draws in the
parent frame. CC compiles it and proves full-RGBA None parity at all three
sizes. Original-M path replay changes 250/83/78 pixels; 12/5/0 fall outside
the fixed M windows. The full floor(A-left) boundary changes five pixels
each on 1600 cards 1/4 and fractional card 3. Unchanged advances therefore
do not establish unchanged composite boundary pixels. This exact transfer
fails before rendering the proposed contour. No shared renderer change
is integrated. See [round forty-four](../reference-svg-round44.md) and
`/tmp/cp-eras-next/cc-kitsch-m-review/original-transfer.json`.

## CD font-route replay

A separately named FreeSans copy preserves every glyph, advance and
hint program, with only names and the required checksum changed. The
actual Store changes the face of the three ordinary second compliance
lines. Baseline and alias both exactly match BE at all three sizes in
full RGBA. Startup verifies original/alias font bytes and weight 600
under Basic and Advanced shaping; Astra independently checks the tables,
six captures and logs. This provides a verified transfer mechanism.

The frozen 15-point M contour still needs a valid hint strategy: the old
13-point program includes both phantom-point and CVT operands with the
same numbers, plus central instructions tied to the old topology. The
next diagnostic removes only M's instructions while keeping its original
outline, and requires exact replay before any proposed contour. CE fails
that check at 4K: 81 pixels change, including one outside the fixed M
windows. Smaller unhinted frames and the proposed contour are not run.
CG retains the 138-byte hints, remaps only eight point-index operands and
adds two duplicate points without changing the original polygon. All
three fresh baselines and three topology controls are full-RGBA exact;
all 2910 other glyphs stay byte-identical. The actual changed contour
still needs a hint-behavior review before rendering. No font or renderer
change is integrated. See
[round forty-five](../reference-svg-round45.md).


## CI–CK ordinary second-line M

The hint probe preserves the proposed shallower notch. Actual-Store
review at three sizes confines all 166/42/40 changed pixels to the three
ordinary M windows, with the full A fringe, suffix and selected line exact.
Whole-M and whole-line RGB error improves on every card/size. Two tiny gap
RGB losses and card one's strict 1600 threshold loss remain explicit.

The native fit now uses separately named CP Eras Kitsch Sans Bold, generated
from pinned FreeSans by a checked, reproducible recipe. Only M's outline
and its operand-aware hint remap differ; all other glyphs and metrics
remain exact. Only the ordinary second compliance line selects this face.
Source font identity is not claimed. The SVG/component fit remained a
separate calibration at CK. The named face passes six fractional state
pairs, three-size production parity, fresh packaged 4K parity, its Store
implementation gate and the local source controls, 295 Rust tests and
full repository checks. See [round forty-six](../reference-svg-round46.md).

## CM–CQ shared-font SVG M

The Store trace and component sheet now select the already generated
`CP Eras Kitsch Sans` family at weight 600 for only the ordinary second
compliance line. The selected line and every other face stay unchanged.
A same-named original-outline control is full-RGBA exact at 4K, 1600 and
fractional sizes, unlike the earlier SVG clip/path replay. The derivative
changes 173/47/46 Store pixels, all in the three ordinary first-M windows;
Full A/suffix boundary, selected card, exterior and alpha remain exact.
All nine M and notch source RGB comparisons improve. The 1600 card-three
M/A gap still loses 232 RGB L1 and threshold overlap; at `(1131,565)`
source `(56,141,132)` becomes `(44,89,78)` against baseline
`(68,149,131)`. Card one's `(490,563)` remains falsely bright, and the
fractional card-three gap loses six RGB L1 without a mask change.
Two additional SVG-only font contours reduce some losses but weaken
held-out card four, so neither is integrated. The existing licensed,
reproducibly generated face is installed for external SVG viewers, whose
Fontconfig must include the package font directory. Native font/runtime
are unchanged by this SVG correction. CQ package and full checks
pass; broad K5 remains open. See [round forty-seven](../reference-svg-round47.md).
