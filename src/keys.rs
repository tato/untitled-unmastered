use sdl2::keyboard::Keycode;

pub const BACKSPACE: &'static str = "\x08";
pub const ESCAPE: &'static str = "\x1B";
pub const LEFT: &'static str = "\u{00FDD0}";
pub const RIGHT: &'static str = "\u{00FDD1}";
pub const UP: &'static str = "\u{00FDD2}";
pub const DOWN: &'static str = "\u{00FDD3}";

pub fn get_utf8_for_keycode(keycode: Keycode) -> Option<&'static str> {
    match keycode {
        Keycode::Backspace => Some(BACKSPACE),
        Keycode::Escape => Some(ESCAPE),
        Keycode::Return => Some("\n"),
        Keycode::Left => Some(LEFT),
        Keycode::Right => Some(RIGHT),
        Keycode::Up => Some(UP),
        Keycode::Down => Some(DOWN),
        _ => None,
    }
}

