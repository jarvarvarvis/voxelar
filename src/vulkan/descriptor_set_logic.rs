use ash::vk::{DescriptorPool, DescriptorPoolCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDescriptorSetLogic {
    pub descriptor_pool: DescriptorPool,
}

impl SetUpDescriptorSetLogic {
    pub unsafe fn create_from_build_info(
        virtual_device: &SetUpVirtualDevice,
        descriptor_pool_create_info: DescriptorPoolCreateInfo,
    ) -> crate::Result<Self> {
        let descriptor_pool = virtual_device
            .device
            .create_descriptor_pool(&descriptor_pool_create_info, None)?;

        Ok(Self { descriptor_pool })
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
