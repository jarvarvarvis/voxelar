extern crate voxelar;

use voxelar::glfw::*;
use voxelar::receivable_events::*;
use voxelar::vulkan::VulkanContext;
use voxelar::window::*;
use voxelar::*;

fn main() -> Result<()> {
    let mut ctx = Voxelar::new();

    ctx.window_hint(WindowHint::Visible(true));
    ctx.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    let (mut window, mut events) = ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed)?;
    
    window.set_receivable_events(ReceivableEvents::all());

    let vulkan_context = ctx.load_render_context_for_window::<VulkanContext>(&mut window)?;

    while !window.should_close() {
        ctx.poll_events();
        for (_, event) in events.flush() {
            handle_window_event(&mut window, event);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut VoxelarWindow, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
