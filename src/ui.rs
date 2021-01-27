use crate::{IO,Editor, editor};
use resource::resource;
use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId,
    LineCap, LineJoin, Paint, Path, Renderer, Solidity,
};

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
        
        let cursor_color_ms_interval = 500;
        let elapsed_ms = editor.cursor_animation_instant.elapsed().as_millis();
        let cursor_color = if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
            self.foreground_color
        } else {
            self.background_color
        };
        
        let font_metrics = self.canvas.measure_font(self.font_paint)
            .expect("Unexpected error: Can't measure font");
        let cursor_width = match editor.mode {
            editor::Mode::INSERT => font_metrics.width() / 4,
            _ => font_metrics.width(),
        };
        
        let cursor = editor.buffer.cursor();

        let cursor_screen_x = ((cursor.0 as u32) * font_metrics.width() as u32) as i32;
        let cursor_screen_y = (((cursor.1 - editor.y_render_offset) as u32) * font_metrics.height() as u32) as i32;
        let mut cursor_target = Path::new();
        cursor_target.rect(
            cursor_screen_x as f32, cursor_screen_y as f32,
            cursor_width as f32, font_metrics.height() as f32,
        );
        let cursor_paint = Paint::color(Color::rgbf(
            cursor_color[0], cursor_color[1], cursor_color[2],
        ));
        self.canvas.fill_path(&mut cursor_target, cursor_paint);

        self.canvas.flush();





        // let _window_width_in_characters = render.width() / character_width;
        // let window_height_in_characters = render.height() / character_height;

        // let status_line_y =
        //     ((window_height_in_characters - 2) * character_height) as i32;
        // let status_line_rect = Rect::new(
        //     0, status_line_y,
        //     render.width(), character_height
        // );
        // render.fill_rect(status_line_rect, foreground_color).unwrap_or(());

        // let status_text = format!(" {} > {} < $ {} {:?}",
        //                           cursor.1,
        //                           editor.editing_file_path,
        //                           editor.matching_input_text,
        //                           editor.matching_input_timeout);

        // let gcs = UnicodeSegmentation::graphemes(status_text.as_str(), true);
        // for (ci_usize, c) in gcs.enumerate() {
        //     let ci: i32 = ci_usize as i32;
        //     let cw: i32 = character_width as i32;

        //     let target_x = ci * cw;
        //     let target_y = status_line_y;
        //     render
        //         .draw_character(c, background_color, target_x, target_y)
        //         .unwrap_or(());
        // }

        // for (line_index, line) in editor
        //     .buffer
        //     .to_string()
        //     .split('\n')
        //     .skip(editor.y_render_offset)
        //     .take((window_height_in_characters - 2) as usize)
        //     .enumerate()
        // {
        //     let gcs = UnicodeSegmentation::graphemes(line, true);
        //     for (ch_index, c) in gcs.enumerate() {
        //         let character_color = if line_index == cursor.1 as usize
        //             && ch_index == cursor.0 as usize
        //             && editor.mode != editor::Mode::INSERT
        //         {
        //             if (elapsed_ms / cursor_color_ms_interval) % 2 == 0 {
        //                 background_color
        //             } else {
        //                 foreground_color
        //             }
        //         } else {
        //             foreground_color
        //         };

        //         let target_x: i32 = (ch_index as i32) * (character_width as i32);
        //         let target_y: i32 = (line_index as i32) * (character_height as i32);
        //         render
        //             .draw_character(c, character_color, target_x, target_y)
        //             .unwrap_or(());
        //     }
        // }

        // render.finish_frame();
    }
}

