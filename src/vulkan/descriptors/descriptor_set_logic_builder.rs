use ash::vk::{DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorPoolSize};
use ash::vk::{DescriptorSetLayout, DescriptorType};

use crate::result::Context;
use crate::vulkan::logical_device::SetUpLogicalDevice;

use super::descriptor_set_layout::SetUpDescriptorSetLayout;
use super::descriptor_set_logic::SetUpDescriptorSetLogic;

#[derive(Default)]
pub struct DescriptorSetLogicBuilder<'builder> {
    pool_sizes: Vec<DescriptorPoolSize>,
    set_layouts: Option<&'builder [SetUpDescriptorSetLayout]>,
}

impl<'builder> DescriptorSetLogicBuilder<'builder> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pool_size(mut self, descriptor_type: DescriptorType, descriptor_count: u32) -> Self {
        let pool_size = DescriptorPoolSize::builder()
            .ty(descriptor_type)
            .descriptor_count(descriptor_count)
            .build();
        self.pool_sizes.push(pool_size);
        self
    }

    pub fn set_layouts(mut self, set_layouts: &'builder [SetUpDescriptorSetLayout]) -> Self {
        self.set_layouts = Some(set_layouts);
        self
    }

    pub fn build(
        self,
        logical_device: &SetUpLogicalDevice,
    ) -> crate::Result<SetUpDescriptorSetLogic> {
        unsafe {
            let set_layouts = self
                .set_layouts
                .context("Descriptor set layouts must be set".to_string())?;

            let descriptor_pool_create_info = DescriptorPoolCreateInfo::builder()
                .flags(DescriptorPoolCreateFlags::empty())
                .max_sets(set_layouts.len() as u32)
                .pool_sizes(&self.pool_sizes);

            // SAFETY: Transmuting the set_layouts slice is safe because
            //         SetUpDescriptorSetLayout is a repr(transparent) struct that holds one
            //         value of DescriptorSetLayout. This guarantees that a slice over
            //         SetUpDescriptorSetLayouts has the same memory layout as a slice over
            //         DescriptorSetLayouts.
            SetUpDescriptorSetLogic::create(
                logical_device,
                *descriptor_pool_create_info,
                std::mem::transmute::<&[SetUpDescriptorSetLayout], &[DescriptorSetLayout]>(
                    set_layouts,
                ),
            )
        }
    }
}
