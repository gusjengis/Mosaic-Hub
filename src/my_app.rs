use plinth_core::plinth_app::PlinthApp;
use plinth_util_temp::logging::log;

pub struct MyApp {}

impl PlinthApp for MyApp {
    fn init(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            log("Hello from Rust!");
        }
    }
}
