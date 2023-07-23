use ash::vk::GraphicsPipelineCreateInfo;
use ash::vk::Pipeline;
use ash::vk::PipelineCache;
use ash::vk::PipelineShaderStageCreateInfo;
use ash::vk::PipelineVertexInputStateCreateInfo;
use ash::vk::PipelineViewportStateCreateInfo;
use ash::vk::{
    BlendFactor, BlendOp, ColorComponentFlags, LogicOp, PipelineColorBlendAttachmentState,
    PipelineColorBlendStateCreateInfo,
};
use ash::vk::{CompareOp, PipelineDepthStencilStateCreateInfo, StencilOp, StencilOpState};
use ash::vk::{DynamicState, PipelineDynamicStateCreateInfo};
use ash::vk::{FrontFace, PipelineRasterizationStateCreateInfo, PolygonMode};
use ash::vk::{PipelineInputAssemblyStateCreateInfo, PrimitiveTopology};
use ash::vk::{PipelineMultisampleStateCreateInfo, SampleCountFlags};
use ash::vk::{Rect2D, Viewport};

use crate::result::Context;

use super::logical_device::SetUpLogicalDevice;
use super::pipeline_layout::SetUpPipelineLayout;
use super::render_pass::SetUpRenderPass;
use super::shader::CompiledShaderModule;

#[derive(Default)]
pub struct GraphicsPipelineBuilder {
    shader_stages: Vec<PipelineShaderStageCreateInfo>,
    vertex_input: Option<PipelineVertexInputStateCreateInfo>,
    input_assembly: Option<PipelineInputAssemblyStateCreateInfo>,
    rasterization: Option<PipelineRasterizationStateCreateInfo>,
    multisample: Option<PipelineMultisampleStateCreateInfo>,
    color_blend_attachment: Option<PipelineColorBlendAttachmentState>,
    depth_stencil: Option<PipelineDepthStencilStateCreateInfo>,
    dynamic_states: Vec<DynamicState>,

    viewport: Option<Viewport>,
    scissor: Option<Rect2D>,
}

fn get_safe_ref<'member, T>(
    opt: &'member Option<T>,
    state_name: &str,
) -> crate::Result<&'member T> {
    opt.as_ref()
        .context(format!("The {} was not set yet", state_name,))
}

fn get_safe_copy<T: Copy>(opt: Option<T>, state_name: &str) -> crate::Result<T> {
    opt.context(format!("The {} was not set yet", state_name,))
}

macro_rules! builder_setter {
    ($name:ident, $type:ty) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

impl GraphicsPipelineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shader_stages(mut self, shader_stages: Vec<PipelineShaderStageCreateInfo>) -> Self {
        self.shader_stages = shader_stages;
        self
    }

    pub fn add_shader_stage(mut self, shader_stage: PipelineShaderStageCreateInfo) -> Self {
        self.shader_stages.push(shader_stage);
        self
    }

    pub fn add_shader_stage_from_module(mut self, shader_module: &CompiledShaderModule) -> Self {
        self.shader_stages.push(shader_module.get_stage_create_info());
        self
    }

    builder_setter!(vertex_input, PipelineVertexInputStateCreateInfo);

    builder_setter!(input_assembly, PipelineInputAssemblyStateCreateInfo);
    pub fn input_assembly_with_topology(self, topology: PrimitiveTopology) -> Self {
        let input_assembly = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(topology)
            .primitive_restart_enable(false)
            .build();
        self.input_assembly(input_assembly)
    }

    builder_setter!(rasterization, PipelineRasterizationStateCreateInfo);
    pub fn rasterization_with_polygon_mode(self, polygon_mode: PolygonMode) -> Self {
        let rasterization = PipelineRasterizationStateCreateInfo::builder()
            .polygon_mode(polygon_mode)
            .front_face(FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0)
            .build();
        self.rasterization(rasterization)
    }

    builder_setter!(multisample, PipelineMultisampleStateCreateInfo);
    pub fn multisample_with_samples(self, rasterization_samples: SampleCountFlags) -> Self {
        let multisample = PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(rasterization_samples)
            .build();
        self.multisample(multisample)
    }

    builder_setter!(color_blend_attachment, PipelineColorBlendAttachmentState);
    pub fn color_blend_attachment_with_defaults(self) -> Self {
        let color_blend_attachment = PipelineColorBlendAttachmentState::builder()
            .blend_enable(false)
            .src_color_blend_factor(BlendFactor::SRC_COLOR)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_DST_COLOR)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ZERO)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
            .color_write_mask(ColorComponentFlags::RGBA)
            .build();
        self.color_blend_attachment(color_blend_attachment)
    }

    builder_setter!(depth_stencil, PipelineDepthStencilStateCreateInfo);
    pub fn depth_stencil_with_stencil_ops(
        self,
        front_stencil_op_state: StencilOpState,
        back_stencil_op_state: StencilOpState,
    ) -> Self {
        let depth_stencil = PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(CompareOp::LESS_OR_EQUAL)
            .front(front_stencil_op_state)
            .back(back_stencil_op_state)
            .max_depth_bounds(1.0)
            .build();
        self.depth_stencil(depth_stencil)
    }
    pub fn depth_stencil_with_default_ops(self) -> Self {
        let noop_stencil_state = StencilOpState::builder()
            .fail_op(StencilOp::KEEP)
            .pass_op(StencilOp::KEEP)
            .depth_fail_op(StencilOp::KEEP)
            .compare_op(CompareOp::ALWAYS)
            .build();
        self.depth_stencil_with_stencil_ops(noop_stencil_state, noop_stencil_state)
    }

    pub fn add_dynamic_state(mut self, dynamic_state: DynamicState) -> Self {
        self.dynamic_states.push(dynamic_state);
        self
    }

    builder_setter!(viewport, Viewport);

    builder_setter!(scissor, Rect2D);

    pub fn build(
        &self,
        logical_device: &SetUpLogicalDevice,
        render_pass: &SetUpRenderPass,
        pipeline_layout: &SetUpPipelineLayout,
    ) -> crate::Result<Pipeline> {
        let viewports = [get_safe_copy(self.viewport, "viewport")?];
        let scissor = [get_safe_copy(self.scissor, "scissors")?];
        let viewport_state = PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissor);

        let color_blend_attachments = [get_safe_copy(
            self.color_blend_attachment,
            "color blend attachment",
        )?];
        let color_blend_state = PipelineColorBlendStateCreateInfo::builder()
            .logic_op(LogicOp::CLEAR)
            .attachments(&color_blend_attachments);

        let dynamic_state =
            PipelineDynamicStateCreateInfo::builder().dynamic_states(&self.dynamic_states);

        let pipeline_create_info = GraphicsPipelineCreateInfo::builder()
            .stages(&self.shader_stages)
            .vertex_input_state(get_safe_ref(&self.vertex_input, "vertex input state")?)
            .input_assembly_state(get_safe_ref(&self.input_assembly, "input assembly state")?)
            .viewport_state(&viewport_state)
            .rasterization_state(get_safe_ref(&self.rasterization, "rasterization state")?)
            .multisample_state(get_safe_ref(&self.multisample, "multisample state")?)
            .color_blend_state(&color_blend_state)
            .depth_stencil_state(get_safe_ref(&self.depth_stencil, "depth stencil state")?)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout.pipeline_layout)
            .render_pass(render_pass.render_pass)
            .build();

        unsafe {
            let graphics_pipeline = logical_device
                .create_graphics_pipelines(PipelineCache::null(), &[pipeline_create_info], None)
                .map_err(|(_, err)| err)?;
            Ok(graphics_pipeline[0])
        }
    }
}
