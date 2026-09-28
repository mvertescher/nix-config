# Store card lower corners

Source: `images/kitsch-store.png` (#52), 3840×2160 native, measured at
2.4 source pixels per 1600×900 design pixel. The right foot of plain card 3
is centered near (1385, 539); the selected card 2 lower body ends near
(1065.5, 680). Native right-edge profiles bend away from the vertical about
10–11 design pixels above those bottom rules. The left feet show the same
rounding, though the first socket's QR cells interfere with automatic edge
extraction there. This supports a **10.5px corner span** on both lower feet
of each contour. The trace and Rust use quadratic corners with control
points at the square's outer vertex; this span is not an exact circle
radius, and the source's blur prevents a more exact curve claim.

The plain trace keeps its r6 upper-left corner and 24px upper-right chamfer.
Its lower baseline moves down 0.5 design px, from local y320.5 to y321 in
the SVG, matching the source's bright bottom rule around screen y539. The
three socket dividers extend to the new rule. The Rust contour likewise
moves from local y320 to y320.5; its centerline convention was already
0.5px above the SVG's. The selected lower outline stays at local y462 in
Rust and y461.5 in SVG; that preexisting half-pixel difference remains.
The selected upper amber step and all card contents are unchanged.

For an edge comparison, the source and both SVG versions were rasterized
or read at 3840×2160. At 18 rows per foot, from 9 to 1.5 design pixels
above the bottom, the brightest colored edge in a 15px right-foot window
was compared in design pixels. The plain-card mean absolute horizontal
error fell from **0.93px to 0.67px** (worst 3.33 to 1.25px). The selected
card fell from **1.13px to 0.42px** (worst 3.75 to 0.83px). These are
edge-position comparisons, not color or glow scores. Native source,
previous SVG and revised SVG crops are saved at
`/tmp/cp-eras-round5/kitsch-corners-{normal,selected}-{left,right}.png`.

`CARD_EDGE` supplies normal, hover and held faces and the idle hover ghost.
The selected lower outline supplies both selected faces, while its lifted
ghost follows the unchanged selected upper slab. The fourth card retains
its opaque slice and five fading slices through x1550; its visible foot is
on the left side of the viewport. Its selected and feedback states follow
the same inferred contour as the source-supported plain/selected card 2
shapes. The selected card 2 gun translation (1.5, −13.7), 86/30/5/5
figures, `Sperad`, and selection-aware hit rectangles are unchanged.

The source's colored halo is broader and less even than the one-stroke SVG
or Iced contour. Corner curvature is measured only on the visible default
shelf: alternate selection, hover and held faces are design continuations.
