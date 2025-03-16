use crate::camera::Camera;
use mosaic_model::log::Log;

pub enum GPU_Data {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color_index: f32,
        fixed: f32,
    },
    Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
}

impl GPU_Data {
    pub fn from_log(log: &Log, camera: &Camera) -> Self {
        Self::Rect {
            x: (log.timestamp - camera.init_pos) as f32,
            y: 0.0,
            w: 0.001,
            h: 0.04,
            color_index: 1.0,
            fixed: 0.0,
        }
    }
}
