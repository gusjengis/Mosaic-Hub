use plinth_util_temp::logging::log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn print_from_rust(s: String) {
    log(s.as_str());
}
