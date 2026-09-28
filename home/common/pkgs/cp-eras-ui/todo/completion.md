# Completion campaign — 2026-09-27

User requested continued work through all open cp-eras TODOs. Preserve
existing staged changes, keep new work staged/uncommitted, and use Astra
for orchestration/verification with Sol implementation lanes. Each source
correction needs local source/SVG/Iced review, relevant states, gates and
repository checks before its owning item can close.

This is a coordination ledger; the linked workstreams retain acceptance
criteria and completion status. Historical open lists are superseded by
later results. Repeated acceptance checklists are requirements for their
workstream, not separate features.

## Current lanes

| Lane | Scope | Owner record |
| --- | --- | --- |
| Neomil | Native verification of login materials; restore omitted store printing/details | [Login](neomil-login.md), [store](neomil-store.md) |
| Entropism/Kitsch store | Verify direct inverse contours; shared plain weapon art, socket scatter and marks | [E4/K8](reference-svg.md) |
| Neo-kitsch | Source veneer direction/convergence, then frame ink | [NK-07/NK-14](reference-svg.md) |
| Integration | Astra source/SVG/Iced review, shared material rendering and repository checks | This record |

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

K1 passes 267 Rust tests and all 22 repository checks. All27 visual cases
pass first attempt: 25 at 100.000%, and the two Neomil store cases at 99.998%.
The latter differ only in 170 margin-CJK pixels due to fallback fonts; a
source-vector replacement is assigned to L. Fourteen affected fidelity
gates pass, and all18 state captures are reviewed with exact production /
rest-preview parity. See the [eighth-batch record](../docs/reference-svg-round8.md).

Closed: broad inactive-login card/badge fields, broad store card/navigation
fields, and NK-08 weapon contours. All20 category/card backgrounds preserve
navigation/hit geometry and clipping. Independent material patches and
primary-art masks support those closures; fine typography, certification,
scan printing and veneer-flow work remain open. NK-15 records the runtime
socket/BASKET scatter mismatch found during native review.

## L batch under review

The next edits include Entropism body baselines, Kitsch mailbox/store type,
Neomil mailbox contour/terminal ridges, store certification/KIROSHI marks,
Neo-kitsch socket scatter and independently fitted maker-text copies.
The first L test run caught an outer-edge sample in the mailbox interior
seam test after its measured top moved; the sample now starts inside the
new contour, retaining every internal join. A deterministic source-vector
replacement for the small Neomil margin CJK run is pending before the
retry. L is not covered by K1's full-check result.

## Remaining implementation queue

- Reference typography: E2/E4, K5, NK-05 and remaining Neomil screen text.
- Source artwork: E4 inverse verification, K8, NK-08; Neomil store branding,
  weapons and trace-to-runtime printing.
- Materials: E5, K2, NK-07/NK-14; Neomil login/store fields and fine echoes.
- Neomil fine printing: dashboard maker/chip/footer, mailbox upper contour
  and cartridge/text details, login/store secondary copies.
- Performance: interactive resize evaluation; live hardware presentation
  remains permission-dependent.
- Integration: component/source documentation, state/clip review, relevant
  gates and full checks after integrated batches.

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
