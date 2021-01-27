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
pub mod wacky;
use wacky::*;

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

    let font = canvas
        .add_font_mem(&resource!("src/Cousine-Regular.ttf"))
        .expect("Cannot add font");

    let images = vec![];

    let mut screenshot_image_id = None;

    let start = Instant::now();
    let mut prevt = start;

    let mut mousex = 0.0;
    let mut mousey = 0.0;
    let mut dragging = false;

    let mut perf = PerfGraph::new();

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
                    if dragging {
                        let p0 = canvas
                            .transform()
                            .inversed()
                            .transform_point(mousex, mousey);
                        let p1 = canvas
                            .transform()
                            .inversed()
                            .transform_point(position.x as f32, position.y as f32);

                        canvas.translate(p1.0 - p0.0, p1.1 - p0.1);
                    }

                    mousex = position.x as f32;
                    mousey = position.y as f32;
                }
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    ..
                } => match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_, y) => {
                        let pt = canvas
                            .transform()
                            .inversed()
                            .transform_point(mousex, mousey);
                        canvas.translate(pt.0, pt.1);
                        canvas.scale(1.0 + (y / 10.0), 1.0 + (y / 10.0));
                        canvas.translate(-pt.0, -pt.1);
                    }
                    _ => (),
                },
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => match state {
                    ElementState::Pressed => dragging = true,
                    ElementState::Released => dragging = false,
                },
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::S),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    if let Some(screenshot_image_id) = screenshot_image_id {
                        canvas.delete_image(screenshot_image_id);
                    }

                    if let Ok(image) = canvas.screenshot() {
                        screenshot_image_id = Some(
                            canvas
                                .create_image(image.as_ref(), ImageFlags::empty())
                                .unwrap(),
                        );
                    }
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let dt = (now - prevt).as_secs_f32();
                prevt = now;

                perf.update(dt);

                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                let t = start.elapsed().as_secs_f32();

                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    size.width as u32,
                    size.height as u32,
                    Color::rgbf(0.3, 0.3, 0.32),
                );

                let height = size.height as f32;
                let width = size.width as f32;

                let pt = canvas
                    .transform()
                    .inversed()
                    .transform_point(mousex, mousey);
                let mousex = pt.0;
                let mousey = pt.1;

                draw_graph(&mut canvas, 0.0, height / 2.0, width, height / 2.0, t);

                draw_eyes(
                    &mut canvas,
                    width - 250.0,
                    50.0,
                    150.0,
                    100.0,
                    mousex,
                    mousey,
                    t,
                );

                draw_paragraph(
                    &mut canvas,
                    font,
                    width - 450.0,
                    50.0,
                    150.0,
                    100.0,
                    mousex,
                    mousey,
                );

                draw_colorwheel(&mut canvas, width - 300.0, height - 350.0, 250.0, 250.0, t);

                draw_lines(&mut canvas, 120.0, height - 50.0, 600.0, 50.0, t);
                draw_widths(&mut canvas, 10.0, 50.0, 30.0);
                draw_fills(&mut canvas, width - 200.0, height - 100.0, mousex, mousey);
                draw_caps(&mut canvas, 10.0, 300.0, 30.0);

                draw_scissor(&mut canvas, 50.0, height - 80.0, t);

                draw_window(
                    &mut canvas,
                    font,
                    "Widgets `n Stuff",
                    50.0,
                    50.0,
                    300.0,
                    400.0,
                );

                let x = 60.0;
                let mut y = 95.0;

                draw_search_box(&mut canvas, font, "Search", x, y, 280.0, 25.0);
                y += 40.0;
                draw_drop_down(&mut canvas, font, "Effects", 60.0, 135.0, 280.0, 28.0);
                let popy = y + 14.0;
                y += 45.0;

                draw_label(&mut canvas, font, "Login", x, y, 280.0, 20.0);
                y += 25.0;
                draw_edit_box(&mut canvas, font, "Email", x, y, 280.0, 28.0);
                y += 35.0;
                draw_edit_box(&mut canvas, font, "Password", x, y, 280.0, 28.0);
                y += 38.0;
                draw_check_box(&mut canvas, font, "Remember me", x, y, 140.0, 28.0);
                draw_button(
                    &mut canvas,
                    font,
                    Some("\u{E740}"),
                    "Sign in",
                    x + 138.0,
                    y,
                    140.0,
                    28.0,
                    Color::rgba(0, 96, 128, 255),
                );
                y += 45.0;

                // Slider
                draw_label(&mut canvas, font, "Diameter", x, y, 280.0, 20.0);
                y += 25.0;
                draw_edit_box_num(&mut canvas, font, "123.00", "px", x + 180.0, y, 100.0, 28.0);
                draw_slider(&mut canvas, 0.4, x, y, 170.0, 28.0);
                y += 55.0;

                draw_button(
                    &mut canvas,
                    font,
                    Some("\u{E729}"),
                    "Delete",
                    x,
                    y,
                    160.0,
                    28.0,
                    Color::rgba(128, 16, 8, 255),
                );
                draw_button(
                    &mut canvas,
                    font,
                    None,
                    "Cancel",
                    x + 170.0,
                    y,
                    110.0,
                    28.0,
                    Color::rgba(0, 0, 0, 0),
                );

                draw_thumbnails(&mut canvas, 365.0, popy - 30.0, 160.0, 300.0, &images, t);

                /*
                draw_spinner(&mut canvas, 15.0, 285.0, 10.0, t);
                */

                if let Some(image_id) = screenshot_image_id {
                    let x = size.width as f32 - 512.0;
                    let y = size.height as f32 - 512.0;

                    let paint = Paint::image(image_id, x, y, 512.0, 512.0, 0.0, 1.0);

                    let mut path = Path::new();
                    path.rect(x, y, 512.0, 512.0);
                    canvas.fill_path(&mut path, paint);
                    canvas.stroke_path(&mut path, Paint::color(Color::hex("454545")));
                }

                // if true {
                //     let paint = Paint::image(image_id, size.width as f32, 15.0, 1920.0, 1080.0, 0.0, 1.0);
                //     let mut path = Path::new();
                //     path.rect(size.width as f32, 15.0, 1920.0, 1080.0);
                //     canvas.fill_path(&mut path, paint);
                // }

                canvas.save_with(|canvas| {
                    canvas.reset();
                    perf.render(canvas, 5.0, 5.0);
                });

                //canvas.restore();

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                //scroll = 1.0;
                window.request_redraw()
            }
            _ => (),
        }
    });
}
