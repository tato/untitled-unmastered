use sdl2::keyboard::Keycode;
// use crate::editor::Mode;
// use sdl2::keyboard::{Keycode,Mod};

// struct KeyBinding<'a> {
//     keys: &'a [Keycode],
//     modifs: &'a [Mod],
//     mode: Mode,
//     action: fn(),
// }

// const BINDINGS: &[KeyBinding] = &[
//     KeyBinding { 
//         keys: &[Keycode::Backspace],
//         modifs: &[],
//         mode: Mode::INSERT,
//         action: |editor, render| {
//             let pos = editor.cursor_position_in_buffer();
//             editor.buffer.remove(pos);
//             editor.move_cursor(render, -1, 0);
//         },
//     },
//     KeyBinding {
//         keys: &[Keycode::Return],
//         modifs: &[],
//         mode: Mode::INSERT,
//         action: |editor, render| {
//             let pos = editor.cursor_position_in_buffer();
//             editor.buffer.insert("\n", pos);
//             editor.move_cursor(render, 0, 1);
//             editor.cursor_x = 0;
//         }
//     }
// ];

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
        Keycode::Left => Some(LEFT),
        Keycode::Right => Some(RIGHT),
        Keycode::Up => Some(UP),
        Keycode::Down => Some(DOWN),
        _ => None,
    }
}

