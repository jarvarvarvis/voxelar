extern crate voxelar;

mod demo;
mod vertex;

use voxelar::ash::vk::PresentModeKHR;

use voxelar::vulkan::creation_info::*;
use voxelar::vulkan::debug::*;
use voxelar::vulkan::VulkanContext;
use voxelar::window::*;
use voxelar::winit::event::*;
use voxelar::winit::event_loop::ControlFlow;
use voxelar::*;

use crate::demo::Demo;

fn main() -> Result<()> {
    let mut ctx = Voxelar::new()?;

    let (mut window, event_loop) =
        ctx.create_window(800, 600, "Demo", VoxelarWindowMode::Windowed)?;

    let mut vulkan_context = ctx
        .load_render_context_for_window::<KHRVerificationAndDebugMessenger, VulkanContext>(
            &mut window,
        )?;

    let creation_info = DataStructureCreationInfo {
        swapchain_present_mode: PresentModeInitMode::Find(PresentModeKHR::FIFO),
        frame_overlap: 2,
        allocator_debug_settings: Default::default(),
    };
    vulkan_context.create_default_data_structures(window.get_size(), creation_info)?;

    let phys_device = vulkan_context.physical_device()?;
    println!("Found physical device: {:?}", phys_device.name());

    let egui_integration = vulkan_context.create_egui_integration(&window, &event_loop)?;

    let mut demo = Demo::new(&ctx, &vulkan_context, egui_integration)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: window_event,
                window_id: _,
            } => {
                demo.handle_egui_integration_event(&window_event);
                match window_event {
                    WindowEvent::Resized(_) => {
                        demo.on_resize(&vulkan_context)?;
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        demo.on_resize(&vulkan_context)?;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                demo.render(&mut window, &mut vulkan_context)?;
                demo.update_frame_time_manager(&ctx);
            }
            Event::LoopDestroyed => {
                demo.destroy(&vulkan_context)?;
            }
            _ => {}
        };
        Ok(())
    });
}
