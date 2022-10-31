
mod data;
mod parsing;

use proc_macro::{TokenStream, TokenTree};

use crate::parsing::pattern_parser;


#[proc_macro]
pub fn blarg(x : TokenStream) -> TokenStream {
    let mut z = x.into_iter();

    pattern_parser::parse(&mut z).unwrap();


    "".parse().unwrap()
}

//#[proc_macro]
//fn parse_pat(x : TokenStream) -> Result<
