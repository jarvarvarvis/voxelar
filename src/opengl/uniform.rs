use std::marker::PhantomData;

use gl::types::*;
use voxelar_math::vec2::Vec2;
use voxelar_math::vec3::Vec3;
use voxelar_math::vec4::Vec4;

pub enum Uniforms {
    // Uniform1_
    Uniform1d(GLdouble),
    Uniform1dv(Vec<GLdouble>),
    Uniform1f(GLfloat),
    Uniform1fv(Vec<GLfloat>),
    Uniform1i(GLint),
    Uniform1iv(Vec<GLint>),
    Uniform1ui(GLuint),
    Uniform1uiv(Vec<GLuint>),

    // Uniform2_
    Uniform2d(Vec2<GLdouble>),
    Uniform2dv(Vec<Vec2<GLdouble>>),
    Uniform2f(Vec2<GLfloat>),
    Uniform2fv(Vec<Vec2<GLfloat>>),
    Uniform2i(Vec2<GLint>),
    Uniform2iv(Vec<Vec2<GLint>>),
    Uniform2ui(Vec2<GLuint>),
    Uniform2uiv(Vec<Vec2<GLuint>>),

    // Uniform3_
    Uniform3d(Vec3<GLdouble>),
    Uniform3dv(Vec<Vec3<GLdouble>>),
    Uniform3f(Vec3<GLfloat>),
    Uniform3fv(Vec<Vec3<GLfloat>>),
    Uniform3i(Vec3<GLint>),
    Uniform3iv(Vec<Vec3<GLint>>),
    Uniform3ui(Vec3<GLuint>),
    Uniform3uiv(Vec<Vec3<GLuint>>),

    // Uniform4_
    Uniform4d(Vec4<GLdouble>),
    Uniform4dv(Vec<Vec4<GLdouble>>),
    Uniform4f(Vec4<GLfloat>),
    Uniform4fv(Vec<Vec4<GLfloat>>),
    Uniform4i(Vec4<GLint>),
    Uniform4iv(Vec<Vec4<GLint>>),
    Uniform4ui(Vec4<GLuint>),
    Uniform4uiv(Vec<Vec4<GLuint>>),

    // UniformMatrix2_
    UniformMatrix2dv(),
    UniformMatrix2fv(),
    UniformMatrix2x3dv(),
    UniformMatrix2x3fv(),
    UniformMatrix2x4dv(),
    UniformMatrix2x4fv(),

    // UniformMatrix3_
    UniformMatrix3dv(),
    UniformMatrix3fv(),
    UniformMatrix3x2dv(),
    UniformMatrix3x2fv(),
    UniformMatrix3x4dv(),
    UniformMatrix3x4fv(),

    // UniformMatrix4_
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
                // Uniform1_
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

                // Uniform2_
                Uniforms::Uniform2d(value) => gl::Uniform2d(location, value.x(), value.y()),
                Uniforms::Uniform2dv(value) => gl::Uniform2dv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLdouble,
                ),
                Uniforms::Uniform2f(value) => gl::Uniform2f(location, value.x(), value.y()),
                Uniforms::Uniform2fv(value) => gl::Uniform2fv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLfloat,
                ),
                Uniforms::Uniform2i(value) => gl::Uniform2i(location, value.x(), value.y()),
                Uniforms::Uniform2iv(value) => {
                    gl::Uniform2iv(location, value.len() as i32, value.as_ptr() as *const GLint)
                }
                Uniforms::Uniform2ui(value) => gl::Uniform2ui(location, value.x(), value.y()),
                Uniforms::Uniform2uiv(value) => gl::Uniform2uiv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLuint,
                ),

                // Uniform3_
                Uniforms::Uniform3d(value) => {
                    gl::Uniform3d(location, value.x(), value.y(), value.z())
                }
                Uniforms::Uniform3dv(value) => gl::Uniform3dv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLdouble,
                ),
                Uniforms::Uniform3f(value) => {
                    gl::Uniform3f(location, value.x(), value.y(), value.z())
                }
                Uniforms::Uniform3fv(value) => gl::Uniform3fv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLfloat,
                ),
                Uniforms::Uniform3i(value) => {
                    gl::Uniform3i(location, value.x(), value.y(), value.z())
                }
                Uniforms::Uniform3iv(value) => {
                    gl::Uniform3iv(location, value.len() as i32, value.as_ptr() as *const GLint)
                }
                Uniforms::Uniform3ui(value) => {
                    gl::Uniform3ui(location, value.x(), value.y(), value.z())
                }
                Uniforms::Uniform3uiv(value) => gl::Uniform3uiv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLuint,
                ),

                // Uniform4_
                Uniforms::Uniform4d(value) => {
                    gl::Uniform4d(location, value.x(), value.y(), value.z(), value.w())
                }
                Uniforms::Uniform4dv(value) => gl::Uniform4dv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLdouble,
                ),
                Uniforms::Uniform4f(value) => {
                    gl::Uniform4f(location, value.x(), value.y(), value.z(), value.w())
                }
                Uniforms::Uniform4fv(value) => gl::Uniform4fv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLfloat,
                ),
                Uniforms::Uniform4i(value) => {
                    gl::Uniform4i(location, value.x(), value.y(), value.z(), value.w())
                }
                Uniforms::Uniform4iv(value) => {
                    gl::Uniform4iv(location, value.len() as i32, value.as_ptr() as *const GLint)
                }
                Uniforms::Uniform4ui(value) => {
                    gl::Uniform4ui(location, value.x(), value.y(), value.z(), value.w())
                }
                Uniforms::Uniform4uiv(value) => gl::Uniform4uiv(
                    location,
                    value.len() as i32,
                    value.as_ptr() as *const GLuint,
                ),

                // UniformMatrix2_
                Uniforms::UniformMatrix2dv() => todo!(),
                Uniforms::UniformMatrix2fv() => todo!(),
                Uniforms::UniformMatrix2x3dv() => todo!(),
                Uniforms::UniformMatrix2x3fv() => todo!(),
                Uniforms::UniformMatrix2x4dv() => todo!(),
                Uniforms::UniformMatrix2x4fv() => todo!(),

                // UniformMatrix3_
                Uniforms::UniformMatrix3dv() => todo!(),
                Uniforms::UniformMatrix3fv() => todo!(),
                Uniforms::UniformMatrix3x2dv() => todo!(),
                Uniforms::UniformMatrix3x2fv() => todo!(),
                Uniforms::UniformMatrix3x4dv() => todo!(),
                Uniforms::UniformMatrix3x4fv() => todo!(),

                // UniformMatrix4_
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
