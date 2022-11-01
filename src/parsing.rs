
use proc_macro::{TokenStream, TokenTree};

use motif::*;

use crate::data::*;

pub fn parse( input : TokenStream ) -> Result<Pattern, MatchError>  {

    Err(MatchError::Fatal(0))
}

group!(pattern<'a>: &'a TokenTree => Pattern = |input| {

    /*pred!(wild<'a>: &'a TokenTree => char = |z| match z { TokenTree::Punct(p) => p.as_char() == ',', _ => false } => {
        match z {
            TokenTree::Punct(p) => p.as_char(),
            _ => unreachable!(),
        }
    } );*/


    Err(MatchError::Fatal(0))
});