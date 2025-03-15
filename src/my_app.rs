use js_sys::Uint8Array;
use mosaic_model::log::Log;
use winit::dpi::PhysicalSize;

use crate::{
    camera::Camera, data::Data, gpu_data::GPU_Data, gpu_resources::GPU_Resources,
    input_manager::Input_State,
};

pub struct MyApp {
    pub data: Data,
    pub gpu_resources: GPU_Resources,
    pub frame_start: f64,
    pub camera: Camera,
    pub input_state: Input_State,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            data: Data::new(),
            gpu_resources: GPU_Resources::new(),
            frame_start: 0.0,
            camera: Camera::new(),
            input_state: Input_State::new(),
        }
    }

    pub fn receive_logs(&mut self, body: String) {
        for log in body.lines() {
            self.data.logs.push(Log::from_http_body(log.to_string()));
        }

        for log in &self.data.logs {
            self.data
                .queue
                .push_back(GPU_Data::from_log(&log, &self.camera));
        }
    }

    fn convert_logs_to_buffs() {}
}
