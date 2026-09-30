# Completion campaign — 2026-09-27

User requested continued work through all open cp-eras TODOs and authorized
committing the accumulated N–AC work on 2026-09-29. Earlier staged-state
notes preserve the state during validation. Use Astra for
orchestration/verification with Sol implementation lanes. Each source
correction needs local source/SVG/Iced review, relevant states, gates and
repository checks before its owning item can close.

This is a coordination ledger; the linked workstreams retain acceptance
criteria and completion status. Historical open lists are superseded by
later results. Repeated acceptance checklists are requirements for their
workstream, not separate features.

## Current lanes

| Lane | Scope | Owner record |
| --- | --- | --- |
| Source research | Remaining fine Neomil printing, Kitsch type/fan material, Neo-kitsch veneer/frame ink and Entropism glyph limits | Owning source records below |
| Prerequisites | Original assets, reviewed interaction design, callers and authorized live desktop checks | External prerequisites below |
| Integration | AC complete: 286 Rust tests, 22 checks, 27 exact visual cases; included in this commit | This record |

## Completed H checkpoint

The immutable H snapshot passes 263 Rust tests and all 22 repository
checks. All 27 visual cases match 100.000% on their first attempt. All 16
source/SVG and 13 affected SVG/Iced gates pass with the final extractor;
19 controls preserve missing/moved and genuine-overlap failures, without
lowering matching thresholds. Thirteen goldens are intentionally refreshed.
See [the seventh-batch record](../docs/reference-svg-round7.md).

Closed after Astra review: Neomil primary avatars/portraits, password/card
printing and header/margin printing; Neo-kitsch login branding NK-04 and
shared ground NK-11; Kitsch ground K3; Entropism derived-sheet consistency
E6; and the remaining-upload profile/fix. Neomil long input and caret
behavior remain verified. Entropism's reference login footer is also fixed,
while E5 hub printing remains open.

The cache omits only all-zero RGBA bands. At 4K payload is 82.4 MiB versus
253.1 MiB, the measured later software-renderer callback interval is .511 s
versus 1.817 s, and peak process RSS is 988 MiB versus 3723 MiB. Rest/opening
captures are identical; the fractional interaction preview differs at one
pixel by one red level. Live hardware presentation remains separate.

## Completed K1 checkpoint

K1 passes 267 Rust tests and all 22 repository checks. All 27 visual cases
pass first attempt: 25 at 100.000%, and the two Neomil store cases at 99.998%.
The latter differ only in 170 margin-CJK pixels due to fallback fonts; a
source-vector replacement is assigned to L. Fourteen affected fidelity
gates pass, and all 18 state captures are reviewed with exact production /
rest-preview parity. See the [eighth-batch record](../docs/reference-svg-round8.md).

Closed: broad inactive-login card/badge fields, broad store card/navigation
fields, and NK-08 weapon contours. All 20 category/card backgrounds preserve
navigation/hit geometry and clipping. Independent material patches and
primary-art masks support those closures; fine typography, certification,
scan printing and veneer-flow work remain open. NK-15 records the runtime
socket/BASKET scatter mismatch found during native review.

## L and M review, resumed 2026-09-29

The earlier work was committed separately as WIP in `e04d72d`; the current
checkout was clean on resume. That commit records six failed visual cases
(21/27 passing). Their changes have identified provenance, but stale
goldens are not automatically accepted as the cause of every failure.

L passes 268 Rust tests and all 14 affected fidelity gates. Native and
fractional review supports the mailbox upper contour, tighter terminal
ridges, primary store certification/KIROSHI paths, source-vector margin
CJK, Neo-kitsch's 25-cell scatter, and dashboard microtext copies. Maker
native RGB RMS falls 21.19→12.18 and 23.02→12.82, with all changed pixels
inside the two runs; the early opening frame is unchanged. Repeated faint
printing and source texture remain separate. No full L repository check
was completed.

The resumed immutable M baseline passes 269 Rust tests. Astra review
accepts Kitsch selection-weight transfer across first/second/last messages,
Entropism stat height and EMPTY placement, and held fourth-card clipping.
It caught the Kitsch certification inset's wrong geometry, a still-tall
mailbox notice, and SOCKET text 1–2 native pixels low. Those have scoped
follow-up corrections awaiting new native verification. M's two affected
source gates and three implementation gates pass; these gates did not
expose all of the local defects.

## Completed N checkpoint

All 269 Rust tests and all 22 repository checks pass. All 27 visual cases
match 100.000% on the first attempt; the store's margin CJK vectors remove
K1's font-dependent pixels. N passes all 12 affected fidelity gates, all
24 SVGs parse with unique IDs, and seven fractional rest/held/selected/
custom states are reviewed. Ten reviewed goldens are refreshed. See the
[ninth checkpoint](../docs/reference-svg-round9.md).

The resumed M defects are corrected: Kitsch certification inset geometry,
notice cap height and Entropism SOCKET placement. Also accepted are the
four Entropism action widths, Neo-kitsch inner frame turns, and Neomil
navigation/footer fit with reference-only footer clearing. Tiny native
navigation/footer baseline differences move to O, alongside separately
measured ordinary/selected card type. Accepted L work now has integrated
validation: mailbox upper contour/ribs, primary certification/KIROSHI,
margin CJK, NK scatter and dashboard maker trails. Fine printing/material
limits remain explicit in the owning records.

Changes after resume stay staged/uncommitted; no deployment was run. The
immutable N result does not validate later O edits automatically.

## Completed T checkpoint — P–T source/native review

T passes 272 Rust tests and the release build. P–T local source/SVG/native
review covers eight affected screens, including fractional selection,
held, opening and custom-palette states. Accepted changes include the
Neomil card/footer fits, `0B` codes and margin calibration; Neo-kitsch
store branding/type, seven mailbox rows and dashboard customer header;
Kitsch compliance/footer tracking and fan fields; and the Neomil tiny
leading tape-mark ink. Reviewed goldens are refreshed for the full check.

The 200 ms bounded resize preview has real compositor-event evidence and
a separately reviewed preview image. The integer final frame matches an
exact baseline byte-for-byte; the fractional resized control differs at
three pixels by one level, with common footer history differences measured
separately. One fractional sequence still performs a 404 ms intermediate
preparation, so continuous resize is not declared fully solved. See the
[resize measurements](../docs/neomil/resize-preparation.md).

The immutable T snapshot passes all 22 repository checks and all 27
visual cases, each matching 100.000% on its first attempt. All 24 SVGs
parse with unique IDs and the latest 16 affected fidelity gates pass.
A new selected-card frame/side-printing misalignment is recorded in the
Neomil store TODO and remains open; U implementation follows T.

## Completed Z checkpoint — U–Z integration

Z passes 275 Rust tests, 20 extractor/gate tests and all 22 repository
checks. All 27 visual cases match 100.000% on their first attempt. The
Nix source agrees with all 195 frozen source/script/documentation/test
hashes. Eight reviewed goldens integrate selected-frame/chip-2 corrections,
Kitsch fan/label work, Entropism values, Neomil mailbox ribs, Neo-kitsch
store frame/veneer and the native footer-code calibration.

The classification-boundary gate fix uses strict .95 canvas-mask overlap
and compatible template/ink/corner evidence. It recovers exactly one
component and passes at 89%; missing, moved, merged, wrong-corner and
malformed-evidence controls pass, and 80 legacy verdicts remain unchanged.
See [round eleven](../docs/reference-svg-round11.md). This closes the
bounded selected-frame and chip-2 integration items. Finer source fidelity
and new AA work remain separate; nothing was committed or deployed.

## Completed AA checkpoint

AA passes 285 Rust tests and all 22 repository checks. All 27 visual
cases match 100.000% on their first attempt, and all 197 frozen
source/script/documentation/test hashes agree with the Nix snapshot.
Both affected store gates pass without threshold changes. Sixteen store
captures verify socket directions, semantic feedback, selection, opening,
custom palettes and the fourth-card cutoff; only the reviewed store golden
changes. The bounded directional-echo task is complete; exact irregular
grain remains separate.

Eligible post-quiet resize preparation now runs on one worker, with stale
job rejection and complete atomic publication. The no-heartbeat sequence
proves completion polling produces an exact redraw. Complete preview
callbacks measure .015–.033 ms; final controls include the explicitly
recorded three one-red-level fractional edge differences. This closes
UI-thread offloading, not hardware presentation or smooth-drag verification.
See [round twelve](../docs/reference-svg-round12.md). Changes remain staged;
nothing was committed or deployed.

## Completed AB checkpoint

AB passes 286 Rust tests and all 22 repository checks. All 27 visual cases
match 100.000% on their first attempt, and all 199 frozen file hashes match
the Nix source. Six affected fidelity gates pass unchanged; 26 native
captures cover source comparison, fractional states, selection/held,
custom inks and opening. Three reviewed goldens change in this batch.

Closed bounded corrections: Neomil's small margin word/slash placement and
primary contours, Kitsch's SC corner apertures and native shelf-brand runs,
and Neo-kitsch's native RIFLES action labels. A store material-selection
regression also preserves independent opacity artwork. The O/plaque and
A2 junction, photographed ink softness and exact glyphs remain explicit.
See [round thirteen](../docs/reference-svg-round13.md). No commit or deploy.

## Completed AC checkpoint

AC trims the unsupported rectangular end of the Neomil margin plaque.
Only 126 native 4K pixels change; source-dark intrusions in the leading
patch fall 88→4. The O counter, A2, primary letters, CJK and slash stay
unchanged. Ten native captures preserve fractional card/held/opening and
custom-color behavior. Both gates and 286 Rust tests pass.

The full check passes all 22 checks, including 27 exact visual cases on
their first attempt. All 200 frozen file hashes match the Nix source, and
only the Neomil store golden changes in AC. The wider plaque clip and
uniform letter-dimming proposals are rejected. See
[round fourteen](../docs/reference-svg-round14.md). Changes stay staged;
nothing was committed, pushed or deployed.

## Remaining implementation queue

- Fit Neo-kitsch veneer density/direction and remaining frame ink locally.
- Neomil margin O/plaque: primary word/slash and slanted leading boundary
  are complete. Exact O ink and the second-A junction remain separate.
- Neomil footer fine printing, repeated/margin store marks,
  and source-supported login/mailbox/store scan echoes.
  X's footer-stripe migration is rejected for worse native edge controls.
  Directional socket tails are complete in AA; irregular fine grain remains.
- Kitsch SC corner apertures and shelf-brand width/weight are complete in
  AB. Exact tiny certification lettering, metadata ink, fan overlap and
  photographed softness remain.
- Entropism E2/E4 and Neo-kitsch NK-05 retain exact glyph/printing limits.
  No additional primary-content omission was identified by the AB audit.
- New-size cache characterization is complete (see performance record).
  Bounded resize previews and exact final redraw are implemented and reviewed.
  Eligible post-quiet preparation is off the UI thread and verified in AA.
  Actual desktop resize/input/presentation remains permission-dependent.
- Apply source/component/native/state/gate/full-check requirements to each
  new batch; these are acceptance rules, not duplicate implementation tasks.

The old broad queue included already-fitted Entropism grounds/hub printing,
replaced rifles, ported Neomil printing and fixed socket scatters. The owning
records now distinguish those completions from exact glyph/material limits.

## External prerequisites and non-tasks

- Live desktop windows require explicit authorization under repository
  agent guidance. A request is pending for temporary demo windows using
  synthetic input, with no switch or greeter restart. Headless validation
  cannot close live input/IME, hardware presentation or actual login-session
  validation.
- Exact original font/texture recovery needs original assets or a separately
  validated reconstruction. Available source images support further local
  fitting; source assets/paths have been requested. Do not claim unknown
  authoring recipes are recovered from an approximation.
- Kitsch dashboard hover and animated interaction transitions need concrete,
  reviewed design rules before implementation. Still-image destinations do
  not establish transition timing or an extra hub hover state.
- Toolkit widgets without callers remain explicitly deferred in
  [toolkit.md](toolkit.md). Do not create unused widgets to tick a box.
- The Kitsch mailbox yellow extractor diagnostic is explicitly do-not-fix
  in [design-pipeline.md](design-pipeline.md); it is not a renderer defect.

Validation and task closures will be recorded as each lane is reviewed.
