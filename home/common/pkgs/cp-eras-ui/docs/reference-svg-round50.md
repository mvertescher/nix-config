# Reference source review — round fifty

This round narrows the remaining Kitsch fan and Neomil inactive-notice
work. It does not accept a new runtime or SVG correction. Astra reviews
the source and reproduces the measurements; scoped Sol workers prepare
the diagnostics, and a Luna inventory checks the remaining queue.
All 29 broad acceptance scopes remain open. Changes remain staged and
uncommitted.

## Kitsch: separate hidden ground from fan material

DF compares the original Dashboard, Mail and Store photographs at the
same 3840×2160 coordinates. A previously unused clear footer patch,
design `(1400,800)..(1510,835)`, is pixel-exact across all three screens
(22,176 pixels). A clear patch above the right fan has per-channel mean
differences below 0.02 and high-pass correlations of 0.948–0.991.
Zero-offset texture error is 0.36–0.40 levels versus 2.70–2.87 at adjacent
one-pixel offsets. A clear patch right of the fan agrees between Dashboard
and Mail; Store paints a selected card there. These controls support the
existing registration and shared ground locally, not everywhere.

The left Dashboard wash differs from Mail/Store. Mail's lower body also
differs from Store by about 8–11 red levels at jointly clear LOCATIONS
sample coordinates, despite neither SVG modeling foreground there.
Visible halos and screen-dependent shading remain possible. Matching
grain does not establish whether grain was applied before or after UI
composition, or identify the original transparency.

The donor audit keeps each current Mail/Store SVG's exact definitions and
three backdrop rectangles, removing only foreground. Its foreground
support is the difference between full and ground-only renders, expanded
by eight design pixels. Astra reproduces both complete exclusion masks
and all sixteen intersections with the frozen CX depth-2/3 sample sets,
then inspects the original crops. Store excludes every right-PRODUCTS
sample; Mail exposes part of the upper depth-3 cap. The mask is an
exclusion aid, not proof of source-photo ownership. Transparent panels,
untraced shading and halos can invalidate an apparently clear donor.

DG measures current-model ground sensitivity by changing only eleven
backdrop red values: six rose stops, four wash stops and the page color.
The no-op SVG is byte-identical to production, and its render equals the
reviewed CZ SVG render. Predeclared symmetric steps of 1, 4 and 8 leave
every green, blue and alpha pixel unchanged, with no color clipping.
Astra independently checks the XML changes and complete output planes.
Step-8 region-average output-red responses range from 0.515 to 0.697 per
input level; step-4/8 means differ by at most 0.014. Small bins remain
quantized. These are renderer sensitivities, not measured source alphas.

Stronger donor guards retain the following original-pixel comparisons.
The right-PRODUCTS subset is restricted to Mail above design y=300, away
from its solid header. Each row compares card and donor at the same
frozen coordinates; sample counts change with the guard.

| Region and donor | Foreground guard, design px | Pixels | Card source minus current red | Donor source minus modeled ground red |
| --- | ---: | ---: | ---: | ---: |
| WEAPONS depth 2, Store | 8 | 4,294 | +25.81 | −2.43 |
| WEAPONS depth 2, Store | 16 | 1,789 | +27.00 | −2.36 |
| WEAPONS depth 3, Store | 8 | 3,177 | +18.14 | −0.54 |
| WEAPONS depth 3, Store | 16 | 1,576 | +15.49 | +0.13 |
| WEAPONS depth 3, Store | 24 | 166 | +12.48 | +1.91 |
| Right PRODUCTS depth 3, Mail | 8 | 3,034 | +42.78 | +20.77 |
| Right PRODUCTS depth 3, Mail | 16 | 2,239 | +43.02 | +20.58 |
| Right PRODUCTS depth 3, Mail | 24 | 373 | +43.63 | +20.32 |

No depth-2 WEAPONS samples survive the 24-pixel guard. These data provide
better evidence than the neighboring-edge controls in round forty-nine:
the Weapons deficit persists where the donor ground is close to the
model, while the right Products background itself has a substantial red
deficit. A conditional linearized calculation explains about fifteen
levels of the latter's forty-three-level card deficit. That estimate
extends beyond the tested ±8 perturbation and is not an accepted fit.
Do not assign either entire deficit to opacity or tune both stacks with
an unsupported common rose response. The next material experiment must
preserve these separate ground controls and account for overlapping
painted layers.

## Neomil: reject common coverage adjustment

DF freezes BJ's glyph geometry, ink and backgrounds and tests one common
coverage response `a^gamma`. Its predeclared domain is `[1,4]`; only the
original 4K card-2 first line trains the value, 2.1621068. Gamma 1 replays
BJ within one RGB level. Candidate alpha and every exterior pixel are
exact, and fully opaque mask pixels retain their original color.

Nevertheless, the four-line RGB L1 changes from 1,797,457 to 1,757,734 at
4K but worsens from 237,328 to 260,867 at 1600. Every 1600 line worsens.
At 1600 the R155 letter area falls from BJ's 1,440 to 669, versus 1,294
in the source. The fitted line's 4K source-core RGB error grows from
190,587 to 226,025 while exterior error falls. CLASS, O-hole and gap
losses remain. Astra reproduces 12,312 regional error/coverage checks,
the candidate formula and its exact exterior/alpha controls, and views
the source/current/BJ/candidate crops. Reject this trial; it supplies no
reason for a new local renderer or global blend change.

DG then separates repeated placement from coverage. The two source CLASS
words have R155 mask F1 0.910 at 4K and 0.667 at canonical 1600. Their
4K C/L/A/S/S column starts are `0/12/21/32/43` and `0/12/21/33/43` in
the frozen word boxes. BJ's second card reads `0/10/18/30/40`; its first
card L/A touch. Astra reproduces these exact runs and areas and views
both sizes. This supports testing internal spacing independently; the
fixed boxes alone do not identify exact glyph origins or solve contour
weight. Coverage, O holes, adjacent gaps and the full line remain
separate gates.

DH stops at its no-op gate. Merely splitting CLASS into five zero-offset
SVG `tspan` elements changes 4,503 pixels at 4K and 1,180 at 1600, with
maximum channel differences of 183 and 124. The differences extend into
the following words; alpha stays exact. Astra independently reproduces
the complete-image counts and bounds and confirms the two span insertions
are the only XML-text changes. No source origins are fitted and there is
no candidate to score. An algebraic partition of the existing raster
cannot replace a faithful glyph replay. The repeated spacing observation
stands, but a future origin experiment first needs to preserve the
original shaping, coverage and suffix exactly.

## Queue and verification boundary

The inventory confirms 29 literal unchecked boxes: seventeen source
fidelity scopes, six caller/design scopes, two original-background
provenance scopes and four live-desktop scopes. Unknown font identity is
not an external blocker. The accepted selected Entropism M and lower
Neomil A2 gap from rounds forty-six/seven must not be dispatched again.
Other broad residuals still need a concrete local acceptance question.

No Rust, SVG, golden, package, source photograph or lock changes in this
round. The CZ runtime/art checkpoint remains the tested implementation:
295 local/Nix tests, 24 SVG checks, 22 repository checks, 27 visual cases
and an exact packaged 4K Kitsch capture. Those checks are retained prior
evidence, not a new test run. This round adds measurement/prose records.

Scratch evidence: `/tmp/cp-eras-next/df-kitsch-shared-ground`,
`df-kitsch-donor-mask`, `dg-kitsch-ground-sensitivity`,
`df-neomil-coverage`, `dg-neomil-notice-shape`,
`dh-neomil-class-origins` and `dg-queue-audit`.
The root-review directory contains frozen input state, original-pixel
donor comparisons, complete mask replay, signed-RGB notice checks and
independent SVG/output sensitivity checks. Scratch evidence is local;
the conclusions and acceptance limits above are the durable record.
