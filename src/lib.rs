pub extern crate glfw;
pub extern crate voxelar_math;

use glfw::*;

pub mod receivable_events;
pub mod render_context;
pub mod result;
pub mod vulkan;
pub mod window;
pub mod window_events;

pub use result::*;

use render_context::RenderContext;
use result::Context;
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

    pub fn create_window<'win>(
        &'win mut self,
        width: u32,
        height: u32,
        title: &'win str,
        mode: WindowMode,
    ) -> crate::Result<(VoxelarWindow, VoxelarWindowEvents)> {
        let (window, events) = self
            .glfw
            .create_window(width, height, title, mode)
            .context("Failed to create GLFW window.".to_string())?;
        let window = VoxelarWindow::new(window, title);
        let events = VoxelarWindowEvents::new(events);

        Ok((window, events))
    }

    pub fn set_swap_interval(&mut self, interval: SwapInterval) {
        self.glfw.set_swap_interval(interval)
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn vulkan_supported(&self) -> bool {
        self.glfw.vulkan_supported()
    }

    pub fn get_required_instance_extensions(&self) -> Option<Vec<String>> {
        self.glfw.get_required_instance_extensions()
    }

    pub fn load_render_context_for_window<C: RenderContext>(
        &mut self,
        window: &mut VoxelarWindow,
    ) -> crate::Result<C> {
        C::load(self, window)
    }
}
