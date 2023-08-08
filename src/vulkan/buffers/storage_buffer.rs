use std::sync::MutexGuard;

use ash::vk::BufferUsageFlags;
use ash::vk::SharingMode;
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use crate::vulkan::logical_device::SetUpLogicalDevice;

use super::aligned_buffer::AlignedBuffer;

pub struct SetUpStorageBuffer<T> {
    pub buffer: AlignedBuffer<T>,
}

impl<T> SetUpStorageBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_amount: usize,
    ) -> crate::Result<Self> {
        let alignment = std::mem::align_of::<T>();
        Ok(Self {
            buffer: AlignedBuffer::allocate(
                logical_device,
                allocator,
                element_amount,
                alignment as usize,
                BufferUsageFlags::STORAGE_BUFFER,
                SharingMode::EXCLUSIVE,
                MemoryLocation::CpuToGpu,
            )?,
        })
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}

impl<T> std::ops::Deref for SetUpStorageBuffer<T> {
    type Target = AlignedBuffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
