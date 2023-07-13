use voxelar_vertex::*;
use voxelar::nalgebra::Vector3;

#[repr(C)]
#[derive(Clone, Debug, Copy, VertexInput)]
#[vertex_input_spec(binding = 0)]
pub struct Vertex {
    #[input(location = 0, binding = 0, format = Format::R32G32B32_SFLOAT)]
    pub pos: Vector3<f32>,
}
