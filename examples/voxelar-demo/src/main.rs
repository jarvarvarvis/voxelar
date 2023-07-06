extern crate voxelar;

use voxelar::glfw::*;
use voxelar::receivable_events::*;
use voxelar::window::*;
use voxelar::*;

fn main() {
    let mut ctx = Voxelar::new();
    let (mut window, mut events) = ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed);
    window.set_receivable_events(ReceivableEvents::all());

    ctx.set_swap_interval(SwapInterval::Sync(1));

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
