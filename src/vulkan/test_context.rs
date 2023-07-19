use std::any::type_name;
use std::any::Any;

use ash::vk::PresentModeKHR;
use glfw::ClientApiHint;
use glfw::WindowHint;

use crate::receivable_events::ReceivableEvents;
use crate::result::Context;
use crate::vulkan::creation_info::DataStructureCreationInfo;
use crate::vulkan::creation_info::PresentModeInitMode;
use crate::window::VoxelarWindow;
use crate::Voxelar;

use super::allocator::Allocator;
use super::debug::VerificationProvider;
use super::VulkanContext;

pub struct TestContext {
    ctx: Voxelar,
    window: VoxelarWindow,
    vulkan_context: VulkanContext,
}

impl TestContext {
    pub fn create<Alloc: Allocator + 'static, Verification: VerificationProvider + 'static>(
    ) -> crate::Result<Self> {
        let mut ctx = Voxelar::new()?;

        ctx.window_hint(WindowHint::Visible(true));
        ctx.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        let (mut window, _) = ctx.create_window(800, 600, "[Test]", glfw::WindowMode::Windowed)?;

        window.set_receivable_events(ReceivableEvents::all());

        let mut vulkan_context = ctx
            .load_render_context_for_window::<(Alloc, Verification), VulkanContext>(&mut window)?;

        let creation_info = DataStructureCreationInfo {
            swapchain_present_mode: PresentModeInitMode::Find(PresentModeKHR::FIFO),
            frame_overlap: 2,
        };
        vulkan_context.create_default_data_structures(window.get_size(), creation_info)?;

        Ok(Self {
            ctx,
            window,
            vulkan_context,
        })
    }

    pub fn vulkan_context(&self) -> &VulkanContext {
        &self.vulkan_context
    }

    pub fn try_get_allocator<T: Allocator>(&self) -> crate::Result<&T> {
        let allocator = self.vulkan_context.allocator.as_ref();
        let any_allocator: &dyn Any = allocator.as_any();
        any_allocator.downcast_ref::<T>().context(format!(
            "Failed to downcast allocator to {}",
            type_name::<T>()
        ))
    }
}
