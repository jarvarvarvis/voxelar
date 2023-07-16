use ash::vk::{DescriptorPool, DescriptorPoolCreateInfo, DescriptorSetLayout};
use ash::vk::{DescriptorSet, DescriptorSetAllocateInfo};

use super::descriptor_set::SetUpDescriptorSet;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDescriptorSetLogic {
    pub descriptor_pool: DescriptorPool,
    pub descriptor_sets: Vec<SetUpDescriptorSet>,
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

        let vk_descriptor_sets = virtual_device
            .device
            .allocate_descriptor_sets(&descriptor_set_allocate_info)?;

        let mut descriptor_sets = Vec::with_capacity(vk_descriptor_sets.len());
        for descriptor_set in vk_descriptor_sets {
            descriptor_sets.push(SetUpDescriptorSet::create(descriptor_set)?);
        }

        Ok(Self {
            descriptor_pool,
            descriptor_sets,
        })
    }

    pub fn get_set(&self, index: usize) -> &SetUpDescriptorSet {
        &self.descriptor_sets[index]
    }

    pub fn get_descriptor_sets(&self) -> &[DescriptorSet] {
        unsafe {
            std::mem::transmute::<&[SetUpDescriptorSet], &[DescriptorSet]>(&self.descriptor_sets)
        }
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
