use ash::vk::{DescriptorPool, DescriptorPoolCreateInfo, DescriptorSetLayout};
use ash::vk::{DescriptorSet, DescriptorSetAllocateInfo};

use super::logical_device::SetUpLogicalDevice;

pub struct SetUpDescriptorSetLogic {
    pub descriptor_pool: DescriptorPool,
    pub descriptor_sets: Vec<DescriptorSet>,
}

impl SetUpDescriptorSetLogic {
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        descriptor_pool_create_info: DescriptorPoolCreateInfo,
        set_layouts: &[DescriptorSetLayout],
    ) -> crate::Result<Self> {
        let descriptor_pool =
            logical_device.create_descriptor_pool(&descriptor_pool_create_info, None)?;

        let descriptor_set_allocate_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&set_layouts);

        let descriptor_sets =
            logical_device.allocate_descriptor_sets(&descriptor_set_allocate_info)?;

        Ok(Self {
            descriptor_pool,
            descriptor_sets,
        })
    }

    pub fn get_set(&self, index: usize) -> &DescriptorSet {
        &self.descriptor_sets[index]
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
