use ash::vk::PushConstantRange;
use ash::vk::ShaderStageFlags;
use ash::vk::{PipelineLayout, PipelineLayoutCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpPipelineLayout {
    pub pipeline_layout: PipelineLayout,
}

impl SetUpPipelineLayout {
    pub unsafe fn create(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        let pipeline_layout_create_info = PipelineLayoutCreateInfo::default();

        let pipeline_layout = virtual_device
            .device
            .create_pipeline_layout(&pipeline_layout_create_info, None)?;

        Ok(Self { pipeline_layout })
    }

    pub unsafe fn create_with_push_constants<PushConstants>(
        virtual_device: &SetUpVirtualDevice,
        shader_stages: ShaderStageFlags,
    ) -> crate::Result<Self> {
        let push_constant = PushConstantRange::builder()
            .offset(0)
            .size(std::mem::size_of::<PushConstants>() as u32)
            .stage_flags(shader_stages)
            .build();
        let push_constant_ranges = &[push_constant];

        let pipeline_layout_create_info =
            PipelineLayoutCreateInfo::builder().push_constant_ranges(push_constant_ranges);

        let pipeline_layout = virtual_device
            .device
            .create_pipeline_layout(&pipeline_layout_create_info, None)?;

        Ok(Self { pipeline_layout })
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
