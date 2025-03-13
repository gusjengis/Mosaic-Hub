#![allow(warnings)]
use my_app::MyApp;
use plinth_core::graphics::Rc;
use std::cell::{Ref, RefCell, RefMut};
use wasm_bindgen::prelude::wasm_bindgen;

mod gpu_resources;
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

pub fn app() -> Option<Ref<'static, MyApp>> {
    unsafe {
        if let Some(app) = &APP_INSTANCE {
            return Some(app.borrow());
        } else {
            eprintln!("❌ ERROR: APP_INSTANCE is None!");
        }
    }

    None
}
pub fn app_mut() -> Option<RefMut<'static, MyApp>> {
    unsafe {
        if let Some(app) = &APP_INSTANCE {
            return Some(app.borrow_mut());
        } else {
            eprintln!("❌ ERROR: APP_INSTANCE is None!");
        }
    }

    None
}
