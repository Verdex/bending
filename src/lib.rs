
mod data;
mod parsing;


use proc_macro::{TokenStream, TokenTree};

use crate::parsing::parse;


#[proc_macro]
pub fn blarg(input : TokenStream) -> TokenStream {

    let _x = parse(input).unwrap();


    "".parse().unwrap()
}
