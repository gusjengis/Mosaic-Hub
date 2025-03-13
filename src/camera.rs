use plinth_util::time::now;

static ms_per_day: i64 = 1000 * 60 * 60 * 24;

pub struct Camera {
    pos: i64,   // timestamp, focused point in time on the timeline, center of screen
    scale: i64, // width of the view in ms
}

impl Camera {
    pub fn new() -> Self {
        Self {
            // this will center our view on the last 24 hours at the time of loading
            pos: now() as i64 - ms_per_day / 2,
            scale: ms_per_day,
        }
    }
}
