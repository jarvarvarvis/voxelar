use ash::vk::{MemoryAllocateInfo, MemoryPropertyFlags, MemoryRequirements};

use crate::result::Context;

use super::allocator::{Allocation, Allocator};
use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct NaiveAllocator;

impl Allocator for NaiveAllocator {
    fn new() -> Self {
        Self
    }

    fn setup(&self, _: &SetUpVirtualDevice, _: &SetUpPhysicalDevice) -> crate::Result<()> {
        todo!()
    }

    fn allocate(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        memory_requirements: MemoryRequirements,
        memory_properties: MemoryPropertyFlags,
    ) -> crate::Result<Allocation> {
        unsafe {
            let memory_type_index = physical_device
                .find_memory_type_index(&memory_requirements, memory_properties)
                .context("Unable to find suitable memory type for the buffer".to_string())?;

            let allocate_info = MemoryAllocateInfo {
                allocation_size: memory_requirements.size,
                memory_type_index,
                ..Default::default()
            };

            let memory = virtual_device
                .device
                .allocate_memory(&allocate_info, None)?;

            let allocation = Allocation { memory, offset: 0 };

            #[cfg(feature = "allocator-debug-logs")]
            {
                println!("===== DedicatedAllocator - Allocation =====");
                println!("Memory requirements: {memory_requirements:?}");
                println!("Memory properties: {memory_properties:?}");
                println!("Memory type index: {memory_type_index}");
                println!("Made allocation: {allocation:?}");
                println!("===========================================\n");
            }

            Ok(allocation)
        }
    }

    fn deallocate(&self, virtual_device: &SetUpVirtualDevice, allocation: Allocation) {
        unsafe {
            virtual_device.device.free_memory(allocation.memory, None);
        }
    }

    fn destroy(&mut self, _: &SetUpVirtualDevice) {}
}
