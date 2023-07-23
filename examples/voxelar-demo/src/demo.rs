use voxelar::ash::vk;
use voxelar::compile_shader;
use voxelar::engine::frame_time::FrameTimeManager;
use voxelar::nalgebra::*;
use voxelar::shaderc::ShaderKind;
use voxelar::vulkan::descriptor_set_layout::SetUpDescriptorSetLayout;
use voxelar::vulkan::descriptor_set_layout_builder::DescriptorSetLayoutBuilder;
use voxelar::vulkan::descriptor_set_logic::SetUpDescriptorSetLogic;
use voxelar::vulkan::descriptor_set_logic_builder::DescriptorSetLogicBuilder;
use voxelar::vulkan::descriptor_set_update_builder::DescriptorSetUpdateBuilder;
use voxelar::vulkan::dynamic_descriptor_buffer::DynamicDescriptorBuffer;
use voxelar::vulkan::egui_integration::SetUpEguiIntegration;
use voxelar::vulkan::graphics_pipeline_builder::GraphicsPipelineBuilder;
use voxelar::vulkan::per_frame::PerFrame;
use voxelar::vulkan::pipeline_layout::SetUpPipelineLayout;
use voxelar::vulkan::pipeline_layout_builder::PipelineLayoutBuilder;
use voxelar::vulkan::sampler::SetUpSampler;
use voxelar::vulkan::shader::CompiledShaderModule;
use voxelar::vulkan::texture::Texture;
use voxelar::vulkan::typed_buffer::TypedAllocatedBuffer;
use voxelar::vulkan::VulkanContext;
use voxelar::window::VoxelarWindow;
use voxelar::winit::event::*;
use voxelar::Voxelar;

use voxelar_vertex::input_state_builder::VertexInputStateBuilder;

use crate::vertex::*;

#[repr(C)]
pub struct DemoCameraBuffer {
    mvp_matrix: Matrix4<f32>,
}

#[repr(C)]
pub struct DemoSceneBuffer {
    ambient_color: Vector4<f32>,
}

pub struct DemoDescriptorBuffers {
    camera_buffer: DynamicDescriptorBuffer<DemoCameraBuffer>,
    scene_buffer: DynamicDescriptorBuffer<DemoSceneBuffer>,
}

pub struct PerFrameData {
    descriptor_set_logic: SetUpDescriptorSetLogic,
}

pub struct Demo {
    pub recreate_swapchain: bool,
    viewport: vk::Viewport,
    scissor: vk::Rect2D,

    descriptor_set_layouts: Vec<SetUpDescriptorSetLayout>,
    per_frame_data: PerFrame<PerFrameData>,
    descriptor_buffers: DemoDescriptorBuffers,

    pipeline_layout: SetUpPipelineLayout,
    pipelines: Vec<vk::Pipeline>,

    vertex_shader_module: CompiledShaderModule,
    fragment_shader_module: CompiledShaderModule,
    vertex_buffer: TypedAllocatedBuffer<VertexData>,
    index_buffer: TypedAllocatedBuffer<u32>,
    index_count: usize,

    sampler: SetUpSampler,
    texture: Texture<u8>,

    pub frame_time_manager: FrameTimeManager,
    camera_position: Point3<f32>,

    egui_integration: SetUpEguiIntegration,
}

impl Demo {
    pub unsafe fn create(
        voxelar_context: &Voxelar,
        vulkan_context: &VulkanContext,
        egui_integration: SetUpEguiIntegration,
    ) -> crate::Result<Self> {
        let render_pass = vulkan_context.render_pass()?;
        let logical_device = vulkan_context.logical_device()?;

        let global_set_layout = DescriptorSetLayoutBuilder::new()
            .add_binding(
                0, // In the shader, this will specify binding = 0 in the uniform layout
                1,
                vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                vk::ShaderStageFlags::VERTEX,
            )
            .add_binding(
                1, // In the shader, this will specify binding = 1 in the uniform layout
                1,
                vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                vk::ShaderStageFlags::FRAGMENT,
            )
            .add_binding(
                2, // In the shader, this will specifiy binding = 2 in the uniform layout
                1,
                vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                vk::ShaderStageFlags::FRAGMENT,
            )
            .build(logical_device)?;
        let descriptor_set_layouts = vec![global_set_layout];

        let descriptor_buffers = DemoDescriptorBuffers {
            camera_buffer: vulkan_context
                .allocate_dynamic_descriptor_uniform_buffer(vulkan_context.frame_overlap())?,
            scene_buffer: vulkan_context
                .allocate_dynamic_descriptor_uniform_buffer(vulkan_context.frame_overlap())?,
        };

        let image = voxelar::vulkan::image::open("textures/brick.jpg")?.into_rgba8();
        let image_extent = vk::Extent3D {
            width: image.width(),
            height: image.height(),
            depth: 4, // 4 channels
        };

        let texture =
            vulkan_context.create_texture(vk::Format::R8G8B8A8_SRGB, image_extent, &image)?;
        let sampler =
            vulkan_context.create_sampler(vk::Filter::NEAREST, vk::SamplerAddressMode::REPEAT)?;

        let per_frame_data = PerFrame::try_init(
            |_| {
                let descriptor_set_logic = DescriptorSetLogicBuilder::new()
                    .add_pool_size(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC, 2)
                    .add_pool_size(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 1)
                    .set_layouts(&descriptor_set_layouts)
                    .build(logical_device)?;
                let destination_set = descriptor_set_logic.get_set(0);

                DescriptorSetUpdateBuilder::new()
                    .add_dynamic_uniform_buffer_descriptor(
                        &descriptor_buffers.camera_buffer,
                        0,
                        vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                    )?
                    .add_dynamic_uniform_buffer_descriptor(
                        &descriptor_buffers.scene_buffer,
                        1,
                        vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                    )?
                    .add_texture_descriptor(
                        &sampler,
                        &texture,
                        2,
                        vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    )?
                    .update(logical_device, destination_set)?;

                Ok(PerFrameData {
                    descriptor_set_logic,
                })
            },
            vulkan_context.frame_overlap(),
        )?;

        let pipeline_layout = PipelineLayoutBuilder::new()
            .set_layouts(&descriptor_set_layouts)
            .build(logical_device)?;

        let surface_resolution = vulkan_context.get_surface_extent()?;
        let surface_width = surface_resolution.width;
        let surface_height = surface_resolution.height;

        let vertices = vec![
            VertexData {
                pos: Vector3::new(-1.0, 0.0, -1.0),
                uv: Vector2::new(-1.0, -1.0),
            },
            VertexData {
                pos: Vector3::new(1.0, 0.0, -1.0),
                uv: Vector2::new(1.0, -1.0),
            },
            VertexData {
                pos: Vector3::new(1.0, 0.0, 1.0),
                uv: Vector2::new(1.0, 1.0),
            },
            VertexData {
                pos: Vector3::new(-1.0, 0.0, 1.0),
                uv: Vector2::new(-1.0, 1.0),
            },
        ];

        let vertex_buffer = vulkan_context.create_vertex_buffer(&vertices)?;

        let vertex_input_state_builder =
            VertexInputStateBuilder::new().add_data_from_type::<VertexData>(0);
        let vertex_input_state_info = vertex_input_state_builder.build();

        let index_buffer_data = vec![0, 1, 2, 0, 2, 3];
        let index_buffer = vulkan_context.create_index_buffer(&index_buffer_data)?;

        let compiled_vert = compile_shader!(ShaderKind::Vertex, "../shader/triangle.vert")?;
        let vertex_shader_module = vulkan_context.create_vertex_shader(compiled_vert)?;

        let compiled_frag = compile_shader!(ShaderKind::Fragment, "../shader/triangle.frag")?;
        let fragment_shader_module = vulkan_context.create_fragment_shader(compiled_frag)?;

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
            .vertex_input(*vertex_input_state_info)
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
            .build(&logical_device, &render_pass, &pipeline_layout)?;

        Ok(Self {
            recreate_swapchain: false,
            viewport,
            scissor,

            descriptor_set_layouts,
            per_frame_data,
            descriptor_buffers,

            pipeline_layout,
            pipelines: vec![graphics_pipeline],

            vertex_shader_module,
            fragment_shader_module,
            vertex_buffer,
            index_buffer,
            index_count: index_buffer_data.len(),

            sampler,
            texture,

            frame_time_manager: FrameTimeManager::new(&voxelar_context),
            camera_position: Point3::new(0.0, 2.0, -4.0),

            egui_integration,
        })
    }

    pub fn new(
        voxelar_context: &Voxelar,
        vulkan_context: &VulkanContext,
        egui_integration: SetUpEguiIntegration,
    ) -> crate::Result<Self> {
        unsafe { Self::create(voxelar_context, vulkan_context, egui_integration) }
    }

    fn update_viewports_and_scissors(
        &mut self,
        vulkan_context: &VulkanContext,
    ) -> crate::Result<()> {
        let surface_extent = vulkan_context.get_surface_extent()?;
        self.viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_extent.width as f32,
            height: surface_extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        self.scissor = surface_extent.into();

        Ok(())
    }

    pub fn handle_egui_integration_event(&mut self, window_event: &WindowEvent<'_>) {
        let _ = self.egui_integration.handle_event(window_event);
    }

    pub fn update_camera_and_get_mvp_matrix(&mut self, aspect_ratio: f32) -> Matrix4<f32> {
        let projection = Matrix4::new_perspective(aspect_ratio, 60.0f32.to_radians(), 0.1, 100.0);

        let origin = Point3::new(0.0, 0.0, 0.0);
        let rotated_origin_camera_vector =
            Rotation3::from_axis_angle(&Vector3::y_axis(), 1.0f32.to_radians())
                .transform_vector(&(self.camera_position - origin));
        self.camera_position = origin + rotated_origin_camera_vector;

        let view = Matrix4::from(
            Rotation3::look_at_lh(&(origin - self.camera_position), &Vector3::y_axis())
                * Translation3::from(self.camera_position),
        );
        let model = Matrix4::identity();

        projection * view * model
    }

    pub fn render(
        &mut self,
        window: &mut VoxelarWindow,
        vulkan_context: &mut VulkanContext,
    ) -> crate::Result<()> {
        let graphics_pipeline = self.pipelines[0];

        unsafe {
            if self.recreate_swapchain {
                let new_size = window.get_size();
                vulkan_context.update_swapchain(new_size)?;
                vulkan_context
                    .update_egui_integration_swapchain(new_size, &mut self.egui_integration)?;

                self.update_viewports_and_scissors(vulkan_context)?;

                self.recreate_swapchain = false;
                return Ok(());
            }

            let total_frames = self.frame_time_manager.total_frames();
            let current_frame_index = total_frames as usize % vulkan_context.frame_overlap();
            vulkan_context.select_frame(current_frame_index);
            self.per_frame_data.select(current_frame_index);

            vulkan_context.wait_for_current_frame_draw_buffer_fences()?;
            let (present_index, swapchain_suboptimal) = vulkan_context.acquire_next_image()?;

            // If the swapchain is suboptimal for this image, only recreate it on the next frame.
            // At this point, the present complete semaphore is still in a signaled state and we have to
            // submit to the present queue to unsignal it.
            if swapchain_suboptimal {
                self.recreate_swapchain = true;
            }

            vulkan_context.record_commands_to_draw_buffer(|device, draw_command_buffer| {
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

                vulkan_context.record_render_pass(
                    present_index,
                    draw_command_buffer,
                    &clear_values,
                    || {
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
                            &[self.vertex_buffer.raw_buffer()],
                            &[0],
                        );
                        device.cmd_bind_index_buffer(
                            draw_command_buffer,
                            self.index_buffer.raw_buffer(),
                            0,
                            vk::IndexType::UINT32,
                        );

                        let mvp_matrix =
                            self.update_camera_and_get_mvp_matrix(window.aspect_ratio());
                        let current_descriptor_data = self.per_frame_data.current();
                        let camera_buffer = DemoCameraBuffer { mvp_matrix };
                        self.descriptor_buffers.camera_buffer.store_at(
                            device,
                            camera_buffer,
                            current_frame_index,
                        )?;

                        let frame_360_cycle = (total_frames % 360) as f32;
                        let light_cycle = (frame_360_cycle.to_radians().sin() + 1.0) / 2.0;
                        let ambient_color =
                            Vector4::new(light_cycle, light_cycle, light_cycle, 0.0);
                        let scene_buffer = DemoSceneBuffer { ambient_color };
                        self.descriptor_buffers.scene_buffer.store_at(
                            device,
                            scene_buffer,
                            current_frame_index,
                        )?;

                        let camera_buffer_offset = self
                            .descriptor_buffers
                            .scene_buffer
                            .get_dynamic_offset(current_frame_index);
                        let scene_buffer_offset = self
                            .descriptor_buffers
                            .camera_buffer
                            .get_dynamic_offset(current_frame_index);

                        device.cmd_bind_descriptor_sets(
                            draw_command_buffer,
                            vk::PipelineBindPoint::GRAPHICS,
                            self.pipeline_layout.pipeline_layout,
                            0,
                            &current_descriptor_data.descriptor_set_logic.descriptor_sets,
                            &[camera_buffer_offset, scene_buffer_offset],
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

                self.egui_integration.draw(
                    &window,
                    draw_command_buffer,
                    present_index,
                    |integration| {
                        let ctx = integration.context();

                        use voxelar::vulkan::egui;
                        egui::Window::new("Engine Info").show(&ctx, |ui| {
                            ui.label(format!("FPS: {:.4}", self.frame_time_manager.fps()));
                            ui.label(format!(
                                "Delta Time: {:.4}",
                                self.frame_time_manager.delta_time()
                            ));
                        });
                        Ok(())
                    },
                )?;

                Ok(())
            })?;

            vulkan_context.submit_draw_buffers()?;

            let swapchain_suboptimal = vulkan_context.present_image(present_index)?;
            if swapchain_suboptimal {
                self.recreate_swapchain = true;
            }
        }

        Ok(())
    }

    pub fn update_frame_time_manager(&mut self, context: &Voxelar) {
        self.frame_time_manager.update(context);
    }

    pub fn destroy(&mut self, vulkan_context: &VulkanContext) -> crate::Result<()> {
        let logical_device = vulkan_context.logical_device()?;

        logical_device.wait()?;
        self.egui_integration.destroy();

        unsafe {
            let mut allocator = vulkan_context.lock_allocator()?;

            self.descriptor_buffers
                .camera_buffer
                .destroy(logical_device, &mut allocator)?;
            self.descriptor_buffers
                .scene_buffer
                .destroy(logical_device, &mut allocator)?;

            for descriptor_data in self.per_frame_data.iter_mut() {
                descriptor_data.descriptor_set_logic.destroy(logical_device);
            }

            for descriptor_set_layout in self.descriptor_set_layouts.iter_mut() {
                descriptor_set_layout.destroy(logical_device);
            }

            for pipeline in self.pipelines.iter() {
                logical_device.destroy_pipeline(*pipeline, None);
            }
            self.pipeline_layout.destroy(logical_device);

            self.vertex_shader_module.destroy(logical_device);
            self.fragment_shader_module.destroy(logical_device);

            self.index_buffer.destroy(logical_device, &mut allocator)?;
            self.vertex_buffer.destroy(logical_device, &mut allocator)?;

            self.texture.destroy(logical_device, &mut allocator)?;
            self.sampler.destroy(logical_device);
        }

        Ok(())
    }
}
