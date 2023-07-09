extern crate voxelar;

mod triangle;
mod vertex;

use voxelar::glfw::*;
use voxelar::receivable_events::*;
use voxelar::vulkan::debug::*;
use voxelar::vulkan::VulkanContext;
use voxelar::window::*;
use voxelar::*;

use crate::triangle::TriangleDemo;

fn main() -> Result<()> {
    let mut ctx = Voxelar::new();

    ctx.window_hint(WindowHint::Visible(true));
    ctx.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    let (mut window, mut events) =
        ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed)?;

    window.set_receivable_events(ReceivableEvents::all());

    let mut vulkan_context = ctx
        .load_render_context_for_window::<VulkanContext<KHRVerificationAndDebugMessenger>>(
            &mut window,
        )?;

    vulkan_context.find_usable_physical_device()?;
    let phys_device = vulkan_context.physical_device.as_ref().unwrap();
    println!("Found physical device: {:?}", phys_device.name());

    vulkan_context.create_virtual_device()?;
    vulkan_context.create_swapchain(window.get_size())?;
    vulkan_context.create_command_logic()?;
    vulkan_context.create_present_images()?;
    vulkan_context.create_sync_primitives()?;
    vulkan_context.create_depth_image(window.get_size())?;

    let mut demo = TriangleDemo::new(&vulkan_context)?;

    while !window.should_close() {
        demo.render(&vulkan_context)?;
        ctx.poll_events();
        for (_, event) in events.flush() {
            handle_window_event(&vulkan_context, &mut demo, &mut window, event)?;
        }
    }

    demo.destroy(&vulkan_context)?;

    Ok(())
}

fn handle_window_event<V: VerificationProvider>(
    vulkan_context: &VulkanContext<V>,
    triangle_demo: &mut TriangleDemo,
    window: &mut VoxelarWindow,
    event: glfw::WindowEvent,
) -> crate::Result<()> {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => {
            triangle_demo.update_viewports_and_scissors(vulkan_context, width, height)?
        }
        _ => {}
    }
    Ok(())
}
