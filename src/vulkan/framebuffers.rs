use ash::vk::{Framebuffer, FramebufferCreateInfo};

use super::depth_image::SetUpDepthImage;
use super::present_images::SetUpPresentImages;
use super::render_pass::SetUpRenderPass;
use super::surface::SetUpSurfaceInfo;
use super::logical_device::SetUpLogicalDevice;

pub struct SetUpFramebuffers {
    pub framebuffers: Vec<Framebuffer>,
}

impl SetUpFramebuffers {
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        depth_image: &SetUpDepthImage,
        surface_info: &SetUpSurfaceInfo,
        present_images: &SetUpPresentImages,
        render_pass: &SetUpRenderPass,
    ) -> crate::Result<Self> {
        let depth_image_view = &depth_image.depth_image_view;
        let surface_extent = surface_info.surface_extent()?;
        let present_image_views = &present_images.present_image_views;

        let mut framebuffers: Vec<Framebuffer> = Vec::with_capacity(present_image_views.len());
        for present_image_view in present_image_views.iter() {
            let framebuffer_attachments = [*present_image_view, depth_image_view.image_view];
            let frame_buffer_create_info = FramebufferCreateInfo::builder()
                .render_pass(render_pass.render_pass)
                .attachments(&framebuffer_attachments)
                .width(surface_extent.width)
                .height(surface_extent.height)
                .layers(1);

            let framebuffer = logical_device
                .device
                .create_framebuffer(&frame_buffer_create_info, None)?;
            framebuffers.push(framebuffer);
        }

        Ok(Self { framebuffers })
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            for framebuffer in self.framebuffers.iter() {
                logical_device
                    .device
                    .destroy_framebuffer(*framebuffer, None);
            }
        }
    }
}
