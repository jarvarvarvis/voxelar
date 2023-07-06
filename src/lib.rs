pub extern crate glfw;

use glfw::*;

pub mod opengl;
pub mod receivable_events;
pub mod render_context;
pub mod result;
pub mod window;
pub mod window_events;

pub use result::*;

use window::*;
use window_events::*;

pub struct Voxelar {
    glfw: Glfw,
}

impl Voxelar {
    pub fn new() -> Self {
        Self {
            glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
        }
    }

    pub fn window_hint(&mut self, hint: WindowHint) {
        self.glfw.window_hint(hint);
    }

    pub fn create_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
        mode: WindowMode,
    ) -> (VoxelarWindow, VoxelarWindowEvents) {
        let (window, events) = self
            .glfw
            .create_window(width, height, title, mode)
            .expect("Failed to create GLFW window."); // TODO: think about how this could be
                                                      // handled using a user-defined error/result
                                                      // type
        let window = VoxelarWindow::new(window);
        let events = VoxelarWindowEvents::new(events);

        (window, events)
    }

    pub fn set_swap_interval(&mut self, interval: SwapInterval) {
        self.glfw.set_swap_interval(interval)
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}
