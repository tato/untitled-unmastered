use crate::*;

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
    pub matching_input_modifs: Vec<Modifiers>,
    pub matching_input_timeout: Duration,
}

pub struct DisplayInformation {
    pub window_height_in_characters: usize,
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

    pub fn move_cursor_horizontal(&mut self, x: i64) {
        self.buffer.move_cursor_horizontal(x);
        self.cursor_animation_instant = Instant::now();
    }

    pub fn move_cursor_vertical(&mut self, y: i64, info: &DisplayInformation) {
        self.buffer.move_cursor_vertical(y);
        let (_, cursor_y) = self.buffer.cursor();

        if y > 0 && cursor_y > self.y_render_offset + info.window_height_in_characters - 7 {
            self.y_render_offset += y as usize;
        }
        if y < 0 && cursor_y < self.y_render_offset + 5 && (self.y_render_offset as i64) + y >= 0 {
            self.y_render_offset -= (-y) as usize;
        }

        self.cursor_animation_instant = Instant::now();
    }

    pub fn handle_input(
        &mut self,
        text: &str,
        modifs: Modifiers,
        is_text_input: bool,
        info: &DisplayInformation,
    ) {
        self.matching_input_text += text;
        self.matching_input_modifs.push(modifs);
        self.matching_input_timeout = Duration::from_secs(1);

        match self.mode {
            Mode::NORMAL => self.handle_input_in_normal_mode(info),
            Mode::INSERT => self.handle_input_in_insert_mode(text, is_text_input, info),
        }
    }

    fn handle_input_in_normal_mode(&mut self, info: &DisplayInformation) {
        let mut reset_matching_input = true;

        let mit: &str = &self.matching_input_text;
        let mim: &[Modifiers] = &self.matching_input_modifs;
        match (mit, mim) {
            ("i", _) => self.mode = Mode::INSERT,
            ("a", _) => {
                self.move_cursor_horizontal(1);
                self.mode = Mode::INSERT;
            }
            ("h", _) => self.move_cursor_horizontal(-1),
            ("l", _) => self.move_cursor_horizontal(1),
            ("k", _) => self.move_cursor_vertical(-1, info),
            ("j", _) => self.move_cursor_vertical(1, info),
            ("e", _) => loop {
                let c = self.buffer.get_under_cursor();
                if c == " " || c == "\n" {
                    self.move_cursor_horizontal(-1);
                    break;
                }
                self.move_cursor_horizontal(1);
            },
            ("dd", _) => println!("dd is nice!"),
            // binding!(CTRL + a, b, CTRL + c) => println!("abc is nice!"),
            _ => reset_matching_input = false,
        }

        if reset_matching_input {
            self.matching_input_text = String::new();
            self.matching_input_modifs = Vec::new();
        }
    }
    fn handle_input_in_insert_mode(
        &mut self,
        _input: &str,
        _is_text_input: bool,
        _info: &DisplayInformation,
    ) {
        let mut reset_matching_input = true;

        let mit: &str = &self.matching_input_text;
        let mim: &[Modifiers] = &self.matching_input_modifs;
        match (mit, mim) {
            ("a", _) => { /* dummy */ }
            // binding!(BACKSPACE) => {
            //     self.buffer.delete_under_cursor();
            // }
            // binding!(RETURN) => {
            //     self.buffer.insert_under_cursor("\n");
            // }
            // binding!(LEFT) => self.move_cursor_horizontal(-1),
            // binding!(RIGHT) => self.move_cursor_horizontal(1),
            // binding!(UP) => self.move_cursor_vertical(-1/*, render*/),
            // binding!(DOWN) => self.move_cursor_vertical(1/*, render*/),
            // binding!(CTRL + o) => {
            //     let result = nfd::open_file_dialog(None, None).unwrap_or_else(panic_with_dialog);

            //     if let nfd::Response::Okay(file_path) = result {
            //         self.editing_file_path = file_path.clone();
            //         let t = std::fs::read_to_string(file_path).unwrap_or_else(|_| "".to_string());
            //         self.buffer = buffer::Buffer::from(&t);
            //     }
            // }
            // binding!(CTRL + s) => {
            //     if !self.editing_file_path.is_empty() {
            //         std::fs::write(&self.editing_file_path, self.buffer.to_string()).unwrap_or(());
            //     }
            // }
            // binding!(ESCAPE) | binding!(CTRL + c) => self.mode = Mode::NORMAL,
            // _ if is_text_input => {
            //     self.buffer.insert_under_cursor(input);
            // }
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
