use voxelar::ash::vk;
use voxelar::ash::vk::DescriptorType;
use voxelar::ash::vk::ShaderStageFlags;
use voxelar::compile_shader;
use voxelar::engine::frame_time::FrameTimeManager;
use voxelar::engine::per_frame::PerFrame;
use voxelar::nalgebra::Matrix4;
use voxelar::nalgebra::Point3;
use voxelar::nalgebra::Rotation3;
use voxelar::nalgebra::Translation3;
use voxelar::nalgebra::Vector3;
use voxelar::shaderc::ShaderKind;
use voxelar::vulkan::buffer::AllocatedBuffer;
use voxelar::vulkan::debug::VerificationProvider;
use voxelar::vulkan::descriptor_set_layout::SetUpDescriptorSetLayout;
use voxelar::vulkan::descriptor_set_layout_builder::DescriptorSetLayoutBuilder;
use voxelar::vulkan::descriptor_set_logic::SetUpDescriptorSetLogic;
use voxelar::vulkan::descriptor_set_logic_builder::DescriptorSetLogicBuilder;
use voxelar::vulkan::graphics_pipeline_builder::GraphicsPipelineBuilder;
use voxelar::vulkan::pipeline_layout::SetUpPipelineLayout;
use voxelar::vulkan::pipeline_layout_builder::PipelineLayoutBuilder;
use voxelar::vulkan::shader::CompiledShaderModule;
use voxelar::vulkan::VulkanContext;
use voxelar::window::VoxelarWindow;
use voxelar::Voxelar;

use voxelar_vertex::*;

use crate::vertex::Vertex;

#[repr(C)]
pub struct DemoCameraBuffer {
    pub mvp_matrix: Matrix4<f32>,
}

pub struct DemoDescriptorSetData {
    descriptor_set_logic: SetUpDescriptorSetLogic,
    camera_buffer: AllocatedBuffer<DemoCameraBuffer>,
}

pub struct Demo {
    global_set_layout: SetUpDescriptorSetLayout,
    per_frame_descriptor_set_data: PerFrame<DemoDescriptorSetData>,

    pipeline_layout: SetUpPipelineLayout,
    pipelines: Vec<vk::Pipeline>,
    viewport: vk::Viewport,
    scissor: vk::Rect2D,
    vertex_buffer: AllocatedBuffer<Vertex>,
    index_buffer: AllocatedBuffer<u32>,
    index_count: usize,
    vertex_shader_module: CompiledShaderModule,
    fragment_shader_module: CompiledShaderModule,

    frame_time_manager: FrameTimeManager,
    camera_position: Point3<f32>,
}

impl Demo {
    pub unsafe fn create<V: VerificationProvider>(
        voxelar_context: &Voxelar,
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<Self> {
        let render_pass = vulkan_context.render_pass()?;
        let virtual_device = vulkan_context.virtual_device()?;
        let physical_device = vulkan_context.physical_device()?;

        let global_set_layout = DescriptorSetLayoutBuilder::new()
            .add_binding(
                0,
                1,
                DescriptorType::UNIFORM_BUFFER,
                ShaderStageFlags::VERTEX,
            )
            .build(virtual_device)?;

        let per_frame_descriptor_set_data = PerFrame::try_init(
            || {
                let descriptor_set_logic = DescriptorSetLogicBuilder::new()
                    .max_sets(1)
                    .add_pool_size(DescriptorType::UNIFORM_BUFFER, 1)
                    .set_layouts(std::slice::from_ref(&global_set_layout))
                    .build(virtual_device)?;

                let camera_buffer = AllocatedBuffer::<DemoCameraBuffer>::allocate_uniform_buffer(
                    virtual_device,
                    physical_device,
                )?;

                let camera_descriptor_set = descriptor_set_logic.get_set(0);
                camera_descriptor_set.attach_uniform_buffer_to_descriptor(
                    virtual_device,
                    &camera_buffer,
                    0,
                    0,
                )?;

                Ok(DemoDescriptorSetData {
                    descriptor_set_logic,
                    camera_buffer,
                })
            },
            vulkan_context.frame_overlap(),
        )?;

        let pipeline_layout = PipelineLayoutBuilder::new()
            .set_layouts(std::slice::from_ref(&global_set_layout))
            .build(virtual_device)?;

        let surface_resolution = vulkan_context.swapchain()?.surface_extent;
        let surface_width = surface_resolution.width;
        let surface_height = surface_resolution.height;

        let vertices = vec![
            Vertex {
                pos: Vector3::new(-1.0, -1.0, 1.0),
            },
            Vertex {
                pos: Vector3::new(1.0, -1.0, 1.0),
            },
            Vertex {
                pos: Vector3::new(1.0, 1.0, 1.0),
            },
            Vertex {
                pos: Vector3::new(-1.0, 1.0, 1.0),
            },
            Vertex {
                pos: Vector3::new(-1.0, -1.0, -1.0),
            },
            Vertex {
                pos: Vector3::new(1.0, -1.0, -1.0),
            },
            Vertex {
                pos: Vector3::new(1.0, 1.0, -1.0),
            },
            Vertex {
                pos: Vector3::new(-1.0, 1.0, -1.0),
            },
        ];
        let vertex_buffer = vulkan_context.create_vertex_buffer(&vertices)?;

        let index_buffer_data = vec![
            0, 1, 2, 2, 3, 0, // Front
            1, 5, 6, 6, 2, 1, // Right
            7, 6, 5, 5, 4, 7, // Back
            4, 0, 3, 3, 7, 4, // Left
            4, 5, 1, 1, 0, 4, // Bottom
            3, 2, 6, 6, 7, 3, // Top
        ];
        let index_buffer = vulkan_context.create_index_buffer(&index_buffer_data)?;

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
            global_set_layout,
            per_frame_descriptor_set_data,

            pipeline_layout,
            pipelines: vec![graphics_pipeline],
            viewport,
            scissor,
            vertex_buffer,
            index_buffer,
            index_count: index_buffer_data.len(),
            vertex_shader_module,
            fragment_shader_module,

            frame_time_manager: FrameTimeManager::new(&voxelar_context),
            camera_position: Point3::new(0.0, 2.0, -4.0),
        })
    }

    pub fn new<V: VerificationProvider>(
        voxelar_context: &Voxelar,
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<Self> {
        unsafe { Self::create(voxelar_context, vulkan_context) }
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

    pub fn update_camera_and_get_mvp_matrix(&mut self, aspect_ratio: f32) -> Matrix4<f32> {
        let projection = Matrix4::new_perspective(aspect_ratio, 60.0f32.to_radians(), 0.1, 100.0);

        let origin = Point3::new(0.0, 0.0, 0.0);
        let rotated_origin_camera_vector =
            Rotation3::from_axis_angle(&Vector3::y_axis(), 1.0f32.to_radians())
                .transform_vector(&(self.camera_position - origin));
        self.camera_position = origin + rotated_origin_camera_vector;
        self.camera_position.y = ((self.frame_time_manager.total_frames() % 360) as f32)
            .to_radians()
            .sin()
            * 2.0;

        let view = Matrix4::from(
            Rotation3::look_at_lh(&(origin - self.camera_position), &Vector3::y_axis())
                * Translation3::from(self.camera_position),
        );
        let model = Matrix4::identity();

        projection * view * model
    }

    pub fn render<V: VerificationProvider>(
        &mut self,
        window: &mut VoxelarWindow,
        vulkan_context: &mut VulkanContext<V>,
    ) -> crate::Result<()> {
        let graphics_pipeline = self.pipelines[0];

        window.set_title(&format!("FPS: {}", self.frame_time_manager.fps()));

        unsafe {
            let current_frame_index =
                self.frame_time_manager.total_frames() as usize % vulkan_context.frame_overlap();
            vulkan_context.select_frame(current_frame_index);
            self.per_frame_descriptor_set_data
                .select(current_frame_index);

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
                    let vk_device = &device.device;
                    let draw_command_buffer = draw_command_buffer.command_buffer;
                    vk_device.cmd_bind_pipeline(
                        draw_command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        graphics_pipeline,
                    );

                    vk_device.cmd_set_viewport(draw_command_buffer, 0, &[self.viewport]);
                    vk_device.cmd_set_scissor(draw_command_buffer, 0, &[self.scissor]);

                    vk_device.cmd_bind_vertex_buffers(
                        draw_command_buffer,
                        0,
                        &[self.vertex_buffer.buffer],
                        &[0],
                    );
                    vk_device.cmd_bind_index_buffer(
                        draw_command_buffer,
                        self.index_buffer.buffer,
                        0,
                        vk::IndexType::UINT32,
                    );

                    let mvp_matrix = self.update_camera_and_get_mvp_matrix(window.aspect_ratio());
                    let current_descriptor_data = &self.per_frame_descriptor_set_data.current();
                    let camera_buffer = DemoCameraBuffer { mvp_matrix };
                    current_descriptor_data.camera_buffer.store(device, camera_buffer)?;

                    vk_device.cmd_bind_descriptor_sets(
                        draw_command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        self.pipeline_layout.pipeline_layout,
                        0,
                        &[current_descriptor_data
                            .descriptor_set_logic
                            .get_set(0)
                            .descriptor_set],
                        &[],
                    );

                    vk_device.cmd_draw_indexed(
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

    pub fn prepare_time_manager_frame(&mut self, context: &Voxelar) {
        self.frame_time_manager.prepare_frame(context);
    }

    pub fn complete_time_manager_frame(&mut self, context: &Voxelar) {
        self.frame_time_manager.complete_frame(context);
    }

    pub fn destroy<V: VerificationProvider>(
        &mut self,
        vulkan_context: &VulkanContext<V>,
    ) -> crate::Result<()> {
        let virtual_device = vulkan_context.virtual_device()?;

        virtual_device.wait();
        unsafe {
            for descriptor_data in self.per_frame_descriptor_set_data.iter_mut() {
                descriptor_data.camera_buffer.destroy(virtual_device);
                descriptor_data.descriptor_set_logic.destroy(virtual_device);
            }

            self.global_set_layout.destroy(virtual_device);
            self.pipeline_layout.destroy(virtual_device);

            let device = &virtual_device.device;
            for pipeline in self.pipelines.iter() {
                device.destroy_pipeline(*pipeline, None);
            }

            self.vertex_shader_module.destroy(virtual_device);
            self.fragment_shader_module.destroy(virtual_device);

            self.index_buffer.destroy(virtual_device);
            self.vertex_buffer.destroy(virtual_device);
        }

        Ok(())
    }
}
