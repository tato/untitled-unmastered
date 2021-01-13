#![allow(clippy::new_without_default)]

use editor::Editor;
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::{ClipboardBackend, Condition, Context, FontConfig, FontGlyphRanges, FontSource, ImStr, ImString, Ui, Window, im_str};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use clipboard::{self, ClipboardContext, ClipboardProvider};
use unicode_segmentation::UnicodeSegmentation;
use uu_rust_macros::*;
use std::time::{Duration,Instant};
use std::cmp::min;

pub fn panic_with_dialog<Any>(m: impl std::fmt::Display) -> Any {
    // let owned = m.to_string();
    // sdl2::messagebox::show_simple_message_box(
    //     sdl2::messagebox::MessageBoxFlag::ERROR,
    //     "uu error", &owned, None).expect(&owned);
    panic!("{}", m);
}

// pub fn get_utf8_for_keycode(keycode: Keycode) -> Option<&'static str> {
//     match keycode {
//         Keycode::Backspace => Some(BACKSPACE!()),
//         Keycode::Escape => Some(ESCAPE!()),
//         Keycode::Return => Some(RETURN!()),
//         Keycode::Left => Some(LEFT!()),
//         Keycode::Right => Some(RIGHT!()),
//         Keycode::Up => Some(UP!()),
//         Keycode::Down => Some(DOWN!()),
//         _ => None,
//     }
// }

pub mod buffer;
pub mod editor;
pub mod render;

pub struct ClipboardSupport(ClipboardContext);
impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<ImString> {
        self.0.get_contents().ok().map(|text| text.into())
    }
    fn set(&mut self, text: &ImStr) {
        let _ = self.0.set_contents(text.to_str().to_owned());
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title("uu".to_owned())
        .with_maximized(true);
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    if let Ok(context) = ClipboardContext::new() {
        imgui.set_clipboard_backend(Box::new(ClipboardSupport(context)));
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("Cousine-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig::default()),
        },
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");
    
    // let character_width = render.character_width;
    // let character_height = render.character_height;

    // let foreground_color = Color::RGB(0, 0, 0);
    // let background_color = Color::RGB(250, 250, 250);

    let mut editor = editor::Editor::new();
    editor.buffer = buffer::Buffer::from(include_str!("main.rs"));

    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::MainEventsCleared => {
            let gl_window = display.gl_window();
            platform
                .prepare_frame(imgui.io_mut(), gl_window.window())
                .expect("Failed to prepare frame");
            gl_window.window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            let mut ui = imgui.frame();

            let mut run = true;
            run_ui(&mut run, &mut ui, &mut editor);
            if !run {
                *control_flow = ControlFlow::Exit;
            }

            let gl_window = display.gl_window();
            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(&ui, gl_window.window());
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        event => {
            let gl_window = display.gl_window();
            platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
        }
    })
}

fn run_ui(_run: &mut bool, ui: &mut Ui, _editor: &mut Editor) {
    Window::new(im_str!("Hello world"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
    
    //     let cursor_color_ms_interval = 500;
    //     let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
    //     let cursor_color = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
    //         foreground_color
    //     } else {
    //         background_color
    //     };

    //     let cursor_width = match editor.mode {
    //         editor::Mode::INSERT => character_width / 4,
    //         _ => character_width,
    //     };

    //     let cursor = editor.buffer.cursor();

    //     let cursor_screen_x = ((cursor.0 as u32) * character_width) as i32;
    //     let cursor_screen_y = (((cursor.1 - editor.y_render_offset) as u32) * character_height) as i32;
    //     let cursor_target = Rect::new(
    //         cursor_screen_x, cursor_screen_y,
    //         cursor_width, character_height,
    //     );
    //     render.fill_rect(cursor_target, cursor_color).unwrap_or(());

    //     let _window_width_in_characters = render.width() / character_width;
    //     let window_height_in_characters = render.height() / character_height;

    //     let status_line_y =
    //         ((window_height_in_characters - 2) * character_height) as i32;
    //     let status_line_rect = Rect::new(
    //         0, status_line_y,
    //         render.width(), character_height
    //     );
    //     render.fill_rect(status_line_rect, foreground_color).unwrap_or(());

    //     let status_text = format!(" {} > {} < $ {} {:?}",
    //                               cursor.1,
    //                               editor.editing_file_path,
    //                               editor.matching_input_text,
    //                               editor.matching_input_timeout);

    //     let gcs = UnicodeSegmentation::graphemes(status_text.as_str(), true);
    //     for (ci_usize, c) in gcs.enumerate() {
    //         let ci: i32 = ci_usize as i32;
    //         let cw: i32 = character_width as i32;

    //         let target_x = ci * cw;
    //         let target_y = status_line_y;
    //         render
    //             .draw_character(c, background_color, target_x, target_y)
    //             .unwrap_or(());
    //     }

    //     for (line_index, line) in editor
    //         .buffer
    //         .to_string()
    //         .split('\n')
    //         .skip(editor.y_render_offset)
    //         .take((window_height_in_characters - 2) as usize)
    //         .enumerate()
    //     {
    //         let gcs = UnicodeSegmentation::graphemes(line, true);
    //         for (ch_index, c) in gcs.enumerate() {
    //             let character_color = if line_index == cursor.1 as usize
    //                 && ch_index == cursor.0 as usize
    //                 && editor.mode != editor::Mode::INSERT
    //             {
    //                 if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
    //                     background_color
    //                 } else {
    //                     foreground_color
    //                 }
    //             } else {
    //                 foreground_color
    //             };

    //             let target_x: i32 = (ch_index as i32) * (character_width as i32);
    //             let target_y: i32 = (line_index as i32) * (character_height as i32);
    //             render
    //                 .draw_character(c, character_color, target_x, target_y)
    //                 .unwrap_or(());
    //         }
    //     }

    //     render.finish_frame();

    //     let frame_duration = previous_frame_instant.elapsed();
    //     editor.fade_matching_input(frame_duration);
    //     previous_frame_instant = Instant::now();
    // }
}