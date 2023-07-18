use ash::extensions::khr::Surface;
use ash::vk::SurfaceKHR;
use ash::vk::{ComponentMapping, ComponentSwizzle};
use ash::vk::{
    Image, ImageAspectFlags, ImageSubresourceRange, ImageView, ImageViewCreateInfo, ImageViewType,
};

use super::physical_device::SetUpPhysicalDevice;
use super::swapchain::SetUpSwapchain;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpPresentImages {
    pub present_images: Vec<Image>,
    pub present_image_views: Vec<ImageView>,
}

impl SetUpPresentImages {
    pub unsafe fn create(
        physical_device: &SetUpPhysicalDevice,
        virtual_device: &SetUpVirtualDevice,
        swapchain: &SetUpSwapchain,
        surface_loader: &Surface,
        surface: SurfaceKHR,
        components: ComponentMapping,
        subresource_range: ImageSubresourceRange,
    ) -> crate::Result<Self> {
        let device = &virtual_device.device;
        let present_images = swapchain
            .swapchain_loader
            .get_swapchain_images(swapchain.swapchain)?;

        let surface_format = physical_device.get_surface_format(surface_loader, surface)?;

        let mut present_image_views: Vec<ImageView> = Vec::with_capacity(present_images.len());
        for image in present_images.iter() {
            let image_view_create_info = ImageViewCreateInfo::builder()
                .view_type(ImageViewType::TYPE_2D)
                .format(surface_format.format)
                .components(components)
                .subresource_range(subresource_range)
                .image(*image);
            let image_view = device.create_image_view(&image_view_create_info, None)?;
            present_image_views.push(image_view);
        }

        Ok(Self {
            present_images,
            present_image_views,
        })
    }

    pub unsafe fn create_with_defaults(
        physical_device: &SetUpPhysicalDevice,
        virtual_device: &SetUpVirtualDevice,
        swapchain: &SetUpSwapchain,
        surface_loader: &Surface,
        surface: SurfaceKHR,
    ) -> crate::Result<Self> {
        let component_mapping = ComponentMapping {
            r: ComponentSwizzle::R,
            g: ComponentSwizzle::G,
            b: ComponentSwizzle::B,
            a: ComponentSwizzle::A,
        };
        let subresource_range = ImageSubresourceRange {
            aspect_mask: ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };
        Self::create(
            physical_device,
            virtual_device,
            swapchain,
            surface_loader,
            surface,
            component_mapping,
            subresource_range,
        )
    }

    pub fn destroy(&mut self, device: &SetUpVirtualDevice) {
        unsafe {
            for image_view in self.present_image_views.iter() {
                device.device.destroy_image_view(*image_view, None);
            }
        }
    }
}
