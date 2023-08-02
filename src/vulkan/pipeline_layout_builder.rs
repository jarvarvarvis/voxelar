use ash::vk::DescriptorSetLayout;
use ash::vk::PipelineLayoutCreateInfo;
use ash::vk::PushConstantRange;
use ash::vk::ShaderStageFlags;

use crate::vulkan::descriptors::descriptor_set_layout::SetUpDescriptorSetLayout;

use super::logical_device::SetUpLogicalDevice;
use super::pipeline_layout::SetUpPipelineLayout;

#[derive(Default)]
pub struct PipelineLayoutBuilder<'builder> {
    push_constant_ranges: Vec<PushConstantRange>,
    set_layouts: Option<&'builder [SetUpDescriptorSetLayout]>,
}

impl<'builder> PipelineLayoutBuilder<'builder> {
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

    pub fn set_layouts(mut self, set_layouts: &'builder [SetUpDescriptorSetLayout]) -> Self {
        self.set_layouts = Some(set_layouts);
        self
    }

    pub fn build(self, logical_device: &SetUpLogicalDevice) -> crate::Result<SetUpPipelineLayout> {
        unsafe {
            let mut pipeline_layout_create_info = PipelineLayoutCreateInfo::builder();
            if !self.push_constant_ranges.is_empty() {
                pipeline_layout_create_info =
                    pipeline_layout_create_info.push_constant_ranges(&self.push_constant_ranges);
            }
            if let Some(set_layouts) = self.set_layouts {
                // SAFETY: Transmuting the set_layouts slice is safe because
                //         SetUpDescriptorSetLayout is a repr(transparent) struct that holds one
                //         value of DescriptorSetLayout. This guarantees that a slice over
                //         SetUpDescriptorSetLayouts has the same memory layout as a slice over
                //         DescriptorSetLayouts.
                pipeline_layout_create_info =
                    pipeline_layout_create_info.set_layouts(std::mem::transmute::<
                        &[SetUpDescriptorSetLayout],
                        &[DescriptorSetLayout],
                    >(set_layouts));
            }
            SetUpPipelineLayout::create_from_build_info(
                logical_device,
                *pipeline_layout_create_info,
            )
        }
    }
}
