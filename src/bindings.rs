use crate::*;
use crate::editor::Mode;

pub struct KeyBinding {
    pub keys: &'static str,
    modifs: &'static str,
    mode: Mode,
    action: fn(),
}
pub const DEFAULT_BINDING: KeyBinding = KeyBinding {
    keys: "",
    modifs: "",
    mode: Mode::NORMAL,
    action: actions::nop,
};
pub const BINDINGS: &[KeyBinding] = &[
    /*
    KeyBinding {
        keys: "dd",
        mode: Mode::NORMAL,
        action: actions::delete,
        ..DEFAULT_BINDING
    },
    */
];
