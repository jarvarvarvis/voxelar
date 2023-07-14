use ash::vk::{DescriptorType, DescriptorSetLayout};
use ash::vk::{DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorPoolSize};

use crate::result::Context;

use super::descriptor_set_layout::SetUpDescriptorSetLayout;
use super::descriptor_set_logic::SetUpDescriptorSetLogic;
use super::virtual_device::SetUpVirtualDevice;

#[derive(Default)]
pub struct DescriptorSetLogicBuilder<'builder> {
    pool_sizes: Vec<DescriptorPoolSize>,
    max_sets: u32,
    set_layouts: Option<&'builder [SetUpDescriptorSetLayout]>,
}

impl<'builder> DescriptorSetLogicBuilder<'builder> {
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

    pub fn set_layouts(mut self, set_layouts: &'builder [SetUpDescriptorSetLayout]) -> Self {
        self.set_layouts = Some(set_layouts);
        self
    }

    pub fn build(
        self,
        virtual_device: &SetUpVirtualDevice,
    ) -> crate::Result<SetUpDescriptorSetLogic> {
        unsafe {
            let set_layouts = self
                .set_layouts
                .context("Descriptor set layouts must be set".to_string())?;
            
            let descriptor_pool_create_info = DescriptorPoolCreateInfo::builder()
                .flags(DescriptorPoolCreateFlags::empty())
                .max_sets(self.max_sets)
                .pool_sizes(&self.pool_sizes);

            // SAFETY: Transmuting the set_layouts slice is safe because
            //         SetUpDescriptorSetLayout is a repr(transparent) struct that holds one
            //         value of DescriptorSetLayout. This guarantees that a slice over
            //         SetUpDescriptorSetLayouts has the same memory layout as a slice over
            //         DescriptorSetLayouts.
            SetUpDescriptorSetLogic::create(
                virtual_device,
                *descriptor_pool_create_info,
                std::mem::transmute::<&[SetUpDescriptorSetLayout], &[DescriptorSetLayout]>(set_layouts),
            )
        }
    }
}
