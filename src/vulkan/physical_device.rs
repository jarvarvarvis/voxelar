use std::ffi::CStr;

use ash::extensions::khr::Surface;
use ash::vk::Extent2D;
use ash::vk::{MemoryPropertyFlags, MemoryRequirements};
use ash::vk::{
    PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties,
    PhysicalDeviceProperties,
};
use ash::vk::{QueueFamilyProperties, QueueFlags};
use ash::vk::{SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR};
use ash::Instance;

use crate::result::Context;

pub struct SetUpPhysicalDevice {
    pub device: PhysicalDevice,
    pub device_properties: PhysicalDeviceProperties,
    pub device_memory_properties: PhysicalDeviceMemoryProperties,
    pub device_features: PhysicalDeviceFeatures,

    pub queue_family_index: u32,
}

impl SetUpPhysicalDevice {
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

        let device_memory_properties = instance.get_physical_device_memory_properties(device);
        let device_properties = instance.get_physical_device_properties(device);
        let device_features = instance.get_physical_device_features(device);

        Ok(Self {
            device,
            device_memory_properties,
            device_properties,
            device_features,
            queue_family_index: queue_family_index as u32,
        })
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.device_properties.device_name.as_ptr()) }
    }

    pub fn get_surface_capabilities(
        &self,
        surface_loader: &Surface,
        surface: SurfaceKHR,
    ) -> crate::Result<SurfaceCapabilitiesKHR> {
        unsafe {
            let surface_capabilities =
                surface_loader.get_physical_device_surface_capabilities(self.device, surface)?;
            Ok(surface_capabilities)
        }
    }

    pub fn get_surface_format(
        &self,
        surface_loader: &Surface,
        surface: SurfaceKHR,
    ) -> crate::Result<SurfaceFormatKHR> {
        unsafe {
            let surface_format =
                surface_loader.get_physical_device_surface_formats(self.device, surface)?[0];
            Ok(surface_format)
        }
    }

    pub fn get_surface_extent(
        &self,
        surface_loader: &Surface,
        surface: SurfaceKHR,
        fallback_width: u32,
        fallback_height: u32,
    ) -> crate::Result<Extent2D> {
        let surface_capabilities = self.get_surface_capabilities(surface_loader, surface)?;
        Ok(match surface_capabilities.current_extent {
            Extent2D {
                width: std::u32::MAX,
                height: std::u32::MAX,
            } => Extent2D {
                width: fallback_width,
                height: fallback_height,
            },
            _ => surface_capabilities.current_extent,
        })
    }

    pub fn find_memory_type_index(
        &self,
        memory_req: &MemoryRequirements,
        flags: MemoryPropertyFlags,
    ) -> Option<u32> {
        let memory_prop = &self.device_memory_properties;
        memory_prop.memory_types[..memory_prop.memory_type_count as _]
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & memory_req.memory_type_bits != 0
                    && memory_type.property_flags & flags == flags
            })
            .map(|(index, _memory_type)| index as _)
    }
}
