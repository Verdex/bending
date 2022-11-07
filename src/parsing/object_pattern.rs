
use proc_macro::{TokenStream, TokenTree, Delimiter};

use motif::*;
use denest::*;

use crate::data::*;

pub fn parse_object_pattern<'a>( input : (impl Iterator<Item = &'a TokenTree> + Clone) ) -> Result<ObjPatsAct, MatchError> {
    pred!(not_zero: usize = |x| x != 0);
    seq!(legit_sequence: usize => () = * not_zero, 0, { () });

    let mut input = input.enumerate();
    let pats = obj_pat_with_action(&mut input)?;
    if input.count() != 0 {
        return Err(MatchError::FatalEndOfFile);
    }

    let mut next_counts = pats.obj_pats.iter().map(|pat| pat.to_lax().filter(|x| matches!(x, ObjectPattern::Next)).count()).enumerate();

    legit_sequence(&mut next_counts)?;
    if next_counts.count() != 0 {
        return Err(MatchError::FatalEndOfFile);
    }

    Ok(pats)
}

pred!(comma<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ',', _ => false });
pred!(semi_colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ';', _ => false });
group!(arrow<'a>: &'a TokenTree => () = |input| {

    pred!(equal<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '=', _ => false });
    pred!(greater<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == '>', _ => false });

    seq!(main<'a>: &'a TokenTree => () = equal, ! greater, { () });

    main(input)
});

group!(colon_colon<'a>: &'a TokenTree => &'static str = |input| {
    pred!(colon<'a>: &'a TokenTree = |x| match x { TokenTree::Punct(p) => p.as_char() == ':', _ => false });
    seq!(main<'a>: &'a TokenTree => &'static str = colon, ! colon, { "::" });
    main(input)
});

group!(object_pattern<'a>: &'a TokenTree => Vec<ObjectPattern> = |input| {

    pred!(wild<'a>: &'a TokenTree => ObjectPattern = |x| match x { TokenTree::Ident(n) => n.to_string() == "_", _ => false } => {
        ObjectPattern::Wild
    });

    pred!(bang<'a>: &'a TokenTree => ObjectPattern = |x| match x { TokenTree::Punct(p) => p.as_char() == '!',  _ => false } => {
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

    seq!(ident_colon_colon<'a>: &'a TokenTree => String = ident <= TokenTree::Ident(_), colon_colon, {
        if let TokenTree::Ident(i) = ident {
            format!( "{}{}", i.to_string(), "::" )
        }
        else {
            unreachable!();
        }
    });

    seq!(cons_tag<'a>: &'a TokenTree => String = prefix <= ? colon_colon
                                               , body <= * ident_colon_colon
                                               , last <= TokenTree::Ident(_)
                                               , {
        if let TokenTree::Ident(i) = last {
            format!( "{}{}{}", prefix.or(Some("")).unwrap()
                             , body.into_iter().collect::<String>()
                             , i.to_string() )
        }
        else {
            unreachable!();
        }
    });

    seq!(pat_comma<'a>: &'a TokenTree => ObjectPattern = pat <= internal_option, comma, { pat });
    seq!(pat_list<'a>: &'a TokenTree => Vec<ObjectPattern> = first <= * pat_comma, last <= ! internal_option, {
        let mut first = first;
        first.push(last);
        first
    });

    group!(tuple<'a>: &'a TokenTree => Vec<ObjectPattern> = |input| {
        seq!(extract<'a>: &'a TokenTree => Option<TokenStream> = group <= TokenTree::Group(_), {
            if let TokenTree::Group(g) = group {
                if g.delimiter() == Delimiter::Parenthesis {
                    Some(g.stream())
                }
                else {
                    None
                }
            }
            else { 
                unreachable!();
            }
        });

        let group = match extract(input)? {
            Some(g) => g,
            None => { return Err(MatchError::FatalEndOfFile); },
        };

        let input = group.into_iter().collect::<Vec<TokenTree>>();

        if input.len() == 0 {
            Ok(vec![])
        }
        else {
            let mut input = input.iter().enumerate();

            pat_list(&mut input)
        }
    });

    alt!(internal_option<'a>: &'a TokenTree => ObjectPattern = wild
                                                             | literal 
                                                             | bang
                                                             );

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

seq!(obj_pat_with_action<'a>: &'a TokenTree => ObjPatsAct 
    = obj_pats <= object_pattern, arrow, g <= TokenTree::Group(_), { 
        if let TokenTree::Group(g) = g {
            ObjPatsAct { obj_pats, action : g.to_string() }
        }
        else {
            unreachable!();
        }
    });