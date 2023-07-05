use crate::window::VoxelarWindow;

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

    pub fn pos(mut self, pos: bool) -> Self {
        self.pos = pos;
        self
    }

    pub fn size(mut self, size: bool) -> Self {
        self.size = size;
        self
    }

    pub fn close(mut self, close: bool) -> Self {
        self.close = close;
        self
    }

    pub fn refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    pub fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
    }

    pub fn iconify(mut self, iconify: bool) -> Self {
        self.iconify = iconify;
        self
    }

    pub fn framebuffer_size(mut self, framebuffer_size: bool) -> Self {
        self.framebuffer_size = framebuffer_size;
        self
    }

    pub fn key(mut self, key: bool) -> Self {
        self.key = key;
        self
    }

    pub fn char(mut self, char: bool) -> Self {
        self.char = char;
        self
    }

    pub fn char_mods(mut self, char_mods: bool) -> Self {
        self.char_mods = char_mods;
        self
    }

    pub fn mouse_button(mut self, mouse_button: bool) -> Self {
        self.mouse_button = mouse_button;
        self
    }

    pub fn cursor_pos(mut self, cursor_pos: bool) -> Self {
        self.cursor_pos = cursor_pos;
        self
    }

    pub fn cursor_enter(mut self, cursor_enter: bool) -> Self {
        self.cursor_enter = cursor_enter;
        self
    }

    pub fn scroll(mut self, scroll: bool) -> Self {
        self.scroll = scroll;
        self
    }

    pub fn drag_and_drop(mut self, drag_and_drop: bool) -> Self {
        self.drag_and_drop = drag_and_drop;
        self
    }

    pub fn maximize(mut self, maximize: bool) -> Self {
        self.maximize = maximize;
        self
    }

    pub fn content_scale(mut self, content_scale: bool) -> Self {
        self.content_scale = content_scale;
        self
    }
}
