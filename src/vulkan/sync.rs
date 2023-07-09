use ash::vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateInfo};

use super::virtual_device::SetUpVirtualDevice;

pub struct InternalSyncPrimitives {
    pub present_complete_semaphore: Semaphore,
    pub rendering_complete_semaphore: Semaphore,

    pub draw_commands_reuse_fence: Fence,
    pub setup_commands_reuse_fence: Fence,
}

impl InternalSyncPrimitives {
    pub unsafe fn create(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        let semaphore_create_info = SemaphoreCreateInfo::default();

        let present_complete_semaphore = virtual_device
            .device
            .create_semaphore(&semaphore_create_info, None)?;
        let rendering_complete_semaphore = virtual_device
            .device
            .create_semaphore(&semaphore_create_info, None)?;

        let fence_create_info = FenceCreateInfo::builder().flags(FenceCreateFlags::SIGNALED);

        let draw_commands_reuse_fence = virtual_device
            .device
            .create_fence(&fence_create_info, None)?;
        let setup_commands_reuse_fence = virtual_device
            .device
            .create_fence(&fence_create_info, None)?;

        Ok(Self {
            present_complete_semaphore,
            rendering_complete_semaphore,
            draw_commands_reuse_fence,
            setup_commands_reuse_fence,
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
            virtual_device
                .device
                .destroy_fence(self.draw_commands_reuse_fence, None);
            virtual_device
                .device
                .destroy_fence(self.setup_commands_reuse_fence, None);
        }
    }
}
