extern crate voxelar;

use voxelar::glfw::*;
use voxelar::receivable_events::*;
use voxelar::window::*;
use voxelar::*;

fn main() -> Result<()> {
    let mut ctx = Voxelar::new();

    ctx.window_hint(WindowHint::Visible(true));
    ctx.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    let (mut window, mut events) = ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed);

    assert!(ctx.vulkan_supported());

    let required_extensions = ctx.get_required_instance_extensions().unwrap_or(vec![]);
    assert!(required_extensions.contains(&"VK_KHR_surface".to_string()));

    println!("Required Vulkan extensions: {:?}", required_extensions);
    
    window.set_receivable_events(ReceivableEvents::all());

    while !window.should_close() {
        ctx.poll_events();
        for (_, event) in events.flush() {
            handle_window_event(&mut window, event);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut VoxelarWindow, event: glfw::WindowEvent) {
    println!("Event: {:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
