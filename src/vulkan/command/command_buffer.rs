//! This is a module that contains the `SetUpCommandBufferWithFence` structure, an abstraction for
//! command buffers with access synchronization using a `Fence`.

use ash::vk::PipelineStageFlags;
use ash::vk::Queue;
use ash::vk::SubmitInfo;
use ash::vk::{
    CommandBuffer, CommandBufferBeginInfo, CommandBufferResetFlags, CommandBufferUsageFlags,
};
use ash::vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore};

use crate::vulkan::logical_device::SetUpLogicalDevice;

/// A set up command buffer with a fence to synchronize usage.
pub struct SetUpCommandBufferWithFence {
    pub command_buffer: CommandBuffer,
    pub reuse_fence: Fence,
}

impl SetUpCommandBufferWithFence {
    /// This function creates a new `SetUpCommandBufferWithFence` from a command buffer allocated from a `(SetUp)CommandPool`,
    /// as well as the reuse fence.
    pub unsafe fn create(
        logical_device: &SetUpLogicalDevice,
        command_buffer: CommandBuffer,
        fence_create_flags: FenceCreateFlags,
    ) -> crate::Result<Self> {
        let reuse_fence_create_info = FenceCreateInfo::builder().flags(fence_create_flags);
        let reuse_fence = logical_device.create_fence(&reuse_fence_create_info, None)?;
        Ok(SetUpCommandBufferWithFence {
            command_buffer,
            reuse_fence,
        })
    }

    /// This function waits for this `SetUpCommandBufferWithFence`'s reuse fence
    pub fn wait_for_fence(&self, logical_device: &SetUpLogicalDevice) -> crate::Result<()> {
        unsafe {
            logical_device.wait_for_fences(&[self.reuse_fence], true, std::u64::MAX)?;
            logical_device.reset_fences(&[self.reuse_fence])?;
            Ok(())
        }
    }

    /// This function resets this `SetUpCommandBufferWithFence`'s reuse fence
    pub fn reset_fence(&self, logical_device: &SetUpLogicalDevice) -> crate::Result<()> {
        unsafe {
            logical_device.reset_fences(&[self.reuse_fence])?;
            Ok(())
        }
    }

    /// This function resets this `SetUpCommandBufferWithFence`, clearing all recorded commands.
    pub fn reset(
        &self,
        logical_device: &SetUpLogicalDevice,
        flags: CommandBufferResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            logical_device.reset_command_buffer(self.command_buffer, flags)?;
            Ok(())
        }
    }

    /// This function records commands into the command buffer using the provided usage flags.
    ///
    /// The commands are recorded from a lambda expression that may fail using the `Result` type.
    /// If an error occurs, it is propagated upwards.
    pub fn record_commands<F: FnOnce(&SetUpLogicalDevice, &Self) -> crate::Result<()>>(
        &self,
        logical_device: &SetUpLogicalDevice,
        f: F,
        usage: CommandBufferUsageFlags,
    ) -> crate::Result<()> {
        unsafe {
            let command_buffer_begin_info = CommandBufferBeginInfo::builder().flags(usage);

            logical_device.begin_command_buffer(self.command_buffer, &command_buffer_begin_info)?;
            f(&logical_device, &self)?;
            logical_device.end_command_buffer(self.command_buffer)?;

            Ok(())
        }
    }

    /// This function records commands into the command buffer with the `SIMULTANEOUS_USE` usage
    /// flag. This means, that the commands in the buffer can be submitted multiple times and
    /// from multiple threads. If it's desirable to record commands for immediate submitting, use
    /// `CommandBufferWithReuseFence::record_commands_for_one_time_submit`
    ///
    /// The commands are recorded from a lambda expression that may fail using the `Result` type.
    /// If an error occurs, it is propagated upwards.
    pub fn record_commands_for_simultaneous_use<
        F: FnOnce(&SetUpLogicalDevice, &Self) -> crate::Result<()>,
    >(
        &self,
        logical_device: &SetUpLogicalDevice,
        f: F,
    ) -> crate::Result<()> {
        self.record_commands(logical_device, f, CommandBufferUsageFlags::SIMULTANEOUS_USE)
    }

    /// This function records commands into the command buffer with the `ONE_TIME_SUBMIT` usage
    /// flag. This means, that the commands in the buffer can only submitted once before the
    /// buffer is cleared. If it's desirable to record commands and submit them later multiple
    /// times, use `CommandBufferWithReuseFence::record_commands_for_simultaneous_use`.
    ///
    /// The commands are recorded from a lambda expression that may fail using the `Result` type.
    /// If an error occurs, it is propagated upwards.
    pub fn record_commands_for_one_time_submit<
        F: FnOnce(&SetUpLogicalDevice, &Self) -> crate::Result<()>,
    >(
        &self,
        logical_device: &SetUpLogicalDevice,
        f: F,
    ) -> crate::Result<()> {
        self.record_commands(logical_device, f, CommandBufferUsageFlags::ONE_TIME_SUBMIT)
    }

    /// This function submits this command buffer to a given `Queue` using the provided wait mask,
    /// wait `Semaphore`s and signal `Semaphore`s.
    ///
    /// Please see the documentation for Vulkan's `VkSubmitInfo` for more details on these
    /// arguments because I don't know how exactly this works either.
    pub fn submit(
        &self,
        logical_device: &SetUpLogicalDevice,
        submit_queue: Queue,
        wait_mask: &[PipelineStageFlags],
        wait_semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
    ) -> crate::Result<()> {
        unsafe {
            let submit_info = SubmitInfo::builder()
                .wait_semaphores(wait_semaphores)
                .wait_dst_stage_mask(wait_mask)
                .command_buffers(&[self.command_buffer])
                .signal_semaphores(signal_semaphores)
                .build();

            logical_device.queue_submit(submit_queue, &[submit_info], self.reuse_fence)?;

            Ok(())
        }
    }

    /// This function destroys this buffer's reuse fence.
    pub fn destroy(&mut self, logical_device: &SetUpLogicalDevice) {
        unsafe {
            logical_device.destroy_fence(self.reuse_fence, None);
        }
    }
}
