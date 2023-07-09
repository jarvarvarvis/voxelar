use voxelar::voxelar_math::vec4::Vec4;

#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub pos: Vec4<f32>,
    pub color: Vec4<f32>
}
