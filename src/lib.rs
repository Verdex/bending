
mod data;
mod parsing;


use proc_macro::{TokenStream, TokenTree};

use crate::parsing::parse;


#[proc_macro]
pub fn blarg(input : TokenStream) -> TokenStream {

    let input = input.into_iter().collect::<Vec<_>>();
    let input = input.iter();
    let _x = parse(input).unwrap();

    "".parse().unwrap()
}
