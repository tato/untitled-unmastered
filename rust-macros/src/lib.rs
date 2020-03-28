extern crate proc_macro;
use proc_macro::TokenStream;

const CTRL:  u32 = 1 << 0;
const SHIFT: u32 = 1 << 1;
const ALT:   u32 = 1 << 2;

#[proc_macro]
pub fn make_binding(input: TokenStream) -> TokenStream {
    let value = input.to_string();
    let bindings: Vec<_> = value.split(",").map(str::trim).collect();

    let mut keys = String::new();
    let mut modifs = Vec::new();

    for binding in bindings {
        let components: Vec<_> = binding.split("|").map(str::trim).collect();
        for component in components {
            let mut modif = 0;
            match component {
                "CTRL" => modif |= CTRL,
                "SHIFT" => modif |= SHIFT,
                "ALT" => modif |= ALT,
                _ => keys += component,
            }
            modifs.push(modif);
        }
    }

    format!("({:?}, &{:?})", &keys, &modifs).parse().unwrap()
}
