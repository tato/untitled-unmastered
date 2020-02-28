extern crate sdl2;

use std::time::Instant;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
    let sdl_video = sdl_context.video().unwrap();
    let sdl_ttf = sdl2::ttf::init().unwrap();

    let cousine = sdl_ttf.load_font("./Cousine-Regular.ttf", 14).unwrap();
    assert!(cousine.face_is_fixed_width());

    let any_character_metrics = cousine.find_glyph_metrics('A').unwrap();
    let character_width:  u32 = any_character_metrics.advance as u32;
    let character_height: u32 = cousine.recommended_line_spacing() as u32;

    let characters_wide = 80u32;
    let characters_high = 30u32;

    let foreground_color = Color::RGB(0, 0, 0);
    let background_color = Color::RGB(250, 250, 250);

    let mut editor: Editor = Default::default();
    editor.buffer_lines = String::from("#include <stdio.h>

int main(int argc, char **argv) {
    printf(\"%s\", \"Hello World!\");
    return 0;
}").split('\n').map(String::from).collect();

    let window = sdl_video.window("ttttt...", character_width*characters_wide, character_height*characters_high)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(background_color);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    /*
                    if !editor.text.is_empty() {
                        editor.text = editor.text.chars().take(editor.text.len()-1).collect();
                        editor.move_cursor(-1, 0);
                    }
                    */
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    editor.buffer_lines.push(String::from(""));
                    editor.move_cursor(0, 1);
                    editor.cursor_x = 0;
                },
                Event::TextInput { text, .. } => {
                    if let Some(last_line) = editor.buffer_lines.last_mut() {
                        *last_line = format!("{}{}", last_line, text);
                        editor.move_cursor(1, 0);
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

        canvas.set_draw_color(background_color);
        canvas.clear();

        for (line_index, line) in editor.buffer_lines.iter().enumerate() {
            if line.is_empty() { continue; }
            let text_surface = cousine.render(line).blended(foreground_color).unwrap();
            let texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();

            let target_y: i32 = (line_index as i32)*(character_height as i32);
            let target = Rect::new(0, target_y, text_surface.width(), text_surface.height());
            canvas.copy(&texture, None, Some(target)).unwrap();
        }

        let cursor_color_ms_interval = 500;
        let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
        let cursor_color = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
            foreground_color
        } else {
            background_color
        };

        let mut cursor_surface = Surface::new(character_width, character_height, 
                                              canvas.default_pixel_format()).unwrap();
        let cursor_screen_x = (editor.cursor_x*character_width) as i32;
        let cursor_screen_y = (editor.cursor_y*character_height) as i32;
        let rect = Rect::new(cursor_screen_x, cursor_screen_y, 
                             character_width, character_height);
        cursor_surface.fill_rect(None, cursor_color).unwrap();
        let cursor_texture = texture_creator.create_texture_from_surface(&cursor_surface).unwrap();
        canvas.copy(&cursor_texture, None, Some(rect)).unwrap();

        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000u32 / 30));
    }
}
