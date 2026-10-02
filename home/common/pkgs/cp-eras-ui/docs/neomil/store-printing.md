# STORE primary printing and rifle details

The reference is `images/img-09-store.png` at 3840×2160. Trace and Rust coordinates use 1600×900 design units. This pass retains the established shelf opening, selected/cropped card contours, palette roles and interaction coats.

The photo prints `PETROCHEM` inside a 7.2×48.4 design box at card-local `(261.4,182.9)` and `BETTERLIFE TEC` immediately to its right. Both runs turn clockwise. The source PETROCHEM bright threshold spans x697.92..705.00/y182.50..231.67 on card 1; the updated SVG spans x697.92..705.83/y182.50..231.67. The source BETTERLIFE run is heavier and longer than the old trace: changing it to Bold 7.5 with 1.0 tracking increases its bright area in the clean x709..718.5/y182..245 crop from 116 to 474 native pixels, compared with the source's 374, and extends its end to y242.08, matching the source's y242.08. Its bright-threshold IoU rises from 0.04 to 0.31; the repeated printing and glyph shape still differ. Iced now carries both primary runs through `Prim::Turn` and `Prim::Tracked`, including selected and cropped card variants. The source's primary bright printing has p90 red 251 (`#fb3535`), where the old SVG microprint peaked near 150; SVG and the reference-palette STORE foreground use `#fb3535`. Custom palettes retain their own `Fg` role.

The left source margin code has bright native bounds x50.83..56.25/y382.08..468.33. The SVG's prior primary run spanned x50.83..57.92/y377.08..469.17, with an early top edge and extra width. The source's final group reads `0B CP`, with a straight left stem and two bowls in the B; the same literal appears in the unboxed STORE footer and the login margin. Earlier traces and Rust used `08 CP`, which has now been corrected in those runs. The small MASURAO word and 益荒男 brand under it preserve the source rotation. The kanji now uses 16 closed source-traced rings from native x117..133/y1317..1380, inverse-rotated into the existing `Prim::Turn` frame; its supported bright bounds are x48.75..55.42/y548.75..575.00 design units. The Latin word’s old font approximation has since been replaced by compact source-derived stencil contours, detailed below. The main Japanese MASURAO title has since been restored from native-source vectors in the [primary-art pass](store-primary-art.md), without depending on a CJK font or shear primitive. No extra unreadable microtext has been fabricated.

The corrected STORE margin run uses Rajdhani Regular 10.5, tracking 0.4, turned at (56.4,469). At native resolution, a common strong-red mask (R>180, R>2.2G, R>2B) puts the source at x120..134/y917..1123 and the updated SVG at x119..134/y918..1123. Its whole-run IoU rises from 0.043 with the old geometry and corrected B to 0.241. Independent vertical holdouts also improve: `CP/0B` y917..985 from 0.032 to 0.210, middle `54/05` y985..1060 from 0.039 to 0.105, and trailing digits y1060..1125 from 0.061 to 0.490. Source scan echoes and distressed glyph edges remain outside this primary-run fit. Native Iced baseline and width need review; login and footer geometry were retained.

The initial J pass ported the old SVG's `#gundetail` template to Iced. That schematic was then superseded by source-traced five-tone rifle contours for the ordinary and selected cards. The [primary-art record](store-primary-art.md) gives the native mask measurements and held-out threshold comparison. The art remains in the existing `GUN_OUTLINED`/`GUN_SOLID` state derivation, including card 4's permanent x1557 viewport.

The STORE source and SVG use 1 design unit of tracking on nav, title, stat and socket captions. Iced now matches that local tracking. EMPTY/SOCKET is Regular in the SVG and source appearance; its earlier Rust Bold run has been changed to Regular. The source cap heights, footer overflow and some primary label widths still need native fitting after this material/printing checkpoint. The STORE card/nav material proposal remains separate in `/tmp/cp-eras-completion/neomil-store-material-proposal.md`; no STORE material recipe was applied in this printing pass.

Source/trace review crops and candidate measurements are in `/tmp/cp-eras-completion/neomil-store-printing-review.png` and `/tmp/cp-eras-completion/neomil-store-better-bold.png`. J passed 265 Rust tests before the source-traced art integration; native acceptance of the latter remains pending.

## Integrated validation — 2026-09-29

N integrated validation confirms the restored rotated/margin printing and source-vector margin CJK. Both frozen and live store matrix cases now match 100.000%, eliminating the former fallback-font discrepancy. Primary printing presence is complete; Latin margin slant/placement and faint repeated/scan detail remain local follow-ups.

## Q native calibration

Q confirms the corrected B in the store and login, including the retained
login margin geometry. The store margin's native primary run sits about
one pixel right of the source; shifting its Rust origin one native pixel
left/up improves whole-run mask IoU from .176 to .398 at R>180. Independent
thresholds at R>160 and R>200 also improve (.193→.470 and .157→.333). The
source-fitted SVG stays fixed. The unboxed footer needs only a one-pixel
Rust baseline lift: mask IoU rises .229→.341, whereas moving it left worsens
that result. These are candidate renderer corrections pending the next
native capture, not a claim that fine glyph edges or echoes match.

## Card-edge PETROCHEM spacing — 2026-09-29

The unchanged primary run ended too early inside its box: after excluding
the box strokes, the terminal M reaches native y549 in both ordinary and
selected source cards, but y532/533 in the frozen R SVG and y533 in R native.
Increasing only PETROCHEM tracking from 0.5 to 1.3 design units puts the
scratch SVG terminal at y548 in both cards, without colliding with the box.
On locally registered bright-glyph masks, ordinary-card F1 rises
0.191→0.400 and the independent selected-card F1 rises 0.350→0.556;
upper and lower halves improve in each. Local x registration accounts for
the existing card placement offset during measurement and is not part of
the change. The candidate crop and method are recorded in
`/tmp/cp-eras-resume-20260929/s-store-margin/findings.md`. Rust and all
store trace/component card examples use the same 1.3 tracking; native
verification remains pending.

BETTERLIFE TEC needs no shared slant or position change: its ordinary
native bright-core x centers at the first, middle, and last letters match
the source within a fraction of a native pixel, and its y extent already
matches. The selected card has a separate frame placement difference;
moving this common text run would degrade the ordinary match.

## Native R code calibration

Fresh source/SVG/Iced crops accept the `0B` correction in both screens.
The store margin uses a separate one-native-pixel runtime shift in each
axis; its bright-mask source/native overlap improves .176→.398. Native
bounds are x120–135/y916–1122 against source x120–134/y917–1123. The footer
moves one native pixel upward, retaining its fitted width: native and SVG
bounds are x753–959/y2082–2096 against source x752–958/y2082–2096. Source
photographic echoes and exact glyph density remain separate. Fractional
rest and custom-palette captures preserve semantic footer ink. The R
source and implementation gates pass; final integrated validation is pending.

### T PETROCHEM native review

The updated Iced run reaches the photographed lower-letter region in both
ordinary and selected cards, retaining clearance inside its box. Fractional
held ordinary and selected captures preserve dark feedback ink and the
card clipping. Both fidelity gates pass. The selected card's independently
measured frame and side-printing offset is a separate open correction;
tracking does not justify translating its otherwise aligned contents.
The integrated T check passes all 22 checks and all 27 visual cases exactly.

## Directional socket echoes, 2026-09-29

The photographed 25-cell scatter codes have a second printing pass: ordinary card 1 trails left, ordinary cards 3/4 trail right, and the selected card trails down. The primary 3×3 cells, 3.6667-unit pitch, independently fitted origins, inks and hit geometry remain unchanged. The reference trace now adds a source-color `#fb1818` echo behind each cell through a shared horizontal falloff and a continuous vertical scan-band mask. The card-4 ordinary echo is shifted up 0.4167 design unit, fitted as a constant phase from its first isolated tail. The selected downward tail uses its own roughly five-native-pixel scan period and decay, repeated from one isolated cell. These compact gradients reproduce the broad direction and dark bands; their regularity and edge exposure do not claim exact photographic grain.

Scratch source/V-SVG/candidate measurements are in `/tmp/cp-eras-resume-20260929/z-store-echo/findings.md`. Across changed pixels, card-4 baseline/smooth/banded-with-phase RGB MAE was 26.95/15.98/15.80; held-out row 4 improved from 19.39 to 16.88 and row 8 from 15.86 to 15.07 versus unshifted banding. Selected downward periodic printing improved smooth 13.83→12.73, including a disjoint top-row column 10.56→7.36 and bottom-row 13.22→11.77. Some supported bright pixels and dark-gap negatives remain worse than their alternate approximation, so this is a bounded follow-up, not exact scan-texture closure.

`store_material.rs` places idle echoes inside the existing card soft composite and opening clip; the source-only feedback state table redraws the same local pattern after the opaque hover/held coat and immediately before unchanged `Prim::Dots`. Constant-opacity `Prim::Motion` wrappers fade `Ink::Fg` and its existing held `card_ink` color without a new renderer primitive or image layer. `Style::store_states_for` uses the same full-palette/source-backdrop eligibility as `store_layers`; custom palettes and other eras continue to use their existing semantic states. The fourth-card viewport still clips the last card. The source selected card is card 2; alternate picked slots use the same grown/downward profile as an inferred interaction state. Ordinary card 2 uses the ordinary-left profile while unselected, also inferred. Native RGB, performance, alternate-selection and feedback checks remain pending the serialized build/capture.

### AA native and feedback review

The final continuous SVG is verified against the native Iced result, not
just the earlier strip-based scratch trial. Source RGB error on a fixed
echo footprint improves on all four cards (19.76→11.92, 26.46→11.95,
20.09→12.51, 23.34→14.47); bright primary cells remain unchanged. All
19,165 native changes are inside the four socket clusters. Ordinary dark
gaps retain small residual errors, and exact grain remains approximate.
All selected slots, ordinary/selected/fourth-card feedback, custom palettes
and early opening are reviewed. Custom rest is pixel-identical to the
previous state. Ordinary feedback adds about 5.27 ms in the paired local
CPU geometry probe; selected feedback adds .13 ms. See the
[AA checkpoint](../reference-svg-round12.md) for method, gate results and
integrated validation. This correction left the separately misplaced small MASURAO margin word and adjacent rule for the bounded contour pass below.


## Small margin word and slash — bounded source contour pass

The former long 107-unit rule crossed the source word and accepted margin kanji, while the italic font word landed much lower than the photographed stencil. A native-source crop isolates the six clear Latin glyphs and a short lower slash. The revised trace and `store_margin.rs` retain the existing `translate(52,584) rotate(-90)` frame and leave the margin-kanji definition and use unchanged. The six letter paths are simplified from a strong red mask (R>100 and R>1.5G/B): components under four native pixels are removed, only one-pixel diagonal breaks are joined, and closed boundaries are simplified by one native pixel. This yields 20 compact nondegenerate rings and 181 vertices, with no bitmap or photographic grain path. The slash is a measured 1.8-unit stroke between local `(8.94,-5.19)` and `(3.00,5.06)`. Rust uses semantic `Fg` for the word, slash and provisional hatch, with a constant .67 opacity on the hatch outside the rotated group; the dark O ring uses semantic `Bg`. Custom palettes retain these ink roles.

On fixed native-resolution source/SVG crops, first-to-last glyph bright-mask IoU improves M .009→.633, A .022→.656, S 0→.679, U 0→.482, R 0→.626, and second A .042→.563. False bright pixels fall on all six (ranges 70–141 before and 37–59 after). The slash improves .119→.667, and the untouched kanji path improves .496→.947 because the old word and rule no longer overprint it. The terminal O is now a dark ring with a red counter over clipped diagonal hatch, but the broad plaque material and second-A junction remain approximate: its local RGB error falls 31.83→25.00 while false bright pixels rise 48→153 and bright intrusions into source-dark gaps rise 3→37. A rejected solid plaque fill raised those intrusions to 76. This is a primary word/placement correction, not exact O/plaque or fine photographic echo closure. Scratch source/current/candidate crops and per-glyph counts are in `/tmp/cp-eras-resume-20260929/aa-masurao-margin/`; native renderer validation is pending.


### AB native confirmation

The native word/slash correction changes3054 pixels at 4K and 699 at 1600×900,
all within the margin. Nine matched production/state comparisons change
only that region. Fractional card-held/selection/opening margin crops are
identical to fractional rest. A custom cyan foreground renders423 cyan
margin pixels and zero strong red pixels, including the hatch. Native
per-glyph RGB errors improve on all six letters; CJK overlap rises
.485→.950 and slash overlap .121→.727. The native O/plaque RGB error falls
32.06→25.55, but source-dark-gap intrusions rise3→55. This and the A2
junction remain explicit residuals, not closed exact-material work.
Both store gates and the full AB check pass: 286 Rust tests, all 22
repository checks and 27 exact visual cases on their first attempt.

### AC leading plaque boundary

The rectangular hatch end extended beyond the source's slanted leading
edge. Trimming that edge changes only 126 native 4K pixels and 32 at 1600
or1537×947. Leading-patch RGB error falls 20.20→14.52; bright intrusions
into source-dark pixels fall 88→4. The O/plaque crop improves25.55→23.09
and 55→6 intrusions, while the O counter and A2 are unchanged. Ten native
captures preserve card states, opening, clipping and semantic custom ink.
Both gates, 286 Rust tests and all 22 repository checks pass. All 27 visual
cases match 100.000% on their first attempt. The
[AC checkpoint](../reference-svg-round14.md) records the rejected wider
clip/dimming proposals and preserves the remaining fine-material limits.

### Local plaque stroke fit

The source's middle and lower plaque stripes are broader than AC's 0.45-unit
trace. Each diagonal now uses a 0.60-unit foreground stroke only between
local x=89 and x=99; its ends retain 0.45. The existing slanted boundary,
O ring, six letters, slash and CJK are unchanged. This width profile uses
the same semantic foreground ink and constant hatch opacity as AC.

At 3840×2160, the source/SVG RGB error in the fixed plaque crop
x112..140/y1145..1188 falls 23.02→21.53. Its independent middle and lower
sections fall 22.12→21.06 and 33.02→29.66, while the upper section falls
13.98→13.90. Strong source-core matches rise 180→247 without increasing
bright intrusions into source-dark pixels (4 in both). The leading crop
falls 14.11→13.76; the separate A2 crop remains 20.07 with 8 dark-gap
intrusions. All 665 changed SVG pixels lie inside x112..135/y1147..1196.
The SVG split itself changes a few antialias pixels, so these measurements
describe the rendered candidate, not only its nominal stroke width.
The 4K Iced capture changes 184 pixels, all at x112..136/y1163..1187.
Native plaque RGB error falls 23.09→22.00; the first section is unchanged,
the middle improves 22.09→21.26 and the last 31.82→29.36. Strong source-core
matches rise 275→312. Bright intrusions into source-dark pixels increase
6→7, and total bright false positives 185→200, so the wider coverage is
not an exact ink recovery. The A2 crop and the actual slanted leading
boundary are pixel-identical. The broader leading-region crop includes
some widened stripes and improves 14.52→14.44. Source/SVG/native evidence
is in `/tmp/cp-eras-next/ad-neomil/`. Ten native captures cover 4K/1600,
fractional rest, ordinary/selected/fourth-card held, last-card selection,
custom palettes and early opening. All matched changes stay in the hatch
region. The margin is invariant under shelf-only states; the custom
foreground crop contains 434 cyan pixels and no strong red pixels.
Both store gates, 286 Rust tests and all 22 repository checks pass, including
27 exact visual cases. All 201 frozen file hashes match the Nix source.
See the [AD checkpoint](../reference-svg-round15.md); the fine O/plaque
material and second-A junction remain open.

### Second-A counter slit

The photographed second A has a narrow dark counter continuation at native
x126/y1198..1202. The AD native render filled four of those source-dark
pixels with strong red while the adjacent x125/y1198..1200 stroke was already
bright and correctly placed. A 0.25-unit semantic-background stroke along
local `(84.6,0.75)` to `(82.8,0.75)` restores that dark column without
changing the adjacent stroke or accepted letter contours, O ring, hatch,
slash and CJK. The trace and component examples use the matching butt-ended
background stroke.

The full-frame SVG changes five native-resolution pixels at x126/y1198..1202.
Its A2 crop x112..140/y1188..1208 improves RGB MAE 20.070→19.882 and
source-dark intrusions 8→6, retaining all 56 strong source-core overlaps.
The Iced scratch capture changes the same five coordinates; A2 RGB MAE
improves 24.426→24.027, dark intrusions 15→11, and strong source-core
overlap remains 65/83. The tighter counter crop improves RGB MAE
29.739→27.712, intrusions 9→5 and retains 21/23 strong-core overlap.
The bright x125 neighbor, broader junction, O counter, leading and middle
hatch, and other-letter controls are pixel-identical. Source, SVG and native
per-pixel evidence is in `/tmp/cp-eras-next/ae/neomil/`. Ten integrated
captures preserve selection, held, custom-palette and opening behavior.
At 1600×900 and the fractional size only two pixels change; every delta
stays in the counter. The custom margin retains 434 cyan pixels and no
strong red pixels. Both store gates pass. This only corrects the
small counter slit; the broad A2/O junction and plaque material remain open.
The [AE checkpoint](../reference-svg-round16.md) records the passing full
repository check, 286 Rust tests and 27 exact visual cases.

### AK upper-O aperture study

Whole-ring translations, uniform lighter ink and upper-biased gradients
are rejected. The gradients improve average RGB error but lose valid
dark core: upper footprint F1 at red below 50 falls .5915→.5455. Four
lower pixels change despite identical rounded error. A2, other letters
and CJK remain unchanged. Detailed rejection evidence is summarized in
[round twenty-two](../reference-svg-round22.md).

A shorter local counter shoulder is a provisional geometry candidate.
Source x129 is bright in native rows 1176–1179 while the adjacent outer
stem at x130 stays dark; row 1175 at x129 also remains dark. The proposed
inner opening widens .27 design pixel along a short segment, preserving
the outer contour and semantic background ink. Five SVG pixels change,
all improving RGB error. The fit and one informative holdout pixel improve;
lower O, A2, remaining word and CJK controls are pixel-identical. Whole-O
RGB MAE changes 30.1880→30.1102. Dark-footprint F1 improves at thresholds
40/50 and ties at 60/80/100. This tiny photographed glyph provides weak
independent evidence; it cannot establish the entire O's exact contour.
The corrected native trial changes three pixels at x129/y1177–1179.
Each improves RGB error: 80.67→41.67, 44.33→5.33 and 53.00→16.33.
Whole-O dark-mask F1 improves at thresholds 40/50/60/80 and ties at 100;
every other native pixel is unchanged. There is no independently changed
native holdout pixel. At 1600×900 the image is pixel-identical. The before
capture matches the accepted AE store at both sizes. Five fractional
states each change only pixel (51,471): rest, held ordinary card, last-card
selection, custom colors and early opening. Other feedback deltas are
unchanged. The custom margin remains cyan with no bright red pixels;
the aperture keeps semantic ink. All six affected gates, 288 Rust tests and 22 repository checks pass;
all 27 visual cases pass first attempt.
Evidence: `/tmp/cp-eras-next/ak-neomil-margin/`.

## AL: missing ordinary-title printing

Source/SVG/native comparison confirms that ordinary `MAGNUM 650` titles
on cards 1/3/4 omit the photograph's repeated horizontal glyph marks.
For example, native rows 452–454 contain a distinct upper band; counts
of source-red pixels in a shifted-title footprint drop sharply at row
455 and recur lower. The selected card has different printing and is not
evidence for an ordinary-card template. Primary logos, certification,
rifles, socket tails and the margin remain separate, accepted artwork.

A scratch full-title copy at (+2.5,−2.9167) design units and .25 opacity
reaches 69/83/40 source-dark counter pixels on cards 1/3/4; 19/25/34 of
those pixels worsen. It is rejected. Clipping the same copy to the upper
three native rows preserves primary cores and counters, and improves
fixed title-crop RGB MAE 15.221→14.947, 12.468→11.955 and 13.483→13.083.
However, it worsens 95/27/49 source-dark gap pixels, with mean losses
about 13.3/5.1/9.8 RGB levels among those pixels. The resulting isolated
bars do not recover the interrupted glyph printing. This trial is also
rejected despite its improved average.

The narrow trial changes 1,639 SVG pixels, all within native rows 452–454.
Selected title, margin, certification and subtitle controls are identical.
Card 1 supplied the initial fit; cards 3/4 were inspected during exploration
and are transfer controls, not blind holdouts. No native candidate was
built and no trace or runtime change is accepted. The AK native baseline
was compared directly with a fresh 4K SVG render.

The next source-supported task is a model of the repeated glyph contours
and interrupted rows, frozen before judging other cards. Counter and true-gap
controls must accompany average RGB error; neither a global title shift nor
isolated scan bars are sufficient. Source/SVG/native montages, frozen input
hashes, both rejected trials and reproducible measurements are retained in
`/tmp/cp-eras-next/al-store-printing/`.

## AM: interrupted title impressions

A two-impression study covers faint native rows 443–449 and the stronger
interrupted printing through 484. Card 1 supports shifts near (−3,−6)
and (−6,−12) native pixels with a five-row strong/weak cadence. The first
shared shift fails transfer to cards 3/4. Subsequent fitted horizontal
registrations are +3 and +9 pixels; those are exploratory observations,
not blind holdouts. All impressions use glyph contours and scan bands,
without photograph pixels or procedural noise.

The coherent model improves fixed title-crop RGB MAE on cards 1/3/4 from
13.865/11.582/12.550 to 11.695/9.340/9.857. Source-echo masks improve
956/979/1784 pixels, with 0/0/1 losses. These gains do not settle acceptance:
288/221/255 old-primary-core pixels, 9/2/0 source-dark counter pixels and
54/51/165 source-dark gap pixels worsen. Card 4's earliest rows account
for 108 gap losses. The scan bars remain harder and more regular than
the photograph. A broad primary-core knockout is separately rejected
because it erases much of the real repeated impression.

Astra's severity review retains the complete source-supported candidate
for native calibration; no store SVG, Rust or golden changes yet. All
4,122/4,211/4,144 fully opaque #fb3535 title pixels are unchanged. The
"old core" threshold also includes antialiased edges: their worsened pixels
have mean RGB losses 2.34/2.92/3.79, maxima 11.33/11.67/14.67. The few
counter losses have maxima 3.00/3.33/0, and those counters contain real
source echo ink with a net improvement. A broad or shared counter cutout
is therefore unsupported. The earlier contrast≥200 diagnostic is empty
(#fb3535 contrast is 198); the nonempty exact-color masks establish
opaque-primary preservation.

Worsened source-dark gaps have mean losses 3.26/4.52/3.71 and maxima
13.33/23.33/19.00, concentrated in the early rows. These localized losses
and hard scan edges remain controls for the next native trial; raw loss
counts alone do not justify throwing away the recovered full impression. The 12,400 changed scratch pixels are confined to
ordinary titles; selected card 2 and all other artwork remain identical.
Full definitions, frozen hashes, source/SVG/native montages and scripts:
`/tmp/cp-eras-next/am-store-echoes/`. See
[round twenty-four](../reference-svg-round24.md).

## AN native title impressions

AN ports the complete AM two-impression model into the ordinary title
lists. Each displaced full title is clipped to its measured scan rows and
painted before the registered primary text. The selected title is unchanged.
Held variants use the existing dark feedback ink; ordinary and hover
variants retain semantic foreground ink. The native baseline remains
203.25 versus the SVG's 204.5. Native review retains the SVG opacity values.

The current-source preview matches the verified baseline at 4K and 1600
exactly. A capture compiled from the actual modified era tables then
matches the initial projected candidate exactly. This second compilation
caught six socket-echo insertion offsets that needed moving with the new
title entry; those offsets and their existing regression assertions are
updated. Selected and outer fourth-card indices are unchanged.

Smaller-size and state review then catches two rendering issues. A vendored
one-line Iced fix preserves the mesh index offset when a scan clip snaps
empty. Ordinary opaque backgrounds use an earlier separate draft so they
cannot cover the child title clips in custom or feedback states. The draft
inherits the existing shelf/fourth-card clip; the shared Scene renderer is
unchanged. All sixteen final state pairs keep selected titles and opaque
primary ink exact, with visible ordinary impressions and no changed pixels
outside their title crops. Both fixes and their controls are detailed in
[round twenty-five](../reference-svg-round25.md#fractional-clipping-defect).

| 4K native title crop | Card 1 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: |
| Source RGB MAE, before | 13.876 | 12.074 | 12.932 |
| Source RGB MAE, after | 12.124 | 10.076 | 10.135 |
| Opaque primary pixels | 4,498 | 4,481 | 4,465 |
| Changed opaque primary pixels | 0 | 0 | 0 |
| Echo-only pixels improved / worsened | 867 / 0 | 860 / 0 | 1,651 / 1 |
| Source-dark counter pixels improved / worsened | 28 / 6 | 40 / 5 | 4 / 0 |

These use the AM fixed title crops, with source red contrast defining
bright/dark controls. Native baseline masks separate opaque `#fb3535`,
fringes, filled glyph counters and gaps outside the primary. All 11,353
changed pixels lie in the three ordinary title crops; the selected title
and all other regions are identical. The integrated SVG is byte-identical
when rendered to the frozen AM candidate. Its scan definitions and the
ordinary component excerpt match structurally.

The model still has local losses. Worsened primary-fringe mean/max RGB
losses are 1.77/3.67, 1.46/4.33 and 1.89/6.67. The five worsened dark-counter
pixels on card 3 lose at most 10.67; the corresponding card-1 maximum is
.33. True-gap losses number 43/47/185, with maxima 8.33/18.67/21.00;
0/0/12 reach red contrast 40. Each entire gap mask nevertheless improves.
The printing's exact phase, softness and lower repetitions remain open.
No counter knockout or source-pixel exception is introduced.

The additional clips and glyph construction increase measured warmed
4K `Scene.draw` CPU time from 20.734 to 26.280 ms in the final candidate
(20.669→25.315 ms before the rendering corrections). A width-shaping shortcut
does not materially improve it and remains unapplied. See the concrete
[glyph reuse follow-up](../../todo/performance.md#store-title-printing-cost--an).
Integrated state, gate and repository validation is recorded in
[round twenty-five](../reference-svg-round25.md).


## CJ lower second-A gap

Two short ground-colored strokes separate the lower A2 junction while
preserving the photographed bright crossing. The continuous slit trial
is rejected. The accepted geometry is synchronized in native drawing,
Store trace and the component specimen. Native source review at 4K,
1600 and a fractional viewport changes 8/2/4 pixels; every changed pixel
improves RGB distance, with all exterior pixels and alpha exact.

Six paired fractional drawing states preserve the correction through
hover, held, fourth-card selection, custom colors and opening. Both rest
wrappers exactly replay the actual Store. SVG rasterization retains a
three-level 1600 RGB loss and one fractional high-threshold crossing,
although regional RGB error improves. Those limits and the separate native
results are retained in [round forty-six](../reference-svg-round46.md).
Other margin ink and photographic printing remain open. CN completes
production/package parity, both Store gates, 295 Rust tests and all
22 repository checks with 27 first-attempt visual cases.

## DS–DV ordinary subtitle impressions

Wider source context reveals displaced `HAND GUN` printing, separate from
the accepted ordinary title impression. The selected card supplies no
ordinary-copy template. DS's common shift fails cards 3/4 at 4K. DT reuses
the accepted title's relative per-card registration, giving subtitle shifts
(−5,−6), (+1,−7), (+7,−7) original pixels at .25 opacity. Its upper-only
clip improves averages but truncates the source-supported contour.

DU retains the full shifted glyph. All three extended-crop averages and
fixed gap/counter totals improve at both sizes, but card 3's 4K source-dark
RGB L1 rises 395,938→397,937. Its 361 locally worsened dark pixels include
a 22.33-level mean RGB loss. The new row-499 pixels overlap a former title
crop boundary, while every fully opaque primary pixel remains exact.

The actual scratch Iced trial extends the existing echo list without
changing outer card indices or the renderer. It retains semantic inks and
native baseline 222.4. Baseline captures match the latest package at 4K,
accepted CJ frames at three sizes and the 1600 golden. All candidate
changes stay in ordinary crops, with exact alpha, selected and opaque
primary pixels. Native card 3 still has 369/84/51 dark-pixel losses at
4K/1600/fractional size, with worst losses about eighteen RGB levels.
No runtime correction is accepted from the improved averages.

DV freezes a five-row opacity cycle, fitted only to card-1 model-owned
samples without source-color filtering. The resulting strengths are
.35/.299787/.220158/.266557/.35; they do not introduce off rows. Card 3's
4K source-dark error increases again, to 403,021, and its worst loss reaches
31.33 levels. The candidate stays in scratch and receives no native port.

The next task must explain the local subtitle registration, contours and
interrupted printing before another complete-copy port. Common translation,
upper-only fragments and the tested five-phase amplitude model are already
rejected. Do not carve out individual failed source pixels. Existing
inspected cards are transfer controls, not blind holdouts. See
[round fifty-four](../reference-svg-round54.md) for the exact review boundary,
scripts, manifests and source/SVG/native evidence.
