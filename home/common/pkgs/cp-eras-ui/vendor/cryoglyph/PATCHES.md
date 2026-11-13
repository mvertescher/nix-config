# cryoglyph 0.1.0 local patch

This is the crates.io `cryoglyph` 0.1.0 source (the glyph renderer behind
`iced_wgpu` 0.14's text pipeline), copied from the Cargo registry crate
(checksum `08bc795bdbccdbd461736fb163930a009da6597b226d6f6fce33e7a8eb6ec519`).
Upstream is <https://github.com/iced-rs/cryoglyph>. Only `src/`, the three
licence files and `README.md` are kept; `Cargo.toml` is the registry's
normalised manifest with its example, bench and dev-dependency sections
removed, since none of those files are vendored.

## The change: dark-ink coverage correction

`src/shader.wgsl` and the new `src/ink.rs` (its CPU mirror and unit
tests; `pub mod ink` in `src/lib.rs`) are the only edits.

In `ColorMode::Accurate` on an sRGB surface, which is what `iced_wgpu`
uses, a glyph's edge coverage is blended in linear light. The reference
art and the traces (rsvg) blend encoded values. For light ink on a dark
ground the difference is what cp-eras-ui's weights were tuned against and
is left alone; for dark ink on a light fill it makes every partly covered
pixel lighter, measured as text a stop thinner and paler than the trace
in all four eras (todo/review-2026-11-13.md item 1). Switching the whole
pipeline to encoded blending (`web-colors`) was tried and rejected because
it thins light text instead (Cargo.toml).

The fragment shader now maps a mask glyph's coverage `a` to the coverage
at which a linear blend lands where an encoded blend would, on a fill of
assumed encoded lightness `ASSUMED_FILL` (0.78), mixed in by a weight
that depends on the ink's luminance only: full up to `FULL_Y` (0.04),
smoothstep to exactly zero at `OFF_Y` (0.085). `src/ink.rs` has the
derivation and why those numbers. Properties, all unit-tested:

- Inks with luminance >= `OFF_Y` take the upstream expression unchanged,
  so light-on-dark text is bit-identical (the 25-case golden matrix
  changes only inside dark-ink glyph quads).
- Coverage 0 and 1 are fixed, the curve is monotonic in coverage, never
  thins, and varies smoothly with the ink (no hard threshold).
- Web colour mode (encoded blending already) and colour glyphs (emoji)
  are untouched. The atlas, glyph cache and uploaded coverage are
  unchanged; the work is a few ALU ops per mask fragment.

Not covered: text that `iced_wgpu` draws as filled outlines through the
MSAA triangle pipeline (canvas text under a non-uniform transform, and
cp-eras-ui's cached outline runs). That route has the same linear-blend
thinning but no coverage value to correct.

The constants appear in both files; `ink::tests::shader_constants_match`
fails if they drift apart.

Run its tests with `cargo test -p cryoglyph --lib`.

Remove this vendor patch and the Cargo override if upstream gains an
equivalent (contrast- or gamma-adjusted coverage for dark text), or if
cp-eras-ui moves its text off this renderer.
