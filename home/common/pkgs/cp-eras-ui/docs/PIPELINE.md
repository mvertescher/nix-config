# Design pipeline: original image → SVG → iced

The crate's render chain is a three-stage pipeline, each stage the
verifiable input to the next:

```
original image       SVG reference          Rust iced implementation
   images/             docs/<era>/*.svg         src/
   (gitignored)        (committed)              screens/, widgets/, eras/, bar.rs
        │                    │                        │
        └─── G1 ─────────────┘                        │
             trace is                      ┌───────────┘
             faithful to the               │
             material                      └─── G2 ────
                    schematic and its implementation agree
```

## Stages

1. **Original image** — the Behance source (gallery `Cyberpunk 2077 User
   Interface (Part 1)`, `scripts/download_images.py`; the per-era runs are
   documented in `docs/<era>/README.md`). Lives in `images/`, which is
   gitignored: the artwork is not ours to redistribute. This is the source
   of truth for *shapes* and *palette*; nothing downstream may invent
   geometry the material does not show.
2. **SVG reference** — a hand-traced schematic that commits the sampled
   palette and the source's geometry to the repo. One per surface:
   `bar.svg`, `dashboard.svg`, `target-app.svg`, `target-components.svg`.
   This is what a render is compared against, by eye or by script.
3. **iced implementation** — `src/`. Screens, widgets and era tables
   compose from `Style`; the golden matrix renders them headless and locks
   the renders.

The bar is the one surface with no source: no Behance screen shows a
status bar, so `bar.svg` and `bar.rs` are original compositions wearing
each era's palette. `docs/sources.md` records which SVG traces which
source, and which are originals.

## Verification gates

- **G1 — source → svg.** Structural fidelity of the trace:
  `scripts/compare_ref.py images/<src>.png <svg render> --regions`.
  Target: layout correlation well above the "unrelated scenes" baseline
  (~0.15), with the per-region and overlay outputs inspected by eye.
- **G2 — svg → iced.** The implementation matches the reference:
  rasterise the SVG (`rsvg-convert`) and
  `scripts/compare_ref.py <svg render> tests/golden/<screen>-<era>-...png`.
  The golden matrix already locks iced-vs-iced (G0, byte-identical); this
  gate locks iced-vs-reference. A schematic is expected to reach
  ~0.65+ layout / 0.85+ palette against its own golden; text rasterization
  and the hand-drawn gap keep edge correlation below the perfect-1.0 that
  G0 holds.
- Run either with `scripts/fidelity_check.sh [era [screen]]`.

## Iteration loop

```
rsvg-convert docs/<era>/<screen>.svg (or nix shell nixpkgs#librsvg ...)
compare_ref.py <source-or-golden> <svg render> --out /tmp/fid
inspect side-by-side.png / checker.png / edges.png / heatmap.png
edit the SVG  →  repeat until the geometry reads right
```

The SVG is the target; the app follows. Never fix the SVG by editing the
app's render, and never touch the goldens except when the iced
implementation genuinely changes (the procedure in `tests/bar.nix`).

## Direction of change

Any mismatch found by these gates is owned by the earlier stage: if G2
fails, the SVG or the implementation is wrong (fix the earlier one); if
G1 fails, the trace is wrong (fix the SVG). The material always wins.