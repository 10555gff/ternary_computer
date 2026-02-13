use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn trit(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let s = lit.value();

    let mut value: u32 = 0;
    for c in s.chars() {
        value <<= 2;
        value |= match c {
            '-' | 'T' => 0b10,
            '0' => 0b00,
            '+' | '1' => 0b01,
            _ => panic!("invalid trit"),
        };
    }

    let out = quote! { #value };
    out.into()
}
