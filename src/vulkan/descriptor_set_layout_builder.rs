use ash::vk::DescriptorType;
use ash::vk::ShaderStageFlags;
use ash::vk::{
    DescriptorSetLayoutBinding, DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo,
};

use super::descriptor_set_layout::SetUpDescriptorSetLayout;
use super::virtual_device::SetUpVirtualDevice;

#[derive(Default)]
pub struct DescriptorSetLayoutBuilder {
    bindings: Vec<DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_binding(
        mut self,
        binding: u32,
        descriptor_count: u32,
        descriptor_type: DescriptorType,
        stage_flags: ShaderStageFlags,
    ) -> Self {
        let descriptor_set_layout_binding = DescriptorSetLayoutBinding::builder()
            .binding(binding)
            .descriptor_count(descriptor_count)
            .descriptor_type(descriptor_type)
            .stage_flags(stage_flags);
        self.bindings.push(*descriptor_set_layout_binding);
        self
    }

    pub fn build(
        self,
        virtual_device: &SetUpVirtualDevice,
    ) -> crate::Result<SetUpDescriptorSetLayout> {
        unsafe {
            let descriptor_set_layout_create_info = DescriptorSetLayoutCreateInfo::builder()
                .flags(DescriptorSetLayoutCreateFlags::empty())
                .bindings(&self.bindings);
            SetUpDescriptorSetLayout::create_from_build_info(
                virtual_device,
                *descriptor_set_layout_create_info,
            )
        }
    }
}
