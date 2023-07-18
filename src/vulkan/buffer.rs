use std::ffi::c_void;

use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags};
use ash::vk::{MemoryMapFlags, MemoryPropertyFlags, MemoryRequirements};

use super::allocator::{Allocation, Allocator};
use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct AllocatedBuffer {
    pub buffer_allocation: Allocation,
    pub buffer: Buffer,
}

impl AllocatedBuffer {
    pub unsafe fn allocate(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &dyn Allocator,
        size: u64,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_properties: MemoryPropertyFlags,
    ) -> crate::Result<Self> {
        let buffer_info = BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(sharing_mode);

        let buffer = virtual_device.device.create_buffer(&buffer_info, None)?;
        let memory_requirements = virtual_device.device.get_buffer_memory_requirements(buffer);

        let buffer_allocation = allocator.allocate(
            virtual_device,
            physical_device,
            memory_requirements,
            memory_properties,
        )?;

        virtual_device.device.bind_buffer_memory(
            buffer,
            buffer_allocation.memory,
            buffer_allocation.offset,
        )?;

        Ok(Self {
            buffer_allocation,
            buffer,
        })
    }

    pub fn get_buffer_memory_req(&self, virtual_device: &SetUpVirtualDevice) -> MemoryRequirements {
        unsafe {
            virtual_device
                .device
                .get_buffer_memory_requirements(self.buffer)
        }
    }

    pub fn map_memory(&self, virtual_device: &SetUpVirtualDevice) -> crate::Result<*mut c_void> {
        unsafe {
            let buffer_memory_req = virtual_device
                .device
                .get_buffer_memory_requirements(self.buffer);
            let buffer_ptr = virtual_device.device.map_memory(
                self.buffer_allocation.memory,
                self.buffer_allocation.offset,
                buffer_memory_req.size,
                MemoryMapFlags::empty(),
            )?;
            Ok(buffer_ptr)
        }
    }

    pub fn unmap_memory(&self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device
                .device
                .unmap_memory(self.buffer_allocation.memory);
        }
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice, allocator: &dyn Allocator) {
        unsafe {
            allocator.deallocate(virtual_device, self.buffer_allocation);
            virtual_device.device.destroy_buffer(self.buffer, None);
        }
    }
}
