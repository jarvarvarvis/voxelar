use ash::vk::CommandBufferLevel;
use ash::vk::CommandBufferResetFlags;
use ash::vk::FenceCreateFlags;
use ash::vk::PipelineStageFlags;
use ash::vk::Queue;

use super::command::command_buffer::SetUpCommandBufferWithFence;
use super::command::command_pool::SetUpCommandPool;
use super::logical_device::SetUpLogicalDevice;
use super::sync::RenderingSyncPrimitives;

pub struct FrameData {
    pub sync_primitives: RenderingSyncPrimitives,
    pub command_pool: SetUpCommandPool,
}

impl FrameData {
    pub unsafe fn create_with_defaults(logical_device: &SetUpLogicalDevice) -> crate::Result<Self> {
        let sync_primitives = RenderingSyncPrimitives::create(logical_device)?;
        let command_pool = SetUpCommandPool::create(
            logical_device,
            1,
            CommandBufferLevel::PRIMARY,
            FenceCreateFlags::SIGNALED,
        )?;

        Ok(Self {
            sync_primitives,
            command_pool,
        })
    }

    pub fn draw_buffers_count(&self) -> usize {
        self.command_pool.command_buffers.len()
    }

    pub fn wait_for_draw_buffer_fence(
        &self,
        logical_device: &SetUpLogicalDevice,
        draw_buffer_index: usize,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_pool.get_command_buffer(draw_buffer_index);
        draw_command_buffer.wait_for_fence(logical_device)
    }

    pub fn reset_draw_buffer_fence(
        &self,
        logical_device: &SetUpLogicalDevice,
        draw_buffer_index: usize,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_pool.get_command_buffer(draw_buffer_index);
        draw_command_buffer.reset_fence(logical_device)
    }

    pub fn reset_draw_buffer(
        &self,
        logical_device: &SetUpLogicalDevice,
        draw_buffer_index: usize,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_pool.get_command_buffer(draw_buffer_index);
        draw_command_buffer.reset(logical_device, CommandBufferResetFlags::RELEASE_RESOURCES)
    }

    pub fn record_draw_buffer_commands<
        F: FnOnce(&SetUpLogicalDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    >(
        &self,
        logical_device: &SetUpLogicalDevice,
        draw_buffer_index: usize,
        f: F,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_pool.get_command_buffer(draw_buffer_index);
        draw_command_buffer.record_commands_for_one_time_submit(logical_device, f)
    }

    pub fn submit_draw_buffer_to_queue(
        &self,
        logical_device: &SetUpLogicalDevice,
        draw_buffer_index: usize,
        present_queue: Queue,
        wait_mask: &[PipelineStageFlags],
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_pool.get_command_buffer(draw_buffer_index);
        draw_command_buffer.submit(
            logical_device,
            present_queue,
            wait_mask,
            &[self.sync_primitives.present_complete_semaphore],
            &[self.sync_primitives.rendering_complete_semaphore],
        )
    }

    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        self.sync_primitives.destroy(logical_device);
        self.command_pool.destroy(logical_device);
    }
}
