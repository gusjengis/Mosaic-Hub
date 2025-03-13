struct Camera {
    position: f32,
    scale: f32,
    padding: vec2<f32>,
}

@group(0) @binding(0)
var<storage, read> colors: array<vec4<f32>>;
@group(0) @binding(1)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color_index: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color_index: f32,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec2<f32>,
    @location(1) dimensions: vec2<f32>,
    @location(2) color_index: f32,
) -> VertexOutput {
    // The four corners of our rectangle
    var positions = array<vec2<f32>, 4>(
        vec2<f32>(-0.5, -0.5), // Bottom-left
        vec2<f32>( 0.5, -0.5), // Bottom-right
        vec2<f32>( 0.5,  0.5), // Top-right
        vec2<f32>(-0.5,  0.5)  // Top-left
    );
    
    let vertex_pos = positions[vertex_index]; // position of current vertex
    let camera_offset = vec2(camera.position/camera.scale, 0.0); // offset from camera
    let instance_offset = position; // offset of instance 
    let scaled_instance_offset = vec2(instance_offset.x/camera.scale, instance_offset.y);
    let screen_pos = (vertex_pos * dimensions) + scaled_instance_offset  - camera_offset; // screen space position
    
    var output: VertexOutput;
    output.position = vec4<f32>(screen_pos, 0.0, 1.0);
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
