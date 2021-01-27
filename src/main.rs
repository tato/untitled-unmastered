#![allow(clippy::new_without_default)]

use editor::Editor;
use std::cmp::min;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use unicode_segmentation::UnicodeSegmentation;
use uu_rust_macros::*;

use resource::resource;

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId,
    LineCap, LineJoin, Paint, Path, Renderer, Solidity,
};
use glutin::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

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
pub mod ui;
pub mod wacky;
use wacky::*;

#[derive(Debug, Clone)]
pub struct IO {
    mouse_position: [f32; 2],
    dpi_factor: f64,
    window_dimensions: [u32; 2],
}
impl Default for IO {
    fn default() -> Self {
        Self {
            mouse_position: Default::default(),
            dpi_factor: Default::default(),
            window_dimensions: Default::default(),
        }
    }
}


fn main() {
    let el = EventLoop::new();

    let (renderer, windowed_context) = {
        let wb = WindowBuilder::new()
            .with_inner_size(glutin::dpi::PhysicalSize::new(1000, 600))
            .with_title("UNTITLED!");

        let windowed_context = ContextBuilder::new()
            .with_vsync(false)
            .build_windowed(wb, &el)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = OpenGl::new(|s| windowed_context.get_proc_address(s) as *const _)
            .expect("Cannot create renderer");

        (renderer, windowed_context)
    };

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    let mut ui = ui::UI::new(canvas);

    let start = Instant::now();
    let mut prevt = start;

    let mut dragging = false;

    let mut io: IO = Default::default();
    let mut editor = editor::Editor::new();
    editor.buffer = buffer::Buffer::from(include_str!("main.rs"));

    el.run(move |event, _, control_flow| {
        let window = windowed_context.window();

        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    ..
                } => {
                    io.mouse_position[0] = position.x as f32;
                    io.mouse_position[1] = position.y as f32;
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => match state {
                    ElementState::Pressed => dragging = true,
                    ElementState::Released => dragging = false,
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let dt = (now - prevt).as_secs_f32();
                prevt = now;

                io.dpi_factor = window.scale_factor();
                let size = window.inner_size();
                io.window_dimensions = [size.width, size.height];

                let t = start.elapsed().as_secs_f32();

                let height = size.height as f32;
                let width = size.width as f32;

                ui.run(&mut io, &editor);

                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw()
            }
            _ => (),
        }
    });
}
