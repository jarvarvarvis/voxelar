//! This is a module that contains the `RenderContext` trait, an abstract interface for graphics
//! APIs to be loaded from a `Voxelar` context and a `VoxelarWindow`.
//!
//! If wanted, the API can provide an implementation of `RenderContext::get_info` to provide the
//! user with some API-related information.

use crate::window::VoxelarWindow;
use crate::Voxelar;

/// The `RenderContext` trait.
///
/// Generally, an API implementation should provide a struct that implements this trait.
/// (See how `VulkanContext` does it, for example).
pub trait RenderContext {
    /// This function loads this API using the `Voxelar` context and the `VoxelarWindow`.
    ///
    /// If loading the API failed, the result of this function will be an `Err`.
    fn load(ctx: &mut Voxelar, window: &mut VoxelarWindow) -> crate::Result<Self>
    where
        Self: Sized;

    /// This function returns information about this `RenderContext` in the form of the type T.
    ///
    /// Previously, I just returned a String here, but this seems much more flexible.
    fn get_info<T>(&self) -> crate::Result<T>;
}
