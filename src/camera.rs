use plinth_util::time::now;
use winit::{dpi::PhysicalSize, window::Cursor};

static ms_per_day: f64 = 1000.0 * 60.0 * 60.0 * 24.0;

pub struct Camera {
    pub init_pos: f64, // initial position, all positions passed to the gpu are relative to this. Just to avoid precision errors.
    pub pos: f64,      // timestamp, focused point in time on the timeline, center of screen
    pub scale: f64,    // width of the view in ms
    pub dimensions: PhysicalSize<u32>, // width and height of canvas/view in pixels
}

impl Camera {
    pub fn new() -> Self {
        Self {
            // this will center our view on the last 24 hours at the time of loading
            init_pos: now() - ms_per_day / 2.0,
            pos: now() - ms_per_day / 2.0,
            scale: 1.0, //ms_per_day,
            dimensions: PhysicalSize::new(0, 0),
        }
    }

    pub fn to_slice(&self) -> [f32; 4] {
        return [
            (self.init_pos - self.pos) as f32,
            self.scale as f32,
            0.0,
            0.0,
        ];
    }

    pub fn zoom(&mut self, delta: f64, cursor_x: f64) {
        let cursor_pos =
            (cursor_x / self.dimensions.width as f64 - 0.5) * -2.0 * self.scale + self.pos;
        let cursor_delta = self.pos - cursor_pos;
        let scalar = (1.1 as f64).powf(-delta / 120.0);
        self.scale = self.scale * scalar;
        self.pos = cursor_pos + cursor_delta * scalar;
    }

    pub fn pan(&mut self, dx: f64) {
        let screen_width = self.dimensions.width as f64; // need to figure out how to get this dynamically
        self.pos += ((dx / screen_width) * self.scale as f64 * 2.0);
    }
}
