use voxelar::nalgebra::Vector3;
use voxelar_vertex::*;

#[repr(C)]
#[derive(Clone, Debug, Copy, VertexInput)]
pub struct VertexData {
    #[input(location = 0, format = Format::R32G32B32_SFLOAT)]
    pub pos: Vector3<f32>,
}
