use ash::vk::PipelineStageFlags;
use ash::vk::Queue;
use ash::vk::SubmitInfo;
use ash::vk::{
    CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
    CommandBufferResetFlags, CommandBufferUsageFlags,
};
use ash::vk::{CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo};
use ash::vk::{Fence, Semaphore};
use ash::Device;

use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpCommandLogic {
    pub pool: CommandPool,
    pub command_buffers: Vec<CommandBuffer>,
}

impl SetUpCommandLogic {
    pub unsafe fn create(
        virtual_device: &SetUpVirtualDevice,
        command_buffer_count: u32,
        level: CommandBufferLevel,
    ) -> crate::Result<Self> {
        let pool_create_info = CommandPoolCreateInfo::builder()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(virtual_device.queue_family_index);

        let pool = virtual_device
            .device
            .create_command_pool(&pool_create_info, None)?;

        let command_buffer_allocate_info = CommandBufferAllocateInfo::builder()
            .command_buffer_count(command_buffer_count)
            .command_pool(pool)
            .level(level);

        let command_buffers = virtual_device
            .device
            .allocate_command_buffers(&command_buffer_allocate_info)?;

        Ok(Self {
            pool,
            command_buffers,
        })
    }

    pub unsafe fn create_with_defaults(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        Self::create(virtual_device, 2, CommandBufferLevel::PRIMARY)
    }

    pub fn get_command_buffer(&self, index: usize) -> &CommandBuffer {
        &self.command_buffers[index]
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device.device.destroy_command_pool(self.pool, None);
        }
    }
}

/// Helper function for submitting command buffers. Immediately waits for the fence before the command buffer
/// is executed. That way we can delay the waiting for the fences by 1 frame which is good for performance.
/// Make sure to create the fence in a signaled state on the first use.
#[allow(clippy::too_many_arguments)]
pub fn submit_command_buffer<F: FnOnce(&Device, CommandBuffer) -> crate::Result<()>>(
    device: &Device,
    command_buffer: CommandBuffer,
    command_buffer_reuse_fence: Fence,
    submit_queue: Queue,
    wait_mask: &[PipelineStageFlags],
    wait_semaphores: &[Semaphore],
    signal_semaphores: &[Semaphore],
    f: F,
) -> crate::Result<()> {
    unsafe {
        device.wait_for_fences(&[command_buffer_reuse_fence], true, std::u64::MAX)?;
        device.reset_fences(&[command_buffer_reuse_fence])?;

        device.reset_command_buffer(command_buffer, CommandBufferResetFlags::RELEASE_RESOURCES)?;

        let command_buffer_begin_info =
            CommandBufferBeginInfo::builder().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        device.begin_command_buffer(command_buffer, &command_buffer_begin_info)?;
        f(device, command_buffer)?;
        device.end_command_buffer(command_buffer)?;

        let command_buffers = vec![command_buffer];

        let submit_info = SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_mask)
            .command_buffers(&command_buffers)
            .signal_semaphores(signal_semaphores)
            .build();

        device.queue_submit(submit_queue, &[submit_info], command_buffer_reuse_fence)?;
    }

    Ok(())
}
