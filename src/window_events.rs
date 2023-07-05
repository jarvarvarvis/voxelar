use std::sync::mpsc::Receiver;

use glfw::WindowEvent;

pub type WindowEventTuple = (f64, WindowEvent);
pub type WindowEventReceiver = Receiver<WindowEventTuple>;

pub struct VoxelarWindowEvents {
    receiver: WindowEventReceiver,
}

impl VoxelarWindowEvents {
    pub fn new(receiver: WindowEventReceiver) -> Self {
        Self { receiver }
    }

    pub fn flush(&mut self) -> impl Iterator<Item = WindowEventTuple> + '_ {
        glfw::flush_messages(&self.receiver)
    }
}
