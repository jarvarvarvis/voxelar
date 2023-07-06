use super::vbo::Vbo;

pub struct Vao {
    handle: u32,
    vbos: Vec<Vbo>,
}

impl Vao {
    pub fn create() -> Self {
        let mut handle = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut handle);
        }

        Self {
            handle,
            vbos: vec![],
        }
    }

    pub fn add_vbo(&mut self, vbo: Vbo) {
        self.vbos.push(vbo);
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.handle as *const _);
        }
    }
}
