use std::sync::MutexGuard;

use ash::vk::BufferUsageFlags;
use ash::vk::SharingMode;
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use super::aligned_buffer::AlignedBuffer;
use super::logical_device::SetUpLogicalDevice;
use super::physical_device::SetUpPhysicalDevice;

pub struct SetUpStorageBuffer<T> {
    pub buffer: AlignedBuffer<T>,
}

impl<T> SetUpStorageBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_count: usize,
    ) -> crate::Result<Self> {
        let alignment = physical_device
            .device_properties
            .limits
            .min_storage_buffer_offset_alignment;
        Ok(Self {
            buffer: AlignedBuffer::allocate(
                logical_device,
                allocator,
                element_count,
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
