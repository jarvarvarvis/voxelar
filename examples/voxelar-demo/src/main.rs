extern crate voxelar;

mod demo;
mod vertex;

use voxelar::ash::vk::PresentModeKHR;

use voxelar::glfw::*;
use voxelar::receivable_events::*;
use voxelar::vulkan::creation_info::*;
use voxelar::vulkan::debug::*;
use voxelar::vulkan::VulkanContext;
use voxelar::window::*;
use voxelar::*;

use crate::demo::Demo;

fn main() -> Result<()> {
    let mut ctx = Voxelar::new()?;

    ctx.window_hint(WindowHint::Visible(true));
    ctx.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    let (mut window, mut events) =
        ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed)?;

    window.set_receivable_events(ReceivableEvents::all());

    let mut vulkan_context = ctx
        .load_render_context_for_window::<KHRVerificationAndDebugMessenger, VulkanContext>(
            &mut window,
        )?;

    let creation_info = DataStructureCreationInfo {
        swapchain_present_mode: PresentModeInitMode::Find(PresentModeKHR::FIFO),
        frame_overlap: 2,
    };
    vulkan_context.create_default_data_structures(window.get_size(), creation_info)?;

    let phys_device = vulkan_context.physical_device()?;
    println!("Found physical device: {:?}", phys_device.name());

    let mut demo = Demo::new(&ctx, &vulkan_context)?;

    while !window.should_close() {
        demo.render(&mut window, &mut vulkan_context)?;
        demo.update_frame_time_manager(&ctx);

        ctx.poll_events();
        for event in events.flush() {
            handle_window_event(&mut demo, &mut window, event)?;
        }
    }

    demo.destroy(&vulkan_context)?;

    Ok(())
}

fn handle_window_event(
    triangle_demo: &mut Demo,
    window: &mut VoxelarWindow,
    event: glfw::WindowEvent,
) -> crate::Result<()> {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(_, _) => {
            triangle_demo.recreate_swapchain = true;
        }
        _ => {}
    }
    Ok(())
}
