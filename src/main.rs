extern crate sdl2;

use std::time::Instant;
use std::cmp::min;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod font;
mod render;

use render::RenderContext;


struct Editor {
    buffer_lines: Vec<String>,

    cursor_x: u32,
    cursor_y: u32,
    cursor_animation_instant: Instant,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            buffer_lines: vec![ String::from("") ],
            cursor_x: 0,
            cursor_y: 0,
            cursor_animation_instant: Instant::now(),
        }
    }
}
impl Editor {
    pub fn move_cursor(&mut self, x: i32, y: i32) {
        if self.cursor_x == 0 && x < 0 {
            if self.cursor_y != 0 {
                self.cursor_y -= 1;
                self.cursor_x = self.buffer_lines.get(self.cursor_y as usize).unwrap_or(&String::from("")).len() as u32;
            }
        } else {
            self.cursor_x = ((self.cursor_x as i32) + x) as u32;
        }

        if self.cursor_y == 0 && y < 0 {
            self.cursor_y = 0;
        } else {
            self.cursor_y = ((self.cursor_y as i32) + y) as u32;
        }
        self.cursor_animation_instant = Instant::now();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_ttf = sdl2::ttf::init().unwrap();

    let raw_cousine = sdl_ttf.load_font("./Cousine-Regular.ttf", 14).unwrap();
    let mut cousine = font::Font::from(raw_cousine);

    let mut render = RenderContext::from(sdl_context.video().unwrap());

    let character_width = cousine.character_width;
    let character_height = cousine.character_height;

    let characters_wide = 80u32;
    let characters_high = 30u32;

    let initial_window_width = character_width*characters_wide;
    let initial_window_height = character_height*characters_high;
    render.set_window_dimensions(initial_window_width, initial_window_height);

    let foreground_color = Color::RGB(0, 0, 0);
    let background_color = Color::RGB(250, 250, 250);

    let mut editor: Editor = Default::default();
    editor.buffer_lines = String::from("#include <stdio.h>

int main(int argc, char **argv) {
    printf(\"%s\", \"Hello World!\");
    return 0;
}").split('\n').map(String::from).collect();


    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    let actual_y = min(editor.cursor_y as usize, editor.buffer_lines.len() - 1);

                    let mut maybe_cursor_x = -1i32;

                    let bl = &mut editor.buffer_lines;
                    let actual_x = min(editor.cursor_x as usize, bl[actual_y].len());
                    if actual_x == 0 {
                        let deleted_line = bl.remove(actual_y);
                        if let Some(prev_line) = bl.get_mut(actual_y - 1) {
                            maybe_cursor_x = prev_line.len() as i32;
                            prev_line.push_str(&deleted_line);
                        }
                    } else {
                        bl[actual_y].remove(actual_x);
                    }
                    editor.move_cursor(-1, 0);
                    if maybe_cursor_x >= 0 {
                        editor.cursor_x = maybe_cursor_x as u32;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    let actual_y = min(editor.cursor_y as usize, editor.buffer_lines.len() - 1);

                    let line = &mut editor.buffer_lines[actual_y];
                    let actual_x = min(editor.cursor_x as usize, line.len());
                    let new_line = line.split_off(actual_x);

                    editor.buffer_lines.insert(actual_y + 1, new_line);

                    editor.move_cursor(0, 1);
                    editor.cursor_x = 0;
                },
                Event::TextInput { text, .. } => {
                    if let Some(line) = editor.buffer_lines.get_mut(editor.cursor_y as usize) {
                        let actual_x = min(editor.cursor_x as usize, line.len());
                        let mut vec_line = line.chars().collect::<Vec<_>>();
                        vec_line.splice(actual_x..actual_x, text.chars().collect::<Vec<char>>());
                        *line = vec_line.iter().collect();
                        editor.move_cursor(text.len() as i32, 0);
                    }
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

        for (line_index, line) in editor.buffer_lines.iter().enumerate() {
            for (ch_index, ch) in line.chars().enumerate() {
                let ch_surface = cousine.get_surface_for(ch, foreground_color);

                let target_x: i32 = (ch_index as i32)*(character_width as i32);
                let target_y: i32 = (line_index as i32)*(character_height as i32);
                let target = Rect::new(target_x, target_y, ch_surface.width(), ch_surface.height());

                render.copy_surface(ch_surface, target);
            }
        }

        let cursor_color_ms_interval = 500;
        let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
        let cursor_color = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
            foreground_color
        } else {
            background_color
        };

        let cursor_screen_x = (editor.cursor_x*character_width) as i32;
        let cursor_screen_y = (editor.cursor_y*character_height) as i32;
        let target = Rect::new(cursor_screen_x, cursor_screen_y, character_width, character_height);
        render.fill_rect(target, cursor_color);

        render.finish_frame();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000u32 / 30));
    }
}
