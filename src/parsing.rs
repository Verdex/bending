
use proc_macro::{TokenStream, TokenTree};

use motif::*;

use crate::data::*;

pub fn parse_object_pattern<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<Vec<ObjectPattern<'a>>, MatchError>  {
    let mut input = input.enumerate();
    object_pattern(&mut input)
}

group!(pattern<'a>: &'a TokenTree => Pattern<'a> = |input| {

group!(object_pattern<'a>: &'a TokenTree => Vec<ObjectPattern<'a>> = |input| {

    pred!(wild<'a>: &'a TokenTree => ObjectPattern<'a> = |_x| match _x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        ObjectPattern::Wild
    } );

    seq!(literal<'a>: &'a TokenTree => ObjectPattern<'a> = lit <= TokenTree::Literal(_), { 
        if let TokenTree::Literal(lit) = lit {
            ObjectPattern::Literal(lit)
        }
        else {
            unreachable!()
        }
    });

    alt!(option<'a>: &'a TokenTree => ObjectPattern<'a> = wild
                                                        | literal 
                                                        );

    seq!(option_semi<'a>: &'a TokenTree => ObjectPattern<'a> = o <= option,  { o });

    main(input)
});