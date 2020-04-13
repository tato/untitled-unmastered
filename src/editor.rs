use crate::*;
use crate::render::RenderContext;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
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

    pub matching_input_text: String,
    pub matching_input_modifs: Vec<u32>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: Mode::NORMAL,

            buffer: buffer::Buffer::new(),
            y_render_offset: 0,

            editing_file_path: String::from(""),

            cursor_x: 0,
            cursor_y: 0,
            cursor_animation_instant: Instant::now(),

            matching_input_text: String::new(),
            matching_input_modifs: Vec::new(),
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

    pub fn handle_input(&mut self, render: &RenderContext, text: &str, modifs: u32, _is_text_input: bool) {
        self.matching_input_text += text;
        self.matching_input_modifs.push(modifs);

        match self.mode {
            Mode::NORMAL => self.handle_input_in_normal_mode(render),
            Mode::INSERT => self.handle_input_in_insert_mode(render),
        }
    }

    fn handle_input_in_normal_mode(&mut self, render: &RenderContext) {
        let mut reset_matching_input = true;

        let mit: &str = &self.matching_input_text;
        let mim: &[u32] = &self.matching_input_modifs;
        match (mit, mim) {
            make_binding!(i) => self.mode = Mode::INSERT,
            make_binding!(h) => self.move_cursor(render, -1, 0),
            make_binding!(l) => self.move_cursor(render, 1, 0),
            make_binding!(k) => self.move_cursor(render, 0, -1),
            make_binding!(j) => self.move_cursor(render, 0, 1),
            make_binding!(d, d) => println!("dd is nice!"),
            make_binding!(CTRL|a, b, CTRL|c) => println!("abc is nice!"),
            _ => reset_matching_input = false,
        }

        if reset_matching_input {
            self.matching_input_text = String::new();
            self.matching_input_modifs = Vec::new();
        }
    }
    fn handle_input_in_insert_mode(&mut self, _render: &RenderContext) {
        let mut reset_matching_input = true;

        let mit: &str = &self.matching_input_text;
        let mim: &[u32] = &self.matching_input_modifs;
        match (mit, mim) {
            /*
            todo!() BACKSPACE => {
                if self.cursor_x != 0 || self.cursor_y != 0 {
                    self.move_cursor(render, -1, 0);
                    let pos = self.cursor_position_in_buffer();
                    self.buffer.remove(pos);
                }
            }
            todo!() "\n" => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert("\n", pos);
                self.move_cursor(render, 0, 1);
                self.cursor_x = 0;
            }
            todo!() keys::LEFT => self.move_cursor(render, -1, 0),
            todo!() keys::RIGHT => self.move_cursor(render, 1, 0),
            todo!() keys::UP => self.move_cursor(render, 0, -1),
            todo!() keys::DOWN => self.move_cursor(render, 0, 1),
             */
            make_binding!(CTRL|o) => {
                let result = nfd::open_file_dialog(None, None)
                    .unwrap_or_else(panic_with_dialog);

                if let nfd::Response::Okay(file_path) = result {
                    self.editing_file_path = file_path.clone();
                    let t = std::fs::read_to_string(file_path)
                        .unwrap_or_else(|_| "".to_string());
                    self.buffer = buffer::Buffer::from(&t);
                }
            },
            make_binding!(CTRL|s) => {
                if !self.editing_file_path.is_empty() {
                    std::fs::write(
                        &self.editing_file_path,
                        self.buffer.to_string()).unwrap_or(());
                }

            },
            /*
            todo!() keys::ESCAPE => self.mode = Mode::NORMAL,
            _ if is_text_input => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert(input, pos);
                self.move_cursor(&render, 1, 0);
            },
             */
            _ => reset_matching_input = false,
        }

        if reset_matching_input {
            self.matching_input_text = String::new();
            self.matching_input_modifs = Vec::new();
        }
    }
}

