use voxelar::voxelar_math::vec4::Vec4;

use voxelar_vertex::*;

#[repr(C)]
#[derive(Clone, Debug, Copy, VertexInput)]
#[vertex_input_spec(binding = 0)]
pub struct Vertex {
    #[input(location = 0, binding = 0, format = Format::R32G32B32A32_SFLOAT)]
    pub pos: Vec4<f32>,

    #[input(location = 1, binding = 0, format = Format::R32G32B32A32_SFLOAT)]
    pub color: Vec4<f32>,
}
