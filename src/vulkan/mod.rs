use std::ffi::CString;

use ash::vk;

use crate::render_context::RenderContext;
use crate::window::VoxelarWindow;
use crate::{Voxelar, verify};

pub struct VulkanContext {
    entry: ash::Entry,
    instance: ash::Instance,
}

fn create_instance(entry: &ash::Entry, extensions: Vec<String>) -> crate::Result<ash::Instance> {
    let extensions: Vec<CString> = extensions
        .into_iter()
        .map(|ext| CString::new(ext).expect("Failed to convert extension name"))
        .collect();
    let extension_pointers: Vec<*const i8> = extensions
        .into_iter()
        .map(|ext| ext.as_ptr())
        .collect();

    // This is the bare minimum required to create a blank instance
    // TODO: Fill this with real data
    let app_info = vk::ApplicationInfo::default();
    let info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_pointers);

    unsafe {
        let instance = entry.create_instance(&info, None)?;
        Ok(instance)
    }
}

impl RenderContext for VulkanContext {
    fn load(ctx: &mut Voxelar, _: &mut VoxelarWindow) -> crate::Result<Self> {
        verify!(ctx.vulkan_supported(), "Vulkan is not supported");

        // This is a tiny bit hacky but maybe less annoying than also passing an instance of
        // crate::Voxelar to the load function.
        let required_extensions = ctx 
            .glfw
            .get_required_instance_extensions()
            .unwrap_or(vec![]);
        verify!(required_extensions.contains(&"VK_KHR_surface".to_string()), "glfw doesn't require VK_KHR_surface, something seems to be wrong?");

        let entry = unsafe { ash::Entry::load()? };
        let instance = create_instance(&entry, required_extensions)?;

        Ok(VulkanContext { entry, instance })
    }

    fn get_info(&self) -> crate::Result<String> {
        todo!()
    }
}
