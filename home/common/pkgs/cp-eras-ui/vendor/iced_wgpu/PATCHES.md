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
target the size of the surface and up to 13 texture samples per pixel.

Remove with the first change's removal, or when cp-eras-ui drops the print
material, or if upstream gains a post-processing hook that can carry it.
