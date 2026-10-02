# Reference SVG round forty-nine — stroke ownership and notice weight

CW supplies the preceding verified implementation checkpoint: 295 local/Nix Rust
tests, both Kitsch Dashboard inventory gates, 24 SVG checks, all 22
repository checks and all 27 visual cases pass. Its packaged 4K Dashboard
matches the reviewed nearest-card correction exactly. Changes remain
staged and uncommitted. This round integrates the reviewed CZ stroke
correction and records rejected notice trials; all 29 broad TODOs remain
open. CZ passes the new repository/package checkpoint recorded below.

## Kitsch: separate stroke ownership from fill opacity

CX finds source-owned clear interiors at depths two and three on WEAPONS,
VEHICLES, right PRODUCTS and LOCATIONS. The shared nearest-card red law
does not support another simple opacity/ink transfer: the bounded fit
would substantially worsen VEHICLES red, and several independent red-only
opacity estimates fall outside physical bounds. A zero-fill SVG supplies
a modeled underlayer, not an observed photo underlayer.

WEAPONS provides two useful same-stroke comparisons. For depth two over
depth three, source inside/outside G+B peaks are 0.48/161.45, versus
124.41/160.78 in the current SVG. Depth three over depth four gives
−0.20/133.38 versus 103.38/126.71. The other stacks lack sufficiently long
paired spans after excluding crossings; they retain color controls but
do not independently establish these ratios. Astra reviews the ownership
overlay and independently reproduces all twelve source/current/zero-fill
peak and area controls. Evidence: `/tmp/cp-eras-next/cx-kitsch-depth/` and
`cy-root-review/cx-depth-root-review.json`.

CY tests a shared reconstruction rule across all six stacks: hide farther
strokes beneath their own depth-two/three card silhouettes while retaining
every fill, stroke ink/width, card geometry and paint order. This does not
identify the original renderer's mechanism. A split-only SVG control is
pixel-exact at 4K, 1600 and the uniform fractional size. The candidate adds
twelve masks and twenty-eight fill/stroke splits. An initial mask on a
transformed use was invalid and is retained separately; the corrected
candidate uses untransformed wrapper groups.

The scratch SVG improves covered-stroke RGB error on all six stacks at
all three sizes, while WEAPONS and right PRODUCTS retain red-channel
losses. Mask fringes and small raster differences outside the analytic
envelope remain explicit. Astra has reviewed the source/current/candidate
montage and independently verifies all mask coordinates, reversible XML
changes, split replay and changed-region source scores.

CZ ports the same rule through existing bounded masked primitives. The
white-mask native control is exact at 4K, with one/two one-level blue
residuals at 1600/fractional size. Opaque white luminance evaluates to
0.99999994 in the existing f32 compositor; there is no intermediate 8-bit
buffer. This rounding floor is retained explicitly.

Native changed-region RGB MAE falls from 20.39/30.43/28.68 to
11.21/11.41/10.08 at 4K, from 17.06/21.63/20.18 to 10.68/10.79/9.31 at
1600, and from 18.11/22.94/21.41 to 11.77/11.42/10.23 fractionally.
Every stack's mean RGB error improves at every size. WEAPONS and right
PRODUCTS red still worsen; the correction removes excess visible edges,
not the deeper-fill color mismatch. Alpha and foreground interiors are
exact, with one WEAPONS boundary pixel changing at 1600. Seventeen paired
synthetic states pass locality, selection, hover, held-trail, release,
cancel and opening checks; the three rest states match actual Hub output.
The LOCATIONS component preserves parent mask/stroke geometry, placement
and visible text; its full/half split controls replay exactly.

Three paired ABBA CPU blocks show a real cost: 4K cold preparation rises
63.75→70.23 ms (+10.2%); direct compositing rises 51.82→58.84 ms. At 1600,
cold preparation rises 13.45→14.73 ms. The increase exceeds between-block
variation and is accepted as the cost of this bounded fidelity correction.
Cache hits remain about two microseconds; output and retained bytes are
identical. These are CPU preparation measurements, not GPU/frame/FPS
results. No shared renderer or cache policy changes.

The integrated runtime removes only six unused white-control constants
and updates a comment from the reviewed scratch source. SVG, component
and canonical golden use the reviewed candidates. Evidence:
`cy-kitsch-stroke-clip/`, `cz-kitsch-{native,runtime,states,components,perf}/`
and independent `cy-root-review/` records.

DC independently inventories the remaining clear-interior color error.
WEAPONS depths two/three need about +19/+22 displayed red and right
PRODUCTS +36/+34, while VEHICLES and LOCATIONS remain within roughly
three levels. Astra reproduces all eight region means and thirty-two
fixed long-axis bins. Nearby exposed ground differs too, especially at
right PRODUCTS; the coarse controls lie over forty source pixels away
and do not reveal the hidden underlayer.

DD finds different displayed residuals in the same ten-level bins of the
existing rose field. Astra reproduces the bins and corrects an overly
strong inference: these are not exact field matches, and overlapping
depths have different transmission. This does not formally rule out the
field, but it does not justify fitting it alone. Immediate, independently
owned ground/interior controls precede another material candidate.
Evidence: `dc-kitsch-depth/`, `dd-kitsch-rose-depth/` and their
`cy-root-review/` measurement records.

DE finds narrow exterior strips 4–12 original pixels from the card edge.
Astra reviews all four source/owner crops and reproduces the paired
measurements. Near-interior minus exterior red residual is 5.77/7.21 for
WEAPONS and 15.75/10.31 for right PRODUCTS at depths two/three; registered
1600 comparisons retain similar differences. These are boundary controls,
not the whole clear cores: WEAPONS' near-interior deficit is only about
four levels, while its larger core deficit occurs elsewhere.

The 708–1,284 exterior pairs reuse only 46–82 distinct interior pixels.
The 1600 statistics retain the original pair weighting and repeated
destination pixels. Visible photo halos also contaminate these strips;
green/blue residual decays across the exterior distance shells. The
geometric strips are useful for near-edge comparison but do not reveal
clean hidden background or uniquely identify fill ink/opacity. No further
material fit is accepted. Evidence: `de-kitsch-ground/` and
`cy-root-review/de-ground-root-review.json`.

## Neomil Login: separate weight, advance and compositing

CX compares all four inactive notice lines with current native output.
The fixed BJ and CT sRGB composites improve whole-line RGB error at 4K
and 1600, but CLASS, O counters and some of the thirty-five gap controls
regress. Astra independently reproduces 486 regional RGB/extra-ink checks
and reviews enlarged CLASS crops. The fixed masks and ink fail one 1600
line under linear blending; this does not establish that every possible
ink or weight needs a new rendering route.

The existing login echo machinery can rasterize local outlines, but it
draws before the primary Art layer and only approximates compositing
against the page background. A possible primary-print route would need
separate ordering and the verified flat card-foot colors, while retaining
reference-palette eligibility, active AJ artwork and responsive Grid
placement. That architecture is not accepted merely because a whole-line
score improves. Evidence: `cx-neomil-route/` and
`ct-root-review/cx-notice-root-review.json`.

CY substitutes the exact FreeSans Regular face for the four BJ inactive
lines without adjusting size, anchors, tracking or ink. Its Bold control
replays exactly; font resolution is pinned. Regular is lighter, but its
shorter advances worsen every complete line against current native at
both sizes. This is a weight-plus-advance experiment, not isolated hinting
or proof of the original font family.

CZ applies one shared tracking increment, derived only from the first
training line's 4K endpoint shortfall: `20 / (61 × 2.4)` design units per
gap. All four 4K line errors improve, but the held-out first line on card
three worsens at 1600, and CLASS/gap losses remain. No native route or font
substitution is accepted. Evidence: `cy-neomil-weight/` and
`cz-neomil-regular-spacing/`.

DA fits one common encoded-sRGB opacity, 0.692644, for the frozen Bold
mask using only card two's first 4K line. All eight canonical whole-line
RGB errors improve against current native, but every R210 bright-core
pixel disappears; the source retains 4,828 at 4K and 78 at 1600. Error
inside the training line's source core worsens while exterior error
falls. CLASS, O-counter and gap losses remain. Astra independently
reproduces the scalar, all 1,944 regional error/coverage checks, and exact
support/alpha controls, and reviews the canonical montage. This is an
aggregate error tradeoff, not an accepted opacity or printing correction.
Evidence: `da-neomil-bold-opacity/` and `cy-root-review/da-root-review.json`.

DB therefore isolates outline thickness while preserving the existing
Bold outlines' advances and anchors. Its zero-width control uses local
4× rasterization followed by area averaging. This changes antialiasing:
four-line RGB error differs from exact CT by 1,346 at 4K and 75 at 1600,
with larger relative drift in individual gaps. The single training-line
fit chooses an inward radius of 0.115 design pixels. It improves aggregate
RGB error beyond the control drift, but leaves only seven R210 pixels at
1600 versus 78 in the source, worsens source-core error, and retains
CLASS/counter/gap losses. One held-out 1600 line worsens against exact CT.
Astra independently reproduces 2,592 regional error/coverage checks and
reviews the canonical montage. This trial is also rejected. Evidence:
`db-neomil-contour/` and `cy-root-review/db-root-review.json`.

## CZ verification checkpoint

All 295 local and Nix Rust tests, both Kitsch Dashboard inventory gates,
24 SVG structure/reference checks and all 22 repository checks pass.
All 27 visual cases pass on the first attempt: 26 are pixel-exact, with
the historical one-pixel/one-channel Neo-kitsch bar residual unchanged.
The 325 tracked files in the tested 16 MB source snapshot match their
frozen hashes, and all sixteen original photo inputs remain unchanged.

The fresh packaged 4K Kitsch Dashboard is pixel-identical to the reviewed
native candidate; the installed generated font is retained unchanged.
Package: `/nix/store/d64f4x2jhcmcd85aqawirsibp6hdcllc-cp-eras-ui-0.0.0`.
Evidence: `/tmp/cp-eras-next/cz-validation/`. Final reconciliation changes
only Markdown; runtime, SVGs, golden, locks and originals remain exactly
as verified. Repository heads are unchanged and all work remains staged
and uncommitted. The 29 broad open items are not closed by this partial
material correction or by the rejected Login trials.

Builds, native captures and timing runs remain serialized under Astra.
Sol workers own disjoint scratch studies; production code, goldens and
TODO closure remain with the orchestrator after independent review.
