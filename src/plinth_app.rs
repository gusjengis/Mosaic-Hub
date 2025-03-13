use mosaic_model::log::Log;
use plinth_core::plinth_app::PlinthApp;
use plinth_util::{logging::log, time::now};

use crate::my_app::MyApp;

impl PlinthApp for MyApp {
    fn before_render(&mut self) {
        let now = now();
        // log(format!("Frame Interval: {}", now - self.frame_start).as_str());
        self.frame_start = now;
    }

    fn after_render(&mut self) {
        let frame_time = now() - self.frame_start;
        log(format!("Frame Time: {}", frame_time).as_str());
    }
}
