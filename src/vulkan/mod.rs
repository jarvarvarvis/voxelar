use std::ffi::{c_char, CStr, CString};

use ash::extensions::ext::DebugUtils;
use ash::vk;
use ash::vk::ApplicationInfo;
use ash::vk::DebugUtilsMessengerEXT;
use ash::vk::{InstanceCreateFlags, InstanceCreateInfo};
use ash::{Entry, Instance};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};
use raw_window_handle::RawDisplayHandle;

pub mod debug;

use crate::render_context::RenderContext;
use crate::window::VoxelarWindow;
use crate::Voxelar;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,

    pub debug_utils_loader: DebugUtils,
    pub debug_messenger: DebugUtilsMessengerEXT,
}

#[allow(unused_mut)]
fn get_extensions(raw_display_handle: RawDisplayHandle) -> crate::Result<Vec<*const i8>> {
    let mut extension_names =
        ash_window::enumerate_required_extensions(raw_display_handle)?.to_vec();

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        extension_names.push(KhrPortabilityEnumerationFn::name().as_ptr());
        // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
        extension_names.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
    }

    Ok(extension_names)
}

fn create_app_info(window: &VoxelarWindow) -> ApplicationInfo {
    let app_name = CString::new(window.title()).unwrap();

    let app_info = ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));

    *app_info
}

impl RenderContext for VulkanContext {
    fn load(ctx: &mut Voxelar, window: &mut VoxelarWindow) -> crate::Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            // App info
            let app_info = create_app_info(&window);

            // Extension name
            let extension_names = get_extensions(window.raw_display_handle())?;

            // Layer names
            let layer_names = [CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];

            let layers_names_raw: Vec<*const c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            // Create flags
            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                InstanceCreateFlags::default()
            };

            let create_info = InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(&extension_names)
                .enabled_layer_names(&layers_names_raw)
                .flags(create_flags);

            let entry = Entry::load()?;
            let instance: Instance = entry.create_instance(&create_info, None)?;

            let (debug_utils_loader, debug_messenger) =
                debug::create_debug_utils_loader_and_messenger(&entry, &instance)?;

            Ok(Self {
                entry,
                instance,
                debug_utils_loader,
                debug_messenger,
            })
        }
    }

    fn get_info(&self) -> crate::Result<String> {
        todo!()
    }
}
