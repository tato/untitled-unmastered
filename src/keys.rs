use crate::*;
use sdl2::keyboard::Keycode;

pub fn get_utf8_for_keycode(keycode: Keycode) -> Option<&'static str> {
    match keycode {
        Keycode::Backspace => Some(BACKSPACE!()),
        Keycode::Escape => Some(ESCAPE!()),
        Keycode::Return => Some("\n"),
        Keycode::Left => Some(LEFT!()),
        Keycode::Right => Some(RIGHT!()),
        Keycode::Up => Some(UP!()),
        Keycode::Down => Some(DOWN!()),
        _ => None,
    }
}

