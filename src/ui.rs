use crate::{IO,Editor, editor};
use resource::resource;
use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId,
    LineCap, LineJoin, Paint, Path, Renderer, Solidity,
};
use unicode_segmentation::UnicodeSegmentation;

pub struct UI {
    pub canvas: Canvas<OpenGl>,
    font: FontId,
    font_paint: Paint,
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

        let mut font_paint = Paint::color(Color::rgbf(
            foreground_color[0],
            foreground_color[1],
            foreground_color[2],
        ));
        font_paint.set_font(&[font]);
        Self {
            canvas,
            font,
            font_paint,
            background_color,
            foreground_color,
        }
    }
    pub fn run(&mut self, io: &mut IO, editor: &Editor) {

        self.canvas.set_size(io.window_dimensions[0], io.window_dimensions[1], io.dpi_factor as f32);
        self.canvas.clear_rect(
            0,
            0,
            io.window_dimensions[0],
            io.window_dimensions[1],
            Color::rgbf(self.background_color[0], self.background_color[1], self.background_color[2]),
        );

        let foreground_paint = Paint::color(Color::rgbf(
            self.foreground_color[0], self.foreground_color[1], self.foreground_color[2],
        ));
        let background_paint = Paint::color(Color::rgbf(
            self.background_color[0], self.background_color[1], self.background_color[2],
        ));
        
        let cursor_color_ms_interval = 500;
        let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
        let cursor_paint = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
            foreground_paint
        } else {
            background_paint
        };
        
        let text_metrics = self.canvas.measure_text(0.0, 0.0, "A", self.font_paint)
            .expect("Unexpected error: Can't measure font");
        let font_metrics = self.canvas.measure_font(self.font_paint)
            .expect("Unexpected error: Can't measure font");

        let cursor_width = match editor.mode {
            editor::Mode::INSERT => text_metrics.width() / 4.0,
            _ => text_metrics.width(),
        };
        
        let cursor = editor.buffer.cursor();

        let cursor_screen_x = ((cursor.0 as u32) * text_metrics.width() as u32) as i32;
        let cursor_screen_y = (((cursor.1 - editor.y_render_offset) as u32) * font_metrics.height() as u32) as i32;
        let mut cursor_target = Path::new();
        cursor_target.rect(
            cursor_screen_x as f32, cursor_screen_y as f32,
            cursor_width as f32, font_metrics.height() as f32,
        );
        self.canvas.fill_path(&mut cursor_target, cursor_paint);

        let _window_width_in_characters = io.window_dimensions[0] / text_metrics.width() as u32;
        let window_height_in_characters = io.window_dimensions[1] / font_metrics.height() as u32;
        
        let status_line_y =
            ((window_height_in_characters - 2) * font_metrics.height() as u32) as i32;
        let mut status_line_rect = Path::new();
        status_line_rect.rect(
            0.0, status_line_y as f32,
            io.window_dimensions[0] as f32, font_metrics.height(),
        );
        self.canvas.fill_path(&mut cursor_target, cursor_paint);

        let status_text = format!(" {} > {} < $ {} {:?}",
                                  cursor.1,
                                  editor.editing_file_path,
                                  editor.matching_input_text,
                                  editor.matching_input_timeout);
        
        self.canvas.fill_text(0.0, status_line_y as f32 + font_metrics.ascender(), status_text.as_str(), self.font_paint);

        for (line_index, line) in editor
            .buffer
            .to_string()
            .split('\n')
            .skip(editor.y_render_offset)
            .take((window_height_in_characters - 2) as usize)
            .enumerate()
        {
            let y = line_index as f32 * font_metrics.height();
            self.canvas.fill_text(0.0, y + font_metrics.ascender(), line, self.font_paint);
        }

        self.canvas.flush();
    }
}

