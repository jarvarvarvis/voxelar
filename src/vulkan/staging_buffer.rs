use std::ffi::c_void;
use std::mem::align_of;
use std::sync::MutexGuard;

use ash::util::Align;
use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferUsageFlags};
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use super::logical_device::SetUpLogicalDevice;
use super::typed_buffer::TypedAllocatedBuffer;

pub struct SetUpStagingBuffer<T> {
    pub buffer: TypedAllocatedBuffer<T>,
}

impl<T> SetUpStagingBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_amount: usize,
    ) -> crate::Result<Self> {
        let buffer = TypedAllocatedBuffer::allocate(
            logical_device,
            allocator,
            element_amount,
            BufferUsageFlags::TRANSFER_SRC,
            SharingMode::EXCLUSIVE,
            MemoryLocation::CpuToGpu,
        )?;

        Ok(Self { buffer })
    }

    pub unsafe fn copy_from_slice(
        &self,
        logical_device: &SetUpLogicalDevice,
        data: &[T],
    ) -> crate::Result<()>
    where
        T: Copy,
    {
        crate::verify!(
            data.len() == self.buffer.element_amount,
            "The provided data slice must have the same size as the source buffer"
        );

        let buffer_memory_req = self.buffer.buffer.get_buffer_memory_req(logical_device);

        let buffer_ptr = self.buffer.mapped_ptr()? as *mut c_void;

        let mut buffer_align =
            Align::new(buffer_ptr, align_of::<T>() as u64, buffer_memory_req.size);
        buffer_align.copy_from_slice(data);
        Ok(())
    }

    pub fn raw_buffer(&self) -> Buffer {
        self.buffer.raw_buffer()
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}
