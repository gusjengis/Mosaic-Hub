use std::borrow::Cow;

use plinth_core::graphics::Graphics;
use plinth_util::logging::log;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, Buffer, BufferBindingType, BufferDescriptor,
    BufferUsages, Color, CommandEncoderDescriptor, FragmentState, LoadOp, Operations,
    PipelineLayoutDescriptor, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp,
    TextureViewDescriptor, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::{camera::Camera, gpu_data::GPU_Data};

// This is where we store and initialize all of the freaky-ahh wgpu resources that the render pipeline needs to run.
pub struct GPU_Resources {
    pub rect_shader: Option<wgpu::ShaderModule>,
    pub rect_buffer: Option<wgpu::Buffer>,
    pub rect_count: Option<u32>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: Option<u32>,
    pub color_buffer: Option<wgpu::Buffer>,
    pub color_bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub camera_buffer: Option<wgpu::Buffer>,
    pub camera_bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub camera_bind_group: Option<wgpu::BindGroup>,
    pub combined_bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub combined_bind_group: Option<wgpu::BindGroup>,
    pub color_bind_group: Option<wgpu::BindGroup>,
    pub rect_pipeline_layout: Option<wgpu::PipelineLayout>,
}

impl GPU_Resources {
    pub(crate) fn new() -> Self {
        Self {
            rect_shader: None,
            rect_buffer: None,
            rect_count: None,
            index_buffer: None,
            index_count: None,
            color_buffer: None,
            color_bind_group_layout: None,
            color_bind_group: None,
            camera_buffer: None,
            camera_bind_group_layout: None,
            camera_bind_group: None,
            combined_bind_group_layout: None,
            combined_bind_group: None,
            rect_pipeline_layout: None,
        }
    }

    // Add a method to initialize the index buffer for rectangle rendering
    pub fn init_index_buffer(&mut self, gfx: &mut Graphics) {
        // For a rectangle, we need 2 triangles = 6 indices
        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0]; // Rectangle as two triangles

        // Create the index buffer
        let index_buffer = gfx.device.create_buffer(&BufferDescriptor {
            label: Some("Index Buffer"),
            size: (indices.len() * std::mem::size_of::<u16>()) as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // Write the index data to the buffer
        {
            let mut buffer_view = index_buffer.slice(..).get_mapped_range_mut();
            bytemuck::cast_slice_mut::<u8, u16>(&mut buffer_view).copy_from_slice(&indices);
        }
        index_buffer.unmap(); // Unmap after writing

        self.index_buffer = Some(index_buffer);
        self.index_count = Some(indices.len() as u32);
    }

    pub fn init_rect_shader(&mut self, gfx: &mut Graphics) {
        self.rect_shader = Some(gfx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Rect Shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/rect_shader.wgsl"))),
        }));
    }

    pub fn init_rect_buffer(&mut self, rect_data: Vec<f32>, gfx: &mut Graphics) {
        let device = &gfx.device;

        let buffer_size = (rect_data.len() * std::mem::size_of::<f32>()) as u64;
        // Create the rect buffer with the provided vertex data
        let rect_buffer = gfx.device.create_buffer(&BufferDescriptor {
            label: Some("Rectangle Buffer"),
            size: buffer_size,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: true,
        });

        // Write the rect data to the buffer
        {
            let mut buffer_view = rect_buffer.slice(..).get_mapped_range_mut();
            bytemuck::cast_slice_mut::<u8, f32>(&mut buffer_view).copy_from_slice(&rect_data);
        }
        rect_buffer.unmap(); // Unmap after writing

        // Store the GPU buffer in self
        self.rect_count = Some((rect_data.len() / 6) as u32); // Each rectangle has 6 values
        self.rect_buffer = Some(rect_buffer);
    }

    pub fn init_color_buffer(&mut self, color_data: Vec<f32>, gfx: &mut Graphics) {
        let device = &gfx.device;

        // Create the color buffer with the provided color data
        let color_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Color Buffer"),
            size: (color_data.len() * std::mem::size_of::<f32>()) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: true,
        });

        // Write the color data to the buffer
        {
            let mut buffer_view = color_buffer.slice(..).get_mapped_range_mut();
            bytemuck::cast_slice_mut::<u8, f32>(&mut buffer_view).copy_from_slice(&color_data);
        }
        color_buffer.unmap(); // Unmap after writing

        // Step 2: Create the Bind Group Layout (if not already created)
        let color_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Color Buffer Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Step 3: Create the Bind Group
        let color_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Color Bind Group"),
            layout: &color_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding(),
            }],
        });

        // Store the GPU buffer and bind group in self
        self.color_buffer = Some(color_buffer);
        self.color_bind_group = Some(color_bind_group);
        self.color_bind_group_layout = Some(color_bind_group_layout);
    }

    pub fn init_camera_buffer(&mut self, camera: &Camera, gfx: &mut Graphics) {
        let device = &gfx.device;

        // Get camera data as f32 array using the to_slice method
        let camera_data = camera.to_slice();

        // Create the camera uniform buffer
        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: std::mem::size_of::<[f32; 4]>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // Write the camera data to the buffer
        {
            let mut buffer_view = camera_buffer.slice(..).get_mapped_range_mut();
            bytemuck::cast_slice_mut::<u8, [f32; 4]>(&mut buffer_view)
                .copy_from_slice(&[camera_data]);
        }
        camera_buffer.unmap(); // Unmap after writing

        // Create the bind group layout for the camera uniform
        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create the bind group for the camera uniform
        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Store the camera buffer and bind group
        self.camera_buffer = Some(camera_buffer);
        self.camera_bind_group_layout = Some(camera_bind_group_layout);
        self.camera_bind_group = Some(camera_bind_group);
    }

    pub fn update_camera_buffer(&mut self, camera: &Camera, gfx: &mut Graphics) {
        if let Some(camera_buffer) = &self.camera_buffer {
            // Get camera data as f32 array using the to_slice method
            let camera_data = camera.to_slice();

            gfx.queue
                .write_buffer(camera_buffer, 0, bytemuck::cast_slice(&[camera_data]));
        }
    }

    pub fn init_combined_bind_group(&mut self, gfx: &mut Graphics) {
        // Make sure both color and camera bind group layouts are initialized
        if self.color_bind_group_layout.is_none() || self.camera_bind_group_layout.is_none() {
            panic!("Color and camera bind group layouts must be initialized before combined bind group");
        }

        let device = &gfx.device;

        // Create a combined bind group layout that includes both color and camera bindings
        let combined_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Combined Bind Group Layout"),
                entries: &[
                    // Color buffer binding (group 0, binding 0)
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Camera uniform binding (group 0, binding 1)
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // Create the combined bind group
        let combined_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Combined Bind Group"),
            layout: &combined_bind_group_layout,
            entries: &[
                // Color buffer binding
                BindGroupEntry {
                    binding: 0,
                    resource: self.color_buffer.as_ref().unwrap().as_entire_binding(),
                },
                // Camera uniform binding
                BindGroupEntry {
                    binding: 1,
                    resource: self.camera_buffer.as_ref().unwrap().as_entire_binding(),
                },
            ],
        });

        // Store the combined bind group and layout
        self.combined_bind_group_layout = Some(combined_bind_group_layout);
        self.combined_bind_group = Some(combined_bind_group);
    }

    pub fn init_rect_pipeline_layout(&mut self, gfx: &mut Graphics) {
        // Make sure combined bind group layout is initialized
        if self.combined_bind_group_layout.is_none() {
            panic!("Combined bind group layout must be initialized before pipeline layout");
        }

        self.rect_pipeline_layout = Some(gfx.device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: Some("Rectangle Pipeline Layout"),
                bind_group_layouts: &[self.combined_bind_group_layout.as_ref().unwrap()],
                push_constant_ranges: &[],
            },
        ));
    }

    pub fn init_rect_pipeline(&self, gfx: &mut Graphics) -> RenderPipeline {
        let shader = self.rect_shader.as_ref().unwrap();
        let pipeline_layout = self.rect_pipeline_layout.as_ref().unwrap();

        // Define the vertex buffer layout
        let rect_buffer_layout = VertexBufferLayout {
            array_stride: 6 * std::mem::size_of::<f32>() as u64, // x, y, w, h, color_index, fixed
            step_mode: VertexStepMode::Instance, // Use instance mode to draw multiple rectangles
            attributes: &[
                // Position (x, y)
                wgpu::VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                // Size (w, h)
                wgpu::VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 2 * std::mem::size_of::<f32>() as u64,
                    shader_location: 1,
                },
                // Color index
                wgpu::VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: 4 * std::mem::size_of::<f32>() as u64,
                    shader_location: 2,
                },
                // Fixed (bool)
                wgpu::VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: 5 * std::mem::size_of::<f32>() as u64,
                    shader_location: 3,
                },
            ],
        };

        gfx.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Rectangle Pipeline"),
                layout: Some(pipeline_layout),
                vertex: VertexState {
                    module: shader,
                    entry_point: Some("vs_main"),
                    buffers: &[rect_buffer_layout],
                    compilation_options: Default::default(),
                },
                fragment: Some(FragmentState {
                    module: shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(gfx.surface_config.format.into())],
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
    }

    pub fn process_queue(
        &mut self,
        queue: &mut std::collections::VecDeque<crate::gpu_data::GPU_Data>,
        gfx: &mut plinth_core::graphics::Graphics,
    ) {
        let mut rects = vec![];
        let mut colors = vec![];
        while !queue.is_empty() {
            match queue.pop_front().unwrap() {
                GPU_Data::Rect {
                    x,
                    y,
                    w,
                    h,
                    color_index,
                    fixed,
                } => {
                    rects.extend_from_slice(&[x, y, w, h, color_index, fixed]);
                }
                GPU_Data::Color { r, g, b, a } => {
                    colors.extend_from_slice(&[r, g, b, a]);
                }
            }
        }
        if !rects.is_empty() {
            self.append_rects(rects, gfx);
        }
        if !colors.is_empty() {
            self.append_colors(colors, gfx);
        }
    }

    fn append_rects(&mut self, rects: Vec<f32>, gfx: &mut plinth_core::graphics::Graphics) {
        // If we don't have a rect buffer yet, we can't append
        if self.rect_buffer.is_none() {
            self.init_rect_buffer(rects, gfx);
            return;
        }

        // Get the current rect count
        let current_rect_count = self.rect_count.unwrap_or(0);

        // Calculate the new rect count
        let new_rects_count = rects.len() / 6; // Each rectangle has 6 values
        let new_total_count = current_rect_count + (new_rects_count as u32);

        // Check if we need to resize the buffer
        if let Some(rect_buffer) = &self.rect_buffer {
            let current_buffer_size = rect_buffer.size();
            let needed_size = (new_total_count as usize * 6 * std::mem::size_of::<f32>()) as u64;

            if needed_size > current_buffer_size {
                // Need to create a larger buffer
                let new_buffer = gfx.device.create_buffer(&BufferDescriptor {
                    label: Some("Rectangle Buffer (Resized)"),
                    // Allocate some extra space to avoid frequent resizing
                    size: needed_size * 2,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });

                // Create a command encoder to copy the old data
                let mut encoder = gfx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Buffer Copy Encoder"),
                    });

                // Copy existing data to the new buffer
                encoder.copy_buffer_to_buffer(rect_buffer, 0, &new_buffer, 0, current_buffer_size);

                // Submit the copy command
                gfx.queue.submit(std::iter::once(encoder.finish()));

                // Write the new data to the new buffer
                let offset = (current_rect_count * 6 * std::mem::size_of::<f32>() as u32) as u64;
                gfx.queue
                    .write_buffer(&new_buffer, offset, bytemuck::cast_slice(&rects));

                // Replace the old buffer with the new one
                self.rect_buffer = Some(new_buffer);
            } else {
                // Buffer is large enough, just write the new data
                let offset = (current_rect_count * 6 * std::mem::size_of::<f32>() as u32) as u64;
                gfx.queue
                    .write_buffer(rect_buffer, offset, bytemuck::cast_slice(&rects));
            }
        }

        // Update the rect count
        self.rect_count = Some(new_total_count);
    }

    fn append_colors(&mut self, colors: Vec<f32>, gfx: &mut plinth_core::graphics::Graphics) {
        // If we don't have a color buffer yet, we can't append
        if self.color_buffer.is_none() {
            self.init_color_buffer(colors, gfx);
            return;
        }

        // Get the current color count
        let current_color_count = self.color_buffer.as_ref().map_or(0, |buf| {
            buf.size() as usize / (3 * std::mem::size_of::<f32>())
        });

        // Calculate the new total count
        let new_colors_count = colors.len() / 3; // Each color has 3 values (r, g, b)
        let new_total_count = current_color_count + new_colors_count;

        // Check if we need to resize the buffer
        if let Some(color_buffer) = &self.color_buffer {
            let current_buffer_size = color_buffer.size();
            let needed_size = (new_total_count * 3 * std::mem::size_of::<f32>()) as u64;

            if needed_size > current_buffer_size {
                // Need to create a larger buffer
                let new_buffer = gfx.device.create_buffer(&BufferDescriptor {
                    label: Some("Color Buffer (Resized)"),
                    size: needed_size * 2, // Allocate extra space to reduce frequent resizing
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });

                // Create a command encoder to copy the old data
                let mut encoder = gfx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Color Buffer Copy Encoder"),
                    });

                // Copy existing data to the new buffer
                encoder.copy_buffer_to_buffer(color_buffer, 0, &new_buffer, 0, current_buffer_size);

                // Submit the copy command
                gfx.queue.submit(std::iter::once(encoder.finish()));

                // Write the new data to the new buffer
                let offset = (current_color_count * 3 * std::mem::size_of::<f32>()) as u64;
                gfx.queue
                    .write_buffer(&new_buffer, offset, bytemuck::cast_slice(&colors));

                // Replace the old buffer with the new one
                self.color_buffer = Some(new_buffer);
            } else {
                // Buffer is large enough, just write the new data
                let offset = (current_color_count * 3 * std::mem::size_of::<f32>()) as u64;
                gfx.queue
                    .write_buffer(color_buffer, offset, bytemuck::cast_slice(&colors));
            }
        }
    }
}
