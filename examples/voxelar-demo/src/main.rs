extern crate voxelar;

use voxelar::glfw::*;
use voxelar::receivable_events::*;
use voxelar::window::*;
use voxelar::*;

fn main() {
    let mut ctx = Voxelar::new();
    let (mut window, mut events) = ctx.create_window(600, 300, "Demo", glfw::WindowMode::Windowed);

    let receivable_events = ReceivableEvents::all();
    receivable_events.set_for(&mut window);

    while !window.should_close() {
        ctx.poll_events();
        for (_, event) in events.flush() {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut VoxelarWindow, event: glfw::WindowEvent) {
    println!("Event: {:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
