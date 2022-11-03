
mod data;
mod parsing;


use proc_macro::{TokenStream, TokenTree};

use crate::parsing::parse_object_pattern;


#[proc_macro]
pub fn object_pattern(input : TokenStream) -> TokenStream {

    let input = input.into_iter().collect::<Vec<_>>();
    let input = input.iter();
    let _x = parse_object_pattern(input).unwrap();

    "".parse().unwrap()
}
