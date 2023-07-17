use ash::vk::{DescriptorPool, DescriptorPoolCreateInfo, DescriptorSetLayout};
use ash::vk::{DescriptorSet, DescriptorSetAllocateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDescriptorSetLogic {
    pub descriptor_pool: DescriptorPool,
    pub descriptor_sets: Vec<DescriptorSet>,
}

impl SetUpDescriptorSetLogic {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        descriptor_pool_create_info: DescriptorPoolCreateInfo,
        set_layouts: &[DescriptorSetLayout],
    ) -> crate::Result<Self> {
        let descriptor_pool = virtual_device
            .device
            .create_descriptor_pool(&descriptor_pool_create_info, None)?;

        let descriptor_set_allocate_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&set_layouts);

        let descriptor_sets = virtual_device
            .device
            .allocate_descriptor_sets(&descriptor_set_allocate_info)?;

        Ok(Self {
            descriptor_pool,
            descriptor_sets,
        })
    }

    pub fn get_set(&self, index: usize) -> &DescriptorSet {
        &self.descriptor_sets[index]
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
