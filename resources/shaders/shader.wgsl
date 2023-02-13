struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vertex(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let scale = 0.5;
    let x = f32(1 - i32(in_vertex_index)) * scale;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * scale;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = vec4<f32>(0.2, 0.2, 0.8, 1.0);
    return color;
}
