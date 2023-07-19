use std::cell::UnsafeCell;

use ash::vk::{DeviceMemory, MemoryAllocateInfo, MemoryPropertyFlags, MemoryRequirements};

use crate::result::Context;

use super::allocator::{Allocation, Allocator};
use super::memory_range::*;
use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

const PRE_ALLOCATION_BASE_AMOUNT: u64 = 4_194_304;

#[derive(Debug)]
struct SubAllocation {
    offset: u64,
}

#[derive(Debug)]
struct MemoryPoolWithSubAllocations {
    memory: DeviceMemory,
    free_memory_ranges: FreeMemoryRanges,
    sub_allocations: Vec<SubAllocation>,
}

impl MemoryPoolWithSubAllocations {
    fn pre_allocate(
        virtual_device: &SetUpVirtualDevice,
        memory_type_index: u32,
        allocation_size: u64,
    ) -> crate::Result<Self> {
        unsafe {
            let allocate_info = MemoryAllocateInfo {
                allocation_size,
                memory_type_index,
                ..Default::default()
            };

            let memory = virtual_device
                .device
                .allocate_memory(&allocate_info, None)?;

            Ok(Self {
                memory,
                free_memory_ranges: FreeMemoryRanges::fully_free(0, allocate_info.allocation_size)?,
                sub_allocations: vec![],
            })
        }
    }

    fn can_fit_allocation(&self, memory_requirements: MemoryRequirements) -> bool {
        self.free_memory_ranges
            .find_range_that_can_fit_width(memory_requirements.size)
            .is_some()
    }

    fn allocate(&mut self, memory_requirements: MemoryRequirements) -> crate::Result<Allocation> {
        let range = self
            .free_memory_ranges
            .find_range_that_can_fit_width(memory_requirements.size)
            .context(format!(
                "Memory pool can't fit requested memory: {memory_requirements:?}"
            ))?;

        self.free_memory_ranges
            .unfree_range(range.start(), range.start() + memory_requirements.size - 1)?;

        self.sub_allocations.push(SubAllocation {
            offset: range.start(),
        });

        Ok(Allocation {
            memory: self.memory,
            offset: range.start(),
            size: memory_requirements.size,
        })
    }

    fn has_allocation(&self, allocation: Allocation) -> bool {
        allocation.memory == self.memory
    }

    fn deallocate(&mut self, allocation: Allocation) -> crate::Result<()> {
        crate::verify!(
            allocation.memory == self.memory,
            "Allocation {allocation:?} is not allocated in this memory pool"
        );

        for (i, sub_alloc) in self.sub_allocations.iter().enumerate() {
            if sub_alloc.offset == allocation.offset {
                self.free_memory_ranges
                    .free_range(sub_alloc.offset, sub_alloc.offset + allocation.size - 1)?;
                self.sub_allocations.remove(i);
            }

            return Ok(());
        }

        crate::bail!("Allocation {allocation:?} is not allocated in this memory pool")
    }

    fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        unsafe {
            virtual_device.device.free_memory(self.memory, None);
        }
    }
}

#[derive(Debug)]
struct PoolBundleForType {
    memory_type_index: u32,
    memory_pools: UnsafeCell<Vec<MemoryPoolWithSubAllocations>>,
}

impl PoolBundleForType {
    fn get_memory_pools_mut(&self) -> &mut Vec<MemoryPoolWithSubAllocations> {
        unsafe {
            let allocations = self.memory_pools.get();
            &mut *allocations
        }
    }

    fn find_pool_for_allocation(
        &self,
        memory_requirements: MemoryRequirements,
    ) -> Option<&mut MemoryPoolWithSubAllocations> {
        self.get_memory_pools_mut()
            .iter_mut()
            .find(|pool| pool.can_fit_allocation(memory_requirements))
    }

    fn find_pool_of_allocation(
        &self,
        allocation: Allocation,
    ) -> Option<&mut MemoryPoolWithSubAllocations> {
        self.get_memory_pools_mut()
            .iter_mut()
            .find(|pool| pool.has_allocation(allocation))
    }

    fn create_pool_for_allocation(
        &self,
        virtual_device: &SetUpVirtualDevice,
        memory_requirements: MemoryRequirements,
    ) -> crate::Result<&mut MemoryPoolWithSubAllocations> {
        let memory_pools = self.get_memory_pools_mut();

        // Round the allocation amount up to the next power of two
        // If it is smaller than `PRE_ALLOCATION_BASE_AMOUNT`, use `PRE_ALLOCATION_BASE_AMOUNT`
        // instead to avoid small allocations.
        let allocation_size = memory_requirements
            .size
            .next_power_of_two()
            .max(PRE_ALLOCATION_BASE_AMOUNT);

        let new_pool = MemoryPoolWithSubAllocations::pre_allocate(
            virtual_device,
            self.memory_type_index,
            allocation_size,
        )?;
        memory_pools.push(new_pool);
        Ok(memory_pools.last_mut().unwrap())
    }

    fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        for pool in self.get_memory_pools_mut() {
            pool.destroy(virtual_device);
        }
    }
}

pub struct DedicatedPoolAllocator {
    // This UnsafeCell is very cursed, but it allows for more flexibility.
    //
    // I tried to use a mutable reference to the allocator in every function call, but ultimately,
    // this caused issues in destroy functions when I needed to pass a reference to a virtual
    // device and also a mutable reference to the allocator to the function (Which is not
    // allowed!).
    pool_bundles: UnsafeCell<Vec<PoolBundleForType>>,
}

impl DedicatedPoolAllocator {
    fn get_pool_bundles(&self) -> &Vec<PoolBundleForType> {
        unsafe {
            let allocations = self.pool_bundles.get();
            &*allocations
        }
    }

    fn get_pool_bundles_mut(&self) -> &mut Vec<PoolBundleForType> {
        unsafe {
            let allocations = self.pool_bundles.get();
            &mut *allocations
        }
    }

    fn find_pools_for_memory_type_index(
        &self,
        memory_type_index: u32,
    ) -> crate::Result<&mut PoolBundleForType> {
        let pool_bundles = self.get_pool_bundles_mut();
        pool_bundles
            .iter_mut()
            .find(|pool_bundle| pool_bundle.memory_type_index == memory_type_index)
            .context(format!(
                "No pool for memory index {memory_type_index} was created yet"
            ))
    }

    fn find_pool_of_allocation(
        &self,
        allocation: Allocation,
    ) -> Option<&mut MemoryPoolWithSubAllocations> {
        for pool_bundle in self.get_pool_bundles().iter() {
            if let Some(pool) = pool_bundle.find_pool_of_allocation(allocation) {
                return Some(pool);
            }
        }

        None
    }

    #[cfg(test)]
    unsafe fn reset_pools(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
    ) -> crate::Result<()> {
        for pool_bundle in self.get_pool_bundles_mut() {
            pool_bundle.destroy(virtual_device);
        }
        self.get_pool_bundles_mut().clear();
        self.setup(virtual_device, physical_device)?;
        Ok(())
    }
}

impl Allocator for DedicatedPoolAllocator {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            pool_bundles: UnsafeCell::new(vec![]),
        }
    }

    fn setup(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
    ) -> crate::Result<()> {
        let pools = self.get_pool_bundles_mut();
        let memory_properties = physical_device.device_memory_properties;
        for memory_type_index in 0..memory_properties.memory_type_count {
            pools.push(PoolBundleForType {
                memory_type_index,
                memory_pools: UnsafeCell::new(vec![MemoryPoolWithSubAllocations::pre_allocate(
                    virtual_device,
                    memory_type_index,
                    PRE_ALLOCATION_BASE_AMOUNT,
                )?]),
            });
        }
        Ok(())
    }

    fn allocate(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
        memory_requirements: MemoryRequirements,
        memory_properties: MemoryPropertyFlags,
    ) -> crate::Result<Allocation> {
        let memory_type_index = physical_device
            .find_memory_type_index(&memory_requirements, memory_properties)
            .context(format!(
                "No memory type index found for allocation: {:?}, {:?}",
                memory_properties, memory_requirements
            ))?;

        let pool_bundle_for_memory_type =
            self.find_pools_for_memory_type_index(memory_type_index)?;

        #[cfg(feature = "allocator-debug-logs")]
        let mut pool_reallocated = false;

        let pool = match pool_bundle_for_memory_type.find_pool_for_allocation(memory_requirements) {
            Some(pool) => pool,
            None => {
                #[cfg(feature = "allocator-debug-logs")]
                {
                    pool_reallocated = true;
                }

                pool_bundle_for_memory_type
                    .create_pool_for_allocation(virtual_device, memory_requirements)?
            }
        };

        let allocation = pool.allocate(memory_requirements)?;

        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("================= DedicatedPoolAllocator - Allocation =================");
            println!("Memory requirements: {memory_requirements:?}");
            println!("Memory properties: {memory_properties:?}");
            println!("Memory type index: {memory_type_index}");
            println!(
                "Found pool for allocation with memory handle: {:?}",
                pool.memory
            );
            println!("Pool reallocated: {pool_reallocated}");
            println!("Made allocation: {allocation:?}");
            println!("=======================================================================");
        }

        Ok(allocation)
    }

    fn deallocate(&self, _: &SetUpVirtualDevice, allocation: Allocation) {
        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("================ DedicatedPoolAllocator - Deallocation ================");
        }

        if let Some(pool) = self.find_pool_of_allocation(allocation) {
            #[cfg(feature = "allocator-debug-logs")]
            {
                println!(
                    "Found pool that holds allocation for memory handle: {:?}",
                    pool.memory
                );
            }
            let _ = pool.deallocate(allocation);
            #[cfg(feature = "allocator-debug-logs")]
            {
                println!("Made deallocation: {allocation:?}");
            }
        }

        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("=======================================================================\n");
        }
    }

    fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("================ DedicatedPoolAllocator - Destruction ================");
        }
        for pool_bundle in self.get_pool_bundles_mut() {
            #[cfg(feature = "allocator-debug-logs")]
            {
                println!(
                    "Destroying pool bundle ({} memory pools): {pool_bundle:?}",
                    pool_bundle.get_memory_pools_mut().len()
                );
            }
            pool_bundle.destroy(virtual_device);
        }
        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("=======================================================================\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vulkan::debug::NoVerification;
    use crate::vulkan::test_context::*;
    use rstest::*;

    #[fixture]
    #[once]
    fn test_context_fixture() -> TestContext {
        TestContext::create::<DedicatedPoolAllocator, NoVerification>()
            .expect("Unable to create test context")
    }

    #[rstest]
    fn pool_of_just_created_test_context_has_correct_amount_of_pools(
        test_context_fixture: &TestContext,
    ) {
        let vulkan_context = test_context_fixture.vulkan_context();
        let virtual_device = vulkan_context
            .virtual_device()
            .expect("No virtual device created");
        let physical_device = vulkan_context
            .physical_device()
            .expect("No physical device created");

        let memory_type_count = physical_device.device_memory_properties.memory_type_count;
        let pool_allocator = test_context_fixture
            .try_get_allocator::<DedicatedPoolAllocator>()
            .unwrap();

        unsafe {
            pool_allocator
                .reset_pools(virtual_device, physical_device)
                .unwrap();
        }
        assert_eq!(
            memory_type_count as usize,
            pool_allocator.get_pool_bundles().len()
        );
    }
}
