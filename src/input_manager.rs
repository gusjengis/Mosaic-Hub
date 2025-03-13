use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

use crate::{camera, my_app::MyApp};

pub struct Input_State {
    pub left: bool,
    pub right: bool,
    pub cursor_pos: PhysicalPosition<f64>,
}

impl Input_State {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            cursor_pos: PhysicalPosition::new(0.0, 0.0),
        }
    }
}

pub trait Input_Manager {
    fn v_scroll(&mut self, delta: f64);
    fn h_scroll(&mut self, delta: f64);
    fn click(&mut self, button: &MouseButton, state: &ElementState);
    fn cursor_moved(&mut self, position: PhysicalPosition<f64>);
}

// A nice place to store input handling logic, cleaner when insulated from the huge match statement in plinth_app.rs
impl Input_Manager for MyApp {
    fn v_scroll(&mut self, delta: f64) {
        self.camera.zoom(delta, self.input_state.cursor_pos.x);
    }

    fn h_scroll(&mut self, delta: f64) {
        self.camera.pan(delta);
    }

    fn click(&mut self, button: &MouseButton, state: &ElementState) {
        match button {
            MouseButton::Left => self.input_state.left = state == &ElementState::Pressed,
            MouseButton::Right => self.input_state.right = state == &ElementState::Pressed,
            _ => {}
        }
    }

    fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        if self.input_state.left {
            let dx = position.x - self.input_state.cursor_pos.x;
            self.camera.pan(dx);
        }
        self.input_state.cursor_pos = position;
    }
}
