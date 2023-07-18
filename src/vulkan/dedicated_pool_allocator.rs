use std::cell::UnsafeCell;

use ash::vk::{DeviceMemory, MemoryAllocateInfo, MemoryPropertyFlags, MemoryRequirements};

use crate::result::Context;

use super::allocator::{Allocation, Allocator};
use super::physical_device::SetUpPhysicalDevice;
use super::virtual_device::SetUpVirtualDevice;

const PRE_ALLOCATION_AMOUNT: u64 = 4_194_304;

#[derive(Debug)]
struct SubAllocation {
    offset: u64,
}

#[derive(Debug)]
struct MemoryPoolWithSubAllocations {
    memory: DeviceMemory,
    memory_size: u64,
    allocation_amount_so_far: u64,
    sub_allocations: Vec<SubAllocation>,
}

impl MemoryPoolWithSubAllocations {
    fn pre_allocate(
        virtual_device: &SetUpVirtualDevice,
        memory_type_index: u32,
    ) -> crate::Result<Self> {
        unsafe {
            let allocate_info = MemoryAllocateInfo {
                allocation_size: PRE_ALLOCATION_AMOUNT,
                memory_type_index,
                ..Default::default()
            };

            let memory = virtual_device
                .device
                .allocate_memory(&allocate_info, None)?;

            Ok(Self {
                memory,
                memory_size: allocate_info.allocation_size,
                allocation_amount_so_far: 0,
                sub_allocations: vec![],
            })
        }
    }

    fn can_fit_allocation(&self, memory_requirements: MemoryRequirements) -> bool {
        self.allocation_amount_so_far + memory_requirements.size <= self.memory_size
    }

    fn allocate(&mut self, memory_requirements: MemoryRequirements) -> Allocation {
        let allocation_offset = self.allocation_amount_so_far;
        let sub_allocation = SubAllocation {
            offset: allocation_offset,
        };

        self.allocation_amount_so_far += memory_requirements.size;
        self.sub_allocations.push(sub_allocation);

        Allocation {
            memory: self.memory,
            offset: allocation_offset,
        }
    }

    fn has_allocation(&self, allocation: Allocation) -> bool {
        allocation.memory == self.memory
            && self
                .sub_allocations
                .iter()
                .find(|sub_alloc| sub_alloc.offset == allocation.offset)
                .is_some()
    }

    fn deallocate(&mut self, allocation: Allocation) -> crate::Result<()> {
        crate::verify!(
            allocation.memory == self.memory,
            "Allocation {allocation:?} is not allocated in this memory pool"
        );

        for (i, sub_alloc) in self.sub_allocations.iter().enumerate() {
            if sub_alloc.offset == allocation.offset {
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
struct PoolsForMemoryType {
    memory_type_index: u32,
    memory_pools: UnsafeCell<Vec<MemoryPoolWithSubAllocations>>,
}

impl PoolsForMemoryType {
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
    all_pools: UnsafeCell<Vec<PoolsForMemoryType>>,
}

impl DedicatedPoolAllocator {
    fn get_all_pools(&self) -> &Vec<PoolsForMemoryType> {
        unsafe {
            let allocations = self.all_pools.get();
            &*allocations
        }
    }

    fn get_all_pools_mut(&self) -> &mut Vec<PoolsForMemoryType> {
        unsafe {
            let allocations = self.all_pools.get();
            &mut *allocations
        }
    }

    fn find_pools_for_memory_type_index(
        &self,
        memory_type_index: u32,
    ) -> crate::Result<&mut PoolsForMemoryType> {
        let pools = self.get_all_pools_mut();
        pools
            .iter_mut()
            .find(|pools| pools.memory_type_index == memory_type_index)
            .context(format!(
                "No allocation created for memory index {memory_type_index}"
            ))
    }

    fn find_pool_of_allocation(
        &self,
        allocation: Allocation,
    ) -> Option<&mut MemoryPoolWithSubAllocations> {
        for pools in self.get_all_pools().iter() {
            if let Some(pool) = pools.find_pool_of_allocation(allocation) {
                return Some(pool);
            }
        }

        None
    }
}

impl Allocator for DedicatedPoolAllocator {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            all_pools: UnsafeCell::new(vec![]),
        }
    }

    fn setup(
        &self,
        virtual_device: &SetUpVirtualDevice,
        physical_device: &SetUpPhysicalDevice,
    ) -> crate::Result<()> {
        let pools = self.get_all_pools_mut();
        let memory_properties = physical_device.device_memory_properties;
        for memory_type_index in 0..memory_properties.memory_type_count {
            pools.push(PoolsForMemoryType {
                memory_type_index,
                memory_pools: UnsafeCell::new(vec![MemoryPoolWithSubAllocations::pre_allocate(
                    virtual_device,
                    memory_type_index,
                )?]),
            });
        }
        Ok(())
    }

    fn allocate(
        &self,
        _: &SetUpVirtualDevice,
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

        let pools_for_memory_type =
            self.find_pools_for_memory_type_index(memory_type_index)?;

        let pool = pools_for_memory_type
            .find_pool_for_allocation(memory_requirements)
            .context(format!(
                "No allocation found to hold {memory_requirements:?}"
            ))?;

        let allocation = pool.allocate(memory_requirements);

        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("===== DedicatedAllocator - Allocation =====");
            println!("Memory requirements: {memory_requirements:?}");
            println!("Memory properties: {memory_properties:?}");
            println!("Memory type index: {memory_type_index}");
            println!("Found pools for type: {pools_for_memory_type:#?}");
            println!("Found pool for allocation: {pool:#?}");
            println!("Made allocation: {allocation:?}");
            println!("===========================================\n");
        }

        Ok(allocation)
    }

    fn deallocate(&self, _: &SetUpVirtualDevice, allocation: Allocation) {
        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("===== DedicatedAllocator - Deallocation =====");
        }
        
        if let Some(pool) = self.find_pool_of_allocation(allocation) {
            #[cfg(feature = "allocator-debug-logs")]
            {
                println!("Found pool that holds allocation: {pool:#?}");
            }
            let _ = pool.deallocate(allocation);
            #[cfg(feature = "allocator-debug-logs")]
            {
                println!("Made deallocation: {allocation:?}");
            }
        }
        
        #[cfg(feature = "allocator-debug-logs")]
        {
            println!("=============================================\n");
        }
    }

    fn destroy(&mut self, virtual_device: &SetUpVirtualDevice) {
        for pools in self.get_all_pools_mut() {
            pools.destroy(virtual_device);
        }
    }
}
