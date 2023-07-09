use ash::vk::CommandBuffer;
use ash::vk::Extent2D;
use ash::vk::Format;
use ash::vk::SharingMode;
use ash::vk::{AccessFlags, DependencyFlags, PipelineStageFlags, SampleCountFlags};
use ash::vk::{DeviceMemory, MemoryAllocateInfo, MemoryPropertyFlags};
use ash::vk::{
    Image, ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageMemoryBarrier,
    ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfo,
    ImageViewType,
};

use crate::result::Context;

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpDepthImage {
    pub depth_image: Image,
    pub depth_image_view: ImageView,
    pub depth_image_memory: DeviceMemory,
}

impl SetUpDepthImage {
    pub unsafe fn create(
        physical_device: &SetUpPhysicalDevice,
        virtual_device: &SetUpVirtualDevice,
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
        let depth_image_memory_index = physical_device.find_memory_type_index(
            &depth_image_memory_req,
            MemoryPropertyFlags::DEVICE_LOCAL,
        )
        .context("Unable to find suitable memory index for depth image!".to_string())?;

        let depth_image_allocate_info = MemoryAllocateInfo::builder()
            .allocation_size(depth_image_memory_req.size)
            .memory_type_index(depth_image_memory_index);

        let depth_image_memory = virtual_device
            .device
            .allocate_memory(&depth_image_allocate_info, None)?;

        virtual_device
            .device
            .bind_image_memory(depth_image, depth_image_memory, 0)?;

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
            depth_image_memory,
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
        physical_device: &SetUpPhysicalDevice,
        virtual_device: &SetUpVirtualDevice,
        window_width: u32,
        window_height: u32,
    ) -> crate::Result<Self> {
        let surface_capabilities = physical_device.surface_capabilities;
        let surface_extent = match surface_capabilities.current_extent.width {
            std::u32::MAX => Extent2D {
                width: window_width,
                height: window_height,
            },
            _ => surface_capabilities.current_extent,
        };

        Self::create(
            physical_device,
            virtual_device,
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

    pub fn submit_pipeline_barrier_command_buffer(
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
    
    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .free_memory(self.depth_image_memory, None);
            virtual_device
                .device
                .destroy_image_view(self.depth_image_view, None);
            virtual_device.device.destroy_image(self.depth_image, None);
        }
    }
}
