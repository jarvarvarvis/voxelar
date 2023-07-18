use ash::vk::Fence;
use ash::vk::PipelineStageFlags;
use ash::vk::Queue;
use ash::vk::SubmitInfo;

use super::command::SetUpCommandLogic;
use super::command_buffer::SetUpCommandBufferWithFence;
use super::sync::RenderingSyncPrimitives;
use super::virtual_device::SetUpVirtualDevice;

pub struct FrameData {
    pub sync_primitives: RenderingSyncPrimitives,
    pub command_logic: SetUpCommandLogic,
}

impl FrameData {
    pub unsafe fn create_with_defaults(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        let sync_primitives = RenderingSyncPrimitives::create(virtual_device)?;
        let command_logic = SetUpCommandLogic::create_with_one_primary_buffer(virtual_device)?;

        Ok(Self {
            sync_primitives,
            command_logic,
        })
    }

    pub fn wait_for_draw_buffer_fence(
        &self,
        virtual_device: &SetUpVirtualDevice,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_logic.get_command_buffer(0);
        draw_command_buffer.wait_for_fence(virtual_device)
    }

    pub fn reset_draw_buffer(&self, virtual_device: &SetUpVirtualDevice) -> crate::Result<()> {
        let draw_command_buffer = self.command_logic.get_command_buffer(0);
        draw_command_buffer.reset(virtual_device)
    }

    pub fn record_draw_buffer_commands<
        F: FnOnce(&SetUpVirtualDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    >(
        &self,
        virtual_device: &SetUpVirtualDevice,
        f: F,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_logic.get_command_buffer(0);
        draw_command_buffer.record_commands(virtual_device, f)
    }

    pub fn submit_draw_buffer(
        &self,
        virtual_device: &SetUpVirtualDevice,
        present_queue: Queue,
        wait_mask: &[PipelineStageFlags],
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_logic.get_command_buffer(0);
        draw_command_buffer.submit(
            virtual_device,
            present_queue,
            wait_mask,
            &[self.sync_primitives.present_complete_semaphore],
            &[self.sync_primitives.rendering_complete_semaphore],
        )
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        self.sync_primitives.destroy(virtual_device);
        self.command_logic.destroy(virtual_device);
    }
}
