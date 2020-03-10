extern crate sdl2;
extern crate nfd;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::*;
use std::time::Instant;
use std::cmp::min;

mod buffer;
mod render;

use buffer::*;
use render::RenderContext;
use uu::panic_with_dialog;

enum Mode {
    NORMAL,
    INSERT,
}

struct Editor {
    mode: Mode, 

    buffer: Buffer,
    y_render_offset: usize,

    editing_file_path: String,

    cursor_x: usize,
    cursor_y: usize,
    cursor_animation_instant: Instant,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            mode: Mode::INSERT,

            buffer: Buffer::new(),
            y_render_offset: 0,

            editing_file_path: String::from(""),

            cursor_x: 0,
            cursor_y: 0,
            cursor_animation_instant: Instant::now(),
        }
    }
    pub fn move_cursor(&mut self, render: &RenderContext, x: i32, y: i32) {
        let buffer_string = self.buffer.to_string();
        let buffer_lines: Vec<&str> = buffer_string.split('\n').collect();

        if self.cursor_x == 0 && x < 0 {
            if self.cursor_y != 0 {
                self.cursor_y -= 1;
                self.cursor_x = buffer_lines.get(self.cursor_y).unwrap_or(&"").len();
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
        self.cursor_x
            + buffer_string
                .split('\n')
                .take(self.cursor_y)
                .map(|t| t.chars().count() + 1)
                .sum::<usize>()
    }

    pub fn handle_keys(&mut self, render: &RenderContext, keycode: keyboard::Keycode, ctrl: bool) {
        match self.mode {
            Mode::NORMAL => self.handle_keys_in_normal_mode(render, keycode, ctrl),
            Mode::INSERT => self.handle_keys_in_insert_mode(render, keycode, ctrl),
        }
    }

    fn handle_keys_in_normal_mode(&mut self, render: &RenderContext, keycode: keyboard::Keycode, _ctrl: bool) {
        match keycode {
            Keycode::I => self.mode = Mode::INSERT,
            Keycode::H => self.move_cursor(render, -1, 0),
            Keycode::L => self.move_cursor(render, 1, 0),
            Keycode::K => self.move_cursor(render, 0, -1),
            Keycode::J => self.move_cursor(render, 0, 1),
            _ => {},
        }
    }
    fn handle_keys_in_insert_mode(&mut self, render: &RenderContext, keycode: keyboard::Keycode, ctrl: bool) {
        match keycode {
            Keycode::Backspace => {
                let pos = self.cursor_position_in_buffer();
                self.buffer.remove(pos);
                self.move_cursor(render, -1, 0);
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

fn main() {
    let sdl_context = sdl2::init().unwrap_or_else(panic_with_dialog);
    let video_context = sdl_context.video().unwrap_or_else(panic_with_dialog);
    let ttf_context = sdl2::ttf::init().unwrap_or_else(panic_with_dialog);
    let mut render = RenderContext::new(&video_context, &ttf_context);

    let character_width = render.character_width;
    let character_height = render.character_height;

    let foreground_color = Color::RGB(0, 0, 0);
    let background_color = Color::RGB(250, 250, 250);

    let mut editor = Editor::new();
    editor.buffer = Buffer::from(include_str!("main.rs"));

    let mut event_pump = sdl_context
        .event_pump()
        .unwrap_or_else(panic_with_dialog);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), keymod, .. } => {
                    let ctrl = keymod.contains(keyboard::Mod::LCTRLMOD)
                        || keymod.contains(keyboard::Mod::RCTRLMOD);
                    editor.handle_keys(&render, keycode, ctrl);
                }
                Event::TextInput { text, .. } => {
                    if let Mode::INSERT = editor.mode {
                        let pos = editor.cursor_position_in_buffer();
                        editor.buffer.insert(&text, pos);
                        editor.move_cursor(&render, 1, 0);
                    }
                }

                _ => {}
            }
        }

        render.start_frame(background_color);

        let cursor_color_ms_interval = 500;
        let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
        let cursor_color = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
            foreground_color
        } else {
            background_color
        };

        let cursor_screen_x = ((editor.cursor_x as u32) * character_width) as i32;
        let cursor_screen_y = (((editor.cursor_y - editor.y_render_offset) as u32) * character_height) as i32;
        let cursor_target = Rect::new(
            cursor_screen_x, cursor_screen_y,
            character_width, character_height,
        );
        render.fill_rect(cursor_target, cursor_color).unwrap_or(());

        let _window_width_in_characters = render.width() / character_width;
        let window_height_in_characters = render.height() / character_height;

        let status_line_y = 
            ((window_height_in_characters - 2) * character_height) as i32;
        let status_line_rect = Rect::new(
            0, status_line_y, 
            render.width(), character_height
        );
        render.fill_rect(status_line_rect, foreground_color).unwrap_or(());

        let status_text = format!(" {} > {} <",
                                  editor.cursor_y,
                                  editor.editing_file_path);
        for (ci_usize, c) in status_text.chars().enumerate() {
            let ci: i32 = ci_usize as i32;
            let cw: i32 = character_width as i32;

            let target_x = ci * cw;
            let target_y = status_line_y;
            render
                .draw_character(c, background_color, target_x, target_y)
                .unwrap_or(());
        }

        for (line_index, line) in editor
            .buffer
            .to_string()
            .split('\n')
            .skip(editor.y_render_offset)
            .take((window_height_in_characters - 2) as usize)
            .enumerate()
        {
            for (ch_index, ch) in line.chars().enumerate() {
                let character_color = if line_index == editor.cursor_y as usize
                    && ch_index == editor.cursor_x as usize
                {
                    if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
                        background_color
                    } else {
                        foreground_color
                    }
                } else {
                    foreground_color
                };

                let target_x: i32 = (ch_index as i32) * (character_width as i32);
                let target_y: i32 = (line_index as i32) * (character_height as i32);
                render
                    .draw_character(ch, character_color, target_x, target_y)
                    .unwrap_or(());
            }
        }

        render.finish_frame();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000u32 / 30));
    }
}
