use crate::*;
use crate::render::RenderContext;
use sdl2::keyboard::Keycode;
use unicode_segmentation::UnicodeSegmentation;

pub enum Mode {
    NORMAL,
    INSERT,
}

pub struct Editor {
    pub mode: Mode, 

    pub buffer: buffer::Buffer,
    pub y_render_offset: usize,

    pub editing_file_path: String,

    pub cursor_x: usize,
    pub cursor_y: usize,
    pub cursor_animation_instant: Instant,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: Mode::INSERT,

            buffer: buffer::Buffer::new(),
            y_render_offset: 0,

            editing_file_path: String::from(""),

            cursor_x: 0,
            cursor_y: 0,
            cursor_animation_instant: Instant::now(),
        }
    }
    pub fn move_cursor(&mut self, render: &RenderContext, x: i32, y: i32) {
        let buffer_string = self.buffer.to_string();
        let buffer_lines: Vec<_> = buffer_string
            .split('\n')
            .map(|it| UnicodeSegmentation::graphemes(it, true).collect::<Vec<_>>())
            .collect();

        if self.cursor_x == 0 && x < 0 {
            if self.cursor_y != 0 {
                self.cursor_y -= 1;
                self.cursor_x = buffer_lines.get(self.cursor_y)
                    .unwrap_or(&Vec::new()).len();
            }
        } else {
            self.cursor_x = ((self.cursor_x as i32) + x) as usize;
        }

        if self.cursor_y == 0 && y < 0 {
            self.cursor_y = 0;
        } else {
            self.cursor_y = ((self.cursor_y as i32) + y) as usize;
        }

        if self.cursor_y >= buffer_lines.len() {
            self.cursor_y = buffer_lines.len() - 1;
        }
        if self.cursor_x >= buffer_lines[self.cursor_y].len() {
            self.cursor_x = buffer_lines[self.cursor_y].len();
        }

        let ch = render.character_height;
        let window_height_in_characters = (render.height() / ch) as usize;
        if y > 0 &&
            self.cursor_y > self.y_render_offset + window_height_in_characters - 7 
        {
            self.y_render_offset += y as usize;
        }
        if y < 0 && self.cursor_y < self.y_render_offset + 5
            && (self.y_render_offset as i32) + y >= 0
        {
            self.y_render_offset -= (-y) as usize;
        }
        self.y_render_offset = min(self.y_render_offset, buffer_lines.len());

        self.cursor_animation_instant = Instant::now();
    }

    pub fn cursor_position_in_buffer(&self) -> usize {
        let buffer_string = self.buffer.to_string();
        let buffer_lines: Vec<&str> = buffer_string
            .split('\n')
            .take(self.cursor_y + 1)
            .collect();
        let length_before_line = buffer_lines[0..self.cursor_y]
            .iter()
            .map(|t| t.len() + 1)
            .sum::<usize>();
        let length_inside_line = UnicodeSegmentation::
            graphemes(buffer_lines[self.cursor_y], true)
            .take(self.cursor_x)
            .map(|gc| gc.len())
            .sum::<usize>();
        length_before_line + length_inside_line
    }

    pub fn handle_keys(&mut self, render: &RenderContext, keycode: Keycode, ctrl: bool) {
        match self.mode {
            Mode::NORMAL => self.handle_keys_in_normal_mode(render, keycode, ctrl),
            Mode::INSERT => self.handle_keys_in_insert_mode(render, keycode, ctrl),
        }
    }

    fn handle_keys_in_normal_mode(&mut self, render: &RenderContext, keycode: Keycode, _ctrl: bool) {
        match keycode {
            Keycode::I => self.mode = Mode::INSERT,
            Keycode::H => self.move_cursor(render, -1, 0),
            Keycode::L => self.move_cursor(render, 1, 0),
            Keycode::K => self.move_cursor(render, 0, -1),
            Keycode::J => self.move_cursor(render, 0, 1),
            _ => {},
        }
    }
    fn handle_keys_in_insert_mode(&mut self, render: &RenderContext, keycode: Keycode, ctrl: bool) {
        match keycode {
            Keycode::Backspace => {
                if self.cursor_x != 0 || self.cursor_y != 0 {
                    self.move_cursor(render, -1, 0);
                    let pos = self.cursor_position_in_buffer();
                    self.buffer.remove(pos);
                }
            },
            Keycode::Return => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert("\n", pos);
                self.move_cursor(render, 0, 1);
                self.cursor_x = 0;
            },
            Keycode::Left => self.move_cursor(render, -1, 0),
            Keycode::Right => self.move_cursor(render, 1, 0),
            Keycode::Up => self.move_cursor(render, 0, -1),
            Keycode::Down => self.move_cursor(render, 0, 1),
            Keycode::O if ctrl => {
                let result = nfd::open_file_dialog(None, None)
                    .unwrap_or_else(panic_with_dialog);

                if let nfd::Response::Okay(file_path) = result {
                    self.editing_file_path = file_path.clone();
                    let t = std::fs::read_to_string(file_path)
                        .unwrap_or_else(|_| "".to_string());
                    self.buffer = buffer::Buffer::from(&t);
                }
            },
            Keycode::S if ctrl => {
                if !self.editing_file_path.is_empty() {
                    std::fs::write(
                        &self.editing_file_path,
                        self.buffer.to_string()).unwrap_or(());
                }

            },
            Keycode::Escape => self.mode = Mode::NORMAL,
            _ => {},
        }
    }
}
