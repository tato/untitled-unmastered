use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct RenderContext {
    canvas: sdl2::render::WindowCanvas,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}
impl From<sdl2::VideoSubsystem> for RenderContext {
    fn from(video: sdl2::VideoSubsystem) -> Self {
        let window = video.window("ttttt...", 10, 10)
            .maximized()
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self { canvas, texture_creator }
    }
}
impl RenderContext {
    pub fn start_frame(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
    pub fn finish_frame(&mut self) {
        self.canvas.present();
    }
    pub fn copy_surface(&mut self, surface: &Surface, area: Rect) {
        let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
        self.canvas.copy(&texture, None, Some(area)).unwrap();
    }
    pub fn fill_rect(&mut self, area: Rect, color: Color) {
        let mut surface = Surface::new(area.w as u32, area.h as u32, self.canvas.default_pixel_format()).unwrap();
        surface.fill_rect(None, color).unwrap();
        let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
        self.canvas.copy(&texture, None, Some(area)).unwrap();
    }
}
