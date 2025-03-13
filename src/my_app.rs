use js_sys::Uint8Array;
use mosaic_model::log::Log;
use plinth_util::logging::log;

use crate::gpu_resources::GPU_Resources;

pub struct MyApp {
    pub logs: Vec<Log>,
    pub gpu_resources: GPU_Resources,
    pub frame_start: f64,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            logs: vec![],
            gpu_resources: GPU_Resources::new(),
            frame_start: 0.0,
        }
    }

    pub fn receive_logs(&mut self, body: String) {
        for log in body.lines() {
            self.logs.push(Log::from_http_body(log.to_string()));
        }

        // for element in &self.logs {
        //     log(format!("{}", element).as_str());
        // }
    }

    fn convert_logs_to_buffs() {}
}
