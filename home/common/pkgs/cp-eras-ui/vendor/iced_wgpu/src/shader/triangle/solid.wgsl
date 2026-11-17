struct SolidVertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct SolidVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    // cp-eras-ui local patch (`src/light_edges.rs`): 1 for an opaque
    // vertex colour, 0 for a translucent one or one carrying the text tag.
    // Flat, so the test sees the colour exactly as the CPU wrote it.
    @location(1) @interpolate(flat) mark: f32,
}

@vertex
fn solid_vs_main(input: SolidVertexInput) -> SolidVertexOutput {
    var out: SolidVertexOutput;

    out.color = premultiply(input.color);
    out.position = globals.transform * vec4<f32>(input.position, 0.0, 1.0);
    out.mark = select(0.0, 1.0, input.color.a >= 1.0);

    return out;
}

@fragment
fn solid_fs_main(input: SolidVertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}

// The MSAA pass's entry point: the colour, plus one count per opaque
// fragment into the mark target (additive blend), which the resolve in
// `blit.wgsl` reads to tell a mark's samples from what it was drawn over.
struct MarkedOutput {
    @location(0) color: vec4<f32>,
    @location(1) mark: vec4<f32>,
}

@fragment
fn solid_fs_main_marked(input: SolidVertexOutput) -> MarkedOutput {
    var out: MarkedOutput;
    out.color = input.color;
    out.mark = vec4<f32>(input.mark, 0.0, 0.0, 0.0);
    return out;
}
