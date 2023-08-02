use std::marker::PhantomData;
use std::sync::MutexGuard;

use ash::vk::{Buffer, BufferUsageFlags};
use ash::vk::{BufferCopy, SharingMode};
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use crate::vulkan::command_buffer::SetUpCommandBufferWithFence;
use crate::vulkan::logical_device::SetUpLogicalDevice;

use super::buffer::AllocatedBuffer;
use super::staging_buffer::SetUpStagingBuffer;

pub struct TypedAllocatedBuffer<T> {
    pub buffer: AllocatedBuffer,
    pub element_amount: usize,
    phantom: PhantomData<T>,
}

impl<T> TypedAllocatedBuffer<T> {
    pub unsafe fn allocate(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_amount: usize,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self> {
        let size = std::mem::size_of::<T>() * element_amount;
        Ok(Self {
            buffer: AllocatedBuffer::allocate(
                logical_device,
                allocator,
                size,
                usage,
                sharing_mode,
                memory_location,
            )?,
            phantom: PhantomData,
            element_amount,
        })
    }

    pub unsafe fn allocate_uniform_buffer(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<Self> {
        Self::allocate(
            logical_device,
            allocator,
            1,
            BufferUsageFlags::UNIFORM_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryLocation::CpuToGpu,
        )
    }

    pub unsafe fn allocate_vertex_buffer(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_amount: usize,
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        let buffer = Self::allocate(
            logical_device,
            allocator,
            element_amount,
            BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            SharingMode::EXCLUSIVE,
            MemoryLocation::GpuOnly,
        )?;
        Ok(buffer)
    }

    pub unsafe fn allocate_index_buffer(
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
        element_amount: usize,
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        let buffer = Self::allocate(
            logical_device,
            allocator,
            element_amount,
            BufferUsageFlags::INDEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
            SharingMode::EXCLUSIVE,
            MemoryLocation::GpuOnly,
        )?;
        Ok(buffer)
    }

    pub fn copy_from_staging_buffer(
        &self,
        logical_device: &SetUpLogicalDevice,
        staging_buffer: &SetUpStagingBuffer<T>,
        setup_command_buffer: &SetUpCommandBufferWithFence,
    ) -> crate::Result<()> {
        crate::verify!(
            self.element_amount == staging_buffer.buffer.element_amount,
            "The staging buffer must have the same size as the source buffer"
        );
        unsafe {
            let buffer_copy = BufferCopy::builder()
                .dst_offset(0)
                .src_offset(0)
                .size(self.data_amount() as u64);

            logical_device.cmd_copy_buffer(
                setup_command_buffer.command_buffer,
                staging_buffer.raw_buffer(),
                self.raw_buffer(),
                &[*buffer_copy],
            );
        }
        Ok(())
    }

    pub unsafe fn mapped_ptr(&self) -> crate::Result<*mut T> {
        Ok(self.buffer.get_mapped_ptr()?.as_ptr() as *mut T)
    }

    pub unsafe fn store(&self, value: T) -> crate::Result<()> {
        let ptr = self.mapped_ptr()?;
        *ptr = value;
        Ok(())
    }

    pub fn raw_buffer(&self) -> Buffer {
        self.buffer.buffer
    }

    pub fn data_amount(&self) -> usize {
        std::mem::size_of::<T>() * self.element_amount
    }

    pub fn destroy(
        &mut self,
        logical_device: &SetUpLogicalDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(logical_device, allocator)
    }
}

impl<T> std::ops::Deref for TypedAllocatedBuffer<T> {
    type Target = AllocatedBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
