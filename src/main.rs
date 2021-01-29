#![allow(clippy::new_without_default)]

use editor::{Editor, DisplayInformation};
use std::cmp::min;
use std::time::{Duration, Instant};
use unicode_segmentation::UnicodeSegmentation;

use femtovg::{renderer::OpenGl, Canvas};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub mod buffer;
pub mod editor;
pub mod ui;

#[derive(Debug, Clone, Copy)]
pub struct Modifiers {
    ctrl: bool,
    shift: bool,
    alt: bool,
    logo: bool,
}
impl Default for Modifiers {
    fn default() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            logo: false,
        }
    }
}
impl From<&glutin::event::ModifiersState> for Modifiers {
    fn from(it: &glutin::event::ModifiersState) -> Self {
        Self {
            ctrl: it.ctrl(),
            shift: it.shift(),
            alt: it.alt(),
            logo: it.logo(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IO {
    mouse_position: [f32; 2],
    dpi_factor: f64,
    window_dimensions: [u32; 2],

    current_modifiers: Modifiers,
}
impl Default for IO {
    fn default() -> Self {
        Self {
            mouse_position: Default::default(),
            dpi_factor: Default::default(),
            window_dimensions: Default::default(),
            current_modifiers: Default::default(),
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        if msgbox::create("uu error", &info.to_string(), msgbox::IconType::Error).is_err() {
            println!("{}", info);
        }
    }));

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

    let canvas = Canvas::new(renderer).expect("Cannot create canvas");
    let mut ui = ui::UI::new(canvas);

    let start = Instant::now();
    let mut prevt = start;

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
                WindowEvent::ReceivedCharacter(c) => {
                    let cheight = ui.character_height();
                    let window_height_in_characters = (io.window_dimensions[1] / cheight) as usize;
                    let info = DisplayInformation{ window_height_in_characters };
                    editor.handle_input(&c.to_string(), io.current_modifiers, true, &info);
                }
                WindowEvent::ModifiersChanged(modifs) => {
                    io.current_modifiers = modifs.into();
                }
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    ..
                } => {
                    io.mouse_position[0] = position.x as f32;
                    io.mouse_position[1] = position.y as f32;
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let _dt = (now - prevt).as_secs_f32();
                prevt = now;

                io.dpi_factor = window.scale_factor();
                let size = window.inner_size();
                io.window_dimensions = [size.width, size.height];

                ui.run(&mut io, &editor);

                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}
