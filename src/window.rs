use raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

#[derive(Debug, PartialEq, Eq)]
pub enum VoxelarWindowMode {
    Windowed,
    Maximized,
}

pub struct VoxelarWindow {
    window: Window,
}

impl VoxelarWindow {
    pub fn from_window_builder(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>,
    ) -> crate::Result<Self> {
        let window = window_builder.build(&event_loop)?;

        Ok(Self { window })
    }

    pub fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }

    pub fn raw_display_handle(&self) -> RawDisplayHandle {
        self.window.raw_display_handle()
    }

    pub fn get_title(&self) -> String {
        self.window.title()
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title)
    }

    pub fn get_size(&self) -> (i32, i32) {
        let inner_size = self.window.inner_size();
        (inner_size.width as i32, inner_size.height as i32)
    }

    pub fn aspect_ratio(&self) -> f32 {
        let size = self.get_size();
        size.0 as f32 / size.1 as f32
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}

pub struct VoxelarEventLoop {
    event_loop: EventLoop<()>,
}

impl VoxelarEventLoop {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        Self { event_loop }
    }

    pub fn run<EventHandlerFn>(self, mut event_handler: EventHandlerFn) -> !
    where
        EventHandlerFn: 'static
            + FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow) -> crate::Result<()>,
    {
        self.event_loop.run(move |event, target, control_flow| {
            event_handler(event, target, control_flow).unwrap()
        })
    }
}
