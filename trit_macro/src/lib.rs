use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn trits(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let s = lit.value();

    let mut value: u8 = 0;
    for c in s.chars() {
        if c == '_' { continue; }

        value <<= 2;
        value |= match c {
            '-' | 'T' => 0b10,
            '0' => 0b00,
            '+' | '1' => 0b01,
            _ => panic!("invalid trit"),
        };
    }

    let out = quote! { Trit4(#value) }; 
    out.into()
}
