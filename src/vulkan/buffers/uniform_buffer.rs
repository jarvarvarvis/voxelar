use std::sync::MutexGuard;

use ash::vk::BufferUsageFlags;
use ash::vk::SharingMode;
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use crate::vulkan::logical_device::SetUpLogicalDevice;
use crate::vulkan::physical_device::SetUpPhysicalDevice;

use super::aligned_buffer::AlignedBuffer;

pub struct SetUpUniformBuffer<T> {
    pub buffer: AlignedBuffer<T>,
}

impl<T> SetUpUniformBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_count: usize,
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
                BufferUsageFlags::UNIFORM_BUFFER,
                SharingMode::EXCLUSIVE,
                MemoryLocation::CpuToGpu,
            )?,
        })
    }

    pub unsafe fn allocate_static_uniform_buffer(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<Self> {
        Self::allocate(logical_device, physical_device, allocator, 1)
    }

    pub unsafe fn allocate_dynamic_uniform_buffer(
        logical_device: &SetUpLogicalDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        count: usize,
    ) -> crate::Result<Self> {
        Self::allocate(logical_device, physical_device, allocator, count)
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}

impl<T> std::ops::Deref for SetUpUniformBuffer<T> {
    type Target = AlignedBuffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
