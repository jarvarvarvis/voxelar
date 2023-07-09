use lazy_static::lazy_static;

use shaderc::{CompileOptions, Compiler};
use shaderc::ShaderKind;

use crate::result::Context;

lazy_static! {
    static ref SHADERC_COMPILER: Compiler = Compiler::new().unwrap();
}

pub fn compile(shader_kind: ShaderKind, source: &str, path: &str) -> crate::Result<Vec<u8>> {
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
        crate::vulkan::shader::compile($kind, include_str!($path), $path)
    };
}
