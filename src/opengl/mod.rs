pub extern crate gl;

pub mod mesh;
pub mod program;
pub mod shader;
pub mod uniform;
pub mod vao;
pub mod vbo;

use std::ffi::CStr;

use gl::types::*;

use crate::render_context::RenderContext;
use crate::window::VoxelarWindow;

pub struct GlContext;

impl GlContext {
    pub fn get_string(&self, name: GLenum) -> crate::Result<&str> {
        unsafe {
            let ptr = gl::GetString(name);
            let cstr = CStr::from_ptr(ptr as *const GLchar);
            let result = cstr.to_str()?;
            Ok(result)
        }
    }
}

impl RenderContext for GlContext {
    fn load(window: &mut VoxelarWindow) -> Self {
        let glfw_window = window.glfw_window_mut();
        gl::load_with(|s| glfw_window.get_proc_address(s) as *const _);
        Self
    }

    fn get_info(&self) -> crate::Result<String> {
        let vendor = self.get_string(gl::VENDOR)?;
        let renderer = self.get_string(gl::RENDERER)?;
        let version = self.get_string(gl::VERSION)?;
        let shading_language_version = self.get_string(gl::SHADING_LANGUAGE_VERSION)?;

        Ok(format!(
            "Vendor: {}\nRenderer: {}\nVersion: {}\nShading Language Version: {}",
            vendor,
            renderer,
            version,
            shading_language_version
        ))
    }
}
