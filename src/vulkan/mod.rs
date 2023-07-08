use std::ffi::{c_char, CStr, CString};

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::vk;
use ash::vk::ApplicationInfo;
use ash::vk::SurfaceKHR;
use ash::vk::{InstanceCreateFlags, InstanceCreateInfo};
use ash::{Entry, Instance};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};

pub mod debug;
pub mod physical_device;
pub mod swapchain;
pub mod util;
pub mod virtual_device;

use crate::render_context::RenderContext;
use crate::result::Context;
use crate::window::VoxelarWindow;
use crate::Voxelar;

use self::debug::VerificationProvider;
use self::physical_device::SetUpPhysicalDevice;
use self::swapchain::SetUpSwapchain;
use self::virtual_device::SetUpVirtualDevice;

pub struct VulkanContext<Verification: VerificationProvider> {
    pub entry: Entry,
    pub instance: Instance,

    pub verification: Verification,

    pub surface_loader: Surface,
    pub surface: SurfaceKHR,

    pub physical_device: Option<SetUpPhysicalDevice>,
    pub virtual_device: Option<SetUpVirtualDevice>,
    pub swapchain: Option<SetUpSwapchain>,
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

    pub fn find_usable_physical_device(&mut self) -> crate::Result<()> {
        unsafe {
            self.physical_device = Some(SetUpPhysicalDevice::find_usable_device(
                &self.instance,
                &self.surface_loader,
                self.surface,
            )?);
        }
        Ok(())
    }

    pub fn create_virtual_device(&mut self) -> crate::Result<()> {
        let physical_device = self.physical_device
            .as_ref()
            .context("No physical device was set up yet! Use VulkanContext::find_usable_physical_device to do so".to_string())?;

        unsafe {
            self.virtual_device = Some(SetUpVirtualDevice::create_with_defaults(
                &self.instance,
                &physical_device,
            )?);
        }

        Ok(())
    }

    pub fn create_swapchain(&mut self, window_size: (i32, i32)) -> crate::Result<()> {
        let physical_device = self.physical_device
            .as_ref()
            .context("No physical device was set up yet! Use VulkanContext::find_usable_physical_device to do so".to_string())?;
        let virtual_device = self.virtual_device.as_ref().context(
            "No virtual device was set up yet! Use VulkanContext::create_virtual_device to do so"
                .to_string(),
        )?;

        unsafe {
            self.swapchain = Some(SetUpSwapchain::create_with_defaults(
                &self.instance,
                &self.surface_loader,
                self.surface,
                physical_device,
                virtual_device,
                window_size.0 as u32,
                window_size.1 as u32,
            )?);
        }

        Ok(())
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

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                extension_names_raw.push(KhrPortabilityEnumerationFn::name().as_ptr());
                // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
                extension_names_raw.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
            }

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
                util::map_vec_ref(&layer_names, |name| name.as_ptr());

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

            let surface = ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )?;

            let surface_loader = Surface::new(&entry, &instance);

            Ok(Self {
                entry,
                instance,
                verification,
                surface,
                surface_loader,
                physical_device: None,
                virtual_device: None,
                swapchain: None,
            })
        }
    }

    fn get_info(&self) -> crate::Result<String> {
        todo!()
    }
}

impl<Verification: VerificationProvider> Drop for VulkanContext<Verification> {
    fn drop(&mut self) {
        unsafe {
            if let Some(device) = self.virtual_device.as_mut() {
                device.wait();

                if let Some(swapchain) = self.swapchain.as_mut() {
                    swapchain.destroy();
                }

                device.destroy();
            }
            self.surface_loader.destroy_surface(self.surface, None);
            self.verification.destroy();
            self.instance.destroy_instance(None);
        }
    }
}
