use crate::window::VoxelarWindow;

pub trait RenderContext {
    fn load(window: &mut VoxelarWindow);
}
