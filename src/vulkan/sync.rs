use ash::vk::{Semaphore, SemaphoreCreateInfo};

use super::logical_device::SetUpLogicalDevice;

pub struct RenderingSyncPrimitives {
    pub present_complete_semaphore: Semaphore,
    pub rendering_complete_semaphore: Semaphore,
}

impl RenderingSyncPrimitives {
    pub unsafe fn create(logical_device: &SetUpLogicalDevice) -> crate::Result<Self> {
        let semaphore_create_info = SemaphoreCreateInfo::default();

        let present_complete_semaphore = logical_device
            .device
            .create_semaphore(&semaphore_create_info, None)?;
        let rendering_complete_semaphore = logical_device
            .device
            .create_semaphore(&semaphore_create_info, None)?;

        Ok(Self {
            present_complete_semaphore,
            rendering_complete_semaphore,
        })
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device
                .device
                .destroy_semaphore(self.present_complete_semaphore, None);
            logical_device
                .device
                .destroy_semaphore(self.rendering_complete_semaphore, None);
        }
    }
}
