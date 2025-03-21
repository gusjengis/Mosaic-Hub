use crate::global_app::app_mut;
use js_sys::Uint8Array;
use plinth_util::logging::log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn receive_body(body_bytes: Uint8Array) {
    let body_vec = body_bytes.to_vec();
    let body = match String::from_utf8(body_vec) {
        Ok(string) => string,
        Err(e) => {
            log("Failed to convert log body to string. Fn: receive_logs()");

            "".to_string()
        }
    };

    app_mut().unwrap().receive_logs(body);
}
