use std::marker::PhantomData;

use gl::types::*;
use voxelar_math::vec2::Vec2;
use voxelar_math::vec3::Vec3;

pub enum Uniforms {
    Uniform1d(GLdouble),
    Uniform1dv(Vec<GLdouble>),
    Uniform1f(GLfloat),
    Uniform1fv(Vec<GLfloat>),
    Uniform1i(GLint),
    Uniform1iv(Vec<GLint>),
    Uniform1ui(GLuint),
    Uniform1uiv(Vec<GLuint>),
    Uniform2d(Vec2<GLdouble>),
    Uniform2dv(Vec<Vec2<GLdouble>>),
    Uniform2f(),
    Uniform2fv(),
    Uniform2i(),
    Uniform2iv(),
    Uniform2ui(),
    Uniform2uiv(),
    Uniform3d(),
    Uniform3dv(),
    Uniform3f(Vec3<GLfloat>),
    Uniform3fv(Vec<Vec3<GLfloat>>),
    Uniform3i(),
    Uniform3iv(),
    Uniform3ui(),
    Uniform3uiv(),
    Uniform4d(),
    Uniform4dv(),
    Uniform4f(),
    Uniform4fv(),
    Uniform4i(),
    Uniform4iv(),
    Uniform4ui(),
    Uniform4uiv(),
    UniformMatrix2dv(),
    UniformMatrix2fv(),
    UniformMatrix2x3dv(),
    UniformMatrix2x3fv(),
    UniformMatrix2x4dv(),
    UniformMatrix2x4fv(),
    UniformMatrix3dv(),
    UniformMatrix3fv(),
    UniformMatrix3x2dv(),
    UniformMatrix3x2fv(),
    UniformMatrix3x4dv(),
    UniformMatrix3x4fv(),
    UniformMatrix4dv(),
    UniformMatrix4fv(),
    UniformMatrix4x2dv(),
    UniformMatrix4x2fv(),
    UniformMatrix4x3dv(),
    UniformMatrix4x3fv(),
}

impl Uniforms {
    pub fn set_for_location(self, location: GLint) {
        unsafe {
            match self {
                Uniforms::Uniform1d(value) => gl::Uniform1d(location, value),
                Uniforms::Uniform1dv(value) => {
                    gl::Uniform1dv(location, value.len() as i32, value.as_ptr())
                }
                Uniforms::Uniform1f(value) => gl::Uniform1f(location, value),
                Uniforms::Uniform1fv(value) => {
                    gl::Uniform1fv(location, value.len() as i32, value.as_ptr())
                }
                Uniforms::Uniform1i(value) => gl::Uniform1i(location, value),
                Uniforms::Uniform1iv(value) => {
                    gl::Uniform1iv(location, value.len() as i32, value.as_ptr())
                }
                Uniforms::Uniform1ui(value) => gl::Uniform1ui(location, value),
                Uniforms::Uniform1uiv(value) => {
                    gl::Uniform1uiv(location, value.len() as i32, value.as_ptr())
                }
                Uniforms::Uniform2d(value) => gl::Uniform2d(location, value.x(), value.y()),
                Uniforms::Uniform2dv(value) => gl::Uniform2dv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLdouble,
                ), // TODO: Is this safe?
                Uniforms::Uniform2f() => todo!(),
                Uniforms::Uniform2fv() => todo!(),
                Uniforms::Uniform2i() => todo!(),
                Uniforms::Uniform2iv() => todo!(),
                Uniforms::Uniform2ui() => todo!(),
                Uniforms::Uniform2uiv() => todo!(),
                Uniforms::Uniform3d() => todo!(),
                Uniforms::Uniform3dv() => todo!(),
                Uniforms::Uniform3f(value) => {
                    gl::Uniform3f(location, value.x(), value.y(), value.z())
                }
                Uniforms::Uniform3fv(value) => gl::Uniform3fv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLfloat,
                ), // TODO: Is this safe?
                Uniforms::Uniform3i() => todo!(),
                Uniforms::Uniform3iv() => todo!(),
                Uniforms::Uniform3ui() => todo!(),
                Uniforms::Uniform3uiv() => todo!(),
                Uniforms::Uniform4d() => todo!(),
                Uniforms::Uniform4dv() => todo!(),
                Uniforms::Uniform4f() => todo!(),
                Uniforms::Uniform4fv() => todo!(),
                Uniforms::Uniform4i() => todo!(),
                Uniforms::Uniform4iv() => todo!(),
                Uniforms::Uniform4ui() => todo!(),
                Uniforms::Uniform4uiv() => todo!(),
                Uniforms::UniformMatrix2dv() => todo!(),
                Uniforms::UniformMatrix2fv() => todo!(),
                Uniforms::UniformMatrix2x3dv() => todo!(),
                Uniforms::UniformMatrix2x3fv() => todo!(),
                Uniforms::UniformMatrix2x4dv() => todo!(),
                Uniforms::UniformMatrix2x4fv() => todo!(),
                Uniforms::UniformMatrix3dv() => todo!(),
                Uniforms::UniformMatrix3fv() => todo!(),
                Uniforms::UniformMatrix3x2dv() => todo!(),
                Uniforms::UniformMatrix3x2fv() => todo!(),
                Uniforms::UniformMatrix3x4dv() => todo!(),
                Uniforms::UniformMatrix3x4fv() => todo!(),
                Uniforms::UniformMatrix4dv() => todo!(),
                Uniforms::UniformMatrix4fv() => todo!(),
                Uniforms::UniformMatrix4x2dv() => todo!(),
                Uniforms::UniformMatrix4x2fv() => todo!(),
                Uniforms::UniformMatrix4x3dv() => todo!(),
                Uniforms::UniformMatrix4x3fv() => todo!(),
            }
        }
    }
}

pub struct Uniform<'prog> {
    pub location: GLint,
    phantom: PhantomData<&'prog ()>,
}

impl Uniform<'_> {
    pub fn new(location: GLint) -> Self {
        Self {
            location,
            phantom: PhantomData,
        }
    }

    pub fn set(&mut self, kind: Uniforms) {
        kind.set_for_location(self.location);
    }
}
