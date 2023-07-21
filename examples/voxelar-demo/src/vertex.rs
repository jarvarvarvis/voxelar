use voxelar_vertex::*;
use voxelar::nalgebra::Vector3;

#[repr(C)]
#[derive(Clone, Debug, Copy, VertexInput)]
pub struct VertexPosition {
    #[input(location = 0, format = Format::R32G32B32_SFLOAT)]
    pub pos: Vector3<f32>,
}

#[repr(C)]
#[derive(Clone, Debug, Copy, VertexInput)]
pub struct VertexColor {
    #[input(location = 1, format = Format::R32G32B32_SFLOAT)]
    pub color: Vector3<f32>,
}
