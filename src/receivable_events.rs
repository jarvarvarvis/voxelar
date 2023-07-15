//! This is a module that contains the `ReceivableEvents` struct, usefule for specifying all the
//! events that should be received by a `VoxelarWindow`.
//!
//! See the documentation of `ReceivableEvents` for more information on the available events.

use crate::window::VoxelarWindow;

/// This structure stores booleans that specify whether or not an event should be received by the
/// window system.
///
/// # Examples
///
/// The `ReceivableEvents` struct can be used like a builder, e.g.
/// ```ignore
/// let receivable_events = ReceivableEvents::all()
///     .focus(false)
///     .drag_and_drop(false);
/// ```
///
/// This specific example will initialize the struct with all events being able to be received,
/// but will then set the state of the focus event and the drag and drop event to false (i.e. not
/// being able to be received).
#[derive(Default)]
pub struct ReceivableEvents {
    pos: bool,
    size: bool,
    close: bool,
    refresh: bool,
    focus: bool,
    iconify: bool,
    framebuffer_size: bool,
    key: bool,
    char: bool,
    char_mods: bool,
    mouse_button: bool,
    cursor_pos: bool,
    cursor_enter: bool,
    scroll: bool,
    drag_and_drop: bool,
    maximize: bool,
    content_scale: bool,
}

impl ReceivableEvents {
    /// Initialize the `ReceivableEvents` struct with all events to be received by the window.
    ///
    /// Generally, this is the easiest solution.
    pub fn all() -> Self {
        Self {
            pos: true,
            size: true,
            close: true,
            refresh: true,
            focus: true,
            iconify: true,
            framebuffer_size: true,
            key: true,
            char: true,
            char_mods: true,
            mouse_button: true,
            cursor_pos: true,
            cursor_enter: true,
            scroll: true,
            drag_and_drop: true,
            maximize: true,
            content_scale: true,
        }
    }

    /// This function will set the receivable/not receivable state of each event for some
    /// `VoxelarWindow`.
    ///
    /// This will move the current value of `ReceivableEvents`, so you can't use it afterwards.
    pub fn set_for(self, window: &mut VoxelarWindow) {
        let glfw_window = window.glfw_window_mut();

        glfw_window.set_pos_polling(self.pos);
        glfw_window.set_size_polling(self.size);
        glfw_window.set_close_polling(self.close);
        glfw_window.set_refresh_polling(self.refresh);
        glfw_window.set_focus_polling(self.focus);
        glfw_window.set_iconify_polling(self.iconify);
        glfw_window.set_framebuffer_size_polling(self.framebuffer_size);
        glfw_window.set_key_polling(self.key);
        glfw_window.set_char_polling(self.char);
        glfw_window.set_char_mods_polling(self.char_mods);
        glfw_window.set_mouse_button_polling(self.mouse_button);
        glfw_window.set_cursor_pos_polling(self.cursor_pos);
        glfw_window.set_cursor_enter_polling(self.cursor_enter);
        glfw_window.set_scroll_polling(self.scroll);
        glfw_window.set_drag_and_drop_polling(self.drag_and_drop);
        glfw_window.set_maximize_polling(self.maximize);
        glfw_window.set_content_scale_polling(self.content_scale);
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Pos.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn pos(mut self, pos: bool) -> Self {
        self.pos = pos;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Size.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn size(mut self, size: bool) -> Self {
        self.size = size;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Close.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn close(mut self, close: bool) -> Self {
        self.close = close;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Refresh.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Focus.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Iconify.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn iconify(mut self, iconify: bool) -> Self {
        self.iconify = iconify;
        self
    }

    /// This function sets the receivable/not receivable state of
    /// glfw::WindowEvent::FramebufferSize.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn framebuffer_size(mut self, framebuffer_size: bool) -> Self {
        self.framebuffer_size = framebuffer_size;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Key.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn key(mut self, key: bool) -> Self {
        self.key = key;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Char.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn char(mut self, char: bool) -> Self {
        self.char = char;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::CharModifiers.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn char_mods(mut self, char_mods: bool) -> Self {
        self.char_mods = char_mods;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::MouseButton.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn mouse_button(mut self, mouse_button: bool) -> Self {
        self.mouse_button = mouse_button;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::CursorPos.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn cursor_pos(mut self, cursor_pos: bool) -> Self {
        self.cursor_pos = cursor_pos;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::CursorEnter.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn cursor_enter(mut self, cursor_enter: bool) -> Self {
        self.cursor_enter = cursor_enter;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Scroll.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn scroll(mut self, scroll: bool) -> Self {
        self.scroll = scroll;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::FileDrop.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn drag_and_drop(mut self, drag_and_drop: bool) -> Self {
        self.drag_and_drop = drag_and_drop;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::Maximize.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn maximize(mut self, maximize: bool) -> Self {
        self.maximize = maximize;
        self
    }

    /// This function sets the receivable/not receivable state of glfw::WindowEvent::ContentScale.
    ///
    /// See the documentation of [glfw::WindowEvent](https://docs.rs/glfw/0.52.0/glfw/enum.WindowEvent.html)
    /// for details on which additional data each WindowEvent holds.
    pub fn content_scale(mut self, content_scale: bool) -> Self {
        self.content_scale = content_scale;
        self
    }
}
