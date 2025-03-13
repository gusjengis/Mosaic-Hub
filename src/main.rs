#![allow(warnings)]
use global_app::APP_INSTANCE;
use my_app::MyApp;
use plinth_core::graphics::Rc;
use std::cell::RefCell;

mod camera;
mod global_app;
mod gpu_resources;
mod input_manager;
mod io;
mod my_app;
mod plinth_app;
mod plinth_rendering;

pub fn main() {
    let user_app = Rc::new(RefCell::new(MyApp::new()));
    unsafe {
        APP_INSTANCE = Some(user_app.clone());
    }
    plinth_core::app::start_app(user_app.clone());
}
