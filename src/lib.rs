pub extern crate glfw;
use glfw::Glfw;

pub mod window;
use window::Window;

pub struct Voxelar {
    glfw: Glfw,
}

impl Voxelar {
    pub fn new() -> Self {
        Self {
            glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
        }
    }

    pub fn create_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
        mode: glfw::WindowMode,
    ) -> Window {
        let (window, events) = self
            .glfw
            .create_window(width, height, title, mode)
            .expect("Failed to create GLFW window."); // TODO: think about how this could be
                                                      // handled using a user-defined error/result
                                                      // type
        Window::new(window, events)
    }
}
