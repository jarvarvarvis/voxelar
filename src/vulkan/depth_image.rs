use std::sync::MutexGuard;

use ash::vk::CommandBuffer;
use ash::vk::Extent2D;
use ash::vk::Format;
use ash::vk::SharingMode;
use ash::vk::{AccessFlags, DependencyFlags, PipelineStageFlags, SampleCountFlags};
use ash::vk::{
    Image, ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageMemoryBarrier,
    ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfo,
    ImageViewType,
};
use gpu_allocator::vulkan::*;
use gpu_allocator::*;

use super::surface::SetUpSurfaceInfo;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDepthImage {
    pub depth_image: Image,
    pub depth_image_view: ImageView,
    pub depth_image_allocation: Option<Allocation>,
}

impl SetUpDepthImage {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        mut allocator: MutexGuard<Allocator>,
        image_type: ImageType,
        format: Format,
        surface_extent: Extent2D,
        mip_levels: u32,
        array_layers: u32,
        samples: SampleCountFlags,
        tiling: ImageTiling,
        image_usage: ImageUsageFlags,
        sharing_mode: SharingMode,
        subresource_range: ImageSubresourceRange,
    ) -> crate::Result<Self> {
        let depth_image_create_info = ImageCreateInfo::builder()
            .image_type(image_type)
            .format(format)
            .extent(surface_extent.into())
            .mip_levels(mip_levels)
            .array_layers(array_layers)
            .samples(samples)
            .tiling(tiling)
            .usage(image_usage)
            .sharing_mode(sharing_mode);

        let depth_image = virtual_device
            .device
            .create_image(&depth_image_create_info, None)?;
        let depth_image_memory_req = virtual_device
            .device
            .get_image_memory_requirements(depth_image);

        let depth_image_allocation = allocator.allocate(&AllocationCreateDesc {
            name: "depth image",
            requirements: depth_image_memory_req,
            location: MemoryLocation::GpuOnly,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;

        virtual_device.device.bind_image_memory(
            depth_image,
            depth_image_allocation.memory(),
            depth_image_allocation.offset(),
        )?;

        let depth_image_view_info = ImageViewCreateInfo::builder()
            .view_type(ImageViewType::TYPE_2D)
            .format(depth_image_create_info.format)
            .subresource_range(subresource_range)
            .image(depth_image);

        let depth_image_view = virtual_device
            .device
            .create_image_view(&depth_image_view_info, None)?;

        Ok(Self {
            depth_image,
            depth_image_view,
            depth_image_allocation: Some(depth_image_allocation),
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
        allocator: MutexGuard<Allocator>,
        surface_info: &SetUpSurfaceInfo,
    ) -> crate::Result<Self> {
        let surface_extent = surface_info.surface_extent()?;

        Self::create(
            virtual_device,
            allocator,
            ImageType::TYPE_2D,
            Format::D16_UNORM,
            surface_extent,
            1,
            1,
            SampleCountFlags::TYPE_1,
            ImageTiling::OPTIMAL,
            ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            SharingMode::EXCLUSIVE,
            Self::create_default_subresource_range(),
        )
    }

    pub fn submit_pipeline_barrier_command(
        &self,
        virtual_device: &SetUpVirtualDevice,
        setup_command_buffer: CommandBuffer,
    ) {
        unsafe {
            let layout_transition_barriers = ImageMemoryBarrier::builder()
                .image(self.depth_image)
                .dst_access_mask(
                    AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                        | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                )
                .new_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .old_layout(ImageLayout::UNDEFINED)
                .subresource_range(Self::create_default_subresource_range())
                .build();

            virtual_device.device.cmd_pipeline_barrier(
                setup_command_buffer,
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
        unsafe {
            virtual_device
                .device
                .destroy_image_view(self.depth_image_view, None);
            virtual_device.device.destroy_image(self.depth_image, None);
            if let Some(depth_image_allocation) = self.depth_image_allocation.take() {
                allocator.free(depth_image_allocation)?;
            }

            Ok(())
        }
    }
}
