use ash::vk::{PipelineLayout, PipelineLayoutCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpPipelineLayout {
    pub pipeline_layout: PipelineLayout,
}

impl SetUpPipelineLayout {
    pub unsafe fn create_from_build_info(
        virtual_device: &SetUpVirtualDevice,
        pipeline_layout_create_info: PipelineLayoutCreateInfo,
    ) -> crate::Result<Self> {
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
