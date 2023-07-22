use ash::extensions::khr::Swapchain;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::KhrPortabilitySubsetFn;
use ash::vk::PhysicalDeviceFeatures;
use ash::vk::Queue;
use ash::vk::{DeviceCreateInfo, DeviceQueueCreateInfo};
use ash::{Device, Instance};

use super::physical_device::SetUpPhysicalDevice;

pub struct SetUpVirtualDevice {
    pub device: Device,

    pub present_queue: Queue,
    pub queue_family_index: u32,
}

impl SetUpVirtualDevice {
    pub unsafe fn create(
        instance: &Instance,
        physical_device_info: &SetUpPhysicalDevice,
        device_extension_names_raw: &[*const i8],
        features: PhysicalDeviceFeatures,
        priorities: &[f32],
    ) -> crate::Result<Self> {
        let queue_info = DeviceQueueCreateInfo::builder()
            .queue_family_index(physical_device_info.queue_family_index)
            .queue_priorities(&priorities);

        let device_create_info = DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device =
            instance.create_device(physical_device_info.device, &device_create_info, None)?;

        let present_queue = device.get_device_queue(physical_device_info.queue_family_index, 0);

        Ok(Self {
            device,
            present_queue,
            queue_family_index: physical_device_info.queue_family_index,
        })
    }

    pub unsafe fn create_with_defaults(
        instance: &Instance,
        physical_device_info: &SetUpPhysicalDevice,
    ) -> crate::Result<Self> {
        let device_extension_names_raw = [
            Swapchain::name().as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            KhrPortabilitySubsetFn::name().as_ptr(),
        ];
        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];

        Self::create(
            instance,
            physical_device_info,
            &device_extension_names_raw,
            features,
            &priorities,
        )
    }

    pub fn wait(&self) -> crate::Result<()> {
        unsafe {
            self.device.device_wait_idle()?;
            Ok(())
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
