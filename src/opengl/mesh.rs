use super::program::Program;
use super::vao::Vao;

pub struct Mesh {
    vao: Vao,
    program: Program,
    vertices: i32,
}

impl Mesh {
    pub fn new(vao: Vao, program: Program, vertices: i32) -> Self {
        Self {
            vao,
            program,
            vertices,
        }
    }

    pub fn draw(&self) {
        self.program.bind();
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices);
        }
        self.vao.unbind();
        self.program.unbind();
    }
}
