use std::marker::PhantomData;

use ash::vk::BufferUsageFlags;
use ash::vk::MemoryPropertyFlags;
use ash::vk::SharingMode;

use super::buffer::AllocatedBuffer;
use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct DynamicDescriptorBuffer<T> {
    pub buffer: AllocatedBuffer,
    pub aligned_size_of_type: u64,
    phantom: PhantomData<T>,
}

impl<T> DynamicDescriptorBuffer<T> {
    pub unsafe fn allocate(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        count: usize,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_property_flags: MemoryPropertyFlags,
    ) -> crate::Result<Self> {
        let size = std::mem::size_of::<T>() as u64;
        let alignment = physical_device
            .device_properties
            .limits
            .min_uniform_buffer_offset_alignment;
        let aligned_size_of_type = super::util::pad_uniform_buffer_size(size, alignment);
        Ok(Self {
            buffer: AllocatedBuffer::allocate(
                virtual_device,
                physical_device,
                count as u64 * aligned_size_of_type,
                usage,
                sharing_mode,
                memory_property_flags,
            )?,
            aligned_size_of_type,
            phantom: PhantomData,
        })
    }

    pub unsafe fn allocate_uniform_buffer(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        count: usize,
    ) -> crate::Result<Self> {
        Self::allocate(
            virtual_device,
            physical_device,
            count,
            BufferUsageFlags::UNIFORM_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )
    }

    pub unsafe fn map_memory(&self, virtual_device: &SetUpVirtualDevice) -> crate::Result<*mut T> {
        self.buffer
            .map_memory(virtual_device)
            .map(|value| value as *mut T)
    }

    pub unsafe fn unmap_memory(&self, virtual_device: &SetUpVirtualDevice) {
        self.buffer.unmap_memory(virtual_device);
    }

    pub unsafe fn store_at(
        &self,
        virtual_device: &SetUpVirtualDevice,
        value: T,
        index: usize,
    ) -> crate::Result<()> {
        let ptr = self.map_memory(virtual_device)?;
        let offset = self.get_dynamic_offset(index);
        *(ptr.byte_offset(offset as isize)) = value;
        self.unmap_memory(virtual_device);
        Ok(())
    }

    pub fn get_dynamic_offset(&self, index: usize) -> u32 {
        self.aligned_size_of_type as u32 * index as u32
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        self.buffer.destroy(virtual_device);
    }
}
