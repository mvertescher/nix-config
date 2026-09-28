# Store card hit regions

The four shelf cards start at y218. Their idle hit rectangles remain 261×320, while the selected face grows through the lower detail panel and compliance text near local y489. Each card now selects a 261×500 hit rectangle through `Prim::Pick`. The face drawing and pointer bounds use the same card index, with one `Prim::Plate` in each selection branch.

The fourth card keeps its permanent viewport, from local x−32 through x107 (global x1550). Both selected and idle hit rectangles are inside that viewport. The foreground's opaque slice and five fading strips are unchanged. Selection-aware keyboard centers are computed after intersection with the viewport; for selected card4 the center is (1496.5, 468).

The store regression checks the selected lower body at y650, the footer at y705, unselected lower misses, and the cropped fourth-card margin at 0.75, 1, 1.25 and 2.4 scale. It checks every selected card and the visible keyboard center. Shared scene pointer tests own release cancellation after a pointer leaves the selected region. The source photo specifies the second card's selected face; the other selected states follow the store's inferred interaction behavior.
