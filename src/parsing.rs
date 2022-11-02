
use proc_macro::{TokenStream, TokenTree};

use motif::*;

use crate::data::*;

pub fn parse<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<Pattern<'a>, MatchError>  {
    let mut input = input.enumerate();
    pattern(&mut input)
}

group!(pattern<'a>: &'a TokenTree => Pattern<'a> = |input| {

    pred!(wild<'a>: &'a TokenTree => Pattern<'a> = |_x| match _x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        Pattern::Wild
    } );

    seq!(literal<'a>: &'a TokenTree => Pattern<'a> = lit <= TokenTree::Literal(_), { 
        if let TokenTree::Literal(lit) = lit {
            Pattern::Literal(lit)
        }
        else {
            unreachable!()
        }
    });

    alt!(main<'a>: &'a TokenTree => Pattern<'a> = wild
                                                | literal 
                                                );


    main(input)
});