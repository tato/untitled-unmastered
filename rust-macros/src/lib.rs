extern crate proc_macro;
use proc_macro::{Literal,TokenStream,TokenTree};

// the only fields allowed to be public in a proc-macro crate are the
// procedural macros themselves, so this generates some that export
// the key modifier bitfield
macro_rules! modif_definitions {
    ( $(const $name:ident = $value:expr;)*) => {
        mod modif {
            $(
                pub(crate) const $name: u32 = $value;
            )*
        }

        $(
            #[proc_macro]
            #[allow(non_snake_case)]
            pub fn $name(_input: TokenStream) -> TokenStream {
                TokenTree::Literal(Literal::u32_suffixed(modif::$name)).into()
            }
        )*

        fn update_modif(modif: &mut u32, name: &str) -> bool {
            let mut matched = true;
            match name {
                $(
                    stringify!($name) => *modif |= modif::$name,
                )*
                _ => matched = false,
            }
            return matched;
        }
    }
}

modif_definitions!{
    const CTRL  = 1;
    const SHIFT = 1 << 1;
    const ALT   = 1 << 2;
    const META  = 1 << 3;
}

macro_rules! special_key_definitions {
    ( $(const $name:ident = $val:expr;)* ) => {
        mod key {
            $(
                pub(crate) const $name: &str = $val;
            )*
        }
        $(
            #[proc_macro]
            #[allow(non_snake_case)]
            pub fn $name(_input: TokenStream) -> TokenStream {
                TokenTree::Literal(Literal::string(key::$name)).into()
            }
        )*

        fn get_key(name: &str) -> &str {
            match name {
                $(
                    stringify!($name) => key::$name,
                )*
                _ => name,
            }
        }
    };
}

special_key_definitions!{
    const BACKSPACE = "\x08";
    const ESCAPE = "\x1B";
    const RETURN = "\n";
    const LEFT = "\u{00FDD0}";
    const RIGHT = "\u{00FDD1}";
    const UP = "\u{00FDD2}";
    const DOWN = "\u{00FDD3}";
}

#[proc_macro]
pub fn make_binding(input: TokenStream) -> TokenStream {
    let value = input.to_string();
    let bindings: Vec<_> = value.split(',').map(str::trim).collect();

    let mut keys = String::new();
    let mut modifs = Vec::new();

    for binding in bindings {
        let components: Vec<_> = binding.split('|').map(str::trim).collect();
        let mut key = "?";
        let mut modif = 0;
        for component in components {
            let modif_matched = update_modif(&mut modif, component);
            if !modif_matched {
                key = get_key(component);
            }
        }
        keys += key;
        modifs.push(modif);
    }

    format!("({:?}, &{:?})", &keys, &modifs).parse().unwrap()
}
