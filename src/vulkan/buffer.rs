//! This is a module that contains the `AllocatedBuffer` structure, an abstraction for GPU
//! memory-allocated buffers for various purposes

use std::ptr::NonNull;
use std::sync::MutexGuard;

use ash::vk::MemoryRequirements;
use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags};
use gpu_allocator::vulkan::{Allocation, AllocationCreateDesc, AllocationScheme, Allocator};
use gpu_allocator::MemoryLocation;

use crate::result::Context;

use super::logical_device::SetUpLogicalDevice;

/// An allocation-backed buffer
pub struct AllocatedBuffer {
    pub allocation: Option<Allocation>,
    pub buffer: Buffer,
}

impl AllocatedBuffer {
    /// This function allocates a new fixed-size buffer with the given arguments.
    ///
    /// The buffer is created using the `size`, `usage` and `sharing_mode` arguments, and the
    /// buffer data is allocated according to the `MemoryPropertyFlags` and buffer's memory
    /// requirements using the provided `Allocator`.
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        size: usize,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self> {
        let buffer_info = BufferCreateInfo::builder()
            .size(size as u64)
            .usage(usage)
            .sharing_mode(sharing_mode);

        let buffer = logical_device.create_buffer(&buffer_info, None)?;
        let memory_requirements = logical_device.get_buffer_memory_requirements(buffer);

        let buffer_allocation = allocator.allocate(&AllocationCreateDesc {
            name: "buffer",
            requirements: memory_requirements,
            location: memory_location,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;

        logical_device.bind_buffer_memory(
            buffer,
            buffer_allocation.memory(),
            buffer_allocation.offset(),
        )?;

        Ok(Self {
            allocation: Some(buffer_allocation),
            buffer,
        })
    }

    /// This function returns the memory requirements of the internal buffer.
    pub fn get_buffer_memory_req(&self, logical_device: &SetUpLogicalDevice) -> MemoryRequirements {
        unsafe { logical_device.get_buffer_memory_requirements(self.buffer) }
    }

    /// Returns a reference to this buffer's allocation.
    pub fn allocation(&self) -> crate::Result<&Allocation> {
        self.allocation
            .as_ref()
            .context("Buffer has no backing memory allocation".to_string())
    }

    /// Returns the pointer to this buffer's mapped memory if the allocation type supports it.
    pub fn get_mapped_ptr(&self) -> crate::Result<NonNull<std::ffi::c_void>> {
        self.allocation()?
            .mapped_ptr()
            .context("Couldn't get buffer's mapped memory pointer".to_string())
    }

    /// This function destroys this buffer and deallocates its memory using the provided `Allocator`.
    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        unsafe {
            if let Some(buffer_allocation) = self.allocation.take() {
                allocator.free(buffer_allocation)?;
            }
            logical_device.destroy_buffer(self.buffer, None);
            Ok(())
        }
    }
}
