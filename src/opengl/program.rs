use std::{ffi::CString, ptr};

use gl::types::*;

use crate::opengl::shader::Shader;

use super::uniform::Uniform;

pub struct Program {
    pub handle: u32,
}

impl Program {
    pub fn create() -> Self {
        let handle = unsafe { gl::CreateProgram() };
        Self { handle }
    }

    fn is_linked(&self) -> bool {
        let mut success: i32 = 0;
        unsafe {
            gl::GetProgramiv(self.handle, gl::LINK_STATUS, &mut success);
        }
        success == gl::TRUE.into()
    }

    fn get_program_info_log(&self) -> Option<String> {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramiv(self.handle, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
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

    pub fn attach(&self, shaders: Vec<Shader>) {
        for shader in shaders {
            unsafe {
                gl::AttachShader(self.handle, shader.handle);
            }
        }
    }

    pub fn link(&self) -> crate::Result<()> {
        unsafe { gl::LinkProgram(self.handle) };

        if !self.is_linked() {
            if let Some(info_log) = self.get_program_info_log() {
                crate::bail!("Program linking failed: {}", info_log);
            } else {
                crate::bail!("Program linking failed");
            }
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn from_shaders(shaders: Vec<Shader>) -> crate::Result<Self> {
        let program = Self::create();
        program.attach(shaders);
        program.link()?;
        Ok(program)
    }

    /// Uniforms must (at maximum) live as long as the Program that they have been 
    /// created from. Otherwise, setting the value of a uniform after the program has
    /// been moved and dropped (like below) will result in an error (even undefined 
    /// behaviour, perhaps):
    ///
    /// ```
    /// use voxelar::opengl::uniform::Uniforms;
    /// use voxelar::opengl::program::Program;
    /// fn move_program(_: Program) {}
    ///
    /// fn example() {
    ///     let program = Program::create();
    ///     let mut uniform = program.get_uniform("test").unwrap();
    ///     uniform.set(Uniforms::Uniform1f(0.5));
    ///     move_program(program);
    ///     // Moving the call to set after move_program will result
    ///     // in a compiler error, because uniform can't outlive
    ///     // program.
    ///     // uniform.set(Uniforms::Uniform1f(0.5));
    /// }
    /// ```
    pub fn get_uniform<'prog>(&'prog self, name: &str) -> crate::Result<Uniform<'prog>> {
        let c_name = CString::new(name)?;
        let ptr = c_name.as_ptr();

        match unsafe { gl::GetUniformLocation(self.handle, ptr) } {
            -1 => crate::bail!("Uniform with name {} was not found", name),
            id => Ok(Uniform::new(id)),
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handle) }
    }
}
