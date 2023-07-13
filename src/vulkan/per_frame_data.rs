use super::command::SetUpCommandLogic;
use super::sync::InternalSyncPrimitives;
use super::virtual_device::SetUpVirtualDevice;

pub struct PerFrameData {
    pub sync_primitives: InternalSyncPrimitives,
    pub command_logic: SetUpCommandLogic,
}

impl PerFrameData {
    pub unsafe fn create_with_defaults(virtual_device: &SetUpVirtualDevice) -> crate::Result<Self> {
        let sync_primitives = InternalSyncPrimitives::create(virtual_device)?;
        let command_logic = SetUpCommandLogic::create_with_one_primary_buffer(virtual_device)?;

        Ok(Self {
            sync_primitives,
            command_logic,
        })
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        self.sync_primitives.destroy(virtual_device);
        self.command_logic.destroy(virtual_device);
    }
}
