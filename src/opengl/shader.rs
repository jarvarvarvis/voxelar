use std::ffi::CString;
use std::ptr;

use gl::types::*;

pub struct Shader {
    pub handle: u32,
}

impl Shader {
    pub fn create(ty: GLenum) -> Self {
        let handle = unsafe { gl::CreateShader(ty) };
        Self { handle }
    }

    fn is_compiled(&self) -> bool {
        let mut success: i32 = 0;
        unsafe {
            gl::GetShaderiv(self.handle, gl::COMPILE_STATUS, &mut success);
        }
        success == gl::TRUE.into()
    }

    fn get_shader_info_log(&self) -> Option<String> {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(self.handle, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    self.handle,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Some(String::from_utf8(info_log).unwrap());
            }
        }

        None
    }

    pub fn compile(&self, source: String) -> crate::Result<()> {
        let source_c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(self.handle, 1, &source_c_str.as_ptr(), ptr::null());
            gl::CompileShader(self.handle);
        }

        if !self.is_compiled() {
            if let Some(info_log) = self.get_shader_info_log() {
                crate::bail!("Shader compilation failed: {}", info_log);
            } else {
                crate::bail!("Shader compilation failed");
            }
        }

        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle) }
    }
}

#[macro_export]
macro_rules! shader_from_source {
    ($ty:expr, $code:tt) => {{
        let shader = crate::opengl::shader::Shader::create($ty);
        shader.compile(format!("{}", $code))?;
        Ok(shader)
    }};
}

#[macro_export]
macro_rules! shader_from_file {
    ($ty:expr, $file:tt) => {{
        let shader = crate::opengl::shader::Shader::create($ty);
        shader.compile(include_str!($file).to_string())?;
        Ok(shader)
    }};
}
