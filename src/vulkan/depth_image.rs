use std::sync::MutexGuard;

use ash::vk::Extent2D;
use ash::vk::Format;
use ash::vk::SharingMode;
use ash::vk::{AccessFlags, DependencyFlags, PipelineStageFlags, SampleCountFlags};
use ash::vk::{
    ImageAspectFlags, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, ImageTiling,
    ImageType, ImageUsageFlags, ImageViewType,
};
use gpu_allocator::vulkan::*;

use super::command_buffer::SetUpCommandBufferWithFence;
use super::image::AllocatedImage;
use super::image_view::SetUpImageView;
use super::surface::SetUpSurfaceInfo;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDepthImage {
    pub depth_image: AllocatedImage,
    pub depth_image_view: SetUpImageView,
}

impl SetUpDepthImage {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
        format: Format,
        surface_extent: Extent2D,
        samples: SampleCountFlags,
        subresource_range: ImageSubresourceRange,
    ) -> crate::Result<Self> {
        let depth_image = AllocatedImage::create(
            virtual_device,
            allocator,
            ImageType::TYPE_2D,
            format,
            surface_extent,
            1,
            1,
            samples,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            SharingMode::EXCLUSIVE,
        )?;

        let depth_image_view = SetUpImageView::create(
            virtual_device,
            ImageViewType::TYPE_2D,
            format,
            subresource_range,
            depth_image.image,
        )?;

        Ok(Self {
            depth_image,
            depth_image_view,
        })
    }

    pub fn create_default_subresource_range() -> ImageSubresourceRange {
        ImageSubresourceRange::builder()
            .aspect_mask(ImageAspectFlags::DEPTH)
            .level_count(1)
            .layer_count(1)
            .build()
    }

    pub unsafe fn create_with_defaults(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
        surface_info: &SetUpSurfaceInfo,
    ) -> crate::Result<Self> {
        let surface_extent = surface_info.surface_extent()?;

        Self::create(
            virtual_device,
            allocator,
            Format::D16_UNORM,
            surface_extent,
            SampleCountFlags::TYPE_1,
            Self::create_default_subresource_range(),
        )
    }

    pub fn perform_layout_transition_pipeline_barrier(
        &self,
        virtual_device: &SetUpVirtualDevice,
        setup_command_buffer: &SetUpCommandBufferWithFence,
    ) {
        unsafe {
            let layout_transition_barriers = ImageMemoryBarrier::builder()
                .image(self.depth_image.image)
                .dst_access_mask(
                    AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                        | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                )
                .new_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .old_layout(ImageLayout::UNDEFINED)
                .subresource_range(Self::create_default_subresource_range())
                .build();

            virtual_device.device.cmd_pipeline_barrier(
                setup_command_buffer.command_buffer,
                PipelineStageFlags::BOTTOM_OF_PIPE,
                PipelineStageFlags::LATE_FRAGMENT_TESTS,
                DependencyFlags::empty(),
                &[],
                &[],
                &[layout_transition_barriers],
            );
        }
    }

    pub fn destroy(
        &mut self,
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.depth_image.destroy(virtual_device, allocator)?;
        self.depth_image_view.destroy(virtual_device);
        Ok(())
    }
}
