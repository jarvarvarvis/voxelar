use glfw::Window;

use crate::receivable_events::ReceivableEvents;

pub struct VoxelarWindow {
    glfw_window: Window,
}

impl VoxelarWindow {
    pub fn new(glfw_window: Window) -> Self {
        Self { glfw_window }
    }

    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.glfw_window.set_should_close(value)
    }

    pub fn set_receivable_events(&mut self, events: ReceivableEvents) {
        events.set_for(self);
    }

    pub fn glfw_window_mut(&mut self) -> &mut Window {
        &mut self.glfw_window
    }
}
