use ash::vk::{CommandBufferAllocateInfo, CommandBufferLevel};
use ash::vk::{CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo};

use super::command_buffer::SetUpCommandBufferWithFence;
use super::virtual_device::SetUpVirtualDevice;

pub struct SetUpCommandLogic {
    pub pool: CommandPool,
    pub command_buffers: Vec<SetUpCommandBufferWithFence>,
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

        let just_command_buffers = virtual_device
            .device
            .allocate_command_buffers(&command_buffer_allocate_info)?;

        let mut command_buffers = Vec::with_capacity(just_command_buffers.len());
        for command_buffer in just_command_buffers {
            command_buffers.push(SetUpCommandBufferWithFence::create(
                virtual_device,
                command_buffer,
            )?);
        }

        Ok(Self {
            pool,
            command_buffers,
        })
    }

    pub unsafe fn create_with_defaults(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        Self::create(virtual_device, 2, CommandBufferLevel::PRIMARY)
    }

    pub fn get_command_buffer(&self, index: usize) -> &SetUpCommandBufferWithFence {
        &self.command_buffers[index]
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            for command_buffer in self.command_buffers.iter_mut() {
                command_buffer.destroy(self.pool, &virtual_device);
            }
            virtual_device.device.destroy_command_pool(self.pool, None);
        }
    }
}
