
use proc_macro::{TokenStream, TokenTree};

use motif::*;
use denest::*;

use crate::data::*;

pub fn parse_object_pattern<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<Vec<ObjectPattern>, MatchError> {
    pred!(not_zero: usize = |_x| _x != 0);
    seq!(legit_sequence: usize => () = * not_zero, 0, { () });

    let mut input = input.enumerate();
    let pats = object_pattern(&mut input)?;
    if input.count() != 0 {
        return Err(MatchError::FatalEndOfFile);
    }

    let mut next_counts = pats.iter().map(|pat| pat.to_lax().filter(|x| matches!(x, ObjectPattern::Next)).count()).enumerate();

    legit_sequence(&mut next_counts)?;
    if next_counts.count() != 0 {
        return Err(MatchError::FatalEndOfFile);
    }

    Ok(pats)
}

pred!(semi_colon<'a>: &'a TokenTree = |_x| match _x { TokenTree::Punct(p) => p.as_char() == ';', _ => false });
group!(arrow<'a>: &'a TokenTree => () = |input| {

    pred!(equal<'a>: &'a TokenTree = |_x| match _x { TokenTree::Punct(p) => p.as_char() == '=', _=> false });
    pred!(greater<'a>: &'a TokenTree = |_x| match _x { TokenTree::Punct(p) => p.as_char() == '>', _=> false });

    seq!(main<'a>: &'a TokenTree => () = equal, ! greater, { () });

    main(input)
});

group!(object_pattern<'a>: &'a TokenTree => Vec<ObjectPattern> = |input| {

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

    alt!(last_option<'a>: &'a TokenTree => ObjectPattern = wild
                                                         | literal 
                                                         );

    alt!(leading_option<'a>: &'a TokenTree => ObjectPattern = bang
                                                            );

    seq!(option_semi<'a>: &'a TokenTree => ObjectPattern = o <= leading_option, semi_colon, { o });
    
    seq!(options<'a>: &'a TokenTree => Vec<ObjectPattern> = os <= * option_semi, o <= ! last_option, {
        let mut os = os;
        os.push(o);
        os
    });

    options(input)
});
