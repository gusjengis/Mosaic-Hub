use js_sys::Atomics::wait;
use js_sys::Math::random;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, Color,
    CommandEncoderDescriptor, FragmentState, LoadOp, Operations, PipelineLayoutDescriptor,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, StoreOp, TextureViewDescriptor, VertexBufferLayout,
    VertexFormat, VertexState, VertexStepMode,
};

use crate::gpu_data::GPU_Data;
use crate::my_app::MyApp;
use plinth_core::graphics::Graphics;
use plinth_core::plinth_app::PlinthRenderer;
use plinth_util::logging::log;
use std::borrow::Cow;
use web_sys::window;
static rectangles: u32 = 100;

impl PlinthRenderer for MyApp {
    fn create_pipeline(&mut self, gfx: &mut Graphics) -> RenderPipeline {
        // create initial data, this will move to a function later
        // define the first color in our palette
        self.data.queue.push_back(GPU_Data::Color {
            r: 0.4,
            g: 0.4,
            b: 0.4,
            a: 1.0,
        });
        // define the log color
        self.data.queue.push_back(GPU_Data::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        });
        // // create fixed rectangle, this is our timeline
        self.data.queue.push_back(GPU_Data::Rect {
            x: -1.0,
            y: 0.002,
            w: 2.0,
            h: 0.004,
            color_index: 0.0,
            fixed: 1.0,
        });

        // Initialize shader
        self.gpu_resources.init_rect_shader(gfx);

        // Initialize camera, color, and rect buffers
        self.gpu_resources.init_camera_buffer(&self.camera, gfx);
        // Load initial data into these buffers
        if !self.data.queue.is_empty() {
            self.gpu_resources.process_queue(&mut self.data.queue, gfx);
        }

        // Initialize combined bind group that includes both color and camera data
        self.gpu_resources.init_combined_bind_group(gfx);

        // Initialize index buffer
        self.gpu_resources.init_index_buffer(gfx);

        // Initialize pipeline layout and pipeline
        self.gpu_resources.init_rect_pipeline_layout(gfx);
        self.gpu_resources.init_rect_pipeline(gfx)
    }

    fn render(&mut self, gfx: &mut Graphics) {
        if !self.data.queue.is_empty() {
            self.gpu_resources.process_queue(&mut self.data.queue, gfx);
        }

        // Update camera data in the uniform buffer
        self.gpu_resources.update_camera_buffer(&self.camera, gfx);

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
                self.gpu_resources.combined_bind_group.as_ref().unwrap(),
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

        gfx.window.request_redraw();
        // self.render(gfx);
    }
}
