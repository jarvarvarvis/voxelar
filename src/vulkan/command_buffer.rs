use ash::vk::PipelineStageFlags;
use ash::vk::Queue;
use ash::vk::SubmitInfo;
use ash::vk::{
    CommandBuffer, CommandBufferBeginInfo, CommandBufferResetFlags, CommandBufferUsageFlags,
};
use ash::vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore};

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpCommandBufferWithFence {
    pub command_buffer: CommandBuffer,
    pub reuse_fence: Fence,
}

impl SetUpCommandBufferWithFence {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        command_buffer: CommandBuffer,
    ) -> crate::Result<Self> {
        let reuse_fence_create_info = FenceCreateInfo::builder().flags(FenceCreateFlags::SIGNALED);
        let reuse_fence = virtual_device
            .device
            .create_fence(&reuse_fence_create_info, None)?;
        Ok(SetUpCommandBufferWithFence {
            command_buffer,
            reuse_fence,
        })
    }

    pub fn submit<F: FnOnce(&SetUpVirtualDevice, &Self) -> crate::Result<()>>(
        &self,
        virtual_device: &SetUpVirtualDevice,
        submit_queue: Queue,
        wait_mask: &[PipelineStageFlags],
        wait_semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
        f: F,
    ) -> crate::Result<()> {
        unsafe {
            let device = &virtual_device.device;
            device.wait_for_fences(&[self.reuse_fence], true, std::u64::MAX)?;
            device.reset_fences(&[self.reuse_fence])?;

            device.reset_command_buffer(
                self.command_buffer,
                CommandBufferResetFlags::RELEASE_RESOURCES,
            )?;

            let command_buffer_begin_info =
                CommandBufferBeginInfo::builder().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            virtual_device
                .device
                .begin_command_buffer(self.command_buffer, &command_buffer_begin_info)?;
            f(&virtual_device, &self)?;
            virtual_device
                .device
                .end_command_buffer(self.command_buffer)?;

            let command_buffers = vec![self.command_buffer];

            let submit_info = SubmitInfo::builder()
                .wait_semaphores(wait_semaphores)
                .wait_dst_stage_mask(wait_mask)
                .command_buffers(&command_buffers)
                .signal_semaphores(signal_semaphores)
                .build();

            device.queue_submit(submit_queue, &[submit_info], self.reuse_fence)?;
        }

        Ok(())
    }

    pub fn destroy_fence(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device.device.destroy_fence(self.reuse_fence, None);
        }
    }
}
