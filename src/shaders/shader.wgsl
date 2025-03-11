struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    
    var output: VertexOutput;
    output.position = vec4<f32>(x, y, 0.0, 1.0);
    output.coord = vec2<f32>(x, y);
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let pos = vec2(input.coord.x, input.coord.y);
    let a = vec2(-1.0, -1.0);
    let b = vec2(0.0, 1.0);
    let c = vec2(1.0, -1.0);
    let red   = 1.0 - length(a - pos)/2.0;
    let green = 1.0 - length(c - pos)/2.0;
    let blue  = 1.0 - length(b - pos)/2.0;

    return vec4<f32>(red, green, blue, 1.0);
}
