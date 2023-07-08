use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::CompositeAlphaFlagsKHR;
use ash::vk::Extent2D;
use ash::vk::ImageUsageFlags;
use ash::vk::PresentModeKHR;
use ash::vk::SharingMode;
use ash::vk::{SurfaceKHR, SurfaceTransformFlagsKHR};
use ash::vk::{SwapchainCreateInfoKHR, SwapchainKHR};
use ash::Instance;

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpSwapchain {
    pub swapchain_loader: Swapchain,
    pub swapchain: SwapchainKHR,
}

impl SetUpSwapchain {
    pub unsafe fn create(
        instance: &Instance,
        surface: SurfaceKHR,
        desired_image_count: u32,
        image_extent: Extent2D,
        image_usage: ImageUsageFlags,
        image_sharing_mode: SharingMode,
        pre_transform: SurfaceTransformFlagsKHR,
        composite_alpha: CompositeAlphaFlagsKHR,
        present_mode: PresentModeKHR,
        clipped: bool,
        image_array_layers: u32,
        physical_device: &SetUpPhysicalDevice,
        virtual_device: &SetUpVirtualDevice,
    ) -> crate::Result<Self> {
        let swapchain_loader = Swapchain::new(&instance, &virtual_device.device);

        let surface_format = physical_device.surface_format;

        let swapchain_create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(image_extent)
            .image_usage(image_usage)
            .image_sharing_mode(image_sharing_mode)
            .pre_transform(pre_transform)
            .composite_alpha(composite_alpha)
            .present_mode(present_mode)
            .clipped(clipped)
            .image_array_layers(image_array_layers);

        let swapchain = swapchain_loader.create_swapchain(&swapchain_create_info, None)?;

        Ok(Self {
            swapchain_loader,
            swapchain,
        })
    }

    pub unsafe fn create_with_defaults(
        instance: &Instance,
        surface_loader: &Surface,
        surface: SurfaceKHR,
        physical_device: &SetUpPhysicalDevice,
        virtual_device: &SetUpVirtualDevice,
        window_width: u32,
        window_height: u32,
    ) -> crate::Result<Self> {
        let surface_capabilities = &physical_device.surface_capabilities;

        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0
            && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let surface_resolution = match surface_capabilities.current_extent.width {
            std::u32::MAX => Extent2D {
                width: window_width,
                height: window_height,
            },
            _ => surface_capabilities.current_extent,
        };

        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(SurfaceTransformFlagsKHR::IDENTITY)
        {
            SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = surface_loader
            .get_physical_device_surface_present_modes(physical_device.device, surface)
            .unwrap();
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == PresentModeKHR::MAILBOX)
            .unwrap_or(PresentModeKHR::FIFO);

        Self::create(
            instance,
            surface,
            desired_image_count,
            surface_resolution,
            ImageUsageFlags::COLOR_ATTACHMENT,
            SharingMode::EXCLUSIVE,
            pre_transform,
            CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            true,
            1,
            physical_device,
            virtual_device,
        )
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }
}
