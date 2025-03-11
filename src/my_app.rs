use js_sys::Uint8Array;
use mosaic_model::log::Log;
use plinth_util_temp::logging::log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MyApp {
    logs: Vec<Log>,
}

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn receive_logs(&mut self, body: String) {
        for log in body.lines() {
            self.logs.push(Log::from_http_body(log.to_string()));
        }

        for element in &self.logs {
            log(format!("{}", element).as_str());
        }
    }
}
