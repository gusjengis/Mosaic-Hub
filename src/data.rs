use std::collections::VecDeque;

use mosaic_model::log::Log;

use crate::{camera::Camera, gpu_data::GPU_Data};

pub struct Data {
    pub logs: Vec<Log>,
    pub queue: VecDeque<GPU_Data>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            logs: vec![],
            queue: VecDeque::new(),
        }
    }

    pub fn get_log_rects(&self, camera: &Camera) -> Vec<f32> {
        let mut rects = vec![0.0, 0.0, 0.0, 0.0, 0.0];
        if self.logs.len() > 0 {
            for log in &self.logs {
                rects.push(log_pos(&log, camera));
                rects.push(0.0);
                rects.push(0.003);
                rects.push(0.04);
                rects.push(0.0);
            }
        }

        rects
    }
}

pub fn log_pos(log: &Log, camera: &Camera) -> f32 {
    (log.timestamp - camera.init_pos) as f32
}
