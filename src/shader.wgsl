// Vertex shader
struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>, // think homogene Koordinaten, projektive Geometrie
};

@vertex // Marks the entry point.
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput; // `var` needs type specifier, can be modified.
    let x = f32(1 - i32(in_vertex_index)) * 0.5; // `let` allows type inference, cannot be modified.
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5; // `f32(...)`, `i32(...)` are casts.
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> { // `@location(0)` denotes first color target.
    return vec4<f32>(0.3, 0.2, 0.1, 1.0); // Sets color uf current fragment to brown.
}

// vim: ft=wgsl
