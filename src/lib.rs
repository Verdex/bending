
mod data;
mod parsing;


use proc_macro::{TokenStream, TokenTree};

use crate::parsing::parse;

#[proc_macro]
pub fn ikky(x : TokenStream) -> TokenStream {
    let y = x.into_iter().collect::<Vec<TokenTree>>().iter();

/*    pred!(comma<'a>: &'a TokenTree => char = |z| match z { TokenTree::Punct(p) => p.as_char() == ',', _ => false } => {
        match z {
            TokenTree::Punct(p) => p.as_char(),
            _ => unreachable!(),
        }
    } );*/
    //seq!(main<'a>: )

    "".parse().unwrap()
}

#[proc_macro]
pub fn blarg(x : TokenStream) -> TokenStream {



    "".parse().unwrap()
}
