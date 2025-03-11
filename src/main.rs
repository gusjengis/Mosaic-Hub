use plinth_core::graphics::Rc;
use std::cell::RefCell;
mod bindgen_example;
mod my_app;
mod my_rendering;

fn main() {
    let user_app = Rc::new(RefCell::new(my_app::MyApp {}));
    plinth_core::app::start_app(user_app);
}
