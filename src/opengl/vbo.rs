use std::{ffi::c_void, mem, ptr};

use gl::types::{GLenum, GLfloat, GLsizei, GLsizeiptr};

pub struct Vbo {
    handle: u32,
}

impl Vbo {
    pub fn create() -> Self {
        let mut handle = 0;
        unsafe {
            gl::GenBuffers(1, &mut handle);
        }

        Self { handle }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn upload_data(&self, vertices: Vec<GLfloat>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn vertex_attrib(&self, id: u32, size: i32, ty: GLenum) {
        unsafe {
            gl::VertexAttribPointer(
                id,
                size,
                ty,
                gl::FALSE,
                size * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(id);
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle as *const _);
        }
    }
}
