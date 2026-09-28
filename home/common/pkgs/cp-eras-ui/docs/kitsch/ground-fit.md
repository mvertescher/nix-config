# Kitsch ground fit — 2026-09-27

This records the broad rose and grey-green field fit for the four source
screens. Coordinates below are in the 1600×900 trace frame; source #49
(dashboard), #50 (login), #51 (mail), and #52 (store) are 3840×2160,
so source pixels were divided by 2.4. The prior SVGs and Rust tables
used four independently guessed radial blooms, and the dashboard had
no left wash. The foreground silhouettes, ink roles, controls, and
layout are outside this fit.

## Source relationship and model

Dashboard, mail, and store share one photographic ground. Their native
patch at design (400,20)..(700,80) has the same 168.9/75.4/95.7 mean;
the lower clear patch at (800,770)..(1100,820) has the same
11.8/11.6/11.0 mean. On a 20px lattice of 10×10 design-pixel patches,
2,045 positions have mail/store medians within three levels per RGB
channel and low local variation. Dashboard differs locally at its
left margin where its foreground and echoes intrude, but its clear
upper and lower patches agree. Login's upper patch is
158.5/75.1/93.4 and its lower clean pixel field falls near 5/5/4, so
it has separate stop colors and a separate edge wash.

All four use a top rose ellipse centred at (750,-331) with radii
(1600,929). The first two rose stops are the same color because the
ellipse centre lies above the frame. Dashboard/mail/store use page
`#0c0c0b`, stops 0/.2 `#ad465f`, .4 `#ae4c60`, .6 `#8c334d`,
.8 `#16100f`, and 1 the page. Their edge wash is centred at (0,393),
radii (535,400), with colored stops at 0 `#1f211c` α1,
.4 `#202620` α.85, .7 `#15281e` α.45, and 1 `#26884f` α0.
The final transparent color only sets the interpolation into the
outer ring; it does not paint a green rim. Login uses page `#050604`,
rose stops 0/.2 `#a4455e`, .4 `#a34b5d`, .6 `#85354c`,
.8 `#1a1215`, and 1 the page. Its wash is centred at (0,434),
radii (700,433), with stops 0 `#21231f` α1, .4 `#202621` α.85,
.7 `#1f342a` α.45, and 1 `#5c8974` α0.

The SVGs use elliptical bounding-box radial gradients; Rust uses
`Prim::Lobe` with the same centres, radii, stops, and layer order in a
`Prim::Soft`. The component sheet previews the shared field. The
login remains a separate field. No noise, added glow, or foreground
color was fitted into these ground layers.

## Measurement and holdout

Patches were 24×24 native pixels centred every 20 design pixels. Each
patch contributed its channel medians, reducing the influence of
photographic grain. For the common rose fit, mail and store patch
medians had to agree within three levels and pass a low-variation and
non-ink filter. The fit used an alternating checkerboard half of those
patches; the other half was held out. Rose geometry and six color stops
were fitted over x≥350, y≤550 with a centre bound to the central-left
area and radius at least 1600. The edge wash was fitted on x≤360,
y≤840. Login used the same rose geometry, its own six stops, and its
own edge fit, filtering out visible controls by local variation and
large deviations from the common underlying photo. These filters
still admit some dark UI and soft echoes, so the score is a diagnostic
for broad ground color, not a whole-image fidelity claim.

The table compares the original and fitted **SVG renders** against
each screen's native source. Each number is channel RMS in an
alternating, held-out set of low-variation 10×10 design-pixel patches.
Upper excludes the left edge; middle excludes x<300; left samples
x<300, y80..780; footer samples y≥550. Foreground-contaminated
patches are rejected by local variation and bright-ink thresholds.

| Screen | Upper before → after | Middle before → after | Left before → after | Footer before → after |
| --- | ---: | ---: | ---: | ---: |
| Dashboard | 16.27 → 7.48 | 13.56 → 12.08 | 10.94 → 5.13 | 3.53 → 3.51 |
| Mailbox | 30.35 → 10.98 | 19.35 → 8.64 | 11.77 → 5.36 | 1.78 → 1.47 |
| Store | 26.82 → 7.27 | 14.05 → 7.75 | 10.20 → 5.35 | 2.17 → 1.36 |
| Login | 24.13 → 4.75 | 19.14 → 8.27 | 10.85 → 3.78 | 5.00 → 4.27 |

At the clear (400,20)..(700,80) patch, the shared screens now render
168.8/72.2/93.1 versus native 168.9/75.4/95.7; login renders
158.4/72.0/90.7 versus 158.5/75.1/93.4. Dashboard's grey-green
(20,300)..(70,580) patch moves from 17.7/14.6/14.5 to
29.9/33.8/28.6 versus native 27.2/31.1/26.4. Mail/store use the
same fitted wash but have slightly different local photo residues.

The remaining upper green and blue gap, dashboard middle score,
photographic grain, bloom irregularities, and glows around foreground
printing remain outside this broad field model. The patch split
establishes improvement on all four screens; it does not identify the
original material recipe.

## Native Iced check

The combined Rust build passed 257 tests. Native Iced captures from that
build were rendered at 3840×2160 for all four screens. The same clear
source patches were measured in the Iced captures and in 1600×900 SVG
renders, with each capture region scaled by 2.4. The medians match
within one RGB level at every sampled upper, middle, and footer patch;
the login left edge reaches two levels in one patch. This checks that
the Rust `Prim::Soft` field realizes the SVG geometry and stop colors.

| Screen | Upper held-out source RMS: SVG / Iced | Left held-out source RMS: SVG / Iced | Iced–SVG maximum channel gap in sampled clear patches |
| --- | ---: | ---: | ---: |
| Dashboard | 7.48 / 7.48 | 5.13 / 5.17 | 1 |
| Mailbox | 10.98 / 10.89 | 5.36 / 5.40 | 1 |
| Store | 7.28 / 7.29 | 5.36 / 5.41 | 1 |
| Login | 4.75 / 4.89 | 3.78 / 3.79 | 2 |

For example, dashboard/mail/store at (800,50) measure source
169/71/93, SVG 172/75/95, and Iced 172/75/95; at (40,400),
dashboard measures source 28/33/28, SVG 31/33/28, and Iced
31/33/28. Login at (500,50) measures source 158/75/93, SVG
158/71/90, and Iced 157/71/90. Thus the remaining color gaps in
those places are properties of the broad fit, not a Rust/SVG rendering
disagreement.
