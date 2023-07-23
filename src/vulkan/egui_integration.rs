use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex};

use ash::vk::{Queue, SurfaceFormatKHR};
use gpu_allocator::vulkan::Allocator;

use egui_winit::EventResponse;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;

use crate::window::VoxelarWindow;

use super::command_buffer::SetUpCommandBufferWithFence;
use super::logical_device::SetUpLogicalDevice;
use super::surface::SetUpSurfaceInfo;
use super::swapchain::SetUpSwapchain;

pub struct SetUpEguiIntegration {
    pub integration: ManuallyDrop<egui_winit_ash_integration::Integration<Arc<Mutex<Allocator>>>>,
}

impl SetUpEguiIntegration {
    pub fn new(
        event_loop: &EventLoop<()>,
        window_width: u32,
        window_height: u32,
        window_scale_factor: f64,
        logical_device: &SetUpLogicalDevice,
        allocator: Arc<Mutex<Allocator>>,
        graphics_queue_index: u32,
        present_queue: Queue,
        swapchain: &SetUpSwapchain,
        surface_format: SurfaceFormatKHR,
    ) -> Self {
        let integration = ManuallyDrop::new(egui_winit_ash_integration::Integration::new(
            event_loop,
            window_width,
            window_height,
            window_scale_factor,
            egui::FontDefinitions::default(),
            egui::Style::default(),
            logical_device.device.clone(),
            allocator,
            graphics_queue_index,
            present_queue,
            swapchain.swapchain_loader.clone(),
            swapchain.swapchain,
            surface_format,
        ));
        Self { integration }
    }

    pub fn draw<F>(
        &mut self,
        window: &VoxelarWindow,
        draw_command_buffer: &SetUpCommandBufferWithFence,
        present_index: u32,
        draw_op: F,
    ) -> crate::Result<()>
    where
        F: Fn(&Self) -> crate::Result<()>,
    {
        self.integration.begin_frame(&window.window);
        draw_op(&self)?;
        let output = self.integration.end_frame(&window.window);
        let clipped_meshes = self.integration.context().tessellate(output.shapes);
        self.integration.paint(
            draw_command_buffer.command_buffer,
            present_index as usize,
            clipped_meshes,
            output.textures_delta,
        );
        Ok(())
    }

    pub fn handle_event(&mut self, winit_event: &WindowEvent<'_>) -> EventResponse {
        self.integration.handle_event(winit_event)
    }

    pub fn update_swapchain(
        &mut self,
        physical_width: u32,
        physical_height: u32,
        swapchain: &SetUpSwapchain,
        surface_info: &SetUpSurfaceInfo,
    ) -> crate::Result<()> {
        self.integration.update_swapchain(
            physical_width,
            physical_height,
            swapchain.swapchain,
            surface_info.surface_format(0)?,
        );
        Ok(())
    }

    pub fn integration(&self) -> &egui_winit_ash_integration::Integration<Arc<Mutex<Allocator>>> {
        &self.integration
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.integration.destroy();
            ManuallyDrop::drop(&mut self.integration);
        }
    }
}

impl std::ops::Deref for SetUpEguiIntegration {
    type Target = egui_winit_ash_integration::Integration<Arc<Mutex<Allocator>>>;

    fn deref(&self) -> &Self::Target {
        &self.integration
    }
}
