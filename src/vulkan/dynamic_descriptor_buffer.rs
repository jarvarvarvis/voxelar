use std::marker::PhantomData;
use std::sync::MutexGuard;

use ash::vk::BufferUsageFlags;
use ash::vk::MappedMemoryRange;
use ash::vk::SharingMode;
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use super::buffer::AllocatedBuffer;
use super::logical_device::SetUpLogicalDevice;
use super::physical_device::SetUpPhysicalDevice;

pub struct DynamicDescriptorBuffer<T> {
    pub buffer: AllocatedBuffer,
    pub aligned_size_of_type: u64,
    phantom: PhantomData<T>,
}

impl<T> DynamicDescriptorBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        count: usize,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self> {
        let size = std::mem::size_of::<T>() as u64;
        let alignment = physical_device
            .device_properties
            .limits
            .min_uniform_buffer_offset_alignment;
        let aligned_size_of_type = super::util::pad_uniform_buffer_size(size, alignment);
        Ok(Self {
            buffer: AllocatedBuffer::allocate(
                logical_device,
                allocator,
                count as u64 * aligned_size_of_type,
                usage,
                sharing_mode,
                memory_location,
            )?,
            aligned_size_of_type,
            phantom: PhantomData,
        })
    }

    pub unsafe fn allocate_uniform_buffer(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        count: usize,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<Self> {
        Self::allocate(
            logical_device,
            physical_device,
            allocator,
            count,
            BufferUsageFlags::UNIFORM_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryLocation::CpuToGpu,
        )
    }

    pub unsafe fn flush_memory(
        &self,
        logical_device: &SetUpLogicalDevice,
        index: usize,
    ) -> crate::Result<()> {
        let offset = self.get_dynamic_offset(index);
        let memory_range = MappedMemoryRange::builder()
            .memory(self.buffer.allocation()?.memory())
            .size(self.aligned_size_of_type)
            .offset(self.buffer.allocation()?.offset() + offset as u64)
            .build();

        logical_device.flush_mapped_memory_ranges(&[memory_range])?;
        Ok(())
    }

    pub unsafe fn mapped_ptr(&self) -> crate::Result<*mut T> {
        Ok(self.buffer.get_mapped_ptr()?.as_ptr() as *mut T)
    }

    pub unsafe fn store_at(
        &self,
        logical_device: &SetUpLogicalDevice,
        value: T,
        index: usize,
    ) -> crate::Result<()> {
        let ptr = self.mapped_ptr()?;
        let offset = self.get_dynamic_offset(index);

        // Need to cast to *mut u8 to allow byte-level offsets
        let ptr = ptr.cast::<u8>().offset(offset as isize).cast();
        *ptr = value;

        self.flush_memory(logical_device, index)?;
        Ok(())
    }

    pub fn get_dynamic_offset(&self, index: usize) -> u32 {
        self.aligned_size_of_type as u32 * index as u32
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}
