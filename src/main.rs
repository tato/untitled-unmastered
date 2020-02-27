extern crate sdl2;

use sdl2::pixels::Color;

#[derive(Default)]
struct Editor {
    text: String,

    cursor_x: u32,
    cursor_y: u32,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let sdl_ttf = sdl2::ttf::init().unwrap();

    let cousine = sdl_ttf.load_font("./Cousine-Regular.ttf", 14).unwrap();

    let measure_surface = cousine.render("A").blended(Color::RGB(0,0,0)).unwrap();
    let character_width = measure_surface.width();
    let character_height = measure_surface.height();

    let characters_wide = 80u32;
    let characters_high = 30u32;

    let mut editor: Editor = Default::default();

    let window = sdl_video.window("ttttt...", character_width*characters_wide, character_height*characters_high)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(250, 250, 250));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    editor.text = editor.text[0..editor.text.len()-1].to_string();
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    editor.text.push('\n');
                },
                Event::TextInput { text, .. } => {
                    editor.text = format!("{}{}", editor.text, text);
                },
                _ => { }
            }
        }

        canvas.set_draw_color(Color::RGB(250, 250, 250));
        canvas.clear();

        for (line_index, line) in editor.text.split("\n").enumerate() {
            if line.len() == 0 { continue; }
            let text_surface = cousine.render(line).blended(Color::RGB(0,0,0)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();

            let target_y: i32 = (line_index as i32)*(character_height as i32);
            use sdl2::rect::Rect;
            let target = Rect::new(0, target_y, text_surface.width(), text_surface.height());
            canvas.copy(&texture, None, Some(target)).unwrap();
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        let cursor_screen_x = (editor.cursor_x*character_width) as i32;
        let cursor_screen_y = (editor.cursor_y*character_height) as i32;
        let rect = sdl2::rect::Rect::new(cursor_screen_x, cursor_screen_y, character_width, character_height);
        canvas.fill_rect(rect).unwrap();

        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000u32 / 30));
    }
}
