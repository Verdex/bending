
use proc_macro::{TokenStream, TokenTree};

use motif::*;

use crate::data::*;

pub fn parse_object_pattern<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<Vec<ObjectPattern>, MatchError> {
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

group!(object_pattern<'a>: &'a TokenTree => Vec<ObjectPattern> = |input| {
    // TODO every pattern needs a sub !
    // TODO except the last pattern which can't have a !
    // TODO also wild doesn't work by itself

    pred!(wild<'a>: &'a TokenTree => ObjectPattern = |_x| match _x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        ObjectPattern::Wild
    });

    pred!(bang<'a>: &'a TokenTree => ObjectPattern = |_x| match _x { TokenTree::Punct(p) => p.as_char() == '!',  _ => false } => {
        ObjectPattern::Next
    });

    seq!(literal<'a>: &'a TokenTree => ObjectPattern = lit <= TokenTree::Literal(_), { 
        if let TokenTree::Literal(lit) = lit {
            ObjectPattern::Literal(lit.to_string())
        }
        else {
            unreachable!()
        }
    });

    alt!(option<'a>: &'a TokenTree => ObjectPattern = wild
                                                    | bang
                                                    | literal 
                                                    );

    seq!(option_semi<'a>: &'a TokenTree => ObjectPattern = o <= option, semi_colon, { o });
    
    seq!(options<'a>: &'a TokenTree => Vec<ObjectPattern> = os <= * option_semi, o <= ! option, {
        let mut os = os;
        os.push(o);
        os
    });

    options(input)
});

//seq!()