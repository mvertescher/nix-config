# Kitsch store weapon and socket trace

The reference is `images/kitsch-store.png` (3840 × 2160, 2.4 native pixels per 1600 × 900 scene unit). This page documents reconstruction of visible pixels, not the original author's asset files. The same MAGNUM illustration appears in Entropism and NeoKitsch; source-only silhouette alignment against the Entropism plain-card crop gives Kitsch IoU 0.953 at scale 1.00 and interior luminance correlation 0.970. The Kitsch plain card starts at (484,218), with the weapon at local x37–236/y129–183. The shared `magnum_art` paths are translated by (−2,+26) scene units. The selected card starts at (804,218), with its own weapon at local x38.5–237/y115–169. Its photographed inverse art is traced separately because the seam mask is not just a recolor of the plain art.

The plain source mask is mint (G>120, B>100, G>R+30) in native x1225–1760/y790–1005. Shared source-native body and bright-metal paths give 0.911 source/SVG mask IoU at the measured transform, with 0.994 recall but 0.916 precision: Kitsch prints sharper dark seams than the Entropism sample. A Kitsch-only subtraction mask is restricted to pixels inside that projected shared body whose source G and B are each <120. Its 244 rings / 1,352 vertices are simplified at 0.6 native pixels after dropping components/holes under 3 native pixels. The subtraction is filled in the source dark `#0e0e0d`. Plain mint-mask IoU rises to **0.968**, precision 0.974, recall 0.994. The base and bright tiers are source mid mint `#79cdb8` and bright mint `#93ffe4`; pressed and lifted variants recolor the same geometry with their existing card material inks.

The selected source mask is dark (R<120/G<100/B<80) in native x1990–2535/y750–970. Its outer and core contours have 199 rings / 2,792 vertices and 183 rings / 2,544 vertices. Components/holes below 3 native pixels are removed, rings below area 3 omitted, and boundaries simplified at 0.6 native pixels. The two dark inks `#3a2408` and `#302010` leave the selected amber fill visible in rail holes, receiver channels and the butt slot. Native selected source/SVG dark-mask IoU is **0.988** (precision 0.999, recall 0.989). These SVG scores derive from the thresholds used to draw the paths; a native Iced capture is still needed. No bitmap is embedded, and the large shared raw path arrays are not duplicated in Kitsch.

The card-1 socket glyph at native x1185/y1201 has 25 cells on a 9×9 occupancy grid, 3.49 scene-unit pitch and 3.8-unit cells. It repeats the same scatter as Entropism's MAGNUM cards. The old 14-cell grid's native source/SVG mint-mask IoU was 0.239 in x1183–1265/y1198–1279. The corrected grid at card-local x9.6/y282.5 reaches **0.873**, with selected sockets at the same local x and y422.5. The certification and warning marks in the yellow shelf band already had detailed SVG geometry; Rust now carries their corner ticks, two micro-label strokes, SC knockout, inner C, triangle point and seven warning rules. The source mark crop is still photographic, so their tiny letterforms and seven rules should be judged as approximate vector readings.

The shelf/card outlines, fourth-card residue and hit viewport, card typography, selected growth, ghost steps, and material timing remain as previously traced. The component sheet copies the new rifle and socket definitions with `k-` ids. Its main card and selected example use the same source-grounded paths as the parent trace. Source grain, glow and sub-native-pixel engraving remain material residuals.

## M certification pass

The source band at card-local x−17..92/y74..92 has a heavy frame around RG5, a dark square around a light disc marked SC, a double angular C, and a hollow warning triangle. The previous SVG/runtime had thin corner ticks, a bare dark disc and a solid triangle. The source crop is `images/kitsch-store.png` native x1094..1392/y696..744; `/tmp/k-band-source-large.png` is a temporary 3× inspection crop. The four marks now use the same measured paths in the parent SVG, component sheet and runtime. Knockouts follow the band's normal yellow or selected amber fill. The warning micro-print uses 97 row contours from the source's dark pixels, sampled in 2-native-pixel cells after excluding isolated grain. This retains the source's illegible print pattern without inventing words; its exact photographic softness remains unresolved. The refreshed SVG crop is `/tmp/k-band-m-crop.png`. Native M review is pending.

Native M review resolves the inset certification mark as three bars on
a vertical stem, with angular upper/lower tips, rather than a second small
C. Its native stem is near x1203, and bars near y717/725/734; the new inset
uses a 0.65 design-pixel stroke in both SVG and Rust. This also removes the
M mismatch between SVG 1.3 and Rust 0.8. Native confirmation is pending.

## Integrated validation — 2026-09-29

N native review accepts the corrected full-height inset three-bar mark and the broader certification silhouettes. The source/SVG/Iced crop confirms placement and continuity; exact tiny RG5/SC glyphs, coarse warning microprint and photographic softness remain outside that closure. All 22 repository checks pass.

## SC corner apertures — 2026-09-29

The original SC mark also has four small light apertures inside the dark square, outside the central disc. The prior square/disc path omitted them. Four compact triangular contours were fitted to card 1's source pixels and checked without moving them against selected card 2 and ordinary cards 3–4. The traced SVG uses even-odd cutouts; runtime paints the same shapes in the disc's band ink, so ordinary yellow and selected/held feedback retain their existing color mapping. The square, disc, SC letters, and adjacent certification art stay in place.

At 3840 × 2160, RGB absolute error across fixed corner patches falls by 17.1% on card 1, 8.7% on selected card 2, 15.0% on card 3, and 13.9% on card 4. The SVG trial changes 178 pixels, all inside the four corner patches on each card; its central disc and SC letter region are pixel-identical. Source/current/trial native-pixel and enlarged crops, exact bounds, and conservative false-cutout counts are in `/tmp/cp-eras-resume-20260929/ab-k8-fit/findings.md`. The lower-right aperture on card 4 is the weakest holdout because its photographed opening is softer and closer to the square edge. Exact photographic softness and the warning microprint's unreadable contours remain unresolved; fresh AB native captures now verify the geometry. Fifteen of sixteen
native corner patches improve, as does every card's combined score; the
fourth card's lower-right patch worsens20.06→22.63. Fractional held,
selected-last, custom-color and opening captures retain the band/knockout
relationship and clipping. Both gates and the full AB check pass; see the integrated acceptance below.

### AB integrated acceptance

The source/native/state review is integrated: all 286 Rust tests and 22
repository checks pass, including 27 exact visual cases on their first
attempt. All 199 frozen file hashes match the Nix source. This closes the
bounded AB correction above; its stated photographic/glyph limits remain.
