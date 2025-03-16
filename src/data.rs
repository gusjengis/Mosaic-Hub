use std::collections::VecDeque;

use mosaic_model::log::Log;

use crate::camera::Camera;
use crate::gpu_data::GPU_Data;

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
}
