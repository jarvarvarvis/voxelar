use std::ffi::CString;
use std::io::Cursor;

use ash::util::read_spv;
use lazy_static::lazy_static;

use shaderc::ShaderKind;
use shaderc::{CompileOptions, Compiler};

use ash::vk::{
    PipelineShaderStageCreateInfo, ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags,
};

use crate::result::Context;

use super::logical_device::SetUpLogicalDevice;

lazy_static! {
    static ref SHADERC_COMPILER: Compiler = Compiler::new().unwrap();
}

pub struct CompiledShaderModule {
    pub shader_module: ShaderModule,
    pub stage: ShaderStageFlags,
    pub entry_name: CString,
}

impl CompiledShaderModule {
    pub unsafe fn create(
        compiled_bytes: Vec<u8>,
        logical_device: &SetUpLogicalDevice,
        stage: ShaderStageFlags,
        entry_name: CString,
    ) -> crate::Result<Self> {
        let mut cursor = Cursor::new(&compiled_bytes);
        let code = read_spv(&mut cursor)?;
        let create_info = ShaderModuleCreateInfo::builder().code(&code);
        let shader_module = logical_device.create_shader_module(&create_info, None)?;
        Ok(Self {
            shader_module,
            stage,
            entry_name,
        })
    }

    pub unsafe fn create_shader_of_stage(
        compiled_bytes: Vec<u8>,
        logical_device: &SetUpLogicalDevice,
        stage: ShaderStageFlags,
    ) -> crate::Result<Self> {
        Self::create(compiled_bytes, logical_device, stage, CString::new("main")?)
    }

    pub fn get_stage_create_info(&self) -> PipelineShaderStageCreateInfo {
        PipelineShaderStageCreateInfo {
            module: self.shader_module,
            p_name: self.entry_name.as_ptr(),
            stage: self.stage,
            ..Default::default()
        }
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_shader_module(self.shader_module, None);
        }
    }
}

pub fn compile_bytes(shader_kind: ShaderKind, source: &str, path: &str) -> crate::Result<Vec<u8>> {
    let options = CompileOptions::new().context("Unable to create compile options".to_string())?;
    let binary_result = SHADERC_COMPILER
        .compile_into_spirv(source, shader_kind, path, "main", Some(&options))
        .context("Unable to compile shader".to_string())?;
    let bytes = binary_result.as_binary_u8().to_vec();
    Ok(bytes)
}

#[macro_export]
macro_rules! compile_shader {
    ($kind:expr, $path:tt) => {
        crate::vulkan::shader::compile_bytes($kind, include_str!($path), $path)
    };
}
