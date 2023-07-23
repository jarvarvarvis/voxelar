use std::marker::PhantomData;
use std::sync::MutexGuard;

use ash::util::Align;
use ash::vk::BufferUsageFlags;
use ash::vk::MappedMemoryRange;
use ash::vk::SharingMode;
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use super::buffer::AllocatedBuffer;
use super::logical_device::SetUpLogicalDevice;

pub struct AlignedBuffer<T> {
    pub buffer: AllocatedBuffer,
    pub alignment: usize,
    pub aligned_size_of_type: usize,
    pub element_count: usize,
    phantom: PhantomData<T>,
}

impl<T> AlignedBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_count: usize,
        alignment: usize,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self> {
        let size_of_type = std::mem::size_of::<T>();
        let aligned_size_of_type =
            super::util::pad_buffer_size_for_alignment(size_of_type, alignment);
        Ok(Self {
            buffer: AllocatedBuffer::allocate(
                logical_device,
                allocator,
                element_count * aligned_size_of_type,
                usage,
                sharing_mode,
                memory_location,
            )?,
            alignment,
            aligned_size_of_type,
            element_count,
            phantom: PhantomData,
        })
    }

    pub unsafe fn flush_memory(
        &self,
        logical_device: &SetUpLogicalDevice,
        index: usize,
    ) -> crate::Result<()> {
        let offset = self.get_offset(index);
        let memory_range = MappedMemoryRange::builder()
            .memory(self.buffer.allocation()?.memory())
            .size(self.aligned_size_of_type as u64)
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
        let offset = self.get_offset(index);

        // Need to cast to *mut u8 to allow byte-level offsets
        let ptr = ptr.cast::<u8>().offset(offset as isize).cast();
        *ptr = value;

        self.flush_memory(logical_device, index)?;
        Ok(())
    }

    pub unsafe fn copy_from_slice(&self, data: &[T]) -> crate::Result<()>
    where
        T: Copy,
    {
        crate::verify!(
            data.len() as usize == self.element_count,
            "The provided data slice must have the same size as the source buffer"
        );

        let size = self.buffer.allocation()?.size();

        let buffer_ptr = self.mapped_ptr()? as *mut std::ffi::c_void;

        let mut buffer_align = Align::new(buffer_ptr, self.alignment as u64, size);
        buffer_align.copy_from_slice(data);
        Ok(())
    }

    pub fn get_offset(&self, index: usize) -> usize {
        self.aligned_size_of_type * index
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}

impl<T> std::ops::Deref for AlignedBuffer<T> {
    type Target = AllocatedBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
