extern crate voxelar_vertex_derive;

pub use voxelar_vertex_derive::*;

pub use ash::vk::Format;
pub use ash::vk::PipelineInputAssemblyStateCreateInfo;
pub use ash::vk::PipelineVertexInputStateCreateInfo;
pub use ash::vk::PrimitiveTopology;
pub use ash::vk::VertexInputAttributeDescription;
pub use ash::vk::VertexInputBindingDescription;
pub use ash::vk::VertexInputRate;

/// This structure is returned from VertexInput::input_state_info() to retain the data passed
/// to the PipelineVertexInputStateCreateInfo. Otherwise, the memory would be destroyed in the call
/// to VertexInput::input_state_info().
pub struct VertexInputStateInfoConstructionData {
    pub vertex_input_binding_descriptions: Vec<VertexInputBindingDescription>,
    pub vertex_input_attribute_descriptions: Vec<VertexInputAttributeDescription>
}

pub trait VertexInput {
    fn input_state_info() -> (VertexInputStateInfoConstructionData, PipelineVertexInputStateCreateInfo);
}

// Simple offset_of macro akin to C++ offsetof
#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
        }
    }};
}
