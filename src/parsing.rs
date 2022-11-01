
use proc_macro::{TokenStream, TokenTree};

use motif::*;

use crate::data::*;

pub fn parse( input : TokenStream ) -> Result<Pattern, MatchError>  {
    let input = input.into_iter().collect::<Vec<_>>();
    let mut input = input.iter().enumerate();
    pattern(&mut input)
}

group!(pattern<'a>: &'a TokenTree => Pattern = |input| {

    pred!(wild<'a>: &'a TokenTree => Pattern = |_x| match _x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        Pattern::Wild
    } );

    alt!(main<'a>: &'a TokenTree => Pattern = wild);


    main(input)
});