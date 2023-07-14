use ash::vk::PipelineLayoutCreateInfo;
use ash::vk::PushConstantRange;
use ash::vk::ShaderStageFlags;

use super::pipeline_layout::SetUpPipelineLayout;
use super::virtual_device::SetUpVirtualDevice;

#[derive(Default)]
pub struct PipelineLayoutBuilder {
    push_constant_ranges: Vec<PushConstantRange>,
}

impl PipelineLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_push_constant_range<PushConstants>(
        mut self,
        offset: u32,
        shader_stages: ShaderStageFlags,
    ) -> Self {
        let push_constant = PushConstantRange::builder()
            .offset(offset)
            .size(std::mem::size_of::<PushConstants>() as u32)
            .stage_flags(shader_stages)
            .build();
        self.push_constant_ranges.push(push_constant);
        self
    }

    pub fn build(self, virtual_device: &SetUpVirtualDevice) -> crate::Result<SetUpPipelineLayout> {
        unsafe {
            let mut pipeline_layout_create_info = PipelineLayoutCreateInfo::builder();
            if !self.push_constant_ranges.is_empty() {
                pipeline_layout_create_info = pipeline_layout_create_info
                    .push_constant_ranges(&self.push_constant_ranges);
            }
            SetUpPipelineLayout::create_from_build_info(virtual_device, *pipeline_layout_create_info)
        }
    }
}
