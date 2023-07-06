use crate::window::VoxelarWindow;

pub trait RenderContext {
    fn load(window: &mut VoxelarWindow) -> Self;
    fn get_info(&self) -> crate::Result<String>;
}
