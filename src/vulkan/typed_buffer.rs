use std::marker::PhantomData;
use std::mem::align_of;
use std::sync::MutexGuard;

use ash::util::Align;
use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferUsageFlags};
use gpu_allocator::vulkan::Allocator;
use gpu_allocator::MemoryLocation;

use super::buffer::AllocatedBuffer;
use super::virtual_device::SetUpVirtualDevice;

pub struct TypedAllocatedBuffer<T> {
    pub buffer: AllocatedBuffer,
    phantom: PhantomData<T>,
}

impl<T> TypedAllocatedBuffer<T> {
    pub unsafe fn allocate(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self> {
        let size = std::mem::size_of::<T>() as u64;
        Ok(Self {
            buffer: AllocatedBuffer::allocate(
                virtual_device,
                allocator,
                size,
                usage,
                sharing_mode,
                memory_location,
            )?,
            phantom: PhantomData,
        })
    }

    pub unsafe fn allocate_uniform_buffer(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<Self> {
        Self::allocate(
            virtual_device,
            allocator,
            BufferUsageFlags::UNIFORM_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryLocation::CpuToGpu,
        )
    }

    pub unsafe fn create_from_data_slice(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
        data: &[T],
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_location: MemoryLocation,
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        let size = (std::mem::size_of::<T>() * data.len()) as u64;

        let buffer = AllocatedBuffer::allocate(
            virtual_device,
            allocator,
            size,
            usage,
            sharing_mode,
            memory_location,
        )?;
        let buffer_memory_req = buffer.get_buffer_memory_req(virtual_device);

        let buffer_ptr = buffer.get_mapped_ptr()?.as_ptr();

        let mut buffer_align =
            Align::new(buffer_ptr, align_of::<T>() as u64, buffer_memory_req.size);
        buffer_align.copy_from_slice(data);

        Ok(Self {
            buffer,
            phantom: PhantomData,
        })
    }

    pub unsafe fn create_vertex_buffer(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
        data: &[T],
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        Self::create_from_data_slice(
            virtual_device,
            allocator,
            data,
            BufferUsageFlags::VERTEX_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryLocation::CpuToGpu,
        )
    }

    pub unsafe fn create_index_buffer(
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
        data: &[T],
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        Self::create_from_data_slice(
            virtual_device,
            allocator,
            data,
            BufferUsageFlags::INDEX_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryLocation::CpuToGpu,
        )
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

    pub fn destroy(
        &mut self,
        virtual_device: &SetUpVirtualDevice,
        allocator: &mut MutexGuard<Allocator>,
    ) -> crate::Result<()> {
        self.buffer.destroy(virtual_device, allocator)
    }
}
