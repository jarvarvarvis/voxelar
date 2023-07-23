use ash::vk;
use ash::vk::AccessFlags;
use ash::vk::Format;
use ash::vk::ImageLayout;
use ash::vk::{AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp};
use ash::vk::{PipelineBindPoint, PipelineStageFlags};
use ash::vk::{RenderPass, RenderPassCreateInfo};
use ash::vk::{SampleCountFlags, SubpassDependency, SubpassDescription};

use super::logical_device::SetUpLogicalDevice;
use super::surface::SetUpSurfaceInfo;

pub struct SetUpRenderPass {
    pub render_pass: RenderPass,
}

impl SetUpRenderPass {
    pub unsafe fn create_with_renderpass_attachments_and_subpasses(
        logical_device: &SetUpLogicalDevice,
        renderpass_attachments: &[AttachmentDescription],
        subpasses: &[SubpassDescription],
        subpass_dependencies: &[SubpassDependency],
    ) -> crate::Result<Self> {
        let renderpass_create_info = RenderPassCreateInfo::builder()
            .attachments(renderpass_attachments)
            .subpasses(subpasses)
            .dependencies(subpass_dependencies);

        let render_pass = logical_device.create_render_pass(&renderpass_create_info, None)?;

        Ok(Self { render_pass })
    }

    pub unsafe fn create_with_color_depth_subpass(
        logical_device: &SetUpLogicalDevice,
        renderpass_attachments: &[AttachmentDescription],
        color_attachment_refs: &[AttachmentReference],
        depth_attachment_ref: AttachmentReference,
        subpass_dependencies: &[SubpassDependency],
    ) -> crate::Result<Self> {
        let subpass = SubpassDescription::builder()
            .color_attachments(&color_attachment_refs)
            .depth_stencil_attachment(&depth_attachment_ref)
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        Self::create_with_renderpass_attachments_and_subpasses(
            logical_device,
            renderpass_attachments,
            std::slice::from_ref(&subpass),
            subpass_dependencies,
        )
    }

    pub unsafe fn create_with_defaults(
        logical_device: &SetUpLogicalDevice,
        surface_info: &SetUpSurfaceInfo,
    ) -> crate::Result<Self> {
        let surface_format = surface_info.surface_format(0)?;
        let renderpass_attachments = [
            AttachmentDescription::builder()
                .format(surface_format.format)
                .samples(SampleCountFlags::TYPE_1)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::STORE)
                .stencil_load_op(AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(AttachmentStoreOp::DONT_CARE)
                .initial_layout(ImageLayout::UNDEFINED)
                .final_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .build(),
            AttachmentDescription::builder()
                .format(Format::D16_UNORM)
                .samples(SampleCountFlags::TYPE_1)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::DONT_CARE)
                .stencil_load_op(AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(AttachmentStoreOp::DONT_CARE)
                .initial_layout(ImageLayout::UNDEFINED)
                .final_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .build(),
        ];
        let color_attachment_refs = [AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let depth_attachment_ref = AttachmentReference {
            attachment: 1,
            layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };
        let subpass_dependencies = [SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: AccessFlags::COLOR_ATTACHMENT_READ
                | AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];

        Self::create_with_color_depth_subpass(
            logical_device,
            &renderpass_attachments,
            &color_attachment_refs,
            depth_attachment_ref,
            &subpass_dependencies,
        )
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_render_pass(self.render_pass, None);
        }
    }
}
