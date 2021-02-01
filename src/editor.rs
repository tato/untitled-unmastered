use crate::*;
use once_cell::sync::Lazy;

type EditorCommand = fn(&mut Editor);

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

    current_display_info: DisplayInformation,

    pub matching_input: String,
    pub matching_input_timeout: Duration,
}

#[derive(Debug, Clone)]
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

            current_display_info: DisplayInformation {
                window_height_in_characters: 0,
            },

            matching_input: "".into(),
            matching_input_timeout: Duration::from_secs(1),
        }
    }

    pub fn move_cursor_horizontal(&mut self, x: i64) {
        self.buffer.move_cursor_horizontal(x);
        self.cursor_animation_instant = Instant::now();
    }

    pub fn move_cursor_vertical(&mut self, y: i64) {
        self.buffer.move_cursor_vertical(y);
        let (_, cursor_y) = self.buffer.cursor();

        if y > 0
            && cursor_y
                > self.y_render_offset + self.current_display_info.window_height_in_characters - 7
        {
            self.y_render_offset += y as usize;
        }
        if y < 0 && cursor_y < self.y_render_offset + 5 && (self.y_render_offset as i64) + y >= 0 {
            self.y_render_offset -= (-y) as usize;
        }

        self.cursor_animation_instant = Instant::now();
    }

    pub fn handle_input(&mut self, text: &str, is_text_input: bool, info: &DisplayInformation) {
        self.current_display_info = info.clone();

        self.matching_input.push_str(text);
        self.matching_input_timeout = Duration::from_secs(1);

        match self.mode {
            Mode::NORMAL => self.handle_input_in_normal_mode(),
            Mode::INSERT => self.handle_input_in_insert_mode(text, is_text_input),
        }
    }

    fn handle_input_in_normal_mode(&mut self) {
        let mut reset_matching_input = false;

        for (keys, func) in Lazy::force(&NORMAL_BINDINGS) {
            if keys == &self.matching_input {
                func(self);
                reset_matching_input = true;
                break;
            }
        }

        if reset_matching_input {
            self.matching_input = "".into();
        }
    }
    fn handle_input_in_insert_mode(&mut self, input: &str, is_text_input: bool) {
        let mut reset_matching_input = false;

        for (keys, func) in Lazy::force(&INSERT_BINDINGS) {
            if keys == &self.matching_input {
                func(self);
                reset_matching_input = true;
                break;
            }
        }

        if is_text_input && !reset_matching_input {
            self.buffer.insert_under_cursor(input);
            reset_matching_input = true;
        }

        if reset_matching_input {
            self.matching_input = "".into();
        }
    }

    pub fn fade_matching_input(&mut self, delta: Duration) {
        if delta > self.matching_input_timeout {
            self.matching_input = "".into();
        } else {
            self.matching_input_timeout -= delta;
        }
    }
}

// TODO: remove once_cell and inline this stuff into like a match statement lol
static NORMAL_BINDINGS: Lazy<Vec<(&'static str, EditorCommand)>> = Lazy::new(|| {
    vec![
        ("i", |editor| editor.mode = Mode::INSERT),
        ("a", |editor| {
            editor.move_cursor_horizontal(1);
            editor.mode = Mode::INSERT;
        }),
        ("h", |editor| editor.move_cursor_horizontal(-1)),
        ("l", |editor| editor.move_cursor_horizontal(1)),
        ("k", |editor| editor.move_cursor_vertical(-1)),
        ("j", |editor| editor.move_cursor_vertical(1)),
        ("e", |editor| loop {
            let c = editor.buffer.get_under_cursor();
            if c == " " || c == "\n" {
                editor.move_cursor_horizontal(-1);
                break;
            }
            editor.move_cursor_horizontal(1);
        }),
        ("dd", |_editor| println!("dd is nice!")),
        (" s", |editor| {
            if !editor.editing_file_path.is_empty() {
                std::fs::write(&editor.editing_file_path, editor.buffer.as_string()).unwrap_or(());
            }
        }),
        ("  ", |editor| {
            let result = nfd::open_file_dialog(None, None).unwrap();

            if let nfd::Response::Okay(file_path) = result {
                editor.editing_file_path = file_path.clone();
                let t = std::fs::read_to_string(file_path).unwrap_or_else(|_| "".to_string());
                editor.buffer = buffer::Buffer::from(&t);
            }
        }),
    ]
});

static INSERT_BINDINGS: Lazy<Vec<(&'static str, EditorCommand)>> = Lazy::new(|| {
    vec![
        ("\x08", |editor| editor.buffer.delete_under_cursor()),
        ("\x1b", |editor| editor.mode = Mode::NORMAL),
        ("\n", |editor| editor.buffer.insert_under_cursor("\n")),
    ]
});
