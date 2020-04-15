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

    global_cursor_x: usize,
    actual_cursor_x: usize,
    cursor_y: usize,
    pub cursor_animation_instant: Instant,

    pub matching_input_text: String,
    pub matching_input_modifs: Vec<u32>,
    pub matching_input_timeout: Duration,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: Mode::NORMAL,

            buffer: buffer::Buffer::new(),
            y_render_offset: 0,

            editing_file_path: String::from(""),

            global_cursor_x: 0,
            actual_cursor_x: 0,
            cursor_y: 0,
            cursor_animation_instant: Instant::now(),

            matching_input_text: String::new(),
            matching_input_modifs: Vec::new(),
            matching_input_timeout: Duration::from_secs(1),
        }
    }
    pub fn move_cursor(&mut self, render: &RenderContext, x: i32, y: i32) {
        let buffer_string = self.buffer.to_string();
        let buffer_lines: Vec<_> = buffer_string
            .split('\n')
            .map(|it| UnicodeSegmentation::graphemes(it, true).collect::<Vec<_>>())
            .collect();

        if self.global_cursor_x == 0 && x < 0 {
            if self.cursor_y != 0 {
                self.cursor_y -= 1;
                self.global_cursor_x = buffer_lines.get(self.cursor_y)
                    .unwrap_or(&Vec::new()).len();
            }
        } else {
            self.global_cursor_x = ((self.global_cursor_x as i32) + x) as usize;
        }

        if self.cursor_y == 0 && y < 0 {
            self.cursor_y = 0;
        } else {
            self.cursor_y = ((self.cursor_y as i32) + y) as usize;
        }

        if self.cursor_y >= buffer_lines.len() {
            self.cursor_y = buffer_lines.len() - 1;
        }
        if self.global_cursor_x >= buffer_lines[self.cursor_y].len() {
            self.actual_cursor_x = buffer_lines[self.cursor_y].len();
        } else {
            self.actual_cursor_x = self.global_cursor_x;
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

    pub fn cursor(&self) -> (usize,usize) {
        (self.actual_cursor_x,self.cursor_y)
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
            .take(self.actual_cursor_x)
            .map(|gc| gc.len())
            .sum::<usize>();
        length_before_line + length_inside_line
    }

    pub fn handle_input(&mut self, render: &RenderContext, text: &str, modifs: u32, is_text_input: bool) {
        self.matching_input_text += text;
        self.matching_input_modifs.push(modifs);
        self.matching_input_timeout = Duration::from_secs(1);

        match self.mode {
            Mode::NORMAL => self.handle_input_in_normal_mode(render),
            Mode::INSERT => self.handle_input_in_insert_mode(render, text, is_text_input),
        }
    }

    fn handle_input_in_normal_mode(&mut self, render: &RenderContext) {
        let mut reset_matching_input = true;

        let mit: &str = &self.matching_input_text;
        let mim: &[u32] = &self.matching_input_modifs;
        match (mit, mim) {
            binding!(i) => self.mode = Mode::INSERT,
            binding!(h) => self.move_cursor(render, -1, 0),
            binding!(l) => self.move_cursor(render, 1, 0),
            binding!(k) => self.move_cursor(render, 0, -1),
            binding!(j) => self.move_cursor(render, 0, 1),
            binding!(d, d) => println!("dd is nice!"),
            binding!(CTRL|a, b, CTRL|c) => println!("abc is nice!"),
            _ => reset_matching_input = false,
        }

        if reset_matching_input {
            self.matching_input_text = String::new();
            self.matching_input_modifs = Vec::new();
        }
    }
    fn handle_input_in_insert_mode(&mut self, render: &RenderContext, input: &str, is_text_input: bool) {
        let mut reset_matching_input = true;

        let mit: &str = &self.matching_input_text;
        let mim: &[u32] = &self.matching_input_modifs;
        match (mit, mim) {
            binding!(BACKSPACE) => {
                if self.actual_cursor_x != 0 || self.cursor_y != 0 {
                    self.move_cursor(render, -1, 0);
                    let pos = self.cursor_position_in_buffer();
                    self.buffer.remove(pos);
                }
            }
            binding!(RETURN) => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert("\n", pos);
                self.move_cursor(render, 0, 1);
                self.global_cursor_x = 0;
                self.actual_cursor_x = 0;
            }
            binding!(LEFT) => self.move_cursor(render, -1, 0),
            binding!(RIGHT) => self.move_cursor(render, 1, 0),
            binding!(UP) => self.move_cursor(render, 0, -1),
            binding!(DOWN) => self.move_cursor(render, 0, 1),
            binding!(CTRL|o) => {
                let result = nfd::open_file_dialog(None, None)
                    .unwrap_or_else(panic_with_dialog);

                if let nfd::Response::Okay(file_path) = result {
                    self.editing_file_path = file_path.clone();
                    let t = std::fs::read_to_string(file_path)
                        .unwrap_or_else(|_| "".to_string());
                    self.buffer = buffer::Buffer::from(&t);
                }
            },
            binding!(CTRL|s) => {
                if !self.editing_file_path.is_empty() {
                    std::fs::write(
                        &self.editing_file_path,
                        self.buffer.to_string()).unwrap_or(());
                }

            },
            binding!(ESCAPE) | binding!(j, k) => self.mode = Mode::NORMAL,
            _ if is_text_input => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert(input, pos);
                self.move_cursor(&render, 1, 0);
            },
            _ => reset_matching_input = false,
        }

        if reset_matching_input {
            self.matching_input_text = String::new();
            self.matching_input_modifs = Vec::new();
        }
    }

    pub fn fade_matching_input(&mut self, delta: Duration) {
        if delta > self.matching_input_timeout {
            self.matching_input_text = String::new();
            self.matching_input_modifs = Vec::new();
        } else {
            self.matching_input_timeout -= delta;
        }
    }
}

