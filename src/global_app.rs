use crate::my_app::MyApp;
use plinth_core::graphics::Rc;
use std::cell::{Ref, RefCell, RefMut};

// Essential for app to function.
// Gives wasm_bindgen functions access to our instance of MyApp so that data can be passed in from tsx.
pub static mut APP_INSTANCE: Option<Rc<RefCell<MyApp>>> = None;

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
