use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn trits(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let s = lit.value();

    let mut value: u64 = 0;
    let mut len = 0;

    for c in s.chars() {
        if c == '_' { continue; }

        len += 1;
        value <<= 2;

        value |= match c {
            '-' | 'T' => 0b10,
            '0' => 0b00,
            '+' | '1' => 0b01,
            _ => panic!("invalid trit"),
        };
    }

    let ty = match len {
        1..=4 => quote! { Trit4 },
        5..=8 => quote! { Trit8 },
        9..=16 => quote! { Trit16 },
        17..=32 => quote! { Trit32 },
        _ => panic!("too many trits"),
    };

    let out = quote! {
        #ty(#value as _)
    };

    out.into()
}