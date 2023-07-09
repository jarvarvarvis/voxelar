use glfw::Window;
use raw_window_handle::*;

use crate::receivable_events::ReceivableEvents;

pub struct VoxelarWindow {
    glfw_window: Window,
    title: String,
}

impl VoxelarWindow {
    pub fn new(glfw_window: Window, title: &str) -> Self {
        Self {
            glfw_window,
            title: String::from(title),
        }
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

    pub fn glfw_window(&self) -> &Window {
        &self.glfw_window
    }

    pub fn glfw_window_mut(&mut self) -> &mut Window {
        &mut self.glfw_window
    }

    pub fn raw_window_handle(&self) -> RawWindowHandle {
        self.glfw_window.raw_window_handle()
    }

    pub fn raw_display_handle(&self) -> RawDisplayHandle {
        self.glfw_window.raw_display_handle()
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.glfw_window.set_title(title)
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.glfw_window.get_size()
    }
}
