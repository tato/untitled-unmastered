use crate::{editor, Editor, IO};
use femtovg::{renderer::OpenGl, Canvas, Color, FontId, Paint, Path, FontMetrics};
use resource::resource;

pub struct UI {
    pub canvas: Canvas<OpenGl>,
    background_color: [f32; 4],
    foreground_color: [f32; 4],

    font: FontId,
    font_metrics: Option<FontMetrics>,
}
impl UI {
    pub fn new(mut canvas: Canvas<OpenGl>) -> Self {
        let background_color = [0.3, 0.3, 0.32, 1.0];
        let foreground_color = [1.0, 1.0, 1.0, 1.0];

        let font = canvas
            .add_font_mem(&resource!("src/Cousine-Regular.ttf"))
            .expect("Cannot add font");
        let mut ui = Self {
            canvas,
            background_color,
            foreground_color,
            font,
            font_metrics: None,
        };

        // TODO: i'm forced to Option<FontMetrics> :(
        let paint = ui.get_paint(ui.foreground_color);
        let font_metrics = ui
            .canvas
            .measure_font(paint)
            .expect("Unexpected error: Can't measure font");
        ui.font_metrics = Some(font_metrics);

        ui
    }

    fn get_paint(&self, color: [f32; 4]) -> Paint {
        let mut paint = Paint::color(Color::rgbf(color[0], color[1], color[2]));
        paint.set_font(&[self.font]);
        paint.set_font_size(18.0);
        paint.set_text_baseline(femtovg::Baseline::Top);
        paint
    }

    pub fn run(&mut self, io: &mut IO, editor: &Editor) {
        self.canvas
            .set_size(io.window_dimensions[0], io.window_dimensions[1], io.dpi_factor as f32);
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

        let foreground_paint = self.get_paint(self.foreground_color);
        let background_paint = self.get_paint(self.background_color);

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

        let font_width = text_metrics.width();

        let cursor_width = match editor.mode {
            editor::Mode::INSERT => font_width / 4.0,
            _ => font_width,
        };

        let cursor = editor.buffer.cursor();

        let cursor_screen_x = cursor.0 as f32 * font_width;
        let cursor_screen_y =
            (cursor.1 - editor.y_render_offset) as f32 * self.character_height();
        let mut cursor_target = Path::new();
        cursor_target.rect(
            cursor_screen_x,
            cursor_screen_y,
            cursor_width,
            self.character_height(),
        );
        self.canvas.fill_path(&mut cursor_target, cursor_paint);

        let _window_width_in_characters = io.window_dimensions[0] / font_width as u32;
        let window_height_in_characters = io.window_dimensions[1] / self.character_height() as u32;

        let status_line_y = (window_height_in_characters - 2) as f32 * self.character_height();
        let mut status_line_rect = Path::new();
        status_line_rect.rect(
            0.0,
            status_line_y,
            io.window_dimensions[0] as f32,
            self.character_height(),
        );
        self.canvas.fill_path(&mut status_line_rect, foreground_paint);

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
                status_line_y,
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
            let y = line_index as f32 * self.character_height();
            self.canvas
                .fill_text(0.0, y, line, foreground_paint)
                .expect("Unexpected rendering error");
        }

        self.canvas.flush();
    }

    pub fn character_height(&self) -> f32 {
        self.font_metrics.as_ref().map(FontMetrics::height).unwrap_or(0.0)
    }
}
