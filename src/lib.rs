//! The voxelar engine backend.
//!
//! This is the main module that provides all functionality required to create windows, interact 
//! with the Vulkan graphics API for drawing etc. as well as voxel functionality (at least in the future).
//!
//! Module overview:
//! - engine: Provides various game engine utilities (e.g. frame time/FPS measurement etc.)
//! - receivable\_events: Contains the `ReceivableEvents` struct
//! - render\_context: Contains the `RenderContext` trait
//! - result: Provides voxelar's error type, a `Result` with that type and more
//! - vulkan: Provides all Vulkan functionality
//! - window: Provides the window creation API
//! - window_events: Provides the window event API
//!
//! Furthermore, this module provides access to the following external crates:
//! - glfw: Bindings for the glfw window API
//! - ash: Bindings for Vulkan
//! - shaderc: Bindings for the Shaderc library
//! - nalgebra: A linear algebra library

pub extern crate ash;
pub extern crate glfw;
pub extern crate nalgebra;
pub extern crate shaderc;

use glfw::*;

pub mod engine;
pub mod receivable_events;
pub mod render_context;
pub mod result;
pub mod vulkan;
pub mod window;
pub mod window_events;

pub use result::*;

use render_context::RenderContext;
use result::Context;
use window::*;
use window_events::*;

/// The main Voxelar context from which everything is initialized.
///
/// This structure stores an initialized GLFW context.
pub struct Voxelar {
    glfw: Glfw,
}

impl Voxelar {
    /// Tries to create a new Voxelar structure from a GLFW context.
    ///
    /// This function tries to call `glfw::init` with a `glfw::FAIL_ON_ERRORS` parameter to instruct
    /// the glfw crate to panic when an error is encountered.
    ///
    /// It's important to call this function only ONCE and from the main thread.
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
            glfw: glfw::init(glfw::FAIL_ON_ERRORS)?,
        })
    }

    /// This function is a wrapper around `Glfw::window_hint`.
    ///
    /// It is used to set the window hints for the next call to `Voxelar::create_window`. The hints
    /// can be reset to their default values using the `Voxelar::default_window_hints` function.
    ///
    /// See the documentation of [glfw::WindowHint](https://docs.rs/glfw/0.52.0/glfw/enum.WindowHint.html)
    /// for more details on the available hints.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     // WindowHint::Visible specifies whether or not the window will be
    ///     // visible on creation.
    ///     ctx.window_hint(glfw::WindowHint::Visible(true));
    ///     Ok(())
    /// }
    /// ```
    pub fn window_hint(&mut self, hint: WindowHint) {
        self.glfw.window_hint(hint);
    }

    /// This function is a wrapper around `Glfw::default_window_hints`.
    ///
    /// It is used to reset the window hints previously set by the `window_hint`
    /// function to their default values.
    pub fn default_window_hints(&mut self) {
        self.glfw.default_window_hints();
    }

    /// This function is a wrapper around `Glfw::create_window`.
    ///
    /// It is used to create a window with some specific `width` and `height`, a `title` and a
    /// `mode` (windowed or fullscreen).
    /// Glfw will also use the window hints set using `Voxelar::window_hint` for the creation of this
    /// window.
    ///
    /// See the documentation of [glfw::WindowMode](https://docs.rs/glfw/0.52.0/glfw/enum.WindowMode.html)
    /// for details on the `mode` parameter.
    ///
    /// This function returns a tuple of `VoxelarWindow` and `VoxelarWindowEvents`, wrapped in a
    /// `voxelar::Result`. See the documentation for those structures for more information on how
    /// to use them.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     // WindowHint::Visible specifies whether or not the window will be
    ///     // visible on creation.
    ///     ctx.window_hint(glfw::WindowHint::Visible(false));
    ///     let (mut window, mut events) =
    ///         ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn create_window<'win>(
        &'win mut self,
        width: u32,
        height: u32,
        title: &'win str,
        mode: WindowMode,
    ) -> crate::Result<(VoxelarWindow, VoxelarWindowEvents)> {
        let (window, events) = self
            .glfw
            .create_window(width, height, title, mode)
            .context("Failed to create GLFW window.".to_string())?;
        let window = VoxelarWindow::new(window, title);
        let events = VoxelarWindowEvents::new(events);

        Ok((window, events))
    }

    /// This function is a wrapper around `Glfw::poll_events`.
    ///
    /// It is used to immediately process the events that the context has received.
    ///
    /// To be able to use `VoxelarWindowEvents::flush`, this function needs to be called.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     // WindowHint::Visible specifies whether or not the window will be
    ///     // visible on creation.
    ///     ctx.window_hint(glfw::WindowHint::Visible(false));
    ///     let (mut window, mut events) =
    ///         ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed)?;
    ///
    ///     while !window.should_close() {
    ///         ctx.poll_events();
    ///         for event in events.flush() {
    ///
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    /// This function is a wrapper around `Glfw::vulkan_supported`.
    ///
    /// It is used to query whether or not Vulkan is supported by glfw.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     assert!(ctx.vulkan_supported());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn vulkan_supported(&self) -> bool {
        self.glfw.vulkan_supported()
    }

    /// This function is a wrapper around `Glfw::get_required_instance_extensions`.
    ///
    /// Quoting the glfw crate's documentation:
    ///
    /// > This function returns a Vector of names of Vulkan instance extensions
    /// > required by GLFW for creating Vulkan surfaces for GLFW windows. If successful,
    /// > the list will always contains `VK_KHR_surface`, so if you don't require any
    /// > additional extensions you can pass this list directly to the `VkInstanceCreateInfo` struct.
    ///
    /// > Will return `None` if the API is unavailable.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     let instance_extensions = ctx.get_required_instance_extensions();
    ///     assert!(instance_extensions.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_required_instance_extensions(&self) -> Option<Vec<String>> {
        self.glfw.get_required_instance_extensions()
    }

    /// Loads a specific `RenderContext` using its `load` function.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    /// use voxelar::vulkan::*;
    /// use voxelar::vulkan::debug::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     // WindowHint::Visible specifies whether or not the window will be visible on creation.
    ///     ctx.window_hint(glfw::WindowHint::Visible(false));
    ///     let (mut window, mut events) =
    ///         ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed)?;
    ///
    ///     let vulkan_context = ctx
    ///         .load_render_context_for_window::<VulkanContext<KHRVerificationAndDebugMessenger>>(
    ///             &mut window
    ///         );
    ///     assert!(vulkan_context.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn load_render_context_for_window<C: RenderContext>(
        &mut self,
        window: &mut VoxelarWindow,
    ) -> crate::Result<C> {
        C::load(self, window)
    }

    /// This function is a wrapper around `Glfw::get_time`.
    ///
    /// It is used to get the current value of glfw's internal timer.
    ///
    /// The returned value is a floating point value, with whole numbers equaling whole seconds.
    /// Therefore, A value of 5.0 is equal to 5 seconds, a value of 0.5 is equal to 500 milliseconds.
    ///
    /// Unless the time has been reset using `set_time`, this time is equal to the time since the
    /// context has been created.
    pub fn current_time(&self) -> f64 {
        self.glfw.get_time()
    }

    /// This function is a wrapper around `Glfw::set_time`.
    ///
    /// It is used to set the current value of glfw's internal timer.
    pub fn set_time(&mut self, time: f64) {
        self.glfw.set_time(time)
    }
}
