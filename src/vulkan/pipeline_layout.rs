use ash::vk::{PipelineLayout, PipelineLayoutCreateInfo};

use super::logical_device::SetUpLogicalDevice;

pub struct SetUpPipelineLayout {
    pub pipeline_layout: PipelineLayout,
}

impl SetUpPipelineLayout {
    pub unsafe fn create_from_build_info(
        logical_device: &SetUpLogicalDevice,
        pipeline_layout_create_info: PipelineLayoutCreateInfo,
    ) -> crate::Result<Self> {
        let pipeline_layout =
            logical_device.create_pipeline_layout(&pipeline_layout_create_info, None)?;

        Ok(Self { pipeline_layout })
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
