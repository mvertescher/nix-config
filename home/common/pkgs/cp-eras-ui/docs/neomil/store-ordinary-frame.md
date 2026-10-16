# Ordinary Store frame correction — plan and verified geometry

Source: `images/img-09-store.png`, 3840×2160. FF captures the current packaged
Neomil Store at 4K and renders its current SVG at 4K, 1600×900 and
1537×947. Source samples use the uniform top-left sampler and retain
metadata. All 350 pre-plan tracked crate files match the guarded runtime
checkpoint; original files and font bytes remain unchanged.

Inspection of the missing faint contours exposes a primary-geometry error
that must be corrected first. Current ordinary top strokes overlap a dim
source copy rather than the strongest source line. Seventy fixed transverse
profiles and forty-five junction rows independently confirm the finding.
Astra verifies their input hashes, raw RGB/chroma samples and primary
locations. Positions below are physical pixel **indices**, not inferred
vector centerlines; pixels have centers at index + 0.5.

| Fixed straight-region control | Source strongest | Before correction: SVG / native |
| --- | ---: | ---: |
| Ordinary cards 1, 3, 4 top | y370 | y361 / y361 |
| Selected card 2 top | y370 | y370 / y370 |
| Ordinary cards 1, 3, 4 bottom | y1468 | y1471 / y1471 |
| Card 1 left | x1055 | x1048 / x1048 |
| Card 3 left | x2632 | x2629 / x2629 |
| Card 4 left | x3420 | x3419 / x3419 |
| Selected left | x1845 | x1845 / x1845 |
| Card 1 upper right | x1727 | x1730 / x1730 |
| Card 3 upper right | x3304 | x3307 / x3312 |

The top and bottom mismatches repeat at all ten ordinary sample points;
selected top/bottom controls already align. Averaged wide-strip profiles
independently confirm those displacements. Their plateau argmax can differ
by one pixel from single-column profiles; neither is a vector centerline.
Card 3 additionally differs between SVG and Rust: its SVG upper right is
x1378 design units, whereas the native common 284-unit contour reaches
x1380. The frame is a separate target from the calibrated glyph origins,
rifle, certification marks and other contents. Moving the entire card
would disturb those accepted placements.

FG's initial `right_lower` transverse samples at design y332/348/364 lie
inside the filled fan bar. They remain useful material diagnostics but do
not establish the thin lower-stem axis. Additional after-bar profiles and
upper/lower fan joins are required before a complete path is proposed.
Faint parallel source peaks at top y354/y362 also remain separate: a fit
made before relocating the primary would misattribute its current error.

## Implementation and acceptance plan

1. Derive continuous ordinary frame and side-bar geometry from repeated
   primary straight runs and corner intersections, with independent
   threshold controls. Quantify the pixel-center/coverage uncertainty;
   do not use profile argmax as a vector vertex.
2. Preserve selected geometry and all card contents. Update only ordinary
   contour/fill boundaries and the matching bar where its measured joins
   require it. Keep the fourth frame open at the then-current x1557 cut and maintain its
   permanent viewport. Preserve fitted interior ramp coordinates.
3. Render a frozen SVG candidate at all three sizes, using complete edges,
   corners, bar joins, nearby blank bands and content/selected controls.
   Review local losses as well as whole-region RGB. Synchronize the
   component examples after acceptance.
4. Port through the existing drawing primitives and material cache.
   Preserve array-slot assumptions, semantic custom inks, all selections,
   hover/held feedback and opening clips. Check exact baseline/refactor
   replay before source comparison, including the card-3 SVG/native
   discrepancy. Fonts and text caches are outside this correction.
5. Review source→SVG→Iced at three sizes and relevant states; run Store
   gates, Rust tests and full repository checks. Update only reviewed
   intentional goldens, then verify production/package parity. Faint
   echoes remain a later source-supported material task.

The measurements above describe the pre-correction baseline. FH derives
separate ordinary paths and FI/FL verify the geometry at all three sizes;
FP integrates it after sixteen native state pairs, both Store gates, 305
packaged tests and all 22 repository checks/28 matrix cases pass. Fresh
packaged pixels match at all three sizes. The source SVG, component and
native material boundaries are synchronized. See
[round sixty-nine](../reference-svg-round69.md) for source scores and the
explicit fourth-left 1600 L1 tradeoff.

Geometry is complete. Width 1.2 was held fixed during that position
correction. FM's repeated source profiles then support a bounded thinning
study, while physical width/blur and ground separation remain uncertain.

[Round seventy](../reference-svg-round70.md) completes a bounded first/third
top correction: one continuous outline uses a 0.6 top and 1.2 other sides,
with the same inferred path on an ordinary second card. Direct splitting
loses corner joins; short preserved stubs create unsupported width steps.
The continuous offset-line construction avoids both. An original-width
native replay is exact across resting, state and custom-palette controls.
The thinner top improves all fixed top/corner source comparisons at three
sizes; small SVG stroke-to-fill losses elsewhere remain explicit. Source,
component, native, package parity and full repository checks pass.

[Round seventy-one](../reference-svg-round71.md) completes the fourth top
with a continuous U-shaped ribbon and exact butt caps at x1557. Its original
width reproduces native resting, state and custom captures exactly; the
0.6 top improves all fixed source regions at three sizes. The coverage test
checks painted edges and the empty middle/exterior cut. The distinct bright
held border, Rect face and viewport remain intact. Both Store gates,
305 packaged tests, full checks and three-size package parity pass.

[Round seventy-two](../reference-svg-round72.md) completes the ordinary
left-edge correction, including inferred ordinary-second geometry. A fixed
0.6 left width preserves top 0.6 and all other widths. Every fixed source
region improves or stays exact in SVG/native at three sizes; seven paired
states and three custom cases preserve selected art, held suppression and
the bright fourth held border. Native/component paths agree. The component
also loses one ignored duplicate raw attribute line, with pixel-exact cleanup
at two sizes. Both gates, 305 packaged tests, full checks and fresh three-size
package parity pass.

At that checkpoint, primary coverage remained open for bottoms and lower
stems. GO's combined
0.6 left/bottom trial is rejected because all three bottoms regress at both
smaller sizes. GV's complete lower-vertical-segment trial is also rejected:
smaller-size fan regions lose, and fractional first-card lower-stem RMS
worsens. Keeping the bar polygon fixed does not preserve the frame's visible
contribution beside it. A later separation needs a supported bar/stem junction;
do not introduce an arbitrary cutoff, cap, taper or width/opacity sweep.
GX confirms a repeatable visible narrowing at the bar tip using fixed
source cross-sections, but overlap and unequal flanks leave the separate
width/cap/taper law unidentified. Source profiles do not identify unique
physical widths or blur. Faint echoes remain separate from these primary-coverage tasks.
The [echo study](store-frame-echo.md) now supports two fixed affine copies
as a geometric hypothesis. Independent first/fourth-card ground controls
support darker effective top ink. A fixed, previously measured centroid
improves the top waveform with small local losses retained. Independent
top/bottom source controls also locate the fourth primary cut about three
physical pixels beyond the earlier SVG/native endpoint.
[Round seventy-three](../reference-svg-round73.md) completes the coordinated
design-x1558.3 face, content, cap, reference-material and drawing/hit viewport
correction. Whole-card source comparisons improve at all three sizes; the
native fractional bottom endpoint retains a bounded L1 +1.50% / RMS +1.25%
loss. Seven state pairs, three custom pairs and a disabled-reference replay
pass, along with both Store gates, 305 packaged tests, full repository
checks and three-size package parity. The correction adds no primitive or
effect passes and does not change fonts or text caches. The early complete
echo trials at that checkpoint remained unaccepted for smaller-size losses.

[Round seventy-four](../reference-svg-round74.md) completes the later bounded
frame impressions and coupled lower-stem/bar correction. The bar's supporting
lines extend to x709.6 on the first card, preserving a continuous 0.6 lower
stem without the isolated trial's fan gap. Third-card geometry translates
+657.1; inferred ordinary second-card geometry uses +332. The face stays
fixed. Four tiny native junction-row metric losses remain explicit inside
improved enclosing regions. Both Store gates, 307 packaged tests, full checks,
state/custom controls and actual-package parity pass. Primary bottom coverage
remains open; the failed GO bottom trial is not evidence for a new width change.

## Bottom profile follow-up

IM tests six fixed C1/C3 exterior patches below the bottom against the
same-coordinate original Dashboard. None match: its other artwork occupies
those positions. An independent Login layout prediction selects two clear
rightmost positions before measuring; IN verifies 153/153 exact RGB pixels
at each, while all four covered controls differ. This validates those local
exterior patches, not a ground layer beneath the Store face.

IO freezes nine-column 4K strips at x1596–1604 and x3173–3181, y1452–1505.
All 972 Store/Login/SVG/package samples and their band/row metrics are reviewed.
The central primary row y1468 matches exactly. Source mean red on adjacent
rows y1467/y1469 is 138.89/106.89 for C1 and 135.22/102.67 for C3, versus
251/251 in the native output. SVG partly softens the upper row but retains
substantial excess. Echo residuals change sign; source and Login only resume
exact equality from y1481 down. This is a repeated primary-shoulder residual,
not a uniquely identified authored width, blur or opacity.

IP re-evaluates GO's already frozen 0.6 bottom under the accepted echoes
and lower-stem/bar geometry, with no new width or phase sweep. All three
baseline replays are byte-exact and all nine whole-bottom comparisons improve.
Nevertheless, the upper shoulder loses in both C1/C3 at 1600 (RGB L1 +86/+88,
RMS +7.94/+7.77 over four pixels each) and at 4K (+66/+90 L1 over nine pixels
each). The 1600 fourth cut-exterior also loses 40 L1 across four pixels;
fractional checks have no losses. All 71 declared regions, including empty
subpixel strips, remain in the result; review reproduces 410 nonempty
baseline/candidate metric sets. Selected art, contents, alpha and pixels
outside bottom support remain exact.

The literal bottom replay is rejected for the repeated two-card shoulder
loss, despite enclosing gains. The result's empty 1600/fractional loss
intersection does not mean every small-size region passes. No bottom change
is integrated and no native port or new tuning follows. GO's historical
rejection remains valid. A new candidate needs an independently supported
primary coverage/registration or paint constraint, not another width sweep.
Evidence and immutable controls are in `/tmp/cp-eras-next/im-bottom-ground/`,
`in-bottom-login-ground/`, `io-bottom-profiles/` and `ip-bottom-echo-coupling/`.

KI extends the backing check to eleven fixed nine-column strips. Independent
review reproduces all 5,346 original RGB samples, 1,990 metric sets and three
context sheets; the historical SVG/native frame crops remain exact against
the current round-seventy-eight outputs. Four exterior strips match original
Login in all 153 pixels each. Six additional first/third-card strips lie under
Login cards and are not useful donors. The predicted-clear rightmost
fourth-card strip contains Login lettering: only 26/153 exterior pixels match.
Keep that failed layout prediction in the evidence.

The already accepted ordinary-face tint, applied without fitting to Login,
predicts nearby inside-face rows within maximum RGB errors of 2/1/1 at the
first/third strips and two fourth-card strips. The remaining fourth strip
has a localized red error up to 11 near printing. These observations support
the material approximation in the sampled clear bands, not direct equality
beneath the primary or a unique physical width. A further reconstruction may
test an explicitly conditional backing model with uncertainty and separate
controls; it must not treat the current echo approximation as measured source
backing. No geometry or paint changes follow from KI alone. Evidence is in
`/tmp/cp-eras-next/ki-bottom-backing/`; independent verification is retained in
`ht-echo-review/ki-verification.json` under the same scratch root.

KL tests a conditional backing made from the unchanged face tint and frame
echo layers over original Login. C1 off-primary pixels set a conservative
RGB uncertainty of ±(34,7,7), including layer quantization. Every off-primary
pixel in the C3 and two clean C4 strips fits that envelope. Their shoulder/
core projections consistently give effective encoded-RGB coverage near
0.513/1/0.379, with small perpendicular color residuals. The rightmost C4
donor also fails at the lower shoulder and remains explicitly invalid.
All 2,430 raw records, 306 metric sets, 135 projections, exact layer extraction
and an independent rendering replay are verified. The shoulder intervals
remain about 0.27–0.28 wide: these are conditional effective coverages, not
uniquely recovered physical widths or native linear-light/MSAA coverage.

KM derives one opaque box approximation from C1's mean projections alone:
inner y611.4526956 and outer y612.2412530, shared by all ordinary bottoms.
Only twelve existing vertices in three SVG paths change. This also moves
the effective bottom center from y611.9 to y611.8469743; it preserves the
continuous joins, other edges, face, echoes and ink. At 4K/1600/fractional,
all nine full-bottom comparisons improve, and none of 106 fixed regions
loses RGB L1 or RMS. Review reproduces 580 nonempty metric sets and inspects
all nine contact sheets. There are twelve isolated corner/cap pixel losses
across the three sizes, with gross L1 loss 25/132/50; the enclosing regions
improve. Selected art, contents, alpha and pixels outside bottom support
remain exact. KN's identical native geometry improves every changed pixel
at all three sizes (3,237/1,348/648 pixels), with no region losses. Sixteen
KP state cases preserve selection, ordinary held suppression, the fourth
held border, custom colors and clipping. The hidden opening stays exact;
the partial opening changes only the exposed upper shoulder. The component
specimen follows the same geometry.

[Round seventy-nine](../reference-svg-round79.md) integrates this bounded
bottom correction after both Store gates, 311 packaged tests, 22 checks,
28 first-attempt matrix cases and three-size actual-package parity. It
closes ordinary primary frame coverage, retaining the conditional backing
model and SVG corner/cap losses above. Fonts, caches, semantic inks, other
edges, faces, selected geometry and echoes stay unchanged; no primitive or
effect pass is added. Source evidence remains in
`/tmp/cp-eras-next/kl-bottom-coverage/` and `km-bottom-box/`; native, state and
package evidence is in `kn-bottom-native/`, `kp-bottom-states/` and
`kq-bottom-integration/` under the same scratch root.

Evidence: `/tmp/cp-eras-next/ff-neomil-store-frame/` through
`gw-neomil-left-integration/`. GR/GS/GT/GU establish the left-only correction;
GV retains the rejected lower-stem trial and GW the round-seventy-two package
check. GX retains the independently reviewed junction profiles; HI through
HO contain the fourth-cut source, native/state and latest package checks.
Runtime, fonts, text caches and original images match the tested source after
the final documentation-only updates.
