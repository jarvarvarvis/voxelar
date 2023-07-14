use ash::vk::DescriptorType;
use ash::vk::{DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorPoolSize};

use super::descriptor_pool::SetUpDescriptorPool;
use super::virtual_device::SetUpVirtualDevice;

#[derive(Default)]
pub struct DescriptorPoolBuilder {
    pool_sizes: Vec<DescriptorPoolSize>,
    max_sets: u32,
}

impl DescriptorPoolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pool_size(mut self, descriptor_type: DescriptorType, descriptor_count: u32) -> Self {
        let pool_size = DescriptorPoolSize::builder()
            .ty(descriptor_type)
            .descriptor_count(descriptor_count);
        self.pool_sizes.push(*pool_size);
        self
    }

    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.max_sets = max_sets;
        self
    }

    pub fn build(self, virtual_device: &SetUpVirtualDevice) -> crate::Result<SetUpDescriptorPool> {
        unsafe {
            let descriptor_pool_create_info = DescriptorPoolCreateInfo::builder()
                .flags(DescriptorPoolCreateFlags::empty())
                .max_sets(self.max_sets)
                .pool_sizes(&self.pool_sizes);
            SetUpDescriptorPool::create_from_build_info(
                virtual_device,
                *descriptor_pool_create_info,
            )
        }
    }
}
