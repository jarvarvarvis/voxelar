use ash::vk::{DeviceMemory, MemoryPropertyFlags, MemoryRequirements};

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

#[derive(Debug, Clone, Copy)]
pub struct Allocation {
    pub memory: DeviceMemory,
    pub offset: u64,
    pub size: u64,
}

pub trait Allocator {
    fn new() -> Self
    where
        Self: Sized;

    fn setup(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
    ) -> crate::Result<()>;

    fn allocate(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        memory_requirements: MemoryRequirements,
        memory_properties: MemoryPropertyFlags,
    ) -> crate::Result<Allocation>;

    fn deallocate(&self, virtual_device: &SetUpVirtualDevice, allocation: Allocation);

    fn destroy(&mut self, virtual_device: &SetUpVirtualDevice);
}
