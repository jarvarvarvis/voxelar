use std::ffi::CStr;
use std::io::Cursor;

use voxelar::ash::util::read_spv;
use voxelar::ash::vk;
use voxelar::ash::vk::Framebuffer;
use voxelar::ash::vk::RenderPass;
use voxelar::ash::vk::ShaderModule;
use voxelar::ash::vk::{Pipeline, PipelineLayout};
use voxelar::ash::vk::{Rect2D, Viewport};
use voxelar::compile_shader;
use voxelar::shaderc::ShaderKind;
use voxelar::voxelar_math::vec4::Vec4;
use voxelar::vulkan::buffer::AllocatedBuffer;
use voxelar::vulkan::debug::VerificationProvider;
use voxelar::vulkan::VulkanContext;

use voxelar_vertex::*;

use crate::vertex::Vertex;

pub struct TriangleDemo {
    render_pass: RenderPass,
    framebuffers: Vec<Framebuffer>,
    pipeline_layout: PipelineLayout,
    graphics_pipelines: Vec<Pipeline>,
    viewports: [Viewport; 1],
    scissors: [Rect2D; 1],
    vertex_buffer: AllocatedBuffer<Vertex>,
    index_buffer_data: Vec<u32>,
    index_buffer: AllocatedBuffer<u32>,
    vertex_shader_module: ShaderModule,
    fragment_shader_module: ShaderModule,
}

impl TriangleDemo {
    pub unsafe fn create<V: VerificationProvider>(
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<Self> {
        let renderpass_attachments = [
            vk::AttachmentDescription {
                format: vulkan_context.physical_device()?.surface_format.format,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },
            vk::AttachmentDescription {
                format: vk::Format::D16_UNORM,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                ..Default::default()
            },
        ];
        let color_attachment_refs = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };
        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];

        let subpass = vk::SubpassDescription::builder()
            .color_attachments(&color_attachment_refs)
            .depth_stencil_attachment(&depth_attachment_ref)
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

        let renderpass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&renderpass_attachments)
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(&dependencies);

        let device = vulkan_context.virtual_device()?;

        let renderpass = device
            .device
            .create_render_pass(&renderpass_create_info, None)?;

        let depth_image_view = vulkan_context.depth_image()?.depth_image_view;
        let surface_resolution = vulkan_context.swapchain()?.surface_extent;
        let surface_width = surface_resolution.width;
        let surface_height = surface_resolution.height;
        let present_image_views = &vulkan_context.present_images()?.present_image_views;

        let mut framebuffers: Vec<vk::Framebuffer> = Vec::with_capacity(present_image_views.len());
        for present_image_view in present_image_views.iter() {
            let framebuffer_attachments = [*present_image_view, depth_image_view];
            let frame_buffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(renderpass)
                .attachments(&framebuffer_attachments)
                .width(surface_width)
                .height(surface_height)
                .layers(1);

            let framebuffer = device
                .device
                .create_framebuffer(&frame_buffer_create_info, None)?;
            framebuffers.push(framebuffer);
        }

        let index_buffer_data = vec![0u32, 1, 2];
        let index_buffer = vulkan_context.create_index_buffer(&index_buffer_data)?;

        let vertices = vec![
            Vertex {
                pos: Vec4::<f32>::new(-0.5, 0.5, 0.0, 1.0),
                color: Vec4::<f32>::new(0.0, 1.0, 0.0, 1.0),
            },
            Vertex {
                pos: Vec4::<f32>::new(0.5, 0.5, 0.0, 1.0),
                color: Vec4::<f32>::new(0.0, 0.0, 1.0, 1.0),
            },
            Vertex {
                pos: Vec4::<f32>::new(0.0, -0.5, 0.0, 1.0),
                color: Vec4::<f32>::new(1.0, 0.0, 0.0, 1.0),
            },
        ];
        let vertex_buffer = vulkan_context.create_vertex_buffer(&vertices)?;

        let compiled_vert = compile_shader!(ShaderKind::Vertex, "../shader/triangle.vert")?;
        let mut compiled_vert_cursor = Cursor::new(&compiled_vert[..]);
        let compiled_frag = compile_shader!(ShaderKind::Fragment, "../shader/triangle.frag")?;
        let mut compiled_frag_cursor = Cursor::new(&compiled_frag[..]);

        let vertex_code = read_spv(&mut compiled_vert_cursor)?;
        let vertex_shader_info = vk::ShaderModuleCreateInfo::builder().code(&vertex_code);

        let frag_code = read_spv(&mut compiled_frag_cursor)?;
        let frag_shader_info = vk::ShaderModuleCreateInfo::builder().code(&frag_code);

        let vertex_shader_module = device
            .device
            .create_shader_module(&vertex_shader_info, None)?;

        let fragment_shader_module = device
            .device
            .create_shader_module(&frag_shader_info, None)?;

        let layout_create_info = vk::PipelineLayoutCreateInfo::default();

        let pipeline_layout = device
            .device
            .create_pipeline_layout(&layout_create_info, None)?;

        let shader_entry_name = CStr::from_bytes_with_nul_unchecked(b"main\0");
        let shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo {
                module: vertex_shader_module,
                p_name: shader_entry_name.as_ptr(),
                stage: vk::ShaderStageFlags::VERTEX,
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                module: fragment_shader_module,
                p_name: shader_entry_name.as_ptr(),
                stage: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
        ];

        let (_data, vertex_input_state_info) = Vertex::input_state_info();

        let vertex_input_assembly_state_info = PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_width as f32,
            height: surface_height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [surface_resolution.into()];
        let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
            .scissors(&scissors)
            .viewports(&viewports);

        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::FILL,
            ..Default::default()
        };
        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_info =
            vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);

        let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(renderpass)
            .build();

        let graphics_pipelines = device
            .device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[graphic_pipeline_info], None)
            .map_err(|(_, err)| err)?;

        Ok(Self {
            render_pass: renderpass,
            framebuffers,
            pipeline_layout,
            graphics_pipelines,
            viewports,
            scissors,
            vertex_buffer,
            index_buffer,
            index_buffer_data,
            vertex_shader_module,
            fragment_shader_module,
        })
    }

    pub fn new<V: VerificationProvider>(vulkan_context: &VulkanContext<V>) -> crate::Result<Self> {
        unsafe { Self::create(vulkan_context) }
    }

    fn update_viewports_and_scissors<V: VerificationProvider>(
        &mut self,
        vulkan_context: &VulkanContext<V>,
        new_width: i32,
        new_height: i32,
    ) -> crate::Result<()> {
        self.viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: new_width as f32,
            height: new_height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let surface_extent = vulkan_context
            .physical_device()?
            .get_surface_extent(new_width as u32, new_height as u32);
        self.scissors = [surface_extent.into()];

        Ok(())
    }

    pub fn update_size<V: VerificationProvider>(
        &mut self,
        vulkan_context: &VulkanContext<V>,
        new_width: i32,
        new_height: i32,
    ) -> crate::Result<()> {
        self.update_viewports_and_scissors(vulkan_context, new_width, new_height)?;
        Ok(())
    }

    pub fn render<V: VerificationProvider>(
        &self,
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<()> {
        let surface_resolution = vulkan_context.swapchain()?.surface_extent;

        let graphics_pipeline = self.graphics_pipelines[0];

        unsafe {
            let (present_index, _) = vulkan_context
                .swapchain()?
                .swapchain_loader
                .acquire_next_image(
                    vulkan_context.swapchain()?.swapchain,
                    std::u64::MAX,
                    vulkan_context
                        .internal_sync_primitives()?
                        .present_complete_semaphore,
                    vk::Fence::null(),
                )?;
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[present_index as usize])
                .render_area(surface_resolution.into())
                .clear_values(&clear_values);

            vulkan_context.submit_command_buffer(
                *vulkan_context.command_logic()?.get_command_buffer(1),
                vulkan_context
                    .internal_sync_primitives()?
                    .draw_commands_reuse_fence,
                vulkan_context.virtual_device()?.present_queue,
                &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
                &[vulkan_context
                    .internal_sync_primitives()?
                    .present_complete_semaphore],
                &[vulkan_context
                    .internal_sync_primitives()?
                    .rendering_complete_semaphore],
                |device, draw_command_buffer| {
                    let device = &device.device;
                    device.cmd_begin_render_pass(
                        draw_command_buffer,
                        &render_pass_begin_info,
                        vk::SubpassContents::INLINE,
                    );
                    device.cmd_bind_pipeline(
                        draw_command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        graphics_pipeline,
                    );
                    device.cmd_set_viewport(draw_command_buffer, 0, &self.viewports);
                    device.cmd_set_scissor(draw_command_buffer, 0, &self.scissors);
                    device.cmd_bind_vertex_buffers(
                        draw_command_buffer,
                        0,
                        &[self.vertex_buffer.buffer],
                        &[0],
                    );
                    device.cmd_bind_index_buffer(
                        draw_command_buffer,
                        self.index_buffer.buffer,
                        0,
                        vk::IndexType::UINT32,
                    );
                    device.cmd_draw_indexed(
                        draw_command_buffer,
                        self.index_buffer_data.len() as u32,
                        1,
                        0,
                        0,
                        1,
                    );
                    // Or draw without the index buffer
                    // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                    device.cmd_end_render_pass(draw_command_buffer);
                    Ok(())
                },
            )?;

            let wait_semaphors = [vulkan_context
                .internal_sync_primitives()?
                .rendering_complete_semaphore];
            let swapchains = [vulkan_context.swapchain()?.swapchain];
            let image_indices = [present_index];
            let present_info = vk::PresentInfoKHR::builder()
                .wait_semaphores(&wait_semaphors) // &base.rendering_complete_semaphore)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            vulkan_context.swapchain()?.swapchain_loader.queue_present(
                vulkan_context.virtual_device()?.present_queue,
                &present_info,
            )?;
        }
        Ok(())
    }

    pub fn destroy<V: VerificationProvider>(
        &mut self,
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<()> {
        let virtual_device = vulkan_context.virtual_device()?;

        virtual_device.wait();
        unsafe {
            let device = &virtual_device.device;
            for pipeline in self.graphics_pipelines.iter() {
                device.destroy_pipeline(*pipeline, None);
            }

            device.destroy_pipeline_layout(self.pipeline_layout, None);

            device.destroy_shader_module(self.vertex_shader_module, None);
            device.destroy_shader_module(self.fragment_shader_module, None);

            self.index_buffer.destroy(&virtual_device);
            self.vertex_buffer.destroy(&virtual_device);

            for framebuffer in self.framebuffers.iter() {
                device.destroy_framebuffer(*framebuffer, None);
            }

            device.destroy_render_pass(self.render_pass, None);
        }

        Ok(())
    }
}
