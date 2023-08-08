extern crate shaderc;
extern crate walkdir;

use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use walkdir::WalkDir;

use shaderc::*;

fn main() {
    println!("cargo:rerun-if-changed=shader");

    let compiler: Compiler = Compiler::new().unwrap();

    let mut options = CompileOptions::new().expect("Unable to create compile options");

    options.set_include_callback(|requested_name, include_type, containing_name, depth| {
        if depth > 20 {
            return Err(
                "Include depth of 20 has been reached. Have you included files recursively?"
                    .to_string(),
            );
        }

        let path_to_containing = PathBuf::from(containing_name);

        // Does this actually work everywhere? I need to confirm this.
        let mut path_to_requested = path_to_containing.clone();
        path_to_requested.set_file_name(requested_name);

        let resolved_path = std::fs::canonicalize(&path_to_requested)
            .expect("Unable to get absolute path to requested file");

        println!(
            "Got include {requested_name}, {include_type:?}, {containing_name} -> {}",
            path_to_requested.display()
        );

        let resolved_content =
            std::fs::read_to_string(&resolved_path).expect("Unable to read included file");

        Ok(ResolvedInclude {
            resolved_name: resolved_path
                .into_os_string()
                .into_string()
                .expect("Unable to get resolved path name"),
            content: resolved_content,
        })
    });

    options.set_generate_debug_info();

    let ok_entries = WalkDir::new(".").into_iter().filter_map(|e| e.ok());
    let file_entries = ok_entries.filter(|entry| entry.file_type().is_file());
    let shaders = file_entries
        .filter_map(|entry| {
            let path = entry.path().to_owned();
            let extension = path.extension().and_then(OsStr::to_str);
            match extension {
                Some("vert") => {
                    let shader_kind = ShaderKind::Vertex;
                    let source = std::fs::read_to_string(&path).expect("Unable to read file");
                    Some((source, shader_kind, path))
                }
                Some("frag") => {
                    let shader_kind = ShaderKind::Fragment;
                    let source = std::fs::read_to_string(&path).expect("Unable to read file");
                    Some((source, shader_kind, path))
                }
                _ => None,
            }
        })
        .map(|(source, shader_kind, path_buf)| {
            let path = path_buf.to_str().expect("Unable to get str path");
            let binary_result = compiler
                .compile_into_spirv(&source, shader_kind, path, "main", Some(&options))
                .expect("Unable to compile shader");
            (binary_result, path_buf)
        });

    for (binary_result, mut path) in shaders {
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .expect("Unable to get path extension string");
        path.set_extension(format!("{}.spirv", extension));

        let mut file = File::create(&path).expect("Unable to create output file");
        let bytes = binary_result.as_binary_u8();
        file.write_all(&bytes)
            .expect("Unable to write SPIRV bytes to file");
    }
}
