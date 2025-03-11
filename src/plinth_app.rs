use mosaic_model::log::Log;
use plinth_core::plinth_app::PlinthApp;
use plinth_util_temp::logging::log;

use crate::my_app::MyApp;

impl PlinthApp for MyApp {
    fn init(&mut self) {
        let test_log = Log {
            label: "Test".to_string(),
            timestamp: 0,
        };
        log(format!("{}", test_log).as_str());
    }
}
