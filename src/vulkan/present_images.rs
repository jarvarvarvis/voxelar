use ash::vk::{ComponentMapping, ComponentSwizzle};
use ash::vk::{
    Image, ImageAspectFlags, ImageSubresourceRange, ImageView, ImageViewCreateInfo, ImageViewType,
};

use super::surface::SetUpSurfaceInfo;
use super::swapchain::SetUpSwapchain;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpPresentImages {
    pub present_images: Vec<Image>,
    pub present_image_views: Vec<ImageView>,
}

impl SetUpPresentImages {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        swapchain: &SetUpSwapchain,
        surface_info: &SetUpSurfaceInfo,
        components: ComponentMapping,
        subresource_range: ImageSubresourceRange,
    ) -> crate::Result<Self> {
        let device = &virtual_device.device;
        let present_images = swapchain
            .swapchain_loader
            .get_swapchain_images(swapchain.swapchain)?;

        let surface_format = surface_info.surface_format(0)?;

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
        virtual_device: &SetUpVirtualDevice,
        swapchain: &SetUpSwapchain,
        surface_info: &SetUpSurfaceInfo,
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
            virtual_device,
            swapchain,
            surface_info,
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
