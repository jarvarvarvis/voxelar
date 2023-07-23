pub use super::image_crate::*;

use std::sync::MutexGuard;

use ash::vk::AccessFlags;
use ash::vk::DependencyFlags;
use ash::vk::Extent3D;
use ash::vk::Format;
use ash::vk::PipelineStageFlags;
use ash::vk::SampleCountFlags;
use ash::vk::SharingMode;
use ash::vk::{
    Image, ImageCreateInfo, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, ImageTiling,
    ImageType, ImageUsageFlags,
};
use gpu_allocator::vulkan::*;
use gpu_allocator::*;

use super::command_buffer::SetUpCommandBufferWithFence;
use super::logical_device::SetUpLogicalDevice;

pub struct AllocatedImage {
    pub image: Image,
    pub image_extent: Extent3D,
    pub channels: u32,
    pub allocation: Option<Allocation>,
}

impl AllocatedImage {
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
        let channels = image_extent.depth;
        let image_extent = Extent3D {
            width: image_extent.width,
            height: image_extent.height,
            depth: 1,
        };

        let image_create_info = ImageCreateInfo::builder()
            .image_type(image_type)
            .format(format)
            .extent(image_extent)
            .mip_levels(mip_levels)
            .array_layers(array_layers)
            .samples(samples)
            .tiling(tiling)
            .usage(image_usage)
            .sharing_mode(sharing_mode);

        let image = logical_device
            .device
            .create_image(&image_create_info, None)?;
        let image_memory_req = logical_device.device.get_image_memory_requirements(image);

        let allocation = allocator.allocate(&AllocationCreateDesc {
            name: "image",
            requirements: image_memory_req,
            location: MemoryLocation::GpuOnly,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;

        logical_device
            .device
            .bind_image_memory(image, allocation.memory(), allocation.offset())?;

        Ok(Self {
            image,
            image_extent,
            channels,
            allocation: Some(allocation),
        })
    }

    pub fn full_image_size(&self) -> u32 {
        self.image_extent.width * self.image_extent.height * self.channels
    }

    pub fn perform_layout_transition_pipeline_barrier(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        setup_command_buffer: &SetUpCommandBufferWithFence,
        subresource_range: ImageSubresourceRange,
        src_access_mask: AccessFlags,
        dst_access_mask: AccessFlags,
        old_layout: ImageLayout,
        new_layout: ImageLayout,
        src_stage: PipelineStageFlags,
        dst_stage: PipelineStageFlags,
    ) {
        let layout_transition_barrier = ImageMemoryBarrier::builder()
            .image(self.image)
            .src_access_mask(src_access_mask)
            .dst_access_mask(dst_access_mask)
            .old_layout(old_layout)
            .new_layout(new_layout)
            .subresource_range(subresource_range);

        unsafe {
            logical_device.device.cmd_pipeline_barrier(
                setup_command_buffer.command_buffer,
                src_stage,
                dst_stage,
                DependencyFlags::empty(),
                &[],
                &[],
                &[*layout_transition_barrier],
            );
        }
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        unsafe {
            logical_device.device.destroy_image(self.image, None);
            if let Some(image_allocation) = self.allocation.take() {
                allocator.free(image_allocation)?;
            }
            Ok(())
        }
    }
}
