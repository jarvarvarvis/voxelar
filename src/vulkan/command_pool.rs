use ash::vk::FenceCreateFlags;
use ash::vk::{CommandBufferAllocateInfo, CommandBufferLevel};
use ash::vk::{CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, CommandPoolResetFlags};

use super::command_buffer::SetUpCommandBufferWithFence;
use super::logical_device::SetUpLogicalDevice;

pub struct SetUpCommandPool {
    pub pool: CommandPool,
    pub command_buffers: Vec<SetUpCommandBufferWithFence>,
}

impl SetUpCommandPool {
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        command_buffer_count: u32,
        level: CommandBufferLevel,
        command_buffer_fence_create_flags: FenceCreateFlags,
    ) -> crate::Result<Self> {
        let pool_create_info = CommandPoolCreateInfo::builder()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(logical_device.queue_family_index);

        let pool = logical_device.create_command_pool(&pool_create_info, None)?;

        let command_buffer_allocate_info = CommandBufferAllocateInfo::builder()
            .command_buffer_count(command_buffer_count)
            .command_pool(pool)
            .level(level);

        let just_command_buffers =
            logical_device.allocate_command_buffers(&command_buffer_allocate_info)?;

        let mut command_buffers = Vec::with_capacity(just_command_buffers.len());
        for command_buffer in just_command_buffers {
            command_buffers.push(SetUpCommandBufferWithFence::create(
                logical_device,
                command_buffer,
                command_buffer_fence_create_flags,
            )?);
        }

        Ok(Self {
            pool,
            command_buffers,
        })
    }

    pub fn get_command_buffer(&self, index: usize) -> &SetUpCommandBufferWithFence {
        &self.command_buffers[index]
    }

    pub fn reset(
        &self,
        logical_device: &SetUpLogicalDevice,
        flags: CommandPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            logical_device.reset_command_pool(self.pool, flags)?;
            Ok(())
        }
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            // NOTE: It's not necessary to strictly destroy the command buffers
            //       created from the pool. When destroy_command_pool is called,
            //       all command buffers created from that pool will be destroyed as
            //       well.
            for command_buffer in self.command_buffers.iter_mut() {
                command_buffer.destroy(&logical_device);
            }
            logical_device.destroy_command_pool(self.pool, None);
        }
    }
}
