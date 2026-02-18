use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn trits(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let s = lit.value();

    let mut value: u8 = 0;
    for c in s.chars() {
        // 允许用户使用下划线增加可读性，如 trits!(11_TT)
        if c == '_' { continue; }

        value <<= 2;
        match c {
            '-' | 'T' => value |= 0b10,
            '0'       => value |= 0b00,
            '+' | '1' => value |= 0b01,
            _ => panic!("invalid trit"),
        }
    }

    // let out = quote! { #value };
    // out.into()

    let out = quote! { Trit4(#value) }; 
    out.into()
}
