use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


pub struct RenderContext<'sdlttf> {
    canvas: sdl2::render::WindowCanvas,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ttf: sdl2::ttf::Sdl2TtfContext,

    font: sdl2::ttf::Font<'sdlttf, 'static>,
    pub character_width: u32,
    pub character_height: u32,

    cache: HashMap<(char, Color), Surface<'sdlttf>>,
}
impl From<&sdl2::Sdl> for RenderContext<'_> {
    fn from(sdl: &sdl2::Sdl) -> Self {
        let video = sdl.video().unwrap();
        let ttf = sdl2::ttf::init().unwrap();

        let window = video
            .window("ttttt...", 10, 10)
            .maximized()
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let font = ttf.load_font("./Cousine-Regular.ttf", 18).unwrap();
        let any_character_metrics = font.find_glyph_metrics('A').unwrap();
        let character_width = any_character_metrics.advance as u32;
        let character_height = font.recommended_line_spacing() as u32;

        Self {
            canvas,
            texture_creator,
            ttf,
            font,
            character_width,
            character_height,
            cache: HashMap::new(),
        }
    }
}
impl RenderContext<'_> {
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
