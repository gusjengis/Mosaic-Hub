use my_app::MyApp;
use plinth_core::graphics::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

mod io;
mod my_app;
mod plinth_app;
mod plinth_rendering;

static mut APP_INSTANCE: Option<Rc<RefCell<MyApp>>> = None;

pub fn main() {
    let user_app = Rc::new(RefCell::new(MyApp::new()));
    unsafe {
        APP_INSTANCE = Some(user_app.clone());
    }
    plinth_core::app::start_app(user_app.clone());
}
