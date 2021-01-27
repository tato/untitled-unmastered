// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::surface::Surface;
// use sdl2::*;
use crate::panic_with_dialog;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct RenderContext<'a> {
    // canvas: &'a mut render::WindowCanvas,
    // texture_creator: &'a render::TextureCreator<video::WindowContext>,

    // ttf_context: &'a ttf::Sdl2TtfContext,

    // font: ttf::Font<'a, 'static>,
    pub character_width: u32,
    pub character_height: u32,

    pub hahahaha: &'a i32,
    // cache: HashMap<(String, Color), render::Texture<'a>>,
}
impl<'a> RenderContext<'a> {
    // pub fn new(canvas: &'a mut render::WindowCanvas,
    //            texture_creator: &'a render::TextureCreator<video::WindowContext>,
    //            ttf_context: &'a ttf::Sdl2TtfContext) -> Self {
    //     let rwops = rwops::RWops::from_bytes(
    //         include_bytes!("./Cousine-Regular.ttf")).unwrap();
    //     let font = ttf_context.load_font_from_rwops(rwops, 18).unwrap();
    //     let any_character_metrics = font
    //         .find_glyph_metrics('A')
    //         .ok_or("character 'A' was not found in font") // 🤪
    //         .unwrap_or_else(panic_with_dialog);
    //     let character_width = any_character_metrics.advance as u32;
    //     let character_height = font.recommended_line_spacing() as u32;

    //     Self {
    //         canvas,
    //         texture_creator,
    //         // ttf_context,
    //         font,
    //         character_width,
    //         character_height,
    //         cache: HashMap::new(),
    //     }
    // }
    pub fn width(&self) -> u32 {
        todo!("width")
        // self.canvas.window().size().0
    }
    pub fn height(&self) -> u32 {
        todo!("height")
        // self.canvas.window().size().1
    }
    // pub fn start_frame(&mut self, color: Color) {
    //     self.canvas.set_draw_color(color);
    //     self.canvas.clear();
    // }
    // pub fn finish_frame(&mut self) {
    //     self.canvas.present();
    // }
    // pub fn fill_rect(&mut self, area: Rect, color: Color) -> Result<(), String> {
    //     let mut surface = Surface::new(
    //         area.w as u32,
    //         area.h as u32,
    //         self.canvas.default_pixel_format(),
    //     )?;
    //     surface.fill_rect(None, color)?;
    //     let texture = self
    //         .texture_creator
    //         .create_texture_from_surface(&surface)
    //         .map_err(|e| e.to_string())?;
    //     self.canvas.copy(&texture, None, Some(area))?;

    //     Ok(())
    // }

    // pub fn draw_character(&mut self, c: &str, color: Color, x: i32, y: i32)
    //     -> Result<(), String>
    // {
    //     if let Entry::Vacant(entry) = self.cache.entry((c.to_string(), color)) {
    //         let surface = self.font
    //             .render(c)
    //             .blended(color)
    //             .map_err(|e| e.to_string())?;
    //         let tc = &self.texture_creator;
    //         let texture = tc
    //             .create_texture_from_surface(&surface)
    //             .map_err(|e| e.to_string())?;
    //         entry.insert(texture);
    //     }
    //     let texture = &self.cache[&(c.to_string(), color)];
    //     let query = texture.query();
    //     let target = Rect::new(x, y, query.width, query.height);
    //     self.canvas.copy(&texture, None, Some(target))?;

    //     Ok(())
    // }
}
