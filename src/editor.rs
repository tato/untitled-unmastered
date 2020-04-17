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

    pub cursor_animation_instant: Instant,

    pub matching_input_text: String,
    pub matching_input_modifs: Vec<u32>,
    pub matching_input_timeout: Duration,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: Mode::NORMAL,

            buffer: buffer::Buffer::from(""),
            y_render_offset: 0,

            editing_file_path: String::from(""),

            cursor_animation_instant: Instant::now(),

            matching_input_text: String::new(),
            matching_input_modifs: Vec::new(),
            matching_input_timeout: Duration::from_secs(1),
        }
    }

    pub fn cursor(&self) -> (usize,usize) {
        todo!();
        // (self.actual_cursor_x,self.cursor_y)
    }

    pub fn cursor_position_in_buffer(&self) -> usize {
        todo!();
        // let buffer_string = self.buffer.to_string();
        // let buffer_lines: Vec<&str> = buffer_string
        //     .split('\n')
        //     .take(self.cursor_y + 1)
        //     .collect();
        // let length_before_line = buffer_lines[0..self.cursor_y]
        //     .iter()
        //     .map(|t| t.len() + 1)
        //     .sum::<usize>();
        // let length_inside_line = UnicodeSegmentation::
        //     graphemes(buffer_lines[self.cursor_y], true)
        //     .take(self.actual_cursor_x)
        //     .map(|gc| gc.len())
        //     .sum::<usize>();
        // length_before_line + length_inside_line
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
            binding!(a) => {
                self.buffer.move_cursor_horizontal(1);
                self.mode = Mode::INSERT;
            },
            binding!(h) => self.buffer.move_cursor_horizontal(-1),
            binding!(l) => self.buffer.move_cursor_horizontal(1),
            binding!(k) => self.buffer.move_cursor_vertical(-1, render),
            binding!(j) => self.buffer.move_cursor_vertical(1, render),
            binding!(e) | binding!(E) => {
                todo!();
                // loop {
                //     match self.get(self.cursor_position_in_buffer()) {
                //         None => break,
                //         Some(c) => {
                //             if c == " " || c == "\n" {
                //                 self.buffer.move_cursor_horizontal(-1);
                //                 break;
                //             }
                //             self.buffer.move_cursor_horizontal(1);
                //         },
                //     }
                // }
            }
            binding!(d, d) => println!("dd is nice!"),
            binding!(CTRL+a, b, CTRL+c) => println!("abc is nice!"),
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
                todo!();
                // if self.actual_cursor_x != 0 || self.cursor_y != 0 {
                //     self.move_cursor_horizontal(-1);
                //     let pos = self.cursor_position_in_buffer();
                //     self.buffer.remove(pos);
                // }
            }
            binding!(RETURN) => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert("\n", pos);
                todo!();
                // self.move_cursor_vertical(1, render);
                // self.global_cursor_x = 0;
                // self.actual_cursor_x = 0;
            }
            binding!(LEFT) => self.buffer.move_cursor_horizontal(-1),
            binding!(RIGHT) => self.buffer.move_cursor_horizontal(1),
            binding!(UP) => self.buffer.move_cursor_vertical(-1, render),
            binding!(DOWN) => self.buffer.move_cursor_vertical(1, render),
            binding!(CTRL+o) => {
                let result = nfd::open_file_dialog(None, None)
                    .unwrap_or_else(panic_with_dialog);

                if let nfd::Response::Okay(file_path) = result {
                    self.editing_file_path = file_path.clone();
                    let t = std::fs::read_to_string(file_path)
                        .unwrap_or_else(|_| "".to_string());
                    self.buffer = buffer::Buffer::from(&t);
                }
            },
            binding!(CTRL+s) => {
                if !self.editing_file_path.is_empty() {
                    std::fs::write(
                        &self.editing_file_path,
                        self.buffer.to_string()).unwrap_or(());
                }

            },
            binding!(ESCAPE) | binding!(CTRL+c) => self.mode = Mode::NORMAL,
            _ if is_text_input => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.insert(input, pos);
                self.buffer.move_cursor_horizontal(1);
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

