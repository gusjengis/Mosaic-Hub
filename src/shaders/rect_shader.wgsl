// Bind group for color data
@group(0) @binding(0)
var<storage, read> colors: array<vec4<f32>>;

// Vertex input
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color_index: f32,
}

// Vertex output
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color_index: f32,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color_index: f32,
) -> VertexOutput {
    // Define the four corners of a rectangle
    var positions = array<vec2<f32>, 4>(
        vec2<f32>(-0.5, -0.5), // Bottom-left
        vec2<f32>( 0.5, -0.5), // Bottom-right
        vec2<f32>( 0.5,  0.5), // Top-right
        vec2<f32>(-0.5,  0.5)  // Top-left
    );
    
    // Get the position for this vertex
    let pos = positions[vertex_index];
    
    // Scale by size and translate to position
    let world_pos = (pos * size) + position;
    
    var output: VertexOutput;
    output.position = vec4<f32>(world_pos, 0.0, 1.0);
    output.color_index = color_index;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Get the color from the color buffer using the color index
    let color_idx = u32(input.color_index);
    let color = colors[color_idx];
    
    return color;
}
