use primitive_types::U256;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn uint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt);

    let s = format!("{}", input);

    let i = if s.len() > 2 && &s[..2] == "0x" {
        U256::from_str_radix(&s, 16).unwrap()
    } else {
        U256::from_dec_str(&s).unwrap()
    };

    let [a, b, c, d] = i.0;

    let expand = quote! {
        ::patine_core::U256::from_raw(#d, #c, #b, #a)
    };

    TokenStream::from(expand)
}

#[proc_macro_attribute]
pub fn contract(input: TokenStream, _args: TokenStream) -> TokenStream {
    input
}
