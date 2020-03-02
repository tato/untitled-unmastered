extern crate sdl2;
use sdl2::*;
use sdl2::surface::Surface;
use sdl2::pixels::Color;
use std::collections::HashMap;

pub struct Font<'sdlttf, 'rwops, 'a> {
    source: ttf::Font<'sdlttf, 'rwops>,

    pub character_width: u32,
    pub character_height: u32,

    cache: HashMap<(char,Color), Surface<'a>>,
}
// NOTE(ptato) Lifetime god, please forgive me 😩💦
impl<'sdlttf, 'rwops, 'a> From<ttf::Font<'sdlttf, 'rwops>> for Font<'sdlttf, 'rwops, 'a> {
    fn from(source: ttf::Font<'sdlttf, 'rwops>) -> Self {
        assert!(source.face_is_fixed_width());
        let any_character_metrics = source.find_glyph_metrics('A').unwrap();
        let character_width  = any_character_metrics.advance as u32;
        let character_height = source.recommended_line_spacing() as u32;
        let cache = HashMap::new();
        Self { source, character_width, character_height, cache }
    }
}
impl Font<'_, '_, '_> {
    pub fn get_surface_for(&mut self, c: char, color: Color) -> &Surface {
        if !self.cache.contains_key(&(c,color)) {
            let ch_surface = self.source
                .render(&c.to_string())
                .blended(color)
                .unwrap(); // TODO(ptato) .unwrap_or_default()
            self.cache.insert((c,color), ch_surface);
        }
        &self.cache[&(c,color)]
    }
}
