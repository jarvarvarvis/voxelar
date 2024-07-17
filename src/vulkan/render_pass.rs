use ash::vk;
use ash::vk::AccessFlags;
use ash::vk::Format;
use ash::vk::ImageLayout;
use ash::vk::{AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp};
use ash::vk::{PipelineBindPoint, PipelineStageFlags};
use ash::vk::{RenderPass, RenderPassCreateInfo};
use ash::vk::{SampleCountFlags, SubpassDependency, SubpassDescription};
use nalgebra::Vector4;

use super::logical_device::SetUpLogicalDevice;
use super::surface::SetUpSurfaceInfo;

pub fn color_clear_value(color: Vector4<f32>) -> vk::ClearValue {
    vk::ClearValue {
        color: vk::ClearColorValue {
            float32: color.into(),
        },
    }
}

pub fn default_depth_clear_value() -> vk::ClearValue {
    vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
            depth: 1.0,
            stencil: 0,
        },
    }
}

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

    fn get_default_color_attachment(format: ash::vk::Format) -> AttachmentDescription {
        AttachmentDescription::builder()
            .format(format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR)
            .build()
    }

    fn get_default_depth_stencil_attachment() -> AttachmentDescription {
        AttachmentDescription::builder()
            .format(Format::D16_UNORM)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build()
    }

    pub unsafe fn create_with_defaults(
        logical_device: &SetUpLogicalDevice,
        surface_info: &SetUpSurfaceInfo,
    ) -> crate::Result<Self> {
        let surface_format = surface_info.surface_format(0)?;
        let renderpass_attachments = [
            Self::get_default_color_attachment(surface_format.format),
            Self::get_default_depth_stencil_attachment(),
        ];
        let color_attachment_refs = [AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];

        let depth_attachment_ref = AttachmentReference {
            attachment: 1,
            layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let subpass_dependencies = [
            SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: AccessFlags::COLOR_ATTACHMENT_READ
                    | AccessFlags::COLOR_ATTACHMENT_WRITE,
                ..Default::default()
            },
            SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    | PipelineStageFlags::LATE_FRAGMENT_TESTS,
                dst_stage_mask: PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    | PipelineStageFlags::LATE_FRAGMENT_TESTS,
                dst_access_mask: AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                    | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                ..Default::default()
            },
        ];

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
