use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, Color,
    CommandEncoderDescriptor, FragmentState, LoadOp, Operations, PipelineLayoutDescriptor,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, StoreOp, TextureViewDescriptor, VertexBufferLayout,
    VertexFormat, VertexState, VertexStepMode,
};

use crate::my_app::MyApp;
use plinth_core::graphics::Graphics;
use plinth_core::plinth_app::PlinthRenderer;
use std::borrow::Cow;

impl PlinthRenderer for MyApp {
    fn create_pipeline(&mut self, gfx: &mut Graphics) -> RenderPipeline {
        // Initialize shader
        self.gpu_resources.init_rect_shader(gfx);

        // Initialize color buffer first (needed for pipeline layout)
        self.gpu_resources.init_color_buffer(
            vec![
                1.0, 0.0, 0.0, 1.0, // Red (index 0)
                0.0, 1.0, 0.0, 1.0, // Green (index 1)
                0.0, 0.0, 1.0, 1.0, // Blue (index 2)
                1.0, 0.0, 1.0, 1.0, // Purple (index 3)
            ],
            gfx,
        );

        // Initialize rectangle buffer with multiple rectangles
        // Each rectangle has 5 values: x, y, width, height, color_index
        self.gpu_resources.init_rect_buffer(
            vec![
                -0.5, -0.5, 0.4, 0.4, 0.0, // Rectangle 1 (red)
                0.5, -0.5, 0.4, 0.4, 1.0, // Rectangle 2 (green)
                0.5, 0.5, 0.4, 0.4, 2.0, // Rectangle 3 (blue)
                -0.5, 0.5, 0.4, 0.4, 3.0, // Rectangle 4 (purple)
            ],
            gfx,
        );

        // Initialize index buffer
        self.gpu_resources.init_index_buffer(gfx);

        // Initialize pipeline layout and pipeline
        self.gpu_resources.init_rect_pipeline_layout(gfx);
        self.gpu_resources.init_rect_pipeline(gfx)
    }

    fn render(&mut self, gfx: &mut Graphics) {
        let frame = gfx
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture.");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = gfx
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            r_pass.set_pipeline(&gfx.render_pipelines[0]);
            r_pass.set_bind_group(
                0,
                self.gpu_resources.color_bind_group.as_ref().unwrap(),
                &[],
            );
            r_pass.set_vertex_buffer(
                0,
                self.gpu_resources.rect_buffer.as_ref().unwrap().slice(..),
            );
            r_pass.set_index_buffer(
                self.gpu_resources.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            // Draw indexed instances - one for each rectangle
            r_pass.draw_indexed(
                0..self.gpu_resources.index_count.unwrap(),
                0,
                0..self.gpu_resources.rect_count.unwrap(),
            );
        }

        gfx.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
