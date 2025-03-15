use crate::camera::Camera;
use mosaic_model::log::Log;

pub enum GPU_Data {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        ci: f32,
    },
}

impl GPU_Data {
    pub fn from_log(log: &Log, camera: &Camera) -> Self {
        return Self::Rect {
            x: (log.timestamp - camera.init_pos) as f32,
            y: 0.0,
            w: 0.003,
            h: 0.04,
            ci: 0.0,
        };
    }
}
