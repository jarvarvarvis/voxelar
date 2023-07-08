use std::ffi::CStr;

use ash::extensions::khr::Surface;
use ash::vk::SurfaceKHR;
use ash::vk::{PhysicalDevice, PhysicalDeviceMemoryProperties, PhysicalDeviceProperties};
use ash::vk::{QueueFamilyProperties, QueueFlags};
use ash::Instance;

use crate::result::Context;

pub struct PhysicalDeviceInfo {
    pub device: PhysicalDevice,
    pub device_properties: PhysicalDeviceProperties,
    pub device_memory_properties: PhysicalDeviceMemoryProperties,
    pub queue_family_index: u32,
}

impl PhysicalDeviceInfo {
    unsafe fn is_device_suitable(
        info: &QueueFamilyProperties,
        device: &PhysicalDevice,
        surface_loader: &Surface,
        index: u32,
        surface: SurfaceKHR,
    ) -> bool {
        info.queue_flags.contains(QueueFlags::GRAPHICS)
            && surface_loader
                .get_physical_device_surface_support(*device, index, surface)
                .unwrap()
    }

    pub unsafe fn find_usable_device(
        instance: &Instance,
        surface_loader: &Surface,
        surface: SurfaceKHR,
    ) -> crate::Result<Self> {
        let pdevices = instance.enumerate_physical_devices()?;
        let (device, queue_family_index) = pdevices
            .iter()
            .find_map(|pdevice| {
                instance
                    .get_physical_device_queue_family_properties(*pdevice)
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        let supports_graphic_and_surface = Self::is_device_suitable(
                            &info,
                            &pdevice,
                            &surface_loader,
                            index as u32,
                            surface,
                        );
                        if supports_graphic_and_surface {
                            Some((*pdevice, index))
                        } else {
                            None
                        }
                    })
            })
            .context("Unable to find usable physical device".to_string())?;

        Ok(Self {
            device,
            device_memory_properties: instance.get_physical_device_memory_properties(device),
            device_properties: instance.get_physical_device_properties(device),
            queue_family_index: queue_family_index as u32,
        })
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.device_properties.device_name.as_ptr()) }
    }
}
