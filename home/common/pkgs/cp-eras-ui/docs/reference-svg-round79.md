# Round 79 — ordinary Neomil Store bottom coverage

Ordinary bottom borders were too bright on both sides of the central source
row. This correction changes twelve existing SVG/native outline vertices
to inner y611.4526956 and outer y612.2412530. Continuous joins, other edges,
card faces, selected artwork, echoes and semantic inks retain their existing
construction. The inferred ordinary second card shares the first geometry.

The change adds no primitive, material, mask, blur, font measurement or
draw pass. Fonts, text shaping and the audited caches stay unchanged.

## Source model and review

KI validates clear Login/Store ground strips and the existing ordinary-face
tints. KL then tests those fixed tints and accepted frame echoes over Login
as a conditional backing model. First-card off-primary pixels establish
RGB uncertainty ±(34,7,7); all corresponding third-card and two fourth-card
controls fit that envelope. One rightmost donor strip fails because Login
lettering occupies it, including its lower shoulder. It remains in the
evidence and in subsequent direct Store/render comparisons.

Four usable strips consistently show effective encoded-RGB coverage near
0.513/1/0.379 across upper shoulder, center and lower shoulder. KM derives
one opaque box from the first-card means alone, with no parameter sweep or
held-out refit. The resulting effective width is about 0.7886 and its center
is y611.8469743. This is a bounded reconstruction, not recovered physical
width or original authoring. The backing assumption and roughly 0.28-wide
shoulder intervals remain explicit. Native linear-light/MSAA coverage is
different, so KN tests the geometry directly through the actual Iced Store.

At 3840×2160, 1600×900 and 1537×947, all nine complete-bottom source
comparisons improve in each renderer. None of 106 frozen regions regresses
in RGB L1 or RMS. Independent review reproduces 580 nonempty metric sets per
renderer and inspects all eighteen source/baseline/candidate contacts. Empty
small-size one-row controls are unavailable, not successes. Native complete
bottom RGB L1 improvements are:

| Size | Card 1 | Card 3 | Card 4 |
| --- | ---: | ---: | ---: |
| 3840×2160 | 128,071 | 129,054 | 63,401 |
| 1600×900 | 33,309 | 33,835 | 16,605 |
| 1537×947 | 17,464 | 17,975 | 8,826 |

Every changed native pixel improves RGB L1: 3,237/1,348/648 pixels at the
three sizes. SVG retains twelve isolated corner/cap pixel losses, with
gross L1 losses 25/132/50 respectively, inside improved regions. Neither
renderer changes alpha, selected art, contents or pixels outside ordinary
bottom support. The ordinary component specimen changes only its matching
bottom at both tested sizes, with exact current baseline replays.

All 266 library tests pass.

## Integration verification

All sixteen targeted headless state cases pass. Selected geometry and
ordinary held-border suppression remain exact; the fourth held border keeps
its distinct bright construction. Custom colors and inferred ordinary-second
geometry pass, as do hidden and partial bottom reveals. The disabled wrapper
reproduces the actual Store before/after frames. The source SVG, ordinary
component and 1,348 changed golden pixels follow the reviewed correction.
All 311 packaged Rust tests, 22 repository checks and 28 first-attempt
matrix cases pass. Twenty-six of 27 visual images match their goldens
pixel-for-pixel; the existing Neo-kitsch bar difference remains one pixel,
one channel level. The other matrix case verifies source sampling. Fresh
actual-package captures exactly match the reviewed candidate at all three
sizes, and both Store fidelity gates pass. Changes remain staged and
uncommitted.

The earlier uniform 0.6 bottom trial remains rejected; the accepted trial
uses new source coverage evidence and retains its own limitations. Faint
printing, exact background recovery and live desktop behavior remain separate
tasks. No new performance result or live GPU verification is claimed.

Evidence is under `/tmp/cp-eras-next/`: `ki-bottom-backing/`,
`kj-bottom-identifiability/`, `kl-bottom-coverage/`, `km-bottom-box/`,
`kn-bottom-native/`, `ko-bottom-component/`, `kp-bottom-states/`,
`kq-bottom-integration/` and `ht-echo-review/`.
