extern crate proc_macro;
use proc_macro::{Literal,TokenStream,TokenTree};

// the only fields allowed to be public in a proc-macro crate are the
// procedural macros themselves, so this generates some that export
// the key modifier bitfield
macro_rules! modif_definitions {
    ( $(const $name:ident = $value:expr;)*) => {
        mod internal {
            $(
                pub(crate) const $name: u32 = $value;
            )*
        }

        $(
            #[proc_macro]
            #[allow(non_snake_case)]
            pub fn $name(_input: TokenStream) -> TokenStream {
                TokenTree::Literal(Literal::u32_suffixed(internal::$name)).into()
            }
        )*
    }
}

modif_definitions!{
    const CTRL  = 1;
    const SHIFT = 1 << 1;
    const ALT   = 1 << 2;
}

#[proc_macro]
pub fn make_binding(input: TokenStream) -> TokenStream {
    let value = input.to_string();
    let bindings: Vec<_> = value.split(',').map(str::trim).collect();

    let mut keys = String::new();
    let mut modifs = Vec::new();

    for binding in bindings {
        let components: Vec<_> = binding.split('|').map(str::trim).collect();
        for component in components {
            let mut modif = 0;
            match component {
                "CTRL" => modif |= internal::CTRL,
                "SHIFT" => modif |= internal::SHIFT,
                "ALT" => modif |= internal::ALT,
                _ => keys += component,
            }
            modifs.push(modif);
        }
    }

    format!("({:?}, &{:?})", &keys, &modifs).parse().unwrap()
}
