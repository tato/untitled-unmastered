extern crate sdl2;
pub fn panic_with_dialog<T>(m: impl std::fmt::Display) -> T {
    sdl2::messagebox::show_simple_message_box(
        sdl2::messagebox::MessageBoxFlag::ERROR, 
        "uu error", &m.to_string(), None).expect(&m.to_string());
    panic!("{}", m);
}

