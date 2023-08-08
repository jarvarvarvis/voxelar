use ash::vk::DescriptorBufferInfo;
use ash::vk::DescriptorSet;
use ash::vk::DescriptorType;
use ash::vk::WriteDescriptorSet;
use ash::vk::{DescriptorImageInfo, ImageLayout};

use crate::vulkan::buffers::buffer::AllocatedBuffer;
use crate::vulkan::buffers::storage_buffer::SetUpStorageBuffer;
use crate::vulkan::buffers::typed_buffer::TypedAllocatedBuffer;
use crate::vulkan::buffers::uniform_buffer::SetUpUniformBuffer;
use crate::vulkan::image::image_view::SetUpImageView;
use crate::vulkan::image::sampler::SetUpSampler;
use crate::vulkan::image::texture::Texture;
use crate::vulkan::logical_device::SetUpLogicalDevice;

#[derive(Debug)]
pub struct WriteBufferDescriptorSetParams {
    buffer_info: DescriptorBufferInfo,
    destination_binding: u32,
    descriptor_type: DescriptorType,
}

#[derive(Debug)]
pub struct WriteImageDescriptorSetParams {
    image_info: DescriptorImageInfo,
    destination_binding: u32,
    descriptor_type: DescriptorType,
}

#[derive(Default)]
pub struct DescriptorSetUpdateBuilder {
    buffer_write_params: Vec<WriteBufferDescriptorSetParams>,
    image_write_params: Vec<WriteImageDescriptorSetParams>,
}

impl DescriptorSetUpdateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_buffer_descriptor(
        mut self,
        buffer: &AllocatedBuffer,
        destination_binding: u32,
        descriptor_type: DescriptorType,
        offset: usize,
        range: usize,
    ) -> Self {
        let descriptor_buffer_info = DescriptorBufferInfo::builder()
            .buffer(buffer.buffer)
            .offset(offset as u64)
            .range(range as u64);

        self.buffer_write_params
            .push(WriteBufferDescriptorSetParams {
                buffer_info: *descriptor_buffer_info,
                destination_binding,
                descriptor_type,
            });

        self
    }

    pub fn add_typed_buffer_descriptor<T>(
        self,
        buffer: &TypedAllocatedBuffer<T>,
        destination_binding: u32,
        descriptor_type: DescriptorType,
    ) -> Self {
        let range = std::mem::size_of::<T>() * buffer.element_amount;
        self.add_buffer_descriptor(&buffer, destination_binding, descriptor_type, 0, range)
    }

    pub fn add_uniform_buffer_descriptor<T>(
        self,
        buffer: &SetUpUniformBuffer<T>,
        destination_binding: u32,
    ) -> Self {
        let descriptor_type = DescriptorType::UNIFORM_BUFFER;
        let range = buffer.aligned_size_of_type * buffer.element_amount;
        self.add_buffer_descriptor(&buffer, destination_binding, descriptor_type, 0, range)
    }

    pub fn add_dynamic_uniform_buffer_descriptor<T>(
        self,
        buffer: &SetUpUniformBuffer<T>,
        destination_binding: u32,
    ) -> Self {
        let descriptor_type = DescriptorType::UNIFORM_BUFFER_DYNAMIC;
        let range = buffer.aligned_size_of_type;
        self.add_buffer_descriptor(&buffer, destination_binding, descriptor_type, 0, range)
    }

    pub fn add_storage_buffer_descriptor<T>(
        self,
        buffer: &SetUpStorageBuffer<T>,
        destination_binding: u32,
    ) -> Self {
        let descriptor_type = DescriptorType::STORAGE_BUFFER;
        let range = buffer.aligned_size_of_type * buffer.element_amount;
        self.add_buffer_descriptor(&buffer, destination_binding, descriptor_type, 0, range)
    }

    pub fn add_dynamic_storage_buffer_descriptor<T>(
        self,
        buffer: &SetUpStorageBuffer<T>,
        destination_binding: u32,
    ) -> Self {
        let descriptor_type = DescriptorType::STORAGE_BUFFER_DYNAMIC;
        let range = buffer.aligned_size_of_type;
        self.add_buffer_descriptor(&buffer, destination_binding, descriptor_type, 0, range)
    }

    pub fn add_image_descriptor(
        mut self,
        sampler: &SetUpSampler,
        image_view: &SetUpImageView,
        destination_binding: u32,
        descriptor_type: DescriptorType,
    ) -> Self {
        let descriptor_image_info = DescriptorImageInfo::builder()
            .sampler(sampler.sampler)
            .image_view(image_view.image_view)
            .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL);

        self.image_write_params.push(WriteImageDescriptorSetParams {
            image_info: *descriptor_image_info,
            destination_binding,
            descriptor_type,
        });

        self
    }

    pub fn add_texture_descriptor<T>(
        self,
        sampler: &SetUpSampler,
        texture: &Texture<T>,
        destination_binding: u32,
        descriptor_type: DescriptorType,
    ) -> Self {
        self.add_image_descriptor(
            sampler,
            &texture.image_view,
            destination_binding,
            descriptor_type,
        )
    }

    pub fn update(self, logical_device: &SetUpLogicalDevice, destination_set: &DescriptorSet) {
        unsafe {
            let mut writes = Vec::with_capacity(self.buffer_write_params.len());
            for buf_params in self.buffer_write_params.iter() {
                let write = WriteDescriptorSet::builder()
                    .dst_binding(buf_params.destination_binding)
                    .dst_set(*destination_set)
                    .descriptor_type(buf_params.descriptor_type)
                    .buffer_info(std::slice::from_ref(&buf_params.buffer_info))
                    .build();
                writes.push(write);
            }

            for img_params in self.image_write_params.iter() {
                let write = WriteDescriptorSet::builder()
                    .dst_binding(img_params.destination_binding)
                    .dst_set(*destination_set)
                    .descriptor_type(img_params.descriptor_type)
                    .image_info(std::slice::from_ref(&img_params.image_info))
                    .build();
                writes.push(write);
            }

            logical_device.update_descriptor_sets(&writes, &[]);
        }
    }
}
