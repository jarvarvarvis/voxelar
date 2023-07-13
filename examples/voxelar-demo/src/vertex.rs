use voxelar_vertex::*;
use voxelar::nalgebra::Vector4;

#[repr(C)]
#[derive(Clone, Debug, Copy, VertexInput)]
#[vertex_input_spec(binding = 0)]
pub struct Vertex {
    #[input(location = 0, binding = 0, format = Format::R32G32B32A32_SFLOAT)]
    pub pos: Vector4<f32>,

    #[input(location = 1, binding = 0, format = Format::R32G32B32A32_SFLOAT)]
    pub color: Vector4<f32>,
}
