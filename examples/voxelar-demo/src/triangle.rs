use voxelar::ash::vk;

use voxelar::compile_shader;
use voxelar::shaderc::ShaderKind;
use voxelar::voxelar_math::vec4::Vec4;
use voxelar::vulkan::buffer::AllocatedBuffer;
use voxelar::vulkan::debug::VerificationProvider;
use voxelar::vulkan::graphics_pipeline_builder::GraphicsPipelineBuilder;
use voxelar::vulkan::pipeline_layout::SetUpPipelineLayout;
use voxelar::vulkan::shader::CompiledShaderModule;
use voxelar::vulkan::VulkanContext;

use voxelar_vertex::*;

use crate::vertex::Vertex;

pub struct TriangleDemo {
    pipeline_layout: SetUpPipelineLayout,
    pipelines: Vec<vk::Pipeline>,
    viewport: vk::Viewport,
    scissor: vk::Rect2D,
    vertex_buffer: AllocatedBuffer<Vertex>,
    index_buffer: AllocatedBuffer<u32>,
    index_count: usize,
    vertex_shader_module: CompiledShaderModule,
    fragment_shader_module: CompiledShaderModule,
}

impl TriangleDemo {
    pub unsafe fn create<V: VerificationProvider>(
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<Self> {
        let render_pass = vulkan_context.render_pass()?;
        let virtual_device = vulkan_context.virtual_device()?;
        let pipeline_layout = vulkan_context.create_default_pipeline_layout()?;

        let surface_resolution = vulkan_context.swapchain()?.surface_extent;
        let surface_width = surface_resolution.width;
        let surface_height = surface_resolution.height;

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
        let vertex_shader_module = vulkan_context.create_vertex_shader(compiled_vert)?;

        let compiled_frag = compile_shader!(ShaderKind::Fragment, "../shader/triangle.frag")?;
        let fragment_shader_module = vulkan_context.create_fragment_shader(compiled_frag)?;

        let (_data, vertex_input_state_info) = Vertex::input_state_info();

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_width as f32,
            height: surface_height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = surface_resolution.into();

        let graphics_pipeline = GraphicsPipelineBuilder::new()
            .vertex_input(vertex_input_state_info)
            .add_shader_stage(vertex_shader_module.get_stage_create_info())
            .add_shader_stage(fragment_shader_module.get_stage_create_info())
            .input_assembly_with_topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .rasterization_with_polygon_mode(vk::PolygonMode::FILL)
            .multisample_with_samples(vk::SampleCountFlags::TYPE_1)
            .color_blend_attachment_with_defaults()
            .depth_stencil_with_default_ops()
            .add_dynamic_state(vk::DynamicState::VIEWPORT)
            .add_dynamic_state(vk::DynamicState::SCISSOR)
            .viewport(viewport)
            .scissor(scissor)
            .build(&virtual_device, &render_pass, &pipeline_layout)?;

        Ok(Self {
            pipeline_layout,
            pipelines: vec![graphics_pipeline],
            viewport,
            scissor,
            vertex_buffer,
            index_buffer,
            index_count: index_buffer_data.len(),
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
        self.viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: new_width as f32,
            height: new_height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let surface_extent = vulkan_context
            .physical_device()?
            .get_surface_extent(new_width as u32, new_height as u32);
        self.scissor = surface_extent.into();

        Ok(())
    }

    pub fn update_size<V: VerificationProvider>(
        &mut self,
        vulkan_context: &mut VulkanContext<V>,
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
        let graphics_pipeline = self.pipelines[0];

        unsafe {
            let (present_index, _) = vulkan_context.acquire_next_image()?;

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

            vulkan_context.submit_render_pass_command(
                present_index,
                &clear_values,
                |device, draw_command_buffer| {
                    let device = &device.device;
                    let draw_command_buffer = draw_command_buffer.command_buffer;
                    device.cmd_bind_pipeline(
                        draw_command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        graphics_pipeline,
                    );
                    device.cmd_set_viewport(draw_command_buffer, 0, &[self.viewport]);
                    device.cmd_set_scissor(draw_command_buffer, 0, &[self.scissor]);

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
                        self.index_count as u32,
                        1,
                        0,
                        0,
                        1,
                    );

                    Ok(())
                },
            )?;

            vulkan_context.present_image(present_index)?;
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
            self.pipeline_layout.destroy(&virtual_device);

            let device = &virtual_device.device;
            for pipeline in self.pipelines.iter() {
                device.destroy_pipeline(*pipeline, None);
            }

            self.vertex_shader_module.destroy(&virtual_device);
            self.fragment_shader_module.destroy(&virtual_device);

            self.index_buffer.destroy(&virtual_device);
            self.vertex_buffer.destroy(&virtual_device);
        }

        Ok(())
    }
}
