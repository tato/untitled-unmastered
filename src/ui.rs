use crate::{editor, Editor, IO};
use femtovg::{renderer::OpenGl, Canvas, Color, FontId, Paint, Path};
use resource::resource;

pub struct UI {
    pub canvas: Canvas<OpenGl>,
    font: FontId,
    background_color: [f32; 4],
    foreground_color: [f32; 4],
}
impl UI {
    pub fn new(mut canvas: Canvas<OpenGl>) -> Self {
        let font = canvas
            .add_font_mem(&resource!("src/Cousine-Regular.ttf"))
            .expect("Cannot add font");
        let background_color = [0.3, 0.3, 0.32, 1.0];
        let foreground_color = [1.0, 1.0, 1.0, 1.0];

        Self {
            canvas,
            font,
            background_color,
            foreground_color,
        }
    }
    pub fn run(&mut self, io: &mut IO, editor: &Editor) {
        self.canvas
            .set_size(io.window_dimensions[0], io.window_dimensions[1], 1.0);
        self.canvas.clear_rect(
            0,
            0,
            io.window_dimensions[0],
            io.window_dimensions[1],
            Color::rgbf(
                self.background_color[0],
                self.background_color[1],
                self.background_color[2],
            ),
        );

        let mut foreground_paint = Paint::color(Color::rgbf(
            self.foreground_color[0],
            self.foreground_color[1],
            self.foreground_color[2],
        ));
        foreground_paint.set_font(&[self.font]);
        foreground_paint.set_font_size(18.0);
        let mut background_paint = Paint::color(Color::rgbf(
            self.background_color[0],
            self.background_color[1],
            self.background_color[2],
        ));
        background_paint.set_font(&[self.font]);
        foreground_paint.set_font_size(18.0);

        let cursor_color_ms_interval = 500;
        let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
        let cursor_paint = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
            foreground_paint
        } else {
            background_paint
        };

        let text_metrics = self
            .canvas
            .measure_text(0.0, 0.0, "A", foreground_paint)
            .expect("Unexpected error: Can't measure font");
        let font_metrics = self
            .canvas
            .measure_font(foreground_paint)
            .expect("Unexpected error: Can't measure font");

        let font_width = text_metrics.width();
        let font_height = font_metrics.height();

        let cursor_width = match editor.mode {
            editor::Mode::INSERT => font_width / 4.0,
            _ => font_width,
        };

        let cursor = editor.buffer.cursor();

        let cursor_screen_x = ((cursor.0 as u32) * font_width as u32) as i32;
        let cursor_screen_y =
            (((cursor.1 - editor.y_render_offset) as u32) * font_height as u32) as i32;
        let mut cursor_target = Path::new();
        cursor_target.rect(
            cursor_screen_x as f32,
            cursor_screen_y as f32,
            cursor_width as f32,
            font_height as f32,
        );
        self.canvas.fill_path(&mut cursor_target, cursor_paint);

        let _window_width_in_characters = io.window_dimensions[0] / font_width as u32;
        let window_height_in_characters = io.window_dimensions[1] / font_metrics.height() as u32;

        let status_line_y = ((window_height_in_characters - 2) * font_height as u32) as i32;
        let mut status_line_rect = Path::new();
        status_line_rect.rect(
            0.0,
            status_line_y as f32,
            io.window_dimensions[0] as f32,
            font_height,
        );
        self.canvas
            .fill_path(&mut status_line_rect, foreground_paint);

        let status_text = format!(
            " {} > {} < $ {} {:?}",
            cursor.1,
            editor.editing_file_path,
            // editor.matching_input_text,
            "sorry",
            editor.matching_input_timeout
        );

        self.canvas
            .fill_text(
                0.0,
                status_line_y as f32 + font_metrics.ascender(),
                status_text.as_str(),
                background_paint,
            )
            .expect("Unexpected rendering error");

        for (line_index, line) in editor
            .buffer
            .to_string()
            .split('\n')
            .skip(editor.y_render_offset)
            .take((window_height_in_characters - 2) as usize)
            .enumerate()
        {
            let y = line_index as f32 * font_height;
            self.canvas
                .fill_text(0.0, y + font_metrics.ascender(), line, foreground_paint)
                .expect("Unexpected rendering error");
        }

        self.canvas.flush();
    }

    // TODO: depends on a lot of values from the render function. refactor.
    // TODO: it shouldn't be necessary for this function to take &mut
    pub fn character_height(&mut self) -> u32 {
        let mut foreground_paint = Paint::color(Color::rgbf(
            self.foreground_color[0],
            self.foreground_color[1],
            self.foreground_color[2],
        ));
        foreground_paint.set_font(&[self.font]);
        foreground_paint.set_font_size(18.0);

        let font_metrics = self
            .canvas
            .measure_font(foreground_paint)
            .expect("Unexpected error: Can't measure font");

        let font_height = font_metrics.height();
        font_height.ceil() as u32
    }
}
