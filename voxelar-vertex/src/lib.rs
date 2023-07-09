extern crate voxelar_vertex_derive;

pub use voxelar_vertex_derive::*;

pub use ash::vk::PipelineVertexInputStateCreateInfo;
pub use ash::vk::PipelineInputAssemblyStateCreateInfo;

pub trait VertexInput {
    fn input_state_info() -> PipelineVertexInputStateCreateInfo;
    fn input_assembly_state_info() -> PipelineInputAssemblyStateCreateInfo;
}
