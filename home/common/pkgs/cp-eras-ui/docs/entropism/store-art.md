# Entropism store artwork and lettering

The reference is `images/entropism-dashboard.png`, the 3840 × 2160 store image. The filename is swapped with the hub image; `docs/sources.md` records the Behance id. Measurements below use the 1600 × 900 scene grid (native coordinates ÷ 2.4). This is a reconstruction of visible pixels, not a claim about the original author's vector files or texture pipeline.

## Weapon

The same MAGNUM illustration repeats in all four cards. Card 2, at scene origin (783,260), is the cleanest plain specimen. Its drawn region is local x30–244, y95–170, while the high-confidence silhouette lies roughly x36–241, y101–159. The old four filled blocks occupied x36–242 but erased the trigger opening, the triangular stock opening, the narrow butt, the top rail and the stepped receiver edge. Card 1 reverses the weapon to dark ink on the solid sage header. In the selected source its rail and receiver are 14 native pixels (5.833 scene units) left of the plain artwork's local origin, with no vertical shift; the selected SVG group and Iced paths apply that measured offset. The fourth card remains clipped at scene x1564.6 by its persistent viewport.

The final weapon uses two nested paths measured from the **native 3840 × 2160 pixels**, 2.4 native pixels per scene unit. Card 2 is cropped at native x1950–2470, y850–1038. The mid-body `#rifle` mask keeps pixels with green >80; the bright-metal `#rifle-detail` mask keeps green >160. Both also require green >1.06 × red. Isolated components and holes below 3 native pixels are removed; boundary rings below 5 native pixels are omitted. Ramer–Douglas–Peucker tolerance is 0.6 native pixel (0.25 scene pixel). The paths retain 79 rings/1,746 vertices and 143 rings/2,830 vertices respectively, at card-local coordinates rounded to .001 scene unit. There is no embedded bitmap or hand-drawn interior seam path. Iced uses Mid/Select on plain cards. The selected art has its own native dark masks, colored Dim/OnSelect, preserving custom palette retinting.

Earlier passes sampled one scene pixel and applied a 2-pixel median. They retained the broad body but lost the long rail holes, receiver fasteners, front channels and grip folds. Hand-read strokes then spilled outside the silhouette. Those data were retired after native SVG/Iced review. The native masks recover the recurring hardware features visible in the source; they still omit photographic grain and subpixel glints.

Within native x1951–2468, y852–1035, the plain-card source/SVG green-mask IoU is **0.986 at green >80** and **0.969 at green >160**. These values measure reproduction of the thresholded source geometry, which the paths were derived from; they do not rate the visual material or selected-card ink. The prior F SVG was 0.858 at >80, and its native Iced plain-card mask was 0.847 at green >90. In the G and H native Iced reviews, the plain source/runtime mask IoU is **0.931** with best alignment at (0,0) native pixels. The selected G review found its best horizontal alignment at −14 native pixels; correlation increased from 0.512 to 0.670 at that offset. H applies the offset and its selected placement is accepted in native source/SVG/Iced crops.

The H selected gun still had more dark area than the source. In native x1175–1680, y860–1010, using source dark R<95/G<125 and Iced dark R<105/G<115, the H dark-mask IoU is **0.779**: 25,265 source pixels versus 32,121 runtime pixels, with 0.995 recall but 0.782 precision. The same overfill occurs at the upper rail (IoU 0.771) and receiver (0.767). In these regions the source's dominant dark body is RGB (33,28,15), while the darkest runtime tier is (31,42,28); source light rail/seam pixels center near (165,207,155), close to the runtime sage field (166,211,167). The bright seams are absent chiefly because the plain-card nested mask is reused as a dark selected tier, filling some source-light channels. The I pass traces the selected card directly at source native resolution: outer dark mask R<95/G<125/B<90 and core R/G/B<60, with components and holes below 3 native pixels removed, selected outer rings of area at least 3 native pixels retained, and 0.6 native-pixel RDP. It retains 192 rings/3,152 vertices and 156 rings/2,430 vertices. Static source/SVG dark-mask IoU improves from H 0.779 to **0.989** overall, from 0.771 to **0.988** along the upper rail and from 0.767 to **0.990** at the receiver. The stock region x1500–1690 reaches 0.986 IoU; its thin rightmost edge x1640–1690 retains 280 of 312 source-dark pixels after noise removal (0.891 IoU). The subsequent J native Iced review found source and runtime selected dark bounds both x1186–1663/y872–1000, with dark-mask IoU **0.9665** in the full x1150–1740/y835–1060 crop; the SVG reaches **0.9909** under the same threshold. The selected silhouette and light seams now follow a separately measured source mask rather than a recolored plain-card mask. The residual is thin-edge coverage and source texture, especially at the rightmost stock contour.

## Logotype and printing

The store logo occupies x137–308, y101–157. The 4 follows the measured x137/y133 diagonal corner, y132–145 crossbar and descender to y152; its counter is triangular. The S follows source row spans with curved lobes, a narrow y136–141 waist and a lower terminal ending at y153. The T remains outline-only as in the source. The logo's green mask IoU in x137–265, y100–159 rises from **0.707 to 0.907** after the 4 and S correction (0.779 for the first hand fit). The lower `S T O R E` run is unchanged.

The selected card's stat labels and values keep their measured baselines, and its detail rows retain the source spelling “Sperad.” The detail text now uses Rajdhani Medium at 19.75px. The socket labels use Medium 13px instead of 12px. Compliance text is Medium 8.5px, with its first baseline moved down 2px and both lines widened locally. Pinned librsvg ignores `textLength`, so each SVG run uses an explicit horizontal transform about local x5. At a green threshold of 65, the first compliance run changes from x466–681 to **x466–719**, matching the source x466–719; the second is **x466–630**, matching source x466–630. The SVG's remaining vertical edge differs by about 1px. The socket-row glyph, three dividers and number of sockets are unchanged.

Native Iced captures through F exposed the first pass's out-of-body lines and the second pass's missing internal features. They are historical evidence; the G native review shows the recurring hardware features retained. The accepted logo correction and the text fits remain unchanged. The first Iced compliance run reaches native x1119–1727 against source x1119–1726; the second was 6 native pixels too wide and its Rust stretch was reduced from 1.04 to 1.025. Detail and socket lettering occupy the same measured rows, though source glow and grain are absent in Iced.

The store vector and text edits are confined to the store trace, its copied store specimens in the component sheet and the store section of the Iced table. A separate login footer Plate in that same Rust file now uses the source `#8aac8c` reference fill for the exact built-in or published palette while retaining semantic Select for custom palettes. The ground field, card placement, hit regions, selection timing and fourth-card cut retain their existing geometry. The source contains fine line texture and mild glow around text; those residuals belong to the later material pass rather than the geometry traced here.

## M local lettering adjustment

The M source/native comparison found the four plain and selected DPS/PNT/ACC/ROF labels about 4–8 native pixels low and 4 native pixels too tall. Their local size/baseline now changes from 20/200 to 17.5/196.7 in Rust, source trace and component excerpt. In the grown card's first socket cell, EMPTY was source native x2041..2118/y1228..1247 against J native x2044..2123/y1224..1243; SOCKET was x2033..2127/y1261..1279 against x2037..2130/y1258..1277. The three plain and three grown pairs shift 1.5 scene pixels left and 1.5 down, retaining their size and ink. The supplied comparison is a local estimate; native M capture must confirm the new bounds.

Native M review confirms EMPTY within one source pixel and the stat cap
height within one pixel. SOCKET remains 1–2 native pixels low, so its Rust
baselines alone move up 0.5 design units in the next checkpoint. The source
SVG remains fixed; EMPTY and all socket horizontal positions are preserved.

## Remaining printing review — 2026-09-29

Source/SVG/native review confirms the accepted rifle, stat-label, socket and
compliance layout. The first compliance line ends at native x1726 in the
source and x1727 in Iced. Existing selected and ordinary rifle masks have
source/native IoU .9665 and .931 respectively. The first broad crop did not
isolate a new rifle or compliance feature from grain, faint seams, glyph
contours and bright edge spread; a global scale or silhouette redraw remains
unsupported.

A separate value-only comparison then isolated a repeatable type defect:
in the SVG, `86 / 30 / 5 / 5` was six native pixels short at the cap and two
low at the baseline. The local size, baseline, weight and two-digit width fit is in
[store typography](store-typography.md), with selected and ordinary holdouts.
This does not revise the art or claim exact photographed printing. Compliance
font/material and fine glyph edges remain open.
