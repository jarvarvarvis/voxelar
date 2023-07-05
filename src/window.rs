use std::sync::mpsc::Receiver;

pub struct Window {
    glfw_window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(
        glfw_window: glfw::Window,
        event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    ) -> Self {
        Self {
            glfw_window,
            event_receiver,
        }
    }

    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }
}
