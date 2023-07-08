use std::ffi::{c_char, CStr, CString};

use ash::extensions::ext::DebugUtils;
use ash::vk;
use ash::vk::ApplicationInfo;
use ash::vk::{InstanceCreateFlags, InstanceCreateInfo};
use ash::{Entry, Instance};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};

pub mod debug;
pub mod util;

use util::*;

use crate::render_context::RenderContext;
use crate::window::VoxelarWindow;
use crate::Voxelar;

use self::debug::VerificationProvider;

pub struct VulkanContext<Verification: VerificationProvider> {
    pub entry: Entry,
    pub instance: Instance,

    pub verification: Verification,
}

impl<Verification: VerificationProvider> VulkanContext<Verification> {
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
}

impl<Verification: VerificationProvider> RenderContext for VulkanContext<Verification> {
    fn load(ctx: &mut Voxelar, window: &mut VoxelarWindow) -> crate::Result<Self>
    where
        Self: Sized,
    {
        unsafe {
            // App info
            let app_info = Self::create_app_info(&window);

            // Extension names
            let mut extension_names_raw =
                ash_window::enumerate_required_extensions(window.raw_display_handle())
                    .unwrap()
                    .to_vec();
            extension_names_raw.push(DebugUtils::name().as_ptr());
            println!(
                "Extensions: {:?}",
                extension_names_raw
                    .iter()
                    .map(|ptr| CStr::from_ptr(*ptr).to_str().ok())
                    .collect::<Vec<Option<&str>>>()
            );

            // Layer names
            let layer_names = Verification::get_layers();
            println!("Layers: {:?}", layer_names);

            let layers_names_raw: Vec<*const c_char> =
                map_vec_ref(&layer_names, |name| name.as_ptr());

            // Create flags
            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                InstanceCreateFlags::default()
            };

            let create_info = InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(&extension_names_raw)
                .enabled_layer_names(&layers_names_raw)
                .flags(create_flags);

            let entry = Entry::load()?;
            let instance: Instance = entry.create_instance(&create_info, None)?;

            let verification = Verification::load(&entry, &instance)?;

            Ok(Self {
                entry,
                instance,
                verification,
            })
        }
    }

    fn get_info(&self) -> crate::Result<String> {
        todo!()
    }
}
