use mosaic_model::log::Log;
use plinth_core::plinth_app::PlinthApp;
use plinth_util::{logging::log, time::now};

use crate::{input_manager::Input_Manager, my_app::MyApp};

impl PlinthApp for MyApp
where
    MyApp: Input_Manager,
{
    fn before_render(&mut self) {
        // let now = now();
        // log(format!("Frame Interval: {}", now - self.frame_start).as_str());
        // self.frame_start = now;
    }

    fn after_render(&mut self) {
        // let frame_time = now() - self.frame_start;
        // log(format!("Frame Time: {}", frame_time).as_str());
    }

    fn event_handler(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: &winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => self.cursor_moved(*position),
            // winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
            // winit::event::WindowEvent::CursorLeft { device_id } => todo!(),
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => match delta {
                winit::event::MouseScrollDelta::PixelDelta(delta) => {
                    self.h_scroll(-delta.x);
                    self.v_scroll(delta.y);
                }
                _ => {}
            },
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                self.click(button, state);
            }
            // winit::event::WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::PanGesture {
                device_id,
                delta,
                phase,
            } => self.h_scroll(delta.x as f64),
            // winit::event::WindowEvent::Touch(touch) => todo!(),
            winit::event::WindowEvent::Resized(dimensions) => {
                self.camera.dimensions = *dimensions;
            }
            _ => {}
        }
    }
}
