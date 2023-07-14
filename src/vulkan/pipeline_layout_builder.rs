use ash::vk::DescriptorSetLayout;
use ash::vk::PipelineLayoutCreateInfo;
use ash::vk::PushConstantRange;
use ash::vk::ShaderStageFlags;

use super::descriptor_set_layout::SetUpDescriptorSetLayout;
use super::pipeline_layout::SetUpPipelineLayout;
use super::virtual_device::SetUpVirtualDevice;

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

    pub fn build(self, virtual_device: &SetUpVirtualDevice) -> crate::Result<SetUpPipelineLayout> {
        unsafe {
            let mut pipeline_layout_create_info = PipelineLayoutCreateInfo::builder();
            if !self.push_constant_ranges.is_empty() {
                pipeline_layout_create_info =
                    pipeline_layout_create_info.push_constant_ranges(&self.push_constant_ranges);
            }
            if let Some(descriptor_sets) = self.set_layouts {
                // SAFETY: Transmuting the descriptor_sets slice is safe because
                //         SetUpDescriptorSetLayout is a repr(transparent) struct that holds one
                //         value of DescriptorSetLayout. This guarantees that a slice over
                //         SetUpDescriptorSetLayouts looks the same as a slice over
                //         DescriptorSetLayouts.
                pipeline_layout_create_info =
                    pipeline_layout_create_info.set_layouts(
                        std::mem::transmute::<&[SetUpDescriptorSetLayout], &[DescriptorSetLayout]>(descriptor_sets)
                    );
            }
            SetUpPipelineLayout::create_from_build_info(
                virtual_device,
                *pipeline_layout_create_info,
            )
        }
    }
}