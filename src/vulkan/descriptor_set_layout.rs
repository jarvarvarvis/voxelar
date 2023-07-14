use ash::vk::{DescriptorSetLayout, DescriptorSetLayoutCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

#[repr(transparent)]
pub struct SetUpDescriptorSetLayout {
    pub descriptor_set_layout: DescriptorSetLayout,
}

impl SetUpDescriptorSetLayout {
    pub unsafe fn create_from_build_info(
        virtual_device: &SetUpVirtualDevice,
        descriptor_set_layout_create_info: DescriptorSetLayoutCreateInfo,
    ) -> crate::Result<Self> {
        let descriptor_set_layout = virtual_device
            .device
            .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)?;

        Ok(Self {
            descriptor_set_layout,
        })
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
    }
}
