# Reference SVG round seventy — ordinary Store top coverage

The first and third ordinary Neomil Store tops use a thinner continuous
outline, with the same inferred geometry on the second ordinary card when
another card is selected. This is a bounded top-edge correction. Other
ordinary edges, the fourth top and faint echoes remain open.

## Structural choices and source comparison

A direct top/body split changes corner joins at the original width. Keeping
two-unit stubs restores native pixels, including sixteen resting/state
cases and three custom-palette pairs, but a thinner middle rule then creates
unsupported thick blocks at its endpoints. Visual review rejects that
candidate despite better whole-region scores.

The accepted construction uses one filled outline with an interior hole.
Each vertex is the intersection of adjacent centerlines offset by their
half-widths. Outer and reversed inner contours meet continuously, with no
short caps, arbitrary taper or moved centerline. Geometry is calculated
before compilation and stored as path constants. The original face path,
fill, material mask and bright bar remain intact; the ring paints before
the bar. The one-face underlay and twelve primitive slots are preserved.
Held feedback hides ring ink while retaining its geometry; hover preserves
the source face's stroke setting, so it cannot restore the old thick border.

A uniform-width 1.2 ring first reproduces native pixels exactly at all three
resting sizes and five relevant state controls. One frozen width of 0.6 is
then applied to the top, keeping other sides at 1.2. Repeated source profiles
support thinning but do not identify a unique physical width or blur. No
width, opacity or phase sweep is used.

Fixed regions and uniformly sampled source images are inherited from round
sixty-nine. All eighteen top/corner region-size comparisons improve both
RGB L1 and RMS in SVG and native. The table measures the union of the two
whole top regions, without counting overlaps twice.

| Viewport | SVG RGB RMS, before → after | Native RGB RMS, before → after |
| --- | ---: | ---: |
| 3840×2160 | 17.532 → 11.837 | 21.630 → 12.658 |
| 1600×900 | 16.321 → 10.285 | 16.482 → 11.180 |
| 1537×947 | 16.546 → 10.501 | 20.920 → 11.424 |

Native changes 3855 / 536 / 1033 pixels, all in the first/third top and
corner regions. Other frame edges, selected art, contents, fourth card and
alpha remain exact. Astra reviews source/SVG/native joins at all three sizes.

SVG stroke-to-fill rasterization is not a pixel no-op: at unchanged width
it changes 1566 / 1517 / 1510 pixels, with maximum channel differences
6 / 16 / 5. The thinner candidate retains nine small L1/RMS losses along
other boundaries, including the first-card fan/lower stem and a fractional
bottom corner. The largest relative L1 loss is 0.470% in the fractional
first lower stem. These are explicit tradeoffs; they do not establish full
edge fidelity or justify changing fonts or global color blending.

## Runtime and verification

No new effect layer, raster cache or runtime corner calculation is added.
A filled ring replaces the ordinary stroke using existing primitives. It
adds one path assembly per full ordinary card; this round makes no timing
claim. Fonts, text caches, shared renderer and original images remain intact.
The component uses the same source SVG ring.

Each native variant passes the fifteen existing Store interaction tests.
Eight reference scene cases per variant cover three resting sizes plus
selection, ordinary hover/held, fourth selected/held and early opening.
Three custom-palette cases per variant and disabled-probe parity checks
preserve semantic inks and held suppression. All original-width controls
reproduce the accepted pixels. Thinner-width changes stay in the expected
ordinary top/corner regions. State captures use synthetic Pointer snapshots
through production drawing and Store selection logic.

Both Store gates pass, with G2i using the newly built package. All 22
repository checks and 28 matrix cases pass on the first attempt; packaged
tests pass 260 library and 45 other tests (305 total). Direct pixel review
finds 26 of 27 visual cases exact, with the existing one-pixel, one-level
Neo-kitsch bar difference unchanged. The other case checks source sampling.
All 352 frozen tracked crate files match the tested Nix source. Fresh
packaged Store captures are byte/RGBA-exact to the reviewed candidate at
all three sizes. The final six documentation records are the only changes
after verification; runtime, artwork, tests, fonts and originals still
match that tested source.

This completes the bounded first/third top correction, including inferred
ordinary-second geometry. The wider primary-coverage task remains open,
with the fourth top, other thin edges and faint printing still to address.
There are still 30 open scopes. Changes remain staged and uncommitted.

Evidence: `/tmp/cp-eras-next/fq-neomil-top-noop/` through
`gi-neomil-ring-final-controls/`; GA/GB retain geometric construction,
GG contains Astra's source scores and explicit SVG losses, and GH records
integration and repository/package verification.
