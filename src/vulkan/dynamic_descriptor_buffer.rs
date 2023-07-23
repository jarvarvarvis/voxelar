use std::sync::MutexGuard;

use ash::vk::BufferUsageFlags;
use ash::vk::SharingMode;
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use super::aligned_buffer::AlignedBuffer;
use super::logical_device::SetUpLogicalDevice;
use super::physical_device::SetUpPhysicalDevice;

pub struct DynamicDescriptorBuffer<T> {
    pub buffer: AlignedBuffer<T>,
}

impl<T> DynamicDescriptorBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_count: usize,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self> {
        let alignment = physical_device
            .device_properties
            .limits
            .min_uniform_buffer_offset_alignment;
        Ok(Self {
            buffer: AlignedBuffer::allocate(
                logical_device,
                allocator,
                element_count,
                alignment as usize,
                usage,
                sharing_mode,
                memory_location,
            )?,
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

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}

impl<T> std::ops::Deref for DynamicDescriptorBuffer<T> {
    type Target = AlignedBuffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
