# iced_wgpu 0.14.0 local patch

This is the crates.io `iced_wgpu` 0.14.0 source, corresponding to upstream
iced commit `3997291f318a8bc06fa522f5579836fb3feb94df` (`wgpu/`). The
source came from the Cargo registry crate; `LICENSE` is the MIT license from
the sibling crates.io `iced` 0.14.0 crate.

`src/triangle.rs` has one local change (the second, the print pass, is below): when a mesh's clip does not survive
physical-pixel snapping, advance the render pass's index offset by that
mesh's index count before skipping it. The prepare pass uploads all indices,
including those of skipped meshes, so leaving this offset unchanged makes
later meshes read the wrong indices.

Remove this vendor patch and the Cargo override when the pinned upstream
`iced_wgpu` release contains the same skipped-index correction.

## Second change: the optional print pass

`src/print.rs` and `src/shader/print.wgsl` are new; `src/lib.rs` gains
`pub mod print`, a `print: Option<print::Pass>` field on `Renderer` and a
branch at the top of `Renderer::present`. Nothing else changes.

cp-eras-ui states an era's print material (registration copies, fill
scanlines, grain; see `src/print.rs` in the crate and
`docs/print-overlay.md`) once per process with `iced_wgpu::print::set`.
While a material is set, `present` draws the frame into an offscreen
texture of the engine format and size, then runs one full-screen fragment
pass from it into the surface texture. While none is set -- every process
that never calls `set`, or one that set only zero strengths
(`Params::is_off`) -- `present` takes the upstream single draw unchanged, so
those frames are bit-identical (the 16 Kitsch/Neo-kitsch/bar/mail cases of
the golden matrix, and the Neomil and Entropism cases with
`CP_ERAS_UI_PRINT=0`, were checked byte-for-byte).

Not covered: `Renderer::screenshot` (iced's in-process screenshot task)
renders without the pass, and the tiny-skia fallback renderer has none.
The pass is a fixed cost per presented frame while set: one extra render
target the size of the surface and up to 16 texture samples per pixel
(four for the fill gate, four per registration copy, up to three copies);
since 2026-11-16 the far copy may take 16, the enclosure gate up to
312 more on pixels a copy changed (usually one ray, 39), and the grain
up to 16 noise taps per pixel.

Since 2026-11-16 the material may also carry (`docs/print-overlay.md`,
"Above the print, and the bloom"):

- Up to `MAX_OVER` (12) regions above the print (`print::Over`, three
  `vec4`s each after the parameters' eighteen): no copy shows inside one,
  a non-casting one leaves no copy of itself, and one may wear a hatched
  halo around its light marks (up to 44 taps, region pixels only).
- A bloom (`shader/bloom.wgsl`, new): three more full-screen passes at
  half resolution into two `Rgba16Float` targets kept with the frame
  target -- threshold + 2x2 downsample, a separable Gaussian across, then
  down (at most 49 taps each) -- sampled once by the print pass (binding
  3) and screened over. They run only while the material has a bloom.
- `print::bench`, hidden, times the pass for the doc; not used in drawing.
  The frame target gained `COPY_DST` usage for it.

Remove with the first change's removal, or when cp-eras-ui drops the print
material, or if upstream gains a post-processing hook that can carry it.

## Third change: light edges resolved in encoded space

`src/light_edges.rs` is new (the rule's CPU mirror, its constants and
unit tests; `pub mod light_edges` in `src/lib.rs`). Edited:
`src/shader/blit.wgsl` (the MSAA resolve-and-composite), `src/triangle.rs`
and `src/triangle/msaa.rs` (a second MSAA attachment and the frame copy),
`src/shader/triangle/solid.wgsl` and `gradient.wgsl` (an entry point that
also writes it), `src/geometry.rs` (`fill_text`'s outline route),
`src/window/compositor.rs` (the surface is also a copy source where the
platform allows) and `src/print.rs` (the print pass's frame target gains
`COPY_SRC`; nothing the pass draws changes).

Why: every canvas fill and stroke is drawn into a multisampled target of
the sRGB surface format, resolved by averaging the samples in linear light
and composited in linear light. The reference art and the traces (rsvg)
blend encoded values. The sRGB curve is convex, so every partly covered
pixel of a light mark on a dark ground comes out lighter than the encoded
blend makes it, and every thin light line reads heavy: Neomil's 0.7 dp
header rule measured 2.21 px against the reference's 1.80 px
(`docs/review-2026-11-16/neomil-entropism-fixes.md`). Dark text on light
fills had the mirror-image problem, corrected for glyphs in cryoglyph and
on the outline route by `widgets::stem` in cp-eras-ui.

What: the solid triangle pipeline's MSAA entry point
(`solid_fs_main_marked`) also adds one per fragment, into an `R16Float`
attachment with additive blending, when the vertex colour is opaque
(`alpha >= 1`, carried flat). Within a pixel, samples above the lowest
count are a *mark*; the others are what it was drawn over. Before the
resolve draws, the part of the frame under the layer is copied into a
texture of the targets' size (`copy_ground`), so samples that hold nothing
from this pass are composited over what they will cover. The resolve
(`fs_ground`, or `fs_main` when the frame cannot be copied) then:

- returns the hardware resolve unchanged (bit-identical to upstream)
  where every sample has the same count -- interiors, abutting fills,
  translucent layers, text -- and where the mark is not lighter than what
  it covers;
- otherwise averages the samples' *encoded* values, which is exactly the
  coverage blend an encoded rasteriser gives those samples, and mixes it
  over the linear result by the mark's Michelson contrast of luminance
  against the under: 0 at or below zero, a smoothstep to 1 at
  `CONTRAST_FULL` (0.1). The result is opaque and replaces the frame's
  pixel.

Properties, unit-tested in `light_edges::tests` (`cargo test -p iced_wgpu
--lib light_edges`): uniform and fully covered pixels unchanged; dark marks
on light fills unchanged bit for bit (so `widgets::stem` and Neomil's dark
tile insets keep what they had); for a light mark the edge row lands on
the encoded blend (a quarter-covered row of #fb3535 on #14244d at encoded
coverage 0.25, where linear gave 0.50); monotonic in coverage; never
lightens a pixel; continuous in the inks; a mark drawn over nothing
resolves, given the copied frame, exactly as if its ground had been drawn
in the pass. `shader_constants_match` keeps the WGSL and Rust constants
together.

Text is excluded. Light text was tuned against the linear blend
(cryoglyph's `PATCHES.md`) and is left alone: `geometry::Frame::fill_text`
fills glyph outlines with `light_edges::text_ink`, an alpha of `1 - 2^-20`
(a millionth of the colour under it shows through), so they are never
marks; cp-eras-ui uses the same function where it fills or strokes glyph
outlines itself. Gradient meshes are never marks either.

`ICED_WGPU_LIGHT_EDGES=off` restores the upstream resolve exactly (for A/B
renders from one binary); `=debug` paints each mark-edge pixel by its class
instead (green: corrected, by weight; blue: a dark mark, untouched;
purple: drawn over nothing and no frame copy, untouched).

Not covered:

- The quad pipeline (iced container and control borders, which this crate
  uses only for built-in widget edges) keeps its analytic linear-light
  antialiasing.
- A frame that cannot be a copy source (a surface whose platform does not
  offer `COPY_SRC`, `Renderer::screenshot`'s target): marks drawn over
  nothing in their layer keep the linear blend there.
- Translucent strokes are not marks, so their edges keep the linear blend.

Cost per mesh layer: one copy of the layer's bounds from the frame, an
extra `R16Float` MSAA attachment the size of the targets, and in the
resolve up to two loads per sample per pixel, plus a few more on mark-edge
pixels. Remove with the first change's removal, or if upstream resolves
MSAA in encoded space for light marks.
