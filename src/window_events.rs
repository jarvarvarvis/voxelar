//! This is a module that provides window event handling in the form of the `VoxelarWindowEvents`
//! struct.

use std::sync::mpsc::Receiver;

use glfw::WindowEvent;

pub(crate) type WindowEventTuple = (f64, WindowEvent);
pub(crate) type WindowEventReceiver = Receiver<WindowEventTuple>;

/// A struct holding the instance of the a window's event receiver.
///
/// By calling `VoxelarWindowEvents::flush`, the events that have been queried from glfw can be
/// iterated.
pub struct VoxelarWindowEvents {
    receiver: WindowEventReceiver,
}

impl VoxelarWindowEvents {
    /// Creates a new `VoxelarWindowEvents` struct using the provided `WindowEventReceiver`.
    pub(crate) fn new(receiver: WindowEventReceiver) -> Self {
        Self { receiver }
    }

    /// Flushes the events that have been received from glfw.
    ///
    /// This function returns an iterator over `WindowEvent`s, which can be matched against.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut ctx = ...;
    /// let (mut window, mut events) = ...;
    ///
    /// while !window.should_close() {
    ///     ctx.poll_events();
    ///     for event in events.flush() {
    ///     }
    /// }
    /// ```
    pub fn flush(&mut self) -> impl Iterator<Item = WindowEvent> + '_ {
        glfw::flush_messages(&self.receiver).map(|(_, event)| event)
    }
}
