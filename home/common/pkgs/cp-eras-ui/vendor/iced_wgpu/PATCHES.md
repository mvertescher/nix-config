# iced_wgpu 0.14.0 local patch

This is the crates.io `iced_wgpu` 0.14.0 source, corresponding to upstream
iced commit `3997291f318a8bc06fa522f5579836fb3feb94df` (`wgpu/`). The
source came from the Cargo registry crate; `LICENSE` is the MIT license from
the sibling crates.io `iced` 0.14.0 crate.

`src/triangle.rs` has one local change: when a mesh's clip does not survive
physical-pixel snapping, advance the render pass's index offset by that
mesh's index count before skipping it. The prepare pass uploads all indices,
including those of skipped meshes, so leaving this offset unchanged makes
later meshes read the wrong indices.

Remove this vendor patch and the Cargo override when the pinned upstream
`iced_wgpu` release contains the same skipped-index correction.
