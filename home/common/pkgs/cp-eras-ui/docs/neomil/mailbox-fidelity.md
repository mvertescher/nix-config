# Mailbox source correction

The design target is source #61, `images/img-08-main.png` at 3840×2160.
Its 1600×900 coordinates divide native coordinates by 2.4. This screen is
`cp-eras-ui-mailbox` / `screens::mail::MailBox`, including the dashboard
hub route; the separate `cp-eras-ui-mail` example is a different client.

The 2026-09-21 audit compared the original, `mailbox-trace.svg` and a
fresh Iced capture. Shape gates passed despite visibly undersized body
copy, simplified cartridges, missing rotated printing and incorrect
material. Frozen screenshots also concealed the standalone launcher's
missing animation subscription. A passing inventory or golden alone
therefore does not establish correctness.

## Typography and primary printing

| Region | Source-supported setting in design pixels |
| --- | --- |
| Body | Rajdhani Regular 17.5; x750, first baseline 347.5; 21 line pitch; one extra line between paragraphs |
| List subjects/senders | Rajdhani Regular 17.5; baseline 339.4167, then 409.4167 +70 per row |
| Message heading | Rajdhani Bold 20; original x742/baseline287 |
| Action labels | Rajdhani Regular 16.6667; left inset 12.5, baseline 746.6667 |
| Header captions/tabs | Rajdhani Medium 12.9167; native-source positions |
| Margin codes | Rajdhani Regular 11.25, no added tracking, rotated −90° |
| Maker labels | Clockwise 90°; BETTERLIFE TEC and framed PETROCHEM in the panel corner |

At native scale the first body line's core ink starts around x1804
(design 751.67), while its advance at font 42 is 1531.64 native pixels
(design 638.18). Its font origin is x1800/design 750; the visible sidebearing
accounts for the remaining offset. The previous 15px SVG advance was
about 545px. All ten original line breaks and words are retained. The
source's bright stroke cores in body, normal list, heading and outlined
button text are RGB(251,53,53), sampled independently in each region.

The LEVEL lettering and custom T1–T4 shapes match the dashboard's source
forms. Their paths are reused with mailbox placements; the header's
protocol block, notices and tape are measured separately. The tape's
leading miniature inscription is not legible. It is recorded as observed
small ink modules rather than assigned a guessed string.

Static text is outlined from the bundled Rajdhani fonts, so SVG and Rust
use the same geometry without depending on text rotation inside the
software compositor. `neomil_mailbox_art.rs` exports:

- `HEADER`: primary header/badge outlines, protocol and tabs; badge
  interiors come from the mailbox material table.
- `MARGINS` and `FOOTER`: margin chips/codes and footer arrow/printing.
- `PANEL_PRINTING`: both maker labels and the PETROCHEM frame, painted
  inside the message opening motion.
- `NEW_PILL`: source geometric NEW lettering and its complete rounded
  outline, recolored with the row printing state.
- `ICON_NORMAL`, `ICON_SELECTED` and `ICON_POSITIONS`: complete cartridge
  paths and measured placements.

## Cartridges

The old four-corner casing and solid ellipse did not match source #61.
Native crops show a stepped right edge, a narrow contact strip, an inset
lower tile, a C-shaped disc with a lower cutout and two eccentric inner
rings. The new SVG and Rust paths preserve all of these features.

The normal cartridge template is traced from the second source icon,
whose native reference origin is (452,948). Correlation of its bright disc
against the six following source crops gives these design origins:

| Row | Normal anchor x | Anchor y |
| --- | ---: | ---: |
| 1 | 188.3333 | 326.6667 |
| 2 | 188.3333 | 395.0000 |
| 3 | 188.3333 | 463.3333 |
| 4 | 188.3333 | 531.2500 |
| 5 | 188.3333 | 599.5833 |
| 6 | 188.3333 | 667.9167 |
| 7 | 188.3333 | 741.6667 |
| 8 | 188.3333 | 809.5833 |

The selected variant carries a −15.8333px horizontal displacement, giving
row 1 its source top position x172.5. Keeping that displacement in the
selected variant makes selection of another row use the same visual
state; the source only establishes the initial row 1 selection. The list
opening clip starts at x115 and is 410px wide so the corrected left tip
at about x122 is fully visible. It retains the original 440ms timing.

The normal terminal's ribs follow a separate source measurement. Row 2
gives a 3.316-native-pixel rib pitch; six independent rows give 3.311–3.320.
The old 22-rib template had a 2.999-pixel pitch, so W used 20 full ribs.
AM later adds two partial end strokes at the corrected pitch; it does not
restore the old evenly spaced template. W's first correction improved its native changed-footprint
RGB error from 26.73 to 22.64, and full-strip error on held-out rows 3, 5,
and 7 from 24.65/27.96/29.54 to 21.76/25.72/24.06. It retained a visible
angle residual: its native phase slope was about −0.612, while source rows
2, 3, 5, and 7 were −0.86 to −0.87 at the same 3.316-pixel cadence.

Those phase slopes are measured at increasing depth relative to the
terminal's sloping top border, not as a path endpoint `dx/dy`. With top
slope 0.33846, measured phase slope `m` relates to physical line slope
`s` by `m=s/(1−0.33846s)`. A row-2 source fit and three independent row
holdouts give `m≈−0.85`, or inferred `s≈−1.20`; W's endpoint slope was
about −0.815. The revised scratch geometry moves the 20 starts +0.70
native pixel along the top for phase registration and sets each endpoint
−2.48 design x and +2.0773 design y from its start, on the same lower
border. Native-size SVG full-strip RGB MAE moves 21.48→20.38 on row 2
and 19.86→17.76, 23.27→21.70, 21.79→20.01 on held-out rows 3, 5,
and 7. Their gap-only errors worsen by 0.33–0.56, so softness/ink remains
an independent limit. This revised angle awaits the next native trial.
The terminal rule, stroke ink `#9a2326`, 0.4-design width, shell, disc,
and selected cartridge are unchanged. No backing or secondary material
was added.

The row-2 rib signal has source red amplitude 8.36; opaque W SVG ink
yields 12.35, so contrast remains too strong. The revised geometry is a
direction/count/phase correction, not exact terminal ink; its new native
render and row feedback still need review.

## Review and remaining limits

Inspect source→SVG→Iced with `scripts/triptych.sh --diff neomil mailbox`,
and run both inventory and implementation modes of
`scripts/fidelity_check.sh`. Use the freshly built binary directory via
`--bin-dir`. Also inspect a real-clock headless launch: fixed rest-frame
renders cannot prove an animation subscription is wired.

These corrections target primary geometry, typography, missing printing,
row fields and mailbox-specific material. Native source images contain
scan variation, glow and offset printing echoes. The primary vector
paths do not completely reconstruct those residual effects around every
letter and cartridge. Cartridge curves are compact geometric traces,
not an assertion of pixel-perfect correspondence. Their terminal strip
is schematic at subpixel scale. Such residuals should remain visible in
source comparison rather than being hidden by changing gate thresholds
or tagging primary artwork as photographic residue.

The body remains fixed source lorem; action/scroll artwork remains
primarily a visual design target. Row selection must show the selected
subject after leaving the explicit initial fixture. These are separate
requirements from a working mail backend.

## Row follow-up from the combined capture

Native review corrected two more inherited assumptions. The bright primary
selected plate begins at native y757 (315.4167); the higher y313 mark is
an offset printing echo. Primary row geometry is x241.25,y315.4167,
269.5833×67.5 on a70px pitch, with a15px lower-left cut. Normal outlines
are approximately RGB(102,14,15),0.8333px wide. No solid3px spine appears
at x237–240; rendering one turned faint source residue into bright artwork.
The selected plate's clear interior is approximately RGB(227,49,49).
Selected text's flat representative is RGB(83,23,25); native ink varies
under the source's scan/echo treatment.

Direct correlation of the bundled Regular42px glyphs against native list
crops confirms17.5design type and baselines near339.58,409.17 and479.17.
The shared layout uses row origin+24.0, retaining at most0.42px of source
placement variation. The earlier337.5/407.5 baseline transcription was
too high and is superseded by those native fits.

The source NEW letters occupy native(1028,1038)–(1096,1052):
28.3333×5.8333 design pixels. They are wide geometric forms, not Rajdhani
Bold. The pill centreline frame is75.4167×10.8333, with a1px stroke and
3px corner turns. Its normal origin is relative(162.5,44.375) and its
selected origin is relative(162.5,46.875), both measured against the
logical row. Keeping the selected offset explicit preserves the initial
source placement without shifting every normal pill down.

### X native angle confirmation

The corrected angle changes 1,141 native pixels, all confined to the seven
normal terminal strips. Changed-pixel RGB MAE improves 24.264→20.396.
Full-strip errors improve on row 2 (22.889→21.105) and held-out rows 3, 5,
and 7 (21.758→19.633, 25.716→23.494, 24.060→22.759). Source cadence
is 3.311–3.316 native pixels and X measures 3.313–3.316. Its apparent
sloped-coordinate phase slope improves from about −.612 to −.800, versus
the source's −.861 to −.870; finite stroke coverage remains approximate.
The fitted row's gap error increases slightly (22.767→23.031), while
all three held-out gaps improve. This is an explicit local tradeoff,
not a claim that faint printing is exact. Fractional rest, last-row
selection and ordinary held feedback are reviewed; selected art and all
other screen pixels remain unchanged. Both affected X fidelity gates pass.

### AA uniform-contrast trial rejected

A single opacity fitted to the normal row-2 ribs (.721) does not transfer
to held-out rows 3/5/7, which independently imply .943/.824/1.025. The
fit row's central RGB MAE improves only 15.354→15.299; two held-out rows
worsen, and row 3's rib-footprint error rises 11.871→13.636. Dark gaps
do not improve. Source, SVG and native crop review shows residual local
softness that uniform opacity cannot reproduce. The 20-rib geometry and
current ink remain unchanged; selected art and shell controls are identical.
This rejected trial does not close the fine printing/material task.

A follow-up rib-only Gaussian diagnostic also fails to identify a shared
material correction. A .4-native-pixel spread improves SVG whole-patch RGB
error in all four rows, but row 3's positive-ridge red error worsens
21.41→22.20. Native positive cores and adjacent gaps are both already
brighter than source, so spreading that native ink can worsen the gaps.
The source's ridge-to-gap modulation varies substantially by row. This
image-space probe preserves geometry but cannot separate local background,
registration and adjacent edge printing well enough to justify an actual
renderer change. No blur or anisotropic fit is adopted.

### Ordinary-terminal width holdout

The next native source review tested only the 20 ordinary rib strokes' width.
The source, current 3840×2160 SVG, and X Iced capture were compared at the
normal row origins (452,948), (452,1112), (452,1439), and (452,1780). Each
full-strip sample is 73×30 native pixels. Color `#9a2326`, the terminal rule,
all rib coordinates, casing, selected 22-rib icon, and surrounding material
were fixed. The SVG width was changed from 0.40 design pixels in scratch
renders; no change was ported to the trace or renderer.

| Ordinary row | 0.30 width | 0.35 width | Current 0.40 | 0.45 width | 0.50 width |
| --- | ---: | ---: | ---: | ---: | ---: |
| 2, fitted | 14.732 | 14.695 | 14.654 | 14.637 | 14.617 |
| 3, holdout | 14.510 | 14.422 | 14.382 | 14.375 | 14.403 |
| 5, holdout | 15.932 | 15.929 | 15.949 | 15.981 | 16.016 |
| 7, holdout | 15.221 | 15.184 | 15.165 | 15.215 | 15.284 |

Values are native-pixel, full-strip RGB MAE against source. Narrowing is
not a consistent improvement; widening for the fitted row reverses the
direction in rows 5 and 7. Width alone cannot explain the apparent fine
source lines. To isolate the residual, samples at the center 20–80% of each
rib were classified by distance from its fitted centerline: core <0.35,
edge 0.35–0.85, and adjacent gap 1.4–2.1 native pixels. The native X Iced
red means for rows 2/3/5/7 are 143/143/143/143 in the core versus source
113/123/106/113; 112/111/111/110 at the edge versus source 100/89/84/80;
and 74/73/71/69 in the gaps versus source 65/59/48/34. Width changes leave
those gap samples unchanged in the SVG. The gap sample is small (21 pixels
per row) and does not establish an echo contour; it is a separate control
showing why a width fit cannot solve the observed source variation. No new
secondary art or opacity was adopted from this diagnostic.

### AL: inner terminal material controls

A fresh 4K capture from the verified AK package confirms that the clear
ground above ordinary terminals already matches the source closely: RGB
MAE is 1.10–1.27 across rows 2/3/5/7. A common background adjustment is
unsupported. At fixed registration, native rib-core red means are
140.4/140.4/140.3/139.8 against source 118.5/127.4/112.8/115.9. Gap means
are 59.3/57.7/55.4/52.8 against 68.8/60.2/51.2/36.0: the required gap
correction reverses direction between the upper and lower rows.

A scratch SVG adds a bounded .12-opacity inner fill and lowers rib opacity
to .80. Full-strip RGB MAE improves in all four inspected rows, from
13.671/13.445/14.802/14.213 to 13.366/13.225/14.526/14.007. Separate
controls reject it: row 3 positive-core error rises 12.70→13.04, and row 7
gap red rises 46.9→59.2 away from source 36.0. The latter's pixelwise RGB
error nevertheless improves, illustrating why one average is insufficient.
The small masks sample the central 22–78% of ribs 2–17: 58 core pixels,
75 edge pixels and 37 gap pixels per row. These masks differ from AF's;
their values are not evidence of a runtime change since AF.

All 3,626 changed SVG pixels stay within the seven ordinary terminal boxes.
Selected art, disc cores and lower-shell controls are identical. The fill
is a diagnostic hypothesis, not a source-traced polygon; no native
candidate was built and no trace or Rust correction is accepted. A retry
needs source-supported terminal boundaries/material and separate core,
gap, shell and selected controls. Scratch evidence and reproducible scripts:
`/tmp/cp-eras-next/al-mail-terminals/`; Astra's independent containment and
full-strip check: `/tmp/cp-eras-next/al/mail-root-review.json`.

### AL: terminal depth and inner boundary

The follow-up geometry audit finds a source-supported depth discrepancy.
At fixed ordinary-icon origins, sample along the upper edge with slope
.33846 and let `q` be the vertical offset below that line, in native pixels.
The source has a coherent lower ridge near q10.25–11 across rows 2/3/5/7,
including separate left/right strip halves. SVG and native instead have a
strong inner edge near q5; the standalone lower rule lies near q7.37.
Rib modulation in the source persists farther down the strip. Exact rib
endpoints remain softened, but the repeated lower boundary is distinct
from a single photographed echo. See `depth.py` in the AL scratch record.

The q5 edge is the normal cartridge's second path, through
`(-0.4167,2.0833)` and `(27.9167,11.6667)` in design coordinates.
Extending the ribs from vector `(-2.48,2.0773)` to `(-3.72,3.11595)` and
moving only the standalone rule improves four strip averages but leaves
that inner edge intact, producing a doubled ladder. A second scratch
trial moves the inner edge down and removes the redundant rule. Its
center has the deeper source silhouette, but the right endpoint crosses
the outer notch, which the source does not do. The new gaps also become
too dark: row 2 mean red changes 54.2→26.3 against source 68.8. Lower
rib modulation becomes too strong, and portions of the lower-shell control
change. Both geometry trials are rejected; no native candidate is built.

This is now a concrete geometry/material follow-up, separate from the
rejected width and opacity fits. Reconstruct the deeper strip together
with its inner-edge joins and gap material. Preserve or independently
verify the upper pitch/phase, outer casing, disc and selected icon; inspect
both end connections and the full icon, not just the central strip. The
current rightmost rib coverage is also a residual requiring source review,
not a reason to change the count without evidence. Evidence includes
`depth-q10.5.svg`, `depth-coherent.svg` and the left/right four-way crop
comparisons in `/tmp/cp-eras-next/al-mail-terminals/`.

### AM: deeper terminals and bounded interior material

AM corrects the ordinary cartridge's inner terminal boundary rather than
changing rib width or dimming all strokes. The lower rim moves from the
unsupported q≈5 edge to q≈10.25, against source q10.25–11. The redundant
q≈7 rule is removed. The rim joins the left casing at design (−4,3.021)
and ends at (26.5,13.3449), before the right notch, without a crossing.
The twenty established rib starts/pitch stay fixed; endpoints extend to
q≈8.5, leaving the source-supported fade before the lower rim. Two shorter
end strokes cover faint marks within the curved cap. Their exact ink remains
approximate; they are not evidence of recovered authoring geometry.

An independent blank interior patch shows missing case material: at
x486..495, q18..23, source red is 49–55 versus the old 14–23. Two closed
fills follow the corrected case and terminal boundaries, beneath the existing
strokes/disc. The SVG uses #9a2326 at .20 and .15 opacity; ribs retain
full ink and .4 width. The stronger .30 terminal fill is rejected for
brightening lower-row gaps; a depth fade adds complexity without improving
the endpoint controls. A single shared cartridge is retained for every row.

Iced blends these translucent vector fills in linear color space. Using
SVG alpha directly would therefore be incorrect. A row-2 ground sample
(23.4,11.6,14.4) predicts case #85282c/.10 and bed #702226/.10; a fresh native
capture confirms the case patch at (49.0,16.3,19.0), close to the SVG target
(49.5,16.3,19.1). No shared renderer, palette role or background changes.
The other six rows validate the same parameters without per-row adjustments.

Source-defined bright/dark masks sample x464..511 at .5 native-pixel steps
and q2..7 at .5 steps. Each q uses the source's upper/lower red quartiles;
registration and masks are fixed before evaluating the candidate. Native
RGB MAE against the original improves for both masks in every ordinary row:

| Row | Dark gaps, before→after | Bright ribs, before→after |
| --- | ---: | ---: |
| 2 | 21.190→7.330 | 10.227→8.719 |
| 3 | 22.096→6.310 | 10.135→7.053 |
| 4 | 24.465→11.232 | 14.010→12.829 |
| 5 | 24.744→8.099 | 12.561→9.383 |
| 6 | 26.152→6.135 | 12.549→8.031 |
| 7 | 28.732→7.107 | 12.736→8.402 |
| 8 | 30.184→11.646 | 14.972→11.080 |

Whole-icon, terminal and both endpoint crops also improve in all seven
native rows. The SVG dark masks improve in all seven; bright masks improve
in six, with row 4 retaining a small loss, 14.160→14.559. Full-period
profiles still expose excess upper-row modulation and missing photographed
softness. This closes neither exact contrast/phase nor fine printing.

There are 21,920 changed SVG pixels and 19,362 changed native 4K pixels,
all inside ordinary icons. The selected icon, surrounding ground, and opaque
disc/lower-shell cores are identical. Case fill intentionally changes its
bounded interior and antialiased fringes. Source/SVG/native endpoint and
whole-icon crops support the correction independently of average error.

The first scratch geometry file was mistakenly revised during compilation.
That capture is excluded from acceptance. Root-owned immutable SVG/Rust
copies were compiled again; all reported native evidence uses those copies.
The final last-rib revision is an inside-cap correction, not an untouched
blind holdout. Evidence and reproducible measurements are under
`/tmp/cp-eras-next/am/`, `am-mail-geometry/` and `am-mail-material/`.
State/gate/full-check results are recorded in
[round twenty-four](../reference-svg-round24.md).

### AZ body-text registration and faint ink

A fixed-phrase source/SVG audit does not isolate a separate body-text echo
contour. At 4K, an image-space comparison one native pixel higher improves
primary overlap for “Lorem ipsum” and the independent “ut labore et”,
“Excepteur sint” and “Nemo enim” controls. Training union-mask red MAE
falls 83.16→42.97 and F1 rises .743→.886; the three held-out errors fall
93.13→47.28, 86.61→42.51 and 62.70→23.74. Bright masks at four thresholds
support a primary-registration difference. Astra independently reproduces
the four fixed comparisons and reviews the source/SVG crops.

Core, counter, near/far-gap and blank controls find narrow residual softness
around the primary strokes, without a consistent separate displaced shape.
Horizontal image-phase preferences differ between phrases. At 1600px, an
independent one-pixel upward image shift helps some lines but worsens the
last control, 39.49→46.55. This is a different design displacement from
one pixel at 4K; it does not reject a shared design-space baseline change.
The diagnostic does not justify adding another text copy or changing ink.

No mailbox artwork changes in AZ. A separate SVG trial can translate only
the ten body text nodes by −1/2.4 design pixel; native acceptance would
still require same-size source/Iced controls. Mailbox responsive sx/sy
placement differs from uniform scene scaling, so image offsets alone do
not establish fractional native geometry. Exact printing remains open.
Frozen controls, source hashes and reproducible measurements are in
`/tmp/cp-eras-next/az-mailbox-printing/`; Astra's fixed comparison is
`/tmp/cp-eras-next/az/mailbox-root-review.json`.

### BA body baseline: source and native review

AZ's phrase audit found a primary vertical registration difference before any
stable secondary printing contour. BA tests one shared design-space
displacement: translate only the group of ten mailbox body lines by `−1/2.4`
design pixel. The source/SVG controls were frozen before that single
candidate; no x, font, text, spacing, ink, panel geometry, echo or shared
renderer changes. At 4K and 1600, all ten SVG whole-line red errors and F1 at
thresholds 100/150/200/230 improve. The SVG still has local losses at 1600:
the training phrase's union-mask blue MAE rises 1.320→1.536, “Nemo enim”
F1@200 falls .34646→.33333, and the five changed line-1/line-2 gap pixels
raise blue MAE .723→.728 while red and green improve. Those controls prevent
calling the SVG a complete text fit.

The guarded actual Mailbox native preview changes 67,488 pixels at 3840×2160,
all inside the body, with unchanged alpha. Against the source photo, the exact
full-body RGB MAE falls 8.742→7.014; fixed-union red MAE falls 109.705→79.502.
The body-core F1 rises .587→.720 at threshold 100 and .513→.637 at 150. All
four fixed phrases and all ten whole lines improve red MAE and F1 at
100/150/200/230. The full body nevertheless has 49,880 RGB-L1-gaining and
17,506 losing pixels, so the improvement is regional rather than pixelwise
uniform. The **frozen baseline-defined counter mask loses in six controls**:
“ut labore et” and lines 2, 3, 6, 7 and 10. It follows the old glyph holes,
whose masks have only about .65–.66 IoU with corresponding source-defined
holes in those six controls. A **post-capture diagnostic**, explicitly not a
frozen acceptance holdout, defines holes from the source and finds red-MAE
gains in all 28 region/threshold comparisons (four phrases plus ten lines, at
150 and 200). Individual source-hole pixels still lose, especially around line
7. Both counter results remain in the record; neither justifies claiming exact
glyph contours or adding a copy.

At 1600×900 the native baseline and candidate are byte-identical. The
design-space displacement is subpixel there, and this raster phase produces no
image change; the 1600 SVG improvement must not be described as a native one.
At 1537×947, Mailbox places text anchors with `sx=width/1600`, `sy=height/900`
and glyphs with `min(sx,sy)`, so a uniformly scaled photo is not a valid
source-fit reference. Rest, first/last selection, held-first/held-last and
five-role custom captures each change 9,194 body pixels without alpha or
outside-body changes. At 0.12s opening and time zero, the body remains hidden
and the pairs are identical. A supplemental 0.20s capture reveals part of the
body: 4,018 pixels change within the clip, with no alpha or outside-clip
changes. These are headless drawing-state results, not live desktop or pointer
verification.

The component sheet companion changes only the existing body-text group's
transform. Its two visible specimens change 9,644 and 3,892 RGBA pixels at
full/half sheet size, entirely within the mailbox excerpts; all other content
and alpha remain identical. The proposed correction is bounded to body
baseline registration. Ordinary-terminal phase/softness, faint text printing,
exact glyph counters, the original shared texture recipe and live desktop
behavior remain open. Evidence: `/tmp/cp-eras-next/ba-mailbox-baseline/`,
`ba-mailbox-native/`, `ba-mailbox-review/`, `ba-mailbox-components/`, and
`ba/mailbox-opening200-review.json`.
