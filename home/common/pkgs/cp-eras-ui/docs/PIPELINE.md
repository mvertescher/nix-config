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
   palette and the source's geometry to the repo. One trace per sourced
   screen — `login-trace.svg`, `dashboard-trace.svg`, `mailbox-trace.svg`,
   `store-trace.svg`, sixteen in all — plus the app-shaped `dashboard.svg`
   composite that G2 compares against the golden, and `bar.svg`.
   `target-app.svg` and `target-components.svg` predate the traces and
   are superseded where a `store-trace.svg` exists. This is what a render
   is compared against, by eye or by script.
3. **iced implementation** — `src/`. Screens, widgets and era tables
   compose from `Style`; the golden matrix renders them headless and locks
   the renders.

The bar is the one surface with no source: no Behance screen shows a
status bar, so `bar.svg` and `bar.rs` are original compositions wearing
each era's palette. `docs/sources.md` records which SVG traces which
source, and which are originals.

## Verification gates

- **G1i — source → svg, measured.** *The gate that matters, and the only
  one here with a pass/fail.* `scripts/extract_spec.py` measures an image
  into a spec — palette by k-means, connected components split by
  nearest-peak on the distance transform, each fitted against rect /
  diamond / chamfered-rect / rule templates, plus a per-ink-family 80x45
  occupancy grid — and `scripts/spec_diff.py` compares two specs. Run it
  as `scripts/fidelity_check.sh --inventory <era> <screen>`. Two verdict
  modes, picked per era in that script:
    - **shapes** (neomil, entropism): match the shape inventories piece
      by piece. Fails when the candidate draws none of a class the
      source has, or matches under 60% of the source's shape area.
      Right for axis-aligned design languages, where the templates fit
      whole widgets.
    - **inks** (kitsch, neokitsch): rotated, overlapping, translucent
      geometry — fan blades, onion cascades — fragments differently on
      a photo and on a clean render, so fragment identity is not a
      stable invariant there; where each colour sits on the canvas is.
      Families are paired by colour and their occupancy grids compared
      by IoU; fails under a weighted IoU of 0.45 or when a major source
      family has no counterpart. Calibrated: faithful traces score
      ~0.5, the known-wrong app composites 0.03–0.07.

- **G1 — source → svg.** Grid statistics:
  `scripts/compare_ref.py images/<src>.png <svg render> --regions`.
  Directional only, and *not sufficient*: a trace that drew three chart
  cards for a photo holding a six-diamond menu scored 0.560 layout here,
  against a "unrelated scenes" baseline of ~0.15. Mass in roughly the
  right place is not the same as drawing the right things. Read it
  alongside G1i and the overlays, never on its own.
- **G2i — svg → iced, measured.** *The gate an SVG→iced conversion
  iterates against, and the second one here with a pass/fail.* What G1i
  is to G1: the design SVG and a live headless capture of the matching
  binary, put through `extract_spec.py` and diffed by `spec_diff.py`
  design→implementation. Run it as
  `scripts/fidelity_check.sh --implementation <era> [screen]`, with
  `--bin-dir DIR` to name where the binaries are (default `target/debug`).
  Unlike G1i this uses the **shapes** verdict for every era, kitsch and
  neokitsch included: the `inks` exception there is about the *photo*,
  whose glow fragments rotated geometry differently from a clean render,
  and both sides here are clean renders of our own. Two knobs differ from
  G1i's defaults and the reasoning is in the script: shapes are matched at
  IoU 0.65 rather than 0.30, because two renders have no glow to excuse a
  loose box, and the pairs that have actually converged hold their score
  to 0.90. It also writes `compare_ref.py`'s overlays plus both inputs to
  `/tmp/g2i-<era>-<screen>/`, because "unmatched in source" names a
  bounding box and only the picture says what was in it.
  One thing G2i hides: a trace records how the material *photographs*,
  and part of that is residue the implementation is told not to draw —
  entropism's sharpening ring around every edge, neokitsch's blurred
  copy of its own content. The extractor bins such residue as an ink
  family of its own, and on the mailbox it was 48% (entropism) and 77%
  (neokitsch) of the design's shape area, so no faithful screen could
  clear the 60% bar. A trace marks those elements `class="photo"`; G2i
  renders the design with that class hidden and G1i renders it whole.
  The tag is a claim about the photo, so adding it is vision work like
  any other trace edit, and the header should say what was tagged.
  Two more things about the extractor worth knowing before reading a
  G2i number. Its colour budget is k=8 clusters and a haze or bloom
  takes about five of them, so on a hazed screen every ink family is
  competing for three bins: line art drawn dimmer than measured merges
  into a neighbouring family (the neokitsch mailbox wire scored as
  tape ink, RIFLES as foreground), and a screen drawn without its
  ground scores against a design whose ground *is* five of the eight
  families — backgrounds are not optional. And iced strokes cover
  ~15% more than rsvg's at the same width (.87 vs .75 coverage per
  edge pixel), which is enough to flip a hairline's bin; the pairs are
  a match by eye when that is all the diff says.
- **G2 — svg → iced.** The implementation matches the reference:
  rasterise the SVG (`rsvg-convert`) and
  `scripts/compare_ref.py <svg render> tests/golden/<screen>-<era>-...png`.
  The golden matrix already locks iced-vs-iced (G0: byte-identical run to
  run for one build — a toolkit bump can still move it, as iced 0.14 did
  by a few edge pixels on every screen); this
  gate locks iced-vs-reference. A schematic is expected to reach
  ~0.65+ layout / 0.85+ palette against its own golden; text rasterization
  and the hand-drawn gap keep edge correlation below the perfect-1.0 that
  G0 holds. G2 has no verdict and moves smoothly; where G2i can run,
  it is the one to iterate against and G2 is the sanity read beside it.
- Run any of them with `scripts/fidelity_check.sh [--source|--inventory|--implementation] [era [screen]]`.

## Rendering one screen by hand

`scripts/render.sh` is the golden matrix's recipe — headless weston,
pixman, a lavapipe ICD, `WGPU_BACKEND=vulkan`, the era's palette
published into a scratch `HOME` — pointed at a binary already on disk
instead of at a nix build of the crate:

```
scripts/render.sh --era neomil --size 1600x220 --out /tmp/bar.png cp-eras-ui-bar-window
scripts/render.sh --era kitsch --out /tmp/login.png cp-eras-ui-login   # 1600x900 default
scripts/render.sh --era none  --bin /path/to/cp-eras-ui-login ...      # compiled fallback
```

It is what G2i captures the implementation with, and it takes about ten
seconds against a warm nix store. It does not build anything: use
`nix-shell shell.nix --run 'cargo build --bin <name>'` first, or pass
`--bin`. The capture is faithful — `cp-eras-ui-login` in neomil comes out
byte-identical to `tests/golden/login-neomil-1600x900.png` — but it is
not hermetic, so `scripts/run_test_matrix.sh` remains what gates a
change.

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
G1/G1i fails, the trace is wrong (fix the SVG). The material always wins.

## Division of labour: who may touch which stage

**Only a model with strong vision may write or modify the SVGs in
`docs/`.** Every reference trace in this repo that was written without
opening the source image turned out to be a fabrication, and a wrong
SVG poisons everything downstream — the SVG is the *target* the iced
implementation is built and judged against. Editing one means reading
the source photo, cropping into regions, and judging renders by eye;
the gates check a reading, they cannot substitute for one.

**SVG → iced (stage 2 → 3) is coding work and does not need vision.**
The SVG is text carrying measured coordinates and sampled colours, and
G2/G0 give scripted, numeric feedback (`fidelity_check.sh`, the golden
matrix). A coding model converting `dashboard-trace.svg` into a
`screens::` arm should treat the SVG as the spec: take geometry and
palette from it verbatim, iterate against the gates, and **never edit
the SVG to make its own render match** — any mismatch it wants to fix
by touching `docs/` is a signal to stop and report instead
(see Direction of change above).

## Read the image

Stage 1 is a picture, and the only way to know what it holds is to look
at it. Two of the two traces ever checked against their material turned
out to have been written without that — both scored respectably on G1,
both drew things that are not in the photo, and both had their invented
descriptions copied into `docs/sources.md` as if observed, where
everything downstream then trusted them. `Layout::OpsCharts` exists
because of one of them.

So: before writing or trusting a trace, open the source. If the tool in
hand cannot display an image, that is a reason to stop and say so, not a
reason to infer the contents from a downsampled colour grid. `compare_ref.py`
and `extract_spec.py` are for *checking* a reading of the material; neither
is a substitute for having read it. `extract_spec.py --crops DIR` writes a
zoom per detected shape, which is the cheapest way to inspect a source
region by region.