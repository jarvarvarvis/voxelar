use ash::vk::DescriptorBufferInfo;
use ash::vk::DescriptorSet;
use ash::vk::DescriptorType;
use ash::vk::WriteDescriptorSet;

use crate::result::Context;

use super::buffer::AllocatedBuffer;
use super::dynamic_descriptor_buffer::DynamicDescriptorBuffer;
use super::typed_buffer::TypedAllocatedBuffer;
use super::virtual_device::SetUpVirtualDevice;

#[derive(Debug)]
pub struct WriteDescriptorSetParams {
    buffer_info: DescriptorBufferInfo,
    destination_binding: u32,
    descriptor_type: DescriptorType,
}

#[derive(Default)]
pub struct DescriptorSetUpdateBuilder {
    write_params: Vec<WriteDescriptorSetParams>,
    destination_set: Option<DescriptorSet>,
}

impl DescriptorSetUpdateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn destination_set(mut self, destination_set: DescriptorSet) -> Self {
        self.destination_set = Some(destination_set);
        self
    }

    pub fn add_buffer_write(
        mut self,
        buffer: &AllocatedBuffer,
        destination_binding: u32,
        descriptor_type: DescriptorType,
        offset: u64,
        range: u64,
    ) -> crate::Result<Self> {
        let descriptor_buffer_info = DescriptorBufferInfo::builder()
            .buffer(buffer.buffer)
            .offset(offset)
            .range(range)
            .build();

        self.write_params.push(WriteDescriptorSetParams {
            buffer_info: descriptor_buffer_info,
            destination_binding,
            descriptor_type,
        });

        Ok(self)
    }

    pub fn add_typed_buffer_write<T>(
        self,
        buffer: &TypedAllocatedBuffer<T>,
        destination_binding: u32,
        descriptor_type: DescriptorType,
    ) -> crate::Result<Self> {
        let range = std::mem::size_of::<T>() as u64;
        self.add_buffer_write(
            &buffer.buffer,
            destination_binding,
            descriptor_type,
            0,
            range,
        )
    }

    pub fn add_dynamic_descriptor_buffer_write<T>(
        self,
        buffer: &DynamicDescriptorBuffer<T>,
        destination_binding: u32,
        descriptor_type: DescriptorType,
    ) -> crate::Result<Self> {
        let range = buffer.aligned_size_of_type;
        self.add_buffer_write(
            &buffer.buffer,
            destination_binding,
            descriptor_type,
            0,
            range,
        )
    }

    pub fn update(self, virtual_device: &SetUpVirtualDevice) -> crate::Result<()> {
        unsafe {
            let mut writes = Vec::with_capacity(self.write_params.len());
            for params in self.write_params.iter() {
                let destination_set = self
                    .destination_set
                    .context("No destination descriptor set specified".to_string())?;
                let write = WriteDescriptorSet::builder()
                    .dst_binding(params.destination_binding)
                    .dst_set(destination_set)
                    .descriptor_type(params.descriptor_type)
                    .buffer_info(std::slice::from_ref(&params.buffer_info))
                    .build();
                writes.push(write);
            }

            virtual_device.device.update_descriptor_sets(&writes, &[]);

            Ok(())
        }
    }
}
