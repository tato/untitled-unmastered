extern crate sdl2;

use std::time::Instant;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod buffer;
mod font;
mod render;

use buffer::*;
use render::RenderContext;


struct Editor {
    buffer: Buffer,

    cursor_x: usize,
    cursor_y: usize,
    cursor_animation_instant: Instant,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor_x: 0,
            cursor_y: 0,
            cursor_animation_instant: Instant::now(),
        }
    }
}
impl Editor {
    pub fn move_cursor(&mut self, x: i32, y: i32) {
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

        self.cursor_animation_instant = Instant::now();
    }
    pub fn cursor_position_in_buffer(&self) -> usize {
        let buffer_string = self.buffer.to_string();
        self.cursor_x + buffer_string
            .split('\n')
            .take(self.cursor_y)
            .map(|t| t.len() + 1)
            .sum::<usize>()
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_ttf = sdl2::ttf::init().unwrap();

    let raw_cousine = sdl_ttf.load_font("./Cousine-Regular.ttf", 18).unwrap();
    let mut cousine = font::Font::from(raw_cousine);

    let mut render = RenderContext::from(sdl_context.video().unwrap());

    let character_width = cousine.character_width;
    let character_height = cousine.character_height;

    let foreground_color = Color::RGB(0, 0, 0);
    let background_color = Color::RGB(250, 250, 250);

    let mut editor: Editor = Default::default();
    editor.buffer = Buffer::from(include_str!("main.rs"));


    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    let pos = editor.cursor_position_in_buffer();
                    editor.buffer.remove(pos);
                    editor.move_cursor(-1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    let pos = editor.cursor_position_in_buffer();
                    editor.buffer.insert("\n", pos);
                    editor.move_cursor(0, 1);
                    editor.cursor_x = 0;
                },
                Event::TextInput { text, .. } => {
                    let pos = editor.cursor_position_in_buffer();
                    editor.buffer.insert(&text, pos);
                    editor.move_cursor(1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    editor.move_cursor(-1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    editor.move_cursor(1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    editor.move_cursor(0, -1);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    editor.move_cursor(0, 1);
                },
                _ => { }
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

        let cursor_screen_x = ((editor.cursor_x as u32)*character_width) as i32;
        let cursor_screen_y = ((editor.cursor_y as u32)*character_height) as i32;
        let cursor_target = Rect::new(cursor_screen_x, cursor_screen_y, character_width, character_height);
        render.fill_rect(cursor_target, cursor_color);

        for (line_index, line) in editor.buffer.to_string().split('\n').enumerate() {
            for (ch_index, ch) in line.chars().enumerate() {
                let character_color = if line_index == editor.cursor_y as usize && ch_index == editor.cursor_x as usize {
                    if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
                        background_color
                    } else {
                        foreground_color
                    }
                } else {
                    foreground_color
                };
                
                let ch_surface = cousine.get_surface_for(ch, character_color);

                let target_x: i32 = (ch_index as i32)*(character_width as i32);
                let target_y: i32 = (line_index as i32)*(character_height as i32);
                let target = Rect::new(target_x, target_y, ch_surface.width(), ch_surface.height());


                render.copy_surface(ch_surface, target);
            }
        }


        render.finish_frame();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000u32 / 30));
    }
}
