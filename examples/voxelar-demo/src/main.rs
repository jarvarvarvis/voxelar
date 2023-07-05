extern crate voxelar;

use voxelar::*;

fn main() {
    let mut ctx = Voxelar::new();
    let window = ctx.create_window(600, 300, "Demo", glfw::WindowMode::Windowed);
}
