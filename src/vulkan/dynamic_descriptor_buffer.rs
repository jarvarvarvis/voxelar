use std::marker::PhantomData;

use ash::vk::BufferUsageFlags;
use ash::vk::MappedMemoryRange;
use ash::vk::MemoryPropertyFlags;
use ash::vk::SharingMode;

use super::experimental::allocator::Allocator;
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
        allocator: &dyn Allocator,
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
                allocator,
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
        allocator: &dyn Allocator,
    ) -> crate::Result<Self> {
        Self::allocate(
            virtual_device,
            physical_device,
            allocator,
            count,
            BufferUsageFlags::UNIFORM_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE,
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

    pub unsafe fn flush_memory(
        &self,
        virtual_device: &SetUpVirtualDevice,
        index: usize,
    ) -> crate::Result<()> {
        let offset = self.get_dynamic_offset(index);
        let memory_range = MappedMemoryRange::builder()
            .memory(self.buffer.buffer_allocation.memory)
            .size(self.aligned_size_of_type)
            .offset(self.buffer.buffer_allocation.offset + offset as u64)
            .build();

        virtual_device
            .device
            .flush_mapped_memory_ranges(&[memory_range])?;
        Ok(())
    }

    pub unsafe fn store_at(
        &self,
        virtual_device: &SetUpVirtualDevice,
        value: T,
        index: usize,
    ) -> crate::Result<()> {
        let ptr = self.map_memory(virtual_device)?;
        let offset = self.get_dynamic_offset(index);

        // Need to cast to *mut u8 to allow byte-level offsets
        let ptr = ptr.cast::<u8>().offset(offset as isize).cast();
        *ptr = value;

        self.flush_memory(virtual_device, index)?;
        self.unmap_memory(virtual_device);
        Ok(())
    }

    pub fn get_dynamic_offset(&self, index: usize) -> u32 {
        self.aligned_size_of_type as u32 * index as u32
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice, allocator: &dyn Allocator) {
        self.buffer.destroy(virtual_device, allocator);
    }
}
