use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::ttf;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct RenderContext<'sdlttf> {
    canvas: sdl2::render::WindowCanvas,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    // ttf_context: &'sdlttf ttf::Sdl2TtfContext,

    font: ttf::Font<'sdlttf, 'static>,
    pub character_width: u32,
    pub character_height: u32,

    cache: HashMap<(char, Color), Surface<'sdlttf>>,
}
impl<'a> RenderContext<'a> {
    pub fn new(video_context: &sdl2::VideoSubsystem,
               ttf_context: &'a ttf::Sdl2TtfContext) -> Self {

        let window = video_context
            .window("ttttt...", 10, 10)
            .maximized()
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let font = ttf_context.load_font("./Cousine-Regular.ttf", 18).unwrap();
        let any_character_metrics = font.find_glyph_metrics('A').unwrap();
        let character_width = any_character_metrics.advance as u32;
        let character_height = font.recommended_line_spacing() as u32;

        Self {
            canvas,
            texture_creator,
            // ttf_context,
            font,
            character_width,
            character_height,
            cache: HashMap::new(),
        }
    }
    pub fn width(&self) -> u32 {
        self.canvas.window().size().0
    }
    pub fn height(&self) -> u32 {
        self.canvas.window().size().1
    }
    pub fn start_frame(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
    pub fn finish_frame(&mut self) {
        self.canvas.present();
    }
    pub fn fill_rect(&mut self, area: Rect, color: Color) {
        let mut surface = Surface::new(
            area.w as u32,
            area.h as u32,
            self.canvas.default_pixel_format(),
        )
        .unwrap();
        surface.fill_rect(None, color).unwrap();
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        self.canvas.copy(&texture, None, Some(area)).unwrap();
    }

    pub fn draw_character(&mut self, c: char, color: Color, x: i32, y: i32) {
        if let Entry::Vacant(entry) = self.cache.entry((c, color)) {
            let surface = self.font
                .render(&c.to_string())
                .blended(color)
                .unwrap(); // TODO(ptato) .unwrap_or_default()
            entry.insert(surface);
        }
        let surface = &self.cache[&(c, color)];
        let target = Rect::new(x, y, surface.width(), surface.height());
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}
