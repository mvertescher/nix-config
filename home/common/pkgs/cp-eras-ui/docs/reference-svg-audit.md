# Reference SVG audit, 2026-09-21

The references retain their screen identities and broad layouts, but
**none of the four eras is complete at source-detail fidelity**. All 16
source gates pass despite missing text, simplified primary artwork,
incorrect contours, typography differences and approximate materials.
Tasks are in [the correction plan](../todo/reference-svg.md); existing
Neomil work remains in its four screen records.

Follow-up implementations and measured improvements are recorded in
[the correction batch](reference-svg-fixes.md). Findings below describe
the audited baseline; current task status is in the linked correction plan.

## Scope and method

All 24 SVGs under `docs/{entropism,kitsch,neomil,neokitsch}` were freshly
rendered and visually inspected. Each era has four photographed screen
traces, one component sheet derived from those traces, and one original
bar. The four bars have no source photographs; the component sheets have
no independent photographs either. Neither is counted as a photographic
fidelity pass. No reference SVG, application code or golden was edited.

All 16 local source images are 3840×2160. Comparisons used librsvg 2.62.3,
repository fonts via pinned fontconfig, full-screen pairs and detail
crops; targeted measurements used native source pixels. Coordinates here
are 1600×900 design units unless stated otherwise. The original download
identities remain in [sources.md](sources.md). Entropism's dashboard/store
filenames are deliberately swapped; the gate mapping is correct.

The complete existing command was rerun:

```sh
scripts/fidelity_check.sh --inventory
```

All 24 XML documents also parse with no duplicate IDs or unresolved local
`href`/`url(#...)` references. This checks structure, not visual accuracy.
The review includes whole-screen observations and selected native crops;
it does not claim an exhaustive pixel-by-pixel reconstruction of every
micro-mark. Source recipe/origin uncertainty is kept separate from visible
primary geometry and content errors.

## Complete reference inventory

Screen paths below are `docs/<era>/<screen>-trace.svg`; source paths are
under `images/`. Shape percentages measure matched bounding-box area;
ink scores measure coarse color-family occupancy. Neither is a percentage
of visual correctness. All entries below passed their configured gate.

| Era | Screen | Source image | Configured G1i result |
|---|---|---|---|
| Entropism | login | `entropism-login.png` | shapes 3/27, 83% area |
| Entropism | dashboard | `entropism-store.png` | shapes 10/16, 92% area |
| Entropism | mailbox | `entropism-mail.png` | shapes 8/17, 88% area |
| Entropism | store | `entropism-dashboard.png` | shapes 16/38, 79% area |
| Kitsch | login | `kitsch-login.png` | ink IoU 0.63 |
| Kitsch | dashboard | `kitsch-dashboard.png` | ink IoU 0.69 |
| Kitsch | mailbox | `kitsch-mail.png` | ink IoU 0.67 |
| Kitsch | store | `kitsch-store.png` | ink IoU 0.73 |
| Neomil | login | `img-06-private.png` | shapes 15/34, 78% area |
| Neomil | dashboard | `img-07-dashboard.png` | shapes 18/30, 83% area |
| Neomil | mailbox | `img-08-main.png` | shapes 11/31, 84% area |
| Neomil | store | `img-09-store.png` | shapes 12/43, 61% area |
| Neo-kitsch | login | `neokitsch-login.png` | ink IoU 0.77 |
| Neo-kitsch | dashboard | `neokitsch-dashboard.png` | ink IoU 0.64 |
| Neo-kitsch | mailbox | `neokitsch-mail.png` | ink IoU 0.73 |
| Neo-kitsch | store | `neokitsch-store.png` | ink IoU 0.66 |

| Additional SVG | Review basis | Finding |
|---|---|---|
| `entropism/components.svg` | parent traces and source citations | inherited defects; stale halo/implementation claims |
| `entropism/bar.svg` | original design and accepted palette contract | obsolete border/selection colors and citations |
| `kitsch/components.svg` | parent traces and source citations | shared definitions agree; inherit source mismatches |
| `kitsch/bar.svg` | original design and source citations | no identified visual defect; stale citations |
| `neomil/components.svg` | current parent traces and source citations | corrected excerpts coexist with old specimens/prose |
| `neomil/bar.svg` | original design and source citations | no missing source; source-borrowing descriptions need review |
| `neokitsch/components.svg` | current parent traces and source citations | stale upper dashboard specimens |
| `neokitsch/bar.svg` | original design and source citations | original, not a source-backed screen |

## Entropism

**E1 — Login mask count/printing.**
[login-trace.svg](entropism/login-trace.svg) emits eleven stars at line 113;
the source contains ten. Native green-channel runs in x575..675/y423..435
resolve ten primary stars, starting at x577.92 and ending at x663.75,
about 9.58px apart. The SVG has eleven at about 8.75px pitch, with heavier
glyphs. Preserve the source underline beneath the first pair. Component
examples and provenance repeat the wrong count. This concerns the frozen
reference, not the runtime password length.

**E2 — Mailbox text widths and secondary type.**
[mailbox-trace.svg](entropism/mailbox-trace.svg), lines 329–339, specifies
17px text with `textLength`. Removing every `textLength`/`lengthAdjust`
attribute produces a byte-identical native render in the pinned tool:
those attributes do not enforce the intended widths here. Native bright
core extents, excluding frame edges, are:

| Body line | Source x extent | SVG x extent |
|---|---|---|
| 1 | 557.5..1233.3 | 558.3..1149.6 |
| 2 | 557.1..1248.8 | 557.9..1167.1 |
| 7 | 557.1..1255.4 | 557.9..1171.2 |
| 10 | 557.1..1116.2 | 557.9..1046.7 |

Words and 4+4+2 line breaks are broadly supported. Section A/B/C letters,
row/from/button text and microprinting also need primary width/weight
fitting. Do not attribute these large differences solely to source echoes.
The component sheet retains ineffective width attributes too, despite an
old global TODO claiming all had been removed.

**E3 — Store fourth-card crop.**
[store-trace.svg](entropism/store-trace.svg), lines 187–195, assumes the
fourth product continues to the x1600 frame. Source yellow-band green
falls from 197 at x1564 to 68 at x1565, leaving faint displaced residue
through about 1568 and reaching17 by1578. The primary cut is near 1564.6.
The SVG retains extra title, ACC/stat and socket content. Measure a
persistent viewport for the product/caption; keep its cut edge open and
classify the faint exterior residue separately.

**E4 — Store art and typography.** The same trace's `#rifle` is three
rectangles plus a block stock, versus a detailed segmented receiver,
barrel, trigger/handgrip openings and thin angled stock in the source.
The 4ST S is an orthogonal zigzag instead of curved lobes/counters; the
4's crossbar/stem also differs. Store stats, socket and compliance text
remain too thin. Preserve measured object bounds and the first selected
card's inverse ink. Earlier bounding-box corrections did not establish
finished artwork.

**E5 — Grounds and remaining hub printing.** All four traces use a
coarse radial lift. The source has more upper illumination and darker
sides/lower ground. Median native RGB in two unobstructed patches:

| Screen | x650..750/y100..125 source → SVG | x40..60/y400..450 source → SVG |
|---|---|---|
| Login | (27,27,19) → (23,21,12) | (14,9,3) → (21,19,9) |
| Dashboard | (32,32,21) → (23,20,10) | (15,9,1) → (22,19,9) |
| Mailbox | (33,33,23) → (25,23,12) | (15,9,1) → (23,20,9) |
| Store | (32,32,21) → (26,24,13) | (15,9,1) → (23,21,10) |

Fit broad fields before fine texture; establish source equivalence before
sharing a model. The login footer's sampled mean already closely matches
(about 137/173/142 versus 138/172/140). Dashboard's six-tile layout,
BRAINDANCE/T2 state and 7+3 body lines are supported; remaining caption
and body fitting is smaller than its ground discrepancy.

**E6 — Derived references/provenance.** Component copies carry the above
defects. The original bar still uses border `#5d7752` and selection
`#9cb795`, while accepted palette roles in `src/eras/entropism.rs` are
`#8fba97` and `#a6d3a7`. Reconcile this contract, rather than calling the
old values current source measurements. README/component claims about a
universal2px stroke and a photographic dark ring also conflict with later
native-source findings: the mailbox trace itself attributes that ring to
Lanczos resampling. Keep primary edges and source residue distinct.

Preserve USERNAME with masks, STORE ACCESS SCREEN, the tall login footer,
the hub's 3×2 tiles, source ENCRIPTION/Sperad spellings, SMG over HAND GUN
products, and the mailbox's separate filled-row versus open-message state.

## Kitsch

**K1 — Dashboard content/panel.**
[dashboard-trace.svg](kitsch/dashboard-trace.svg), lines 336–369, replaces
eight readable source lines with nine filled11px bars. Source wording:

```text
Ut enim ad minim veniam,
quis nostrud exercitation
ullamco laboris nisi ut aliquip
ex ea commodo consequat.

Duis aute irure dolor in repre-
henderit in voluptate velit
esse cillum dolore eu fugiat
nulla pariatur.
```

The warning ribbon should connect its diagonal flag near 1172,306 to the
right edge near 1432. The SVG instead has a disconnected180×24 rectangle
ending1352. Rounded source body/tab corners are square/angular. Components
repeat these placeholders; this is missing content, not rasterizer drift.

**K2 — Fan label polarity/material.** Five idle labels use dark `#123c38`
where the source uses light mint on teal. Selected EVENTS correctly uses
dark ink on yellow. Preserve two three-blade fans, six-stack geometry and
supported(+20,−20) ghost pitch. Face variation and strokes need measurement;
the old unmeasured-stroke follow-up remains relevant. Correct component
hover rationale that cites dark idle text as source evidence; hover states
themselves remain inferred.

**K3 — Ground fields.** In clear patch (400,20)..(700,80), source→SVG mean
RGB and RMS are:

| Screen | Source | SVG | RGB RMS |
|---|---|---|---|
| Login | 158.5,75.1,93.4 | 133.8,55.5,77.9 | 21.18 |
| Dashboard | 168.9,75.4,95.7 | 141.3,60.6,80.0 | 20.76 |
| Mailbox | 168.9,75.4,95.7 | 133.9,53.1,76.1 | 27.03 |
| Store | 168.9,75.4,95.7 | 138.3,55.1,78.4 | 23.89 |

Dashboard also misses a grey-green left wash: patch (20,300)..(70,580)
averages source 27.2/31.1/26.4 versus 17.7/14.6/14.5. Dashboard/mail/store
share a byte-identical native clear patch (1920,1848)..(2640,1968); another
upper patch differs by at most2 levels. Test a shared field for those
three, but do not assume login is identical. Fine material origin remains
unknown; broad field errors can be measured independently.

**K4 — Login field/control contours.**
[login-trace.svg](kitsch/login-trace.svg), lines 167–182, uses a flat
335×51 field to y464. The source bottom follows ENTER's step, leaving
a dark gap on both sides: nearx550 the field ends around454. The SVG
field touches/overlaps the button there. Source ENTER/PROTECTED corners
and step joins are also rounded. Preserve the full-height bracket.

**K5 — Scoped typography.** Source PROTECTED's mint bounds are about
101×10 at x853..953/y475..484, versus SVG 78×9 at 865..942/y477..485.
The source clock spansx781..852 versus 784..841. ENTER, store metadata,
stats/socket/compliance text and mailbox sender/notice text need local
width/weight/ink fits. Pinned fonts were used; this is not missing font
installation. Mailbox's three body paragraphs and line breaks are already
supported; avoid replacing correct copy or scaling every label globally.

**K6 — Mailbox selected envelope.**
[mailbox-trace.svg](kitsch/mailbox-trace.svg) reuses a closed20×13 symbol
for all rows. Source row 1 has an open pentagonal flap around
x166..185/y322..340. Keep closed symbols for the other four rows. Correct
the old finding that Kitsch had no open-envelope source glyph.

**K7 — Store fourth-card fade.**
[store-trace.svg](kitsch/store-trace.svg) continues through 1600, but source
content disappears around1523..1550. At band rows280..289, source RGB
falls from(254,190,11) at 1520 to(54,40,12) at 1530 and(12,12,11) at 1550;
SVG remains(254,195,47). The source includes colored residue, so a hard
clip alone is not yet an established complete model. This makes the old
vignette follow-up concrete.

**K8 — Store details/corners.** Rifles still lack much of the source's
receiver, fastener, stock and contour detail. The socket mark is reduced
to 14 cells instead of the source scatter. Normal and selected lower
card corners are square where the source is rounded. Preserve the
selected card 2 gun offset(1.5,−13.7), stats86/30/5/5 and Sperad. Earlier
silhouette correction remains valid; it did not finish internal artwork.

**K9 — Derived references/provenance.** Normalized component definitions
for nav/gun/details/band marks/socket/guest/envelope/chevron/fan agree with
their traces; their defects are inherited. Source descriptions still give
old login bracket/barcode bounds and bar USER-box endpoint331 instead of
349.5. Blanket ink/stroke claims are stale. The original bar has no
identified clipping/overlap defect and no photograph to match.

Preserve PRODUCTS twice, selected EVENTS with BRAINDANCE caption, login
GUEST7702 versus hub/mail GUES7702, five mailbox rows, SMG/HAND GUN wording
and selected card 2. Keep the known mailbox yellow extraction diagnostic
separate from actual rounded source geometry.

## Neo-kitsch

**NK-01 — Login wire curls.**
[login-trace.svg](neokitsch/login-trace.svg), line 227 onward, approaches
each left plateau from below (`M35,812 V735 Q35,727…`). The source
descends from above into a rounded bottom-left corner and runs right;
the right endpoint mirrors it. Crop(22,700)..(88,825) shows the reversal
clearly. Preserve 22 strands and the broad low central plateau. An earlier
endpoint-clamping fix addressed a different defect.

**NK-02 — Header-band displacement.**
[dashboard-trace.svg](neokitsch/dashboard-trace.svg), line 374, and
[mailbox-trace.svg](neokitsch/mailbox-trace.svg), line 295, rise fromx52 to 130.
The source rise runs approximately160 to 220. Current SVG crosses the
customer T1 label; source leaves dark space beneath it. Left curls are
missing and the right end uses an angular tail. Remeasure each screen's
terminal strands independently; components repeat the wrong construction.

**NK-03 — T2 badge silhouette/rings.** Source crop (1280,30)..(1350,108)
has a shallow shoulder, angular rise and tightly swept echo fan. SVG has
a deeper rounded rise and conspicuous nearly parallel vertical rails.
The dashboard comment explicitly widens top ring spread to about 5px
versus measured3px for readability. That adaptation is not a source
match. Retain outlined selection and its upward lower tab.

**NK-04 — ARASAKA artwork and annotation fit.** Login source
x92..272/y59..88 contains a custom stencil logo; line 267 substitutes
scaled Rajdhani Bold. Its annotation's COMBAT COLONIZATION also exceeds
the right cell. Reconstruct the mark's source contours and fit the small
copy independently. Width alone did not establish artwork correctness.

**NK-05 — Four-screen typography.** Login ENTER/LOGIN is overly tracked
and narrow; boxed A/B are too small. Dashboard customer/module printing
and badge letters are narrower/lighter or smaller than source. Mailbox
FROM lines are overly tracked/condensed; heading and body weight differ,
although words/line breaks closely agree. Store stat/socket type is too
small/light, and 4ST letterforms remain approximations. The historical
pipeline already acknowledged small DPS and lighter text as deferred;
this is scoped follow-up, not a claim those observations were fixed.

**NK-06 — Dashboard copy substitution.** Lines705–732 explicitly use
selected mailbox text. Source body near(1181,343)..(1390,509) has6+2
lines; trace has5+3 and replaces the source's Quis ipsum/Risus commodo
passages with Ut enim ad minim veniam. Keep this documented application
adaptation separate from a literal source reference; it changes visible
line masses. Earlier cascade geometry fixes remain supported.

**NK-07 — Veneer structure.** Presence of grain was implemented, but
source pattern remains approximate. Store selected body
x667..929/y411..653 flows mostly vertically and bends into a horizontal
seam near 531; SVG line 546 draws horizontal waves across the body with a
conspicuous vertical line near 864. Dashboard detail has a curved lower
convergence near 570..610 absent from its simpler strands. Selected mail
bar has dense smooth waves, central seam and curved ends where SVG has
coarse diagonal zigzags and blank triangles. EMAIL, SMG and basket need
their own source checks. Dominant orientation/seam corrections are
possible from local images; exact original material remains unknown.

**NK-08 — Store weapon placeholders.**
[store-trace.svg](neokitsch/store-trace.svg), line 406 and repeated groups,
uses a rectangle, block paths and a few rules. Source has an articulated
stock, open trigger, angled receiver and mechanical hatching/cells.
Restore supported detail across all four copies and preserve the raised
selected position. Earlier TODOs explicitly deferred coarse silhouettes.

**NK-09 — Store tabs and shoulders.** A representative tab at line 405
is `M459.8,632.9 H524.3 L520.8,641.4 H462.8 Z`: wide top, narrow base.
Source crop (438,624)..(538,647) shows a narrow top and wide base. Product
and nav copies share this reversed taper. Their shoulder joins are also
hard diagonals where source curves into the rise. Preserve shelf bounds,
selection expansion and the independently inverted mailbox tab.

**NK-10 — Closed envelope folds.** Selected row 2 and closed rows4/5/6
show crossing upper/lower folds in source. SVG draws a rectangle and
only `M0,0 L8,6 L16,0`. Restore the lower diagonals. Rows1/3/7 remain open;
selected row 2 remains closed. Components repeat the simplification.

**NK-11 — Ground differences.** Clear original patches support a common
field, while dashboard retains an older haze model. Source means at
(600,20)..(900,60) are approximately96.73/70.32/111.46 on all four;
dashboard SVG is81.01/66.31/99.66(RMS 12.75), other traces
95.25/72.04/111.63(RMS 3.10). At(1000,240)..(1100,270), source means
22.86/39.57/57.49; dashboard is32.16/45.72/71.66 and others
27.02/28.92/43.90, both about 10.6–10.8 RMS from source. Fit on clear
regions/holdouts; a lower login sample crossed its wire and was excluded.

**NK-12 — Stale component specimens/provenance.** Unlike Kitsch's shared
definitions, upper Neo-kitsch examples lag their traces. Main EMAIL is
flat while the lower press example has grain. `ncaption` remains five
bars although dashboard now has text. The main detail sample retains
rectangle paragraphs, flat fill and tape bars. These contradict current
“verbatim-spliced” claims. Old implementation deltas and bar haze/CTA
citations also need refreshing: store no longer uses the dashboard's old
825/1030/.515 haze, and login CTA tracking is4.1 rather than 3. Bar is an
original, with no photographic fidelity verdict.

Preserve sparse two-entry login with no photographed caret, selected EMAIL
without rings, six staircase cards, the corrected stepped reader panel,
seven mailbox rows with selected MOM and repeated Rachel Ross senders,
four RIFLES buttons, source SPERAD and store card2 expansion. Still images
do not establish hover, press, animation or live input behavior.

## Neomil

**NM1 — Dashboard footer primary type and copies.**
[dashboard-trace.svg](neomil/dashboard-trace.svg), lines 1641–1661, uses
Bold 8 for the code and Semibold 7.5 for its captions. In native crop
(1195,862)..(1372,899), primary red(`R > 170`, `R > 2G`) cap heights are
source/SVG 6.25/5.00 for the code and5.83/4.58 for each caption. The first
caption width already matches at 69.17, so simply increasing size would
over-widen it. Fit weight/height/baseline independently, then replace the
single sharp offset rectangle with supported frame/text copies. Preserve
two-cell layout and spelling. Added to [dashboard TODO](../todo/neomil-dashboard.md).

**NM2 — Mailbox invented lower chamfer.**
[mailbox-trace.svg](neomil/mailbox-trace.svg), line 545, ends
`V691 L1442,699 H729 Z`, cutting8px from the lower-right corner. Native
source crop (1400,680)..(1460,705) shows a square primary corner with square
offset printing. Retain the real upper-right chamfer and side bar while
remeasuring the lower contour. This is additional primary geometry,
separate from fine echoes. Added to [mailbox TODO](../todo/neomil-mailbox.md).

**NM3 — Store socket occupancy and selected offset.**
[store-trace.svg](neomil/store-trace.svg), lines 200–213, defines24 squares.
Native thresholds(`R > 200`, `G < 90`, component area≥12 native pixels)
resolve25 in both normal and selected source examples. Missing lattice
cell: column 2,row 6, local(10.5,24.5), near source(459.79,543.96) on card1.
The selected source's first cell center is(782.71,712.71); current SVG
placement produces(786.5,714.5), about 3.8px right/1.8px down. Both source
rows have the same primary orientation; horizontal versus vertical echoes
create the apparent directional difference. Do not rotate the pattern or
invent a QR payload. Added to [store TODO](../todo/neomil-store.md).

Reconfirmed existing open work:

- [Login](../todo/neomil-login.md): filled avatar/wedge versus hollow
  substitute, real portraits versus silhouettes, mask/card/header type,
  inactive-card/badge fields and fine printing.
- [Store](../todo/neomil-store.md): opaque brown fills erase blue/cyan,
  selected/nav fields differ, typography/footer fit and weapon/brand art
  remain schematic.
- [Dashboard](../todo/neomil-dashboard.md): maker microtext copies,
  remaining chip/tape ink and exact common texture remain open.
- [Mailbox](../todo/neomil-mailbox.md): cartridge terminal detail, fine
  printing and exact common texture remain open.

Preserve corrected shared ground, six dashboard tile/glyph groups and
nine GO HOME lines, mailbox rows/body/cartridge primaries, login extent
and separate hollow slot, store lower chamfer andx1557 cut. The source
continues to support SPERAD, the tall second card and solid gun offset.

**NM4 — Components and provenance.** Corrected dashboard/mailbox excerpts
coexist with older login/store specimens. Component prose still refers
to deleted `Layout::OpsCharts` and different screen grounds, contradicting
the all-four shared-ground section. README/source rows retain old gate
numbers and store y800 despite the corrected 797.1 corner. Refresh current
claims while retaining dated history. The bar explicitly declares an
original design; no missing source should be invented for it.

## Validation boundary

All 16 configured G1i checks and all 24 structural SVG checks pass. No
thresholds or `photo` classifications were changed. The earlier249 Rust
tests and27 visual cases concern the staged implementation batch; they
were not rerun for this documentation-only audit and do not establish
source fidelity. Fresh source-region comparisons are required when each
recorded correction is implemented.
