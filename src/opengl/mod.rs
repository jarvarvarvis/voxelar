pub extern crate gl;

pub mod mesh;
pub mod program;
pub mod shader;
pub mod vao;
pub mod vbo;

use crate::render_context::RenderContext;
use crate::window::VoxelarWindow;

pub struct GlContext;

impl RenderContext for GlContext {
    fn load(window: &mut VoxelarWindow) {
        let glfw_window = window.glfw_window_mut();
        gl::load_with(|s| glfw_window.get_proc_address(s) as *const _);
    }
}
