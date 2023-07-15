//! This is a module that provides the window creation API in the form of the `VoxelarWindow`
//! struct.

use glfw::Window;
use raw_window_handle::*;

use crate::receivable_events::ReceivableEvents;

/// A struct holding an instance of a window created using glfw.
///
/// Additionally, this structure holds the current title of the window to make it easier to query.
pub struct VoxelarWindow {
    glfw_window: Window,
    title: String,
}

impl VoxelarWindow {
    /// Creates a new `VoxelarWindow` from an initialized glfw window and a title.
    pub(crate) fn new(glfw_window: Window, title: &str) -> Self {
        Self {
            glfw_window,
            title: String::from(title),
        }
    }

    /// This function is a wrapper around `glfw::Window::should_close`.
    ///
    /// It is used to check whether or not this window should be closed or not.
    ///
    /// The closed/not closed state can be set using `VoxelarWindow::set_should_close`.
    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    /// This function is a wrapper around `glfw::Window::set_should_close`.
    ///
    /// It is used to set the closed/not closed state of this window.
    pub fn set_should_close(&mut self, value: bool) {
        self.glfw_window.set_should_close(value)
    }

    /// Sets the receivable events for this window.
    ///
    /// See the documentation of `ReceivableEvents` for information on how it is initialized.
    pub fn set_receivable_events(&mut self, events: ReceivableEvents) {
        events.set_for(self);
    }

    /// Returns a reference to the internal `Window` handle created using glfw.
    pub fn glfw_window(&self) -> &Window {
        &self.glfw_window
    }

    /// Returns a mutable reference to the internal `Window` handle created using glfw.
    pub fn glfw_window_mut(&mut self) -> &mut Window {
        &mut self.glfw_window
    }

    /// This function is a wrapper around `glfw::Window::raw_window_handle`.
    ///
    /// Returns the `RawWindowHandle` (from the raw-window-handle crate) of the internal glfw
    /// `Window` handle.
    pub fn raw_window_handle(&self) -> RawWindowHandle {
        self.glfw_window.raw_window_handle()
    }

    /// This function is a wrapper around `glfw::Window::raw_display_handle`.
    ///
    /// Returns the `RawDisplayHandle` (from the raw-window-handle crate) of the internal glfw
    /// `Window` handle.
    pub fn raw_display_handle(&self) -> RawDisplayHandle {
        self.glfw_window.raw_display_handle()
    }

    /// Updates the title of the window to a new value.
    ///
    /// For one, this function is a wrapper around `glfw::Window::set_title`. Additionally, this function 
    /// stores the provided `title` value, so that it can be queried later on.
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.glfw_window.set_title(title)
    }

    /// Gets the last title set using `VoxelarWindow::set_title` or the title that this window has 
    /// been created with if `VoxelarWindow::set_title` was not called yet.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     ctx.window_hint(glfw::WindowHint::Visible(false));
    ///     let (mut window, mut events) =
    ///         ctx.create_window(200, 100, "Demo", glfw::WindowMode::Windowed)?;
    ///
    ///     assert_eq!("Demo", window.get_title());
    ///
    ///     // Set a new title
    ///     window.set_title("New title");
    ///     assert_eq!("New title", window.get_title());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    /// This function is a wrapper around `glfw::Window::get_size`.
    ///
    /// It returns a tuple of this window's width (the first value of the tuple) and height (the
    /// second value of the tuple).
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     ctx.window_hint(glfw::WindowHint::Visible(false));
    ///     let (mut window, mut events) =
    ///         ctx.create_window(200, 100, "Demo", glfw::WindowMode::Windowed)?;
    ///
    ///     assert_eq!((200, 100), window.get_size());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_size(&self) -> (i32, i32) {
        self.glfw_window.get_size()
    }

    /// Calculates the current aspect ratio of this window.
    ///
    /// # Examples
    ///
    /// ```
    /// use voxelar::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut ctx = Voxelar::new()?;
    ///
    ///     ctx.window_hint(glfw::WindowHint::Visible(false));
    ///     let (mut window, mut events) =
    ///         ctx.create_window(200, 100, "Demo", glfw::WindowMode::Windowed)?;
    ///
    ///     assert_eq!(2.0, window.aspect_ratio());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn aspect_ratio(&self) -> f32 {
        let (width, height) = self.get_size();
        width as f32 / height as f32
    }
}
