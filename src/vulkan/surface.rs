use ash::extensions::khr::Surface;
use ash::vk::Extent2D;
use ash::vk::PresentModeKHR;
use ash::vk::{SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR};
use ash::{Entry, Instance};

use crate::result::Context;
use crate::window::VoxelarWindow;

use super::physical_device::SetUpPhysicalDevice;

pub struct SetUpSurfaceInfo {
    pub surface_loader: Surface,
    pub surface: SurfaceKHR,

    pub surface_formats: Option<Vec<SurfaceFormatKHR>>,
    pub surface_extent: Option<Extent2D>,
    pub surface_capabilities: Option<SurfaceCapabilitiesKHR>,
    pub surface_present_modes: Option<Vec<PresentModeKHR>>,
}

impl SetUpSurfaceInfo {
    pub unsafe fn create(
        window: &VoxelarWindow,
        entry: &Entry,
        instance: &Instance,
    ) -> crate::Result<Self> {
        let surface = ash_window::create_surface(
            &entry,
            &instance,
            window.raw_display_handle(),
            window.raw_window_handle(),
            None,
        )?;

        let surface_loader = Surface::new(&entry, &instance);

        Ok(Self {
            surface_loader,
            surface,

            surface_formats: None,
            surface_extent: None,
            surface_capabilities: None,
            surface_present_modes: None,
        })
    }

    pub fn update(
        &mut self,
        physical_device: &SetUpPhysicalDevice,
        fallback_size: (u32, u32),
    ) -> crate::Result<()> {
        unsafe {
            self.surface_formats = Some(self.surface_loader.get_physical_device_surface_formats(
                physical_device.physical_device,
                self.surface,
            )?);

            let surface_capabilities = self
                .surface_loader
                .get_physical_device_surface_capabilities(
                    physical_device.physical_device,
                    self.surface,
                )?;

            self.surface_extent = Some(match surface_capabilities.current_extent {
                Extent2D {
                    width: std::u32::MAX,
                    height: std::u32::MAX,
                } => Extent2D {
                    width: fallback_size.0 as u32,
                    height: fallback_size.1 as u32,
                },
                _ => surface_capabilities.current_extent,
            });

            self.surface_capabilities = Some(surface_capabilities);

            self.surface_present_modes = Some(
                self.surface_loader
                    .get_physical_device_surface_present_modes(
                        physical_device.physical_device,
                        self.surface,
                    )?,
            );

            Ok(())
        }
    }

    pub fn surface_format(&self, index: usize) -> crate::Result<SurfaceFormatKHR> {
        let formats = self
            .surface_formats
            .as_ref()
            .context("Surface formats weren't queried yet or querying failed".to_string())?;
        Ok(formats[index])
    }

    pub fn surface_extent(&self) -> crate::Result<Extent2D> {
        self.surface_extent
            .context("Surface extent wasn't queried yet or querying failed".to_string())
    }

    pub fn surface_capabilities(&self) -> crate::Result<SurfaceCapabilitiesKHR> {
        self.surface_capabilities
            .context("Surface capabilities weren't queried yet or querying failed".to_string())
    }

    pub fn surface_present_modes(&self) -> crate::Result<Vec<PresentModeKHR>> {
        self.surface_present_modes
            .clone()
            .context("Surface present modes weren't queried yet or querying failed".to_string())
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}
