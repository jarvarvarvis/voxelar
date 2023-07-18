use std::marker::PhantomData;
use std::mem::align_of;

use ash::util::Align;
use ash::vk::MemoryPropertyFlags;
use ash::vk::SharingMode;
use ash::vk::{Buffer, BufferUsageFlags};

use super::allocator::Allocator;
use super::buffer::AllocatedBuffer;
use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

pub struct TypedAllocatedBuffer<T> {
    pub buffer: AllocatedBuffer,
    phantom: PhantomData<T>,
}

impl<T> TypedAllocatedBuffer<T> {
    pub unsafe fn allocate(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &dyn Allocator,
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_property_flags: MemoryPropertyFlags,
    ) -> crate::Result<Self> {
        let size = std::mem::size_of::<T>() as u64;
        Ok(Self {
            buffer: AllocatedBuffer::allocate(
                virtual_device,
                physical_device,
                allocator,
                size,
                usage,
                sharing_mode,
                memory_property_flags,
            )?,
            phantom: PhantomData,
        })
    }

    pub unsafe fn allocate_uniform_buffer(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &dyn Allocator,
    ) -> crate::Result<Self> {
        Self::allocate(
            virtual_device,
            physical_device,
            allocator,
            BufferUsageFlags::UNIFORM_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )
    }

    pub unsafe fn create_from_data_slice(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &dyn Allocator,
        data: &[T],
        usage: BufferUsageFlags,
        sharing_mode: SharingMode,
        memory_property_flags: MemoryPropertyFlags,
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        let size = (std::mem::size_of::<T>() * data.len()) as u64;

        let buffer = AllocatedBuffer::allocate(
            virtual_device,
            physical_device,
            allocator,
            size,
            usage,
            sharing_mode,
            memory_property_flags,
        )?;
        let buffer_memory_req = buffer.get_buffer_memory_req(virtual_device);

        let buffer_ptr = buffer.map_memory(virtual_device)?;

        let mut buffer_align =
            Align::new(buffer_ptr, align_of::<T>() as u64, buffer_memory_req.size);
        buffer_align.copy_from_slice(data);

        buffer.unmap_memory(virtual_device);

        Ok(Self {
            buffer,
            phantom: PhantomData,
        })
    }

    pub unsafe fn create_vertex_buffer(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &dyn Allocator,
        data: &[T],
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        Self::create_from_data_slice(
            virtual_device,
            physical_device,
            allocator,
            data,
            BufferUsageFlags::VERTEX_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )
    }

    pub unsafe fn create_index_buffer(
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        allocator: &dyn Allocator,
        data: &[T],
    ) -> crate::Result<Self>
    where
        T: Copy,
    {
        Self::create_from_data_slice(
            virtual_device,
            physical_device,
            allocator,
            data,
            BufferUsageFlags::INDEX_BUFFER,
            SharingMode::EXCLUSIVE,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )
    }

    pub unsafe fn map_memory(&self, virtual_device: &SetUpVirtualDevice) -> crate::Result<*mut T> {
        self.buffer
            .map_memory(virtual_device)
            .map(|value| value as *mut T)
    }

    pub unsafe fn unmap_memory(&self, virtual_device: &SetUpVirtualDevice) {
        self.buffer.unmap_memory(virtual_device);
    }

    pub unsafe fn store(&self, virtual_device: &SetUpVirtualDevice, value: T) -> crate::Result<()> {
        let ptr = self.map_memory(virtual_device)?;
        *ptr = value;
        self.unmap_memory(virtual_device);
        Ok(())
    }

    pub fn raw_buffer(&self) -> Buffer {
        self.buffer.buffer
    }

    pub fn destroy(&mut self, virtual_device: &SetUpVirtualDevice, allocator: &dyn Allocator) {
        self.buffer.destroy(virtual_device, allocator);
    }
}
