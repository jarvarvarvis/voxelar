use std::sync::MutexGuard;

use ash::vk::Extent3D;
use ash::vk::Format;
use ash::vk::ImageSubresourceLayers;
use ash::vk::ImageType;
use ash::vk::SampleCountFlags;
use ash::vk::SharingMode;
use ash::vk::{
    ImageAspectFlags, ImageSubresourceRange, ImageTiling, ImageUsageFlags, ImageViewType,
};
use gpu_allocator::vulkan::Allocator;

use crate::vulkan::buffers::staging_buffer::SetUpStagingBuffer;
use crate::vulkan::command::command_buffer::SetUpCommandBufferWithFence;
use crate::vulkan::logical_device::SetUpLogicalDevice;

use super::image_view::SetUpImageView;
use super::typed_image::TypedAllocatedImage;

pub struct Texture<T> {
    pub image: TypedAllocatedImage<T>,
    pub image_view: SetUpImageView,
}

impl<T> Texture<T> {
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        format: Format,
        texture_extent: Extent3D,
    ) -> crate::Result<Self> {
        let image = TypedAllocatedImage::<T>::allocate(
            logical_device,
            allocator,
            ImageType::TYPE_2D,
            format,
            texture_extent,
            1,
            1,
            SampleCountFlags::TYPE_1,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_DST,
            SharingMode::EXCLUSIVE,
        )?;
        let image_view = SetUpImageView::create(
            logical_device,
            ImageViewType::TYPE_2D,
            format,
            Self::create_default_subresource_range(),
            image.image.image,
        )?;

        Ok(Self { image, image_view })
    }

    fn create_default_subresource_range() -> ImageSubresourceRange {
        ImageSubresourceRange::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build()
    }

    fn create_default_subresource_layers() -> ImageSubresourceLayers {
        ImageSubresourceLayers::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build()
    }

    pub fn layout_transition_to_copy_target(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        setup_command_buffer: &SetUpCommandBufferWithFence,
    ) {
        self.image.layout_transition_to_copy_target(
            logical_device,
            setup_command_buffer,
            Self::create_default_subresource_range(),
        )
    }

    pub fn copy_from_staging_buffer(
        &self,
        logical_device: &SetUpLogicalDevice,
        staging_buffer: &SetUpStagingBuffer<T>,
        setup_command_buffer: &SetUpCommandBufferWithFence,
    ) -> crate::Result<()> {
        self.image.copy_from_staging_buffer(
            logical_device,
            staging_buffer,
            setup_command_buffer,
            Self::create_default_subresource_layers(),
        )
    }

    pub fn layout_transition_to_shader_readable(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        setup_command_buffer: &SetUpCommandBufferWithFence,
    ) {
        self.image.layout_transition_to_shader_readable(
            logical_device,
            setup_command_buffer,
            Self::create_default_subresource_range(),
        )
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.image.destroy(logical_device, allocator)?;
        self.image_view.destroy(logical_device);
        Ok(())
    }
}
