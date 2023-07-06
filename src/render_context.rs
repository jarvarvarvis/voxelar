use crate::window::VoxelarWindow;
use crate::Voxelar;

pub trait RenderContext {
    fn load(ctx: &mut Voxelar, window: &mut VoxelarWindow) -> crate::Result<Self>
    where
        Self: Sized;
    fn get_info(&self) -> crate::Result<String>;
}
