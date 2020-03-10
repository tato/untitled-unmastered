extern crate sdl2;
extern crate nfd;

use sdl2::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;
use std::cmp::min;

pub fn panic_with_dialog<T>(m: impl std::fmt::Display) -> T {
    sdl2::messagebox::show_simple_message_box(
        sdl2::messagebox::MessageBoxFlag::ERROR, 
        "uu error", &m.to_string(), None).expect(&m.to_string());
    panic!("{}", m);
}

pub mod buffer;
pub mod render;
pub mod editor;
use render::RenderContext;

fn main() {
    let sdl_context = sdl2::init().unwrap_or_else(panic_with_dialog);
    let video_context = sdl_context.video().unwrap_or_else(panic_with_dialog);
    let ttf_context = sdl2::ttf::init().unwrap_or_else(panic_with_dialog);
    let mut render = RenderContext::new(&video_context, &ttf_context);

    let character_width = render.character_width;
    let character_height = render.character_height;

    let foreground_color = Color::RGB(0, 0, 0);
    let background_color = Color::RGB(250, 250, 250);

    let mut editor = editor::Editor::new();
    editor.buffer = buffer::Buffer::from(include_str!("main.rs"));

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
                    if let editor::Mode::INSERT = editor.mode {
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
