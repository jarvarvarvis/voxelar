use std::ffi::c_void;

use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags};
use ash::vk::{
    DeviceMemory, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags, MemoryRequirements,
};

use crate::result::Context;

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct AllocatedBuffer {
    pub buffer_memory: DeviceMemory,
    pub buffer: Buffer,
}

impl AllocatedBuffer {
    pub unsafe fn allocate(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        size: u64,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_property_flags: MemoryPropertyFlags,
    ) -> crate::Result<Self> {
        let buffer_info = BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(sharing_mode);

        let buffer = virtual_device.device.create_buffer(&buffer_info, None)?;
        let buffer_memory_req = virtual_device.device.get_buffer_memory_requirements(buffer);
        let buffer_memory_index = physical_device
            .find_memory_type_index(&buffer_memory_req, memory_property_flags)
            .context("Unable to find suitable memory type for the buffer".to_string())?;

        let allocate_info = MemoryAllocateInfo {
            allocation_size: buffer_memory_req.size,
            memory_type_index: buffer_memory_index,
            ..Default::default()
        };

        let buffer_memory = virtual_device
            .device
            .allocate_memory(&allocate_info, None)?;

        virtual_device
            .device
            .bind_buffer_memory(buffer, buffer_memory, 0)?;

        Ok(Self {
            buffer_memory,
            buffer,
        })
    }

    pub unsafe fn get_buffer_memory_req(
        &self,
        virtual_device: &SetUpVirtualDevice,
    ) -> MemoryRequirements {
        virtual_device
            .device
            .get_buffer_memory_requirements(self.buffer)
    }

    pub unsafe fn map_memory(
        &self,
        virtual_device: &SetUpVirtualDevice,
    ) -> crate::Result<*mut c_void> {
        let buffer_memory_req = virtual_device
            .device
            .get_buffer_memory_requirements(self.buffer);
        let buffer_ptr = virtual_device.device.map_memory(
            self.buffer_memory,
            0,
            buffer_memory_req.size,
            MemoryMapFlags::empty(),
        )?;
        Ok(buffer_ptr)
    }

    pub unsafe fn unmap_memory(&self, virtual_device: &SetUpVirtualDevice) {
        virtual_device.device.unmap_memory(self.buffer_memory);
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device.device.free_memory(self.buffer_memory, None);
            virtual_device.device.destroy_buffer(self.buffer, None);
        }
    }
}
