use sdl2::keyboard::Keycode;

pub const BACKSPACE: &str = "\x08";
pub const ESCAPE: &str = "\x1B";
pub const LEFT: &str = "\u{00FDD0}";
pub const RIGHT: &str = "\u{00FDD1}";
pub const UP: &str = "\u{00FDD2}";
pub const DOWN: &str = "\u{00FDD3}";

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

