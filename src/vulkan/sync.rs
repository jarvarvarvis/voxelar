use ash::vk::{Semaphore, SemaphoreCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct RenderingSyncPrimitives {
    pub present_complete_semaphore: Semaphore,
    pub rendering_complete_semaphore: Semaphore,
}

impl RenderingSyncPrimitives {
    pub unsafe fn create(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        let semaphore_create_info = SemaphoreCreateInfo::default();

        let present_complete_semaphore = virtual_device
            .device
            .create_semaphore(&semaphore_create_info, None)?;
        let rendering_complete_semaphore = virtual_device
            .device
            .create_semaphore(&semaphore_create_info, None)?;

        Ok(Self {
            present_complete_semaphore,
            rendering_complete_semaphore,
        })
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .destroy_semaphore(self.present_complete_semaphore, None);
            virtual_device
                .device
                .destroy_semaphore(self.rendering_complete_semaphore, None);
        }
    }
}
