use std::marker::PhantomData;
use std::mem::align_of;

use ash::util::Align;
use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags};
use ash::vk::{DeviceMemory, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags};

use crate::result::Context;

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct AllocatedBuffer<T> {
    pub buffer_memory: DeviceMemory,
    pub buffer: Buffer,
    phantom: PhantomData<T>
}

impl<T: Copy> AllocatedBuffer<T> {
    pub unsafe fn allocate(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_property_flags: MemoryPropertyFlags,
    ) -> crate::Result<Self> {
        let buffer_info = BufferCreateInfo::builder()
            .size(std::mem::size_of::<T>() as u64)
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
            phantom: PhantomData
        })
    }

    pub unsafe fn create_from_data_slice(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        data: &[T],
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_property_flags: MemoryPropertyFlags,
    ) -> crate::Result<Self> {
        let buffer_info = BufferCreateInfo::builder()
            .size((std::mem::size_of::<T>() * data.len()) as u64)
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
        let buffer_ptr = virtual_device.device.map_memory(
            buffer_memory,
            0,
            buffer_memory_req.size,
            MemoryMapFlags::empty(),
        )?;
        let mut buffer_align =
            Align::new(buffer_ptr, align_of::<T>() as u64, buffer_memory_req.size);
        buffer_align.copy_from_slice(data);
        virtual_device.device.unmap_memory(buffer_memory);
        virtual_device
            .device
            .bind_buffer_memory(buffer, buffer_memory, 0)?;

        Ok(Self {
            buffer,
            buffer_memory,
            phantom: PhantomData
        })
    }

    pub unsafe fn create_vertex_buffer(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        data: &[T],
    ) -> crate::Result<Self> {
        Self::create_from_data_slice(
            virtual_device,
            physical_device,
            data,
            BufferUsageFlags::VERTEX_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )
    }

    pub unsafe fn create_index_buffer(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        data: &[T],
    ) -> crate::Result<Self> {
        Self::create_from_data_slice(
            virtual_device,
            physical_device,
            data,
            BufferUsageFlags::INDEX_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device.device.free_memory(self.buffer_memory, None);
            virtual_device.device.destroy_buffer(self.buffer, None);
        }
    }
}
