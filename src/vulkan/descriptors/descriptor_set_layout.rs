use ash::vk::{DescriptorSetLayout, DescriptorSetLayoutCreateInfo};

use crate::vulkan::logical_device::SetUpLogicalDevice;

#[repr(transparent)]
pub struct SetUpDescriptorSetLayout {
    pub descriptor_set_layout: DescriptorSetLayout,
}

impl SetUpDescriptorSetLayout {
    pub unsafe fn create_from_build_info(
        logical_device: &SetUpLogicalDevice,
        descriptor_set_layout_create_info: DescriptorSetLayoutCreateInfo,
    ) -> crate::Result<Self> {
        let descriptor_set_layout = logical_device
            .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)?;

        Ok(Self {
            descriptor_set_layout,
        })
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
    }
}
