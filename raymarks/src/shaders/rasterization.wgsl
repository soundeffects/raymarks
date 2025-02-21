@vertex
fn vertex_shader(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var vertices = array<vec4<f32>, 3>(
        vec4<f32>(0.0, 1.0, 0.0, 1.0),
        vec4<f32>(-1.0, -1.0, 0.0, 1.0),
        vec4<f32>(1.0, -1.0, 0.0, 1.0)
    );
    return vertices[in_vertex_index];
}

@fragment
fn fragment_shader() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
