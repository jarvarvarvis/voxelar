//! The voxelar engine backend.
//!
//! This is the main module that provides all functionality required to create windows, interact
//! with the Vulkan graphics API for drawing etc. as well as voxel functionality (at least in the future).
//!
//! Module overview:
//! - engine: Provides various game engine utilities (e.g. frame time/FPS measurement etc.)
//! - render\_context: Contains the `RenderContext` trait
//! - result: Provides voxelar's error type, a `Result` with that type and more
//! - voxel: Provides all functionality for voxel world storage, rendering etc.
//! - vulkan: Provides all Vulkan functionality
//! - window: Provides the window creation API
//!
//! Furthermore, this module provides access to the following external crates:
//! - ash: Bindings for Vulkan
//! - nalgebra: A linear algebra library
//! - voxelar_utils: Various general-purpose Rust utilities used by voxelar
//! - winit: A cross-platform windowing library

pub extern crate ash;
pub extern crate nalgebra;
pub extern crate voxelar_utils;
pub extern crate winit;

pub mod engine;
pub mod render_context;
pub mod result;
pub mod voxel;
pub mod vulkan;
pub mod window;

use std::time::Instant;

pub use result::*;

use render_context::RenderContext;
use window::*;

use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

/// The main Voxelar context from which everything is initialized.
///
/// This structure stores an initialized GLFW context.
pub struct Voxelar {
    start_time: Instant,
}

impl Voxelar {
    /// This function creates a new Voxelar context.
    ///
    /// NOTE: It's good practice to make the main function return a voxelar::Result<()>
    ///       to make it possible to use the question mark operator on all functions that
    ///       return a voxelar::Result.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let ctx = Voxelar::new()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            start_time: Instant::now(),
        })
    }

    /// This function is used to create a window with some specific `width` and `height`, a `title` and a
    /// `window_mode` (windowed or maximized), as well as an `EventLoop` receiving from this
    /// window.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    /// use voxelar::window::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     // WindowHint::Visible specifies whether or not the window will be
    ///     // visible on creation.
    ///     let (mut window, event_loop) = ctx.create_window(800, 600, "Demo", VoxelarWindowMode::Windowed)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn create_window(
        &self,
        width: u32,
        height: u32,
        title: &str,
        window_mode: VoxelarWindowMode,
    ) -> crate::Result<(VoxelarWindow, VoxelarEventLoop)> {
        let event_loop = EventLoop::new();
        let builder = WindowBuilder::new()
            .with_title(title)
            .with_maximized(window_mode == VoxelarWindowMode::Maximized)
            .with_inner_size(LogicalSize::new(width, height));
        let window = VoxelarWindow::from_window_builder(builder, &event_loop)?;
        let event_loop = VoxelarEventLoop::new(event_loop);
        Ok((window, event_loop))
    }

    /// This function loads a specific `RenderContext` using its `load` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    /// use voxelar::window::*;
    /// use voxelar::vulkan::*;
    /// use voxelar::vulkan::debug::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     // WindowHint::Visible specifies whether or not the window will be visible on creation.
    ///     let (mut window, event_loop) = ctx.create_window(800, 600, "Demo", VoxelarWindowMode::Windowed)?;
    ///
    ///     let vulkan_context = ctx
    ///         .load_render_context_for_window::<NoVerification, VulkanContext>(
    ///             &mut window
    ///         );
    ///     assert!(vulkan_context.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn load_render_context_for_window<LoadData, C: RenderContext<LoadData>>(
        &mut self,
        window: &mut VoxelarWindow,
    ) -> crate::Result<C> {
        C::load(self, window)
    }

    pub fn current_time(&self) -> f64 {
        let current_time = Instant::now();
        let time_diff = current_time - self.start_time;
        time_diff.as_secs_f64()
    }
}
