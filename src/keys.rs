use crate::editor::Mode;
use sdl2::keyboard::{Keycode,Mod};

struct KeyBinding<'a> {
    keys: &'a [Keycode],
    modifs: &'a [Mod],
    mode: Mode,
    action: fn(),
}

const BINDINGS: &[KeyBinding] = &[
    KeyBinding { 
        keys: &[Keycode::Backspace],
        modifs: &[],
        mode: Mode::INSERT,
        action: |editor, render| {
            let pos = editor.cursor_position_in_buffer();
            editor.buffer.remove(pos);
            editor.move_cursor(render, -1, 0);
        },
    },
    KeyBinding {
        keys: &[Keycode::Return],
        modifs: &[],
        mode: Mode::INSERT,
        action: |editor, render| {
            let pos = editor.cursor_position_in_buffer();
            editor.buffer.insert("\n", pos);
            editor.move_cursor(render, 0, 1);
            editor.cursor_x = 0;
        }
    }
];

