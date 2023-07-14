use ash::vk::DescriptorBufferInfo;
use ash::vk::{DescriptorSet, DescriptorType, WriteDescriptorSet};

use super::typed_buffer::TypedAllocatedBuffer;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDescriptorSet {
    pub descriptor_set: DescriptorSet,
}

impl SetUpDescriptorSet {
    pub unsafe fn create(descriptor_set: DescriptorSet) -> crate::Result<Self> {
        Ok(Self { descriptor_set })
    }

    pub fn attach_buffer_to_descriptor<T>(
        &self,
        virtual_device: &SetUpVirtualDevice,
        buffer: &TypedAllocatedBuffer<T>,
        buffer_offset: u64,
        destination_binding: u32,
        descriptor_type: DescriptorType,
    ) -> crate::Result<()> {
        unsafe {
            let descriptor_buffer_info = DescriptorBufferInfo::builder()
                .buffer(buffer.raw_buffer())
                .offset(buffer_offset)
                .range(std::mem::size_of::<T>() as u64)
                .build();

            let descriptor_write = WriteDescriptorSet::builder()
                .dst_binding(destination_binding)
                .dst_set(self.descriptor_set)
                .descriptor_type(descriptor_type)
                .buffer_info(&[descriptor_buffer_info])
                .build();

            virtual_device
                .device
                .update_descriptor_sets(&[descriptor_write], &[]);

            Ok(())
        }
    }

    pub fn attach_uniform_buffer_to_descriptor<T>(
        &self,
        virtual_device: &SetUpVirtualDevice,
        buffer: &TypedAllocatedBuffer<T>,
        buffer_offset: u64,
        destination_binding: u32,
    ) -> crate::Result<()> {
        self.attach_buffer_to_descriptor(
            virtual_device,
            buffer,
            buffer_offset,
            destination_binding,
            DescriptorType::UNIFORM_BUFFER,
        )
    }
}
