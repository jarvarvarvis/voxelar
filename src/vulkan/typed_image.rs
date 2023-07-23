use std::marker::PhantomData;
use std::sync::MutexGuard;

use ash::vk::AccessFlags;
use ash::vk::Extent3D;
use ash::vk::Format;
use ash::vk::PipelineStageFlags;
use ash::vk::SampleCountFlags;
use ash::vk::SharingMode;
use ash::vk::{
    BufferImageCopy, Image, ImageLayout, ImageSubresourceLayers, ImageSubresourceRange,
    ImageTiling, ImageType, ImageUsageFlags,
};
use gpu_allocator::vulkan::Allocator;

use super::command_buffer::SetUpCommandBufferWithFence;
use super::image::AllocatedImage;
use super::logical_device::SetUpLogicalDevice;
use super::staging_buffer::SetUpStagingBuffer;

pub struct TypedAllocatedImage<T> {
    pub image: AllocatedImage,
    phantom: PhantomData<T>,
}

impl<T> TypedAllocatedImage<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        image_type: ImageType,
        format: Format,
        image_extent: Extent3D,
        mip_levels: u32,
        array_layers: u32,
        samples: SampleCountFlags,
        tiling: ImageTiling,
        image_usage: ImageUsageFlags,
        sharing_mode: SharingMode,
    ) -> crate::Result<Self> {
        Ok(Self {
            image: AllocatedImage::allocate(
                logical_device,
                allocator,
                image_type,
                format,
                image_extent,
                mip_levels,
                array_layers,
                samples,
                tiling,
                image_usage,
                sharing_mode,
            )?,
            phantom: PhantomData,
        })
    }

    pub fn layout_transition_to_copy_target(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        setup_command_buffer: &SetUpCommandBufferWithFence,
        image_subresource: ImageSubresourceRange,
    ) {
        self.image.perform_layout_transition_pipeline_barrier(
            logical_device,
            setup_command_buffer,
            image_subresource,
            AccessFlags::empty(),
            AccessFlags::TRANSFER_WRITE,
            ImageLayout::UNDEFINED,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            PipelineStageFlags::TOP_OF_PIPE,
            PipelineStageFlags::TRANSFER,
        );
    }

    pub fn copy_from_staging_buffer(
        &self,
        logical_device: &SetUpLogicalDevice,
        staging_buffer: &SetUpStagingBuffer<T>,
        setup_command_buffer: &SetUpCommandBufferWithFence,
        image_subresource: ImageSubresourceLayers,
    ) -> crate::Result<()> {
        let image_size = self.image.full_image_size();
        let staging_buffer_size = staging_buffer.buffer.element_amount;
        crate::verify!(
            image_size as usize == staging_buffer_size,
            "The staging buffer must have the same size as the source buffer! Expected: {image_size}, got: {staging_buffer_size}"
        );
        unsafe {
            let buffer_image_copy = BufferImageCopy::builder()
                .buffer_offset(0)
                .image_subresource(image_subresource)
                .image_extent(self.image.image_extent);

            logical_device.cmd_copy_buffer_to_image(
                setup_command_buffer.command_buffer,
                staging_buffer.raw_buffer(),
                self.raw_image(),
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &[*buffer_image_copy],
            );
        }
        Ok(())
    }

    pub fn layout_transition_to_shader_readable(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        setup_command_buffer: &SetUpCommandBufferWithFence,
        image_subresource: ImageSubresourceRange,
    ) {
        self.image.perform_layout_transition_pipeline_barrier(
            logical_device,
            setup_command_buffer,
            image_subresource,
            AccessFlags::TRANSFER_WRITE,
            AccessFlags::SHADER_READ,
            ImageLayout::TRANSFER_DST_OPTIMAL,
            ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            PipelineStageFlags::TRANSFER,
            PipelineStageFlags::FRAGMENT_SHADER,
        );
    }

    pub fn raw_image(&self) -> Image {
        self.image.image
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.image.destroy(logical_device, allocator)
    }
}
