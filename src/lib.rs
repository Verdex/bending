
mod data;
mod parsing;


use proc_macro::{TokenStream, TokenTree};

use crate::parsing::parse;


#[proc_macro]
pub fn blarg(x : TokenStream) -> TokenStream {



    "".parse().unwrap()
}
