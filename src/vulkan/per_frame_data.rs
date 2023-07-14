use ash::vk::PipelineStageFlags;
use ash::vk::Queue;

use super::command::SetUpCommandLogic;
use super::command_buffer::SetUpCommandBufferWithFence;
use super::sync::PerFrameSyncPrimitives;
use super::virtual_device::SetUpVirtualDevice;

pub struct PerFrameData {
    pub sync_primitives: PerFrameSyncPrimitives,
    pub command_logic: SetUpCommandLogic,
}

impl PerFrameData {
    pub unsafe fn create_with_defaults(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        let sync_primitives = PerFrameSyncPrimitives::create(virtual_device)?;
        let command_logic = SetUpCommandLogic::create_with_one_primary_buffer(virtual_device)?;

        Ok(Self {
            sync_primitives,
            command_logic,
        })
    }

    pub fn submit_to_draw_buffer<
        F: FnOnce(&SetUpVirtualDevice, &SetUpCommandBufferWithFence) -> crate::Result<()>,
    >(
        &self,
        virtual_device: &SetUpVirtualDevice,
        present_queue: Queue,
        wait_mask: &[PipelineStageFlags],
        f: F,
    ) -> crate::Result<()> {
        let draw_command_buffer = self.command_logic.get_command_buffer(0);
        draw_command_buffer.submit(
            virtual_device,
            present_queue,
            wait_mask,
            &[self.sync_primitives.present_complete_semaphore],
            &[self.sync_primitives.rendering_complete_semaphore],
            f,
        )
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        self.sync_primitives.destroy(virtual_device);
        self.command_logic.destroy(virtual_device);
    }
}
