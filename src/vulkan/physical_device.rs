use std::ffi::CStr;

use ash::extensions::khr::Surface;
use ash::vk::SurfaceKHR;
use ash::vk::{MemoryPropertyFlags, MemoryRequirements, MemoryType};
use ash::vk::{
    PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceMemoryProperties,
    PhysicalDeviceProperties, PhysicalDeviceType,
};
use ash::vk::{QueueFamilyProperties, QueueFlags};
use ash::Instance;

use crate::result::Context;

use super::surface::SetUpSurfaceInfo;

pub struct SetUpPhysicalDevice {
    pub device: PhysicalDevice,
    pub device_properties: PhysicalDeviceProperties,
    pub device_memory_properties: PhysicalDeviceMemoryProperties,
    pub device_features: PhysicalDeviceFeatures,

    pub queue_family_index: u32,
}

impl SetUpPhysicalDevice {
    unsafe fn is_device_suitable_for_graphics(
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
        surface_info: &SetUpSurfaceInfo,
    ) -> crate::Result<Self> {
        let pdevices = instance.enumerate_physical_devices()?;
        let mut supported_physical_devices = pdevices
            .iter()
            .filter_map(|pdevice| {
                instance
                    .get_physical_device_queue_family_properties(*pdevice)
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        let device_suitable = Self::is_device_suitable_for_graphics(
                            &info,
                            &pdevice,
                            &surface_info.surface_loader,
                            index as u32,
                            surface_info.surface,
                        );
                        if device_suitable {
                            Some((*pdevice, index))
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<_>>();
        if supported_physical_devices.is_empty() {
            crate::bail!("No supported physical devices found!");
        }

        supported_physical_devices.sort_by_key(|(pdevice, _)| {
            let properties = instance.get_physical_device_properties(*pdevice);
            let device_type = properties.device_type;
            if device_type == PhysicalDeviceType::DISCRETE_GPU {
                2
            } else if device_type == PhysicalDeviceType::INTEGRATED_GPU {
                1
            } else {
                0
            }
        });

        let (device, queue_family_index) = *supported_physical_devices
            .iter()
            .max_by_key(|(pdevice, _)| {
                let properties = instance.get_physical_device_memory_properties(*pdevice);
                let mut heaps_size_sum = 0;
                for i in 0..(properties.memory_heap_count as usize) {
                    let heap_size = properties.memory_heaps[i].size;
                    heaps_size_sum += heap_size;
                }
                heaps_size_sum
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

    pub fn find_memory_type_index(
        &self,
        memory_requirements: &MemoryRequirements,
        memory_flags: MemoryPropertyFlags,
    ) -> Option<u32> {
        let memory_properties = &self.device_memory_properties;
        memory_properties.memory_types[..memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & memory_requirements.memory_type_bits != 0
                    && memory_type_supports_flags(**memory_type, memory_flags)
            })
            .map(|(index, _)| index as _)
    }
}

fn memory_type_supports_flags(memory_type: MemoryType, flags: MemoryPropertyFlags) -> bool {
    memory_type.property_flags & flags == flags
}
