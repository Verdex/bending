
use proc_macro::{TokenStream, TokenTree};

use motif::*;

use crate::data::*;

pub fn parse_object_pattern<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<Vec<ObjectPattern<'a>>, MatchError> {
    let mut input = input.enumerate();
    object_pattern(&mut input)
}

pred!(semi_colon<'a>: &'a TokenTree = |_x| match _x { TokenTree::Punct(p) => p.as_char() == ';', _ => false });
group!(arrow<'a>: &'a TokenTree => () = |input| {

    pred!(equal<'a>: &'a TokenTree = |_x| match _x { TokenTree::Punct(p) => p.as_char() == '=', _=> false });
    pred!(greater<'a>: &'a TokenTree = |_x| match _x { TokenTree::Punct(p) => p.as_char() == '>', _=> false });

    seq!(main<'a>: &'a TokenTree => () = equal, ! greater, { () });

    main(input)
});

group!(object_pattern<'a>: &'a TokenTree => Vec<ObjectPattern<'a>> = |input| {

    pred!(wild<'a>: &'a TokenTree => ObjectPattern<'a> = |_x| match _x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        ObjectPattern::Wild
    });

    pred!(bang<'a>: &'a TokenTree => ObjectPattern<'a> = |_x| match _x { TokenTree::Punct(p) => p.as_char() == '!',  _ => false } => {
        ObjectPattern::Next
    });

    seq!(literal<'a>: &'a TokenTree => ObjectPattern<'a> = lit <= TokenTree::Literal(_), { 
        if let TokenTree::Literal(lit) = lit {
            ObjectPattern::Literal(lit)
        }
        else {
            unreachable!()
        }
    });

    alt!(option<'a>: &'a TokenTree => ObjectPattern<'a> = wild
                                                        | bang
                                                        | literal 
                                                        );

    seq!(option_semi<'a>: &'a TokenTree => ObjectPattern<'a> = o <= option, semi_colon, { o });
    
    seq!(options<'a>: &'a TokenTree => Vec<ObjectPattern<'a>> = os <= * option_semi, o <= ! option, {
        let mut os = os;
        os.push(o);
        os
    });

    options(input)
});