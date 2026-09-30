# Eleventh source-reference checkpoint — U–Z, 2026-09-29

This checkpoint records the reviewed U–Y corrections and Z footer
calibration. U passed 273 Rust tests and six affected fidelity gates;
Z passes 275 tests. Native review, the original Neo-kitsch gate failure
and its tested extraction-evidence correction are recorded below.
The immutable Z snapshot passes all 22 repository checks and all 27
visual cases, each matching 100.000% on its first attempt.
Changes remain staged/uncommitted.

## Initial U changes

- Neomil selected store frame: move only its top/chamfer/right contour,
  fan and selected side printing to measured source anchors. Preserve
  the left edge, primary icons/title/gun, ordinary cards, material ramp
  coordinates and fourth-card clipping. See the
  [measured frame plan](neomil/store-selected-frame.md).
- Neo-kitsch dashboard: fit security/level/T labels independently,
  including the selected T2 LEVEL. Preserve inks and frame geometry.
  A separate native trial adds local tracking to seven module labels
  while retaining their right anchors and baselines. See
  [typography measurements](neokitsch/dashboard-header-typography.md).
- Kitsch dashboard: fit idle-face red illumination to the established
  rose field. An existing masked-ramp software layer carries the face
  materials; foreground outlines/labels and selected/pressed gold remain
  in their current branches. Held backdrops retain the material layer
  while removing only the target ghost trail. See
  [material measurements](kitsch/dashboard-material.md).

## U native findings and V follow-up

The selected Neomil top/right movement and side printing improve source
alignment while native icons, title, gun, middle wash and an ordinary card
remain pixel-identical. Native review catches a top stroke that is four
bright rows instead of the source's one, and a selected PETROCHEM box
still using dark gun ink despite the bright source/SVG. A split-outline
trial with a .4-design-pixel top rule at y154.5 reduces independent top
strip red MAE from about 61.3 to 20.0–21.1. Its selected box uses the
foreground role. These scoped runtime corrections move to V.

Neo-kitsch header geometry improves across the isolated text masks. Module
tracking improves all seven native glyph-center RMSEs, including BRAINDANCE
11.31→1.92 pixels and DEVICES 6.04→.76. Small remaining constant offsets
support per-run native anchor corrections in V; a left T1 size/baseline
trial does not improve overlap and is rejected. Brighter flat header inks
also fail independent edge comparisons and remain unapplied.

Kitsch's five idle-face native held-out patches improve RGB MAE exactly
as predicted by SVG, within rounding: 10.51→7.17, 8.04→6.69, 10.55→4.64,
12.28→5.83 and 10.91→4.62. All 217,294 changed 4K pixels lie inside those
five faces. Fractional rest/held/opening comparisons against equivalent
T controls have three gold-edge pixels differing by one channel level;
all other changes stay in the idle faces. The selected0 control has no
changes outside idle faces. Layer order, held-trail removal, selected gold,
opening clips and custom-palette review are preserved.

A serialized 4K cache harness measures a 12,533,760-byte increase (11.95 MiB)
in retained RGBA, both at rest and after all six held variants: the uncut
field has one shared cache entry. Total payload after those variants is
204.96 MiB, within the unchanged 384 MiB budget. Three U cold draws range
208–246 ms (median 233 ms), versus adjacent T 210–237 ms (median 218 ms);
these overlapping ranges do not establish a precise timing regression.
Warm preparation is about .0036 ms. This is CPU material preparation, not
GPU presentation. Native label review identifies a separate Medium-weight,
per-run spacing/placement fit for V; it does not change these materials.

V must receive fresh native/state review and affected gates before golden
updates and the full integrated repository check.

Fine source softness, exact glyph shapes and unresolved veneer/printing
fits remain separate. No live desktop or deployment validation is claimed.

## V native review

V passes all 274 Rust tests and the release build. The compositor checks
caught an invalid foreground placement for the new chip mask; the two
chip-2 faces and numeral now join the existing leading material layer.
Selected store presses retain their outline geometry with its paint
disabled, preserving the existing feedback contract.

The complete native Neomil store frame is pixel-identical to the reviewed
split-top/bright-box trial. Three independent top strips improve red MAE
from 61.37/61.26/61.34 to 20.03/20.71/21.08. Fractional rest, held ordinary,
held selected, selected/held fourth card, custom palette and opening
captures preserve clipping and feedback.

Chip-2's continuous lateral ink field and independently sampled echo
phase improve all three native numeral regions: core RGB MAE falls
15.21→13.52 at the top, 14.33→10.36 in the middle and 18.10→13.17 at the
foot. Across all 113 core pixels, error falls 15.73→11.95; the 66 edge
pixels improve 20.05→17.43. Exactly 200 native pixels change. Of those,
182 are in the numeral and 18 are chip-face boundary rounding changes
of one channel level after moving those faces into the software layer.

Neo-kitsch module glyph-center RMSE improves for all seven labels:
EMAIL 2.65→.71, MATRIX 1.71→.87, BRAINDANCE 1.92→.61, PRIVATE 1.59→.57,
SECURITY 1.24→.35, SYSTEMS 1.71→.63 and DEVICES .76→.33 native pixels.
The left LEVEL now has the source's exact thresholded bounding box;
its IoU is .571. Rejected flat-ink and left-T1 changes remain unapplied.

Kitsch's Medium label fits improve all six native labels. A separate
first-third calibration predicts further runtime-only shifts for VEHICLES,
WEAPONS and both PRODUCTS runs; all middle/last holdouts improve at three
thresholds. Similar EVENTS/LOCATIONS shifts fail independent holdouts and
are rejected. The four supported calibrations move to W for actual native
validation, alongside source-backed ghost width/count/pitch, Neo-kitsch
lower outer turns and Neomil normal-cartridge rib corrections.

All 26 V native captures are reviewed and all eight affected source and implementation gates pass.
Chip-2's fractional rest/opening comparisons each change the same 99
pixels, confined to its two chip faces; the continuous field has no band
seams. Later source snapshots receive separate review below; this
historical V result does not validate them automatically.

## W native verification

The frozen W snapshot passes all 275 Rust tests, the release build and
all four source gates. The value-label feedback test now accepts both
text primitive forms while preserving its original ink contract. Native
Kitsch label calibration improves all first/middle/last comparisons at
three thresholds. Ghost edge widths approach the source: the reviewed
profiles are about 2 native pixels wide versus the previous 2.5–2.75.
All seven measured edge-profile errors improve. The larger right reveal
contains the added far silhouettes; native/SVG changed-region correlation
is .99. Two exposed opposite-edge holdouts agree within 0–1 native pixel at every
fan depth. The original middle-edge probe sampled an obscured line with
less than 1.1 intensity units of signal; its apparent offsets are unusable.
Translucent overlap visibility remains a separate material difference.

The normal mailbox rib audit found a coordinate distinction: a phase
slope measured relative to the sloped strip top cannot be used directly
as Cartesian endpoint dx/dy. W is retained as an intermediate capture;
the corrected Cartesian angle is implemented and reviewed in X below.

All 25 W native/state captures are reviewed. Three implementation gates
pass; Neo-kitsch store's shape gate fails at 46.40% matched area. Local
comparison diagnoses a classifier boundary rather than a missing card:
P→W changes only the four lower-frame regions (3,173 SVG pixels and 2,986
native pixels at 1600×900), with no changed pixels above y580. The same
12 eligible shapes still match, accounting for 127,580 square pixels.
The SVG's large selected amber component changes fitted IoU from .6003
to .6217, crossing the unchanged .62 classification cutoff. It adds
115,983 square pixels to the denominator; the existing native component
remains an ignored blob at .5808 despite .965 bounding-box overlap.
Source-anchored palette extraction also fails and is not substituted.
No drawing, threshold or class exemption is changed to force a pass.
The original X and Y gates also fail at 47%. Local source/native review
remains necessary for this component regardless of the aggregate verdict.

## X native corrections and rejected footer trial

The normal-mailbox strip uses 20 ribs at a native cadence of about 3.31px.
X converts the phase slope relative to the sloped top into Cartesian
endpoint geometry. All four native row comparisons improve; complete
changed-pixel RGB MAE falls 24.26→20.40. Exactly 1,141 pixels change, all
inside the seven normal terminal strips. Fractional selection and held
feedback preserve the distinct selected art. Both mailbox gates pass.

Entropism's W value fit improves ordinary and selected glyphs. X moves
only the stretched native runs up one 4K pixel. Source-mask overlap rises
.446→.554 for the selected pair, .602→.642 for the third-card pair and
.614→.664 for held card4. Exactly 2,057 pixels change, all in the five
stretched runs; the already-aligned `5` runs stay unchanged. Both gates
pass and fractional selection/press states retain their ink contract.

Neo-kitsch lower-frame echoes now resolve all six native bottom ridges.
Ordinary positions are within one source pixel; selected inner positions
retain 1–3px differences. A selected-only spacing trial worsens independent
side bends and is rejected. The fifth line's faintness and material remain
open, as does the shape-classification limitation described above.

The footer side-stripe proposal improves one edge but worsens the complete
native changed footprint 15.55→16.56 RGB levels and multiple independent
edge controls. It is rejected; the previous frame rendering is restored.
No software-layer migration or stripe masks remain in production. See
[the rejected-trial record](neomil/dashboard-fidelity.md).

## Y selected veneer seam

The selected store seam turns upward at its right end, toward the source
centerline, while all 125 surrounding strands retain their geometry.
Native ridge-location errors improve 5.0→2.5 pixels in the fit region and
7.0→.67 in the spatially separate comparison. All 1,084 native changes are
within the seam. Fractional rest and held fourth-card states carry the same
correction. The source gate passes; the original implementation gate still
fails at 47%. A broader cubic-strand trial is rejected because local ridge
density and RGB errors worsen despite better angles. See the
[veneer measurements](neokitsch/veneer-fit.md).

## Z footer calibration

Four actual native trials isolate baseline, glyph height and font weight.
The unchanged control is pixel-identical to the restored V/W dashboard.
A height/baseline correction using Bold 9.2, width compensation and a
−.833333 design-pixel baseline shift improves every one of the 13 glyph
comparisons. The first `6` now matches the source bounding box; its RGB
error falls 25.02→15.69, while the independent `8` improves 23.50→18.11 and
the later-code region 29.06→24.19. All 1,749 changes are inside the code;
the frame and captions are unchanged. Z production matches that trial
pixel-for-pixel. Fractional rest/custom/opening/held changes are confined
to the code, and no-config fallback equals the reference dashboard. Both
fidelity gates pass. Final integrated checks follow below.
Exact contours, fine striations and repeated-letter softness remain open.

## Component identity across a classification boundary

The extractor now retains the latent template class and a compact,
lossless mask of each extracted split cell. These are the existing
post-processed extraction cells, including intentional hole filling;
they are not raw photographic masks or proof of exact edge fidelity.
Ordinary classified matches are unchanged. Only an unmatched source
template may recover an unmatched candidate blob, and only with the
existing box threshold, compatible ink/template/corner orientation and
at least .95 mask IoU at the original canvas coordinates. Matching is
one-to-one, without translation or dropping source area. The template
classification and area thresholds remain unchanged.

Fresh X and Y specs recover only the large selected component, whose
mask IoU is .96859. Both gates pass at 13/21 shapes and 89% matched area.
Astra independently decoded and viewed the masks; the real upper/left
edge differences remain visible and are not treated as exact fidelity.
All 20 extractor/gate tests pass, including missing/moved components,
different corners, merged overlap, duplicate sources and malformed or
incomplete evidence. Old specs lacking this evidence preserve their
previous behavior, including the original 47% failure.

## Integrated Z validation

All 275 Rust tests, 20 extractor/gate tests, the release build and all 22
repository checks pass. The 27 visual cases each match 100.000% on their
first attempt. The Nix source snapshot matches all 195 frozen source,
script, documentation and test hashes and occupies 15 MB. Eight reviewed
goldens are refreshed from the V–Z native captures; X's rejected footer
stripe trial is excluded. All 24 SVGs parse with unique IDs. Replaying
80 cached legacy shape/ink comparisons changes no verdict.

This integrates the local corrections described above, including the
selected store frame and chip-2 ink. Exact material/printing limits and
live-desktop checks remain open. Subsequent socket-tail and background
resize preparation changes need their own review and are not covered by Z.
