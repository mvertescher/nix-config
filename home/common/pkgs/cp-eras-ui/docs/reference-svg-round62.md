# Reference SVG round sixty-two — ordinary stock band

EI adds the missing lower segmented band to the ordinary Entropism rifle
in the source trace and component sheet. It uses one filled vector shape;
Rust, fonts, caches and goldens remain unchanged. Native ink needs the
coordinated review identified in [round sixty-one](reference-svg-round61.md).

## Source and corrected geometry

The fixed original-pixel band box is card 2 x2390–2422/y975–999, with
card 3 shifted 773 pixels. Source RGB differs by only .339 levels per
channel between those copies; 504 of 768 pixels match exactly. The dark
slot differs by .244 levels per channel. This supports one repeated
structural band rather than a per-card pixel correction. Astra reproduces
all three uniform whole-source projections and these comparisons.

One manually specified sweep and two returns use the accepted seam's
.8 design-pixel width. Seven predeclared card-2 source samples choose one
median ink, #8bac89. Card 3 is not used to fit ink. The first trial improves
the whole band at three sizes, but its left entry crosses a bright source
patch: the fixed 4K entry box loses 347/551 RGB L1 units in cards 2/3.
Astra catches this in source-support boxes recorded before the trial.

One correction moves the entry and its shared divider junction to the
observed source centers, retaining width, ink, second divider and terminal.
In original card-2 pixel coordinates, the accepted lower sweep is
`M2395.5,977.5 L2401,983 Q2405.5,987 2412,988.7 L2420.5,991`;
the first return is `M2401,983 L2404,980.2 L2406,977.8`; the second is
`M2410.5,988.2 L2412,983.4`. Division by 2.4 followed by subtraction of
card origin (783,260) gives design coordinates. Expanded strokes are
united and intersected offline with the existing bright mask. The final
78-vertex polygon adds no raster, filter or runtime clipping mechanism.

## Source review and controls

Whole-band RGB L1 against the original improves:

| Viewport | Card 2 | Card 3 |
| --- | ---: | ---: |
| 3840×2160 | 21157→18632 | 20984→18797 |
| 1600×900 | 3622→2847 | 4304→3559 |
| 1537×947 | 3736→3085 | 3713→3068 |

All 24 entry/return/sweep comparisons improve across both cards and all
three sizes. The corrected 4K left entry improves 2882→2144 and
2877→2306. The card-3 4K right return is nearly neutral, 1507→1497:
its 668 units of gain and 658 of loss remain explicit. The reference
line is still more uniform than the photograph, and source pixel
(2397,980) remains too dark, G221→182. These residuals do not disappear
inside an aggregate score or imply exact photographic printing.

All 229/64/57 changed pixels remain inside the two fixed band boxes.
Neighbors, dark slot cores, alpha and the upper diagonal control are exact.
The primary masks, accepted diagonal path, selected inverse art and card-4
clip are unchanged. Whole-stock gains equal band gains because the rest
of each stock stays intact. No native fit or interaction-state claim is
made for this SVG correction.

## Verification

Astra independently reproduces both candidates and checks thirty fixed
regional measurements plus thirty source-support regions per trial.
The ink samples, median choice and XML preservation also reproduce.
Production SVG renders match the reviewed corrected candidate at all
three sizes. The component retains an exact ordinary-card copy of its
parent after insignificant whitespace normalization; its full and half-size
renders change only 32 and 14 band pixels, with alpha unchanged. All 24
SVG structural checks and both Entropism Store fidelity gates pass.

The full repository check passes all 22 checks and 28 matrix cases on
the first attempt. All matrix results are cached against the unchanged
EF package; 343 frozen crate files match the Nix source. Its recorded
build log verifies 303 passing Rust tests, which were not rerun for this
reference-only change. Of 27 visual cases, 26 remain RGBA exact and the
existing one-pixel, one-level NeoKitsch bar residual is unchanged.

Evidence is under `/tmp/cp-eras-next/ei-entropism-band/` and
`/tmp/cp-eras-next/ei-root-review/`, including both frozen trials and root
verification records. EF remains the verified runtime checkpoint; EH and
EI change only references and documentation, with the package unchanged.
All 29 broader scopes remain open. Changes stay staged and uncommitted.
