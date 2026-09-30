# STORE primary brand and rifle art

The source is `images/img-09-store.png` at 3840×2160. The local geometry uses the 1600×900 design grid (2.4 native pixels per design unit). `src/eras/neomil_store_art.rs`, `store-trace.svg`, and the STORE excerpts in `components.svg` use the same native-source contour samples. They are editable vector paths, not embedded photo pixels.

The top-left MASURAO artwork occupies approximately x145..320, y62..135 in design coordinates. A bright mask from the native crop (red >180 and red >2.5×green) supports the slant bar, kanji, italic wordmark, band hatching and short horizontal echoes as 272 closed rings, simplified within 0.75 native pixel. The former font/skew approximation's bright-mask IoU was 0.45; this contour's is 0.946 on the measured crop. One semantic `Fg` layer keeps the brand recolorable under custom palettes. The native source's lower/dimmer color echoes remain unresolved.

The rifle drawings are separate source crops: ordinary card x466..705/y289..361, selected card x785..1035/y289..361. Each uses five nested native red masks at thresholds 60, 90, 130, 180 and 220, requiring red >2.5×green, simplified within one native pixel. The layers map to the existing low fixed tone, `Border`, `Dim`, `Mid`, and `Fg`; the old card feedback derivation still recolors semantic tones for hover and hold. Selected geometry retains its 14-unit left offset. Repeated ordinary cards use the same source-supported silhouette; the fourth is clipped by its existing viewport. No new marks were invented.

The held-out native mask comparison below uses red thresholds 90, 130, 180 and 220 respectively. Values are intersection over union with the source crop; these are mask checks, not perceptual or runtime acceptance:

| Art | Old SVG | Source-traced SVG |
| --- | --- | --- |
| Ordinary rifle | .640 / .421 / .233 / .193 | .902 / .883 / .803 / .746 |
| Selected rifle | .705 / .629 / .486 / .410 | .952 / .950 / .918 / .912 |

The candidate source/vector review is `/tmp/cp-eras-completion/neomil-store-art-preview.png`; the brand comparison is `/tmp/cp-eras-completion/neomil-store-logo-vector-review.png`. J's 265 Rust tests covered the preceding printing implementation. A native build and source/SVG/Iced crop review are still needed for the new vector art, especially the exact palette tone order and small anti-aliased machining details.

The three certification marks at every card head and foot are source-supported RG5, SC, and the double-line E/C symbol. The former schematic square/circle/C template had native bright-mask IoU .176 on the selected header x775..848/y159..185. The new primary contour uses the source's red >180 and red >2.5×green mask with 0.5-native-pixel simplification: 26 closed rings and 366 vertices. The source-matched SVG raster has the same 1,908 bright pixels in the construction crop. **That 1.0 mask score verifies contour transfer, not independent visual acceptance.** The dim RG5 lettering is a second path: native rows in x782.5..791.25/y166.5..170 are normalized by their 15th-percentile red to remove scan bands; a residual >20 gives three recognizable letter contours (91 pixels, 55 vertices). Letter pixels have median red 141; semantic `Dim` preserves custom-palette recoloring and is close to that tone. With both tiers, red MAE in the RG5 letter crop falls from 37.17 to 20.23. The card hover and held variants recolor both direct path leaves through the existing feedback derivation. Fine horizontal echoes and the varying dim normal-card fields remain open.

The top KIROSHI/chip mark uses a separate primary source contour from x746..838/y26..52 (13 rings, 279 vertices). It includes the red square, black numeral hole and source letter shapes; the former schematic bright-mask IoU was .334 overall and zero on the word. The traced SVG exactly reproduces its 1,238 construction-mask pixels. The code run and left arrow next to it remain separate. Source/prototype review crops are `/tmp/cp-eras-completion/store-rg5-prototype-review.png` and `/tmp/cp-eras-completion/store-marks-{cert-head,cert-foot,kiroshi}.png`. Native runtime review is still required for the whole symbol and repeated cards.

## Integrated validation — 2026-09-29

N integrated validation confirms the previously reviewed primary MASURAO/rifle paths and L certification/KIROSHI contours. Selected certification/KIROSHI source/native mask overlap is .984/.990; held and cropped states retain their two-tone geometry and contrast. All 22 repository checks pass. Repeated dim fields and photographic printing echoes remain explicit limits.
