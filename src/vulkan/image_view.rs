use ash::vk::{Format, Image};
use ash::vk::{ImageSubresourceRange, ImageView, ImageViewCreateInfo, ImageViewType};

use super::logical_device::SetUpLogicalDevice;

pub struct SetUpImageView {
    pub image_view: ImageView,
}

impl SetUpImageView {
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        image_view_type: ImageViewType,
        format: Format,
        subresource_range: ImageSubresourceRange,
        image: Image,
    ) -> crate::Result<Self> {
        let image_view_info = ImageViewCreateInfo::builder()
            .view_type(image_view_type)
            .format(format)
            .subresource_range(subresource_range)
            .image(image);

        let image_view = logical_device.create_image_view(&image_view_info, None)?;

        Ok(Self { image_view })
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_image_view(self.image_view, None);
        }
    }
}
