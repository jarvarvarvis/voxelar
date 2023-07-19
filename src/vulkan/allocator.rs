//! This is a module that contains generic stuff for GPU memory allocators.
//!
//! Currently, two implementations of `Allocator` exist:
//!
//! `NaiveAllocator` (in naive_allocator.rs):
//! A simple allocator that just allocates the requested amount of memory on the device.
//!
//! `DedicatedPoolAllocator` (dedicated_pool_allocator.rs):
//! A more complex allocator that has memory pools for the different memory indices. This allocator
//! tracks free memory ranges. When an allocation is made, it searches for the next free range in the
//! pool for the requested memory type. When no pool is found that can hold the allocation, a new pool
//! with a size of the next power of two (subject to change) of the requested amount is allocated.
//! See the implementation of `DedicatedPoolAllocator` for more details.

use ash::vk::{DeviceMemory, MemoryPropertyFlags, MemoryRequirements};

use crate::as_any_trait::AsAny;

use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

/// A generic allocation.
///
/// It contains a handle to the `DeviceMemory` where the allocation is located, as well as an offset
/// from the start of the `DeviceMemory` and the allocation's size.
#[derive(Debug, Clone, Copy)]
pub struct Allocation {
    pub memory: DeviceMemory,
    pub offset: u64,
    pub size: u64,
}

/// The `Allocator` trait.
///
/// Allocator implementations need to provide the following five functions:
/// - new: The `Allocator` instance is created and stored when a `VulkanContext` is loaded.
/// - setup: After the creation of the virtual device and physical device in the `VulkanContext`,
///          this function is called. It can be used to  prepare the allocator for allocations,
///          e.g. by making pre-allocations.
/// - allocate: Make a new allocation in device memory. The requested memory should have the
///             requirements and properties given to this function. It returns a new `Allocation`
///             instance.
/// - deallocate: Make a deallocation of the given `Allocation` from device memory.
/// - destroy: After the `VulkanContext` is destroyed, this function is called. It should be used
///            to clean up any remaining internal memory allocations.
pub trait Allocator: AsAny {
    /// This function is used to create a new allocator instance.
    fn new() -> Self
    where
        Self: Sized;

    /// This function is used to set up the allocator before any allocations can be made.
    ///
    /// This step might fail, so a `Result` is returned.
    fn setup(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
    ) -> crate::Result<()>;

    /// This function is used to make a new allocation in device memory given some
    /// `MemoryRequirements` and `MemoryPropertyFlags`.
    ///
    /// This step might fail, so a `Result` is returned.
    fn allocate(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        memory_requirements: MemoryRequirements,
        memory_properties: MemoryPropertyFlags,
    ) -> crate::Result<Allocation>;

    /// This function is used to deallocate memory.
    ///
    /// The Vulkan functions related to memory deallocation don't return `Result`, so this function
    /// does neither.
    fn deallocate(&self, virtual_device: &SetUpVirtualDevice, allocation: Allocation);

    /// This function is used to destroy the allocator instance.
    fn destroy(&mut self, virtual_device: &SetUpVirtualDevice);
}
