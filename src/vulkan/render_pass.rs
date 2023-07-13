use ash::vk;
use ash::vk::AccessFlags;
use ash::vk::Format;
use ash::vk::ImageLayout;
use ash::vk::{AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp};
use ash::vk::{PipelineBindPoint, PipelineStageFlags};
use ash::vk::{RenderPass, RenderPassCreateInfo};
use ash::vk::{SampleCountFlags, SubpassDependency, SubpassDescription};

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpRenderPass {
    pub render_pass: RenderPass,
}

impl SetUpRenderPass {
    pub unsafe fn create_with_renderpass_attachments_and_subpasses(
        virtual_device: &SetUpVirtualDevice,
        renderpass_attachments: &[AttachmentDescription],
        subpasses: &[SubpassDescription],
        subpass_dependencies: &[SubpassDependency],
    ) -> crate::Result<Self> {
        let renderpass_create_info = RenderPassCreateInfo::builder()
            .attachments(renderpass_attachments)
            .subpasses(subpasses)
            .dependencies(subpass_dependencies);

        let render_pass = virtual_device
            .device
            .create_render_pass(&renderpass_create_info, None)?;

        Ok(Self { render_pass })
    }

    pub unsafe fn create_with_color_depth_subpass(
        virtual_device: &SetUpVirtualDevice,
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
            virtual_device,
            renderpass_attachments,
            std::slice::from_ref(&subpass),
            subpass_dependencies,
        )
    }

    pub unsafe fn create_with_defaults(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
    ) -> crate::Result<Self> {
        let renderpass_attachments = [
            AttachmentDescription {
                format: physical_device.surface_format.format,
                samples: SampleCountFlags::TYPE_1,
                load_op: AttachmentLoadOp::CLEAR,
                store_op: AttachmentStoreOp::STORE,
                final_layout: ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },
            AttachmentDescription {
                format: Format::D16_UNORM,
                samples: SampleCountFlags::TYPE_1,
                load_op: AttachmentLoadOp::CLEAR,
                store_op: AttachmentStoreOp::STORE,
                initial_layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                final_layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                ..Default::default()
            },
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
            virtual_device,
            &renderpass_attachments,
            &color_attachment_refs,
            depth_attachment_ref,
            &subpass_dependencies,
        )
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_render_pass(self.render_pass, None);
        }
    }
}
