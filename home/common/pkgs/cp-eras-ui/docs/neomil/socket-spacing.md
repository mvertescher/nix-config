# Store socket spacing, 2026-09-21

Source: `images/img-09-store.png`, 3840×2160. Measurements use the
1600×900 design coordinate system. All four symbols contain the same
25 occupied cells; their orientation is unchanged.

Bright connected components were measured at red thresholds 180, 210
and 230, excluding green values above 120. Each threshold finds all 25
cells in each symbol. Median component centroids reduce JPEG edge
sensitivity. Coordinates refer to pixel centers: native index plus 0.5,
divided by 2.4. The earlier audit used pixel indices without that half-pixel
offset; its first-cell coordinate differs by 0.2083 design pixels.

Independent axis fits on the first three symbols give pitches from
3.646 to 3.683px. A shared 3.6667px pitch fits them within source-edge
uncertainty. Card 4 independently confirms it (x 3.682, y 3.672). Each
symbol has a measured origin, rounded to 1/24 of a design pixel. Normal
cards share a y origin; their card-local x offsets differ. Cell size stays
3px; source ink softness and directional echoes are separate work.

| Symbol | First-cell center | Previous center RMS | Fitted center RMS |
|---|---|---|---|
| Card 1 | 452.875, 522.0833 | 0.385px | 0.098px |
| Selected card 2 | 782.9167, 712.7917 | 0.495px | 0.126px |
| Card 3 | 1109.9583, 522.0833 | 1.917px | 0.098px |
| Card 4 | 1438.2917, 522.0833 | 2.569px | 0.102px |

RMS is Euclidean center error across all 25 cells. Maximum residual is
under 0.24px. These are primary geometry measurements, not whole-crop
image similarity or a claim to have reproduced the printing material.

`store-trace.svg`, component excerpts and Iced use the same 3.6667px
spacing and origins. The SVG symbol explicitly disables an inherited 1px
stroke that enlarged 3px cells to 4px and merged neighboring corners at
the fitted pitch; Iced already draws plain filled cells. Normal-card socket geometry is kept separate from
shared statistics so each card retains its source position through hover
and press. Other selected cards reuse the existing selected-card fixture;
the source only provides selected card 2.
